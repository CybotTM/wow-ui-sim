//! State and attribute driver stubs.
//!
//! `RegisterStateDriver`, `UnregisterStateDriver`, `RegisterAttributeDriver`,
//! `UnregisterAttributeDriver` — record the registered macro-condition string
//! and apply the resolved final value to the frame's attributes (or
//! visibility for `state-visibility`).

use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult};

use crate::lua_api::methods::{borrow_state_mut, frame_id_from_stack, val_to_string};
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
    let Some(resolved) = resolve_driver_value(values) else {
        return Ok(());
    };

    if attribute == "state-visibility" {
        apply_visibility_driver(state, id, resolved);
        return Ok(());
    }

    let attr = coerce_driver_attribute(resolved);
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
    Ok(())
}

fn resolve_driver_value(values: &str) -> Option<&str> {
    values
        .split(';')
        .next_back()
        .map(str::trim)
        .filter(|value| !value.is_empty())
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
