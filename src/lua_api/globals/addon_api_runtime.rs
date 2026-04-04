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
    {
        let s = state.borrow();
        if s.addons
            .iter()
            .any(|a| a.folder_name == addon_name && a.loaded)
        {
            return Ok((true, Value::Nil));
        }
    }

    let toc_path = match find_addon_toc(state, addon_name) {
        Some(path) => path,
        None => {
            let reason = lua.create_string("MISSING")?;
            return Ok((false, Value::String(reason)));
        }
    };

    if let Ok(toc) = crate::toc::TocFile::from_file(&toc_path) {
        for dep in toc.dependencies() {
            let already_loaded = {
                let s = state.borrow();
                s.addons.iter().any(|a| a.folder_name == dep && a.loaded)
            };
            if !already_loaded {
                let _ = load_addon_runtime(lua, state, &dep);
            }
        }
    }

    let loader_env = crate::lua_api::LoaderEnv::new(lua, Rc::clone(state));
    match crate::loader::load_addon(&loader_env, &toc_path) {
        Ok(result) => {
            let load_time_secs = result.timing.total().as_secs_f64();
            if std::env::var("WOW_SIM_VERBOSE").is_ok() {
                eprintln!(
                    "[LoadAddOn] {} loaded: {} Lua, {} XML ({:.1?})",
                    addon_name,
                    result.lua_files,
                    result.xml_files,
                    result.timing.total()
                );
            }
            register_loaded_addon(state, addon_name, load_time_secs);
            fire_addon_loaded(&loader_env, addon_name);
            crate::lua_api::workarounds::apply_post_runtime_addon_load_from_lua(
                lua,
                Rc::clone(state),
                addon_name,
            );
            Ok((true, Value::Nil))
        }
        Err(e) => {
            eprintln!("[LoadAddOn] {} failed: {}", addon_name, e);
            let reason = lua.create_string("CORRUPT")?;
            Ok((false, Value::String(reason)))
        }
    }
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
