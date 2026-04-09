//! C_* namespace stubs and global function stubs for Blizzard UI code.
//! See also: c_stubs_api_missing.rs, c_stubs_api_namespaces.rs, c_stubs_api_extra.rs,
//! c_stubs_api_combat.rs, c_stubs_api_glue.rs, c_stubs_api_professions.rs.

use mlua::{Lua, MultiValue, Result, Value};

#[derive(Clone, Copy)]
struct SeededMacro {
    id: i32,
    name: &'static str,
    icon: &'static str,
    body: &'static str,
}

const SEEDED_ACCOUNT_MACROS: &[SeededMacro] = &[
    SeededMacro {
        id: 1,
        name: "Raid Beacon",
        icon: "Interface\\Icons\\INV_Misc_QuestionMark",
        body: "/rw Stack on star",
    },
    SeededMacro {
        id: 2,
        name: "Pull Timer",
        icon: "Interface\\Icons\\INV_Misc_PocketWatch_01",
        body: "/pull 10",
    },
];

const SEEDED_CHARACTER_MACROS: &[SeededMacro] = &[SeededMacro {
    id: 121,
    name: "Crusader",
    icon: "Interface\\Icons\\Spell_Holy_CrusaderAura",
    body: "/cast Crusader Aura",
}];

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

/// Register all additional C_* namespace stubs.
pub fn register_c_stubs_api(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    register_core_namespaces(lua, std::rc::Rc::clone(&state))?;
    register_ui_and_chat_stubs(lua, state.clone())?;
    super::c_stubs_api_missing::register_missing_globals(lua, state.clone())?;
    super::c_stubs_api_namespaces::register_missing_namespaces(lua, state.clone())?;
    super::c_stubs_api_namespaces::register_c_perks_activities(lua)?;
    super::c_stubs_api_namespaces::register_game_state_stubs(lua)?;
    super::c_stubs_api_namespaces::register_c_incoming_summon(lua)?;
    super::c_stubs_api_extra::register_extra_stubs(lua, state.clone())?;
    super::c_stubs_api_combat::register_combat_stubs(lua)?;
    super::c_stubs_api_professions::register_profession_stubs(lua, state)?;
    Ok(())
}

fn register_core_namespaces(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    register_c_achievement_info(lua)?;
    super::hero_talents::register_c_class_talents(lua, std::rc::Rc::clone(&state))?;
    register_c_guild(lua, std::rc::Rc::clone(&state))?;
    register_c_guild_info(lua)?;
    register_c_lfg_list(lua, std::rc::Rc::clone(&state))?;
    register_c_loss_of_control(lua, std::rc::Rc::clone(&state))?;
    register_c_mail(lua, std::rc::Rc::clone(&state))?;
    register_c_stable_info(lua)?;
    register_c_tutorial(lua)?;
    super::action_bar_api::register_c_action_bar_namespace(lua, state.clone())?;
    register_unit_frame_global_stubs(lua, std::rc::Rc::clone(&state))?;
    register_powerbar_prediction_colors(lua)?;
    super::c_stubs_achievement::register_achievement_stubs(lua)?;
    super::c_stubs_achievement::register_tracking_stubs(lua)?;
    Ok(())
}

fn register_ui_and_chat_stubs(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    register_c_log(lua)?;
    register_c_campaign_info(lua)?;
    register_quest_global_functions(lua, state)?;
    register_chat_stubs(lua)?;
    register_chat_window_stubs(lua)?;
    register_c_macro(lua)?;
    register_c_wowlabs_matchmaking(lua)?;
    super::fading_frame_api::register_fading_frame_stubs(lua)?;
    Ok(())
}

fn register_c_achievement_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetRewardItemID",
        lua.create_function(|_, _achievement_id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetAchievementInfo",
        lua.create_function(|_, _achievement_id: i32| Ok(Value::Nil))?,
    )?;
    lua.globals().set("C_AchievementInfo", t)?;
    Ok(())
}

fn register_c_guild(
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

fn register_c_guild_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
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
    lua.globals().set("C_GuildInfo", t)?;
    Ok(())
}

