//! C_MajorFactions temporary display-policy shims.
//!
//! Renown faction data is modeled in `c_major_factions`; expansion-page
//! visibility, reward-track policy, and renown reward display are not. Keep the
//! inert defaults here until those UI policy flags are seeded.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::LuaResult;
use rilua::Val;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_major_faction_display_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_MajorFactions")?;
    table_set_rust_fn_static(
        state,
        ns,
        "IsMajorFactionHiddenFromExpansionPage",
        is_major_faction_hidden_from_expansion_page,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "ShouldDisplayMajorFactionAsJourney",
        should_display_major_faction_as_journey,
    )?;
    table_set_rust_fn_static(state, ns, "HasMaximumRenown", return_false)?;
    table_set_rust_fn_static(state, ns, "GetCurrentRenownLevel", return_one)?;
    table_set_rust_fn_static(state, ns, "GetRenownRewardsForLevel", return_empty_table)?;
    table_set_rust_fn_static(state, ns, "ShouldUseJourneyRewardTrack", return_false)?;
    table_set_rust_fn_static(state, ns, "GetRenownNPCFactionID", return_zero)?;
    Ok(())
}

fn is_major_faction_hidden_from_expansion_page(state: &mut LuaState) -> LuaResult<u32> {
    let _ = i64::from_stack(state, 1);
    state.push(Val::Bool(false));
    Ok(1)
}

fn should_display_major_faction_as_journey(state: &mut LuaState) -> LuaResult<u32> {
    let _ = i64::from_stack(state, 1);
    return_false(state)
}

fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn return_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn return_one(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(1.0));
    Ok(1)
}

fn return_empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}
