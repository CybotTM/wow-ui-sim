//! Explicit empty-state queries exposed after removal of C_* synthesis.
//!
//! Missing systems: quest-session coordination, client scenes, frame-manager
//! visibility policy, lobby matchmaking, and covenant calling service snapshots.
//! Retire each namespace's defaults when that backing system supplies queries
//! (and calling request completion) through c_api. Ordinary quest/LFG/widget
//! state does not model these separate service contracts.
//!
//! Values below are simulator empty-state choices consistent with cached API
//! return types, not documented client defaults or native-parity evidence.
//! Commands without a modeled effect are deliberately absent, not accept-and-drop.

use crate::client_profile::{ACTIVE, ClientProfile};

const SESSION_AND_QUEUE_DEFAULTS: &str = r#"
local function install(namespace, name, fn)
    if rawget(namespace, name) == nil then
        namespace[name] = fn
    end
end

-- These namespaces are declared in all cached profiles except Wrath.
if rawget(_G, "C_QuestSession") == nil then C_QuestSession = {} end
if rawget(_G, "C_ClientScene") == nil then C_ClientScene = {} end
if rawget(_G, "C_LobbyMatchmakerInfo") == nil then C_LobbyMatchmakerInfo = {} end

install(C_QuestSession, "CanStart", function() return false end)
install(C_QuestSession, "CanStop", function() return false end)
install(C_QuestSession, "Exists", function() return false end)
install(C_QuestSession, "HasJoined", function() return false end)
install(C_QuestSession, "HasPendingCommand", function() return false end)
-- QuestSessionConstantsDocumentation.lua:13: QuestSessionCommand.None = 0.
install(C_QuestSession, "GetPendingCommand", function() return 0 end)
install(C_QuestSession, "GetAvailableSessionCommand", function() return 0 end)
install(C_QuestSession, "GetProposedMaxLevelForSession", function() return 0 end)
install(C_QuestSession, "GetSessionBeginDetails", function() return nil end)
install(C_QuestSession, "GetSuperTrackedQuest", function() return nil end)
install(C_LobbyMatchmakerInfo, "IsInQueue", function() return false end)
"#;

const MODERN_DEFAULTS: &str = r#"
local function install(namespace, name, fn)
    if rawget(namespace, name) == nil then
        namespace[name] = fn
    end
end

-- These functions/namespaces are declared only in Retail, PTR, and Forever.
install(C_ClientScene, "IsSceneTypeActive", function(desiredSceneType) return false end)
install(C_LobbyMatchmakerInfo, "GetQueueFromMainlineEnabled", function() return false end)
-- Queued-detail queries are not needed on the disabled/no-queue startup path.
-- Do not invent a non-nil PartyPlaylistEntry or silently accept queue commands.
if rawget(_G, "C_FrameManager") == nil then C_FrameManager = {} end
install(C_FrameManager, "GetFrameVisibilityState", function(frameType) return false end)

if rawget(_G, "C_CovenantCallings") == nil then C_CovenantCallings = {} end
install(C_CovenantCallings, "AreCallingsUnlocked", function() return false end)
install(C_CovenantCallings, "RequestCallings", function()
    -- Complete the documented request with an empty BountyInfo array. FireEvent
    -- uses the existing synchronous dispatcher; this is not a discarded request.
    FireEvent("COVENANT_CALLINGS_UPDATED", {})
end)
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    if ACTIVE == ClientProfile::Wrath {
        return Ok(());
    }
    lua.exec(SESSION_AND_QUEUE_DEFAULTS)?;
    if matches!(
        ACTIVE,
        ClientProfile::Retail | ClientProfile::Ptr | ClientProfile::WowForever
    ) {
        lua.exec(MODERN_DEFAULTS)?;
    }
    Ok(())
}
