//! C_UIWidgetManager temporary power-bar shim.
//!
//! Power-bar widget set state is not modeled yet. Blizzard callers treat `0`
//! as "no widget set", so keep that inert compatibility value isolated here
//! instead of patching namespace stubs at runtime.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_ui_widget_manager_power_bar(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_UIWidgetManager")?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetPowerBarWidgetSetID",
        get_power_bar_widget_set_id,
    )?;
    Ok(())
}

fn get_power_bar_widget_set_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}
