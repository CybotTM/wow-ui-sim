#!/usr/bin/env bash
# Build and start wow-sim for live, PTR, or Mists.
#
# Usage:
#   scripts/start-profile.sh live [wow-sim args...]
#   scripts/start-profile.sh ptr [wow-sim args...]
#   scripts/start-profile.sh mists [wow-sim args...]
#   scripts/start-profile.sh all [wow-sim args...]
#
# Environment:
#   WOW_SIM_START_RELEASE=1  build release binaries instead of debug
#   WOW_SIM_START_FOREGROUND=1  run a single profile in the foreground
#   WOW_SIM_START_NO_BUILD=1  skip cargo build

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$REPO_ROOT/target/profile-runs"
PROFILE="${1:-}"

usage() {
    sed -n '2,/^set -euo/p' "$0" | sed 's/^# \?//;$d'
}

if [ -z "$PROFILE" ] || [ "$PROFILE" = "--help" ] || [ "$PROFILE" = "-h" ]; then
    usage
    exit 0
fi
shift

profile_feature() {
    case "$1" in
        live|retail) echo "client-retail" ;;
        ptr) echo "client-ptr" ;;
        mists) echo "client-mists" ;;
        *) echo "ERROR: unknown profile '$1' (expected live, ptr, mists, or all)" >&2; return 2 ;;
    esac
}

profile_label() {
    case "$1" in
        live|retail) echo "live" ;;
        ptr) echo "ptr" ;;
        mists) echo "mists" ;;
        *) echo "ERROR: unknown profile '$1' (expected live, ptr, mists, or all)" >&2; return 2 ;;
    esac
}

build_profile() {
    local feature="$1"
    local cargo_args=(build --bin wow-sim --no-default-features --features "sound,gui,casc,$feature")
    if [ "${WOW_SIM_START_RELEASE:-0}" = "1" ]; then
        cargo_args+=(--release)
    fi

    if [ "${WOW_SIM_START_NO_BUILD:-0}" != "1" ]; then
        cargo "${cargo_args[@]}"
    fi
}

binary_path() {
    if [ "${WOW_SIM_START_RELEASE:-0}" = "1" ]; then
        echo "$REPO_ROOT/target/release/wow-sim"
    else
        echo "$REPO_ROOT/target/debug/wow-sim"
    fi
}

start_profile() {
    local requested="$1"
    shift
    local label feature log_file pid_file bin
    label="$(profile_label "$requested")"
    feature="$(profile_feature "$requested")"
    log_file="$OUT_DIR/$label.log"
    pid_file="$OUT_DIR/$label.pid"

    mkdir -p "$OUT_DIR"
    cd "$REPO_ROOT"

    build_profile "$feature"
    bin="$(binary_path)"

    if [ "${WOW_SIM_START_FOREGROUND:-0}" = "1" ]; then
        exec "$bin" "$@"
    fi

    "$bin" "$@" >"$log_file" 2>&1 &
    printf '%s\n' "$!" >"$pid_file"
    printf '%-5s pid=%s log=%s\n' "$label" "$(<"$pid_file")" "$log_file"
}

if [ "$PROFILE" = "all" ]; then
    if [ "${WOW_SIM_START_FOREGROUND:-0}" = "1" ]; then
        echo "ERROR: WOW_SIM_START_FOREGROUND=1 only works with a single profile" >&2
        exit 2
    fi
    for profile in live ptr mists; do
        start_profile "$profile" "$@"
    done
else
    start_profile "$PROFILE" "$@"
fi
