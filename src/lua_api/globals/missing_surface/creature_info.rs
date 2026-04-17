//! `C_CreatureInfo` helpers backed by static game data.

use super::ensure_namespace;
use crate::lua_api::game_data::class_info_by_index;
use crate::lua_api::methods::{create_string, create_table, table_set};
use crate::lua_bridge::FromStack;
use crate::lua_bridge::table_set_rust_fn;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_creature_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_CreatureInfo")?;
    table_set_rust_fn(
        state,
        table_ref,
        "GetClassInfo",
        c_creature_info_get_class_info,
    )?;
    Ok(())
}

fn c_creature_info_get_class_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1).unwrap_or(1);
    let (_class_label, class_file, class_id) = class_info_by_index(index);
    let info = create_table(state);
    let class_name = create_string(state, class_file);
    let class_file = create_string(state, class_file);
    table_set(state, info, "className", class_name);
    table_set(state, info, "classFile", class_file);
    table_set(state, info, "classID", Val::Num(class_id as f64));
    state.push(info);
    Ok(1)
}
