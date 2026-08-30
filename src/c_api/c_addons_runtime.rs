use crate::loader::LoadError;
use crate::lua_api::LoaderEnv;
use crate::lua_api::globals::inventory_slot::REGISTERED_GET_INVENTORY_SLOT_INFO_KEY;
use crate::lua_api::methods::{
    borrow_lua, borrow_state, create_string, create_table, registry_get, registry_set,
    state_handle, table_get_static, table_set_static,
};
use rilua::{Val, vm::state::LuaState};
use std::{collections::HashSet, io};

const TRANSMOG_SCOPE_ENV_REGISTRY_KEY: &str = "__transmog_inventory_slot_scope_env";

pub(super) fn load_runtime_addon(state: &mut LuaState, addon_name: &str) -> Result<(), LoadError> {
    let loader_env = LoaderEnv::from_parts_active(
        borrow_lua(state).map_err(|err| LoadError::Lua(err.to_string()))?,
        state_handle(state).map_err(|err| LoadError::Lua(err.to_string()))?,
        state,
    );
    crate::lua_api::workarounds::apply_for_runtime_addon_preload(&loader_env, addon_name);

    load_runtime_addon_recursive(state, &loader_env, addon_name)
}

fn load_runtime_addon_recursive(
    state: &mut LuaState,
    loader_env: &LoaderEnv<'_>,
    addon_name: &str,
) -> Result<(), LoadError> {
    if is_addon_loaded_by_name(state, addon_name) || is_addon_loading_by_name(state, addon_name) {
        return Ok(());
    }

    load_runtime_addon_with_dependencies(state, loader_env, addon_name)
}

fn load_runtime_addon_with_dependencies(
    state: &mut LuaState,
    loader_env: &LoaderEnv<'_>,
    addon_name: &str,
) -> Result<(), LoadError> {
    let origin = crate::loader::runtime_load_addon_origin(
        borrow_state(state)
            .map(|sim| sim.xml_load_addon_depth)
            .unwrap_or(0),
    );
    crate::loader::trace_load_addon(origin, format!("begin {addon_name}"));
    let toc_path = super::find_runtime_addon_toc(state, addon_name)
        .ok_or_else(|| missing_runtime_addon_error(addon_name))?;
    crate::loader::trace_load_addon(origin, format!("toc {}", toc_path.display()));
    let toc = crate::toc::TocFile::from_file(&toc_path).map_err(LoadError::Toc)?;
    let loading_guard = crate::loader::begin_addon_load(loader_env, addon_name, &toc);
    apply_mists_runtime_preload(loader_env, &toc, &toc_path, origin)?;

    for dependency in runtime_foundation_dependencies(state, addon_name) {
        crate::loader::trace_load_addon(origin, format!("{addon_name} -> foundation {dependency}"));
        load_runtime_addon_recursive(state, loader_env, dependency)?;
    }

    for dependency in runtime_addon_dependencies(state, &toc) {
        crate::loader::trace_load_addon(origin, format!("{addon_name} -> dep {dependency}"));
        if super::addon_is_disabled(state, &dependency) {
            return Err(disabled_dep_error(&dependency));
        }
        load_runtime_addon_recursive(state, loader_env, &dependency)?;
    }

    crate::loader::trace_load_addon(origin, format!("files {addon_name}"));
    let mut result = load_runtime_addon_files(state, loader_env, addon_name, &toc)?;
    crate::lua_api::workarounds::apply_for_runtime_addon_load(loader_env, addon_name);
    loading_guard.commit_loaded();
    fire_addon_loaded(state, loader_env, addon_name);
    crate::loader::append_pending_nested_addon_diagnostics(
        loader_env,
        loading_guard.addon_index(),
        &mut result,
    );
    crate::loader::trace_load_result_diagnostics(origin, addon_name, &result);
    route_finalized_runtime_addon_diagnostics(loader_env, result.diagnostics());
    loader_env.state().borrow_mut().invalidate_strata_buckets();
    crate::loader::trace_load_addon(origin, format!("event {addon_name}"));
    crate::loader::trace_load_addon(origin, format!("loaded {addon_name}"));
    Ok(())
}

fn route_finalized_runtime_addon_diagnostics(
    loader_env: &LoaderEnv<'_>,
    diagnostics: crate::loader::LoadDiagnostics,
) {
    if diagnostics.is_empty() {
        return;
    }

    let mut state = loader_env.state().borrow_mut();
    if let Some(parent_addon_index) = state.loading_addon_stack.iter().rev().nth(1).copied() {
        state
            .pending_nested_addon_diagnostics
            .entry(parent_addon_index)
            .or_default()
            .extend(diagnostics);
    } else {
        state.runtime_addon_diagnostics.extend(diagnostics);
    }
}

