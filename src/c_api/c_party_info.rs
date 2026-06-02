//! `C_PartyInfo` probe surface backed by group state.
//!
//! `GetActiveCategories`, `GetActiveGroupType`, and `IsPartyFull` read the
//! existing party roster model. `LeaveParty` mutates the same roster path as the
//! legacy global `LeaveParty`. Static loot-method defaults remain here because
//! they are coherent seeded `C_PartyInfo` values, while unrelated instance
//! abandon defaults stay in temporary workarounds.

use crate::c_api::helpers::{ensure_namespace, set_table_array};
use crate::lua_api::globals::group_queries::active_party_count;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::FromStack;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

const AVAILABLE_LOOT_METHODS: [i32; 5] = [0, 1, 2, 3, 4];

pub(crate) fn register_c_party_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_PartyInfo")?;
    register_group_membership_probes(state, table_ref)?;
    register_loot_method_probes(state, table_ref)?;
    Ok(())
}

fn register_group_membership_probes(
    state: &mut LuaState,
    table_ref: GcRef<Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetActiveCategories",
        c_party_info_get_active_categories,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetActiveGroupType",
        c_party_info_get_active_group_type,
    )?;
    table_set_rust_fn_static(state, table_ref, "IsPartyFull", c_party_info_is_party_full)?;
    table_set_rust_fn_static(state, table_ref, "LeaveParty", c_party_info_leave_party)?;
    Ok(())
}

fn register_loot_method_probes(state: &mut LuaState, table_ref: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetAvailableLootMethods",
        c_party_info_get_available_loot_methods,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "IsLootMethodAvailable",
        c_party_info_is_loot_method_available,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetLootMethod",
        c_party_info_get_loot_method,
    )?;
    Ok(())
}

fn c_party_info_get_active_categories(state: &mut LuaState) -> LuaResult<u32> {
    let member_count = active_party_count(state)?;
    let array = create_table(state);
    if member_count > 0 {
        set_table_array(state, array, 1, Val::Num(1.0));
    }
    state.push(array);
    Ok(1)
}

fn c_party_info_get_active_group_type(state: &mut LuaState) -> LuaResult<u32> {
    let member_count = active_party_count(state)?;
    if member_count == 0 {
        state.push(Val::Nil);
    } else if member_count >= 6 {
        state.push(Val::Num(1.0));
    } else {
        state.push(Val::Num(0.0));
    }
    Ok(1)
}

fn c_party_info_is_party_full(state: &mut LuaState) -> LuaResult<u32> {
    let member_count = active_party_count(state)?;
    let full = if member_count == 0 {
        false
    } else if member_count >= 6 {
        member_count >= 39
    } else {
        member_count >= 4
    };
    state.push(Val::Bool(full));
    Ok(1)
}

fn c_party_info_leave_party(state: &mut LuaState) -> LuaResult<u32> {
    crate::lua_api::globals::group_verbs::clear_party_roster(state)?;
    crate::lua_api::globals::group_verbs::push_event(state, "GROUP_ROSTER_UPDATE")?;
    Ok(0)
}

fn c_party_info_get_available_loot_methods(state: &mut LuaState) -> LuaResult<u32> {
    let array = create_table(state);
    for (index, method) in AVAILABLE_LOOT_METHODS.iter().enumerate() {
        set_table_array(state, array, index as i64 + 1, Val::Num(*method as f64));
    }
    state.push(array);
    Ok(1)
}

fn c_party_info_is_loot_method_available(state: &mut LuaState) -> LuaResult<u32> {
    let method = i32::from_stack(state, 1)?;
    let available = matches!(method, 0..=4);
    state.push(Val::Bool(available));
    Ok(1)
}

fn c_party_info_get_loot_method(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(3.0));
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(3)
}
