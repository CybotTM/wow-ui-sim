//! Attribute, frame-flag, and script-flag RustFn methods.

use super::helpers::{attribute_to_val, store_simple_attribute, val_to_f32};
use crate::lua_api::frame::methods::methods_helpers::can_change_protected_state_for;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, frame_id_from_stack,
    frame_ref, val_to_string,
};
use crate::lua_api::script_helpers::{
    call_error_handler_state, get_script as get_rilua_script, protected_lua_pcall_state,
};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

pub(super) fn get_attribute(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let first = val_to_string(state, stack_val(state, 2));
    let second = val_to_string(state, stack_val(state, 3));
    let third = val_to_string(state, stack_val(state, 4));
    let keys = build_attribute_keys(state, first, second, third);
    let Some(keys) = keys else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let attr = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|frame| {
            keys.iter()
                .find_map(|key| frame.attributes.get(key.as_str()).cloned())
        })
    };
    let val = attribute_to_val(state, attr.as_ref());
    state.push(val);
    Ok(1)
}

fn build_attribute_keys(
    _state: &LuaState,
    first: Option<String>,
    second: Option<String>,
    third: Option<String>,
) -> Option<Vec<String>> {
    match (first, second, third) {
        (Some(name), None, None) => Some(vec![name]),
        (Some(prefix), Some(name), suffix) => Some(attribute_lookup_keys(
            &prefix,
            &name,
            suffix.as_deref().unwrap_or_default(),
        )),
        _ => None,
    }
}

fn attribute_lookup_keys(prefix: &str, name: &str, suffix: &str) -> Vec<String> {
    let mut keys = Vec::with_capacity(5);
    keys.push(format!("{prefix}{name}{suffix}"));
    keys.push(format!("*{name}{suffix}"));
    keys.push(format!("{prefix}{name}*"));
    keys.push(format!("*{name}*"));
    keys.push(name.to_string());
    keys
}

pub(super) fn set_attribute(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let name_val = stack_val(state, 2);
    let value = stack_val(state, 3);
    let Some(name) = val_to_string(state, name_val) else {
        return Ok(0);
    };
    if protected_write_blocked(state, id) {
        return Ok(0);
    }
    let name_arg = create_string(state, &name);
    let force_dispatch = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).is_some_and(|frame| frame.forbidden)
    };
    let changed = store_simple_attribute(state, id, &name, value)?;
    let compatibility_dispatch = should_dispatch_unchanged_attribute(&name, value);
    if (changed || force_dispatch || compatibility_dispatch)
        && let Some(handler) = get_rilua_script(state, id, "OnAttributeChanged")
    {
        let frame = frame_ref(state, id)?;
        dispatch_attribute_changed(state, handler, frame, name_arg, value);
    }
    Ok(0)
}

fn should_dispatch_unchanged_attribute(name: &str, value: Val) -> bool {
    name == "createframes" && matches!(value, Val::Bool(true))
}

/// True when the caller cannot mutate protected state for the current frame.
/// Attribute writes use the same lockdown gate as other protected frame
/// mutations.
pub(super) fn protected_write_blocked(state: &mut LuaState, id: u64) -> bool {
    !can_change_protected_state_for(state, id)
}

pub(super) fn dispatch_attribute_changed(
    state: &mut LuaState,
    handler: Val,
    frame: Val,
    name: Val,
    value: Val,
) {
    let Ok(dispatcher) = state.load(
        r#"
        local handler, frame, name, value = ...
        handler(frame, name, value)
        "#,
    ) else {
        return;
    };
    let call_base = state.top;
    state.ensure_stack(call_base + 5);
    state.stack_set(call_base, Val::Function(dispatcher.gc_ref()));
    state.stack_set(call_base + 1, handler);
    state.stack_set(call_base + 2, frame);
    state.stack_set(call_base + 3, name);
    state.stack_set(call_base + 4, value);
    state.top = call_base + 5;
    if let Err(error) = state.call_function(call_base, 0) {
        call_error_handler_state(state, &error.to_string());
    }
    state.top = call_base;
}

pub(super) fn set_attribute_no_handler(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let name_val = stack_val(state, 2);
    let value = stack_val(state, 3);
    let Some(name) = val_to_string(state, name_val) else {
        return Ok(0);
    };
    if protected_write_blocked(state, id) {
        return Ok(0);
    }
    let _ = store_simple_attribute(state, id, &name, value)?;
    Ok(0)
}

pub(super) fn clear_attributes(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.attributes.clear();
    }
    Ok(0)
}

