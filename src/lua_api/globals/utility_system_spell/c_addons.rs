//! C_AddOns, legacy addon globals, and LoadAddOn implementation.

use crate::loader::LoadError;
use crate::lua_api::LoaderEnv;
use crate::lua_api::methods::{
    borrow_lua, borrow_state, borrow_state_mut, create_string, create_table, registry_get,
    registry_set, state_handle, val_to_string,
};
use crate::lua_bridge::{stack_val, table_set_rust_fn};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};
use std::path::PathBuf;
use std::{collections::HashSet, io};

use super::set_global_val;

const ADDON_VERSION_CHECK_KEY: &str = "__addon_version_check_enabled";

// ── C_AddOns registration ─────────────────────────────────────────────────────

pub fn register_c_addons(state: &mut LuaState) -> LuaResult<()> {
    let c_addons = create_table(state);
    let Val::Table(c_addons_ref) = c_addons else {
        unreachable!("create_table must return a table");
    };
    register_c_addons_methods(state, c_addons_ref)?;
    set_global_val(state, "C_AddOns", c_addons);
    Ok(())
}

fn register_c_addons_methods(
    state: &mut LuaState,
    t: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    register_c_addons_queries(state, t)?;
    register_c_addons_state(state, t)?;
    Ok(())
}

/// Query/info methods: read-only addon introspection.
fn register_c_addons_queries(
    state: &mut LuaState,
    t: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn(state, t, "GetNumAddOns", c_addons_get_num_addons)?;
    table_set_rust_fn(state, t, "GetAddOnInfo", c_addons_get_addon_info)?;
    table_set_rust_fn(state, t, "IsAddOnLoaded", c_addons_is_addon_loaded)?;
    table_set_rust_fn(
        state,
        t,
        "IsAddOnLoadOnDemand",
        c_addons_is_addon_load_on_demand,
    )?;
    table_set_rust_fn(
        state,
        t,
        "GetAddOnEnableState",
        c_addons_get_addon_enable_state,
    )?;
    table_set_rust_fn(state, t, "GetAddOnMetadata", c_addons_get_addon_metadata)?;
    table_set_rust_fn(state, t, "DoesAddOnExist", c_addons_does_addon_exist)?;
    table_set_rust_fn(state, t, "GetAddOnName", c_addons_get_addon_name)?;
    table_set_rust_fn(state, t, "GetAddOnTitle", c_addons_get_addon_title)?;
    table_set_rust_fn(state, t, "GetAddOnNotes", c_addons_get_addon_notes)?;
    table_set_rust_fn(state, t, "GetAddOnSecurity", c_addons_get_addon_security)?;
    Ok(())
}

/// State-mutation methods: enable/disable, version check, load.
fn register_c_addons_state(
    state: &mut LuaState,
    t: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn(state, t, "EnableAddOn", c_addons_enable_addon)?;
    table_set_rust_fn(state, t, "DisableAddOn", c_addons_disable_addon)?;
    table_set_rust_fn(state, t, "EnableAllAddOns", c_addons_enable_all_addons)?;
    table_set_rust_fn(state, t, "DisableAllAddOns", c_addons_disable_all_addons)?;
    table_set_rust_fn(
        state,
        t,
        "IsAddonVersionCheckEnabled",
        c_addons_is_addon_version_check_enabled,
    )?;
    table_set_rust_fn(
        state,
        t,
        "SetAddonVersionCheck",
        c_addons_set_addon_version_check,
    )?;
    table_set_rust_fn(state, t, "LoadAddOn", c_addons_load_addon)?;
    Ok(())
}

pub fn register_legacy_addon_globals(state: &mut LuaState) -> LuaResult<()> {
    register_legacy_addon_fns(state)?;
    register_addon_actions_blocked(state);
    Ok(())
}

