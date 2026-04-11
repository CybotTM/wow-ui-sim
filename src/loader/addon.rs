//! Addon loading internals.

use crate::lua_api::LoaderEnv;
use crate::saved_variables::SavedVariablesManager;
use crate::toc::TocFile;
use mlua::Table;
use std::path::{Path, PathBuf};
use std::time::Instant;

use super::error::LoadError;
use super::lua_file::load_lua_file;
use super::xml_file::load_xml_file;
use super::{LoadResult, LoadTiming};

/// Context for loading addon files (name, private table, and addon root for path resolution).
pub struct AddonContext<'a> {
    pub name: &'a str,
    pub table: Table,
    /// Addon root directory for fallback path resolution
    pub addon_root: &'a Path,
    /// Whether this addon uses the secure Lua environment (UseSecureEnvironment: 1)
    pub use_secure_env: bool,
    /// Whether to taint code with the addon name (false for Blizzard base UI).
    pub taint: bool,
}

/// Initialize saved variables for an addon (WTF first, then JSON fallback).
fn init_saved_variables(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    folder_name: &str,
    mgr: &mut SavedVariablesManager,
) -> Vec<String> {
    let mut warnings = Vec::new();
    match mgr.load_wtf_for_addon(env.lua(), folder_name) {
        Ok(count) if count > 0 => {
            tracing::debug!(
                "Loaded {} WTF SavedVariables file(s) for {}",
                count,
                toc.name
            );
        }
        Ok(_) => {
            let saved_vars = toc.saved_variables();
            let saved_vars_per_char = toc.saved_variables_per_character();
            if (!saved_vars.is_empty() || !saved_vars_per_char.is_empty())
                && let Err(e) =
                    mgr.init_for_addon(env.lua(), folder_name, &saved_vars, &saved_vars_per_char)
            {
                warnings.push(format!(
                    "Failed to initialize saved variables for {}: {}",
                    folder_name, e
                ));
            }
        }
        Err(e) => {
            warnings.push(format!(
                "Failed to load WTF SavedVariables for {}: {}",
                folder_name, e
            ));
        }
    }
    warnings
}

/// Internal addon loading with optional saved variables.
pub fn load_addon_internal(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    saved_vars_mgr: Option<&mut SavedVariablesManager>,
) -> Result<LoadResult, LoadError> {
    let folder_name = toc
        .addon_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(&toc.name);

    let mut result = LoadResult {
        name: toc.name.clone(),
        lua_files: 0,
        xml_files: 0,
        timing: LoadTiming::default(),
        warnings: Vec::new(),
    };

    maybe_init_saved_variables(env, toc, folder_name, saved_vars_mgr, &mut result);
    let ctx = build_addon_context(env, toc, folder_name)?;
    let nil_symbol_access_start = env.state().borrow().nil_symbol_accesses.len();
    let addon_name = result.name.clone();

    load_addon_files(env, toc, folder_name, &ctx, &mut result);
    append_nil_symbol_access_warnings(env, &addon_name, nil_symbol_access_start, &mut result);
    env.state().borrow_mut().loading_addon_index = None;
    Ok(result)
}

fn maybe_init_saved_variables(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    folder_name: &str,
    saved_vars_mgr: Option<&mut SavedVariablesManager>,
    result: &mut LoadResult,
) {
    let Some(mgr) = saved_vars_mgr else {
        return;
    };

    let sv_start = Instant::now();
    result
        .warnings
        .extend(init_saved_variables(env, toc, folder_name, mgr));
    result.timing.saved_vars_time = sv_start.elapsed();
}

fn build_addon_context<'a>(
    env: &LoaderEnv<'a>,
    toc: &'a TocFile,
    folder_name: &'a str,
) -> Result<AddonContext<'a>, LoadError> {
    let addon_table = env
        .create_addon_table()
        .map_err(|e| LoadError::Lua(e.to_string()))?;
    register_loading_addon(env, folder_name, toc.is_secure_env());

    Ok(AddonContext {
        name: folder_name,
        table: addon_table,
        addon_root: &toc.addon_dir,
        use_secure_env: toc.is_secure_env(),
        taint: !is_blizzard_addon(toc),
    })
}