fn register_c_lfg_list(
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

fn register_c_loss_of_control(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let t = lua.create_table()?;
    let global_state = std::rc::Rc::clone(&state);
    t.set(
        "GetActiveLossOfControlData",
        lua.create_function(move |lua, index: i32| {
            create_loss_of_control_data(lua, &global_state, None, index)
        })?,
    )?;
    t.set(
        "GetActiveLossOfControlDataCount",
        lua.create_function(|_, ()| Ok(1i32))?,
    )?;
    let by_unit_state = std::rc::Rc::clone(&state);
    t.set(
        "GetActiveLossOfControlDataByUnit",
        lua.create_function(move |lua, (unit_token, index): (String, i32)| {
            create_loss_of_control_data(lua, &by_unit_state, Some(unit_token.as_str()), index)
        })?,
    )?;
    let count_state = std::rc::Rc::clone(&state);
    t.set(
        "GetActiveLossOfControlDataCountByUnit",
        lua.create_function(move |_, unit_token: String| {
            Ok(loss_of_control_count_for_unit(
                &count_state.borrow(),
                Some(unit_token.as_str()),
            ))
        })?,
    )?;
    let duration_state = std::rc::Rc::clone(&state);
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
    lua.globals().set("C_LossOfControl", t)?;
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

fn register_c_mail(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let globals = lua.globals();
    let t: mlua::Table = match globals.get::<Value>("C_Mail")? {
        Value::Table(existing) => existing,
        _ => {
            let created = lua.create_table()?;
            globals.set("C_Mail", created.clone())?;
            created
        }
    };
    t.set(
        "GetNumItems",
        lua.create_function({
            let state = std::rc::Rc::clone(&state);
            move |_, ()| Ok(state.borrow().player.inbox.len() as i32)
        })?,
    )?;
    t.set(
        "HasNewMail",
        lua.create_function(move |_, ()| {
            let has_unread = state
                .borrow()
                .player
                .inbox
                .iter()
                .any(|mail| !mail.was_read);
            Ok(has_unread)
        })?,
    )?;
    t.set("IsCommandPending", lua.create_function(|_, ()| Ok(false))?)?;
    Ok(())
}

fn register_c_stable_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("GetNumStablePets", lua.create_function(|_, ()| Ok(0i32))?)?;
    lua.globals().set("C_StableInfo", t)?;
    Ok(())
}

fn register_c_tutorial(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetTutorialStatus",
        lua.create_function(|_, _tutorial_id: Option<i32>| Ok(false))?,
    )?;
    t.set(
        "SetTutorialFlag",
        lua.create_function(|_, _tutorial_id: i32| Ok(()))?,
    )?;
    lua.globals().set("C_Tutorial", t)?;
    Ok(())
}

/// Resolve a texture path or file data ID to a WoW interface path.
/// Global function stubs needed by Blizzard_UnitFrame.
fn register_unit_frame_global_stubs(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    register_combat_state_globals(lua, state)?;
    register_unit_frame_stateless_stubs(lua)?;
    register_unit_frame_global_stubs_2(lua)?;
    Ok(())
}

fn register_combat_state_globals(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let g = lua.globals();
    let s2 = std::rc::Rc::clone(&state);
    g.set(
        "InCombatLockdown",
        lua.create_function(move |_, ()| Ok(s2.borrow().player.in_combat))?,
    )?;
    g.set(
        "IsResting",
        lua.create_function(move |_, ()| Ok(state.borrow().player.is_resting))?,
    )?;
    Ok(())
}

