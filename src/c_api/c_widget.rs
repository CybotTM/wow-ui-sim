use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::extract_frame_id;
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_widget_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Widget")?;
    table_set_rust_fn_static(state, table_ref, "IsFrameWidget", is_frame_widget)?;
    Ok(())
}

fn is_frame_widget(state: &mut LuaState) -> LuaResult<u32> {
    let is_frame = extract_frame_id(state, stack_val(state, 1)).is_some();
    state.push(Val::Bool(is_frame));
    Ok(1)
}
