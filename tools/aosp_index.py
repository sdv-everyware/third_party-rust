#!/usr/bin/env python3
"""Prints, as JSON, the Rust library modules AOSP has that any module may use:
crate_name -> [(module, version, type, file, features)].

A module whose visibility (its own, or its package's default_visibility) does not let
vendor/sdv-everyware/third_party/rust depend on it is left out: it is vendored like a missing
one.

The modules under vendor/sdv-everyware/third_party/rust (ours) are left out.
"""

import json
import os
import re
import sys

tree = sys.argv[1]
# where our crates are: a visibility rule must cover it
HERE = "vendor/sdv-everyware/third_party/rust"


def visible(rules):
    """Whether a visibility list lets modules under HERE depend on the module."""
    for rule in re.findall(r'"([^"]+)"', rules):
        if rule == "//visibility:public":
            return True
        m = re.match(r"//([^:]*):(__subpackages__|__pkg__)$", rule)
        if m:
            path, kind = m.group(1), m.group(2)
            if kind == "__subpackages__" and (HERE == path or HERE.startswith(path + "/") or path == ""):
                return True
    return False
roots = ["external/rust", "system/software_defined_vehicle", "external/boringssl"]
index = {}
for r in roots:
    for d, _, files in os.walk(os.path.join(tree, r)):
        if "Android.bp" not in files:
            continue
        f = os.path.join(d, "Android.bp")
        try:
            s = open(f).read()
        except OSError:
            continue
        pkg = re.search(r"package\s*\{(.*?)\n\}", s, re.S)
        default_vis = re.search(r"default_visibility:\s*\[(.*?)\]", pkg.group(1), re.S) if pkg else None
        for b in re.finditer(r"(rust_library\w*|rust_proc_macro)\s*\{(.*?)\n\}", s, re.S):
            body = b.group(2)
            vis = re.search(r"\n\s*visibility:\s*\[(.*?)\]", body, re.S) or default_vis
            if vis and not visible(vis.group(1)):
                continue
            name = re.search(r'\n\s*name:\s*"([^"]+)"', body)
            crate = re.search(r'crate_name:\s*"([^"]+)"', body)
            version = re.search(r'cargo_pkg_version:\s*"([^"]+)"', body)
            features = re.search(r"\n    features:\s*\[(.*?)\]", body, re.S)
            if name and crate:
                index.setdefault(crate.group(1), []).append(
                    (name.group(1), version.group(1) if version else "?", b.group(1),
                     os.path.relpath(f, tree),
                     re.findall(r'"([^"]+)"', features.group(1)) if features else []))
json.dump(index, sys.stdout)
