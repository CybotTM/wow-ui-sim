//! FrameRef UserData-based frame representation helpers.
//!
//! Frames are represented as FrameRef UserData with the frame ID stored directly.
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
/// Looks up the numeric `__frame_refs[id]` cache first, then `_G["__frame_{id}"]`.
/// If not cached, creates a new FrameRef UserData with an empty per-frame Lua
/// table as user_value and caches it in both places.
pub fn frame_ref(lua: &mlua::Lua, id: u64) -> mlua::Result<mlua::Value> {
    let frame_refs = get_or_create_frame_ref_cache(lua)?;
    let cached: mlua::Value = frame_refs.raw_get(id as i64)?;
    if !cached.is_nil() {
        return Ok(cached);
    }

    let key = format!("__frame_{}", id);
    let global_cached: mlua::Value = lua.globals().raw_get(key.as_str())?;
    if !global_cached.is_nil() {
        frame_refs.raw_set(id as i64, global_cached.clone())?;
        return Ok(global_cached);
    }

    // Create and cache on demand
    let ud = lua.create_userdata(FrameRef(id))?;
    let fields = lua.create_table()?;
    ud.set_user_value(fields)?;
    let val = mlua::Value::UserData(ud);
    frame_refs.raw_set(id as i64, val.clone())?;
    lua.globals().raw_set(key.as_str(), val.clone())?;
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
    if let Value::UserData(ud) = &parent_val {
        if let Ok(fields) = ud.user_value::<mlua::Table>() {
            fields.raw_set(key, child_val)?;
        }
    }
    Ok(())
}

/// Extract a frame ID from a Lua Value (FrameRef UserData).
#[inline]
pub fn extract_frame_id(value: &Value) -> Option<u64> {
    match value {
        Value::UserData(ud) => ud.borrow::<FrameRef>().ok().map(|r| r.0),
        Value::Table(t) => t
            .raw_get::<Value>("__lud")
            .ok()
            .and_then(|inner| extract_frame_id(&inner)),
        _ => None,
    }
}
