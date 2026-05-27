//! Lua `UIWidgetContainerMixin` surface.
//!
//! This is a global mixin used by Blizzard XML templates, not a `C_*`
//! namespace. Keep it in the Lua globals layer so `c_api` remains reserved
//! for state-backed C API compatibility contracts.

use crate::c_api::set_global_val;
use crate::lua_api::methods::{borrow_state, create_table, frame_id_from_stack};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_widget_container_mixin(state: &mut LuaState) -> LuaResult<()> {
    let mixin = create_table(state);
    let Val::Table(mixin_ref) = mixin else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn_static(state, mixin_ref, "OnLoad", ui_widget_container_on_load)?;
    table_set_rust_fn_static(
        state,
        mixin_ref,
        "GetNumWidgetsShowing",
        ui_widget_container_get_num_widgets_showing,
    )?;
    set_global_val(state, "UIWidgetContainerMixin", mixin);
    Ok(())
}

fn ui_widget_container_get_num_widgets_showing(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(frame_id)
            .map(|frame| {
                frame
                    .children
                    .iter()
                    .filter(|&&child_id| {
                        sim.widgets
                            .get(child_id)
                            .map(|child| child.visible)
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0) as f64
    };
    state.push(Val::Num(count));
    Ok(1)
}

fn ui_widget_container_on_load(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
