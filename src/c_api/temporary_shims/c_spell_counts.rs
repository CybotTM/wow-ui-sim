//! C_Spell temporary count shims — spell cast/display counts are not modeled.
//!
//! ZoneAbility and SpellFlyout use these as fallback display counts when no
//! charge or reagent count is available. Until the simulator tracks that
//! spell-count state, both methods return the inert baseline `0`.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_spell_count_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Spell")?;
    table_set_rust_fn_static(state, ns, "GetSpellCastCount", get_spell_cast_count)?;
    table_set_rust_fn_static(state, ns, "GetSpellDisplayCount", get_spell_display_count)?;
    Ok(())
}

fn get_spell_cast_count(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Num(0.0));
    Ok(1)
}

fn get_spell_display_count(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    let _max_display_count = Option::<u32>::from_stack(state, 2)?;
    state.push(Val::Num(0.0));
    Ok(1)
}
