#!/usr/bin/env python3
"""Lays out the crates rs-matter needs and writes their Android.bp files.

Input: the `cargo metadata` of a scratch crate that depends on rs-matter as the Matter bundle
does (update-rs-matter.sh writes it), the `cargo vendor` directory, the index of the Rust modules
AOSP already has, and the list of generated cluster modules to keep.

Output, under the directory given as --out:
  rs-matter/<crate>/   rs-matter's own crates
  <crate>/             a third-party crate AOSP does not have, or has in a semver-incompatible
                       version (<crate>-<version>/ when two versions are needed)
Each with an Android.bp. A crate AOSP has in a compatible version is not copied: the module of
AOSP is used.

Module names: lib<crate>-rsm (lib<crate>-<major>-rsm for a second version). Soong requires a
library's name to start with lib<crate_name>; the suffix keeps them apart from AOSP's modules
(AOSP names a second version lib<crate>-<version>).
"""

import argparse
import json
import os
import re
import shutil

# Crates Soong does not need: the build scripts' helpers and what only they use.
SKIP = {"openssl-sys"}  # AOSP's openssl is BoringSSL's (bssl_sys), no openssl-sys

# Features AOSP's module of a crate may lack and still be the one used. AOSP's Android.bp lists
# the features a crate is built with, never the word "default". The others either change
# nothing (num-traits' i128 is always on) or give impls std gives too; and a second copy of
# log would split the logging facade. Any other missing feature makes the crate ours.
IGNORED_FEATURES = {
    "*": {"default"},
    "bitflags": {"std"},
    "log": {"alloc"},
    "memchr": {"use_std"},
    "num-integer": {"i128"},
    "num-traits": {"i128"},
    "rustix": {"termios"},
    "serde": {"alloc"},
    "serde_core": {"alloc"},
    "toml_datetime": {"std"},
}
# Always AOSP's, whatever its features: AOSP's openssl is BoringSSL's, which rs-matter's
# openssl backend is meant to run on (PATCHES.md).
ALWAYS_AOSP = {"openssl"}
# The device libraries here are rlibs only (rust_library_rlib): they serve the Matter bundle and
# are linked into it, nothing shares them as a dylib; and some cannot be dylibs at all, a dylib
# resolving every symbol and exporting through a version script: embassy-executor-timer-queue
# names a symbol of embassy's executor (`extern "Rust"`, used by its integrated timer queue only;
# the bundle runs rs-matter's timers on the generic queue, update-rs-matter.sh), defmt defines
# its encoding as a linker symbol (`_defmt_encoding_ = rzcobs`). What is shared, log, tokio,
# openssl, is AOSP's, and those stay dylibs.

# Parts of a crate not copied. linux-raw-sys has the kernel's bindings for every architecture
# (17 MB); each is a `#[path = "<arch>/..."]` module under `#[cfg(target_arch)]`, so the ones
# not built for (anything but Android arm64/arm and the x86 hosts) are never read.
LINUX_RAW_SYS_ARCHES = {"aarch64", "arm", "x86", "x86_64"}


def prune(name, dst):
    if name == "linux-raw-sys":
        src = os.path.join(dst, "src")
        for d in os.listdir(src):
            if os.path.isdir(os.path.join(src, d)) and d not in LINUX_RAW_SYS_ARCHES:
                shutil.rmtree(os.path.join(src, d))


# What the build scripts would set (rustc 1.93 and 1.95, aarch64-linux-android and x86_64 host).
# A list applies to every target; a dict has "android" and "host" lists (the target: { } block)
# and may have an "all" list for both.
CFGS = {
    "pinned-init": ["RUSTC_LINT_REASONS_IS_STABLE", "RUSTC_NEW_UNINIT_IS_STABLE",
                    "CONFIG_RUSTC_HAS_UNSAFE_PINNED"],
    "pinned-init-macro": ["RUSTC_LINT_REASONS_IS_STABLE"],
    # rustix picks the libc backend on Android and, with no inline assembly probe at build time,
    # the same here on the host: one backend for both; linux_raw_dep takes constants from
    # linux-raw-sys (a dependency the generator lays out). The two compiler probes pass on 1.93.
    "rustix": ["libc", "linux_raw_dep", "linux_kernel", "linux_like",
               "lower_upper_exp_for_non_zero", "rustc_diagnostics"],
    # nix: cfg_aliases of the operating system
    "nix": {"android": ["android", "linux_android"], "host": ["linux", "linux_android"]},
}


