//! Import action bar state captured by the ServerSnapshot addon.
//!
//! The addon writes account SavedVariables from a real client. During simulator
//! startup we load that file, pick the requested/latest character snapshot, and
//! seed the simulator action bar model before Blizzard action buttons initialize.

use crate::lua_api::WowLuaEnv;
use crate::saved_variables::SavedVariablesManager;

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

/// Apply an already-loaded `ServerSnapshotDB` global to the simulator action bar model.
pub fn apply_loaded_snapshot(env: &WowLuaEnv) -> crate::Result<i64> {
    env.eval(APPLY_SNAPSHOT_LUA)
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
