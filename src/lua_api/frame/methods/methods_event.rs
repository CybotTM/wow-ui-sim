//! Event registration methods: RegisterEvent, UnregisterEvent, etc.

use super::super::handle::FrameRef;
use crate::event::{is_registerable_event, is_restricted_event};
use crate::lua_api::frame::handle::get_sim_state;
use mlua::Value;

/// Add event registration methods to the frame methods table.
pub fn add_event_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_event_register_methods(methods);
    add_event_query_methods(methods);
    add_keyboard_propagation_methods(methods);
}

fn add_event_register_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_register_event_methods(methods);
    add_unregister_event_methods(methods);
}

/// Build the error returned when an unrecognised event name is passed to RegisterEvent.
fn unknown_event_error(frame_name: &str, event: &str) -> mlua::Error {
    crate::lua_api::script_helpers::lua_error_val(format!(
        "{}:RegisterEvent(): {}:RegisterEvent(): Attempt to register unknown event \"{}\"",
        frame_name, frame_name, event
    ))
}

/// Insert a frame into the per-event Lua table (__event_individual[event][id] = true).
fn lua_register_individual(lua: &mlua::Lua, id: u64, event: &str) -> mlua::Result<()> {
    let individual: mlua::Table = lua.named_registry_value("__event_individual")?;
    let event_tbl = match individual.get::<mlua::Table>(event) {
        Ok(t) => t,
        Err(_) => {
            let t = lua.create_table()?;
            individual.set(event, t.clone())?;
            t
        }
    };
    event_tbl.set(id, true)?;
    Ok(())
}

/// Remove a frame from the per-event Lua table (__event_individual[event][id] = nil).
fn lua_unregister_individual(lua: &mlua::Lua, id: u64, event: &str) -> mlua::Result<()> {
    let individual: mlua::Table = lua.named_registry_value("__event_individual")?;
    if let Ok(event_tbl) = individual.get::<mlua::Table>(event) {
        event_tbl.set(id, Value::Nil)?;
    }
    Ok(())
}

/// Insert a frame into the all-events Lua table (__event_all[id] = true).
fn lua_register_all(lua: &mlua::Lua, id: u64) -> mlua::Result<()> {
    let all_events: mlua::Table = lua.named_registry_value("__event_all")?;
    all_events.set(id, true)?;
    Ok(())
}

/// Remove a frame from all individual event tables and the all-events table.
fn lua_unregister_all(lua: &mlua::Lua, id: u64) -> mlua::Result<()> {
    let individual: mlua::Table = lua.named_registry_value("__event_individual")?;
    for pair in individual.pairs::<String, mlua::Table>() {
        if let Ok((_, event_tbl)) = pair {
            event_tbl.set(id, Value::Nil)?;
        }
    }
    let all_events: mlua::Table = lua.named_registry_value("__event_all")?;
    all_events.set(id, Value::Nil)?;
    Ok(())
}

fn add_register_event_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_register_event(methods);
    add_register_unit_event(methods);
}

fn add_register_event<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("RegisterEvent", |lua, this, event: String| {
        let id = this.0;
        let newly_registered = {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if !is_registerable_event(&event) {
                let frame_name = state.widgets.get(id)
                    .and_then(|f| f.name.clone())
                    .unwrap_or_else(|| "Frame".to_string());
                return Err(unknown_event_error(&frame_name, &event));
            }
            state.widgets.get_mut(id)
                .map(|f| f.registered_events.insert(event.clone()))
                .unwrap_or(false)
        };
        if newly_registered {
            lua_register_individual(lua, id, &event)?;
        }
        Ok(newly_registered && !is_restricted_event(&event))
    });
}

fn add_register_unit_event<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("RegisterUnitEvent", |lua, this, (event, _args): (String, mlua::Variadic<Value>)| {
        let id = this.0;
        let newly_registered = {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            state.widgets.get_mut(id)
                .map(|f| f.registered_events.insert(event.clone()))
                .unwrap_or(false)
        };
        if newly_registered {
            lua_register_individual(lua, id, &event)?;
        }
        Ok(newly_registered)
    });
}

fn add_unregister_event_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_unregister_event(methods);
    add_unregister_all_events(methods);
    add_register_all_events(methods);
}

fn add_unregister_event<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("UnregisterEvent", |lua, this, event: String| {
        let id = this.0;
        let was_registered = {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if !is_registerable_event(&event) {
                let frame_name = state.widgets.get(id)
                    .and_then(|f| f.name.clone())
                    .unwrap_or_else(|| "Frame".to_string());
                return Err(unknown_event_error(&frame_name, &event));
            }
            state.widgets.get_mut(id)
                .map(|f| f.registered_events.remove(&event))
                .unwrap_or(false)
        };
        if was_registered {
            lua_unregister_individual(lua, id, &event)?;
        }
        Ok(was_registered)
    });
}

fn add_unregister_all_events<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("UnregisterAllEvents", |lua, this, ()| {
        let id = this.0;
        {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut(id) {
                frame.registered_events.clear();
                frame.register_all_events = false;
            }
        }
        lua_unregister_all(lua, id)?;
        Ok(())
    });
}

fn add_register_all_events<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("RegisterAllEvents", |lua, this, ()| {
        let id = this.0;
        {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut(id) {
                frame.register_all_events = true;
            }
        }
        lua_register_all(lua, id)?;
        Ok(())
    });
}

fn add_event_query_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_is_event_registered(methods);
    add_register_event_callback(methods);
}

fn add_is_event_registered<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsEventRegistered", |lua, this, event: String| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let registered = state.widgets.get(this.0)
            .map(|f| f.registered_events.contains(&event))
            .unwrap_or(false);
        Ok((registered, Value::Nil))
    });
}

/// RegisterEventCallback only accepts the ~9 callback events (COMBAT_LOG_*, etc).
/// All other events — even valid RegisterEvent events — get "unknown event" error.
fn add_register_event_callback<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    use crate::event::is_callback_event;
    methods.add_method("RegisterEventCallback", |lua, this, (event, _cb): (String, Value)| {
        if !is_callback_event(&event) {
            return Err(crate::lua_api::script_helpers::lua_error_val(format!(
                "Frame:RegisterEventCallback(): Attempt to register unknown event \"{}\"",
                event
            )));
        }
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(f) = state.widgets.get_mut(id) {
            f.registered_events.insert(event.clone());
        }
        Ok(Value::Boolean(!is_restricted_event(&event)))
    });
}

fn add_keyboard_propagation_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetPropagateKeyboardInput", |lua, this, propagate: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(f) = state.widgets.get_mut(this.0) {
            f.propagate_keyboard_input = propagate;
        }
        Ok(())
    });

    methods.add_method("GetPropagateKeyboardInput", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(|f| f.propagate_keyboard_input).unwrap_or(false))
    });
}
