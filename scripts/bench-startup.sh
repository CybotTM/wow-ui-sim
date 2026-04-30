#!/usr/bin/env bash
# Measure wow-sim startup wall-clock per client profile (Phase 9.1).
#
# For each profile: build wow-sim once (excluded from the timing), then run
# `lua-errors` N=5 times back-to-back and report min/median/max wall-clock.
# `lua-errors` covers the full startup path (Blizzard UI load + addon load
# if not suppressed + startup events dispatched + JSON emit), so it's a fair
# end-to-end proxy for "ready".
#
# Usage:
#   scripts/bench-startup.sh                       # all profiles
#   scripts/bench-startup.sh wrath mists           # subset
#   scripts/bench-startup.sh --runs 10             # custom run count
#   scripts/bench-startup.sh --release             # release builds
#   scripts/bench-startup.sh --no-addons           # skip third-party addons
#   scripts/bench-startup.sh --tsv > out.tsv       # tab-separated output
#
# Exit codes:
#   0  measurements completed (regardless of error counts)
#   1  build failed for at least one profile

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RUNS=5
PROFILE="dev"
NO_ADDONS=0
TSV=0
PROFILES_REQUESTED=()

while [ $# -gt 0 ]; do
    case "$1" in
        --runs)       RUNS="$2"; shift 2 ;;
        --release)    PROFILE="release"; shift ;;
        --no-addons)  NO_ADDONS=1; shift ;;
        --tsv)        TSV=1; shift ;;
        --help|-h)
            sed -n '2,/^set -euo/p' "$0" | sed 's/^# \?//;$d'
            exit 0
            ;;
        -*)           echo "ERROR: unknown flag '$1'" >&2; exit 2 ;;
        *)            PROFILES_REQUESTED+=("$1"); shift ;;
    esac
done

if [ ${#PROFILES_REQUESTED[@]} -eq 0 ]; then
    PROFILES_REQUESTED=(retail wrath mists era anniversary)
fi

target_dir="$REPO_ROOT/target/$([ "$PROFILE" = "release" ] && echo release || echo debug)"
build_flag=""
[ "$PROFILE" = "release" ] && build_flag="--release"

env_prefix="WOW_SIM_NO_SAVED_VARS=1"
[ "$NO_ADDONS" -eq 1 ] && env_prefix="$env_prefix WOW_SIM_NO_ADDONS=1"

if [ "$TSV" -eq 1 ]; then
    printf "profile\tbuild\truns\tmin\tmedian\tmax\n"
else
    printf "%-13s %-7s %-5s %-7s %-7s %-7s\n" profile build runs min median max
    printf "%-13s %-7s %-5s %-7s %-7s %-7s\n" --------- ------- ----- ------- ------- -------
fi

build_failed=0
for prof in "${PROFILES_REQUESTED[@]}"; do
    cd "$REPO_ROOT"
    if ! cargo build $build_flag --bin wow-sim --no-default-features \
            --features "sound,gui,casc,client-$prof" >/dev/null 2>&1; then
        echo "  ✗ build failed for $prof" >&2
        build_failed=1
        continue
    fi

    times_file=$(mktemp)
    for _ in $(seq 1 "$RUNS"); do
        start=$(date +%s.%N)
        eval "$env_prefix timeout 90 \"$target_dir/wow-sim\" lua-errors > /dev/null 2>&1" || true
        end=$(date +%s.%N)
        echo "$end - $start" | bc >> "$times_file"
    done

    sorted=$(sort -n "$times_file")
    min=$(echo "$sorted" | head -1)
    max=$(echo "$sorted" | tail -1)
    middle=$(( (RUNS + 1) / 2 ))
    median=$(echo "$sorted" | sed -n "${middle}p")
    rm -f "$times_file"

    if [ "$TSV" -eq 1 ]; then
        printf "%s\t%s\t%d\t%.2f\t%.2f\t%.2f\n" "$prof" "$PROFILE" "$RUNS" "$min" "$median" "$max"
    else
        printf "%-13s %-7s %-5d %-7.2f %-7.2f %-7.2f\n" "$prof" "$PROFILE" "$RUNS" "$min" "$median" "$max"
    fi
done

[ "$build_failed" -eq 1 ] && exit 1
exit 0
