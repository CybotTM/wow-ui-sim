#!/usr/bin/env bash
# Run the Mists panel parity matrix once per installed third-party addon.
#
# Reads tools/classic-addon-manifest.tsv, selects the Mists rows, symlinks one
# addon at a time into Interface/AddOns, and runs scripts/mists-panel-parity.sh
# with third-party addons enabled. Any panel lua-error, missing frame,
# low-signal render, or visual-baseline regression fails that addon row.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST="$REPO_ROOT/tools/classic-addon-manifest.tsv"
COMPAT_ROOT="$REPO_ROOT/tools/classic-addon-compat"
ADDONS_DIR="$REPO_ROOT/Interface/AddOns"
OUT_DIR="$REPO_ROOT/target/mists-addon-panel-parity"
WOW_SIM_BIN="${WOW_SIM_BIN:-$REPO_ROOT/target/debug/wow-sim}"
PANEL_VISUAL_METRICS_BIN="${PANEL_VISUAL_METRICS_BIN:-$REPO_ROOT/target/debug/panel-visual-metrics}"

source "$REPO_ROOT/scripts/classic-addon-sources.sh"

NAME_FILTER=""
PANEL_FILTER=""
SKIP_BUILD=0
WITH_SAVED_VARS=0
KEEP_SYMLINKS=0
VALIDATE_ONLY=0
ACTIVE_ADDON=""

usage() {
    sed -n '2,/^set -euo/p' "$0" | sed 's/^# \?//;$d'
}

while [ $# -gt 0 ]; do
    case "$1" in
        --addon) NAME_FILTER="$2"; shift 2 ;;
        --panel) PANEL_FILTER="$2"; shift 2 ;;
        --out-dir) OUT_DIR="$2"; shift 2 ;;
        --skip-build) SKIP_BUILD=1; shift ;;
        --with-saved-vars) WITH_SAVED_VARS=1; shift ;;
        --keep-symlinks) KEEP_SYMLINKS=1; shift ;;
        --validate-only) VALIDATE_ONLY=1; shift ;;
        --help|-h) usage; exit 0 ;;
        *) echo "ERROR: unknown argument '$1'" >&2; exit 2 ;;
    esac
done

validate_manifest() {
    [ -f "$MANIFEST" ] || { echo "ERROR: manifest not found: $MANIFEST" >&2; return 2; }
    local count=0
    while IFS=$'\t' read -r name profile url ref subpath; do
        should_skip_manifest_row "$name" "$profile" && continue
        validate_local_mists_source "$name" "$url" "$subpath"
        count=$((count + 1))
    done < "$MANIFEST"
    if [ "$count" -eq 0 ]; then
        echo "ERROR: no installed Mists addons matched filter '$NAME_FILTER'" >&2
        return 2
    fi
    echo "$count installed Mists addon row(s) validated from $MANIFEST"
}

should_skip_manifest_row() {
    local name="${1:-}" profile="${2:-}"
    [[ "$name" =~ ^# ]] && return 0
    [ -z "$name" ] && return 0
    [ "$name" = "name" ] && return 0
    [ "$profile" != "mists" ] && return 0
    [ -n "$NAME_FILTER" ] && [ "$NAME_FILTER" != "$name" ] && return 0
    return 1
}

validate_local_mists_source() {
    local name="$1" url="$2" subpath="$3"
    if ! is_local_source "$url" && ! is_manifest_managed_source "$url"; then
        echo "ERROR: Mists addon $name must use local: or mists-addon: source, got $url" >&2
        return 2
    fi
    local src
    src="$(resolve_addon_source_root "$name" mists "$url" "$REPO_ROOT/vendor/addons")/$subpath"
    [ -d "$src" ] || { echo "ERROR: Mists addon source missing: $src" >&2; return 2; }
}

install_symlink() {
    local name="$1" url="$2" subpath="$3"
    local src
    src="$(resolve_addon_source_root "$name" mists "$url" "$REPO_ROOT/vendor/addons")/$subpath"
    local dst="$ADDONS_DIR/$name"
    [ -d "$src" ] || { echo "ERROR: $src not found" >&2; return 1; }
    [ -L "$dst" ] && rm "$dst"
    [ -e "$dst" ] && { echo "ERROR: $dst exists and is not a symlink" >&2; return 1; }
    ln -s "$src" "$dst"
}

install_compat_shims() {
    local name="$1"
    local compat_dir="$COMPAT_ROOT/$name"
    [ -d "$compat_dir" ] || return 0
    local shim
    for shim in "$compat_dir"/*/; do
        [ -d "$shim" ] || continue
        local shim_name
        shim_name="$(basename "$shim")"
        local dst="$ADDONS_DIR/$shim_name"
        [ -L "$dst" ] && rm "$dst"
        [ -e "$dst" ] && { echo "ERROR: $dst exists and is not a symlink" >&2; return 1; }
        ln -s "$shim" "$dst"
        echo "  -> compat shim: $shim_name"
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
        remove_symlink "$(basename "$shim")"
    done
}

teardown_addon() {
    local name="$1"
    if [ "$KEEP_SYMLINKS" -eq 0 ]; then
        remove_symlink "$name"
        remove_compat_shims "$name"
    fi
}

run_addon_panels() {
    local name="$1" url="$2" subpath="$3"
    local addon_out_dir="$OUT_DIR/$name"
    local args=(--skip-build --with-addons --out-dir "$addon_out_dir")
    if [ "$WITH_SAVED_VARS" -eq 1 ]; then
        args+=(--with-saved-vars)
    fi
    if [ -n "$PANEL_FILTER" ]; then
        args+=(--panel "$PANEL_FILTER")
    fi

    echo ""
    echo "=== $name (mists panels) ==="
    ACTIVE_ADDON="$name"
    install_symlink "$name" "$url" "$subpath"
    install_compat_shims "$name"
    if WOW_SIM_BIN="$WOW_SIM_BIN" PANEL_VISUAL_METRICS_BIN="$PANEL_VISUAL_METRICS_BIN" \
            "$REPO_ROOT/scripts/mists-panel-parity.sh" "${args[@]}"; then
        teardown_addon "$name"
        ACTIVE_ADDON=""
        return 0
    fi
    teardown_addon "$name"
    ACTIVE_ADDON=""
    return 1
}

cleanup_active_addon() {
    if [ -n "$ACTIVE_ADDON" ]; then
        echo "ERROR: interrupted; removing addon symlinks for $ACTIVE_ADDON" >&2
        teardown_addon "$ACTIVE_ADDON"
    fi
}

validate_manifest
if [ "$VALIDATE_ONLY" -eq 1 ]; then
    exit 0
fi

mkdir -p "$OUT_DIR"
if [ "$SKIP_BUILD" -eq 0 ]; then
    cargo build --bin wow-sim --no-default-features --features "sound,gui,casc,client-mists"
fi
trap cleanup_active_addon EXIT
trap 'cleanup_active_addon; exit 130' INT TERM

declare -i pass=0 fail=0
while IFS=$'\t' read -r name profile url ref subpath; do
    should_skip_manifest_row "$name" "$profile" && continue
    if run_addon_panels "$name" "$url" "$subpath"; then
        pass+=1
    else
        fail+=1
    fi
done < "$MANIFEST"

echo ""
echo "================================================"
echo "  panel parity passed: $pass    failed: $fail"
echo "================================================"

[ "$fail" -gt 0 ] && exit 1
exit 0
