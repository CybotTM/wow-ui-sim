#!/usr/bin/env bash
# Compare two lua-errors.json snapshots and report what regressed / got fixed.
#
# Used by:
#   - .github/workflows/test.yml — diff the PR's lua-errors against the
#     committed baseline at docs/baselines/<profile>-lua-errors.json
#   - scripts/test-classic-addons.sh — diff the addon run against the
#     profile baseline to compute "addon-induced errors"
#   - manual debugging: scripts/diff-lua-errors.sh BASELINE NEW
#
# Compares the message-set (jq '.[].message'). Regressions = messages in
# NEW but not BASELINE. Fixes = messages in BASELINE but not NEW.
#
# Usage:
#   scripts/diff-lua-errors.sh BASELINE NEW [--quiet] [--exit-on-regression]
#
# Flags:
#   --quiet                 print only counts ("regressed=N fixed=M")
#   --exit-on-regression    exit 1 if any regression detected (default: 0)
#
# Exit codes:
#   0   no regressions OR --exit-on-regression not set
#   1   regressions detected AND --exit-on-regression set
#   2   one or both inputs missing or unreadable

set -euo pipefail

BASELINE="${1:-}"
NEW="${2:-}"
QUIET=0
FAIL_ON_REGRESSION=0

for arg in "${@:3}"; do
    case "$arg" in
        --quiet)              QUIET=1 ;;
        --exit-on-regression) FAIL_ON_REGRESSION=1 ;;
        *) echo "ERROR: unknown flag '$arg'" >&2; exit 2 ;;
    esac
done

if [ -z "$BASELINE" ] || [ -z "$NEW" ]; then
    echo "Usage: $0 BASELINE NEW [--quiet] [--exit-on-regression]" >&2
    exit 2
fi

extract_messages() {
    local file="$1"
    if [ ! -f "$file" ] || [ ! -s "$file" ]; then
        echo ""
        return
    fi
    # Lua-error messages contain embedded newlines from "stack traceback:".
    # `jq -r` would emit those as real newlines, splitting one message
    # across many "unique" lines. Escape newlines to \n so each message
    # is a single sortable line.
    jq -r '.[].message | gsub("\n"; "\\n")' "$file" 2>/dev/null | sort -u
}

baseline_msgs=$(mktemp)
new_msgs=$(mktemp)
trap 'rm -f "$baseline_msgs" "$new_msgs"' EXIT

extract_messages "$BASELINE" > "$baseline_msgs"
extract_messages "$NEW" > "$new_msgs"

baseline_count=$(grep -c . "$baseline_msgs" || true)
new_count=$(grep -c . "$new_msgs" || true)

# Set differences. Empty inputs → comm prints nothing → counts are 0.
regressions=$(comm -13 "$baseline_msgs" "$new_msgs" || true)
fixes=$(comm -23 "$baseline_msgs" "$new_msgs" || true)
regress_count=$(echo "$regressions" | grep -c . || true)
fix_count=$(echo "$fixes" | grep -c . || true)

if [ "$QUIET" -eq 1 ]; then
    echo "regressed=$regress_count fixed=$fix_count baseline=$baseline_count current=$new_count"
else
    echo "Baseline: $baseline_count distinct errors  ($BASELINE)"
    echo "Current:  $new_count distinct errors  ($NEW)"
    echo ""
    if [ "$fix_count" -gt 0 ]; then
        echo "Fixed (−$fix_count):"
        echo "$fixes" | sed 's/^/  - /; s/\\n.*//' | head -40
        [ "$fix_count" -gt 40 ] && echo "  ... and $((fix_count - 40)) more"
        echo ""
    fi
    if [ "$regress_count" -gt 0 ]; then
        echo "Regressed (+$regress_count):"
        echo "$regressions" | sed 's/^/  + /; s/\\n.*//' | head -40
        [ "$regress_count" -gt 40 ] && echo "  ... and $((regress_count - 40)) more"
        echo ""
    fi
    if [ "$regress_count" -eq 0 ] && [ "$fix_count" -eq 0 ]; then
        echo "No changes."
    fi
fi

if [ "$FAIL_ON_REGRESSION" -eq 1 ] && [ "$regress_count" -gt 0 ]; then
    exit 1
fi
exit 0
