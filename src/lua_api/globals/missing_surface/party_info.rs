//! `C_PartyInfo` probe surface backed by the existing
//! `SimState.party_members` / `party_group_active` fields.
//!
//! Migrates 10 entries off the namespace stub tables:
//!
//! - `C_PartyInfo.GetActiveCategories()` — empty array when solo;
//!   `{1}` (Home category) when in a party or raid.
//! - `C_PartyInfo.GetActiveGroupType()` — 1 if in raid (≥6 members),
//!   0 if in party, nil when solo.
//! - `C_PartyInfo.IsPartyFull()` — true when party is at capacity
//!   (≥5 for party, ≥40 for raid).
//! - `C_PartyInfo.IsPartyInJailersTower()` — always false (Torghast stub).
//! - `C_PartyInfo.GetInviteConfirmationInfo(guid)` — nil for all guids.

use super::{ensure_namespace, set_table_array};
use crate::lua_api::globals::group_queries::active_party_count;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::FromStack;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

const AVAILABLE_LOOT_METHODS: [i32; 5] = [0, 1, 2, 3, 4];

pub(super) fn register_party_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_PartyInfo")?;
    register_group_membership_probes(state, table_ref)?;
    register_loot_method_probes(state, table_ref)?;
    register_invite_and_tower_stubs(state, table_ref)?;
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

fn register_invite_and_tower_stubs(state: &mut LuaState, table_ref: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table_ref,
        "IsPartyInJailersTower",
        c_party_info_is_party_in_jailers_tower,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetInviteConfirmationInfo",
        c_party_info_get_invite_confirmation_info,
    )?;
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

/// `GetActiveCategories()` — returns `{}` when solo, `{1}` (Home) when
/// grouped. The category id `1` corresponds to `Enum.PartyCategory.Home`.
fn c_party_info_get_active_categories(state: &mut LuaState) -> LuaResult<u32> {
    let member_count = active_party_count(state)?;
    let array = create_table(state);
    if member_count > 0 {
        set_table_array(state, array, 1, Val::Num(1.0));
    }
    state.push(array);
    Ok(1)
}

/// `GetActiveGroupType()` — 1 for raid (party ≥ 6), 0 for party, nil
/// when solo. Mirrors `Enum.PartyGroupType.Raid = 1, Party = 0`.
fn c_party_info_get_active_group_type(state: &mut LuaState) -> LuaResult<u32> {
    let member_count = active_party_count(state)?;
    if member_count == 0 {
        state.push(Val::Nil);
    } else if member_count >= 6 {
        state.push(Val::Num(1.0)); // Raid
    } else {
        state.push(Val::Num(0.0)); // Party
    }
    Ok(1)
}

/// `IsPartyFull()` — true when at the capacity limit for the group type.
/// Party capacity is 5 members (excluding player), raid is 40 (excluding
/// player), so full party = 4 party slots filled + player = 5 total,
/// meaning party_members.len() >= 4. Retail: party can hold 4 others +
/// player = 5 total; raid holds 39 others + player = 40. We count
/// party_members (others only), so full = 4 for party, 39 for raid.
fn c_party_info_is_party_full(state: &mut LuaState) -> LuaResult<u32> {
    let member_count = active_party_count(state)?;
    let full = if member_count == 0 {
        false
    } else if member_count >= 6 {
        // In raid mode: full at 39 others + player = 40 total.
        member_count >= 39
    } else {
        // In party mode: full at 4 others + player = 5 total.
        member_count >= 4
    };
    state.push(Val::Bool(full));
    Ok(1)
}

/// `LeaveParty()` — namespace entry point for the same group state mutation
/// used by the legacy global `LeaveParty()`.
fn c_party_info_leave_party(state: &mut LuaState) -> LuaResult<u32> {
    crate::lua_api::globals::group_verbs::clear_party_roster(state)?;
    crate::lua_api::globals::group_verbs::push_event(state, "GROUP_ROSTER_UPDATE")?;
    Ok(0)
}

/// `IsPartyInJailersTower()` — Torghast layer probe; always false in the
/// simulator (no Torghast state modelled).
fn c_party_info_is_party_in_jailers_tower(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// `GetInviteConfirmationInfo(guid)` — returns nil for all guids.
/// In retail returns `(inviterName, relationship, friendInfo, isQuickJoin,
/// clubId)` for a pending invite. Sim has no invite queue.
fn c_party_info_get_invite_confirmation_info(state: &mut LuaState) -> LuaResult<u32> {
    let _ = state; // consume argument
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
