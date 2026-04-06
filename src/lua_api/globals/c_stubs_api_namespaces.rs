//! Missing C_* namespace stubs and game-state globals referenced during startup.
//!
//! Split from c_stubs_api.rs — contains social/system namespaces, video options,
//! perks activities, game-state stubs, and incoming summon.

use mlua::{Lua, MultiValue, Result, Value};

/// C_PerksActivities - Monthly activities / Trading Post tracking.
pub(crate) fn register_c_perks_activities(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetTrackedPerksActivities",
        lua.create_function(|lua, ()| {
            let result = lua.create_table()?;
            result.set("trackedIDs", lua.create_table()?)?;
            Ok(result)
        })?,
    )?;
    t.set(
        "GetPerksActivityInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetPerksActivityChatLink",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "RemoveTrackedPerksActivity",
        lua.create_function(|_, _id: i32| Ok(()))?,
    )?;
    lua.globals().set("C_PerksActivities", t)?;
    Ok(())
}

/// Missing C_* namespaces and globals referenced during startup events.
pub(crate) fn register_missing_namespaces(lua: &Lua) -> Result<()> {
    register_social_namespaces(lua)?;
    register_system_namespaces(lua)?;
    Ok(())
}

/// Social, friends, and matchmaking namespace stubs.
fn register_social_namespaces(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    register_social_status_namespaces(lua, &g)?;
    register_social_queue_namespace(lua, &g)?;
    Ok(())
}

