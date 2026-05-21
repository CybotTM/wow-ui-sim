//! C_SpecializationInfo temporary mastery-spell shim.
//!
//! The simulator models core specialization identity and spell displays, but
//! not mastery spell rows yet. Deprecated specialization wrappers expect this
//! API to return an iterable table, so keep the empty-table compatibility shape
//! isolated here until mastery data is seeded.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_specialization_mastery_shim(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_SpecializationInfo")?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetSpecializationMasterySpells",
        get_specialization_mastery_spells,
    )?;
    Ok(())
}

fn get_specialization_mastery_spells(state: &mut LuaState) -> LuaResult<u32> {
    let spells = create_table(state);
    state.push(spells);
    Ok(1)
}
