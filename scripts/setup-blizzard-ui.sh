#!/usr/bin/env bash
# Sparse-checkout Blizzard UI source from Gethe/wow-ui-source into vendor/,
# then symlink Interface/BlizzardUI → the checkout's Interface/AddOns.
#
# Usage: ./scripts/setup-blizzard-ui.sh [TAG]
#   TAG defaults to 12.0.5

set -euo pipefail

TAG="${1:-12.0.5}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VENDOR_DIR="$REPO_ROOT/vendor/wow-ui-source"
SYMLINK="$REPO_ROOT/Interface/BlizzardUI"

if [ -d "$VENDOR_DIR" ]; then
    echo "Updating existing checkout to tag $TAG..."
    cd "$VENDOR_DIR"
    git fetch --depth=1 origin "refs/tags/$TAG:refs/tags/$TAG" 2>/dev/null || \
        git fetch --depth=1 origin tag "$TAG"
    git checkout "$TAG"
    cd "$REPO_ROOT"
else
    echo "Cloning wow-ui-source (sparse, tag $TAG)..."
    git clone --filter=blob:none --no-checkout --depth=1 --branch "$TAG" \
        https://github.com/Gethe/wow-ui-source.git "$VENDOR_DIR"
    cd "$VENDOR_DIR"
    git sparse-checkout init --cone
    git sparse-checkout set Interface/AddOns
    git checkout "$TAG"
    cd "$REPO_ROOT"
fi

# Create symlink
if [ -L "$SYMLINK" ]; then
    rm "$SYMLINK"
elif [ -d "$SYMLINK" ]; then
    echo "ERROR: $SYMLINK is a directory, not a symlink. Remove it first."
    exit 1
fi

ln -s "$VENDOR_DIR/Interface/AddOns" "$SYMLINK"
echo "Done: Interface/BlizzardUI → vendor/wow-ui-source/Interface/AddOns (tag $TAG)"