def cfgs_for(name):
    """(the cfgs of every target, {platform: its cfgs})."""
    c = CFGS.get(name, [])
    if isinstance(c, dict):
        return c.get("all", []), {k: v for k, v in c.items() if k != "all"}
    return c, {}

# Libraries a patched crate needs on top of its Cargo dependencies (see PATCHES.md).
EXTRA_RUSTLIBS = {
    # none: the crypto patch declares the BoringSSL functions it needs itself
}

# rs-matter crates whose own unit tests run on the host (rust_test_host): the crypto backend,
# patched for BoringSSL, against its test vectors.
HOST_TESTS = {"rs-matter-crypto"}

LICENSE_FILES = ("LICENSE", "LICENSE-APACHE", "LICENSE-MIT", "LICENSE-BSD", "LICENSE.md",
                 "LICENSE.txt", "COPYING", "UNLICENSE", "LICENSE-UNICODE", "LICENSE-ZLIB")


INCLUDE = re.compile(r'include(?:_str|_bytes)?!\s*\(\s*"([^"]+)"')


def compile_data(crate_dir):
    """The files the sources include (include_str!, include_bytes!, include! of a literal path)
    that are not Rust sources: Soong wants every input of rustc declared (compile_data)."""
    data = set()
    for root, _, files in os.walk(crate_dir):
        for f in files:
            if not f.endswith(".rs"):
                continue
            path = os.path.join(root, f)
            for m in INCLUDE.finditer(open(path, errors="replace").read()):
                target = os.path.normpath(os.path.join(root, m.group(1)))
                if os.path.isfile(target) and not target.endswith(".rs"):
                    data.add(os.path.relpath(target, crate_dir))
    return sorted(data)


def semver_compatible(a, b):
    a, b = a.split("."), b.split(".")
    if a[0] != b[0]:
        return False
    return a[1] == b[1] if a[0] == "0" else True