fn register_unit_frame_stateless_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set("IsPVPTimerRunning", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("GetPVPTimer", lua.create_function(|_, ()| Ok(0.0f64))?)?;
    g.set(
        "GetReadyCheckStatus",
        lua.create_function(|_, _unit: Option<String>| Ok(Value::Nil))?,
    )?;
    g.set(
        "HasLFGRestrictions",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetPartyLFGID",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "RequestGuildPartyState",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "GetLFGCategoryForID",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    g.set(
        "IsEveryoneAssistant",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "WorldLootObjectExists",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    g.set("IsInRaid", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "GetRaidRosterInfo",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    g.set("PartialPlayTime", lua.create_function(|_, ()| Ok(false))?)?;
    g.set("NoPlayTime", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "GetBillingTimeRested",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    lua.load(
        r#"
        function SetPortraitToTexture(tex, path)
            if tex and tex.SetTexture then
                tex:SetTexture(path)
                -- Apply circular mask if not already applied
                if tex.GetNumMaskTextures and tex:GetNumMaskTextures() == 0 and tex.GetParent then
                    local parent = tex:GetParent()
                    if parent and parent.CreateMaskTexture then
                        local mask = parent:CreateMaskTexture(nil, "ARTWORK")
                        mask:SetTexture("Interface\\CharacterFrame\\TempPortraitAlphaMask")
                        mask:SetAllPoints(tex)
                        tex:AddMaskTexture(mask)
                    end
                end
            end
        end
    "#,
    )
    .exec()?;
    Ok(())
}

/// Continuation of unit-frame global stubs (combat, arena, UIParent handlers).
fn register_unit_frame_global_stubs_2(lua: &Lua) -> Result<()> {
    register_combat_and_arena_stubs(lua)?;
    register_uiparent_entering_world_stubs(lua)?;
    Ok(())
}

/// Combat, threat, arena, pet, and misc OnUpdate handler stubs.
fn register_combat_and_arena_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "GetUnitTotalModifiedMaxHealthPercent",
        lua.create_function(|_, _unit: Option<String>| Ok(0.0f64))?,
    )?;
    g.set(
        "IsThreatWarningEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetThreatStatusColor",
        lua.create_function(|_, _status: i32| Ok((1.0f64, 1.0f64, 1.0f64)))?,
    )?;
    g.set("LE_REALM_RELATION_VIRTUAL", 3i32)?;
    g.set(
        "IsActiveBattlefieldArena",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetNumArenaOpponents",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GetBattlefieldEstimatedWaitTime",
        lua.create_function(|_, _index: Value| Ok(0i32))?,
    )?;
    g.set("PetUsesPetFrame", lua.create_function(|_, ()| Ok(true))?)?;
    g.set(
        "UnitIsPossessed",
        lua.create_function(|_, _unit: Option<String>| Ok(false))?,
    )?;
    g.set(
        "GetReleaseTimeRemaining",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "FCF_OnUpdate",
        lua.create_function(|_, _elapsed: Option<f64>| Ok(()))?,
    )?;
    g.set(
        "HelpOpenWebTicketButton_OnUpdate",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    g.set(
        "GetLootSpecialization",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    Ok(())
}

/// UIParent PLAYER_ENTERING_WORLD handler stubs.
fn register_uiparent_entering_world_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "GetSpellConfirmationPromptsInfo",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set(
        "ResurrectGetOfferer",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetActiveLootRollIDs",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set(
        "GetTutorialsEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "BoostTutorial_AttemptLoad",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "ExpansionTrial_CheckLoadUI",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "SubscriptionInterstitial_LoadUI",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    g.set(
        "ShowResurrectRequest",
        lua.create_function(|_, _offerer: String| Ok(()))?,
    )?;
    g.set(
        "GroupLootContainer_AddRoll",
        lua.create_function(|_, (_id, _dur): (Value, Value)| Ok(()))?,
    )?;
    g.set(
        "RemixArtifactTutorialUI_LoadUI",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    Ok(())
}

/// POWERBAR_PREDICTION_COLOR_* globals used by PowerBarColorUtil.lua at parse time.
const POWERBAR_COLORS: &[(&str, f64, f64, f64)] = &[
    ("POWERBAR_PREDICTION_COLOR_MANA", 0.0, 0.0, 1.0),
    ("POWERBAR_PREDICTION_COLOR_RAGE", 1.0, 0.0, 0.0),
    ("POWERBAR_PREDICTION_COLOR_FOCUS", 1.0, 0.5, 0.25),
    ("POWERBAR_PREDICTION_COLOR_ENERGY", 1.0, 1.0, 0.0),
    ("POWERBAR_PREDICTION_COLOR_RUNIC_POWER", 0.0, 0.82, 1.0),
    ("POWERBAR_PREDICTION_COLOR_LUNAR_POWER", 0.3, 0.52, 0.9),
    ("POWERBAR_PREDICTION_COLOR_MAELSTROM", 0.0, 0.5, 1.0),
    ("POWERBAR_PREDICTION_COLOR_INSANITY", 0.4, 0.0, 0.8),
    ("POWERBAR_PREDICTION_COLOR_FURY", 0.788, 0.259, 0.992),
    ("POWERBAR_PREDICTION_COLOR_PAIN", 1.0, 0.612, 0.0),
];

fn build_color_entry(
    lua: &Lua,
    r: f64,
    green: f64,
    b: f64,
    get_rgba: &mlua::Function,
    get_rgb: &mlua::Function,
) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set("r", r)?;
    t.set("g", green)?;
    t.set("b", b)?;
    t.set("a", 0.5f64)?;
    t.set("GetRGBA", get_rgba.clone())?;
    t.set("GetRGB", get_rgb.clone())?;
    Ok(t)
}

fn register_powerbar_prediction_colors(lua: &Lua) -> Result<()> {
    let get_rgba = lua.create_function(|_, this: mlua::Table| {
        Ok((
            this.get::<f64>("r")?,
            this.get::<f64>("g")?,
            this.get::<f64>("b")?,
            this.get::<f64>("a")?,
        ))
    })?;
    let get_rgb = lua.create_function(|_, this: mlua::Table| {
        Ok((
            this.get::<f64>("r")?,
            this.get::<f64>("g")?,
            this.get::<f64>("b")?,
        ))
    })?;
    let g = lua.globals();
    for &(name, r, green, b) in POWERBAR_COLORS {
        g.set(
            name,
            build_color_entry(lua, r, green, b, &get_rgba, &get_rgb)?,
        )?;
    }
    Ok(())
}

fn register_c_log(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("LogMessage", lua.create_function(|_, _msg: Value| Ok(()))?)?;
    t.set(
        "LogErrorMessage",
        lua.create_function(|_, _msg: Value| Ok(()))?,
    )?;
    lua.globals().set("C_Log", t)?;
    Ok(())
}

/// C_CampaignInfo namespace - campaign/war campaign data.
fn register_c_campaign_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetCampaignID",
        lua.create_function(|_, _quest_id: i32| Ok(0i32))?,
    )?;
    t.set(
        "GetCampaignInfo",
        lua.create_function(|_, _campaign_id: i32| Ok(Value::Nil))?,
    )?;
    lua.globals().set("C_CampaignInfo", t)?;
    Ok(())
}