#[cfg(feature = "client-mists")]
fn apply_mists_runtime_preload(
    loader_env: &LoaderEnv<'_>,
    toc: &crate::toc::TocFile,
    toc_path: &std::path::Path,
    origin: crate::loader::LoadAddonTraceOrigin,
) -> Result<(), LoadError> {
    if let Some(result) =
        crate::mists::character_frame_preload::ensure_before_addon(loader_env, toc, toc_path)?
    {
        crate::loader::trace_load_result_diagnostics(origin, &result.name, &result);
    }
    Ok(())
}

#[cfg(not(feature = "client-mists"))]
fn apply_mists_runtime_preload(
    _loader_env: &LoaderEnv<'_>,
    _toc: &crate::toc::TocFile,
    _toc_path: &std::path::Path,
    _origin: crate::loader::LoadAddonTraceOrigin,
) -> Result<(), LoadError> {
    Ok(())
}

fn load_runtime_addon_files(
    state: &mut LuaState,
    loader_env: &LoaderEnv<'_>,
    addon_name: &str,
    toc: &crate::toc::TocFile,
) -> Result<crate::loader::LoadResult, LoadError> {
    if addon_name == "Blizzard_TransmogShared" {
        return with_registered_inventory_slot_info(state, |state| {
            load_runtime_addon_files_unscoped(state, loader_env, toc)
        });
    }

    load_runtime_addon_files_unscoped(state, loader_env, toc)
}

fn with_registered_inventory_slot_info<T>(
    state: &mut LuaState,
    load: impl FnOnce(&mut LuaState) -> Result<T, LoadError>,
) -> Result<T, LoadError> {
    let sim_state = state_handle(state).map_err(|error| LoadError::Lua(error.to_string()))?;
    let global = Val::Table(state.global);
    let previous_global = table_get_static(state, global, "GetInventorySlotInfo");
    let registered = registry_get(state, REGISTERED_GET_INVENTORY_SLOT_INFO_KEY);
    let scope_env = create_inventory_slot_scope_env(state, registered);
    registry_set(state, TRANSMOG_SCOPE_ENV_REGISTRY_KEY, scope_env);

    let previous_scope_env = {
        let mut sim = sim_state.borrow_mut();
        sim.loading_scoped_script_env.replace(scope_env)
    };
    table_set_static(state, global, "GetInventorySlotInfo", registered);

    let result = load(state);

    table_set_static(state, global, "GetInventorySlotInfo", previous_global);
    sim_state.borrow_mut().loading_scoped_script_env = previous_scope_env;
    registry_set(state, TRANSMOG_SCOPE_ENV_REGISTRY_KEY, Val::Nil);
    result
}

fn create_inventory_slot_scope_env(state: &mut LuaState, registered: Val) -> Val {
    let scope_env = create_table(state);
    let metatable = create_table(state);
    let global = Val::Table(state.global);
    table_set_static(state, scope_env, "GetInventorySlotInfo", registered);
    table_set_static(state, metatable, "__index", global);
    table_set_static(state, metatable, "__newindex", global);

    let (Val::Table(scope_ref), Val::Table(metatable_ref)) = (scope_env, metatable) else {
        unreachable!("created scope environment and metatable must be tables");
    };
    state
        .gc
        .tables
        .get_mut(scope_ref)
        .expect("created scope environment must remain live")
        .set_metatable(Some(metatable_ref));
    scope_env
}

fn load_runtime_addon_files_unscoped(
    state: &mut LuaState,
    loader_env: &LoaderEnv<'_>,
    toc: &crate::toc::TocFile,
) -> Result<crate::loader::LoadResult, LoadError> {
    if !toc.loads_as_blizzard_code() {
        return crate::loader::load_addon_from_toc(loader_env, toc);
    }

    let saved_taints = crate::lua_api::taint::clear_active_stack_taint(state);
    let result = crate::loader::load_addon_from_toc(loader_env, toc);
    crate::lua_api::taint::restore_active_stack_taint(state, saved_taints);
    result
}

