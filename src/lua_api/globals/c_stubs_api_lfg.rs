//! LFG, guild, and loss-of-control stubs split from c_stubs_api.rs.

use mlua::{Lua, Result, Value};

struct SeededLossOfControl {
    spell_id: i32,
    loc_type: &'static str,
    display_text: &'static str,
    icon_texture: &'static str,
    duration: f64,
    lockout_school: i32,
    priority: i32,
    display_type: i32,
}

const SEEDED_LOSS_OF_CONTROL: SeededLossOfControl = SeededLossOfControl {
    spell_id: 408,
    loc_type: "STUN_MECHANIC",
    display_text: "Kidney Shot",
    icon_texture: "Interface\\Icons\\Ability_Rogue_KidneyShot",
    duration: 4.0,
    lockout_school: 0,
    priority: 100,
    display_type: 2,
};

pub(super) fn register_c_guild(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let t = lua.create_table()?;
    let st = std::rc::Rc::clone(&state);
    t.set(
        "GetNumMembers",
        lua.create_function(move |_, ()| Ok(st.borrow().world.guild_num_members))?,
    )?;
    let st = std::rc::Rc::clone(&state);
    t.set(
        "IsInGuild",
        lua.create_function(move |_, ()| Ok(st.borrow().world.guild_name.is_some()))?,
    )?;
    t.set(
        "GetGuildInfo",
        lua.create_function(move |_, _unit: Option<String>| {
            let s = state.borrow();
            match &s.world.guild_name {
                Some(name) => {
                    let rank = s.world.guild_rank.clone().unwrap_or_default();
                    Ok((name.clone(), rank, s.world.guild_num_members, String::new()))
                }
                None => Ok((String::new(), String::new(), 0i32, String::new())),
            }
        })?,
    )?;
    t.set(
        "GetMemberInfo",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    lua.globals().set("C_Guild", t)?;
    Ok(())
}

pub(super) fn register_c_guild_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    seed_guild_info_data(&t)?;
    register_guild_info_queries(lua, &t)?;
    register_guild_info_accessors(lua, &t)?;
    lua.globals().set("C_GuildInfo", t)?;
    Ok(())
}

fn seed_guild_info_data(t: &mlua::Table) -> Result<()> {
    t.set(
        "__motd",
        "Raid invites tonight at 20:00 server. Repairs are on for progression.",
    )?;
    t.set(
        "__infoText",
        "Mythic-focused guild recruiting healers and a warlock for weekend raids.",
    )
}

fn register_guild_info_queries(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetGuildTabardInfo",
        lua.create_function(|_, _unit: Option<String>| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetGuildNewsInfo",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "AreGuildEventsEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set("GuildRoster", lua.create_function(|_, ()| Ok(()))?)?;
    Ok(())
}

fn register_guild_info_accessors(lua: &Lua, t: &mlua::Table) -> Result<()> {
    let guild_info = t.clone();
    t.set(
        "GetMOTD",
        lua.create_function(move |_, ()| guild_info.get::<String>("__motd"))?,
    )?;
    let guild_info = t.clone();
    t.set(
        "GetInfoText",
        lua.create_function(move |_, ()| guild_info.get::<String>("__infoText"))?,
    )?;
    let guild_info = t.clone();
    t.set(
        "SetInfoText",
        lua.create_function(move |_, info_text: String| guild_info.set("__infoText", info_text))?,
    )?;
    Ok(())
}

pub(super) fn register_c_lfg_list(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let t = lua.create_table()?;
    register_lfg_list_stubs(lua, &t)?;
    register_lfg_search_result_info(lua, &t, std::rc::Rc::clone(&state))?;
    register_lfg_search_and_activity(lua, &t, state)?;
    lua.globals().set("C_LFGList", t)?;
    Ok(())
}

fn register_lfg_list_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetActiveEntryInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "HasActiveEntryInfo",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "CanCreateQuestGroup",
        lua.create_function(|_, _: i32| Ok(false))?,
    )?;
    t.set(
        "GetAvailableRoles",
        lua.create_function(|_, ()| Ok((true, true, true)))?,
    )?;
    t.set(
        "GetApplications",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetNumApplications",
        lua.create_function(|_, ()| Ok((0i32, 0i32)))?,
    )?;
    t.set("IsSquelched", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetAvailableCategories",
        lua.create_function(|lua, _: mlua::MultiValue| lua.create_table())?,
    )?;
    t.set("HasActivityList", lua.create_function(|_, ()| Ok(false))?)
}