fn register_loading_addon(env: &LoaderEnv<'_>, folder_name: &str, use_secure_env: bool) {
    // Set loading_addon_index so frames created during this addon's load
    // are attributed to it. Panic if addon not registered — caller bug.
    let addon_idx = resolve_addon_index(env, folder_name);
    let mut state = env.state().borrow_mut();
    state.loading_addon_index = Some(addon_idx);
    if let Some(addon) = state.addons.get_mut(addon_idx as usize) {
        addon.use_secure_env = use_secure_env;
    }
}

fn is_blizzard_addon(toc: &TocFile) -> bool {
    // Blizzard base UI code runs securely (no taint). Third-party addons
    // get tainted with their folder name so issecurevariable tracks the source.
    toc.addon_dir.to_string_lossy().contains("BlizzardUI")
}

/// Find or auto-register addon in the addon list, returning its index.
fn resolve_addon_index(env: &LoaderEnv<'_>, folder_name: &str) -> u16 {
    let mut s = env.state().borrow_mut();
    let idx = s
        .addons
        .iter()
        .position(|a| a.folder_name == folder_name)
        .unwrap_or_else(|| {
            let idx = s.addons.len();
            s.addons.push(crate::lua_api::AddonInfo {
                folder_name: folder_name.to_string(),
                title: folder_name.to_string(),
                enabled: true,
                ..Default::default()
            });
            idx
        });
    idx as u16
}

/// Load all Lua/XML files listed in the TOC, applying local overlay paths.
fn load_addon_files(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    folder_name: &str,
    ctx: &AddonContext,
    result: &mut LoadResult,
) {
    let overlay_dir = Path::new("Interface/AddOns").join(folder_name);

    for (file_rel, file) in toc.files.iter().zip(toc.file_paths()) {
        let resolved_file = resolve_addon_file_path(&overlay_dir, file_rel, file);
        load_addon_file(env, ctx, result, &resolved_file);
    }
}

fn resolve_addon_file_path(
    overlay_dir: &Path,
    file_rel: &Path,
    default_file: std::path::PathBuf,
) -> std::path::PathBuf {
    let overlay_file = overlay_dir.join(file_rel);
    if overlay_file.exists() {
        return overlay_file;
    }
    default_file
}

fn load_addon_file(
    env: &LoaderEnv<'_>,
    ctx: &AddonContext<'_>,
    result: &mut LoadResult,
    file: &std::path::Path,
) {
    match file.extension().and_then(|ext| ext.to_str()).unwrap_or("") {
        "lua" => load_addon_lua_file(env, ctx, result, file),
        "xml" => load_addon_xml_file(env, ctx, result, file),
        _ => result
            .warnings
            .push(format!("{}: unknown file type", file.display())),
    }
}

fn append_nil_symbol_access_warnings(
    env: &LoaderEnv<'_>,
    addon_name: &str,
    start_index: usize,
    result: &mut LoadResult,
) {
    let grouped_accesses = {
        let state = env.state().borrow();
        summarize_nil_symbol_accesses(&state.nil_symbol_accesses[start_index..])
    };
    result.warnings.extend(
        grouped_accesses
            .into_iter()
            .map(|report| format_missing_symbol_report(addon_name, &report)),
    );
}

