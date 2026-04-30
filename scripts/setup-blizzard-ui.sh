#!/usr/bin/env bash
# Sparse-checkout a Blizzard UI source for the requested client profile,
# then create Interface/BlizzardUI/<Profile> symlink pointing at the checkout.
#
# Usage: ./scripts/setup-blizzard-ui.sh <profile> [ref]
#   profile: retail | wrath | mists
#   ref:     optional override; defaults to the profile's pinned ref below

set -euo pipefail

PROFILE="${1:-}"
REF_OVERRIDE="${2:-}"

if [ -z "$PROFILE" ]; then
    echo "Usage: $0 <profile> [ref]"
    echo "  profile: retail | wrath | mists | era"
    exit 2
fi

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Profile → repo URL + default ref + sparse-checkout paths.
# Refs are pinned SHAs as of 2026-04-29; pass [ref] to override.
case "$PROFILE" in
    retail)
        REPO_URL="https://github.com/Gethe/wow-ui-source.git"
        DEFAULT_REF="b062d332c0abafec23645d08dbeb7d24fe83b7f0"  # tag 12.0.5
        SPARSE_PATHS=("Interface/AddOns")
        SUBDIR="Retail"
        ;;
    wrath)
        REPO_URL="https://github.com/andrew6180/WoTLK-3.3.5-UI-Source.git"
        DEFAULT_REF="27334191d9e774390e1df329adc2b077133a9c4f"  # master HEAD
        SPARSE_PATHS=("Interface/AddOns" "Interface/FrameXML")
        SUBDIR="Wrath"
        ;;
    mists)
        REPO_URL="https://github.com/Gethe/wow-ui-source.git"
        DEFAULT_REF="33d87412adad30d52eadcd5b334049788ebaae31"  # branch classic HEAD
        SPARSE_PATHS=("Interface/AddOns")
        SUBDIR="Mists"
        ;;
    era)
        REPO_URL="https://github.com/Gethe/wow-ui-source.git"
        DEFAULT_REF="e0099491e5ce94ef87c791b053f1e1509b5fd7ac"  # branch classic_era HEAD
        SPARSE_PATHS=("Interface/AddOns")
        SUBDIR="Era"
        ;;
    *)
        echo "ERROR: unknown profile '$PROFILE' (expected: retail | wrath | mists | era)"
        exit 2
        ;;
esac

REF="${REF_OVERRIDE:-$DEFAULT_REF}"
VENDOR_DIR="$REPO_ROOT/vendor/wow-ui-source-$PROFILE"
BLIZZARD_UI_DIR="$REPO_ROOT/Interface/BlizzardUI"
PROFILE_LINK="$BLIZZARD_UI_DIR/$SUBDIR"

# Convert legacy single-symlink BlizzardUI → directory layout if needed.
if [ -L "$BLIZZARD_UI_DIR" ]; then
    echo "Converting Interface/BlizzardUI from symlink to directory..."
    rm "$BLIZZARD_UI_DIR"
fi
mkdir -p "$BLIZZARD_UI_DIR"

# Clone or update the vendor checkout.
if [ -d "$VENDOR_DIR/.git" ]; then
    echo "Updating $VENDOR_DIR to ref $REF..."
    cd "$VENDOR_DIR"
    git fetch --depth=1 origin "$REF" 2>/dev/null || git fetch origin
    git checkout --detach "$REF"
    cd "$REPO_ROOT"
else
    echo "Cloning $REPO_URL @ $REF (sparse) into $VENDOR_DIR..."
    rm -rf "$VENDOR_DIR"
    git clone --filter=blob:none --no-checkout "$REPO_URL" "$VENDOR_DIR"
    cd "$VENDOR_DIR"
    git sparse-checkout init --cone
    git sparse-checkout set "${SPARSE_PATHS[@]}"
    git fetch --depth=1 origin "$REF"
    git checkout --detach "$REF"
    cd "$REPO_ROOT"
fi

# Refresh the per-profile symlink.
if [ -L "$PROFILE_LINK" ]; then
    rm "$PROFILE_LINK"
elif [ -d "$PROFILE_LINK" ]; then
    echo "ERROR: $PROFILE_LINK is a directory, not a symlink. Remove it first."
    exit 1
fi
ln -s "$VENDOR_DIR/Interface" "$PROFILE_LINK"

echo "Done: Interface/BlizzardUI/$SUBDIR → $VENDOR_DIR/Interface (ref $REF)"
