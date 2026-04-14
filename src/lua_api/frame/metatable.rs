//! Table-backed frame proxy metatables.
//!
//! Public frame values are Lua tables with a hidden `__lud` pointing at the internal
//! `FrameRef` userdata. The proxy metatable implements frame lookup and assignment
//! behavior while `FrameRef` itself only carries the registered Rust methods.

use super::handle::{extract_frame_id, frame_fields, frame_ref, frame_userdata, get_sim_state};
use mlua::{Lua, Value};

/// Install shared metatables for table-backed frame proxies.
/// Called once during Lua env setup.
pub fn setup_frame_helpers(lua: &Lua) -> mlua::Result<()> {
    register_bind_method_helper(lua)?;
    register_custom_metatable_store(lua)?;
    install_frame_proxy_metatable(lua)?;
    install_forbidden_proxy_metatable(lua)?;
    Ok(())
}

/// Factory that binds a method's first argument to a frame value.
/// Used by frame proxies to rebind method `self` from proxy to hidden userdata.
fn register_bind_method_helper(lua: &Lua) -> mlua::Result<()> {
    lua.set_named_registry_value(
        "__frame_bind_method_helper",
        crate::lua_api::cfunc_wrap::create_bind_factory(lua)?,
    )
}

/// Empty table for storing per-frame custom metatables (used by setmetatable(frame, mt)).
fn register_custom_metatable_store(lua: &Lua) -> mlua::Result<()> {
    let store = lua.create_table()?;
    lua.set_named_registry_value("__frame_custom_mt", store)
}

/// Shared metatable reused by all forbidden proxy tables.
fn install_frame_proxy_metatable(lua: &Lua) -> mlua::Result<()> {
    let mt = create_frame_proxy_metatable(lua)?;
    lua.set_named_registry_value("__frame_proxy_mt", mt)
}

fn install_forbidden_proxy_metatable(lua: &Lua) -> mlua::Result<()> {
    let mt = create_forbidden_proxy_metatable(lua)?;
    lua.set_named_registry_value("__forbidden_proxy_mt", mt)
}

/// Create the normal table-backed Lua representation for a frame.
pub fn create_frame_proxy(lua: &Lua, frame_val: Value) -> mlua::Result<Value> {
    let proxy = lua.create_table()?;
    proxy.raw_set("__lud", frame_val)?;
    let mt: mlua::Table = lua.named_registry_value("__frame_proxy_mt").or_else(|_| {
        let mt = create_frame_proxy_metatable(lua)?;
        lua.set_named_registry_value("__frame_proxy_mt", mt.clone())?;
        Ok(mt)
    })?;
    proxy.set_metatable(Some(mt));
    Ok(Value::Table(proxy))
}

/// Build the shared metatable used by all forbidden proxy tables.
///
/// Reads `__lud` (the FrameRef UserData) from the proxy table at call time so this single
/// metatable can be reused for every forbidden proxy instance.
fn create_forbidden_proxy_metatable(lua: &Lua) -> mlua::Result<mlua::Table> {
    let mt = create_shared_proxy_metatable(lua)?;
    mt.raw_set("__metatable", "Forbidden")?;
    Ok(mt)
}

fn create_frame_proxy_metatable(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_shared_proxy_metatable(lua)
}

fn create_shared_proxy_metatable(lua: &Lua) -> mlua::Result<mlua::Table> {
    let mt = lua.create_table()?;
    mt.raw_set("__index", create_proxy_index(lua)?)?;
    mt.raw_set("__newindex", create_proxy_newindex(lua)?)?;
    mt.raw_set("__len", create_proxy_len(lua)?)?;
    mt.raw_set("__tostring", create_proxy_tostring(lua)?)?;
    mt.raw_set("__eq", create_proxy_eq(lua)?)?;
    Ok(mt)
}

/// Wrap a function so its first argument is replaced with `frame_val`.
fn wrap_fn_with_frame(
    lua: &Lua,
    f: mlua::Function,
    frame_val: Value,
) -> mlua::Result<mlua::Function> {
    let bind_fn: mlua::Function = lua.named_registry_value("__frame_bind_method_helper")?;
    bind_fn.call((f, frame_val))
}

/// __index for frame proxies: resolves per-frame fields first, then registered methods.
fn create_proxy_index(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|lua, (this, key): (mlua::Table, Value)| {
        let frame_val = Value::Table(this.clone());
        let Some(userdata) = frame_userdata(&frame_val) else {
            return Ok(Value::Nil);
        };
        let frame_id = extract_frame_id(&frame_val).unwrap_or(0);

        match key {
            Value::Integer(idx) => lookup_proxy_integer_key(lua, frame_id, idx),
            Value::String(name) => {
                let key_str = name.to_string_lossy().to_string();
                lookup_proxy_string_key(lua, &frame_val, &userdata, frame_id, key_str.as_str())
            }
            _ => Ok(Value::Nil),
        }
    })
}

