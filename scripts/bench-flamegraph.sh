#!/usr/bin/env bash
# Per-profile flamegraph generation (Phase 9.4).
#
# Wraps the wow-sim binary in `cargo flamegraph` (cargo-flamegraph plugin,
# which uses perf record + inferno-flamegraph under the hood) and saves
# the SVG to target/flamegraphs/<profile>.svg.
#
# Defaults to release builds because debug optimization noise dominates
# the call graph and obscures real hot spots.
#
# Usage:
#   scripts/bench-flamegraph.sh                   # all profiles
#   scripts/bench-flamegraph.sh wrath             # one profile
#   scripts/bench-flamegraph.sh --debug           # debug build (cheaper, noisier)
#   scripts/bench-flamegraph.sh --command lua-errors  # default; alternatives: dump-tree, screenshot
#
# Requirements:
#   - cargo-flamegraph (cargo install flamegraph)
#   - perf (linux-tools)
#   - kernel.perf_event_paranoid <= 1, OR run with sudo. On Arch:
#       sudo sysctl kernel.perf_event_paranoid=1
#
# Exit codes:
#   0  flamegraphs generated for all requested profiles
#   1  build / perf record / flamegraph generation failed for at least one profile
#   2  cargo-flamegraph or perf not on PATH

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_DIR="$REPO_ROOT/target/flamegraphs"
BUILD_PROFILE="release"
SUBCOMMAND="lua-errors"
PROFILES_REQUESTED=()

while [ $# -gt 0 ]; do
    case "$1" in
        --debug)    BUILD_PROFILE="dev"; shift ;;
        --command)  SUBCOMMAND="$2"; shift 2 ;;
        --help|-h)
            sed -n '2,/^set -euo/p' "$0" | sed 's/^# \?//;$d'
            exit 0
            ;;
        -*)         echo "ERROR: unknown flag '$1'" >&2; exit 2 ;;
        *)          PROFILES_REQUESTED+=("$1"); shift ;;
    esac
done

if [ ${#PROFILES_REQUESTED[@]} -eq 0 ]; then
    PROFILES_REQUESTED=(retail wrath mists era anniversary)
fi

if ! command -v cargo-flamegraph >/dev/null 2>&1; then
    echo "ERROR: cargo-flamegraph not found. Install: cargo install flamegraph" >&2
    exit 2
fi
if ! command -v perf >/dev/null 2>&1; then
    echo "ERROR: perf not found. Install linux-tools / perf." >&2
    exit 2
fi

paranoid=$(cat /proc/sys/kernel/perf_event_paranoid 2>/dev/null || echo 4)
if [ "$paranoid" -gt 1 ]; then
    echo "WARNING: kernel.perf_event_paranoid=$paranoid — perf may need sudo." >&2
    echo "  Lower with: sudo sysctl kernel.perf_event_paranoid=1" >&2
fi

mkdir -p "$OUT_DIR"

build_failed=0
for prof in "${PROFILES_REQUESTED[@]}"; do
    cd "$REPO_ROOT"
    out_svg="$OUT_DIR/$prof.svg"

    echo ""
    echo "=== $prof ($BUILD_PROFILE / $SUBCOMMAND) ==="

    # cargo-flamegraph builds release by default (--release is a no-op).
    # `--dev` selects the debug profile when requested.
    cargo_args=(--bin wow-sim --no-default-features
                --features "sound,gui,casc,client-$prof"
                --output "$out_svg")
    [ "$BUILD_PROFILE" = "dev" ] && cargo_args+=(--dev)

    # Run from the repo root: wow-sim's loader resolves `./Interface/...`
    # relative to cwd. perf.data also lands in cwd; we move it to OUT_DIR
    # afterwards so the artifacts live together.
    if ! WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 \
            cargo flamegraph "${cargo_args[@]}" -- "$SUBCOMMAND" \
            2>&1 | tail -10; then
        echo "  ✗ flamegraph generation failed for $prof"
        [ -f "$REPO_ROOT/perf.data" ] && mv "$REPO_ROOT/perf.data" "$OUT_DIR/$prof.perf.data"
        build_failed=1
        continue
    fi

    [ -f "$REPO_ROOT/perf.data" ] && mv "$REPO_ROOT/perf.data" "$OUT_DIR/$prof.perf.data"

    if [ -f "$out_svg" ]; then
        size=$(stat -c%s "$out_svg" 2>/dev/null || echo 0)
        echo "  ✓ $out_svg ($((size / 1024)) KB)"
    else
        echo "  ✗ expected $out_svg not produced"
        build_failed=1
    fi
done

[ "$build_failed" -eq 1 ] && exit 1
exit 0
