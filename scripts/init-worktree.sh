#!/usr/bin/env bash
# Bring a freshly-created git worktree up to a buildable state.
#
# Blizzard UI source is stored under the user cache, not inside the worktree.
# This script syncs the active client profile cache from local WoW CASC data so
# benchmarks and startup paths can find Blizzard addon sources.
#
# Run this once per new worktree, immediately after `git worktree add`.
# Idempotent — safe to re-run.
#
# Usage: ./scripts/init-worktree.sh [profile1 profile2 ...]
#   profiles are accepted for compatibility; the active Cargo feature selects
#   which profile `wow-cli casc sync-blizzard-ui` populates.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PROFILES=("$@")
if [ ${#PROFILES[@]} -eq 0 ]; then
    PROFILES=(retail wrath mists era anniversary)
fi

start=$(date +%s)
"$REPO_ROOT/scripts/setup-blizzard-ui.sh" "${PROFILES[0]}"
elapsed=$(($(date +%s) - start))

profile_list="$(IFS=,; echo "${PROFILES[*]}")"
echo ""
echo "Worktree initialized in ${elapsed}s — Blizzard UI cache sync requested for {$profile_list}."