/// Quest-related global functions used by ObjectiveTracker.
fn register_quest_global_functions(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let g = lua.globals();
    g.set(
        "IsInInstance",
        lua.create_function(move |_, ()| {
            let s = state.borrow();
            Ok((s.world.in_instance, s.world.instance_type.clone()))
        })?,
    )?;
    register_quest_query_stubs(lua, &g)?;
    register_quest_leaderboard_functions(lua, &g)?;
    Ok(())
}

/// Stateless quest query/popup stubs.
fn register_quest_query_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "IsQuestSequenced",
        lua.create_function(|_, _quest_id: i32| Ok(false))?,
    )?;
    g.set(
        "GetQuestLogCompletionText",
        lua.create_function(|_, _log_idx: i32| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetQuestProgressBarPercent",
        lua.create_function(|_, _quest_id: i32| Ok(0.0f64))?,
    )?;
    g.set(
        "QuestMapFrame_GetFocusedQuestID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "IsModifiedClick",
        lua.create_function(|_, _action: String| Ok(false))?,
    )?;
    g.set(
        "GetQuestLink",
        lua.create_function(|_, _quest_id: i32| Ok(Value::Nil))?,
    )?;
    g.set("IsInJailersTower", lua.create_function(|_, ()| Ok(false))?)?;
    g.set(
        "IsOnGroundFloorInJailersTower",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetNumAutoQuestPopUps",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set(
        "GetAutoQuestPopUp",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetQuestLogSpecialItemInfo",
        lua.create_function(|_, _log_idx: i32| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetTasksTable",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    g.set(
        "ExpandQuestHeader",
        lua.create_function(|_, (_idx, _no_update): (i32, Option<bool>)| Ok(()))?,
    )?;
    g.set(
        "CollapseQuestHeader",
        lua.create_function(|_, (_idx, _no_update): (i32, Option<bool>)| Ok(()))?,
    )?;
    Ok(())
}

/// GetNumQuestLeaderBoards / GetQuestLogLeaderBoard - quest objective data.
/// Delegates to c_quest_api which owns the single source of truth for quest data.
fn register_quest_leaderboard_functions(lua: &Lua, g: &mlua::Table) -> Result<()> {
    g.set(
        "GetNumQuestLeaderBoards",
        lua.create_function(|_, log_idx: i32| {
            Ok(super::c_quest_api::num_quest_leaderboards(log_idx))
        })?,
    )?;
    g.set(
        "GetQuestLogLeaderBoard",
        lua.create_function(
            |_, (obj_idx, log_idx, _suppress): (i32, i32, Option<bool>)| {
                Ok(super::c_quest_api::quest_leaderboard_entry(
                    log_idx, obj_idx,
                ))
            },
        )?,
    )?;
    Ok(())
}

/// Chat window management stubs needed by FloatingChatFrame.
fn register_chat_window_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "SetChatWindowLocked",
        lua.create_function(|_, (_id, _locked): (i32, bool)| Ok(()))?,
    )?;
    g.set(
        "SetChatWindowUninteractable",
        lua.create_function(|_, (_id, _flag): (i32, bool)| Ok(()))?,
    )?;
    g.set(
        "GetChatWindowSavedDimensions",
        lua.create_function(|_, _id: i32| Ok((430.0f64, 120.0f64)))?,
    )?;
    g.set(
        "SetChatWindowColor",
        lua.create_function(|_, (_id, _r, _g, _b): (i32, f64, f64, f64)| Ok(()))?,
    )?;
    g.set(
        "SetChatWindowAlpha",
        lua.create_function(|_, (_id, _a): (i32, f64)| Ok(()))?,
    )?;
    g.set(
        "GetChatWindowSavedPosition",
        lua.create_function(|_, _id: i32| {
            // Returns: point, yOffset, xOffset, relativePoint
            Ok(("BOTTOMLEFT", 0.0f64, 0.0f64, "BOTTOMLEFT"))
        })?,
    )?;
    // ChangeChatColor: sets r,g,b on ChatTypeInfo[type]
    g.set(
        "ChangeChatColor",
        lua.create_function(|lua, (ct, r, g, b): (String, f64, f64, f64)| {
            let cti: mlua::Table = lua
                .globals()
                .get::<mlua::Table>("ChatTypeInfo")?
                .get(&*ct)?;
            cti.set("r", r)?;
            cti.set("g", g)?;
            cti.set("b", b)?;
            Ok(())
        })?,
    )?;
    Ok(())
}

