//! C_Spell temporary classification shims — passive/ranged/hold metadata is not modeled.
//!
//! The simulator has local spell names, icons, targeting, and cooldowns, but
//! not full class metadata for passive, ranged auto-attack, or press-hold
//! spells. These probes keep the addon-facing API shape while returning false.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

type SpellClassificationFn = fn(&mut LuaState) -> LuaResult<u32>;

const SPELL_CLASSIFICATION_METHODS: &[(&str, SpellClassificationFn)] = &[
    ("IsSpellPassive", return_false_for_spell_id),
    ("IsRangedAutoAttackSpell", return_false_for_spell_id),
    ("IsPressHoldReleaseSpell", return_false_for_spell_id),
];

pub(crate) fn register_c_spell_classification_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Spell")?;
    for &(name, func) in SPELL_CLASSIFICATION_METHODS {
        table_set_rust_fn_static(state, ns, name, func)?;
    }
    Ok(())
}

fn return_false_for_spell_id(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}
