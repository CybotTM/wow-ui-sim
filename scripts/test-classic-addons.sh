#!/usr/bin/env bash
# Third-party-addon test harness for the Mists client profile.
#
# Reads tools/classic-addon-manifest.tsv. For each entry:
#   1. Resolve the addon source:
#      - Git rows clone into vendor/addons/<name>/ if not present (idempotent)
#        and check out the pinned ref
#      - local:<absolute-path> rows use the existing on-disk addon directory
#      - mists-addon:<addon> rows use the installed Mists addon when present,
#        otherwise fall back to committed CI fixtures
#   2. Symlink Interface/AddOns/<name> -> source/<subpath>
#   3. Build wow-sim with the addon's profile feature
#   4. Run lua-errors, save to target/addon-harness/<name>-lua-errors.json
#   5. Diff message-set against docs/baselines/mists-lua-errors.json and
#      report the count of *addon-induced* errors (= new vs baseline)
#   6. Tear down the symlink so the test directory is clean for the next run
#
# Pass criterion by default: wow-sim must boot without crashing (exit 0).
# Addon-induced Lua errors are counted and reported. Pass
# --fail-on-addon-errors to fail the harness on any addon-induced regression.
#
# Usage:
#   scripts/test-classic-addons.sh                    # run every addon in manifest
#   scripts/test-classic-addons.sh AllTheThings       # run a single addon by name
#   scripts/test-classic-addons.sh --profile mists    # filter by profile
#   scripts/test-classic-addons.sh --skip-clone       # use already-cloned vendors
#   scripts/test-classic-addons.sh --skip-build       # use an existing wow-sim binary
#   scripts/test-classic-addons.sh --keep-symlinks    # don't tear down on finish
#   scripts/test-classic-addons.sh --with-saved-vars  # load WTF SavedVariables
#   scripts/test-classic-addons.sh --fail-on-addon-errors
#
# Exit codes:
#   0  every addon's wow-sim invocation exited 0
#   1  one or more wow-sim invocations failed to boot
#   2  manifest unreadable / repo state invalid

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST="$REPO_ROOT/tools/classic-addon-manifest.tsv"
COMPAT_ROOT="$REPO_ROOT/tools/classic-addon-compat"
VENDOR_DIR="$REPO_ROOT/vendor/addons"
ADDONS_DIR="$REPO_ROOT/Interface/AddOns"
OUT_DIR="$REPO_ROOT/target/addon-harness"
WOW_SIM_BIN="${WOW_SIM_BIN:-$REPO_ROOT/target/debug/wow-sim}"

source "$REPO_ROOT/scripts/classic-addon-sources.sh"

NAME_FILTER=""
PROFILE_FILTER=""
SKIP_CLONE=0
SKIP_BUILD=0
KEEP_SYMLINKS=0
WITH_SAVED_VARS=0
FAIL_ON_ADDON_ERRORS=0

while [ $# -gt 0 ]; do
    case "$1" in
        --profile)        PROFILE_FILTER="$2"; shift 2 ;;
        --skip-clone)     SKIP_CLONE=1; shift ;;
        --skip-build)     SKIP_BUILD=1; shift ;;
        --keep-symlinks)  KEEP_SYMLINKS=1; shift ;;
        --with-saved-vars) WITH_SAVED_VARS=1; shift ;;
        --fail-on-addon-errors) FAIL_ON_ADDON_ERRORS=1; shift ;;
        --help|-h)
            sed -n '2,/^set -euo/p' "$0" | sed 's/^# \?//;$d'
            exit 0
            ;;
        -*)
            echo "ERROR: unknown flag '$1'" >&2; exit 2 ;;
        *)
            NAME_FILTER="$1"; shift ;;
    esac
done

if [ ! -f "$MANIFEST" ]; then
    echo "ERROR: manifest not found at $MANIFEST" >&2
    exit 2
fi

mkdir -p "$VENDOR_DIR" "$OUT_DIR"

ensure_source() {
    local name="$1" profile="$2" url="$3" ref="$4"
    if is_local_source "$url" || is_manifest_managed_source "$url"; then
        local src
        src=$(resolve_addon_source_root "$name" "$profile" "$url" "$VENDOR_DIR")
        [ -d "$src" ] || { echo "ERROR: local source $src not found" >&2; return 1; }
        echo "  → using source $src"
        return 0
    fi

    local dest="$VENDOR_DIR/$name"
    if [ "$SKIP_CLONE" -eq 1 ] && [ -d "$dest/.git" ]; then
        return 0
    fi
    if [ -d "$dest/.git" ]; then
        echo "  → updating $name to $ref"
        git -C "$dest" fetch --depth=1 origin "$ref" 2>/dev/null || git -C "$dest" fetch origin
        git -C "$dest" checkout --detach "$ref"
    else
        echo "  → cloning $url @ $ref into $dest"
        rm -rf "$dest"
        git clone --filter=blob:none --no-checkout "$url" "$dest"
        git -C "$dest" fetch --depth=1 origin "$ref"
        git -C "$dest" checkout --detach "$ref"
    fi
}

install_symlink() {
    local name="$1" src_root="$2" subpath="$3"
    local src="$src_root/$subpath"
    local dst="$ADDONS_DIR/$name"
    if [ ! -d "$src" ]; then
        echo "ERROR: $src not found" >&2
        return 1
    fi
    [ -L "$dst" ] && rm "$dst"
    [ -e "$dst" ] && { echo "ERROR: $dst exists and is not a symlink" >&2; return 1; }
    ln -s "$src" "$dst"
}

