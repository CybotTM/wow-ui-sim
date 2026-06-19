#!/usr/bin/env bash
# Populate the active profile's Blizzard UI cache from local WoW CASC data.
#
# Usage: ./scripts/setup-blizzard-ui.sh [profile]
#   profile is accepted for compatibility with old worktree setup scripts, but
#   the active Cargo client feature selects the cache profile.

set -euo pipefail

PROFILE="${1:-}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

if [ -n "$PROFILE" ]; then
    echo "Requested profile '$PROFILE'; wow-cli uses the active Cargo client feature for cache sync."
fi

cd "$REPO_ROOT"
cargo run --quiet --bin wow-cli -- casc sync-blizzard-ui
