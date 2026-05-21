//! SpellBook temporary call-pet shim.
//!
//! Pet call spell metadata is not seeded yet. Blizzard and addon code can
//! probe the legacy global, so keep the current no-call-pet result explicit
//! here until pet spell data is modeled.

use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::LuaResult;
use rilua::Val;
use rilua::vm::state::LuaState;

pub(crate) fn register_spell_book_call_pet_shim(state: &mut LuaState) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        state.global,
        "GetCallPetSpellInfo",
        get_call_pet_spell_info,
    )
}

fn get_call_pet_spell_info(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}
