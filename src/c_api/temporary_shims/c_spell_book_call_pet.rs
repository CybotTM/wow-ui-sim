//! SpellBook temporary static shims.
//!
//! Pet call spell metadata is not seeded yet. Blizzard and addon code can
//! probe the legacy global, so keep the current no-call-pet result explicit
//! here until pet spell data is modeled. Spell override replacement is also
//! unmodeled; `C_SpellBook` keeps the same identity fallback as `C_Spell`.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::LuaResult;
use rilua::Val;
use rilua::vm::state::LuaState;

pub(crate) fn register_spell_book_static_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_SpellBook")?;
    table_set_rust_fn_static(state, ns, "GetOverrideSpell", get_override_spell)?;
    table_set_rust_fn_static(state, ns, "FindSpellOverrideByID", get_override_spell)?;
    table_set_rust_fn_static(
        state,
        state.global,
        "GetCallPetSpellInfo",
        get_call_pet_spell_info,
    )?;
    Ok(())
}

fn get_override_spell(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Num(spell_id as f64));
    Ok(1)
}

fn get_call_pet_spell_info(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}
