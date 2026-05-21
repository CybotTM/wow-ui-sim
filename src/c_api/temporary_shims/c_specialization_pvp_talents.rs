//! C_SpecializationInfo temporary PvP talent shims.
//!
//! Core specialization data is modeled in `c_spec`, but PvP talent rows,
//! selections, and lock state are not seeded yet. Keep the inert values that
//! Blizzard talent code expects isolated here until that state exists.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::{create_string, create_table, table_set_static};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::LuaResult;
use rilua::Val;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_specialization_pvp_talent_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_SpecializationInfo")?;
    table_set_rust_fn_static(state, ns, "GetPvpTalentSlotInfo", get_pvp_talent_slot_info)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetPvpTalentSlotUnlockLevel",
        get_pvp_talent_slot_unlock_level,
    )?;
    table_set_rust_fn_static(state, ns, "GetPvpTalentInfo", get_pvp_talent_info)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetPvpTalentUnlockLevel",
        get_pvp_talent_unlock_level,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetInspectSelectedPvpTalent",
        get_inspect_selected_pvp_talent,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetAllSelectedPvpTalentIDs",
        get_all_selected_pvp_talent_ids,
    )?;
    table_set_rust_fn_static(state, ns, "IsPvpTalentLocked", is_pvp_talent_locked)?;
    table_set_rust_fn_static(state, ns, "SetPvpTalentLocked", set_pvp_talent_locked)?;
    Ok(())
}

fn get_pvp_talent_slot_info(state: &mut LuaState) -> LuaResult<u32> {
    let slot_index = match stack_val(state, 1) {
        Val::Num(n) => n as i32,
        _ => 0,
    };
    if !is_pvp_talent_slot_index_valid(slot_index) {
        state.push(Val::Nil);
        return Ok(1);
    }

    let info = create_table(state);
    let available_talent_ids = create_table(state);
    table_set_static(state, info, "enabled", Val::Bool(true));
    table_set_static(state, info, "locked", Val::Bool(false));
    table_set_static(
        state,
        info,
        "level",
        Val::Num(20.0 + ((slot_index - 1) * 10) as f64),
    );
    table_set_static(state, info, "selectedTalentID", Val::Nil);
    table_set_static(state, info, "slotIndex", Val::Num(slot_index as f64));
    table_set_static(state, info, "availableTalentIDs", available_talent_ids);
    state.push(info);
    Ok(1)
}

fn is_pvp_talent_slot_index_valid(slot_index: i32) -> bool {
    const FIRST_PVP_TALENT_SLOT: i32 = 1;
    const LAST_PVP_TALENT_SLOT: i32 = 3;

    matches!(slot_index, FIRST_PVP_TALENT_SLOT..=LAST_PVP_TALENT_SLOT)
}

fn get_pvp_talent_slot_unlock_level(state: &mut LuaState) -> LuaResult<u32> {
    let slot_index = match stack_val(state, 1) {
        Val::Num(n) => n as i32,
        _ => 0,
    };
    let level = match slot_index {
        1 => 20.0,
        2 => 30.0,
        3 => 40.0,
        _ => 0.0,
    };
    state.push(Val::Num(level));
    Ok(1)
}

fn get_pvp_talent_info(state: &mut LuaState) -> LuaResult<u32> {
    let talent_id = match stack_val(state, 1) {
        Val::Num(n) => n as i32,
        _ => 0,
    };
    if talent_id <= 0 {
        state.push(Val::Nil);
        return Ok(1);
    }

    let info = create_table(state);
    let name = create_string(state, "PvP Talent");
    let icon = create_string(state, "Interface\\Icons\\Spell_Holy_PowerWordShield");
    table_set_static(state, info, "talentID", Val::Num(talent_id as f64));
    table_set_static(state, info, "name", name);
    table_set_static(state, info, "icon", icon);
    table_set_static(state, info, "unlocked", Val::Bool(true));
    table_set_static(state, info, "dependenciesUnmet", Val::Bool(false));
    table_set_static(state, info, "dependenciesUnmetReason", Val::Nil);
    state.push(info);
    Ok(1)
}

fn get_pvp_talent_unlock_level(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(20.0));
    Ok(1)
}

fn get_inspect_selected_pvp_talent(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn get_all_selected_pvp_talent_ids(state: &mut LuaState) -> LuaResult<u32> {
    let selected = create_table(state);
    state.push(selected);
    Ok(1)
}

fn is_pvp_talent_locked(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn set_pvp_talent_locked(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
