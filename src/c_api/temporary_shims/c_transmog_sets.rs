//! Temporary `C_TransmogSets` empty set surface.
//!
//! The wardrobe set inventory is not modeled yet. These defaults keep set-tab
//! Blizzard code on an empty-data path while keeping the compatibility gap out
//! of the generic runtime bootstrap.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::{create_string_static, create_table, create_table_with_fields};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_transmog_sets_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_TransmogSets")?;
    table_set_rust_fn_static(state, ns, "GetBaseSetID", get_base_set_id)?;
    table_set_rust_fn_static(state, ns, "GetVariantSets", empty_table_result)?;
    table_set_rust_fn_static(state, ns, "GetSetInfo", get_set_info)?;
    table_set_rust_fn_static(state, ns, "GetSetPrimaryAppearances", empty_table_result)?;
    table_set_rust_fn_static(state, ns, "GetBaseSets", empty_table_result)?;
    table_set_rust_fn_static(state, ns, "GetAllSets", empty_table_result)?;
    table_set_rust_fn_static(state, ns, "GetUsableSets", empty_table_result)?;
    table_set_rust_fn_static(state, ns, "HasAvailableSets", false_result)?;
    table_set_rust_fn_static(state, ns, "IsBaseSetCollected", false_result)?;
    table_set_rust_fn_static(state, ns, "GetSourcesForSlot", empty_table_result)?;
    table_set_rust_fn_static(state, ns, "GetAllSetAppearancesByID", empty_table_result)
}

fn get_base_set_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn empty_table_result(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn get_set_info(state: &mut LuaState) -> LuaResult<u32> {
    let name = create_string_static(state, "");
    let info = create_table_with_fields(
        state,
        &[
            ("setID", Val::Num(0.0)),
            ("name", name),
            ("collected", Val::Bool(false)),
        ],
    );
    state.push(info);
    Ok(1)
}

fn false_result(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn set_inventory_defaults_to_empty_tables() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let result: String = env
            .eval(
                r#"
                if C_TransmogSets.GetBaseSetID(1) ~= 0 then return "base_id" end
                if #C_TransmogSets.GetBaseSets() ~= 0 then return "base_sets" end
                if #C_TransmogSets.GetAllSetAppearancesByID(1) ~= 0 then return "appearances" end
                if C_TransmogSets.HasAvailableSets() ~= false then return "available" end
                local info = C_TransmogSets.GetSetInfo(1)
                if info.setID ~= 0 or info.name ~= "" or info.collected ~= false then
                    return "set_info"
                end
                return "ok"
                "#,
            )
            .expect("transmog sets defaults should be callable");

        assert_eq!(result, "ok");
    }
}
