//! UserData meta methods for FrameRef.
//!
//! Registers __index, __newindex, __len, __tostring, __eq on FrameRef's UserData impl.
//! Methods are registered directly on FrameRef via add_method (mlua resolves them before
//! __index is called), so __index only handles children / per-frame fields.

use super::handle::{extract_frame_id, frame_ref, get_sim_state, FrameRef};
use mlua::{Lua, Value};

/// Register meta methods on the FrameRef UserData type.
/// Called from FrameRef's UserData::add_methods impl.
pub fn add_meta_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_index_metamethod(methods);
    add_newindex_metamethod(methods);
    add_len_metamethod(methods);
    add_tostring_metamethod(methods);
    add_eq_metamethod(methods);
}

/// Build and install shared Lua helper values (assign fn, index helper, forbidden proxy mt).
/// Called once during Lua env setup (replaces the old setup_frame_metatable).
pub fn setup_frame_helpers(lua: &Lua) -> mlua::Result<()> {
    // Helper function for triggering __newindex from Rust (used by SetParentKey).
    // Doing `parent[key] = value` in Lua triggers the UserData's __newindex.
    let assign_fn = lua
        .load("return function(parent, key, value) parent[key] = value end")
        .eval::<mlua::Function>()?;
    lua.set_named_registry_value("__frame_assign_fn", assign_fn)?;

    // Helper for forbidden proxy __index fallback: access a property on a FrameRef
    // frame via its __index metamethod (handles script handlers, children, custom fields).
    let index_helper = lua
        .load("return function(ud, key) return ud[key] end")
        .eval::<mlua::Function>()?;
    lua.set_named_registry_value("__frame_index_helper", index_helper)?;

    // Registry table for per-frame custom metatables (used by setmetatable(frame, mt)).
    let custom_mt_store = lua.create_table()?;
    lua.set_named_registry_value("__frame_custom_mt", custom_mt_store)?;

    // Shared metatable for all forbidden proxy tables.
    let forbidden_mt = create_forbidden_proxy_metatable(lua)?;
    lua.set_named_registry_value("__forbidden_proxy_mt", forbidden_mt)?;

    // Extract method table from FrameRef's UserData metatable for getmetatable() per-type filtering.
    // With UserData, methods live in the internal metatable — expose them as __frame_methods_table.
    populate_methods_table(lua)?;

    // Patch __index to check user_value (fenv) BEFORE mlua's method table.
    // mlua's generated __index checks methods first, so children with names matching
    // methods (e.g. parentKey="Lower" vs frame:Lower()) get shadowed. This wraps
    // the existing __index to check the per-frame fields table first.
    patch_metatable_index(lua)?;

    Ok(())
}

// ── Meta method registration ──────────────────────────────────────────────────

/// __index: single rawget on per-frame user_value table.
///
/// mlua resolves registered add_method entries before __index, so we only need
/// to handle children, mixin fns, and custom fields here — all stored in one table.
fn add_index_metamethod<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_meta_function(
        mlua::MetaMethod::Index,
        |lua, (ud, key): (mlua::AnyUserData, mlua::Value)| {
            let this = ud.borrow::<FrameRef>()?;
            let id = this.0;
            drop(this);

            // Numeric index: [0] returns raw LightUserData, [1+] returns children
            if let mlua::Value::Integer(idx) = key {
                if idx == 0 {
                    return Ok(mlua::Value::LightUserData(mlua::LightUserData(
                        id as *mut std::ffi::c_void,
                    )));
                }
                return lookup_child_by_index(lua, id, idx);
            }

            // Single rawget on per-frame table (children, mixin fns, custom fields)
            if let Ok(fields) = ud.user_value::<mlua::Table>() {
                let val: mlua::Value = fields.raw_get(key)?;
                if val != mlua::Value::Nil {
                    return Ok(val);
                }
            }

            Ok(mlua::Value::Nil)
        },
    );
}

