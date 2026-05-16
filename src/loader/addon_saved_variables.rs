//! SavedVariables glue for addon loading.

use std::time::Instant;

use crate::lua_api::LoaderEnv;
use crate::saved_variables::SavedVariablesManager;
use crate::toc::TocFile;

use super::LoadResult;

pub(super) fn maybe_init_saved_variables<'a>(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    folder_name: &str,
    saved_vars_mgr: Option<&'a mut SavedVariablesManager>,
    result: &mut LoadResult,
) -> Option<&'a mut SavedVariablesManager> {
    let sv_start = Instant::now();
    let saved_vars_mgr = match saved_vars_mgr {
        Some(mgr) => {
            result
                .warnings
                .extend(init_saved_variables(env, toc, folder_name, mgr));
            Some(mgr)
        }
        None => {
            seed_console_saved_variables_without_persistence(env, toc, folder_name, result);
            None
        }
    };
    result.timing.saved_vars_time = sv_start.elapsed();
    saved_vars_mgr
}

pub(super) fn maybe_restore_clobbered_saved_variables(
    env: &LoaderEnv<'_>,
    folder_name: &str,
    saved_vars_mgr: Option<&mut SavedVariablesManager>,
) {
    let Some(mgr) = saved_vars_mgr else {
        return;
    };

    let restored = env.with_state(|state| {
        Ok::<usize, crate::Error>(mgr.restore_clobbered_globals(state, folder_name))
    });
    if let Ok(count) = restored
        && count > 0
    {
        tracing::debug!(
            "Restored {} clobbered SavedVariables global(s) for {}",
            count,
            folder_name
        );
    }
}

fn init_saved_variables(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    folder_name: &str,
    mgr: &mut SavedVariablesManager,
) -> Vec<String> {
    let mut warnings = Vec::new();
    let wtf_result = env.with_state(|state| mgr.load_wtf_for_addon(state, folder_name));
    match wtf_result {
        Ok(count) if count > 0 => {
            tracing::debug!(
                "Loaded {} WTF SavedVariables file(s) for {}",
                count,
                toc.name
            );
        }
        Ok(_) => {}
        Err(e) => {
            warnings.push(format!(
                "Failed to load WTF SavedVariables for {}: {}",
                folder_name, e
            ));
        }
    }

    let saved_vars = toc.saved_variables();
    let saved_vars_per_char = toc.saved_variables_per_character();
    if (!saved_vars.is_empty() || !saved_vars_per_char.is_empty())
        && let Err(e) = env.with_state(|state| {
            mgr.init_for_addon(state, folder_name, &saved_vars, &saved_vars_per_char)
        })
    {
        warnings.push(format!(
            "Failed to initialize saved variables for {}: {}",
            folder_name, e
        ));
    }
    warnings
}

fn seed_console_saved_variables_without_persistence(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    folder_name: &str,
    result: &mut LoadResult,
) {
    if folder_name != "Blizzard_Console" {
        return;
    }

    let saved_vars = toc.saved_variables();
    if saved_vars.is_empty() {
        return;
    }

    if let Err(error) = env.with_state(|state| {
        SavedVariablesManager::seed_declared_globals(state, &saved_vars, &[]);
        Ok::<(), crate::Error>(())
    }) {
        result.warnings.push(format!(
            "Failed to seed console saved variables for {} without persistence: {}",
            folder_name, error
        ));
    }
}
