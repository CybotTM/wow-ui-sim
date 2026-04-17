//! Shared attribute-conversion helpers used by text, attributes, and events.

use crate::lua_api::rilua_methods::borrow_state_mut;
use crate::lua_api::rilua_script_helpers;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn val_to_f32(val: Val, default: f32) -> f32 {
    match val {
        Val::Num(n) => n as f32,
        _ => default,
    }
}

pub(super) const ATTR_REFS_KEY: &str = "__wow_attr_refs__";

pub(super) fn attr_ref_key(frame_id: u64, name: &str) -> String {
    format!("{frame_id}\0{name}")
}

pub(super) fn attribute_to_val(
    state: &mut LuaState,
    attr: Option<&crate::widget::AttributeValue>,
) -> Val {
    match attr {
        None => Val::Nil,
        Some(crate::widget::AttributeValue::Nil) => Val::Nil,
        Some(crate::widget::AttributeValue::Boolean(b)) => Val::Bool(*b),
        Some(crate::widget::AttributeValue::Number(n)) => Val::Num(*n),
        Some(crate::widget::AttributeValue::String(s)) => {
            crate::lua_api::rilua_methods::create_string(state, s)
        }
        Some(crate::widget::AttributeValue::LuaRef(key)) => {
            match rilua_script_helpers::registry_table(state, ATTR_REFS_KEY) {
                Some(table) => rilua_script_helpers::table_get_str(state, table, key),
                None => Val::Nil,
            }
        }
    }
}

/// Convert a Lua value into the storage representation. For scalar types,
/// this is lossless. For reference types (Table / Function / UserData),
/// the value is rooted in a Lua registry table and represented as a
/// `LuaRef` keyed by `{frame_id}\0{name}`. Callers that need registry
/// storage must pass a non-None `ref_key` — Nil / scalar paths ignore it.
pub(super) fn val_to_attribute(
    val: Val,
    state: &mut LuaState,
    ref_key: Option<&str>,
) -> crate::widget::AttributeValue {
    match val {
        Val::Nil => crate::widget::AttributeValue::Nil,
        Val::Bool(b) => crate::widget::AttributeValue::Boolean(b),
        Val::Num(n) => crate::widget::AttributeValue::Number(n),
        Val::Str(s) => {
            let text = state
                .gc
                .string_arena
                .get(s)
                .and_then(|ls| String::from_utf8(ls.data().to_vec()).ok())
                .unwrap_or_default();
            crate::widget::AttributeValue::String(text)
        }
        _ => {
            let Some(key) = ref_key else {
                return crate::widget::AttributeValue::Nil;
            };
            let refs = rilua_script_helpers::registry_table_or_create(state, ATTR_REFS_KEY);
            rilua_script_helpers::table_set_str(state, refs, key, val);
            crate::widget::AttributeValue::LuaRef(key.to_string())
        }
    }
}

pub(super) fn store_simple_attribute(
    state: &mut LuaState,
    id: u64,
    name: &str,
    value: Val,
) -> LuaResult<()> {
    let ref_key = attr_ref_key(id, name);
    let attr = val_to_attribute(value, state, Some(&ref_key));
    let replace_with_nil = matches!(attr, crate::widget::AttributeValue::Nil);
    let mut sim = borrow_state_mut(state)?;
    let old_was_ref = sim
        .widgets
        .get(id)
        .and_then(|f| f.attributes.get(name))
        .is_some_and(|v| matches!(v, crate::widget::AttributeValue::LuaRef(_)));
    if let Some(frame) = sim.widgets.get_mut(id) {
        if replace_with_nil {
            frame.attributes.remove(name);
        } else {
            frame.attributes.insert(name.to_string(), attr);
        }
    }
    drop(sim);
    if replace_with_nil && old_was_ref {
        drop_attr_ref(state, &ref_key);
    }
    Ok(())
}

fn drop_attr_ref(state: &mut LuaState, ref_key: &str) {
    if let Some(refs) = rilua_script_helpers::registry_table(state, ATTR_REFS_KEY) {
        rilua_script_helpers::table_set_str(state, refs, ref_key, Val::Nil);
    }
}
