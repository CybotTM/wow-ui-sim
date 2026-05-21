//! C_MajorFactions temporary display-policy shims.
//!
//! Renown faction data is modeled in `c_major_factions`; expansion-page
//! visibility and journey display policy are not. Keep the inert defaults here
//! until those UI policy flags are seeded.

use crate::c_api::helpers::ensure_namespace;
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
    Ok(())
}

fn is_major_faction_hidden_from_expansion_page(state: &mut LuaState) -> LuaResult<u32> {
    let _ = i64::from_stack(state, 1);
    state.push(Val::Bool(false));
    Ok(1)
}

fn should_display_major_faction_as_journey(state: &mut LuaState) -> LuaResult<u32> {
    let _ = i64::from_stack(state, 1);
    state.push(Val::Bool(false));
    Ok(1)
}
