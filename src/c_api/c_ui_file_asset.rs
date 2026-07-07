//! `C_UIFileAsset` path/fileDataID helpers.
//!
//! Patch 12.0.7 exposes a small lookup surface for UI file assets. The
//! simulator backs known-file checks with the bundled limited listfile used by
//! CASC/UI asset loading, so addon probes get the same path normalization and
//! fileDataID answers as the renderer cache bootstrap.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::val_to_string;
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_ui_file_asset(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_UIFileAsset")?;
    table_set_rust_fn_static(state, table_ref, "GetFileID", c_ui_file_asset_get_file_id)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "IsKnownFile",
        c_ui_file_asset_is_known_file,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "IsLooseFile",
        c_ui_file_asset_is_loose_file,
    )
}

fn c_ui_file_asset_get_file_id(state: &mut LuaState) -> LuaResult<u32> {
    match file_id_from_asset_arg(state) {
        Some(file_id) => state.push(Val::Num(file_id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_ui_file_asset_is_known_file(state: &mut LuaState) -> LuaResult<u32> {
    let known = file_id_from_asset_arg(state).is_some();
    state.push(Val::Bool(known));
    Ok(1)
}

fn c_ui_file_asset_is_loose_file(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn file_id_from_asset_arg(state: &LuaState) -> Option<u32> {
    if let Some(file_id) = numeric_file_id_arg(state) {
        return Some(file_id);
    }
    string_file_id_arg(state)
}

fn numeric_file_id_arg(state: &LuaState) -> Option<u32> {
    let file_id = u32::from_stack(state, 1).ok()?;
    (file_id > 0).then_some(file_id)
}

fn string_file_id_arg(state: &LuaState) -> Option<u32> {
    let asset_path = val_to_string(state, stack_val(state, 1))?;
    crate::limited_listfile::lookup_path(&asset_path)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn ui_file_asset_uses_limited_listfile_paths() {
        let env = WowLuaEnv::new().expect("env");
        let result: String = env
            .eval(
                r#"
                if C_UIFileAsset.GetFileID(123) ~= 123 then return "numeric" end
                if C_UIFileAsset.GetFileID("Interface\\Icons\\Trade_Engineering.blp") ~= 136243 then return "path" end
                if C_UIFileAsset.IsKnownFile("Interface/Icons/Trade_Engineering.blp") ~= true then return "known" end
                if C_UIFileAsset.GetFileID("Interface/Unknown") ~= nil then return "unknown-id" end
                if C_UIFileAsset.IsKnownFile("Interface/Unknown") ~= false then return "unknown-known" end
                if C_UIFileAsset.IsLooseFile("Interface/Icons/Trade_Engineering.blp") ~= false then return "loose" end
                return "ok"
                "#,
            )
            .expect("probe should run");
        assert_eq!(result, "ok");
    }
}
