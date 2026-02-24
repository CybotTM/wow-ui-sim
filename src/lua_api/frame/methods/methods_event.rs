//! Event registration methods: RegisterEvent, UnregisterEvent, etc.

use crate::event::{is_restricted_event, is_valid_event};
use crate::lua_api::frame::handle::{get_sim_state, lud_to_id};
use mlua::{LightUserData, Lua, Value};

/// Add event registration methods to the frame methods table.
pub fn add_event_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    add_event_register_methods(lua, methods)?;
    add_event_query_methods(lua, methods)?;
    add_keyboard_propagation_methods(lua, methods)?;
    Ok(())
}

/// RegisterEvent, RegisterUnitEvent, UnregisterEvent, UnregisterAllEvents, RegisterAllEvents
fn add_event_register_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    add_register_event_methods(lua, methods)?;
    add_unregister_event_methods(lua, methods)?;
    Ok(())
}

/// Build the error returned when an unrecognised event name is passed to RegisterEvent.
/// Uses ExternalError (not RuntimeError) to avoid "runtime error: " prefix in pcall.
fn unknown_event_error(frame_name: &str, event: &str) -> mlua::Error {
    crate::lua_api::script_helpers::lua_error_val(format!(
        "{}:RegisterEvent(): {}:RegisterEvent(): Attempt to register unknown event \"{}\"",
        frame_name, frame_name, event
    ))
}

/// Insert a frame into the per-event Lua table (__event_individual[event][lud] = true).
fn lua_register_individual(lua: &Lua, ud: LightUserData, event: &str) -> mlua::Result<()> {
    let individual: mlua::Table = lua.named_registry_value("__event_individual")?;
    let event_tbl = match individual.get::<mlua::Table>(event) {
        Ok(t) => t,
        Err(_) => {
            let t = lua.create_table()?;
            individual.set(event, t.clone())?;
            t
        }
    };
    event_tbl.set(Value::LightUserData(ud), true)?;
    Ok(())
}

/// Remove a frame from the per-event Lua table (__event_individual[event][lud] = nil).
fn lua_unregister_individual(lua: &Lua, ud: LightUserData, event: &str) -> mlua::Result<()> {
    let individual: mlua::Table = lua.named_registry_value("__event_individual")?;
    if let Ok(event_tbl) = individual.get::<mlua::Table>(event) {
        event_tbl.set(Value::LightUserData(ud), Value::Nil)?;
    }
    Ok(())
}

/// Insert a frame into the all-events Lua table (__event_all[lud] = true).
fn lua_register_all(lua: &Lua, ud: LightUserData) -> mlua::Result<()> {
    let all_events: mlua::Table = lua.named_registry_value("__event_all")?;
    all_events.set(Value::LightUserData(ud), true)?;
    Ok(())
}

/// Remove a frame from all individual event tables and the all-events table.
fn lua_unregister_all(lua: &Lua, ud: LightUserData) -> mlua::Result<()> {
    let individual: mlua::Table = lua.named_registry_value("__event_individual")?;
    for pair in individual.pairs::<String, mlua::Table>() {
        if let Ok((_, event_tbl)) = pair {
            event_tbl.set(Value::LightUserData(ud), Value::Nil)?;
        }
    }
    let all_events: mlua::Table = lua.named_registry_value("__event_all")?;
    all_events.set(Value::LightUserData(ud), Value::Nil)?;
    Ok(())
}