pub(super) fn execute_attribute(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let name_val = stack_val(state, 2);
    let Some(name) = val_to_string(state, name_val) else {
        return push_execute_attribute_failure(state, "attribute-missing");
    };
    let attr = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|frame| frame.attributes.get(name.as_str()).cloned())
    };
    let attr = attribute_to_val(state, attr.as_ref());
    let nargs = (state.top as i32 - state.base as i32) as usize;
    let extra_args = (3..=nargs)
        .map(|index| stack_val(state, index as i32))
        .collect::<Vec<_>>();
    match attr {
        Val::Function(_) => {
            let results = protected_lua_pcall_state(state, attr, &extra_args)
                .map_err(|error| error.to_string());
            push_execute_attribute_result(state, results)
        }
        Val::Str(body_ref) => {
            let is_protected = borrow_state(state)?
                .widgets
                .get(id)
                .is_some_and(|frame| frame.is_protected);
            if !is_protected {
                return push_execute_attribute_failure(state, "unsupported-unprotected-snippet");
            }
            let Some(body) = val_to_string(state, Val::Str(body_ref)) else {
                return push_execute_attribute_failure(state, "attribute-missing");
            };
            let Ok(snippet) = compile_execute_attribute_snippet(state, &body) else {
                return push_execute_attribute_failure(state, "snippet-compile-failed");
            };
            let mut args = Vec::with_capacity(extra_args.len() + 1);
            args.push(frame_ref(state, id)?);
            args.extend(extra_args);
            let results = protected_lua_pcall_state(state, Val::Function(snippet.gc_ref()), &args)
                .map_err(|error| error.to_string());
            push_execute_attribute_result(state, results)
        }
        _ => push_execute_attribute_failure(state, "attribute-missing"),
    }
}

fn compile_execute_attribute_snippet(
    state: &mut LuaState,
    body: &str,
) -> LuaResult<rilua::Function> {
    let loader = state.load(&format!("return function(self, ...) {body} end"))?;
    let closure = call_function_state(state, Val::Function(loader.gc_ref()), &[])?;
    let Val::Function(func_ref) = closure else {
        return Err(rilua::runtime_error(
            "ExecuteAttribute: snippet loader did not return a function",
        ));
    };
    Ok(rilua::Function::from_gc_ref(func_ref))
}

fn push_execute_attribute_result(
    state: &mut LuaState,
    results: Result<Vec<Val>, String>,
) -> LuaResult<u32> {
    match results {
        Ok(values) => {
            state.push(Val::Bool(true));
            let return_count = values.len() as u32 + 1;
            for value in values {
                state.push(value);
            }
            Ok(return_count)
        }
        Err(error) => push_execute_attribute_failure(state, &error),
    }
}

fn push_execute_attribute_failure(state: &mut LuaState, reason: &str) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    let reason = create_string(state, reason);
    state.push(reason);
    Ok(2)
}

pub(super) fn set_frame_ref(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let label_val = stack_val(state, 2);
    let frame_val = stack_val(state, 3);
    let Some(label) = val_to_string(state, label_val) else {
        return Ok(0);
    };
    let _ = store_simple_attribute(state, id, &format!("_frame-{label}"), frame_val)?;
    Ok(0)
}

pub(super) fn get_frame_ref(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let label_val = stack_val(state, 2);
    let Some(label) = val_to_string(state, label_val) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let attr = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|frame| {
            frame
                .attributes
                .get(format!("_frame-{label}").as_str())
                .cloned()
        })
    };
    let attr = attribute_to_val(state, attr.as_ref());
    state.push(attr);
    Ok(1)
}

pub(super) fn set_forbidden(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: combat lockdown check
    let forbidden = match stack_val(state, 2) {
        Val::Bool(b) => b,
        _ => true,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.forbidden = forbidden;
    }
    Ok(0)
}

pub(super) fn is_forbidden(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim.widgets.get(id).map(|f| f.forbidden).unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(val));
    Ok(1)
}

pub(super) fn can_change_protected_state(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let allowed = can_change_protected_state_for(state, id);
    state.push(Val::Bool(allowed));
    Ok(1)
}

pub(super) fn set_pass_through_buttons(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let nargs = (state.top as i32 - state.base as i32) as usize;
    let buttons = (2..=nargs)
        .filter_map(|index| val_to_string(state, stack_val(state, index as i32)))
        .map(|button| button.to_ascii_lowercase())
        .collect();
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.pass_through_buttons = buttons;
    }
    Ok(0)
}

pub(super) fn set_flattens_render_layers(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let flatten = matches!(stack_val(state, 2), Val::Bool(b) if b);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.flattens_render_layers = flatten;
    }
    Ok(0)
}

pub(super) fn set_motion_scripts_while_disabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = matches!(stack_val(state, 2), Val::Bool(b) if b);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.motion_scripts_while_disabled = enabled;
    }
    Ok(0)
}

pub(super) fn get_motion_scripts_while_disabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.motion_scripts_while_disabled)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(val));
    Ok(1)
}

pub(super) fn set_clips_children(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let clips = matches!(stack_val(state, 2), Val::Bool(b) if b);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.clips_children = clips;
    }
    Ok(0)
}

pub(super) fn does_clip_children(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.clips_children)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(val));
    Ok(1)
}

pub(super) fn set_hit_rect_insets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: combat lockdown check
    let l = val_to_f32(stack_val(state, 2), 0.0);
    let r = val_to_f32(stack_val(state, 3), 0.0);
    let t = val_to_f32(stack_val(state, 4), 0.0);
    let b = val_to_f32(stack_val(state, 5), 0.0);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.hit_rect_insets = (l, r, t, b);
    }
    Ok(0)
}

pub(super) fn get_hit_rect_insets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let (l, r, t, b) = sim
        .widgets
        .get(id)
        .map(|f| f.hit_rect_insets)
        .unwrap_or((0.0, 0.0, 0.0, 0.0));
    drop(sim);
    state.push(Val::Num(l as f64));
    state.push(Val::Num(r as f64));
    state.push(Val::Num(t as f64));
    state.push(Val::Num(b as f64));
    Ok(4)
}
