//! Small C_* namespace registrations that were previously flat stubs.
//!
//! Migrates 9 entries off the namespace stub tables:
//!
//! - `C_TrophyHall.GetTrophyInfo()` — nil (no trophy data in simulator)
//! - `C_StableInfo.IsAtPetStable()` — reads `SimState.pet_stables_open`
//! - `C_GarrisonInfo.HasGarrison()` — false (garrison not simulated)
//! - `C_GarrisonInfo.GetGarrisonType()` — 0 (no garrison type)
//! - `C_AssistedCombat.*` — empty rotation/action spell state by default
//! - `C_Map.IsMapValidForNavigation(uiMapID)` — false (no nav mesh)
//! - `C_PvP.IsMatchConsideredArena()` — false (not in arena by default)
//! - `C_LossOfControl.*` — one seeded Kidney Shot entry for the player
//! - `C_Bank.HasFullBankAccess()` — true (permissive default)
//! - `C_NewItems.*` — permissive empty/default probes
//! - `C_VignetteInfo.*` — empty vignette set for world-map refreshes
//! - `C_EventUtils.*` — backed by the generated WoW event registry

use super::ensure_namespace;
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_bridge::FromStack;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_small_namespaces(state: &mut LuaState) -> LuaResult<()> {
    register_flat_namespace(state, "C_TrophyHall", C_TROPHY_HALL_METHODS)?;
    register_flat_namespace(state, "C_StableInfo", C_STABLE_INFO_METHODS)?;
    register_flat_namespace(state, "C_GarrisonInfo", C_GARRISON_INFO_METHODS)?;
    register_flat_namespace(state, "C_AssistedCombat", C_ASSISTED_COMBAT_METHODS)?;
    register_flat_namespace(state, "C_Map", C_MAP_METHODS)?;
    register_flat_namespace(state, "C_PvP", C_PVP_METHODS)?;
    register_flat_namespace(state, "C_LossOfControl", C_LOSS_OF_CONTROL_METHODS)?;
    register_flat_namespace(state, "C_Bank", C_BANK_METHODS)?;
    register_flat_namespace(state, "C_NewItems", C_NEW_ITEMS_METHODS)?;
    register_flat_namespace(state, "C_VignetteInfo", C_VIGNETTE_INFO_METHODS)?;
    register_flat_namespace(state, "C_Console", C_CONSOLE_METHODS)?;
    register_flat_namespace(state, "C_Covenants", C_COVENANTS_METHODS)?;
    register_flat_namespace(state, "C_Soulbinds", C_SOULBINDS_METHODS)?;
    register_flat_namespace(state, "C_LevelLink", C_LEVEL_LINK_METHODS)?;
    register_flat_namespace(state, "C_EventUtils", C_EVENT_UTILS_METHODS)?;
    Ok(())
}

/// Look up (or create) the namespace table and register each
/// `(Lua name, RustFn)` pair on it. Keeps the main registrar a
/// per-namespace pipeline.
fn register_flat_namespace(
    state: &mut LuaState,
    namespace: &'static str,
    methods: &[(&'static str, rilua::vm::closure::RustFn)],
) -> LuaResult<()> {
    let ns = ensure_namespace(state, namespace)?;
    for (name, func) in methods {
        table_set_rust_fn_static(state, ns, name, *func)?;
    }
    Ok(())
}

const C_TROPHY_HALL_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] =
    &[("GetTrophyInfo", c_trophy_hall_get_trophy_info)];

const C_STABLE_INFO_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] =
    &[("IsAtPetStable", c_stable_info_is_at_pet_stable)];

const C_GARRISON_INFO_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] = &[
    ("HasGarrison", c_garrison_info_has_garrison),
    ("GetGarrisonType", c_garrison_info_get_garrison_type),
];

const C_ASSISTED_COMBAT_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] = &[
    ("GetActionSpell", c_assisted_combat_get_action_spell),
    ("GetNextCastSpell", c_assisted_combat_get_next_cast_spell),
    ("GetRotationSpells", c_assisted_combat_get_rotation_spells),
    ("IsAvailable", c_assisted_combat_is_available),
];

const C_MAP_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] =
    &[("IsMapValidForNavigation", c_map_is_map_valid_for_navigation)];

