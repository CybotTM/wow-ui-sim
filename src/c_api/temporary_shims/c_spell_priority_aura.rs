//! C_Spell temporary priority-aura shim — priority aura ordering is not modeled.
//!
//! Most `C_Spell` methods are backed by local spell data or `SimState`; this
//! stopgap only exposes `IsPriorityAura(spellID)` as false until aura priority
//! metadata exists.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_spell_priority_aura(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Spell")?;
    table_set_rust_fn_static(state, ns, "IsPriorityAura", is_priority_aura)?;
    Ok(())
}

fn is_priority_aura(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}
