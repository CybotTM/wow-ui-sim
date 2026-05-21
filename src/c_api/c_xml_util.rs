//! C_XMLUtil: template info lookup.

use crate::lua_api::methods::{
    create_string, create_table, create_table_with_capacity, val_to_string,
};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

use super::helpers::set_global_val;

const TEMPLATE_INFO_HASH_FIELDS: usize = 7;
const TEMPLATE_KEY_VALUE_HASH_FIELDS: usize = 3;

pub fn register_c_xml_util(state: &mut LuaState) -> LuaResult<()> {
    let c_xml_util = create_table(state);
    let Val::Table(c_xml_util_ref) = c_xml_util else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn_static(
        state,
        c_xml_util_ref,
        "GetTemplateInfo",
        c_xml_util_get_template_info,
    )?;
    set_global_val(state, "C_XMLUtil", c_xml_util);
    Ok(())
}

pub fn c_xml_util_get_template_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(template_name) = val_to_string(state, stack_val(state, 1)) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let Some(info) = crate::xml::get_template_info(&template_name) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info_table = create_table_with_capacity(state, TEMPLATE_INFO_HASH_FIELDS);
    let Val::Table(info_ref) = info_table else {
        unreachable!("create_table must return a table");
    };
    let key_values_table = build_key_values_table(state, &info.key_values);
    fill_template_info_table(state, info_ref, &info, key_values_table);
    state.push(info_table);
    Ok(1)
}

fn build_key_values_table(
    state: &mut LuaState,
    key_values: &[crate::xml::TemplateKeyValueInfo],
) -> Val {
    let key_values_table = create_table(state);
    let Val::Table(key_values_ref) = key_values_table else {
        unreachable!("create_table must return a table");
    };
    for (index, kv) in key_values.iter().enumerate() {
        let entry = build_key_value_entry(state, kv);
        if let Some(table) = state.gc.tables.get_mut(key_values_ref) {
            let _ = table.raw_set(Val::Num((index + 1) as f64), entry, &state.gc.string_arena);
        }
        state.gc.barrier_back(key_values_ref);
    }
    key_values_table
}

fn build_key_value_entry(state: &mut LuaState, kv: &crate::xml::TemplateKeyValueInfo) -> Val {
    let entry = create_table_with_capacity(state, TEMPLATE_KEY_VALUE_HASH_FIELDS);
    let Val::Table(entry_ref) = entry else {
        unreachable!("create_table must return a table");
    };
    table_set_str(state, entry_ref, "key", &kv.key);
    table_set_str(state, entry_ref, "value", &kv.value);
    if let Some(value_type) = &kv.value_type {
        table_set_str(state, entry_ref, "type", value_type);
    }
    entry
}

fn fill_template_info_table(
    state: &mut LuaState,
    info_ref: GcRef<Table>,
    info: &crate::xml::TemplateInfo,
    key_values_table: Val,
) {
    table_set_str(state, info_ref, "type", &info.frame_type);
    table_set_str(state, info_ref, "frameType", &info.frame_type);
    table_set_str(state, info_ref, "frameTemplate", &info.template_name);
    table_set_str(state, info_ref, "template", &info.template_name);
    table_set_num(state, info_ref, "width", info.width as f64);
    table_set_num(state, info_ref, "height", info.height as f64);
    let key_ref = state.gc.intern_string(b"keyValues");
    if let Some(table) = state.gc.tables.get_mut(info_ref) {
        let _ = table.raw_set(Val::Str(key_ref), key_values_table, &state.gc.string_arena);
    }
    state.gc.barrier_back(info_ref);
}

fn table_set_str(state: &mut LuaState, tbl_ref: GcRef<Table>, key: &str, value: &str) {
    let key_ref = state.gc.intern_string(key.as_bytes());
    let value = create_string(state, value);
    if let Some(table) = state.gc.tables.get_mut(tbl_ref) {
        let _ = table.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
    }
    state.gc.barrier_back(tbl_ref);
}

fn table_set_num(state: &mut LuaState, tbl_ref: GcRef<Table>, key: &str, value: f64) {
    let key_ref = state.gc.intern_string(key.as_bytes());
    if let Some(table) = state.gc.tables.get_mut(tbl_ref) {
        let _ = table.raw_set(Val::Str(key_ref), Val::Num(value), &state.gc.string_arena);
    }
    state.gc.barrier_back(tbl_ref);
}