/// __newindex for frame proxies: syncs children keys, dispatches custom mt, stores fields.
fn create_proxy_newindex(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|lua, (this, key, value): (mlua::Table, Value, Value)| {
        let frame_val = Value::Table(this);
        let Some(fields) = frame_fields(&frame_val)? else {
            return Ok(());
        };
        let frame_id = extract_frame_id(&frame_val).unwrap_or(0);

        match key {
            Value::Integer(idx) => fields.raw_set(idx, value)?,
            Value::String(name) => {
                let key_str = name.to_string_lossy().to_string();
                sync_children_keys(lua, frame_id, &key_str, &value);
                if dispatch_custom_newindex(lua, frame_id, &key_str, &value)? {
                    return Ok(());
                }
                fields.raw_set(key_str.as_str(), value)?;
            }
            _ => {}
        }

        Ok(())
    })
}

fn create_proxy_len(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|lua, this: mlua::Table| Ok(frame_children_len(lua, &Value::Table(this))))
}

fn create_proxy_tostring(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|lua, this: mlua::Table| Ok(frame_display_name(lua, &Value::Table(this))))
}

fn create_proxy_eq(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|_lua, (a, b): (Value, Value)| {
        Ok(extract_frame_id(&a)
            .zip(extract_frame_id(&b))
            .is_some_and(|(a, b)| a == b))
    })
}

// ── Lookup helpers ────────────────────────────────────────────────────────────

fn lookup_proxy_integer_key(lua: &Lua, frame_id: u64, idx: i64) -> mlua::Result<Value> {
    if idx == 0 {
        return Ok(Value::LightUserData(mlua::LightUserData(
            frame_id as *mut std::ffi::c_void,
        )));
    }
    lookup_child_by_index(lua, frame_id, idx)
}

fn lookup_proxy_string_key(
    lua: &Lua,
    frame_val: &Value,
    userdata: &mlua::AnyUserData,
    frame_id: u64,
    key: &str,
) -> mlua::Result<Value> {
    if let Some(fields) = frame_fields(frame_val)? {
        let field_value: Value = fields.raw_get(key)?;
        if !field_value.is_nil() {
            return Ok(field_value);
        }
    }

    if is_disallowed_method(lua, frame_id, key)? {
        return Ok(Value::Nil);
    }

    let value = lookup_registered_method(userdata, key)?;
    if let Value::Function(function) = value {
        return Ok(Value::Function(wrap_fn_with_frame(
            lua,
            function,
            Value::UserData(userdata.clone()),
        )?));
    }

    Ok(value)
}

/// Look up a child frame by numeric index (1-indexed).
fn lookup_child_by_index(lua: &Lua, frame_id: u64, idx: i64) -> mlua::Result<Value> {
    if idx > 0 {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(frame_id)
            && let Some(&child_id) = frame.children.get((idx - 1) as usize)
        {
            drop(state);
            return frame_ref(lua, child_id);
        }
    }
    Ok(Value::Nil)
}

fn lookup_registered_method(userdata: &mlua::AnyUserData, key: &str) -> mlua::Result<Value> {
    let index_value: Value = userdata.metatable()?.get("__index")?;
    match index_value {
        Value::Function(function) => function.call((userdata.clone(), key.to_owned())),
        Value::Table(table) => table.raw_get(key),
        _ => Ok(Value::Nil),
    }
}

fn is_disallowed_method(lua: &Lua, frame_id: u64, key: &str) -> mlua::Result<bool> {
    let (widget_type, is_anim_type) = {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        state
            .widgets
            .get(frame_id)
            .map(|frame| {
                (
                    frame.widget_type,
                    frame
                        .object_type_name
                        .as_deref()
                        .is_some_and(super::methods::methods_core::is_anim_type),
                )
            })
            .unwrap_or((crate::widget::WidgetType::Frame, false))
    };

    if is_anim_type {
        return Ok(false);
    }

    Ok(super::method_registry::is_registered_method(key)
        && !super::method_registry::is_method_allowed(widget_type, key))
}

fn frame_children_len(lua: &Lua, frame_val: &Value) -> i32 {
    let Some(frame_id) = extract_frame_id(frame_val) else {
        return 0;
    };
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .widgets
        .get(frame_id)
        .map(|f| f.children.len())
        .unwrap_or(0) as i32
}

