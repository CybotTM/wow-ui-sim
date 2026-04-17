//! Initialization helpers for the WoW Lua environment.
//!
//! Standalone functions extracted from `env.rs` that are called during
//! `WowLuaEnv::new()` and from event/script dispatch paths.

mod bootstrap;
mod enums;
mod frames;
mod registry;
mod runtime;

use std::cell::RefCell;
use std::rc::Rc;

use super::state::SimState;

// Re-export public-within-crate symbols that env.rs and globals/ import.
pub(crate) use frames::init_builtin_frames;
pub(crate) use runtime::{
    addon_taint_name, is_blizzard_addon, record_addon_time, update_threshold_counters,
};

// Re-export the three functions called from globals/environment_restore.rs.
pub(crate) use bootstrap::init_runtime_surface_bootstrap;
pub(crate) use bootstrap::init_shared_bootstrap;
pub(crate) use enums::init_enum_globals;

/// Initialize the primary rilua state: seed registries, globals, frame methods, and taint.
pub(super) fn init_lua_state(
    lua: &mut rilua::Lua,
    state: Rc<RefCell<SimState>>,
) -> crate::Result<()> {
    registry::init_registry_tables(lua, &state)?;
    bootstrap::init_shared_bootstrap(lua)?;
    enums::init_enum_globals(lua)?;
    frames::init_frame_metatable(lua)?;
    // register_globals calls gc_stop at its entry; finalize_bootstrap_gc
    // below restores the collector once bootstrap is complete. Between
    // those two points the mark phase is paused.
    super::globals::register_globals(lua, state.clone())?;
    bootstrap::init_runtime_surface_bootstrap(lua)?;
    super::globals::security::create_secure_environment(lua)?;
    enable_taint_and_wrap_loadstring(lua)?;
    crate::loader::precompiled::init(lua)?;
    remove_sandbox_globals(lua)?;
    frames::init_frame_metatable(lua)?;
    finalize_bootstrap_gc(lua)?;
    Ok(())
}

/// Run a full collection to drop bootstrap transients, then re-enable
/// the incremental collector. Called at the end of `init_lua_state`
/// (and should be called again by the binary after addon loading, with
/// a matching `gc_stop` before the addon loads).
fn finalize_bootstrap_gc(lua: &mut rilua::Lua) -> crate::Result<()> {
    use rilua::LuaApiMut;
    lua.gc_collect()?;
    lua.gc_restart();
    Ok(())
}

/// Enable Elune taint tracking and wrap loadstring as secure.
fn enable_taint_and_wrap_loadstring(lua: &mut rilua::Lua) -> crate::Result<()> {
    super::taint::enable_taint_mode(lua);
    Ok(())
}

/// Remove globals that WoW's sandbox doesn't expose and internal helpers
/// now stored in the Lua registry.
fn remove_sandbox_globals(_lua: &mut rilua::Lua) -> crate::Result<()> {
    Ok(())
}