const C_PVP_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] = &[
    ("GetActiveMatchState", c_pvp_get_active_match_state),
    ("GetCustomVictoryStatID", c_pvp_get_custom_victory_stat_id),
    ("GetPVPDesired", super::super::pvp_probes::get_pvp_desired),
    (
        "GetPVPLastHonorGain",
        super::super::pvp_probes::get_pvp_last_honor_gain,
    ),
    (
        "GetHolidayBGInfo",
        super::super::pvp_probes::get_holiday_bg_info,
    ),
    (
        "GetRandomBGInfo",
        super::super::pvp_probes::get_holiday_bg_info,
    ),
    ("GetLocklistMap", super::super::pvp_probes::get_locklist_map),
    (
        "GetLocklistMapName",
        super::super::pvp_probes::get_locklist_map_name,
    ),
    ("IsActiveBattlefield", c_pvp_is_active_battlefield),
    ("IsMatchConsideredArena", c_pvp_is_match_considered_arena),
    (
        "IsInActiveWorldPVP",
        super::super::pvp_probes::is_in_active_world_pvp,
    ),
    ("IsSubZonePVP", super::super::pvp_probes::is_sub_zone_pvp),
    ("IsRatedArena", c_pvp_is_rated_arena),
    ("IsRatedBattleground", c_pvp_is_rated_battleground),
    ("IsRatedMap", c_pvp_is_rated_map),
    ("IsRatedSoloRBG", c_pvp_is_rated_solo_rbg),
    ("IsRatedSoloShuffle", c_pvp_is_rated_solo_shuffle),
    ("IsSoloRBG", c_pvp_is_solo_rbg),
    (
        "GetWorldPVPAreaInfo",
        super::super::pvp_probes::get_world_pvp_area_info,
    ),
    (
        "GetPvpTalentsUnlockedLevel",
        c_pvp_get_pvp_talents_unlocked_level,
    ),
    (
        "GetWarModeRewardBonusDefault",
        c_pvp_get_war_mode_reward_bonus_default,
    ),
    ("GetWarModeRewardBonus", c_pvp_get_war_mode_reward_bonus),
    ("SetLocklistMap", super::super::pvp_probes::set_locklist_map),
    (
        "ClearLocklistMap",
        super::super::pvp_probes::clear_locklist_map,
    ),
];

const C_LOSS_OF_CONTROL_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] = &[
    (
        "GetActiveLossOfControlData",
        c_loc_get_active_loss_of_control_data,
    ),
    (
        "GetActiveLossOfControlDataCount",
        c_loc_get_active_loss_of_control_data_count,
    ),
    (
        "GetActiveLossOfControlDataCountByUnit",
        c_loc_get_active_loss_of_control_data_count_by_unit,
    ),
    (
        "GetActiveLossOfControlDataByUnit",
        c_loc_get_active_loss_of_control_data_by_unit,
    ),
    (
        "GetActiveLossOfControlDuration",
        c_loc_get_active_loss_of_control_duration,
    ),
];

const C_BANK_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] =
    &[("HasFullBankAccess", c_bank_has_full_bank_access)];

const C_NEW_ITEMS_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] = &[
    ("IsNewItem", c_new_items_is_new_item),
    ("RemoveNewItem", c_new_items_remove_new_item),
    ("ClearAll", c_new_items_clear_all),
];

const C_VIGNETTE_INFO_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] = &[
    ("GetVignettes", c_vignette_info_get_vignettes),
    ("GetVignetteInfo", c_vignette_info_get_vignette_info),
];

const C_EVENT_UTILS_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] = &[
    ("IsEventValid", c_event_utils_is_event_valid),
    ("IsCallbackEvent", c_event_utils_is_callback_event),
];

const C_CONSOLE_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] = &[
    ("GetAllCommands", c_console_get_all_commands),
    ("GetColorFromType", c_console_get_color_from_type),
];

const C_COVENANTS_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] = &[
    ("GetCovenantData", c_covenants_get_covenant_data),
    ("GetActiveCovenantID", c_covenants_get_active_covenant_id),
    ("GetCovenantIDs", c_covenants_get_covenant_ids),
];

const C_LEVEL_LINK_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] =
    &[("IsActionLocked", c_level_link_is_action_locked)];