fn frame_display_name(lua: &Lua, frame_val: &Value) -> String {
    let Some(frame_id) = extract_frame_id(frame_val) else {
        return "Frame: 0x00000000".to_string();
    };
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let type_name = state
        .widgets
        .get(frame_id)
        .map(|f| {
            f.object_type_name
                .as_deref()
                .unwrap_or(f.widget_type.as_str())
        })
        .unwrap_or("Frame");
    format!("{}: 0x{:08X}", type_name, frame_id)
}

/// If the frame has a per-frame custom `__newindex`, call it and return true.
fn dispatch_custom_newindex(
    lua: &Lua,
    frame_id: u64,
    key: &str,
    value: &Value,
) -> mlua::Result<bool> {
    if let Ok(store) = lua.named_registry_value::<mlua::Table>("__frame_custom_mt") {
        if let Ok(mt) = store.get::<mlua::Table>(frame_id) {
            if let Ok(newindex) = mt.get::<mlua::Function>("__newindex") {
                let frame_val = frame_ref(lua, frame_id)?;
                newindex.call::<()>((frame_val, key.to_owned(), value.clone()))?;
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// Sync children_keys and parent_key when a value is assigned to a frame property.
fn sync_children_keys(lua: &Lua, frame_id: u64, key: &str, value: &Value) {
    let state_rc = get_sim_state(lua);
    if let Some(child_id) = extract_frame_id(value) {
        sync_child_frame(state_rc, frame_id, key, child_id);
    } else {
        remove_stale_child_key(state_rc, frame_id, key);
    }
}

/// Register a child frame under a key on the parent, updating children_keys and parent_key.
fn sync_child_frame(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    key: &str,
    child_id: u64,
) {
    let mut state = state_rc.borrow_mut();
    let is_real_child = state
        .widgets
        .get(child_id)
        .is_some_and(|c| c.parent_id == Some(frame_id));
    if let Some(parent_frame) = state.widgets.get_mut(frame_id) {
        parent_frame.children_keys.insert(key.to_owned(), child_id);
        if is_real_child && !parent_frame.children.contains(&child_id) {
            parent_frame.children.push(child_id);
        }
    }
    if let Some(child) = state.widgets.get_mut_visual(child_id) {
        if child.parent_key.is_none() {
            child.parent_key = Some(key.to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{frame_children_len, frame_display_name};
    use crate::lua_api::SimState;
    use crate::lua_api::frame::frame_ref;
    use crate::widget::{Frame, WidgetType};
    use mlua::{Lua, LuaOptions, StdLib};
    use std::cell::RefCell;
    use std::rc::Rc;

    fn make_lua() -> Lua {
        unsafe { Lua::unsafe_new_with(StdLib::ALL, LuaOptions::default()) }
    }

    #[test]
    fn frame_proxy_helpers_match_expected_frame_shape() {
        let lua = make_lua();
        let state = Rc::new(RefCell::new(SimState::default()));
        let (parent_id, child_id) = {
            let mut state = state.borrow_mut();
            let parent_id = state.widgets.register(Frame::new(
                WidgetType::Frame,
                Some("ParentFrame".to_string()),
                None,
            ));
            let child_id = state.widgets.register(Frame::new(
                WidgetType::Frame,
                Some("ChildFrame".to_string()),
                Some(parent_id),
            ));
            state
                .widgets
                .get_mut(parent_id)
                .unwrap()
                .children
                .push(child_id);
            (parent_id, child_id)
        };
        lua.set_app_data(Rc::clone(&state));

        let frame = frame_ref(&lua, parent_id).expect("frame ref should be created");
        assert_eq!(frame_children_len(&lua, &frame), 1);
        assert_eq!(
            frame_display_name(&lua, &frame),
            format!("Frame: 0x{:08X}", parent_id)
        );
        assert_eq!(
            super::lookup_child_by_index(&lua, parent_id, 1)
                .ok()
                .and_then(|value| crate::lua_api::frame::extract_frame_id(&value)),
            Some(child_id)
        );
    }
}

/// Remove a stale children_keys entry and clear parent_key when a non-frame is assigned.
fn remove_stale_child_key(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    key: &str,
) {
    let mut state = state_rc.borrow_mut();
    if let Some(parent_frame) = state.widgets.get_mut(frame_id) {
        if let Some(old_child_id) = parent_frame.children_keys.remove(key) {
            if let Some(child) = state.widgets.get_mut_visual(old_child_id) {
                if child.parent_key.as_deref() == Some(key) {
                    child.parent_key = None;
                }
            }
        }
    }
}