fn runtime_addon_dependencies(state: &LuaState, toc: &crate::toc::TocFile) -> Vec<String> {
    let mut deps = Vec::new();
    let mut seen = HashSet::new();

    for dep in toc.dependencies() {
        if required_dependency_applies_to_screen(state, &dep) && seen.insert(dep.clone()) {
            deps.push(dep);
        }
    }
    for dep in toc.optional_deps() {
        if super::find_runtime_addon_toc(state, &dep).is_some() && seen.insert(dep.clone()) {
            deps.push(dep);
        }
    }
    deps
}

fn required_dependency_applies_to_screen(state: &LuaState, dependency: &str) -> bool {
    let is_glue = borrow_state(state)
        .map(|sim| sim.screen_kind.is_glue())
        .unwrap_or(false);

    match (dependency, is_glue) {
        ("Blizzard_GlueParent", false) => false,
        ("Blizzard_UIParent", true) => false,
        _ => true,
    }
}

fn runtime_foundation_dependencies(state: &LuaState, addon_name: &str) -> Vec<&'static str> {
    if !addon_name.starts_with("Blizzard_") {
        return Vec::new();
    }

    let screen_kind = borrow_state(state)
        .ok()
        .map(|sim| sim.screen_kind)
        .unwrap_or(crate::screen::ScreenKind::Game);
    let foundations = match screen_kind {
        crate::screen::ScreenKind::Game => super::GAME_RUNTIME_FOUNDATIONS,
        crate::screen::ScreenKind::Login
        | crate::screen::ScreenKind::CharacterSelect
        | crate::screen::ScreenKind::CharacterCreate => super::GLUE_RUNTIME_FOUNDATIONS,
    };
    let end = foundations
        .iter()
        .position(|candidate| *candidate == addon_name)
        .unwrap_or(foundations.len());
    foundations[..end]
        .iter()
        .copied()
        .filter(|dependency| super::find_runtime_addon_toc(state, dependency).is_some())
        .collect()
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

fn is_addon_loading_by_name(state: &LuaState, addon_name: &str) -> bool {
    borrow_state(state)
        .map(|sim| sim.is_addon_loading(addon_name))
        .unwrap_or(false)
}

fn missing_runtime_addon_error(addon_name: &str) -> LoadError {
    LoadError::Io(io::Error::new(
        io::ErrorKind::NotFound,
        format!("runtime addon not found: {addon_name}"),
    ))
}

fn disabled_dep_error(dependency: &str) -> LoadError {
    LoadError::DepDisabled(dependency.to_string())
}

fn fire_addon_loaded(state: &mut LuaState, loader_env: &LoaderEnv<'_>, addon_name: &str) {
    let saved_top = state.top;
    let addon_name_val = create_string(state, addon_name);
    let _ = loader_env.fire_event_with_args("ADDON_LOADED", &[addon_name_val]);
    state.top = saved_top;
}

#[cfg(test)]
mod tests {
    use super::with_registered_inventory_slot_info;
    use crate::loader::LoadError;
    use crate::lua_api::WowLuaEnv;
    use crate::lua_api::methods::{call_function_state, create_string, table_get_static};
    use rilua::{LuaApiMut, Val};

    #[test]
    fn runtime_scope_restores_prior_inventory_slot_global_after_error() {
        let env = WowLuaEnv::new().expect("environment should initialize");
        env.apply_post_event_workarounds();
        env.exec("GetInventorySlotInfo = function() return 'prior' end")
            .expect("prior global should install");

        let mut lua = env.rilua_mut();
        let state = lua.state_mut();
        let global = Val::Table(state.global);
        let previous = table_get_static(state, global, "GetInventorySlotInfo");

        let result: Result<(), LoadError> = with_registered_inventory_slot_info(state, |state| {
            let active = table_get_static(state, Val::Table(state.global), "GetInventorySlotInfo");
            assert!(matches!(active, Val::Function(_)));
            let slot_name = create_string(state, "HeadSlot");
            let values = call_function_state(state, active, &[slot_name])
                .expect("registered inventory slot function should remain callable");
            assert_eq!(values, Val::Num(1.0));
            let probe = LuaApiMut::load_bytes(
                state,
                b"return GetInventorySlotInfo('HeadSlot')",
                "transmog-scope-probe",
            )
            .expect("scope probe should compile");
            let probed = call_function_state(state, Val::Function(probe.gc_ref()), &[])
                .expect("bare global should resolve inside scope");
            assert_eq!(probed, Val::Num(1.0));
            Err(LoadError::Lua("expected load failure".to_string()))
        });

        assert!(result.is_err());
        assert_eq!(
            table_get_static(state, global, "GetInventorySlotInfo"),
            previous
        );
    }
}
