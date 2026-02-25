//! LightUserData metatable for frame handles.
//!
//! Sets up a shared metatable for all LightUserData values (frames):
//! - `__index` = Rust fn that does rawget on methods_table, then falls back
//!   to children_keys / custom fields / methods (children before methods)
//! - `__newindex` = Rust fn (children_keys sync + __frame_fields storage)
//! - `__len` = Rust fn (children count)
//! - No `__eq` needed: same ID = same pointer = Lua `==` works natively.

use super::handle::{extract_frame_id, frame_lud, get_sim_state, lud_to_id};
use crate::widget::WidgetType;
use mlua::{LightUserData, Lua, Value};

/// Build and install the shared LightUserData metatable for frames.
pub fn setup_frame_metatable(lua: &Lua) -> mlua::Result<()> {
    let methods_table = lua.create_table()?;

    // Register all ~200 methods into the table
    super::methods::register_all_methods(lua, &methods_table)?;

    // Store methods_table in registry for getmetatable() and populate_method_index()
    lua.set_named_registry_value("__frame_methods_table", methods_table.clone())?;

    // Build the frame metatable
    let frame_mt = lua.create_table()?;
    frame_mt.set("__index", create_index(lua, methods_table)?)?;
    frame_mt.set("__newindex", create_newindex(lua)?)?;
    frame_mt.set("__len", create_len(lua)?)?;

    // Install as the shared metatable for ALL LightUserData values
    lua.set_type_metatable::<LightUserData>(Some(frame_mt));

    // Helper function for triggering __newindex from Rust (used by SetParentKey).
    // Doing `parent[key] = value` in Lua triggers the type metatable's __newindex.
    let assign_fn = lua
        .load("return function(parent, key, value) parent[key] = value end")
        .eval::<mlua::Function>()?;
    lua.set_named_registry_value("__frame_assign_fn", assign_fn)?;

    // Helper for forbidden proxy __index fallback: access a property on a LightUserData
    // frame via the type metatable (handles script handlers, children, custom fields).
    let index_helper = lua
        .load("return function(lud, key) return lud[key] end")
        .eval::<mlua::Function>()?;
    lua.set_named_registry_value("__frame_index_helper", index_helper)?;

    // Shared metatable for all forbidden proxy tables.
    // __index and __newindex read __lud from the proxy table at call time so the
    // same metatable can be reused across all instances (identity check passes).
    let forbidden_mt = create_forbidden_proxy_metatable(lua)?;
    lua.set_named_registry_value("__forbidden_proxy_mt", forbidden_mt)?;

    Ok(())
}

/// Build the shared metatable used by all forbidden proxy tables.
///
/// Reads `__lud` (LightUserData) from the proxy table at call time so this single
/// metatable can be reused for every forbidden proxy instance.  This makes
/// `getmetatable(proxy1) == getmetatable(proxy2)` true for any two forbidden proxies.
fn create_forbidden_proxy_metatable(lua: &Lua) -> mlua::Result<mlua::Table> {
    let mt = lua.create_table()?;
    mt.raw_set("__metatable", "Forbidden")?;
    mt.raw_set("__index", create_forbidden_index(lua)?)?;
    mt.raw_set("__newindex", create_forbidden_newindex(lua)?)?;
    Ok(mt)
}

/// Wrap a function so its first argument is replaced with `lud`.
fn wrap_fn_with_lud(lua: &Lua, f: mlua::Function, lud: Value) -> mlua::Result<mlua::Function> {
    lua.create_function(move |_, mut args: mlua::MultiValue| {
        if !args.is_empty() {
            args[0] = lud.clone();
        }
        f.call::<mlua::MultiValue>(args)
    })
}

/// __index for forbidden proxy: reads __lud from proxy, forwards to frame methods.
fn create_forbidden_index(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|lua, (this, key): (mlua::Table, Value)| {
        let lud: Value = this.raw_get("__lud")?;

        // Fast path: method in __frame_methods_table.
        let methods_table: mlua::Table = lua.named_registry_value("__frame_methods_table")?;
        let method: Value = methods_table.raw_get(key.clone())?;
        if let Value::Function(f) = method {
            return Ok(Value::Function(wrap_fn_with_lud(lua, f, lud)?));
        }

        // Fall through: handles script handlers, children, custom fields.
        let index_helper: mlua::Function = lua.named_registry_value("__frame_index_helper")?;
        let result: Value = index_helper.call((lud.clone(), key))?;

        if let Value::Function(f) = result {
            return Ok(Value::Function(wrap_fn_with_lud(lua, f, lud)?));
        }

        Ok(result)
    })
}

/// __newindex for forbidden proxy: delegates to the LightUserData's __newindex.
fn create_forbidden_newindex(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|lua, (this, key, value): (mlua::Table, String, Value)| {
        let lud: Value = this.raw_get("__lud")?;
        let assign: mlua::Function = lua.named_registry_value("__frame_assign_fn")?;
        assign.call::<()>((lud, key, value))?;
        Ok(())
    })
}

