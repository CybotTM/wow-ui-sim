use super::super::SimState;
use crate::lua_api::AddonInfo;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Create the `LoadAddOn(name)` function that actually loads on-demand addons.
///
/// Returns `(loaded: bool, reason: string|nil)`.
pub(crate) fn create_load_addon_fn(
    lua: &Lua,
    state: Rc<RefCell<SimState>>,
) -> Result<mlua::Function> {
    lua.create_function(move |lua, addon_name: String| load_addon_runtime(lua, &state, &addon_name))
}

/// Runtime addon loading implementation.
///
/// Searches `addon_base_paths` for the addon directory, loads it via the
/// standard loader pipeline, registers it, and fires `ADDON_LOADED`.
fn load_addon_runtime(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    addon_name: &str,
) -> Result<(bool, Value)> {
    if addon_is_loaded(state, addon_name) {
        return Ok((true, Value::Nil));
    }

    let Some(toc_path) = find_addon_toc(state, addon_name) else {
        return missing_addon_result(lua);
    };
    load_addon_dependencies(lua, state, &toc_path);
    execute_addon_load(lua, state, addon_name, &toc_path)
}

fn addon_is_loaded(state: &Rc<RefCell<SimState>>, addon_name: &str) -> bool {
    let s = state.borrow();
    s.addons
        .iter()
        .any(|a| a.folder_name == addon_name && a.loaded)
}

fn missing_addon_result(lua: &Lua) -> Result<(bool, Value)> {
    let reason = lua.create_string("MISSING")?;
    Ok((false, Value::String(reason)))
}

fn load_addon_dependencies(lua: &Lua, state: &Rc<RefCell<SimState>>, toc_path: &std::path::Path) {
    if let Ok(toc) = crate::toc::TocFile::from_file(toc_path) {
        for dep in toc.dependencies() {
            if !addon_is_loaded(state, &dep) {
                let _ = load_addon_runtime(lua, state, &dep);
            }
        }
    }
}

fn execute_addon_load(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    addon_name: &str,
    toc_path: &std::path::Path,
) -> Result<(bool, Value)> {
    let loader_env = crate::lua_api::LoaderEnv::new(lua, Rc::clone(state));
    match crate::loader::load_addon(&loader_env, toc_path) {
        Ok(result) => handle_addon_load_success(lua, state, addon_name, &loader_env, result),
        Err(e) => addon_load_failure(lua, addon_name, &e),
    }
}

fn handle_addon_load_success(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    addon_name: &str,
    loader_env: &crate::lua_api::LoaderEnv<'_>,
    result: crate::loader::LoadResult,
) -> Result<(bool, Value)> {
    let load_time_secs = result.timing.total().as_secs_f64();
    log_addon_load(addon_name, &result);
    register_loaded_addon(state, addon_name, load_time_secs);
    fire_addon_loaded(loader_env, addon_name);
    crate::lua_api::workarounds::apply_post_runtime_addon_load_from_lua(
        lua,
        Rc::clone(state),
        addon_name,
    );
    Ok((true, Value::Nil))
}

fn log_addon_load(addon_name: &str, result: &crate::loader::LoadResult) {
    if std::env::var("WOW_SIM_VERBOSE").is_ok() {
        eprintln!(
            "[LoadAddOn] {} loaded: {} Lua, {} XML ({:.1?})",
            addon_name,
            result.lua_files,
            result.xml_files,
            result.timing.total()
        );
    }
}

fn addon_load_failure(
    lua: &Lua,
    addon_name: &str,
    err: &impl std::fmt::Display,
) -> Result<(bool, Value)> {
    eprintln!("[LoadAddOn] {} failed: {}", addon_name, err);
    let reason = lua.create_string("CORRUPT")?;
    Ok((false, Value::String(reason)))
}

/// Search addon_base_paths for an addon's TOC file.
fn find_addon_toc(state: &Rc<RefCell<SimState>>, addon_name: &str) -> Option<std::path::PathBuf> {
    let s = state.borrow();
    s.addon_base_paths
        .iter()
        .map(|base| base.join(addon_name))
        .find_map(|dir| {
            if dir.is_dir() {
                crate::loader::find_toc_file(&dir)
            } else {
                None
            }
        })
}

/// Register a newly loaded addon in SimState.
fn register_loaded_addon(state: &Rc<RefCell<SimState>>, name: &str, load_time_secs: f64) {
    let mut s = state.borrow_mut();
    if let Some(existing) = s.addons.iter_mut().find(|a| a.folder_name == name) {
        existing.loaded = true;
        existing.load_time_secs = load_time_secs;
    } else {
        s.addons.push(AddonInfo {
            folder_name: name.to_string(),
            title: name.to_string(),
            enabled: true,
            loaded: true,
            load_on_demand: true,
            load_time_secs,
            ..Default::default()
        });
    }
}

/// Fire the ADDON_LOADED event for a just-loaded addon.
fn fire_addon_loaded(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    let arg = env.lua().create_string(addon_name).ok().map(Value::String);
    if let Some(arg) = arg
        && let Err(e) = env.fire_event_with_args("ADDON_LOADED", &[arg])
    {
        eprintln!(
            "[LoadAddOn] Error firing ADDON_LOADED for {}: {}",
            addon_name, e
        );
    }
}
