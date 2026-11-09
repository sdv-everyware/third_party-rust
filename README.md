# Rust crates for Soong: rs-matter and what it needs, and the few others

[rs-matter](https://github.com/project-chip/rs-matter) (the Matter stack in Rust) and the crates
it needs that AOSP does not have, built by Soong like AOSP's own crates. The Matter bundle links
them. AOSP's `external/rust` is not changed.

A crate rs-matter needs is AOSP's when AOSP has it in a semver-compatible version, its
visibility lets this directory depend on it, and it is built with the features rs-matter uses
(`tools/gen_bp.py`: `IGNORED_FEATURES` lists the ones that may be missing and why, `ALWAYS_AOSP`
the crates that are AOSP's in any case: openssl, which is BoringSSL's). Otherwise the crate is
here. One crate, one copy: `log`, `tokio`, `openssl` and the like are AOSP's, so the bundle has
one logging facade and one TLS stack.

```
vendor/sdv-everyware/third_party/rust/
  rs-matter/<crate>/          rs-matter's own crates (pinned rev in update-rs-matter.sh)
    rs-matter-dm-clusters/src/generated/   the cluster code rs-matter's build.rs would generate
  <crate>/ or <crate>-<ver>/  third-party crates AOSP does not have, has in a semver-incompatible
                              version, or keeps visible to a few modules only
  smoke/                      build check: rsm_smoke_test links rs-matter, the commissioner, the IM
                              client and runs an AES-CCM round trip (device and host)
  tokio-openssl/              not for rs-matter: everyware's libs/https needs it and AOSP's module
                              is visible to the SDV communication SDK only; added by hand from
                              crates.io (cargo vendor, the registry's checksums in
                              .cargo-checksum.json), module libtokio_openssl-ew
  CRATES.md                   every crate the generator lays out: version, license, directory, module
  PATCHES.md                  every change to an upstream source, and why
  clusters.txt                the generated clusters kept
  update-rs-matter.sh         regenerates all of it
  tools/                      gen_bp.py (layout, Android.bp), aosp_index.py, prune_clusters.py
```

## Using it

A module links rs-matter like any Rust library:

```
rustlibs: [
    "librs_matter-rsm",           // rs-matter: os, openssl, log
    "librs_matter_onboard-rsm",   // the commissioner
    "librs_matter_im_core-rsm",   // the interaction model client (read, write, invoke)
],
```

The device libraries here are rlibs only (`rust_library_rlib`): nothing shares them as a
dylib, and some cannot be dylibs (embassy-executor-timer-queue names a symbol of embassy's
executor, defmt defines its encoding as a linker symbol). A module that links them links its
Rust dependencies as rlibs too (`prefer_rlib: true`, `-Wl,--no-as-needed` for the SDV FFI
libraries, as SDV's sample bundles do), or a crate is there twice. rs-matter's timers run on
embassy-time's std driver with its generic queue (`embassy-time-queue-utils` feature
`generic-queue-64`); the integrated queue needs embassy's executor.

The module names are `lib<crate>-rsm`. Soong requires a library's name to start with
`lib<crate_name>`; the suffix keeps them apart from AOSP's modules, which name a second version
`lib<crate>-<version>`. A crate needed in two versions (here or one here and one in AOSP) gets
the version in its crate name, `embedded_hal_0_2`, `syn_3`, and its users an alias to the name
they use: Soong's `aliases` are keyed by crate name, so two dependencies with one name could
not be told apart.

A Soong namespace instead of the suffix would keep the module names apart too, but every product
would have to list it in `PRODUCT_SOONG_NAMESPACES` and every reference would need the full
name (`//vendor/sdv-everyware/third_party/rust:<module>`).

Crypto is the openssl backend of rs-matter on AOSP's openssl crate, which is BoringSSL: the one
TLS and crypto stack of the bundles (`libs/https` uses it too). AES-CCM goes through BoringSSL's
AEAD (PATCHES.md).

## Updating

The layout here is for the public AOSP tree (the `android17-release` manifest): the crates it
does not have in a compatible version, with the versions its modules have in mind. The script
looks at the tree it runs in; run in a tree with other Rust modules, it would lay out what that
tree lacks instead and remove what it has. From another tree, point it at a checkout of the
public one with `ANDROID_TREE`; cargo and rustc come from the script's own tree either way.

```bash
# WHAT: regenerate everything here from the rs-matter rev in the script
# WHY: a new rs-matter, or a cluster added to clusters.txt
# WARNING: needs the network (GitHub, crates.io); replaces the crate directories; lays out what
# the tree lacks, so run it in the public tree or with ANDROID_TREE pointing at one
./vendor/sdv-everyware/third_party/rust/update-rs-matter.sh
ANDROID_TREE=<public tree> ./vendor/sdv-everyware/third_party/rust/update-rs-matter.sh
# WHAT: the build check, and rs-matter-crypto's own tests on the host (its AES-CCM, HKDF,
# PBKDF2, ECDSA and ECDH test vectors, on BoringSSL)
m rsm_smoke_test rs_matter_crypto_test
atest rs_matter_crypto_test rsm_smoke_test
```

`update-rs-matter.sh` clones rs-matter at the pinned rev, resolves the bundle's dependencies for
aarch64 Android (`cargo metadata`, `cargo vendor`), runs rs-matter's cluster code generator,
lays the crates out with their `Android.bp` and applies `patches/`. The dependencies are
resolved for the device (aarch64 Android) and for the host (x86-64 Linux: proc-macros, host
tests); one that a crate takes only on one platform goes into `target: { android | host }`. A
crate a build script would configure gets the same cfgs in `tools/gen_bp.py`. A crate only the
proc-macros use is a host library. Files a crate includes (`include_str!`) are listed in `srcs`
next to `**/*.rs`, as Soong wants every input of rustc declared. `linux-raw-sys` is copied with
the bindings of arm64, arm and x86 only (`prune` in `tools/gen_bp.py`).

## The generated clusters

rs-matter generates Rust code for all 153 clusters of the Matter specification (22 MB) in a build
script. Here only the clusters in `clusters.txt` are kept (59, 11 MB): the ones rs-matter's own
sources name (its cluster handlers and the root endpoint) and the ones the Matter bundle uses
(thermostat, relative humidity, on/off, level control, fan control, basic information,
diagnostics). A cluster that is needed and missing fails the build with an unresolved module;
add it to `clusters.txt` and run the script.

## Licenses

Every crate's license is in `CRATES.md` and its own directory. Each crate here offers
Apache-2.0, MIT or BSD-3-Clause (some also others, such as Unlicense or BlueOak-1.0.0).