/// RegisterEvent, RegisterUnitEvent
fn add_register_event_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("RegisterEvent", lua.create_function(|lua, (ud, event): (LightUserData, String)| {
        let id = lud_to_id(ud);
        // Validate and update Rust-side state first.
        let newly_registered = {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if !is_valid_event(&event) {
                let frame_name = state.widgets.get(id)
                    .and_then(|f| f.name.clone())
                    .unwrap_or_else(|| "Frame".to_string());
                return Err(unknown_event_error(&frame_name, &event));
            }
            if let Some(frame) = state.widgets.get_mut(id) {
                frame.registered_events.insert(event.clone())
            } else {
                false
            }
        };
        // Update Lua-side table for dispatch ordering (borrow released above).
        if newly_registered {
            lua_register_individual(lua, ud, &event)?;
        }
        // WoW returns true only if newly registered AND the event is unrestricted.
        Ok(newly_registered && !is_restricted_event(&event))
    })?)?;

    // Some addons pass a callback function as the last argument (non-standard)
    methods.set("RegisterUnitEvent", lua.create_function(
        |lua, (ud, event, _args): (LightUserData, String, mlua::Variadic<Value>)| {
            let id = lud_to_id(ud);
            let newly_registered = {
                let state_rc = get_sim_state(lua);
                let mut state = state_rc.borrow_mut();
                if let Some(frame) = state.widgets.get_mut(id) {
                    frame.registered_events.insert(event.clone())
                } else {
                    false
                }
            };
            if newly_registered {
                lua_register_individual(lua, ud, &event)?;
            }
            Ok(newly_registered)
        },
    )?)?;

    Ok(())
}

/// UnregisterEvent, UnregisterAllEvents, RegisterAllEvents
fn add_unregister_event_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("UnregisterEvent", lua.create_function(|lua, (ud, event): (LightUserData, String)| {
        let id = lud_to_id(ud);
        let was_registered = {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if !is_valid_event(&event) {
                let frame_name = state.widgets.get(id)
                    .and_then(|f| f.name.clone())
                    .unwrap_or_else(|| "Frame".to_string());
                return Err(unknown_event_error(&frame_name, &event));
            }
            if let Some(frame) = state.widgets.get_mut(id) {
                frame.registered_events.remove(&event)
            } else {
                false
            }
        };
        if was_registered {
            lua_unregister_individual(lua, ud, &event)?;
        }
        Ok(was_registered)
    })?)?;

    methods.set("UnregisterAllEvents", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut(id) {
                frame.registered_events.clear();
                frame.register_all_events = false;
            }
        }
        lua_unregister_all(lua, ud)?;
        Ok(())
    })?)?;

    methods.set("RegisterAllEvents", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut(id) {
                frame.register_all_events = true;
            }
        }
        lua_register_all(lua, ud)?;
        Ok(())
    })?)?;

    Ok(())
}

/// IsEventRegistered, RegisterEventCallback
fn add_event_query_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("IsEventRegistered", lua.create_function(|lua, (ud, event): (LightUserData, String)| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let registered = if let Some(frame) = state.widgets.get(id) {
            frame.registered_events.contains(&event)
        } else {
            false
        };
        Ok((registered, Value::Nil))
    })?)?;

    // RegisterEventCallback(event, callbackContainer) - callback-based event registration
    // Only callback events are accepted; non-callback events produce an error.
    // Implemented in Lua to avoid mlua::Error::RuntimeError overhead (12000x slower
    // than Lua error() due to Elune taint bookkeeping on Rust→Lua error boundary).
    {
        let callback_tbl = lua.create_table()?;
        let restricted_tbl = lua.create_table()?;
        for e in crate::event::callback_events() { callback_tbl.set(*e, true)?; }
        for e in crate::event::restricted_events() { restricted_tbl.set(*e, true)?; }
        let func: mlua::Function = lua.load(r#"
            local callback_events, restricted_events = ...
            return function(self, event, cb)
                if not callback_events[event] then
                    error("Frame:RegisterEventCallback(): Attempt to register unknown event \"" .. event .. "\"", 0)
                end
                return not restricted_events[event]
            end
        "#).call((callback_tbl, restricted_tbl))?;
        methods.set("RegisterEventCallback", func)?;
    }

    Ok(())
}

/// SetPropagateKeyboardInput, GetPropagateKeyboardInput
fn add_keyboard_propagation_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("SetPropagateKeyboardInput", lua.create_function(
        |lua, (ud, propagate): (LightUserData, bool)| {
            let id = lud_to_id(ud);
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(f) = state.widgets.get_mut(id) {
                f.propagate_keyboard_input = propagate;
            }
            Ok(())
        },
    )?)?;

    methods.set("GetPropagateKeyboardInput", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let propagate = state
            .widgets
            .get(id)
            .map(|f| f.propagate_keyboard_input)
            .unwrap_or(false);
        Ok(propagate)
    })?)?;

    Ok(())
}
