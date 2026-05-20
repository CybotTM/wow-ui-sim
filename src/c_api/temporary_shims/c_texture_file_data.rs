//! C_Texture temporary file-data shim — fileDataID lookup is not modeled.
//!
//! Atlas lookups are real and remain in `c_texture`; this module only adds the
//! missing fileDataID-to-filename method. Returning no values preserves the
//! current nil result until the simulator has a real listfile-backed lookup.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_texture_file_data(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Texture")?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetFilenameFromFileDataID",
        get_filename_from_file_data_id,
    )?;
    Ok(())
}

fn get_filename_from_file_data_id(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
