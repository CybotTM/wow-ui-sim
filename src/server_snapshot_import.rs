//! Import action bar state captured by the ServerSnapshot addon.
//!
//! The addon writes account SavedVariables from a real client. During simulator
//! startup we load that file, pick the requested/latest character snapshot, and
//! seed the simulator action bar model before Blizzard action buttons initialize.

use crate::lua_api::WowLuaEnv;
use crate::saved_variables::SavedVariablesManager;
use std::collections::HashMap;

const SERVER_SNAPSHOT_ADDON: &str = "ServerSnapshot";

const APPLY_SNAPSHOT_LUA: &str = r#"
local db = rawget(_G, "ServerSnapshotDB")
if type(db) ~= "table" or type(db.characters) ~= "table" then
    return 0
end

local function chooseSnapshot()
    local key = db.lastCharacterKey
    if type(key) == "string" and type(db.characters[key]) == "table" then
        return db.characters[key]
    end

    local newestSnapshot = nil
    local newestCapturedAt = nil
    for _, snapshot in pairs(db.characters) do
        if type(snapshot) == "table" then
            local capturedAt = tonumber(snapshot.capturedAt) or 0
            if newestSnapshot == nil or capturedAt > newestCapturedAt then
                newestSnapshot = snapshot
                newestCapturedAt = capturedAt
            end
        end
    end
    return newestSnapshot
end

local snapshot = chooseSnapshot()
if type(snapshot) ~= "table" then
    return 0
end

local keybindings = snapshot.keybindings
if type(keybindings) == "table" then
    local keys = keybindings.keys
    local sampledKeys = {}
    if type(keys) == "table" then
        for key, action in pairs(keys) do
            if type(key) == "string" and type(action) == "string" then
                sampledKeys[key] = true
                SetBinding(key, action)
            end
        end
    end

    local entries = keybindings.entries
    if type(entries) == "table" then
        for _, entry in pairs(entries) do
            if type(entry) == "table" and type(entry.action) == "string" and type(entry.keys) == "table" then
                for _, key in ipairs(entry.keys) do
                    if type(key) == "string" and key ~= "" and not sampledKeys[key] then
                        SetBinding(key, entry.action)
                    end
                end
            end
        end
    end
end

local actionBars = snapshot.actionBars
local slots = type(actionBars) == "table" and actionBars.slots or nil
if type(slots) ~= "table" then
    return 0
end

A_Admin.ClearActionBars()

local imported = 0
for slot, entry in pairs(slots) do
    local slotNumber = tonumber(slot)
    if slotNumber ~= nil and type(entry) == "table" and entry.empty ~= true then
        local actionType = entry.type
        local spellID = tonumber(entry.spellID or entry.id)
        if actionType == "spell" and spellID ~= nil then
            A_Admin.SetActionSlot(slotNumber, spellID)
            imported = imported + 1
        end
    end
end

return imported
"#;

const SNAPSHOT_ADDON_ENABLE_OVERRIDES_LUA: &str = r#"
local db = rawget(_G, "ServerSnapshotDB")
if type(db) ~= "table" or type(db.characters) ~= "table" then
    return ""
end

local function chooseSnapshot()
    local key = db.lastCharacterKey
    if type(key) == "string" and type(db.characters[key]) == "table" then
        return db.characters[key]
    end

    local newestSnapshot = nil
    local newestCapturedAt = nil
    for _, snapshot in pairs(db.characters) do
        if type(snapshot) == "table" then
            local capturedAt = tonumber(snapshot.capturedAt) or 0
            if newestSnapshot == nil or capturedAt > newestCapturedAt then
                newestSnapshot = snapshot
                newestCapturedAt = capturedAt
            end
        end
    end
    return newestSnapshot
end

local snapshot = chooseSnapshot()
local entries = type(snapshot) == "table"
    and type(snapshot.addons) == "table"
    and snapshot.addons.entries
    or nil
if type(entries) ~= "table" then
    return ""
end

local rows = {}
for name, entry in pairs(entries) do
    if type(name) == "string" and type(entry) == "table" and type(entry.enabled) == "boolean" then
        table.insert(rows, name .. "=" .. (entry.enabled and "1" or "0"))
    end
end
return table.concat(rows, "\n")
"#;

/// Apply an already-loaded `ServerSnapshotDB` global to the simulator action bar model.
pub fn apply_loaded_snapshot(env: &WowLuaEnv) -> crate::Result<i64> {
    env.eval(APPLY_SNAPSHOT_LUA)
}