/// Chat-related global function stubs needed by Blizzard_ChatFrame.
fn register_chat_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    // GetChatTypeIndex: deterministic integer from chat type name
    g.set(
        "GetChatTypeIndex",
        lua.create_function(|_, name: String| {
            let hash = name
                .bytes()
                .fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
            Ok((hash % 50 + 1) as i32)
        })?,
    )?;
    // CreateSecureDelegate: no taint system, return the function as-is
    g.set(
        "CreateSecureDelegate",
        lua.create_function(|_, func: mlua::Function| Ok(func))?,
    )?;
    // GetChatWindowInfo: return defaults (only window 1 is shown)
    // Returns: name, fontSize, r, g, b, alpha, shown, locked, docked, uninteractable
    // Default color is black (0,0,0) at 25% alpha, matching DEFAULT_CHATFRAME_COLOR/ALPHA
    g.set(
        "GetChatWindowInfo",
        lua.create_function(|_, id: i32| {
            let name = format!("ChatFrame{id}");
            let shown = id == 1;
            Ok((
                name, 14.0f64, 0.0f64, 0.0f64, 0.0f64, 0.25f64, shown, false, false, false,
            ))
        })?,
    )?;
    // GetChatWindowMessages/GetChatWindowChannels: return no message types or channels
    g.set(
        "GetChatWindowMessages",
        lua.create_function(|_, _id: i32| Ok(mlua::MultiValue::new()))?,
    )?;
    g.set(
        "GetChatWindowChannels",
        lua.create_function(|_, _id: i32| Ok(mlua::MultiValue::new()))?,
    )?;
    g.set(
        "GetDefaultLanguage",
        lua.create_function(|_, ()| Ok("Common"))?,
    )?;
    g.set(
        "GetAlternativeDefaultLanguage",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    Ok(())
}

