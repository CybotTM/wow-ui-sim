#!/usr/bin/env bash
# Bring a freshly-created git worktree up to a buildable state.
#
# `Interface/BlizzardUI` and `vendor/` are gitignored so worktrees start
# without any Blizzard UI source. Without these symlinks the addon loader
# fatals at startup ("Interface/BlizzardUI/<Profile>/AddOns is missing")
# and benchmarks like `bench_talents` silently fail with nil-call errors
# on globals like `PlayerSpellsUtil.ToggleClassTalentFrame`.
#
# Run this once per new worktree, immediately after `git worktree add`.
# Idempotent — safe to re-run.
#
# Usage: ./scripts/init-worktree.sh [profile1 profile2 ...]
#   profiles default to: retail wrath mists

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PROFILES=("$@")
if [ ${#PROFILES[@]} -eq 0 ]; then
    PROFILES=(retail wrath mists)
fi

start=$(date +%s)
for profile in "${PROFILES[@]}"; do
    "$REPO_ROOT/scripts/setup-blizzard-ui.sh" "$profile"
done
elapsed=$(($(date +%s) - start))

profile_list="$(IFS=,; echo "${PROFILES[*]}")"
echo ""
echo "Worktree initialized in ${elapsed}s — Interface/BlizzardUI/{$profile_list} symlinks created."