pub fn load_addon_enable_overrides(
    env: &WowLuaEnv,
    saved_vars: &mut SavedVariablesManager,
) -> crate::Result<HashMap<String, bool>> {
    env.loader_env()
        .with_state(|state| saved_vars.load_wtf_for_addon(state, SERVER_SNAPSHOT_ADDON))?;
    let text: String = env.eval(SNAPSHOT_ADDON_ENABLE_OVERRIDES_LUA)?;
    Ok(parse_addon_enable_overrides_text(&text))
}

fn parse_addon_enable_overrides_text(text: &str) -> HashMap<String, bool> {
    text.lines()
        .filter_map(|line| {
            let (name, enabled) = line.split_once('=')?;
            match enabled {
                "1" => Some((name.to_string(), true)),
                "0" => Some((name.to_string(), false)),
                _ => None,
            }
        })
        .collect()
}

/// Load ServerSnapshot SavedVariables from the configured WTF source and apply them.
///
/// Returns the number of spell action slots imported. Missing saved variables are a
/// clean no-op.
pub fn load_from_saved_variables(
    env: &WowLuaEnv,
    saved_vars: &mut SavedVariablesManager,
) -> crate::Result<i64> {
    let loaded = env
        .loader_env()
        .with_state(|state| saved_vars.load_wtf_for_addon(state, SERVER_SNAPSHOT_ADDON))?;
    if loaded == 0 {
        return Ok(0);
    }
    apply_loaded_snapshot(env)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_addon_enable_overrides_reads_snapshot_addon_entries() {
        let env = WowLuaEnv::new().expect("env");
        env.exec(
            r#"
            ServerSnapshotDB = {
                lastCharacterKey = "Realm/Character",
                characters = {
                    ["Realm/Character"] = {
                        capturedAt = 1,
                        addons = {
                            entries = {
                                EllesmereUI = { enabled = false },
                                BetterBlizzFrames = { enabled = true },
                            },
                        },
                    },
                },
            }
            "#,
        )
        .expect("seed snapshot");
        let mut saved_vars = SavedVariablesManager::new();

        let overrides = load_addon_enable_overrides(&env, &mut saved_vars).expect("overrides");

        assert_eq!(overrides.get("EllesmereUI"), Some(&false));
        assert_eq!(overrides.get("BetterBlizzFrames"), Some(&true));
    }

    #[test]
    fn load_addon_enable_overrides_tolerates_missing_snapshot() {
        let env = WowLuaEnv::new().expect("env");
        let mut saved_vars = SavedVariablesManager::new();

        let overrides = load_addon_enable_overrides(&env, &mut saved_vars).expect("overrides");

        assert!(overrides.is_empty());
    }

    #[test]
    fn apply_loaded_snapshot_imports_keybindings_before_action_bars() {
        let env = WowLuaEnv::new().expect("env");
        env.exec(
            r#"
            ServerSnapshotDB = {
                lastCharacterKey = "Realm/Character",
                characters = {
                    ["Realm/Character"] = {
                        capturedAt = 1,
                        keybindings = {
                            keys = {
                                F1 = "",
                                ["CTRL-M"] = "TOGGLEWORLDMAP",
                            },
                        },
                        actionBars = { slots = {} },
                    },
                },
            }
            "#,
        )
        .expect("seed snapshot");

        let imported = apply_loaded_snapshot(&env).expect("apply snapshot");
        let (f1_action, map_action): (String, String) = env
            .eval(r#"return GetBindingAction("F1"), GetBindingAction("CTRL-M")"#)
            .expect("read bindings");

        assert_eq!(imported, 0);
        assert_eq!(f1_action, "");
        assert_eq!(map_action, "TOGGLEWORLDMAP");
    }

    #[test]
    fn apply_loaded_snapshot_keeps_sampled_keybinding_over_entry_collision() {
        let env = WowLuaEnv::new().expect("env");
        env.exec(
            r#"
            ServerSnapshotDB = {
                lastCharacterKey = "Realm/Character",
                characters = {
                    ["Realm/Character"] = {
                        capturedAt = 1,
                        keybindings = {
                            keys = {
                                C = "TOGGLECHARACTER0",
                            },
                            entries = {
                                {
                                    action = "HOUSING_TOGGLEDECORSNAPMODE",
                                    keys = { "C" },
                                },
                                {
                                    action = "TOGGLEWORLDMAP",
                                    keys = { "CTRL-M" },
                                },
                            },
                        },
                        actionBars = { slots = {} },
                    },
                },
            }
            "#,
        )
        .expect("seed snapshot");

        apply_loaded_snapshot(&env).expect("apply snapshot");
        let (c_action, map_action): (String, String) = env
            .eval(r#"return GetBindingAction("C"), GetBindingAction("CTRL-M")"#)
            .expect("read bindings");

        assert_eq!(c_action, "TOGGLECHARACTER0");
        assert_eq!(map_action, "TOGGLEWORLDMAP");
    }
}