/// C_Macro namespace - macro management stubs.
fn register_c_macro(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "SetMacroExecuteLineCallback",
        lua.create_function(|_, _cb: Value| Ok(()))?,
    )?;
    t.set("GetMacroName", lua.create_function(get_macro_name)?)?;
    t.set(
        "GetSelectedMacroIcon",
        lua.create_function(get_selected_macro_icon)?,
    )?;
    t.set("GetMacroInfo", lua.create_function(get_macro_info)?)?;
    t.set("GetNumMacros", lua.create_function(get_num_macros)?)?;
    lua.globals().set("C_Macro", t)?;
    register_macro_globals(lua)?;
    Ok(())
}

fn register_macro_globals(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set("GetMacroInfo", lua.create_function(get_macro_info)?)?;
    globals.set("GetNumMacros", lua.create_function(get_num_macros)?)?;
    globals.set(
        "GetMacroIndexByName",
        lua.create_function(get_macro_index_by_name)?,
    )?;
    Ok(())
}

fn get_num_macros(_: &Lua, (): ()) -> Result<(i32, i32)> {
    Ok((
        SEEDED_ACCOUNT_MACROS.len() as i32,
        SEEDED_CHARACTER_MACROS.len() as i32,
    ))
}

fn get_macro_info(lua: &Lua, macro_id: Value) -> Result<MultiValue> {
    let Some(macro_info) = lookup_macro(macro_id) else {
        return Ok(MultiValue::from_vec(vec![Value::Nil]));
    };
    Ok(MultiValue::from_vec(vec![
        Value::String(lua.create_string(macro_info.name)?),
        Value::String(lua.create_string(macro_info.icon)?),
        Value::String(lua.create_string(macro_info.body)?),
    ]))
}

fn get_macro_name(lua: &Lua, macro_id: Value) -> Result<Value> {
    match lookup_macro(macro_id) {
        Some(macro_info) => Ok(Value::String(lua.create_string(macro_info.name)?)),
        None => Ok(Value::Nil),
    }
}

fn get_selected_macro_icon(lua: &Lua, macro_id: Value) -> Result<Value> {
    match lookup_macro(macro_id) {
        Some(macro_info) => Ok(Value::String(lua.create_string(macro_info.icon)?)),
        None => Ok(Value::Integer(0)),
    }
}

fn get_macro_index_by_name(_: &Lua, name: String) -> Result<i32> {
    Ok(all_seeded_macros()
        .find(|macro_info| macro_info.name.eq_ignore_ascii_case(&name))
        .map(|macro_info| macro_info.id)
        .unwrap_or(0))
}

fn lookup_macro(macro_id: Value) -> Option<&'static SeededMacro> {
    let macro_id = match macro_id {
        Value::Integer(index) => i32::try_from(index).ok()?,
        Value::Number(index) => index as i32,
        Value::String(name) => return lookup_macro_by_name(name.to_string_lossy().as_ref()),
        _ => return None,
    };
    lookup_macro_by_id(macro_id)
}

fn lookup_macro_by_id(macro_id: i32) -> Option<&'static SeededMacro> {
    all_seeded_macros().find(|macro_info| macro_info.id == macro_id)
}

fn lookup_macro_by_name(name: &str) -> Option<&'static SeededMacro> {
    all_seeded_macros().find(|macro_info| macro_info.name.eq_ignore_ascii_case(name))
}

fn all_seeded_macros() -> impl Iterator<Item = &'static SeededMacro> {
    SEEDED_ACCOUNT_MACROS
        .iter()
        .chain(SEEDED_CHARACTER_MACROS.iter())
}

fn register_c_wowlabs_matchmaking(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetCurrentParty",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetPartyPlaylistEntry",
        lua.create_function(|_, ()| Ok(mlua::Value::Nil))?,
    )?;
    t.set("ClearFastLogin", lua.create_function(|_, ()| Ok(()))?)?;
    t.set(
        "SetAutoQueueOnLogout",
        lua.create_function(|_, _flag: bool| Ok(()))?,
    )?;
    lua.globals().set("C_WoWLabsMatchmaking", t)?;

    // C_WowLabsDataManager (note: different casing from C_WoWLabsMatchmaking)
    let dm = lua.create_table()?;
    dm.set("IsInPrematch", lua.create_function(|_, ()| Ok(false))?)?;
    lua.globals().set("C_WowLabsDataManager", dm)?;
    Ok(())
}