fn register_social_status_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let spectating = lua.create_table()?;
    spectating.set("IsSpectating", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("C_SpectatingUI", spectating)?;

    let social = lua.create_table()?;
    social.set("IsMuted", lua.create_function(|_, ()| Ok(false))?)?;
    social.set("IsSilenced", lua.create_function(|_, ()| Ok(false))?)?;
    social.set("IsSquelched", lua.create_function(|_, ()| Ok(false))?)?;
    social.set("IsChatDisabled", lua.create_function(|_, ()| Ok(false))?)?;
    social.set("CanReceiveChat", lua.create_function(|_, ()| Ok(true))?)?;
    g.set("C_SocialRestrictions", social)?;

    let lobby = lua.create_table()?;
    lobby.set("IsParticipating", lua.create_function(|_, ()| Ok(false))?)?;
    lobby.set("IsInQueue", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("C_LobbyMatchmakerInfo", lobby)?;

    let mentorship = lua.create_table()?;
    mentorship.set(
        "GetMentorshipStatus",
        lua.create_function(|_, _unit: Value| Ok(0i32))?,
    )?;
    mentorship.set(
        "IsActivePlayerConsideredNewcomer",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set("C_PlayerMentorship", mentorship)?;

    let recent_allies = lua.create_table()?;
    recent_allies.set("IsSystemEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("C_RecentAllies", recent_allies)?;
    Ok(())
}

fn register_social_queue_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let social_queue = lua.create_table()?;
    social_queue.set(
        "GetAllGroups",
        lua.create_function(|lua, _local_only: Option<bool>| lua.create_table())?,
    )?;
    social_queue.set(
        "GetConfig",
        lua.create_function(|lua, ()| {
            let config = lua.create_table()?;
            config.set("toastDuration", 60.0f64)?;
            config.set("enableToasts", false)?;
            Ok(config)
        })?,
    )?;
    g.set("C_SocialQueue", social_queue)?;
    Ok(())
}

/// System, service, and utility namespace stubs.
fn register_system_namespaces(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    super::c_stubs_api_glue::register_system_namespaces(lua, &g)?;
    super::c_stubs_api_store::register_c_account_store(lua)?;
    register_c_video_options(lua)?;
    Ok(())
}

fn register_shared_character_services_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let shared_character_services = lua.create_table()?;
    shared_character_services.set(
        "GetUpgradeDistributions",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set("C_SharedCharacterServices", shared_character_services)?;
    Ok(())
}

fn register_configuration_warnings_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let configuration_warnings = lua.create_table()?;
    configuration_warnings.set(
        "GetConfigurationWarnings",
        lua.create_function(|lua, _include_seen_warnings: Option<bool>| lua.create_table())?,
    )?;
    configuration_warnings.set(
        "GetConfigurationWarningString",
        lua.create_function(|_, _warning: Value| Ok(Value::Nil))?,
    )?;
    g.set("C_ConfigurationWarnings", configuration_warnings)?;
    Ok(())
}

fn register_store_glue_namespace(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let store_glue = lua.create_table()?;
    store_glue.set(
        "GetDisconnectOnLogout",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    store_glue.set(
        "GetVASProductReady",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    store_glue.set(
        "GetVASPurchaseStateInfo",
        lua.create_function(|_, _guid: Value| Ok((0i32, Value::Nil, Value::Nil)))?,
    )?;
    store_glue.set(
        "RequestCharacterQueueTime",
        lua.create_function(|_, _guid: Value| Ok(()))?,
    )?;
    store_glue.set(
        "UpdateVASPurchaseStates",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set("C_StoreGlue", store_glue)?;
    Ok(())
}

/// C_VideoOptions — screen resolution and graphics queries.
fn register_c_video_options(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    let video = lua.create_table()?;
    video.set(
        "GetDefaultGameWindowSize",
        lua.create_function(|lua, _monitor: i32| {
            let t = lua.create_table()?;
            t.set("x", 1920)?;
            t.set("y", 1080)?;
            Ok(t)
        })?,
    )?;
    video.set(
        "GetCurrentGameWindowSize",
        lua.create_function(|lua, _args: MultiValue| {
            let t = lua.create_table()?;
            t.set("x", 1920)?;
            t.set("y", 1080)?;
            Ok(t)
        })?,
    )?;
    video.set(
        "GetGameWindowSizes",
        lua.create_function(|lua, _args: MultiValue| lua.create_table())?,
    )?;
    video.set(
        "GetGxAdapterInfo",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    video.set(
        "IsSpellVisualDensitySystemSupported",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    video.set(
        "SetGameWindowSize",
        lua.create_function(|_, (_x, _y): (i32, i32)| Ok(()))?,
    )?;
    g.set("C_VideoOptions", video)?;
    Ok(())
}

/// Game-state global stubs for functions referenced during startup events.
pub(crate) fn register_game_state_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    super::c_stubs_api_glue::register_game_state_namespaces(lua, &g)?;
    register_shared_character_services_namespace(lua, &g)?;
    register_configuration_warnings_namespace(lua, &g)?;
    register_store_glue_namespace(lua, &g)?;
    g.set("IsTargetLoose", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("IsPartyLFG", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("IsPartyWorldPVP", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "PlayerGetTimerunningSeasonID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "UnitDistanceSquared",
        lua.create_function(|_, _unit: Value| Ok((0.0f64, true)))?,
    )?;
    g.set(
        "UnitInOtherParty",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "UnitHasIncomingResurrection",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetLFGRoles",
        lua.create_function(|_, ()| Ok((false, false, false)))?,
    )?;
    g.set(
        "GetLFGReadyCheckUpdate",
        lua.create_function(|_, ()| Ok(mlua::MultiValue::new()))?,
    )?;
    g.set(
        "CanPartyLFGBackfill",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetNumArenaOpponentSpecs",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GetArenaOpponentSpec",
        lua.create_function(|_, _index: Value| Ok((0i32, 0i32)))?,
    )?;
    g.set(
        "UnitTreatAsPlayerForDisplay",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetLFGDeserterExpiration",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "UnitHasLFGDeserter",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetWorldPVPQueueStatus",
        lua.create_function(|_, _index: Value| Ok(("none", 0i32, 0i32, 0i32)))?,
    )?;
    g.set(
        "CanHearthAndResurrectFromArea",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetChannelList",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "CanBeRaidTarget",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set(
        "GetRaidTargetIndex",
        lua.create_function(|_, _unit: Value| Ok(Value::Nil))?,
    )?;
    Ok(())
}

/// C_IncomingSummon namespace stubs.
pub(crate) fn register_c_incoming_summon(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "HasIncomingSummon",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    t.set(
        "IncomingSummonStatus",
        lua.create_function(|_, _unit: Value| Ok(0i32))?,
    )?;
    lua.globals().set("C_IncomingSummon", t)?;
    Ok(())
}