fn summarize_nil_symbol_accesses(
    accesses: &[crate::lua_api::state::NilSymbolAccess],
) -> Vec<MissingSymbolReport> {
    let mut reports = std::collections::BTreeMap::new();
    for access in accesses {
        let need = classify_nil_symbol_access(access);
        let location = format_nil_symbol_location(access);
        reports.entry(need).or_insert(location);
    }

    reports
        .into_iter()
        .map(|(need, location)| MissingSymbolReport { need, location })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum MissingSymbolNeed {
    Global(String),
    CNamespace(String),
    CMethod { namespace: String, method: String },
}

fn classify_nil_symbol_access(
    access: &crate::lua_api::state::NilSymbolAccess,
) -> MissingSymbolNeed {
    if access.container == "_G" {
        return classify_global_nil_access(&access.key);
    }

    if access.container.starts_with("C_") {
        return MissingSymbolNeed::CMethod {
            namespace: access.container.clone(),
            method: access.key.clone(),
        };
    }

    MissingSymbolNeed::Global(format!("{}.{}", access.container, access.key))
}

fn classify_global_nil_access(key: &str) -> MissingSymbolNeed {
    if key.starts_with("C_") {
        return MissingSymbolNeed::CNamespace(key.to_string());
    }

    MissingSymbolNeed::Global(key.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct MissingSymbolReport {
    need: MissingSymbolNeed,
    location: Option<String>,
}

fn format_missing_symbol_report(addon_name: &str, report: &MissingSymbolReport) -> String {
    let need = match &report.need {
        MissingSymbolNeed::Global(name) => format!("global {name}"),
        MissingSymbolNeed::CNamespace(namespace) => namespace.clone(),
        MissingSymbolNeed::CMethod { namespace, method } => format!("{namespace}.{method}"),
    };

    match &report.location {
        Some(location) => format!("{addon_name} needs {need} (accessed at {location})"),
        None => format!("{addon_name} needs {need}"),
    }
}

fn format_nil_symbol_location(access: &crate::lua_api::state::NilSymbolAccess) -> Option<String> {
    let source = access.source.as_deref()?;
    let line = access.line?;
    Some(format!("{}:{line}", summarize_chunk_source(source)))
}

fn summarize_chunk_source(source: &str) -> String {
    let stripped = source.trim_start_matches(['@', '=']);
    let path = PathBuf::from(stripped);
    path.file_name()
        .and_then(|name| name.to_str())
        .map(ToOwned::to_owned)
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| stripped.to_string())
}

fn load_addon_lua_file(
    env: &LoaderEnv<'_>,
    ctx: &AddonContext<'_>,
    result: &mut LoadResult,
    file: &std::path::Path,
) {
    match load_lua_file(env, file, ctx, &mut result.timing) {
        Ok(()) => result.lua_files += 1,
        Err(error) => result
            .warnings
            .push(format!("{}: {}", file.display(), error)),
    }
    apply_cpp_mixin_stubs(env);
}

fn load_addon_xml_file(
    env: &LoaderEnv<'_>,
    ctx: &AddonContext<'_>,
    result: &mut LoadResult,
    file: &std::path::Path,
) {
    match load_xml_file(env, file, ctx, &mut result.timing) {
        Ok(count) => {
            result.xml_files += 1;
            result.lua_files += count;
        }
        Err(error) => result
            .warnings
            .push(format!("{}: {}", file.display(), error)),
    }
}

/// Patch Lua mixin tables with methods normally provided by the C++ engine.
///
/// WoW's C++ engine provides OnLoad for certain base control button mixins.
/// The Lua side creates empty tables (e.g. `ModelSceneControlButtonMixin = {}`),
/// and derived mixins call `BaseMixin.OnLoad(self)` expecting the C++ method.
/// Runs after each .lua file so stubs are available before the next .xml file
/// creates frames that depend on them.
fn apply_cpp_mixin_stubs(env: &LoaderEnv<'_>) {
    let _ = env.exec(
        r#"
        local ModelSceneControlButtonMixin = rawget(_G, "ModelSceneControlButtonMixin")
        if ModelSceneControlButtonMixin and not ModelSceneControlButtonMixin.OnLoad then
            ModelSceneControlButtonMixin.OnLoad = function() end
        end
        local PerksModelSceneControlButtonMixin = rawget(_G, "PerksModelSceneControlButtonMixin")
        if PerksModelSceneControlButtonMixin and not PerksModelSceneControlButtonMixin.OnLoad then
            PerksModelSceneControlButtonMixin.OnLoad = function() end
        end
        local PetActionBarMixin = rawget(_G, "PetActionBarMixin")
        if PetActionBarMixin and PetActionBarMixin.Update and not PetActionBarMixin._update_guarded then
            PetActionBarMixin._update_guarded = true
            local origUpdate = PetActionBarMixin.Update
            PetActionBarMixin.Update = function(self)
                if not self.actionButtons or #self.actionButtons == 0 then return end
                return origUpdate(self)
            end
        end
        "#,
    );
}