/// __newindex: sync children_keys, dispatch custom __newindex, then store in user_value.
fn add_newindex_metamethod<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_meta_function(
        mlua::MetaMethod::NewIndex,
        |lua, (ud, key, value): (mlua::AnyUserData, mlua::Value, mlua::Value)| {
            let this = ud.borrow::<FrameRef>()?;
            let id = this.0;
            drop(this);

            let key_str = match &key {
                mlua::Value::String(s) => s.to_string_lossy().to_string(),
                mlua::Value::Integer(idx) => {
                    // Numeric __newindex: store in per-frame table
                    if let Ok(fields) = ud.user_value::<mlua::Table>() {
                        fields.raw_set(*idx, value)?;
                    }
                    return Ok(());
                }
                _ => return Ok(()),
            };

            // Sync to Rust children_keys if value is a frame reference
            sync_children_keys(lua, id, &key_str, &value);

            // Check for per-frame custom __newindex metamethods
            if dispatch_custom_newindex(lua, id, &key_str, &value)? {
                return Ok(());
            }

            // Store in per-frame table (user_value)
            if let Ok(fields) = ud.user_value::<mlua::Table>() {
                fields.raw_set(key_str.as_str(), value)?;
            }

            Ok(())
        },
    );
}

/// __len: returns number of children.
fn add_len_metamethod<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_meta_method(mlua::MetaMethod::Len, |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.children.len())
            .unwrap_or(0) as i32)
    });
}

/// __tostring: returns "WidgetType: 0xID" matching WoW's format.
///
/// WoW returns e.g. "Frame: 0x12345678" for `tostring(frame)`.  Returning the
/// frame *name* caused `CreateFont(tostring(self))` inside
/// `FontableFrameMixin:MakeFontObjectCustom` to overwrite `_G["ChatFrame1"]`
/// (a FrameRef) with a Font table, breaking `:Hide()` / `:Show()` etc.
fn add_tostring_metamethod<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_meta_method(mlua::MetaMethod::ToString, |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let type_name = state
            .widgets
            .get(this.0)
            .map(|f| f.widget_type.as_str())
            .unwrap_or("Frame");
        Ok(format!("{}: 0x{:08X}", type_name, this.0))
    });
}

/// __eq: compares two FrameRef values by ID.
fn add_eq_metamethod<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_meta_function(
        mlua::MetaMethod::Eq,
        |_lua, (a, b): (mlua::AnyUserData, mlua::AnyUserData)| {
            let a_id = a.borrow::<FrameRef>().map(|r| r.0).unwrap_or(u64::MAX);
            let b_id = b
                .borrow::<FrameRef>()
                .map(|r| r.0)
                .unwrap_or(u64::MAX - 1);
            Ok(a_id == b_id)
        },
    );
}

// ── Forbidden proxy metatable ─────────────────────────────────────────────────

/// Build the shared metatable used by all forbidden proxy tables.
///
/// Reads `__lud` (the FrameRef UserData) from the proxy table at call time so this single
/// metatable can be reused for every forbidden proxy instance.
fn create_forbidden_proxy_metatable(lua: &Lua) -> mlua::Result<mlua::Table> {
    let mt = lua.create_table()?;
    mt.raw_set("__metatable", "Forbidden")?;
    mt.raw_set("__index", create_forbidden_index(lua)?)?;
    mt.raw_set("__newindex", create_forbidden_newindex(lua)?)?;
    Ok(mt)
}

/// Wrap a function so its first argument is replaced with `frame_val`.
fn wrap_fn_with_frame(
    lua: &Lua,
    f: mlua::Function,
    frame_val: Value,
) -> mlua::Result<mlua::Function> {
    lua.create_function(move |_, mut args: mlua::MultiValue| {
        if !args.is_empty() {
            args[0] = frame_val.clone();
        }
        f.call::<mlua::MultiValue>(args)
    })
}

/// __index for forbidden proxy: reads __lud from proxy, forwards to frame methods.
fn create_forbidden_index(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|lua, (this, key): (mlua::Table, Value)| {
        let frame_val: Value = this.raw_get("__lud")?;

        // Fall through: handles script handlers, children, custom fields via UserData __index.
        let index_helper: mlua::Function = lua.named_registry_value("__frame_index_helper")?;
        let result: Value = index_helper.call((frame_val.clone(), key))?;

        if let Value::Function(f) = result {
            return Ok(Value::Function(wrap_fn_with_frame(lua, f, frame_val)?));
        }

        Ok(result)
    })
}

/// __newindex for forbidden proxy: delegates to the FrameRef's __newindex.
fn create_forbidden_newindex(lua: &Lua) -> mlua::Result<mlua::Function> {
    lua.create_function(|lua, (this, key, value): (mlua::Table, String, Value)| {
        let frame_val: Value = this.raw_get("__lud")?;
        let assign: mlua::Function = lua.named_registry_value("__frame_assign_fn")?;
        assign.call::<()>((frame_val, key, value))?;
        Ok(())
    })
}