# Install per-addon compat shims, if any. Convention: each subdirectory under
# tools/classic-addon-compat/<name>/ is treated as an in-tree companion
# addon; we symlink it into Interface/AddOns/<subdir> so the loader picks
# it up. The shim's TOC should declare `## LoadFirst: 1` so it runs before
# the third-party addon and gets a chance to stub missing globals.
install_compat_shims() {
    local name="$1"
    local compat_dir="$COMPAT_ROOT/$name"
    [ -d "$compat_dir" ] || return 0
    local shim
    for shim in "$compat_dir"/*/; do
        [ -d "$shim" ] || continue
        local shim_name
        shim_name=$(basename "$shim")
        local dst="$ADDONS_DIR/$shim_name"
        [ -L "$dst" ] && rm "$dst"
        [ -e "$dst" ] && { echo "ERROR: $dst exists and is not a symlink" >&2; return 1; }
        ln -s "$shim" "$dst"
        echo "  → compat shim: $shim_name"
    done
}

remove_symlink() {
    local name="$1"
    local dst="$ADDONS_DIR/$name"
    [ -L "$dst" ] && rm "$dst"
}

remove_compat_shims() {
    local name="$1"
    local compat_dir="$COMPAT_ROOT/$name"
    [ -d "$compat_dir" ] || return 0
    local shim
    for shim in "$compat_dir"/*/; do
        [ -d "$shim" ] || continue
        local shim_name
        shim_name=$(basename "$shim")
        local dst="$ADDONS_DIR/$shim_name"
        [ -L "$dst" ] && rm "$dst"
    done
}

count_distinct_errors() {
    local file="$1"
    [ -s "$file" ] || { echo 0; return; }
    jq 'length' "$file" 2>/dev/null || echo 0
}

addon_induced_errors() {
    local profile="$1" addon_json="$2"
    local baseline="$REPO_ROOT/docs/baselines/${profile}-lua-errors.json"
    if [ ! -f "$baseline" ]; then
        echo "no-baseline"
        return
    fi
    # `regressed` = messages present in addon run but not profile baseline.
    "$REPO_ROOT/scripts/diff-lua-errors.sh" "$baseline" "$addon_json" --quiet \
        | sed -n 's/.*regressed=\([0-9]*\).*/\1/p'
}

run_addon() {
    local name="$1" profile="$2" url="$3" ref="$4" subpath="$5"
    echo ""
    echo "=== $name ($profile) ==="

    ensure_source "$name" "$profile" "$url" "$ref"
    local src_root
    src_root=$(resolve_addon_source_root "$name" "$profile" "$url" "$VENDOR_DIR")
    install_symlink "$name" "$src_root" "$subpath"
    install_compat_shims "$name"

    teardown() {
        if [ "$KEEP_SYMLINKS" -eq 0 ]; then
            remove_symlink "$name"
            remove_compat_shims "$name"
        fi
    }

    if [ "$SKIP_BUILD" -eq 0 ]; then
        echo "  → cargo build --features client-$profile"
        if ! cargo build --bin wow-sim --no-default-features \
                --features "sound,gui,casc,client-$profile" 2>&1 \
                | tail -3; then
            teardown
            echo "  ✗ BUILD FAILED"
            return 1
        fi
    fi

    local out_suffix=""
    local saved_vars_env=(WOW_SIM_NO_SAVED_VARS=1)
    if [ "$WITH_SAVED_VARS" -eq 1 ]; then
        out_suffix="-with-saved-vars"
        saved_vars_env=()
    fi

    local out="$OUT_DIR/$name${out_suffix}-lua-errors.json"
    echo "  → running lua-errors → $out"
    if ! env "${saved_vars_env[@]}" timeout 120 \
            "$WOW_SIM_BIN" lua-errors > "$out" 2>/dev/null; then
        teardown
        echo "  ✗ wow-sim exited nonzero — possible crash"
        return 1
    fi

    local total
    total=$(count_distinct_errors "$out")
    local induced
    induced=$(addon_induced_errors "$profile" "$out")
    echo "  ✓ booted; $total distinct errors total, $induced addon-induced (vs baseline)"
    if [ "$FAIL_ON_ADDON_ERRORS" -eq 1 ] && [ "$induced" != "0" ]; then
        teardown
        echo "  ✗ addon-induced lua-errors regression"
        return 1
    fi

    teardown
    return 0
}

# Iterate manifest, skipping comments and blank lines.
declare -i pass=0 fail=0
while IFS=$'\t' read -r name profile url ref subpath; do
    [[ "$name" =~ ^# ]] && continue
    [ -z "${name:-}" ] && continue
    [ "$name" = "name" ] && continue  # header row
    [ -n "$NAME_FILTER" ] && [ "$NAME_FILTER" != "$name" ] && continue
    [ -n "$PROFILE_FILTER" ] && [ "$PROFILE_FILTER" != "$profile" ] && continue
    if run_addon "$name" "$profile" "$url" "$ref" "$subpath"; then
        pass+=1
    else
        fail+=1
    fi
done < "$MANIFEST"

echo ""
echo "================================================"
echo "  passed: $pass    failed: $fail"
echo "================================================"

[ "$fail" -gt 0 ] && exit 1
exit 0
