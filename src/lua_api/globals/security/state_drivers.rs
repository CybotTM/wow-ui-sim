//! State and attribute driver stubs.
//!
//! `RegisterStateDriver`, `UnregisterStateDriver`, `RegisterAttributeDriver`,
//! `UnregisterAttributeDriver` — record the registered macro-condition string
//! and apply the resolved final value to the frame's attributes (or
//! visibility for `state-visibility`).

use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, frame_id_from_stack, frame_ref, val_to_string,
};
use crate::lua_api::script_helpers::{call_error_handler_state, protected_lua_pcall_state};
use crate::lua_bridge::stack_val;
use crate::widget::AttributeValue;

pub(super) fn register_state_driver_stubs(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "RegisterStateDriver", register_state_driver)?;
    LuaApiMut::register_function(lua, "UnregisterStateDriver", unregister_state_driver)?;
    LuaApiMut::register_function(lua, "RegisterAttributeDriver", register_attribute_driver)?;
    LuaApiMut::register_function(
        lua,
        "UnregisterAttributeDriver",
        unregister_attribute_driver,
    )?;
    Ok(())
}

fn register_state_driver(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(name) = val_to_string(state, stack_val(state, 2)) else {
        return Ok(0);
    };
    let Some(values) = val_to_string(state, stack_val(state, 3)) else {
        return Ok(0);
    };
    register_driver(state, id, &format!("state-{name}"), values)?;
    Ok(0)
}

fn unregister_state_driver(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(name) = val_to_string(state, stack_val(state, 2)) else {
        return Ok(0);
    };
    unregister_driver(state, id, &format!("state-{name}"))
}

fn register_attribute_driver(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(attribute) = val_to_string(state, stack_val(state, 2)) else {
        return Ok(0);
    };
    let Some(values) = val_to_string(state, stack_val(state, 3)) else {
        return Ok(0);
    };
    register_driver(state, id, &attribute, values)
}

fn unregister_attribute_driver(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(attribute) = val_to_string(state, stack_val(state, 2)) else {
        return Ok(0);
    };
    unregister_driver(state, id, &attribute)
}

fn register_driver(
    state: &mut LuaState,
    id: u64,
    attribute: &str,
    values: String,
) -> LuaResult<u32> {
    if attribute.starts_with('_') {
        return Ok(0);
    }
    {
        let mut sim = borrow_state_mut(state)?;
        sim.secure_attribute_drivers
            .entry(id)
            .or_default()
            .insert(attribute.to_string(), values.clone());
    }
    apply_driver(state, id, attribute, &values)?;
    Ok(0)
}

fn unregister_driver(state: &mut LuaState, id: u64, attribute: &str) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    let Some(drivers) = sim.secure_attribute_drivers.get_mut(&id) else {
        return Ok(0);
    };
    drivers.remove(attribute);
    if drivers.is_empty() {
        sim.secure_attribute_drivers.remove(&id);
    }
    Ok(0)
}

fn apply_driver(state: &mut LuaState, id: u64, attribute: &str, values: &str) -> LuaResult<()> {
    let resolved = {
        let sim = borrow_state(state)?;
        super::cmd_option::resolve_cmd_option(values, &sim).map(str::to_string)
    };
    let Some(resolved) = resolved else {
        return Ok(());
    };

    if attribute == "state-visibility" {
        apply_visibility_driver(state, id, &resolved);
        return Ok(());
    }

    let attr = coerce_driver_attribute(&resolved);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        match attr {
            AttributeValue::Nil => {
                frame.attributes.remove(attribute);
            }
            value => {
                frame.attributes.insert(attribute.to_string(), value);
            }
        }
    }
    drop(sim);

    run_state_driver_snippet(state, id, attribute, &resolved)?;
    Ok(())
}

fn run_state_driver_snippet(
    state: &mut LuaState,
    id: u64,
    attribute: &str,
    resolved: &str,
) -> LuaResult<()> {
    let Some(state_name) = attribute.strip_prefix("state-") else {
        return Ok(());
    };
    let snippet_attribute = format!("_onstate-{state_name}");
    let snippet_body = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|frame| frame.attributes.get(&snippet_attribute))
            .and_then(attribute_string)
            .map(str::to_string)
    };
    let Some(snippet_body) = snippet_body else {
        return Ok(());
    };

    let snippet = compile_state_driver_snippet(state, &snippet_body)?;
    let frame = frame_ref(state, id)?;
    let newstate = Val::Str(state.gc.intern_string(resolved.as_bytes()));
    if let Err(error) =
        protected_lua_pcall_state(state, Val::Function(snippet.gc_ref()), &[frame, newstate])
    {
        call_error_handler_state(state, &error);
    }
    Ok(())
}

fn attribute_string(attribute: &AttributeValue) -> Option<&str> {
    match attribute {
        AttributeValue::String(value) => Some(value.as_str()),
        _ => None,
    }
}

fn compile_state_driver_snippet(state: &mut LuaState, body: &str) -> LuaResult<rilua::Function> {
    let loader = state.load(&format!(
        "return function(self, newstate) local strsub = string.sub; {body} end"
    ))?;
    let call_base = state.top;
    state.ensure_stack(call_base + 1);
    state.stack_set(call_base, Val::Function(loader.gc_ref()));
    state.top = call_base + 1;
    state.call_function(call_base, 1)?;
    let closure = state.stack_get(call_base);
    state.top = call_base;
    let Val::Function(func_ref) = closure else {
        return Err(runtime_error(
            "state driver snippet loader did not return a function",
        ));
    };
    Ok(rilua::Function::from_gc_ref(func_ref))
}

fn apply_visibility_driver(state: &mut LuaState, id: u64, resolved: &str) {
    let mut sim = match borrow_state_mut(state) {
        Ok(sim) => sim,
        Err(_) => return,
    };
    let Some(frame) = sim.widgets.get_mut(id) else {
        return;
    };
    match resolved {
        "show" => {
            frame.attributes.remove("statehidden");
            sim.set_frame_visible(id, true);
        }
        "hide" => {
            frame
                .attributes
                .insert("statehidden".into(), AttributeValue::Boolean(true));
            sim.set_frame_visible(id, false);
        }
        _ => {}
    }
}

fn coerce_driver_attribute(resolved: &str) -> AttributeValue {
    if resolved == "nil" {
        AttributeValue::Nil
    } else if let Ok(number) = resolved.parse::<f64>() {
        AttributeValue::Number(number)
    } else {
        AttributeValue::String(resolved.to_string())
    }
}
