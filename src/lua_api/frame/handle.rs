//! Frame-backed Lua table helpers.
//!
//! Frames are exposed to Lua as proxy tables with a hidden `__lud` field pointing
//! at the internal `FrameRef` userdata. This keeps the public Lua value table-like
//! while preserving the existing Rust method registration on `FrameRef`.
//! SimState is stored in Lua app_data, accessed via `get_sim_state()`.

use crate::lua_api::SimState;
use mlua::{Lua, Value};
use std::cell::RefCell;
use std::rc::Rc;

const FRAME_REF_CACHE_KEY: &str = "__frame_refs";

/// Frame reference userdata. Wraps a widget ID for the Lua side.
/// All frame methods are registered via `add_method` on this type,
/// so mlua resolves them directly without going through `__index`.
#[derive(Clone, Copy)]
pub struct FrameRef(pub u64);

impl mlua::UserData for FrameRef {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        // Regular frame methods — registered via add_method so mlua
        // resolves them directly (bypasses __index entirely).
        super::methods::register_all_methods(methods);

        // Meta methods — __index, __newindex, __len, __tostring, __eq
        super::metatable::add_meta_methods(methods);
    }
}

/// Get the cached UserData Value for a frame ID.
///
/// Looks up the numeric `__frame_refs[id]` cache first.
/// If not cached, creates a new hidden `FrameRef` userdata, wraps it in a
/// proxy table, and caches the proxy in `reg.__frame_refs`.
pub fn frame_ref(lua: &mlua::Lua, id: u64) -> mlua::Result<mlua::Value> {
    let frame_refs = get_or_create_frame_ref_cache(lua)?;
    let cached: mlua::Value = frame_refs.raw_get(id as i64)?;
    if !cached.is_nil() {
        return Ok(cached);
    }

    // Create and cache on demand
    let ud = lua.create_userdata(FrameRef(id))?;
    let fields = lua.create_table()?;
    ud.set_user_value(fields)?;
    let val = super::metatable::create_frame_proxy(lua, mlua::Value::UserData(ud))?;
    frame_refs.raw_set(id as i64, val.clone())?;
    Ok(val)
}

fn get_or_create_frame_ref_cache(lua: &Lua) -> mlua::Result<mlua::Table> {
    lua.named_registry_value(FRAME_REF_CACHE_KEY).or_else(|_| {
        let cache = lua.create_table()?;
        lua.set_named_registry_value(FRAME_REF_CACHE_KEY, cache.clone())?;
        Ok(cache)
    })
}

/// Retrieve the shared SimState from Lua app_data.
#[inline]
pub fn get_sim_state(lua: &Lua) -> Rc<RefCell<SimState>> {
    lua.app_data_ref::<Rc<RefCell<SimState>>>()
        .expect("SimState not set in Lua app_data")
        .clone()
}

/// Sync a child frame ref into the parent's user_value table.
///
/// Call this after every `children_keys.insert` from Rust, AFTER dropping the
/// state borrow. Writes `parent_user_value[key] = child_ref` so that Lua-side
/// `parent.key` resolves via the single rawget in `__index`.
pub fn sync_child_to_lua(lua: &Lua, parent_id: u64, key: &str, child_id: u64) -> mlua::Result<()> {
    let parent_val = frame_ref(lua, parent_id)?;
    let child_val = frame_ref(lua, child_id)?;
    if let Some(fields) = frame_fields(&parent_val)? {
        fields.raw_set(key, child_val)?;
    }
    Ok(())
}

/// Extract the hidden FrameRef userdata from either a raw userdata or frame table.
pub fn frame_userdata(value: &Value) -> Option<mlua::AnyUserData> {
    match value {
        Value::UserData(ud) => Some(ud.clone()),
        Value::Table(t) => match t.raw_get::<Value>("__lud") {
            Ok(Value::UserData(ud)) => Some(ud),
            _ => None,
        },
        _ => None,
    }
}

/// Get the hidden per-frame fields table from a frame value.
pub fn frame_fields(value: &Value) -> mlua::Result<Option<mlua::Table>> {
    let Some(userdata) = frame_userdata(value) else {
        return Ok(None);
    };
    userdata.user_value::<mlua::Table>().map(Some)
}

/// Extract a frame ID from a Lua Value (FrameRef UserData).
#[inline]
pub fn extract_frame_id(value: &Value) -> Option<u64> {
    frame_userdata(value).and_then(|ud| ud.borrow::<FrameRef>().ok().map(|r| r.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lua_api::frame::metatable::setup_frame_helpers;
    use crate::widget::{Frame, WidgetType};
    use mlua::{Lua, LuaOptions, StdLib};

    fn make_lua() -> Lua {
        unsafe { Lua::unsafe_new_with(StdLib::ALL, LuaOptions::default()) }
    }

    #[test]
    fn frame_ref_returns_table_proxy() {
        let lua = make_lua();

        let frame = frame_ref(&lua, 7).expect("frame ref should be created");

        assert!(
            matches!(frame, Value::Table(_)),
            "frame should be exposed as a table"
        );
        assert_eq!(extract_frame_id(&frame), Some(7));
    }

    #[test]
    fn frame_table_proxy_preserves_method_calls() {
        let lua = make_lua();
        let state = Rc::new(RefCell::new(SimState::default()));
        let frame_id = {
            let mut state = state.borrow_mut();
            state.widgets.register(Frame::new(
                WidgetType::Frame,
                Some("ProxyFrame".to_string()),
                None,
            ))
        };

        lua.set_app_data(Rc::clone(&state));
        setup_frame_helpers(&lua).expect("frame helpers should install");

        let frame = frame_ref(&lua, frame_id).expect("frame ref should be created");
        lua.globals()
            .set("frame", frame)
            .expect("global frame should be set");

        let (frame_type, name): (String, String) = lua
            .load("return type(frame), frame:GetName()")
            .eval()
            .expect("proxy frame should behave like a table with methods");

        assert_eq!(frame_type, "table");
        assert_eq!(name, "ProxyFrame");
    }
}
