#!/bin/bash
# update-rs-matter.sh: rs-matter and the crates it needs, for Soong, from a pinned rs-matter rev.
#
# What it does:
#   1. clones rs-matter at RS_MATTER_REV into a work directory
#   2. a scratch crate depends on rs-matter as the Matter bundle does (os, openssl, log;
#      rs-matter-onboard, rs-matter-im-core); cargo metadata and cargo vendor of it
#   3. runs rs-matter's cluster code generator (rs-matter-codegen, normally run by
#      rs-matter-dm-clusters/build.rs, which Soong does not run) and keeps the clusters in
#      clusters.txt
#   4. lays the crates out here with their Android.bp (tools/gen_bp.py): rs-matter's own and the
#      third-party ones AOSP does not have in a compatible version
#   5. applies patches/*.patch (PATCHES.md says why each)
#
# Usage: ./update-rs-matter.sh [work dir]      (needs the network: GitHub and crates.io)
# Env:   ANDROID_TREE: the tree whose Rust modules the layout is for (default: the tree this
#                      repository is in). What that tree lacks is laid out here; a crate it has
#                      is removed from here and linked from it. cargo and rustc always come from
#                      this repository's own tree (the newest toolchain under prebuilts).
#        RS_MATTER_REV
set -euo pipefail

HERE=$(cd "$(dirname "$0")" && pwd)
OWN_TREE=$(cd "$HERE/../../../.." && pwd)
TREE=${ANDROID_TREE:-$OWN_TREE}
RS_MATTER_REV=${RS_MATTER_REV:-bf3fe6b94f6428d12611c67fcf698883c269507d}
WORK=${1:-$(mktemp -d /tmp/rs-matter-update.XXXXXX)}
TOOLCHAIN=$(ls -d "$OWN_TREE"/prebuilts/rust-toolchain/linux-x86/*/bin/cargo | sort -V | tail -1)
[ -n "$TOOLCHAIN" ] || { echo "no cargo under $OWN_TREE/prebuilts/rust-toolchain" >&2; exit 1; }
export PATH="$(dirname "$TOOLCHAIN"):$PATH"
export CARGO_HOME=${CARGO_HOME:-$WORK/cargo-home}

say() { echo ">> $*"; }

say "rs-matter $RS_MATTER_REV in $WORK"
mkdir -p "$WORK"
if [ ! -d "$WORK/rs-matter/.git" ]; then
    git clone -q https://github.com/project-chip/rs-matter.git "$WORK/rs-matter"
fi
git -C "$WORK/rs-matter" fetch -q origin "$RS_MATTER_REV" 2>/dev/null || true
git -C "$WORK/rs-matter" checkout -q "$RS_MATTER_REV"
RSM=$WORK/rs-matter

say "the bundle's dependencies (cargo metadata, cargo vendor)"
mkdir -p "$WORK/rsm-deps/src"
cat > "$WORK/rsm-deps/Cargo.toml" <<EOF
[package]
name = "rsm-deps"
version = "0.1.0"
edition = "2021"
[dependencies]
rs-matter = { path = "$RSM/rs-matter", default-features = false, features = ["os", "openssl", "log"] }
rs-matter-onboard = { path = "$RSM/rs-matter-onboard" }
rs-matter-im-core = { path = "$RSM/rs-matter-im-core" }
# rs-matter's timers run on embassy-time's std driver; without a generic queue it falls back to
# the integrated one, which needs embassy's executor (the bundle runs rs-matter on async-io)
embassy-time-queue-utils = { version = "0.3", features = ["generic-queue-64"] }
[workspace]
EOF
: > "$WORK/rsm-deps/src/lib.rs"
(cd "$WORK/rsm-deps" && cargo generate-lockfile -q &&
    cargo metadata -q --format-version 1 --filter-platform aarch64-linux-android > "$WORK/metadata.json" &&
    cargo metadata -q --format-version 1 --filter-platform x86_64-linux-android > "$WORK/metadata-x86_64.json" &&
    cargo metadata -q --format-version 1 --filter-platform x86_64-unknown-linux-gnu > "$WORK/metadata-host.json" &&
    cargo vendor -q --versioned-dirs "$WORK/vendor" > /dev/null)

say "the Rust modules of $TREE (rustc $(rustc --version | cut -d' ' -f2))"
python3 "$HERE/tools/aosp_index.py" "$TREE" > "$WORK/aosp-index.json"

say "the crates and their Android.bp"
python3 "$HERE/tools/gen_bp.py" --metadata "$WORK/metadata.json" \
    --metadata-x86_64 "$WORK/metadata-x86_64.json" \
    --metadata-host "$WORK/metadata-host.json" --vendor "$WORK/vendor" \
    --aosp-index "$WORK/aosp-index.json" --out "$HERE"

say "the cluster code (rs-matter-codegen), $(wc -l < "$HERE/clusters.txt") clusters kept"
mkdir -p "$WORK/codegen/src"
cat > "$WORK/codegen/Cargo.toml" <<EOF
[package]
name = "rsm-codegen"
version = "0.1.0"
edition = "2021"
[dependencies]
rs-matter-codegen = { path = "$RSM/rs-matter-codegen" }
[workspace]
EOF
cat > "$WORK/codegen/src/main.rs" <<'EOF'
fn main() {
    let dir = std::env::args().nth(1).expect("output directory");
    rs_matter_codegen::generate("crate", std::path::Path::new(&dir));
}
EOF
rm -rf "$WORK/generated"
(cd "$WORK/codegen" && cargo run -q --release -- "$WORK/generated")
GEN=$HERE/rs-matter/rs-matter-dm-clusters/src/generated
mkdir -p "$GEN/clusters_generated"
python3 "$HERE/tools/prune_clusters.py" "$WORK/generated" "$GEN" "$HERE/clusters.txt"

say "the build time of rs-matter-utils (its build.rs), the date of this update"
python3 - "$HERE/rs-matter/rs-matter-utils/src/build_time.rs" <<'EOF'
import sys, time
secs = int(time.time()) - 946_684_800  # the Matter epoch, 2000-01-01
open(sys.argv[1], "w").write(
    "// Written by update-rs-matter.sh (rs-matter-utils/build.rs is not run by Soong).\n"
    "/// The time of the update that vendored rs-matter, Matter-epoch microseconds: the lower\n"
    "/// bound of the Last Known Good UTC Time (Matter Core spec 3.5.6.1).\n"
    f"pub const FIRMWARE_BUILD_MATTER_US: u64 = {secs * 1_000_000};\n")
EOF

say "patches"
for p in "$HERE"/patches/*.patch; do
    [ -e "$p" ] || continue
    echo "   $(basename "$p")"
    patch -s -p1 -d "$HERE" < "$p"
done

say "done: $(grep -c '^| ' "$HERE/CRATES.md") crates (CRATES.md)"