/// __index: method lookup via rawget on methods_table, then fallback.
fn create_index(lua: &Lua, methods_table: mlua::Table) -> mlua::Result<mlua::Function> {
    lua.create_function(move |lua, (ud, key): (LightUserData, Value)| {
        let frame_id = lud_to_id(ud);

        // Numeric index → returns n-th child frame (1-indexed)
        if let Value::Integer(idx) = key {
            return lookup_child_by_index(lua, frame_id, idx);
        }

        let key_str = match &key {
            Value::String(s) => s.to_string_lossy(),
            _ => return Ok(Value::Nil),
        };

        // Mixin overrides: functions written by Mixin() shadow everything.
        // In real WoW, Mixin(frame, {GetName=fn}) rawsets into the frame table,
        // which takes precedence over the metatable __index.
        if let Some(value) = lookup_mixin_override(lua, frame_id, &key_str) {
            return Ok(value);
        }

        // Children_keys lookup (own-table keys in real WoW — resolve before methods)
        if let Some(child) = lookup_child_by_key(lua, frame_id, &key_str)? {
            return Ok(child);
        }

        // Custom fields (__frame_fields: script handlers, properties, etc.)
        if let Some(value) = lookup_custom_field(lua, frame_id, &key_str) {
            return Ok(value);
        }

        // Rust methods table — filtered by widget type (metatable __index in real WoW).
        let method: Value = methods_table.raw_get(key_str.as_str())?;
        if method != Value::Nil {
            let widget_type = {
                let state_rc = get_sim_state(lua);
                let state = state_rc.borrow();
                state.widgets.get(frame_id)
                    .map(|f| f.widget_type)
                    .unwrap_or(WidgetType::Frame)
            };
            if super::method_registry::is_method_allowed(widget_type, key_str.as_str()) {
                return Ok(method);
            }
        }

        Ok(Value::Nil)
    })
}

/// __newindex: children_keys sync + __frame_fields storage.
///
/// If the frame has a per-frame custom metatable with `__newindex` (set via
/// `setmetatable(frame, mt)`), delegate to it instead of the default behavior.
fn create_newindex(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|lua, (ud, key, value): (LightUserData, String, Value)| {
        let frame_id = lud_to_id(ud);

        if dispatch_custom_newindex(lua, ud, frame_id, &key, value.clone())? {
            return Ok(());
        }

        sync_children_keys(lua, frame_id, &key, &value);

        let frame_fields =
            crate::lua_api::script_helpers::get_or_create_frame_fields(lua, frame_id);
        frame_fields.set(key, value)?;
        Ok(())
    })
}

/// If the frame has a per-frame custom `__newindex`, call it and return true.
fn dispatch_custom_newindex(
    lua: &Lua,
    ud: LightUserData,
    frame_id: u64,
    key: &str,
    value: Value,
) -> mlua::Result<bool> {
    if let Ok(store) = lua.named_registry_value::<mlua::Table>("__frame_custom_mt") {
        if let Ok(mt) = store.get::<mlua::Table>(frame_id) {
            if let Ok(newindex) = mt.get::<mlua::Function>("__newindex") {
                newindex.call::<()>((Value::LightUserData(ud), key.to_owned(), value))?;
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

/// __len: returns number of children.
fn create_len(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let len = state.widgets.get(id).map(|f| f.children.len()).unwrap_or(0);
        Ok(len)
    })
}

// ── Lookup helpers ──────────────────────────────────────────────────

/// Look up a child frame by numeric index (1-indexed).
fn lookup_child_by_index(lua: &Lua, frame_id: u64, idx: i64) -> mlua::Result<Value> {
    if idx > 0 {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(frame_id)
            && let Some(&child_id) = frame.children.get((idx - 1) as usize)
        {
            return Ok(frame_lud(child_id));
        }
    }
    Ok(Value::Nil)
}

/// Look up a child frame by name from children_keys.
fn lookup_child_by_key(lua: &Lua, frame_id: u64, key: &str) -> mlua::Result<Option<Value>> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    if let Some(frame) = state.widgets.get(frame_id)
        && let Some(&child_id) = frame.children_keys.get(key)
    {
        return Ok(Some(frame_lud(child_id)));
    }
    Ok(None)
}

/// Look up a value from the __frame_fields Lua table (stored in registry).
fn lookup_custom_field(lua: &Lua, frame_id: u64, key: &str) -> Option<Value> {
    let fields_table = crate::lua_api::script_helpers::get_frame_fields_table(lua)?;
    let frame_fields: mlua::Table = fields_table.get::<mlua::Table>(frame_id).ok()?;
    let value: Value = frame_fields.get::<Value>(key).unwrap_or(Value::Nil);
    if value != Value::Nil {
        Some(value)
    } else {
        None
    }
}

/// Look up a Mixin-applied override from the dedicated __mixin_overrides registry table.
///
/// When Mixin(frame, mixin) is called on a LightUserData frame, function values are written
/// to `__mixin_overrides[frame_id][key]` so they can shadow Rust built-in methods. Only
/// function values go here — non-function properties still go through __frame_fields.
fn lookup_mixin_override(lua: &Lua, frame_id: u64, key: &str) -> Option<Value> {
    let overrides: mlua::Table = lua.named_registry_value("__mixin_overrides").ok()?;
    let frame_overrides: mlua::Table = overrides.get::<mlua::Table>(frame_id).ok()?;
    let value: Value = frame_overrides.get::<Value>(key).unwrap_or(Value::Nil);
    if value != Value::Nil { Some(value) } else { None }
}