def lib_target(pkg):
    for t in pkg["targets"]:
        if any(k in ("lib", "proc-macro", "rlib") for k in t["kind"]):
            return t
    return None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--metadata", required=True, help="cargo metadata for aarch64-linux-android")
    ap.add_argument("--metadata-x86_64", help="cargo metadata for x86_64-linux-android")
    ap.add_argument("--metadata-host", required=True, help="cargo metadata for x86_64-unknown-linux-gnu")
    ap.add_argument("--vendor", required=True)
    ap.add_argument("--aosp-index", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--root", default="rsm-deps")
    args = ap.parse_args()

    aosp = json.load(open(args.aosp_index))
    # The dependencies are resolved for each platform a crate is built for: the 64-bit device
    # architectures (aarch64 and x86_64 Android) and the host (x86-64 Linux, for the proc-macros
    # and the host tests). A dependency that varies by CPU architecture (cpufeatures on x86_64,
    # libc in cpufeatures on arm64) goes into arch: { ... }; one that varies by OS goes into
    # target: { android | host }.
    metas = {"android_arm64": json.load(open(args.metadata))}
    android_plats = ["android_arm64"]
    if args.metadata_x86_64:
        metas["android_x86_64"] = json.load(open(args.metadata_x86_64))
        android_plats.append("android_x86_64")
    metas["host"] = json.load(open(args.metadata_host))
    all_plats = android_plats + ["host"]
    pkgs = {}
    for m in metas.values():
        pkgs.update({p["id"]: p for p in m["packages"]})
    nodes_on = {plat: {n["id"]: n for n in m["resolve"]["nodes"]} for plat, m in metas.items()}
    root = next(p["id"] for p in metas["android_arm64"]["packages"] if p["name"] == args.root)

    def normal(dep):
        return any(k["kind"] in (None, "normal") for k in dep["dep_kinds"])

    def deps_on(plat, i):
        node = nodes_on[plat].get(i)
        return [d for d in node["deps"] if normal(d)] if node else []

    # Everything reachable through normal dependencies on either platform (a build script's
    # dependencies are not built by Soong).
    need = set()
    for plat in metas:
        seen, stack = set(), [root]
        while stack:
            i = stack.pop()
            if i in seen:
                continue
            seen.add(i)
            need.add(i)
            stack += [d["pkg"] for d in deps_on(plat, i)]
    need.discard(root)
    need = {i for i in need if pkgs[i]["name"] not in SKIP}

    def is_proc_macro(i):
        t = lib_target(pkgs[i])
        return t is not None and "proc-macro" in t["kind"]

    # On the device: what is reachable without going through a proc-macro (a proc-macro and its
    # dependencies run in the compiler, on the host). A crate only the proc-macros use is a
    # host library (rust_library_host), like syn and quote in AOSP.
    device = set()
    for plat in android_plats:
        stack = [root]
        while stack:
            i = stack.pop()
            if i in device or is_proc_macro(i):
                continue
            device.add(i)
            stack += [d["pkg"] for d in deps_on(plat, i)]

    names = {}
    for i in need:
        names.setdefault(pkgs[i]["name"], []).append(i)

    def base_crate(i):
        return lib_target(pkgs[i])["name"].replace("-", "_")

    def version_tag(i):
        v = pkgs[i]["version"].split(".")
        return v[0] if v[0] != "0" else "0_" + v[1]

    def crate_name(i):
        """The crate's name for rustc. A second version of a crate gets the version in its name
        (embedded_hal_0_2): Soong's aliases are keyed by crate name, so two dependencies with
        one name could not be told apart; the users get an alias to the name they use."""
        if ours(i) and len(names[pkgs[i]["name"]]) > 1:
            return f"{base_crate(i)}_{version_tag(i)}"
        return base_crate(i)

    def wanted_features(i):
        return set().union(*(set(nodes_on[plat][i]["features"]) for plat in nodes_on
                             if i in nodes_on[plat]))

    def aosp_module(i):
        """AOSP's module for the crate: a compatible version with every feature we use."""
        p = pkgs[i]
        for name, version, kind, _bp, features in aosp.get(base_crate(i), []):
            # a crate the device needs must be a device library, not a host-only one
            if i in device and kind.startswith("rust_library_host"):
                continue
            if version != "?" and not semver_compatible(p["version"], version):
                continue
            ignored = IGNORED_FEATURES["*"] | IGNORED_FEATURES.get(p["name"], set())
            if p["name"] not in ALWAYS_AOSP and not wanted_features(i) - ignored <= set(features):
                continue
            return name
        return None

    def ours(i):
        p = pkgs[i]
        return p["source"] is None or aosp_module(i) is None

    # A crate reached only through AOSP's crates is not laid out: AOSP's module links AOSP's
    # own dependencies (indexmap its hashbrown, jiff its jiff-core), so a copy here would be
    # built by nobody.
    reached = set()
    for plat in metas:
        stack = [root]
        while stack:
            i = stack.pop()
            if i in reached:
                continue
            reached.add(i)
            if i == root or ours(i):
                stack += [d["pkg"] for d in deps_on(plat, i)]
    need &= reached
    names.clear()
    for i in need:
        names.setdefault(pkgs[i]["name"], []).append(i)

    def module(i):
        p = pkgs[i]
        if not ours(i):
            return aosp_module(i)
        return f"lib{crate_name(i)}-rsm"

    def directory(i):
        p = pkgs[i]
        if p["source"] is None:
            return os.path.join(args.out, "rs-matter", p["name"])
        if len(names[p["name"]]) > 1:
            return os.path.join(args.out, f"{p['name']}-{p['version']}")
        return os.path.join(args.out, p["name"])

    # the directories an earlier run generated: the ones not generated again are removed
    marker = "// Generated by vendor/sdv-everyware/third_party/rust/update-rs-matter.sh."
    earlier = set()
    for d, _, files in os.walk(args.out):
        if "Android.bp" in files and open(os.path.join(d, "Android.bp")).read().startswith(marker):
            earlier.add(os.path.normpath(d))

    report = []
    for i in sorted(need, key=lambda i: (pkgs[i]["name"], pkgs[i]["version"])):
        p = pkgs[i]
        if not ours(i):
            continue
        src = os.path.dirname(p["manifest_path"])
        if p["source"] is not None:
            src = os.path.join(args.vendor, f"{p['name']}-{p['version']}")
        dst = directory(i)
        if os.path.exists(dst):
            shutil.rmtree(dst)
        shutil.copytree(src, dst, ignore=shutil.ignore_patterns(
            "target", ".git", ".cargo-checksum.json", "Android.bp", "benches", "examples",
            "tests", "fuzz"))
        prune(p["name"], dst)

        t = lib_target(p)
        proc_macro = "proc-macro" in t["kind"]
        on_device = not proc_macro and i in device
        aliases = []
        per = {}  # platform -> (rustlibs, proc_macros)
        for plat in (all_plats if on_device else ("host",)):
            libs, macros = set(), set()
            for d in deps_on(plat, i):
                if d["pkg"] not in need:
                    continue
                (macros if is_proc_macro(d["pkg"]) else libs).add(module(d["pkg"]))
                if d["name"] != crate_name(d["pkg"]):
                    aliases.append(f"{crate_name(d['pkg'])}:{d['name']}")
            per[plat] = (libs, macros)
        arch_libs = {}
        target_libs = {}
        if on_device:
            common = set.intersection(*(per[plat][0] for plat in all_plats))
            all_libs = set.union(*(per[plat][0] for plat in all_plats))
            proc_macros = set.union(*(per[plat][1] for plat in all_plats))
            if "android_x86_64" in per:
                for lib in all_libs - common:
                    on_a64 = lib in per["android_arm64"][0]
                    on_ax64 = lib in per["android_x86_64"][0]
                    on_host = lib in per["host"][0]
                    if on_a64 and on_ax64 and not on_host:
                        target_libs.setdefault("android", set()).add(lib)
                    elif on_host and not on_a64 and not on_ax64:
                        target_libs.setdefault("host", set()).add(lib)
                    elif on_ax64 == on_host:
                        if on_a64:
                            arch_libs.setdefault("arm64", set()).add(lib)
                        if on_ax64:
                            arch_libs.setdefault("x86_64", set()).add(lib)
                    else:
                        for plat in all_plats:
                            if lib in per[plat][0]:
                                target_libs.setdefault(plat, set()).add(lib)
            else:
                if per["android_arm64"][0] - common:
                    target_libs["android"] = per["android_arm64"][0] - common
                if per["host"][0] - common:
                    target_libs["host"] = per["host"][0] - common
        else:
            common, proc_macros = per["host"][0], per["host"][1]
        rustlibs = sorted(common) + EXTRA_RUSTLIBS.get(p["name"], [])
        features = set()
        for plat in nodes_on:
            if i in nodes_on[plat]:
                features |= set(nodes_on[plat][i]["features"])

        lines = [
            "// Generated by vendor/sdv-everyware/third_party/rust/update-rs-matter.sh. Do not edit.",
            "",
            "package {",
            '    default_applicable_licenses: ["vendor_sdv_everyware_rust_license"],',
            "}",
            "",
            ("rust_proc_macro {" if proc_macro else
             "rust_library_rlib {" if on_device else "rust_library_host {"),
            f'    name: "{module(i)}",',
        ]
        if on_device:
            lines.append("    host_supported: true,")
        elif not proc_macro:
            # as AOSP's host libraries (syn, quote): only for the compiler's host
            lines += ["    host_cross_supported: false,", '    compile_multilib: "first",']
        lines += [
            f'    crate_name: "{crate_name(i)}",',
            "    cargo_env_compat: true,",
            f'    cargo_pkg_version: "{p["version"]}",',
            f'    crate_root: "{os.path.relpath(t["src_path"], os.path.dirname(p["manifest_path"]))}",',
            f'    edition: "{t.get("edition", p["edition"])}",',
        ]

        def plist(key, values):
            if values:
                lines.append(f"    {key}: [")
                lines.extend(f'        "{v}",' for v in sorted(set(values)))
                lines.append("    ],")

        plist("features", features)
        common_cfgs, target_cfgs = cfgs_for(p["name"])
        plist("cfgs", common_cfgs)
        plist("rustlibs", rustlibs)
        plist("proc_macros", proc_macros)
        plist("aliases", aliases)
        if arch_libs:
            lines.append("    arch: {")
            for arch in sorted(arch_libs):
                lines.append(f"        {arch}: {{")
                lines.append("            rustlibs: [")
                lines.extend(f'                "{v}",' for v in sorted(arch_libs[arch]))
                lines.append("            ],")
                lines.append("        },")
            lines.append("    },")
        plats = sorted(set(plat for plat, libs in target_libs.items() if libs) | set(target_cfgs))
        if plats:
            lines.append("    target: {")
            for plat in plats:
                lines.append(f"        {plat}: {{")
                for key, values in (("rustlibs", target_libs.get(plat)), ("cfgs", target_cfgs.get(plat))):
                    if values:
                        lines.append(f"            {key}: [")
                        lines.extend(f'                "{v}",' for v in sorted(values))
                        lines.append("            ],")
                lines.append("        },")
            lines.append("    },")
        # With crate_root set, srcs lists every file rustc may read: the sources and the files
        # they include (this Soong has no compile_data property).
        data = compile_data(dst)
        if data:
            lines += ["    srcs: [", '        "**/*.rs",'] + [f'        "{d}",' for d in data] + ["    ],"]
        # warnings are upstream's, not ours
        lines.append('    flags: ["--cap-lints=allow"],')
        if on_device:
            lines += [
                "    apex_available: [",
                '        "//apex_available:platform",',
                '        "//apex_available:anyapex",',
                "    ],",
                "    product_available: true,",
                "    vendor_available: true,",
                '    min_sdk_version: "29",',
            ]
        lines.append("}")
        if p["name"] in HOST_TESTS:
            lines += [
                "",
                "rust_test_host {",
                f'    name: "{crate_name(i)}_test",',
                f'    crate_name: "{crate_name(i)}",',
                "    cargo_env_compat: true,",
                f'    cargo_pkg_version: "{p["version"]}",',
                f'    crate_root: "{os.path.relpath(t["src_path"], os.path.dirname(p["manifest_path"]))}",',
                f'    edition: "{t.get("edition", p["edition"])}",',
                '    test_suites: ["general-tests"],',
                "    auto_gen_config: true,",
            ]
            plist("features", features)
            plist("rustlibs", sorted(per["host"][0]) + EXTRA_RUSTLIBS.get(p["name"], []))
            plist("proc_macros", proc_macros)
            plist("aliases", aliases)
            lines += ['    flags: ["--cap-lints=allow"],', "}"]
        open(os.path.join(dst, "Android.bp"), "w").write("\n".join(lines) + "\n")
        report.append((p["name"], p["version"], p.get("license") or "?",
                       os.path.relpath(dst, args.out), module(i)))

    for d in sorted(earlier - {os.path.normpath(os.path.join(args.out, r[3])) for r in report}):
        print("removed", os.path.relpath(d, args.out))
        shutil.rmtree(d)

    with open(os.path.join(args.out, "CRATES.md"), "w") as f:
        f.write("# Crates\n\nGenerated by update-rs-matter.sh. The crates rs-matter needs that AOSP "
                "does not have in a compatible version; the others are AOSP's.\n\n")
        f.write("| Crate | Version | License | Directory | Module |\n|---|---|---|---|---|\n")
        for r in report:
            f.write("| " + " | ".join(r) + " |\n")
    print(len(report), "crates written")


if __name__ == "__main__":
    main()
