//! Initialization helpers for the WoW Lua environment.
//!
//! Standalone functions extracted from `env.rs` that are called during
//! `WowLuaEnv::new()` and from event/script dispatch paths.

mod bootstrap;
mod enums;
mod frames;
mod freeze_globals;
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
    // secureenv is shallow-copied from `_G` here. It keeps its copy of
    // the dangerous globals (dofile / loadfile / require / string.dump /
    // math.randomseed) so secure chunks — which Blizzard trusts —
    // retain them. Insecure addon code that reads through `_G` sees nil
    // after the cleanup below.
    super::globals::security::create_secure_environment(lua)?;
    enable_taint_and_wrap_loadstring(lua)?;
    crate::loader::precompiled::init(lua)?;
    remove_sandbox_globals(lua)?;
    frames::init_frame_metatable(lua)?;
    finalize_bootstrap_gc(lua)?;
    // Opt-in until the "overwrite stable global" audit is complete.
    // Blizzard's SharedXMLBase utility files (Mixin / TableUtil /
    // EnumUtil / FunctionUtil / Compat) overwrite existing `_G`
    // entries during addon load; freezing `_G` rejects those writes
    // and breaks the entire addon pipeline. The infrastructure stays
    // available for measurement / selective freezing of known-stable
    // subtrees (Track 3).
    if std::env::var("WOW_SIM_FREEZE_GLOBALS").as_deref() == Ok("1") {
        freeze_globals::freeze_globals_with_live_shadow(lua)?;
    }
    install_global_slots(lua);
    Ok(())
}

/// Walk the Track 1 whitelist against post-bootstrap `_G` and stash the
/// resulting slot vector on `WowLuaAppData`. Runs after
/// `freeze_globals_with_live_shadow` so the captured values include any
/// post-freeze shadow bootstrapping, but before addon load so the
/// captured snapshot is the canonical pre-addon state.
fn install_global_slots(lua: &mut rilua::Lua) {
    use super::env::WowLuaAppData;
    use super::global_slots;
    use rilua::LuaApiMut;
    let slots = global_slots::install(lua.state_mut());
    let app = lua
        .state_mut()
        .app_data_mut::<WowLuaAppData>()
        .expect("WowLuaEnv rilua app_data should always exist");
    app.global_slots = Some(slots);
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

/// Nil WoW-forbidden globals from `_G`: the filesystem / module
/// loaders (`dofile`, `loadfile`, `require`), the bytecode dumper
/// (`string.dump`), and the process-global RNG seeder
/// (`math.randomseed`).
///
/// Runs AFTER `create_secure_environment` so `__secureenv` keeps its
/// shallow copy — secure chunks (audited Blizzard code) retain these
/// tools. Insecure addon code that reads through `_G` sees nil.
///
/// Covered by `tests/sandbox_dangerous_globals.rs`.
fn remove_sandbox_globals(lua: &mut rilua::Lua) -> crate::Result<()> {
    use rilua::LuaApiMut;
    for name in ["dofile", "loadfile", "require"] {
        LuaApiMut::set_global_val(lua, name, rilua::Val::Nil)?;
    }
    // `string` and `math` are shared tables: secureenv's shallow copy
    // holds the same reference. Niling a field would mutate both. Swap
    // `_G.string` and `_G.math` for fresh shallow copies minus the
    // dangerous fields — secureenv keeps the originals intact.
    lua.exec(
        r#"
        if string then
            local safe_string = {}
            for k, v in pairs(string) do safe_string[k] = v end
            safe_string.dump = nil
            rawset(_G, "string", safe_string)
        end
        if math then
            local safe_math = {}
            for k, v in pairs(math) do safe_math[k] = v end
            safe_math.randomseed = nil
            rawset(_G, "math", safe_math)
        end
        "#,
    )?;
    Ok(())
}