// ── Methods table for getmetatable() ─────────────────────────────────────────

/// Extract all non-meta methods from FrameRef's UserData metatable into __frame_methods_table.
/// This is used by the getmetatable() override to build per-type metatables for Wowless.
fn populate_methods_table(lua: &Lua) -> mlua::Result<()> {
    let dummy = lua.create_userdata(FrameRef(0))?;
    let methods_table = lua.create_table()?;
    lua.load(
        r#"
        local ud, out = ...
        local mt = debug.getmetatable(ud)
        if mt then
            for k, v in pairs(mt) do
                if type(k) == "string" and type(v) == "function" and not k:match("^__") then
                    out[k] = v
                end
            end
        end
    "#,
    )
    .call::<()>((dummy, methods_table.clone()))?;
    lua.set_named_registry_value("__frame_methods_table", methods_table)?;
    Ok(())
}

/// Lua code to patch a UserData metatable's __index: check per-instance fields
/// (via debug.getfenv) before falling through to mlua's method resolution.
/// Args: (userdata_instance).
const PATCH_INDEX_LUA: &str = r#"
    local ud = ...
    local mt = debug.getmetatable(ud)
    local old_index = mt.__index
    local dgetfenv = debug.getfenv

    local function frame_index(self, key)
        if type(key) == "string" then
            local env = dgetfenv(self)
            if env then
                local val = rawget(env[1], key)
                if val ~= nil then return val end
            end
        end
        return old_index(self, key)
    end

    rawset(mt, "__index", frame_index)
"#;

/// Patch the FrameRef metatable __index to check per-frame fields before methods.
/// Called once during init — all FrameRef instances share the same metatable.
fn patch_metatable_index(lua: &Lua) -> mlua::Result<()> {
    let dummy = lua.create_userdata(FrameRef(0))?;
    lua.load(PATCH_INDEX_LUA).call::<()>(dummy)?;
    Ok(())
}

// ── Lookup helpers ────────────────────────────────────────────────────────────

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
    use mlua::{Lua, MetaMethod, StdLib, UserData, UserDataMethods, Value};

    struct TestObj;

    impl UserData for TestObj {
        fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
            methods.add_method("Lower", |_, _, ()| Ok("hello"));
            methods.add_meta_function(MetaMethod::Index, |_, (_ud, _key): (mlua::AnyUserData, Value)| {
                Ok(Value::Nil)
            });
            methods.add_meta_function(
                MetaMethod::NewIndex,
                |_, (ud, key, value): (mlua::AnyUserData, String, Value)| {
                    let fields = ud.user_value::<mlua::Table>()?;
                    fields.raw_set(key, value)?;
                    Ok(())
                },
            );
        }
    }

    #[test]
    fn test_method_shadows_property_by_default() {
        let lua = unsafe { Lua::unsafe_new_with(StdLib::ALL, mlua::LuaOptions::default()) };
        let obj = lua.create_userdata(TestObj).unwrap();
        let fields = lua.create_table().unwrap();
        obj.set_user_value(fields).unwrap();
        lua.globals().set("obj", obj).unwrap();

        // Without patching, method wins over property
        let result: String = lua.load(r#"
            obj.Lower = "world"
            return type(obj.Lower)
        "#).eval().unwrap();
        assert_eq!(result, "function", "method shadows property by default");
    }

    fn make_lua_and_patch() -> Lua {
        let lua = unsafe { Lua::unsafe_new_with(StdLib::ALL, mlua::LuaOptions::default()) };
        let obj = lua.create_userdata(TestObj).unwrap();
        let fields = lua.create_table().unwrap();
        obj.set_user_value(fields).unwrap();
        lua.load(super::PATCH_INDEX_LUA).call::<()>(obj.clone()).unwrap();
        lua.globals().set("obj", obj).unwrap();
        lua
    }

    #[test]
    fn test_patched_index_property_wins() {
        let lua = make_lua_and_patch();
        let result: String = lua.load(r#"
            obj.Lower = "world"
            return obj.Lower
        "#).eval().unwrap();
        assert_eq!(result, "world", "property should shadow method");
    }

    #[test]
    fn test_patched_index_method_still_works() {
        let lua = make_lua_and_patch();
        let result: String = lua.load("return obj:Lower()").eval().unwrap();
        assert_eq!(result, "hello", "method works when no property set");
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
