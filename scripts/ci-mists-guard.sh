#!/usr/bin/env bash
# CI guard for the Mists profile parity contract.
#
# Enforces the zero `lua-errors` baseline and runs the scripted Mists panel
# parity runner. The panel runner captures artifacts under
# target/mists-panel-parity/ for upload by CI.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LUA_ERRORS_OUT="$REPO_ROOT/target/mists-ci-lua-errors.json"
PANEL_ARGS=()
SKIP_BUILD=0

usage() {
    sed -n '2,/^set -euo/p' "$0" | sed 's/^# \?//;$d'
}

while [ $# -gt 0 ]; do
    case "$1" in
        --panel)
            PANEL_ARGS+=(--panel "$2")
            shift 2
            ;;
        --skip-build)
            SKIP_BUILD=1
            shift
            ;;
        --help|-h)
            usage
            exit 0
            ;;
        *)
            echo "ERROR: unknown argument '$1'" >&2
            exit 2
            ;;
    esac
done

mkdir -p "$REPO_ROOT/target"

if [ "$SKIP_BUILD" -eq 0 ]; then
    cargo build --bin wow-sim --no-default-features --features "sound,gui,casc,client-mists"
fi

WOW_SIM_NO_ADDONS=1 WOW_SIM_NO_SAVED_VARS=1 timeout 90 \
    "$REPO_ROOT/target/debug/wow-sim" --no-addons --no-saved-vars lua-errors \
    > "$LUA_ERRORS_OUT"

if [ "$(jq 'length' "$LUA_ERRORS_OUT")" -ne 0 ]; then
    echo "ERROR: Mists lua-errors baseline is not zero: $LUA_ERRORS_OUT" >&2
    jq . "$LUA_ERRORS_OUT" >&2
    exit 1
fi

"$REPO_ROOT/scripts/diff-lua-errors.sh" \
    "$REPO_ROOT/docs/baselines/mists-lua-errors.json" \
    "$LUA_ERRORS_OUT" \
    --exit-on-regression

"$REPO_ROOT/scripts/mists-panel-parity.sh" --skip-build "${PANEL_ARGS[@]}"