fn register_legacy_addon_fns(state: &mut LuaState) -> LuaResult<()> {
    table_set_rust_fn(state, state.global, "GetNumAddOns", c_addons_get_num_addons)?;
    table_set_rust_fn(
        state,
        state.global,
        "IsAddOnLoaded",
        c_addons_is_addon_loaded,
    )?;
    table_set_rust_fn(
        state,
        state.global,
        "GetAddOnMetadata",
        c_addons_get_addon_metadata,
    )?;
    table_set_rust_fn(
        state,
        state.global,
        "GetAddOnEnableState",
        legacy_get_addon_enable_state,
    )?;
    table_set_rust_fn(
        state,
        state.global,
        "IsAddOnLoadOnDemand",
        c_addons_is_addon_load_on_demand,
    )?;
    table_set_rust_fn(state, state.global, "LoadAddOn", c_addons_load_addon)?;
    Ok(())
}

fn register_addon_actions_blocked(state: &mut LuaState) {
    let blocked = create_table(state);
    let key_ref = state.gc.intern_string(b"ADDON_ACTIONS_BLOCKED");
    let global_ref = state.global;
    if let Some(global) = state.gc.tables.get_mut(global_ref) {
        let _ = global.raw_set(Val::Str(key_ref), blocked, &state.gc.string_arena);
    }
    state.gc.barrier_back(global_ref);
}

// ── Addon query helpers ───────────────────────────────────────────────────────

fn addon_index_from_value(state: &LuaState, addon: Val) -> Option<usize> {
    match addon {
        Val::Num(index) if index.is_finite() && index.fract() == 0.0 && index >= 1.0 => {
            Some(index as usize - 1)
        }
        Val::Str(_) => {
            let name = val_to_string(state, addon)?;
            let sim = borrow_state(state).ok()?;
            sim.addons.iter().position(|a| a.folder_name == name)
        }
        _ => None,
    }
}

fn addon_name_from_value(state: &LuaState, addon: Val) -> Option<String> {
    match addon {
        Val::Str(_) => val_to_string(state, addon),
        other => {
            let index = addon_index_from_value(state, other)?;
            let sim = borrow_state(state).ok()?;
            sim.addons.get(index).map(|a| a.folder_name.clone())
        }
    }
}

fn with_addon<R>(
    state: &LuaState,
    addon: Val,
    f: impl FnOnce(&crate::lua_api::AddonInfo) -> R,
) -> Option<R> {
    let index = addon_index_from_value(state, addon)?;
    let sim = borrow_state(state).ok()?;
    sim.addons.get(index).map(f)
}

fn addon_metadata(addon: &crate::lua_api::AddonInfo, field: &str) -> Option<String> {
    match field {
        "Title" => Some(addon.title.clone()),
        "Notes" => (!addon.notes.is_empty()).then(|| addon.notes.clone()),
        "Version" => Some("@project-version@".to_string()),
        _ => None,
    }
}

fn push_addon_info(state: &mut LuaState, addon: &crate::lua_api::AddonInfo) -> u32 {
    let folder_name = create_string(state, &addon.folder_name);
    let title = create_string(state, &addon.title);
    let notes = (!addon.notes.is_empty()).then(|| create_string(state, &addon.notes));
    state.push(folder_name);
    state.push(title);
    if addon.notes.is_empty() {
        state.push(Val::Nil);
    } else {
        state.push(notes.unwrap_or(Val::Nil));
    }
    state.push(Val::Bool(addon.enabled));
    4
}

fn registry_bool(state: &mut LuaState, key: &'static str) -> bool {
    matches!(registry_get(state, key), Val::Bool(true))
}

fn set_registry_bool(state: &mut LuaState, key: &'static str, value: bool) {
    registry_set(state, key, Val::Bool(value));
}

// ── Addon existence / TOC ─────────────────────────────────────────────────────

fn default_runtime_addon_bases() -> Vec<PathBuf> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    vec![
        root.join("Interface/BlizzardUI"),
        root.join("Interface/AddOns"),
    ]
}