fn register_lfg_search_result_info(
    lua: &Lua,
    t: &mlua::Table,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    t.set(
        "GetSearchResultInfo",
        lua.create_function(move |lua, result_id: u32| {
            let st = state.borrow();
            let Some(listing) = st
                .world
                .premade_listings
                .iter()
                .find(|l| l.search_result_id == result_id)
            else {
                return Ok(Value::Nil);
            };
            let tbl = lua.create_table()?;
            tbl.set("searchResultID", listing.search_result_id)?;
            tbl.set("name", lua.create_string(&listing.name)?)?;
            tbl.set("comment", lua.create_string(&listing.comment)?)?;
            tbl.set("leaderName", lua.create_string(&listing.leader_name)?)?;
            tbl.set("numMembers", listing.num_members)?;
            tbl.set("maxMembers", listing.max_members)?;
            tbl.set("activityID", listing.activity_id)?;
            tbl.set("voiceChat", listing.voice_chat)?;
            tbl.set("autoAccept", listing.auto_accept)?;
            tbl.set("isDelisted", listing.is_delisted)?;
            Ok(Value::Table(tbl))
        })?,
    )
}

fn register_lfg_search_and_activity(
    lua: &Lua,
    t: &mlua::Table,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    register_lfg_search_methods(lua, t, state)?;
    register_lfg_activity_stubs(lua, t)
}

fn register_lfg_search_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    t.set(
        "GetSearchResults",
        lua.create_function({
            let s = std::rc::Rc::clone(&state);
            move |lua, ()| {
                let st = s.borrow();
                let active: Vec<_> = st
                    .world
                    .premade_listings
                    .iter()
                    .filter(|l| !l.is_delisted)
                    .collect();
                let count = active.len() as i32;
                let results = lua.create_table()?;
                for (i, listing) in active.iter().enumerate() {
                    results.set(i + 1, listing.search_result_id)?;
                }
                Ok((count, results))
            }
        })?,
    )?;
    t.set(
        "Search",
        lua.create_function({
            move |lua, _args: mlua::MultiValue| {
                let count = state.borrow().world.premade_listings.len();
                if count > 0 {
                    let fire: mlua::Function = lua.globals().get("FireEvent")?;
                    fire.call::<()>(("LFG_LIST_SEARCH_RESULTS_RECEIVED",))?;
                }
                Ok(())
            }
        })?,
    )
}

fn register_lfg_activity_stubs(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set(
        "GetActivityInfoTable",
        lua.create_function(|lua, activity_id: u32| {
            let tbl = lua.create_table()?;
            tbl.set("activityID", activity_id)?;
            tbl.set(
                "fullName",
                lua.create_string(&format!("Activity {activity_id}"))?,
            )?;
            tbl.set(
                "shortName",
                lua.create_string(&format!("Act{activity_id}"))?,
            )?;
            tbl.set("categoryID", 2)?;
            tbl.set("groupFinderActivityGroupID", 0)?;
            tbl.set("maxPlayers", 5)?;
            tbl.set("minLevel", 70)?;
            tbl.set("maxLevel", 80)?;
            tbl.set("isMythicPlusActivity", activity_id > 1000)?;
            Ok(tbl)
        })?,
    )?;
    t.set(
        "GetActivityGroupInfo",
        lua.create_function(|lua, _: u32| Ok(lua.create_string("Dungeons")?))?,
    )?;
    t.set(
        "GetAvailableActivities",
        lua.create_function(|lua, _: mlua::MultiValue| lua.create_table())?,
    )
}