const C_SOULBINDS_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] = &[
    ("GetActiveSoulbindID", c_soulbinds_get_active_soulbind_id),
    ("GetSoulbindData", c_soulbinds_get_soulbind_data),
    ("GetConduitCollection", c_soulbinds_get_conduit_collection),
    (
        "GetConduitCollectionData",
        c_soulbinds_get_conduit_collection_data,
    ),
    ("IsConduitInstalled", c_soulbinds_is_conduit_installed),
];

fn c_trophy_hall_get_trophy_info(_state: &mut LuaState) -> LuaResult<u32> {
    // No trophy-hall data in the simulator.
    Ok(0)
}

fn c_stable_info_is_at_pet_stable(state: &mut LuaState) -> LuaResult<u32> {
    let open = borrow_state(state)?.pet_stables_open;
    state.push(Val::Bool(open));
    Ok(1)
}

fn c_garrison_info_has_garrison(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_garrison_info_get_garrison_type(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn c_assisted_combat_get_action_spell(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn c_assisted_combat_get_next_cast_spell(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = borrow_state(state)?.assisted_combat.next_cast_spell_id;
    match spell_id {
        Some(spell_id) => state.push(Val::Num(spell_id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_assisted_combat_get_rotation_spells(state: &mut LuaState) -> LuaResult<u32> {
    let spells = create_table(state);
    state.push(spells);
    Ok(1)
}

fn c_assisted_combat_is_available(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    let reason = create_string(state, "Not available");
    state.push(reason);
    Ok(2)
}

fn c_map_is_map_valid_for_navigation(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_pvp_is_match_considered_arena(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_pvp_is_active_battlefield(state: &mut LuaState) -> LuaResult<u32> {
    let active = borrow_state(state)?.is_active_battlefield;
    state.push(Val::Bool(active));
    Ok(1)
}

fn c_level_link_is_action_locked(state: &mut LuaState) -> LuaResult<u32> {
    let is_locked = match crate::lua_bridge::stack_val(state, 1) {
        Val::Num(slot) => borrow_state(state)?
            .locked_action_slots
            .contains(&(slot as i32)),
        _ => false,
    };
    state.push(Val::Bool(is_locked));
    Ok(1)
}

fn c_pvp_get_active_match_state(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn c_pvp_get_custom_victory_stat_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn c_pvp_is_rated_arena(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_pvp_is_rated_battleground(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_pvp_is_rated_map(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_pvp_is_rated_solo_rbg(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_pvp_is_rated_solo_shuffle(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_pvp_is_solo_rbg(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_pvp_get_pvp_talents_unlocked_level(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(20.0));
    Ok(1)
}

fn c_pvp_get_war_mode_reward_bonus_default(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(10.0));
    Ok(1)
}

fn c_pvp_get_war_mode_reward_bonus(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(10.0));
    Ok(1)
}

fn c_loc_get_active_loss_of_control_data(state: &mut LuaState) -> LuaResult<u32> {
    let index = <i32 as FromStack>::from_stack(state, 1)?;
    if index != 1 {
        return Ok(0);
    }
    push_seeded_loss_of_control_data(state);
    Ok(1)
}

fn c_loc_get_active_loss_of_control_data_count(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(1.0));
    Ok(1)
}

fn c_loc_get_active_loss_of_control_data_count_by_unit(state: &mut LuaState) -> LuaResult<u32> {
    let unit = <String as FromStack>::from_stack(state, 1)?;
    let count = if unit == "player" { 1.0 } else { 0.0 };
    state.push(Val::Num(count));
    Ok(1)
}

fn c_loc_get_active_loss_of_control_data_by_unit(state: &mut LuaState) -> LuaResult<u32> {
    let unit = <String as FromStack>::from_stack(state, 1)?;
    let index = <i32 as FromStack>::from_stack(state, 2)?;
    if unit != "player" || index != 1 {
        return Ok(0);
    }
    push_seeded_loss_of_control_data(state);
    Ok(1)
}

fn c_loc_get_active_loss_of_control_duration(state: &mut LuaState) -> LuaResult<u32> {
    let unit = <String as FromStack>::from_stack(state, 1)?;
    let index = <i32 as FromStack>::from_stack(state, 2)?;
    if unit != "player" || index != 1 {
        state.push(Val::Nil);
        return Ok(1);
    }
    state.push(Val::Num(4.0));
    Ok(1)
}

fn push_seeded_loss_of_control_data(state: &mut LuaState) {
    let data = create_table(state);
    let display_text = create_string(state, "Kidney Shot");
    let icon_texture = create_string(state, "Interface\\Icons\\Ability_Rogue_KidneyShot");
    table_set(state, data, "spellID", Val::Num(408.0));
    table_set(state, data, "displayText", display_text);
    table_set(state, data, "iconTexture", icon_texture);
    table_set(state, data, "displayType", Val::Num(2.0));
    table_set(state, data, "timeRemaining", Val::Num(4.0));
    state.push(data);
}

fn c_bank_has_full_bank_access(state: &mut LuaState) -> LuaResult<u32> {
    // Permissive default — simulator grants full bank access.
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_new_items_is_new_item(state: &mut LuaState) -> LuaResult<u32> {
    let _ = state;
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_new_items_remove_new_item(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn c_new_items_clear_all(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn c_vignette_info_get_vignettes(state: &mut LuaState) -> LuaResult<u32> {
    let vignettes = create_table(state);
    state.push(vignettes);
    Ok(1)
}

fn c_vignette_info_get_vignette_info(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn c_event_utils_is_event_valid(state: &mut LuaState) -> LuaResult<u32> {
    let event_name = String::from_stack(state, 1).unwrap_or_default();
    state.push(Val::Bool(crate::event::is_valid_event(&event_name)));
    Ok(1)
}

fn c_event_utils_is_callback_event(state: &mut LuaState) -> LuaResult<u32> {
    let event_name = String::from_stack(state, 1).unwrap_or_default();
    state.push(Val::Bool(crate::event::is_callback_event(&event_name)));
    Ok(1)
}

fn c_console_get_all_commands(state: &mut LuaState) -> LuaResult<u32> {
    let commands = create_table(state);
    state.push(commands);
    Ok(1)
}

fn c_console_get_color_from_type(state: &mut LuaState) -> LuaResult<u32> {
    let color = create_table(state);
    table_set(state, color, "r", Val::Num(1.0));
    table_set(state, color, "g", Val::Num(1.0));
    table_set(state, color, "b", Val::Num(1.0));
    table_set(state, color, "a", Val::Num(1.0));
    state.push(color);
    Ok(1)
}

fn c_covenants_get_covenant_data(state: &mut LuaState) -> LuaResult<u32> {
    let covenant_id = match crate::lua_bridge::stack_val(state, 1) {
        Val::Num(value) if value.is_finite() => value as i32,
        _ => 0,
    };
    let (id, name) = match covenant_id {
        1 => (1, "Kyrian"),
        2 => (2, "Venthyr"),
        3 => (3, "Night Fae"),
        4 => (4, "Necrolord"),
        _ => (0, "None"),
    };
    let covenant = create_table(state);
    let name = create_string(state, name);
    table_set(state, covenant, "ID", Val::Num(id as f64));
    table_set(state, covenant, "name", name);
    state.push(covenant);
    Ok(1)
}

fn c_covenants_get_active_covenant_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn c_covenants_get_covenant_ids(state: &mut LuaState) -> LuaResult<u32> {
    let ids = create_table(state);
    if let Val::Table(table_ref) = &ids {
        for (index, value) in [1_i32, 2, 3, 4].into_iter().enumerate() {
            if let Some(table) = state.gc.tables.get_mut(*table_ref) {
                let _ = table.raw_set(
                    Val::Num((index + 1) as f64),
                    Val::Num(value as f64),
                    &state.gc.string_arena,
                );
            }
        }
    }
    state.push(ids);
    Ok(1)
}

fn c_soulbinds_get_active_soulbind_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn c_soulbinds_get_soulbind_data(state: &mut LuaState) -> LuaResult<u32> {
    let soulbind = create_table(state);
    let name = create_string(state, "");
    table_set(state, soulbind, "ID", Val::Num(0.0));
    table_set(state, soulbind, "name", name);
    table_set(state, soulbind, "covenantID", Val::Num(0.0));
    state.push(soulbind);
    Ok(1)
}

fn c_soulbinds_get_conduit_collection(state: &mut LuaState) -> LuaResult<u32> {
    let conduits = create_table(state);
    state.push(conduits);
    Ok(1)
}

fn c_soulbinds_get_conduit_collection_data(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn c_soulbinds_is_conduit_installed(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}