fn find_runtime_addon_toc(state: &LuaState, addon_name: &str) -> Option<PathBuf> {
    let bases = {
        let sim = borrow_state(state).ok()?;
        if sim.addon_base_paths.is_empty() {
            default_runtime_addon_bases()
        } else {
            sim.addon_base_paths.clone()
        }
    };
    for base in bases {
        let addon_dir = base.join(addon_name);
        if let Some(toc_path) = crate::loader::find_toc_file(&addon_dir) {
            return Some(toc_path);
        }
    }
    None
}

fn addon_exists(state: &LuaState, addon_name: &str) -> bool {
    let registered = {
        let sim = match borrow_state(state) {
            Ok(sim) => sim,
            Err(_) => return false,
        };
        sim.addons.iter().any(|a| a.folder_name == addon_name)
    };
    registered || find_runtime_addon_toc(state, addon_name).is_some()
}

// ── C_AddOns function implementations ────────────────────────────────────────

fn c_addons_get_num_addons(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.addons.len() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

fn c_addons_get_addon_info(state: &mut LuaState) -> LuaResult<u32> {
    let addon = stack_val(state, 1);
    let Some(index) = addon_index_from_value(state, addon) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = {
        let sim = borrow_state(state)?;
        sim.addons.get(index).cloned()
    };
    let Some(info) = info else {
        state.push(Val::Nil);
        return Ok(1);
    };
    Ok(push_addon_info(state, &info))
}

fn c_addons_is_addon_loaded(state: &mut LuaState) -> LuaResult<u32> {
    let addon_name = addon_name_from_value(state, stack_val(state, 1)).unwrap_or_default();
    let (loaded_or_loading, loaded) = if addon_name.is_empty() {
        (false, false)
    } else {
        let sim = borrow_state(state)?;
        let loaded = sim
            .addons
            .iter()
            .find(|addon| addon.folder_name == addon_name)
            .map(|addon| addon.loaded)
            .unwrap_or(false);
        let loading = sim
            .loading_addon_index
            .and_then(|idx| sim.addons.get(idx as usize))
            .map(|addon| addon.folder_name == addon_name)
            .unwrap_or(false);
        (loaded || loading, loaded)
    };
    state.push(Val::Bool(loaded_or_loading));
    state.push(Val::Bool(loaded));
    Ok(2)
}

fn c_addons_is_addon_load_on_demand(state: &mut LuaState) -> LuaResult<u32> {
    let lod = with_addon(state, stack_val(state, 1), |a| a.load_on_demand).unwrap_or(false);
    state.push(Val::Bool(lod));
    Ok(1)
}

fn c_addons_get_addon_enable_state(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = with_addon(state, stack_val(state, 1), |a| a.enabled).unwrap_or(false);
    state.push(Val::Num(if enabled { 2.0 } else { 0.0 }));
    Ok(1)
}

fn c_addons_enable_addon(state: &mut LuaState) -> LuaResult<u32> {
    if let Some(index) = addon_index_from_value(state, stack_val(state, 1))
        && let Some(addon) = borrow_state_mut(state)?.addons.get_mut(index)
    {
        addon.enabled = true;
    }
    Ok(0)
}

fn c_addons_disable_addon(state: &mut LuaState) -> LuaResult<u32> {
    if let Some(index) = addon_index_from_value(state, stack_val(state, 1))
        && let Some(addon) = borrow_state_mut(state)?.addons.get_mut(index)
        && addon.folder_name != "__BuiltIn"
    {
        addon.enabled = false;
    }
    Ok(0)
}

fn c_addons_enable_all_addons(state: &mut LuaState) -> LuaResult<u32> {
    for addon in &mut borrow_state_mut(state)?.addons {
        addon.enabled = true;
    }
    Ok(0)
}

fn c_addons_disable_all_addons(state: &mut LuaState) -> LuaResult<u32> {
    for addon in &mut borrow_state_mut(state)?.addons {
        if addon.folder_name != "__BuiltIn" {
            addon.enabled = false;
        }
    }
    Ok(0)
}

fn c_addons_get_addon_metadata(state: &mut LuaState) -> LuaResult<u32> {
    let addon_name = addon_name_from_value(state, stack_val(state, 1)).unwrap_or_default();
    let field = val_to_string(state, stack_val(state, 2)).unwrap_or_default();
    let value = with_addon(state, stack_val(state, 1), |a| addon_metadata(a, &field))
        .flatten()
        .or_else(|| (field == "Title").then_some(addon_name));
    match value {
        Some(v) => {
            let v = create_string(state, &v);
            state.push(v);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_addons_does_addon_exist(state: &mut LuaState) -> LuaResult<u32> {
    let addon_name = addon_name_from_value(state, stack_val(state, 1)).unwrap_or_default();
    let exists = !addon_name.is_empty() && addon_exists(state, &addon_name);
    state.push(Val::Bool(exists));
    Ok(1)
}

fn c_addons_get_addon_name(state: &mut LuaState) -> LuaResult<u32> {
    match with_addon(state, stack_val(state, 1), |a| a.folder_name.clone()) {
        Some(name) => {
            let v = create_string(state, &name);
            state.push(v);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_addons_get_addon_title(state: &mut LuaState) -> LuaResult<u32> {
    match with_addon(state, stack_val(state, 1), |a| a.title.clone()) {
        Some(title) => {
            let v = create_string(state, &title);
            state.push(v);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_addons_get_addon_notes(state: &mut LuaState) -> LuaResult<u32> {
    match with_addon(state, stack_val(state, 1), |a| a.notes.clone()).filter(|n| !n.is_empty()) {
        Some(notes) => {
            let v = create_string(state, &notes);
            state.push(v);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_addons_get_addon_security(state: &mut LuaState) -> LuaResult<u32> {
    let security = with_addon(state, stack_val(state, 1), |a| {
        if a.folder_name == "__BuiltIn" || a.folder_name.starts_with("Blizzard_") {
            "SECURE"
        } else {
            "INSECURE"
        }
    })
    .unwrap_or("INSECURE");
    let v = create_string(state, security);
    state.push(v);
    Ok(1)
}

fn c_addons_is_addon_version_check_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = registry_bool(state, ADDON_VERSION_CHECK_KEY);
    state.push(Val::Bool(enabled));
    Ok(1)
}

fn c_addons_set_addon_version_check(state: &mut LuaState) -> LuaResult<u32> {
    set_registry_bool(
        state,
        ADDON_VERSION_CHECK_KEY,
        !matches!(stack_val(state, 1), Val::Nil | Val::Bool(false)),
    );
    Ok(0)
}

pub fn c_addons_load_addon(state: &mut LuaState) -> LuaResult<u32> {
    let Some(addon_name) = addon_name_from_value(state, stack_val(state, 1)) else {
        return push_load_result(state, false, Some("MISSING"));
    };
    if with_addon(state, stack_val(state, 1), |a| a.loaded).unwrap_or(false) {
        return push_load_result(state, true, None);
    }
    if addon_is_disabled(state, &addon_name) {
        return push_load_result(state, false, Some("DISABLED"));
    }
    let loader_env = LoaderEnv::from_parts_active(borrow_lua(state)?, state_handle(state)?, state);
    let mut loading = HashSet::new();
    match load_runtime_addon_recursive(state, &loader_env, &addon_name, &mut loading) {
        Ok(()) => push_load_result(state, true, None),
        Err(error) => push_load_result(state, false, Some(&error.to_string())),
    }
}

/// `true` iff the addon is registered AND its `enabled` flag is false.
/// Unregistered addons are not considered disabled — `LoadAddOn` will
/// fall through to `MISSING` (or auto-register at load time).
fn addon_is_disabled(state: &LuaState, addon_name: &str) -> bool {
    let Ok(sim) = borrow_state(state) else {
        return false;
    };
    sim.addons
        .iter()
        .find(|a| a.folder_name == addon_name)
        .map(|a| !a.enabled)
        .unwrap_or(false)
}

fn load_runtime_addon_recursive(
    state: &mut LuaState,
    loader_env: &LoaderEnv<'_>,
    addon_name: &str,
    loading: &mut HashSet<String>,
) -> Result<(), LoadError> {
    if is_addon_loaded_by_name(state, addon_name) {
        return Ok(());
    }
    if !loading.insert(addon_name.to_string()) {
        return Ok(());
    }

    let result = load_runtime_addon_with_dependencies(state, loader_env, addon_name, loading);
    loading.remove(addon_name);
    result
}

fn load_runtime_addon_with_dependencies(
    state: &mut LuaState,
    loader_env: &LoaderEnv<'_>,
    addon_name: &str,
    loading: &mut HashSet<String>,
) -> Result<(), LoadError> {
    eprintln!("[load_addon] begin {addon_name}");
    let toc_path = find_runtime_addon_toc(state, addon_name)
        .ok_or_else(|| missing_runtime_addon_error(addon_name))?;
    eprintln!("[load_addon] toc {}", toc_path.display());
    let toc = crate::toc::TocFile::from_file(&toc_path).map_err(LoadError::Toc)?;

    for dependency in runtime_addon_dependencies(state, &toc) {
        eprintln!("[load_addon] {addon_name} -> dep {dependency}");
        if addon_is_disabled(state, &dependency) {
            return Err(disabled_dep_error(&dependency));
        }
        load_runtime_addon_recursive(state, loader_env, &dependency, loading)?;
    }

    eprintln!("[load_addon] files {addon_name}");
    let result = crate::loader::load_addon_from_toc(loader_env, &toc)?;
    for warning in &result.warnings {
        eprintln!("[load_addon] warning {addon_name}: {warning}");
    }
    eprintln!("[load_addon] loaded {addon_name}");
    mark_addon_loaded(loader_env, addon_name);
    fire_addon_loaded(state, loader_env, addon_name);
    eprintln!("[load_addon] event {addon_name}");
    Ok(())
}

fn runtime_addon_dependencies(state: &LuaState, toc: &crate::toc::TocFile) -> Vec<String> {
    let mut deps = toc.dependencies();
    for dep in toc.optional_deps() {
        if find_runtime_addon_toc(state, &dep).is_some() && !deps.contains(&dep) {
            deps.push(dep);
        }
    }
    deps
}

fn is_addon_loaded_by_name(state: &LuaState, addon_name: &str) -> bool {
    borrow_state(state)
        .ok()
        .and_then(|sim| {
            sim.addons
                .iter()
                .find(|addon| addon.folder_name == addon_name)
                .map(|addon| addon.loaded)
        })
        .unwrap_or(false)
}

fn missing_runtime_addon_error(addon_name: &str) -> LoadError {
    LoadError::Io(io::Error::new(
        io::ErrorKind::NotFound,
        format!("runtime addon not found: {addon_name}"),
    ))
}

/// Construct the `LoadError` whose `Display` renders to exactly
/// `"DEP_DISABLED"` — that string flows back through
/// `c_addons_load_addon`'s `error.to_string()` and becomes the second
/// return of `LoadAddOn`, matching real WoW.
fn disabled_dep_error(dependency: &str) -> LoadError {
    LoadError::DepDisabled(dependency.to_string())
}

fn fire_addon_loaded(state: &mut LuaState, loader_env: &LoaderEnv<'_>, addon_name: &str) {
    let addon_name_val = create_string(state, addon_name);
    let _ = loader_env.fire_event_with_args("ADDON_LOADED", &[addon_name_val]);
}

fn mark_addon_loaded(loader_env: &LoaderEnv, addon_name: &str) {
    let mut sim = loader_env.state().borrow_mut();
    if let Some(addon) = sim.addons.iter_mut().find(|a| a.folder_name == addon_name) {
        addon.loaded = true;
        addon.enabled = true;
    }
}

fn push_load_result(
    state: &mut LuaState,
    success: bool,
    error_msg: Option<&str>,
) -> LuaResult<u32> {
    state.push(Val::Bool(success));
    match error_msg {
        Some(msg) => {
            let msg_val = create_string(state, msg);
            state.push(msg_val);
        }
        None => state.push(Val::Nil),
    }
    Ok(2)
}

fn legacy_get_addon_enable_state(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(2.0));
    Ok(1)
}