pub(super) fn register_c_loss_of_control(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let t = lua.create_table()?;
    register_loss_of_control_data_methods(lua, &t, &state)?;
    register_loss_of_control_count_methods(lua, &t, &state)?;
    lua.globals().set("C_LossOfControl", t)?;
    Ok(())
}

fn register_loss_of_control_data_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: &std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let global_state = std::rc::Rc::clone(state);
    t.set(
        "GetActiveLossOfControlData",
        lua.create_function(move |lua, index: i32| {
            create_loss_of_control_data(lua, &global_state, None, index)
        })?,
    )?;
    let by_unit_state = std::rc::Rc::clone(state);
    t.set(
        "GetActiveLossOfControlDataByUnit",
        lua.create_function(move |lua, (unit_token, index): (String, i32)| {
            create_loss_of_control_data(lua, &by_unit_state, Some(unit_token.as_str()), index)
        })?,
    )?;
    Ok(())
}

fn register_loss_of_control_count_methods(
    lua: &Lua,
    t: &mlua::Table,
    state: &std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    t.set(
        "GetActiveLossOfControlDataCount",
        lua.create_function(|_, ()| Ok(1i32))?,
    )?;
    let count_state = std::rc::Rc::clone(state);
    t.set(
        "GetActiveLossOfControlDataCountByUnit",
        lua.create_function(move |_, unit_token: String| {
            Ok(loss_of_control_count_for_unit(
                &count_state.borrow(),
                Some(unit_token.as_str()),
            ))
        })?,
    )?;
    let duration_state = std::rc::Rc::clone(state);
    t.set(
        "GetActiveLossOfControlDuration",
        lua.create_function(move |_, (unit_token, index): (String, i32)| {
            if loss_of_control_count_for_unit(&duration_state.borrow(), Some(unit_token.as_str()))
                == 1
                && index == 1
            {
                Ok(Some(SEEDED_LOSS_OF_CONTROL.duration))
            } else {
                Ok(None::<f64>)
            }
        })?,
    )?;
    Ok(())
}

fn create_loss_of_control_data(
    lua: &Lua,
    state: &std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    unit_token: Option<&str>,
    index: i32,
) -> Result<Value> {
    if index != 1 || loss_of_control_count_for_unit(&state.borrow(), unit_token) == 0 {
        return Ok(Value::Nil);
    }

    let data = lua.create_table()?;
    data.set("locType", SEEDED_LOSS_OF_CONTROL.loc_type)?;
    data.set("spellID", SEEDED_LOSS_OF_CONTROL.spell_id)?;
    data.set("displayText", SEEDED_LOSS_OF_CONTROL.display_text)?;
    data.set("iconTexture", SEEDED_LOSS_OF_CONTROL.icon_texture)?;
    data.set("startTime", Value::Nil)?;
    data.set("timeRemaining", SEEDED_LOSS_OF_CONTROL.duration)?;
    data.set("duration", SEEDED_LOSS_OF_CONTROL.duration)?;
    data.set("lockoutSchool", SEEDED_LOSS_OF_CONTROL.lockout_school)?;
    data.set("priority", SEEDED_LOSS_OF_CONTROL.priority)?;
    data.set("displayType", SEEDED_LOSS_OF_CONTROL.display_type)?;
    data.set("auraInstanceID", Value::Nil)?;
    Ok(Value::Table(data))
}

fn loss_of_control_count_for_unit(
    state: &crate::lua_api::SimState,
    unit_token: Option<&str>,
) -> i32 {
    match unit_token {
        None => 1,
        Some("player") => 1,
        Some("target") if state.current_target.is_some() => 1,
        _ => 0,
    }
}
