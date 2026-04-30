#!/usr/bin/env bash
# Capture per-profile memory + widget-count snapshots (Phase 9.3).
#
# For each profile:
#   1. Build wow-sim (excluded from measurement)
#   2. Launch `wow-sim dump-tree` with --exec-lua appending a marker
#      line that reports `collectgarbage("count")` (rilua heap, KB)
#   3. While the process is running, sample /proc/<pid>/status every
#      100ms to track peak VmRSS
#   4. Parse the dump-tree output to count widgets by type
#
# Reports per profile:
#   heap_kb       — rilua heap at end-of-startup (Lua-managed memory only)
#   rss_peak_kb   — process resident-set-size peak during the run
#   widgets_total — total widget count (Frame/Texture/FontString/etc.)
#
# These are the three numbers the plan asked for; together they give a
# rough picture of where memory goes (rilua heap = Lua tables/strings;
# rss − heap_kb ≈ Rust-side widget tree, atlas, asset cache, libraries).
#
# Usage:
#   scripts/bench-memory.sh                   # all profiles
#   scripts/bench-memory.sh wrath mists       # subset
#   scripts/bench-memory.sh --tsv             # tab-separated for diff/dashboard
#   scripts/bench-memory.sh --no-addons       # skip third-party addons
#
# Exit codes:
#   0  measurements completed
#   1  build failed for at least one profile

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TSV=0
NO_ADDONS=0
PROFILES_REQUESTED=()

while [ $# -gt 0 ]; do
    case "$1" in
        --tsv)        TSV=1; shift ;;
        --no-addons)  NO_ADDONS=1; shift ;;
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

env_prefix="WOW_SIM_NO_SAVED_VARS=1"
[ "$NO_ADDONS" -eq 1 ] && env_prefix="$env_prefix WOW_SIM_NO_ADDONS=1"

EXEC_LUA='io.stdout:write("__METRIC__ heap_kb="..tostring(collectgarbage("count")).."\n"); io.stdout:flush()'

# Sample /proc/<pid>/status in the background while $1 (pid) is alive.
# Writes peak VmRSS in KB to $2 (output file).
sample_rss_peak() {
    local pid="$1" out="$2"
    local peak=0 cur
    while kill -0 "$pid" 2>/dev/null; do
        cur=$(awk '/^VmRSS:/ {print $2}' "/proc/$pid/status" 2>/dev/null || echo 0)
        [ -n "$cur" ] && [ "$cur" -gt "$peak" ] && peak=$cur
        sleep 0.1
    done
    echo "$peak" > "$out"
}

if [ "$TSV" -eq 1 ]; then
    printf "profile\theap_kb\trss_peak_kb\twidgets_total\n"
else
    printf "%-13s %-10s %-13s %-13s\n" profile heap_kb rss_peak_kb widgets_total
    printf "%-13s %-10s %-13s %-13s\n" --------- ------- ----------- -------------
fi

build_failed=0
for prof in "${PROFILES_REQUESTED[@]}"; do
    cd "$REPO_ROOT"
    if ! cargo build --bin wow-sim --no-default-features \
            --features "sound,gui,casc,client-$prof" >/dev/null 2>&1; then
        echo "  ✗ build failed for $prof" >&2
        build_failed=1
        continue
    fi

    out_file=$(mktemp)
    rss_file=$(mktemp)

    extra_env=""
    [ "$NO_ADDONS" -eq 1 ] && extra_env="WOW_SIM_NO_ADDONS=1"
    # Launch wow-sim directly (no shell wrapper) so $! captures its real
    # PID. Otherwise /proc/<pid>/status reports the wrapping subshell's
    # RSS, which is tiny and useless for our purposes.
    WOW_SIM_NO_SAVED_VARS=1 $extra_env \
        "$REPO_ROOT/target/debug/wow-sim" \
        --no-saved-vars --exec-lua "$EXEC_LUA" dump-tree \
        > "$out_file" 2>/dev/null &
    sim_pid=$!
    sample_rss_peak "$sim_pid" "$rss_file" &
    sampler_pid=$!
    wait "$sim_pid" || true
    wait "$sampler_pid" || true

    heap=$(grep -oE '__METRIC__ heap_kb=[0-9.]+' "$out_file" | head -1 | sed 's/.*=//')
    rss=$(cat "$rss_file" 2>/dev/null || echo 0)
    widgets=$(grep -cE '^\s*[A-Za-z_][A-Za-z0-9_]*[[:space:]]*\[(Frame|Button|Texture|FontString|EditBox|StatusBar|MessageFrame|CheckButton|ScrollFrame|GameTooltip|ModelScene|WorldFrame|Slider)\]' "$out_file" || true)

    rm -f "$out_file" "$rss_file"

    if [ "$TSV" -eq 1 ]; then
        printf "%s\t%s\t%s\t%d\n" "$prof" "${heap:-0}" "${rss:-0}" "${widgets:-0}"
    else
        printf "%-13s %-10s %-13s %-13d\n" "$prof" "${heap:-?}" "${rss:-?}" "${widgets:-0}"
    fi
done

[ "$build_failed" -eq 1 ] && exit 1
exit 0
