#!/usr/bin/env bash
# Shared source resolution for Mists addon harnesses.

classic_addon_fixture_root() {
    echo "${CLASSIC_ADDON_FIXTURE_ROOT:-$REPO_ROOT/tools/classic-addon-fixtures}"
}

mists_installed_addon_root() {
    echo "${MISTS_ADDON_ROOT:-${WOW_MISTS_ADDON_ROOT:-/syncthing/World of Warcraft/_classic_/Interface/AddOns}}"
}

is_local_source() {
    local url="$1"
    [[ "$url" == local:* || "$url" == /* ]]
}

is_manifest_managed_source() {
    local url="$1"
    [[ "$url" == mists-addon:* ]]
}

local_source_root() {
    local url="$1"
    if [[ "$url" == local:* ]]; then
        echo "${url#local:}"
    else
        echo "$url"
    fi
}

mists_managed_addon_name() {
    local url="$1"
    echo "${url#mists-addon:}"
}

mists_fixture_root() {
    local addon_name="$1"
    echo "$(classic_addon_fixture_root)/mists/$addon_name"
}

resolve_mists_managed_source_root() {
    local manifest_name="$1" url="$2"
    local addon_name
    addon_name="$(mists_managed_addon_name "$url")"
    local installed_root
    installed_root="$(mists_installed_addon_root)/$addon_name"
    if [ -d "$installed_root" ]; then
        echo "$installed_root"
        return 0
    fi

    local fixture_root
    fixture_root="$(mists_fixture_root "$manifest_name")"
    [ -d "$fixture_root" ] || {
        echo "ERROR: no installed Mists addon at $installed_root and no fixture at $fixture_root" >&2
        return 1
    }
    echo "$fixture_root"
}

resolve_addon_source_root() {
    local name="$1" profile="$2" url="$3" vendor_dir="$4"
    if is_manifest_managed_source "$url"; then
        [ "$profile" = "mists" ] || {
            echo "ERROR: managed source '$url' is only supported for Mists rows" >&2
            return 1
        }
        resolve_mists_managed_source_root "$name" "$url"
        return
    fi

    if is_local_source "$url"; then
        local_source_root "$url"
        return
    fi

    echo "$vendor_dir/$name"
}
