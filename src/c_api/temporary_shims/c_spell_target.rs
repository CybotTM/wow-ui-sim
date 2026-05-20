//! C_Spell temporary target-spell shims — target spell metadata is not modeled.
//!
//! These probes describe the spell currently selected as a target action. The
//! simulator does not yet track that metadata, so each method returns false
//! until a real target-spell state model exists.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

type SpellTargetFn = fn(&mut LuaState) -> LuaResult<u32>;

const SPELL_TARGET_METHODS: &[(&str, SpellTargetFn)] = &[
    ("TargetSpellIsEnchanting", return_false),
    ("TargetSpellJumpsUpgradeTrack", return_false),
    ("TargetSpellReplacesBonusTree", return_false),
];

pub(crate) fn register_c_spell_target_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Spell")?;
    for &(name, func) in SPELL_TARGET_METHODS {
        table_set_rust_fn_static(state, ns, name, func)?;
    }
    Ok(())
}

fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}
