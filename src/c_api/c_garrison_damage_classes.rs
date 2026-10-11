//! Simulator-defined enUS pure-school mask labels for garrison combat logs.
//! This catalog does not establish native catalog ordering or locale parity.

use crate::c_api::helpers::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{create_table, create_table_with_fields};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const DAMAGE_CLASSES: [(u8, &str); 7] = [
    (1, "Physical"),
    (2, "Holy"),
    (4, "Fire"),
    (8, "Nature"),
    (16, "Frost"),
    (32, "Shadow"),
    (64, "Arcane"),
];

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_Garrison")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetAutoCombatDamageClassValues",
        damage_class_values,
    )?;
    Ok(())
}

fn damage_class_values(state: &mut LuaState) -> LuaResult<u32> {
    let array = create_table(state);
    for (index, (mask, label)) in DAMAGE_CLASSES.iter().enumerate() {
        let label = Val::Str(state.gc.intern_string_static(label.as_bytes()));
        let row = create_table_with_fields(
            state,
            &[
                ("damageClassValue", Val::Num(f64::from(*mask))),
                ("locString", label),
            ],
        );
        set_table_array(state, array, index as i64 + 1, row);
    }
    state.push(array);
    Ok(1)
}
