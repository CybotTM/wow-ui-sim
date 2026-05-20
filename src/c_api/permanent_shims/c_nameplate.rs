//! C_NamePlate permanent shim — 3D world nameplate rendering is out of scope.
//!
//! Retail `GetNamePlateForUnit(unit)` returns a Frame rendered above the
//! unit's 3D model. The simulator has no 3D world nameplate renderer, so nil is
//! the correct "no plate is currently shown" signal for addon guards. Addon
//! iteration over `GetNamePlates()` receives an empty array.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_nameplate(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_NamePlate")?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetNamePlateForUnit",
        c_nameplate_get_nameplate_for_unit,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetNamePlates",
        c_nameplate_get_nameplates,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "SetNamePlateSize",
        c_nameplate_set_nameplate_size,
    )?;
    Ok(())
}

fn c_nameplate_get_nameplate_for_unit(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn c_nameplate_get_nameplates(state: &mut LuaState) -> LuaResult<u32> {
    let array = create_table(state);
    state.push(array);
    Ok(1)
}

fn c_nameplate_set_nameplate_size(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
