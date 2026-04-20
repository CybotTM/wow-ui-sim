//! Addon loader - loads addons from TOC files.

mod addon;
pub(crate) mod button;
pub(crate) mod bytecode;
pub(crate) mod bytecode_cache;
pub(crate) mod chunk_cache;
mod error;
pub(crate) mod helpers;
pub(crate) mod helpers_anim;
mod load_addon_trace;
pub(crate) mod lua_file;
pub(crate) mod precompiled;
mod xml_file;
mod xml_fontstring;
mod xml_frame;
mod xml_frame_codegen;
pub(crate) mod xml_frame_extras;
pub(crate) mod xml_layer_batch;
mod xml_lifecycle;
mod xml_texture;

use crate::lua_api::LoaderEnv;
use crate::saved_variables::SavedVariablesManager;
use crate::screen::ScreenKind;
use crate::toc::TocFile;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::Duration;

pub use error::LoadError;
pub(crate) use load_addon_trace::{
    LoadAddonTraceOrigin, enter_xml_load_addon_context, runtime_load_addon_origin, trace_load_addon,
};
pub use xml_frame::create_frame_from_xml;
pub use xml_frame::{fast_create_frame_profile_body_report, fast_create_frame_profile_report};

/// Find the TOC file for an addon directory.
/// Prefers Mainline variant, then exact name match, then any non-Classic TOC.
pub fn find_toc_file(addon_dir: &Path) -> Option<PathBuf> {
    let addon_name = addon_dir.file_name()?.to_str()?;
    let toc_variants = [
        format!("{}_Mainline.toc", addon_name),
        format!("{}.toc", addon_name),
    ];
    for variant in &toc_variants {
        let toc_path = addon_dir.join(variant);
        if toc_path.exists() {
            return Some(toc_path);
        }
    }
    // Fallback: find any .toc file (skip Classic/TBC/etc.)
    if let Ok(entries) = std::fs::read_dir(addon_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "toc").unwrap_or(false) {
                let name = path.file_name().unwrap().to_str().unwrap();
                if !name.contains("_Cata")
                    && !name.contains("_Wrath")
                    && !name.contains("_TBC")
                    && !name.contains("_Vanilla")
                    && !name.contains("_Mists")
                {
                    return Some(path);
                }
            }
        }
    }
    None
}

/// Result of loading an addon.
#[derive(Debug)]
pub struct LoadResult {
    /// Addon name
    pub name: String,
    /// Number of Lua files loaded
    pub lua_files: usize,
    /// Number of XML files loaded
    pub xml_files: usize,
    /// Time breakdown
    pub timing: LoadTiming,
    /// Errors encountered (non-fatal)
    pub warnings: Vec<String>,
}

/// Timing breakdown for addon loading.
#[derive(Debug, Default, Clone)]
pub struct LoadTiming {
    /// Time reading files from disk
    pub io_time: Duration,
    /// Time parsing XML
    pub xml_parse_time: Duration,
    /// Time processing parsed XML elements (excludes raw parse time)
    pub xml_process_time: Duration,
    /// Time creating/configuring frames from XML
    pub xml_frame_create_time: Duration,
    /// Time in the initial frame creation/setup phase (subset of xml_frame_create_time)
    pub xml_frame_setup_time: Duration,
    /// Time in child creation/finalization (subset of xml_frame_create_time)
    pub xml_frame_finalize_time: Duration,
    /// Time executing CreateFrame Lua code (subset of setup)
    pub frame_exec_lua_time: Duration,
    /// Time applying XML properties in Rust (subset of setup)
    pub frame_apply_props_time: Duration,
    /// Time creating layer children (textures/fontstrings, subset of finalize)
    pub frame_layer_children_time: Duration,
    /// Time firing OnLoad/OnShow lifecycle scripts (subset of finalize)
    pub frame_lifecycle_time: Duration,
    /// Number of frames created
    pub frame_count: u32,
    /// Number of OnLoad/OnShow fires
    pub lifecycle_fire_count: u32,
    /// Number of textures created
    pub texture_count: u32,
    /// Number of fontstrings created
    pub fontstring_count: u32,
    /// Time building Lua code strings (template chain, mixins, etc., subset of setup)
    pub frame_code_build_time: Duration,
    /// Time in animation groups (subset of finalize)
    pub frame_anim_time: Duration,
    /// Time in button textures+text (subset of finalize)
    pub frame_button_time: Duration,
    /// Time compiling Lua chunks into functions (source compile or bytecode load)
    pub lua_compile_time: Duration,
    /// Time preparing and calling compiled Lua functions
    pub lua_call_time: Duration,
    /// Time executing Lua (compile + call)
    pub lua_exec_time: Duration,
    /// Time loading SavedVariables
    pub saved_vars_time: Duration,
    /// Number of Lua files loaded from bytecode cache
    pub cache_hits: u32,
    /// Number of Lua files compiled from source (cache miss)
    pub cache_misses: u32,
}

impl LoadTiming {
    pub fn total(&self) -> Duration {
        self.io_time
            + self.xml_parse_time
            + self.xml_process_time
            + self.lua_exec_time
            + self.saved_vars_time
    }

    /// Add another timing's fields into this one.
    pub fn accumulate(&mut self, other: &LoadTiming) {
        self.io_time += other.io_time;
        self.xml_parse_time += other.xml_parse_time;
        self.xml_process_time += other.xml_process_time;
        self.xml_frame_create_time += other.xml_frame_create_time;
        self.xml_frame_setup_time += other.xml_frame_setup_time;
        self.xml_frame_finalize_time += other.xml_frame_finalize_time;
        self.frame_exec_lua_time += other.frame_exec_lua_time;
        self.frame_apply_props_time += other.frame_apply_props_time;
        self.frame_layer_children_time += other.frame_layer_children_time;
        self.frame_lifecycle_time += other.frame_lifecycle_time;
        self.frame_count += other.frame_count;
        self.lifecycle_fire_count += other.lifecycle_fire_count;
        self.texture_count += other.texture_count;
        self.fontstring_count += other.fontstring_count;
        self.frame_code_build_time += other.frame_code_build_time;
        self.frame_anim_time += other.frame_anim_time;
        self.frame_button_time += other.frame_button_time;
        self.lua_compile_time += other.lua_compile_time;
        self.lua_call_time += other.lua_call_time;
        self.lua_exec_time += other.lua_exec_time;
        self.saved_vars_time += other.saved_vars_time;
    }
}

/// Load an addon from its TOC file.
pub fn load_addon(env: &LoaderEnv<'_>, toc_path: &Path) -> Result<LoadResult, LoadError> {
    load_addon_path(env, toc_path, None)
}

/// Load an addon from its TOC file with saved variables support.
pub fn load_addon_with_saved_vars(
    env: &LoaderEnv<'_>,
    toc_path: &Path,
    saved_vars_mgr: &mut SavedVariablesManager,
) -> Result<LoadResult, LoadError> {
    load_addon_path(env, toc_path, Some(saved_vars_mgr))
}

fn load_addon_path(
    env: &LoaderEnv<'_>,
    toc_path: &Path,
    saved_vars_mgr: Option<&mut SavedVariablesManager>,
) -> Result<LoadResult, LoadError> {
    let addon_name = toc_path
        .parent()
        .and_then(|dir| dir.file_name())
        .and_then(|name| name.to_str())
        .unwrap_or("Unknown");
    trace_load_addon(LoadAddonTraceOrigin::Toc, format!("begin {addon_name}"));
    trace_load_addon(
        LoadAddonTraceOrigin::Toc,
        format!("toc {}", toc_path.display()),
    );
    let toc = TocFile::from_file(toc_path)?;
    trace_load_addon(LoadAddonTraceOrigin::Toc, format!("files {addon_name}"));
    let result = addon::load_addon_internal(env, &toc, saved_vars_mgr)?;
    for warning in &result.warnings {
        trace_load_addon(
            LoadAddonTraceOrigin::Toc,
            format!("warning {addon_name}: {warning}"),
        );
    }
    trace_load_addon(LoadAddonTraceOrigin::Toc, format!("loaded {addon_name}"));
    Ok(result)
}

/// Load an addon from a parsed TOC.
pub fn load_addon_from_toc(env: &LoaderEnv<'_>, toc: &TocFile) -> Result<LoadResult, LoadError> {
    addon::load_addon_internal(env, toc, None)
}

/// Load an addon from a parsed TOC with saved variables support.
pub fn load_addon_from_toc_with_saved_vars(
    env: &LoaderEnv<'_>,
    toc: &TocFile,
    saved_vars_mgr: &mut SavedVariablesManager,
) -> Result<LoadResult, LoadError> {
    addon::load_addon_internal(env, toc, Some(saved_vars_mgr))
}

/// Sort a list of `(name, toc_path)` pairs by their `## Dependencies:` / `## OptionalDeps:`.
///
/// Addons whose dependencies aren't in the list are treated as having no deps (they load early).
/// Ties are broken alphabetically for deterministic output.
pub fn sort_addons_by_dependencies(addons: &mut Vec<(String, PathBuf)>) {
    // Parse TOC files to build dependency info
    let mut toc_map: HashMap<String, (PathBuf, TocFile)> = HashMap::new();
    for (name, toc_path) in addons.iter() {
        if let Ok(toc) = TocFile::from_file(toc_path) {
            toc_map.insert(name.clone(), (toc_path.clone(), toc));
        }
    }

    let available: HashSet<&str> = toc_map.keys().map(|s| s.as_str()).collect();
    let deps = build_dependency_graph(&toc_map, &available);
    let load_first = build_load_first_set(&toc_map);
    let sorted = kahns_sort(&deps, toc_map.len(), &load_first);

    // Rebuild the vec in sorted order, appending any addons not in the graph at the end
    let name_to_path: HashMap<&str, &PathBuf> =
        addons.iter().map(|(n, p)| (n.as_str(), p)).collect();
    let mut result: Vec<(String, PathBuf)> = sorted
        .iter()
        .filter_map(|&name| {
            name_to_path
                .get(name)
                .map(|&p| (name.to_string(), p.clone()))
        })
        .collect();
    // Append addons that weren't in the toc_map (failed to parse)
    for (name, path) in addons.iter() {
        if !toc_map.contains_key(name) {
            result.push((name.clone(), path.clone()));
        }
    }
    *addons = result;
}

/// Discover all Blizzard addons in a BlizzardUI directory, topologically sorted by dependencies.
///
/// Scans for `Blizzard_*` subdirectories, parses their TOC files, filters out `LoadOnDemand`
/// addons (unless required by a non-LOD addon), and returns them in dependency order.
pub fn discover_blizzard_addons(blizzard_ui_dir: &Path) -> Vec<(String, PathBuf)> {
    discover_blizzard_addons_for_screen(blizzard_ui_dir, ScreenKind::Game)
}

/// Discover every Blizzard addon directory in a BlizzardUI tree, including LoadOnDemand addons.
///
/// This is stricter than `discover_blizzard_addons_for_screen`: it includes all 315
/// `Blizzard_*` directories present in the checkout, regardless of screen restrictions
/// or `LoadOnDemand`, then sorts them by dependencies.
pub fn discover_all_blizzard_addons(blizzard_ui_dir: &Path) -> Vec<(String, PathBuf)> {
    let entries = match std::fs::read_dir(blizzard_ui_dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    let mut addons = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !name.starts_with("Blizzard_") {
            continue;
        }
        if let Some(toc_path) = find_toc_file(&path) {
            addons.push((name.to_string(), toc_path));
        }
    }

    sort_addons_by_dependencies(&mut addons);
    addons
}

/// Discover Blizzard addons for a specific screen mode, topologically sorted by dependencies.
pub fn discover_blizzard_addons_for_screen(
    blizzard_ui_dir: &Path,
    screen: ScreenKind,
) -> Vec<(String, PathBuf)> {
    let Some((mut addons, mut lod_pool)) =
        discover_blizzard_addon_toc_pools_for_screen(blizzard_ui_dir, screen)
    else {
        return Vec::new();
    };

    // Pull LOD addons that are required by non-LOD addons
    pull_required_lod_addons(&mut addons, &mut lod_pool);

    topological_sort_addons(addons)
}

/// Discover the explicit dependency closure for one or more Blizzard addons.
///
/// The returned list is ordered the same way as `discover_blizzard_addons_for_screen`,
/// but filtered to the requested roots and everything they require via
/// `## Dependencies:` and `## OptionalDeps:`.
/// Roots or dependencies not present in the screen-allowed Blizzard TOC set are ignored.
pub fn discover_blizzard_addon_closure_for_screen(
    blizzard_ui_dir: &Path,
    screen: ScreenKind,
    roots: &[&str],
) -> Vec<(String, PathBuf)> {
    let Some((addons, lod_pool)) =
        discover_blizzard_addon_toc_pools_for_screen(blizzard_ui_dir, screen)
    else {
        return Vec::new();
    };
    let toc_map: HashMap<String, (PathBuf, TocFile)> = addons.into_iter().chain(lod_pool).collect();
    let wanted = collect_declared_dependency_closure(&toc_map, roots);
    let filtered: HashMap<String, (PathBuf, TocFile)> = toc_map
        .into_iter()
        .filter(|(name, _)| wanted.contains(name))
        .collect();
    topological_sort_addons(filtered)
}

/// Per-addon override entry for Blizzard addon closure discovery.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlizzardAddonOverride<'a> {
    pub addon: &'a str,
    pub extra_roots: &'a [&'a str],
}

/// Discover the explicit dependency closure for one or more Blizzard addons.
///
/// `overrides` adds extra non-TOC roots before the dependency walk runs.
/// Use it for shared templates, startup-order assumptions, and implicit
/// addons that Blizzard code reaches outside TOC metadata.
pub fn discover_blizzard_addon_closure_for_screen_with_overrides(
    blizzard_ui_dir: &Path,
    screen: ScreenKind,
    roots: &[&str],
    overrides: &[BlizzardAddonOverride<'_>],
) -> Vec<(String, PathBuf)> {
    let Some((addons, lod_pool)) =
        discover_blizzard_addon_toc_pools_for_screen(blizzard_ui_dir, screen)
    else {
        return Vec::new();
    };
    let toc_map: HashMap<String, (PathBuf, TocFile)> = addons.into_iter().chain(lod_pool).collect();
    let extra_roots_by_addon = build_extra_roots_map(overrides);
    let wanted =
        collect_declared_dependency_closure_with_overrides(&toc_map, roots, &extra_roots_by_addon);
    let extra_dependencies = build_extra_dependency_map(overrides);
    let filtered: HashMap<String, (PathBuf, TocFile)> = toc_map
        .into_iter()
        .filter(|(name, _)| wanted.contains(name))
        .collect();
    topological_sort_addons_with_extra_dependencies(filtered, &extra_dependencies)
}

pub(crate) fn collect_declared_dependency_closure(
    toc_map: &HashMap<String, (PathBuf, TocFile)>,
    roots: &[&str],
) -> HashSet<String> {
    let extra_roots_by_addon: HashMap<&str, Vec<&str>> = HashMap::new();
    collect_declared_dependency_closure_with_overrides(toc_map, roots, &extra_roots_by_addon)
}

fn collect_declared_dependency_closure_with_overrides(
    toc_map: &HashMap<String, (PathBuf, TocFile)>,
    roots: &[&str],
    extra_roots_by_addon: &HashMap<&str, Vec<&str>>,
) -> HashSet<String> {
    let mut wanted = HashSet::new();
    let mut pending: Vec<String> = roots.iter().map(|name| (*name).to_string()).collect();
    let mut queued: HashSet<String> = pending.iter().cloned().collect();

    while let Some(name) = pending.pop() {
        if !wanted.insert(name.clone()) {
            continue;
        }

        if let Some(extra_roots) = extra_roots_by_addon.get(name.as_str()) {
            for extra_root in extra_roots {
                queue_pending_addon((*extra_root).to_string(), &mut pending, &mut queued);
            }
        }

        let Some((_, toc)) = toc_map.get(&name) else {
            continue;
        };

        for dep in toc.dependencies().into_iter().chain(toc.optional_deps()) {
            if toc_map.contains_key(&dep) {
                queue_pending_addon(dep, &mut pending, &mut queued);
            }
        }
    }

    wanted
}

fn queue_pending_addon(name: String, pending: &mut Vec<String>, queued: &mut HashSet<String>) {
    if queued.insert(name.clone()) {
        pending.push(name);
    }
}

fn build_extra_roots_map<'a>(
    overrides: &'a [BlizzardAddonOverride<'a>],
) -> HashMap<&'a str, Vec<&'a str>> {
    let mut map: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
    for override_entry in overrides {
        map.entry(override_entry.addon)
            .or_default()
            .extend(override_entry.extra_roots.iter().copied());
    }
    map
}

fn build_extra_dependency_map(
    overrides: &[BlizzardAddonOverride<'_>],
) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for override_entry in overrides {
        map.entry(override_entry.addon.to_string())
            .or_default()
            .extend(
                override_entry
                    .extra_roots
                    .iter()
                    .map(|extra_root| (*extra_root).to_string()),
            );
    }
    map
}

fn discover_blizzard_addon_toc_pools_for_screen(
    blizzard_ui_dir: &Path,
    screen: ScreenKind,
) -> Option<(
    HashMap<String, (PathBuf, TocFile)>,
    HashMap<String, (PathBuf, TocFile)>,
)> {
    let entries = std::fs::read_dir(blizzard_ui_dir).ok()?;
    let mut addons: HashMap<String, (PathBuf, TocFile)> = HashMap::new();
    let mut lod_pool: HashMap<String, (PathBuf, TocFile)> = HashMap::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let dir_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if !dir_name.starts_with("Blizzard_")
            || excluded_addons_for_screen(screen).contains(&dir_name.as_str())
        {
            continue;
        }
        let Some(toc_path) = find_toc_file(&path) else {
            continue;
        };
        let Ok(toc) = TocFile::from_file(&toc_path) else {
            continue;
        };
        if !toc.allows_screen(screen) || toc.is_ptr_only() || toc.is_game_type_restricted() {
            continue;
        }
        let pool = if toc.is_load_on_demand() {
            &mut lod_pool
        } else {
            &mut addons
        };
        pool.insert(dir_name, (toc_path, toc));
    }

    Some((addons, lod_pool))
}

fn excluded_addons_for_screen(screen: ScreenKind) -> &'static [&'static str] {
    match screen {
        ScreenKind::Game | ScreenKind::CharacterSelect | ScreenKind::CharacterCreate => &[],
        ScreenKind::Login => &[
            "Blizzard_CharacterCreate",
            "Blizzard_CharacterCustomize",
            "Blizzard_TimerunningCharacterCreate",
        ],
    }
}

/// Recursively pull LoadOnDemand addons into the main set when required by loaded addons.
fn pull_required_lod_addons(
    addons: &mut HashMap<String, (PathBuf, TocFile)>,
    lod_pool: &mut HashMap<String, (PathBuf, TocFile)>,
) {
    let mut needed: Vec<String> = addons
        .values()
        .flat_map(|(_, toc)| toc.dependencies())
        .filter(|dep| lod_pool.contains_key(dep))
        .collect();

    while let Some(name) = needed.pop() {
        if addons.contains_key(&name) {
            continue;
        }
        if let Some((toc_path, toc)) = lod_pool.remove(&name) {
            // This LOD addon may itself depend on other LOD addons
            for dep in toc.dependencies() {
                if lod_pool.contains_key(&dep) {
                    needed.push(dep);
                }
            }
            addons.insert(name, (toc_path, toc));
        }
    }
}

/// Order Blizzard addons using a two-pass eager load model.
///
/// First pass eagerly emits addons marked `LoadFirst` or `UseSecureEnvironment`,
/// recursively pulling in any declared dependencies first. Second pass emits the
/// remaining addons the same way. This matches wowless's "load first pass, then
/// load the rest" behavior more closely than treating `LoadFirst` as a mere sort
/// tiebreaker.
///
/// After emitting each addon, any addon with `LoadWith` pointing to it is emitted
/// immediately (matching WoW's inline load-on-trigger behavior).
fn topological_sort_addons(addons: HashMap<String, (PathBuf, TocFile)>) -> Vec<(String, PathBuf)> {
    let extra_dependencies = HashMap::new();
    topological_sort_addons_with_extra_dependencies(addons, &extra_dependencies)
}

fn topological_sort_addons_with_extra_dependencies(
    mut addons: HashMap<String, (PathBuf, TocFile)>,
    extra_dependencies: &HashMap<String, Vec<String>>,
) -> Vec<(String, PathBuf)> {
    let load_with_map = build_load_with_map(&addons);
    let mut result = Vec::with_capacity(addons.len());
    let mut loaded: HashSet<String> = HashSet::new();
    let mut visiting: HashSet<String> = HashSet::new();
    let ctx = (
        &load_with_map,
        extra_dependencies,
        &mut result,
        &mut loaded,
        &mut visiting,
    );

    emit_early_addons(&mut addons, ctx.0, ctx.1, ctx.2, ctx.3, ctx.4);
    emit_remaining_addons(&mut addons, ctx.0, ctx.1, ctx.2, ctx.3, ctx.4);
    result
}

fn emit_early_addons(
    addons: &mut HashMap<String, (PathBuf, TocFile)>,
    load_with_map: &HashMap<String, Vec<String>>,
    extra_dependencies: &HashMap<String, Vec<String>>,
    result: &mut Vec<(String, PathBuf)>,
    loaded: &mut HashSet<String>,
    visiting: &mut HashSet<String>,
) {
    let mut early: Vec<String> = addons
        .iter()
        .filter_map(|(name, (_, toc))| {
            (toc.is_load_first() || toc.is_secure_env()).then_some(name.clone())
        })
        .collect();
    early.sort();
    for name in early {
        emit_addon_recursive(
            &name,
            addons,
            load_with_map,
            extra_dependencies,
            result,
            loaded,
            visiting,
        );
    }
}

fn emit_remaining_addons(
    addons: &mut HashMap<String, (PathBuf, TocFile)>,
    load_with_map: &HashMap<String, Vec<String>>,
    extra_dependencies: &HashMap<String, Vec<String>>,
    result: &mut Vec<(String, PathBuf)>,
    loaded: &mut HashSet<String>,
    visiting: &mut HashSet<String>,
) {
    let mut remaining: Vec<String> = addons.keys().cloned().collect();
    remaining.sort();
    for name in remaining {
        emit_addon_recursive(
            &name,
            addons,
            load_with_map,
            extra_dependencies,
            result,
            loaded,
            visiting,
        );
    }
}

fn emit_addon_recursive(
    name: &str,
    addons: &mut HashMap<String, (PathBuf, TocFile)>,
    load_with_map: &HashMap<String, Vec<String>>,
    extra_dependencies: &HashMap<String, Vec<String>>,
    result: &mut Vec<(String, PathBuf)>,
    loaded: &mut HashSet<String>,
    visiting: &mut HashSet<String>,
) {
    if loaded.contains(name) || !addons.contains_key(name) {
        return;
    }
    if !visiting.insert(name.to_string()) {
        return;
    }

    let deps = collect_emit_dependencies(name, addons, extra_dependencies);

    for dep in deps {
        emit_addon_recursive(
            &dep,
            addons,
            load_with_map,
            extra_dependencies,
            result,
            loaded,
            visiting,
        );
    }

    visiting.remove(name);

    if let Some((toc_path, _)) = addons.remove(name) {
        result.push((name.to_string(), toc_path));
        loaded.insert(name.to_string());
        emit_load_with(name, load_with_map, addons, result, loaded);
    }
}

fn collect_emit_dependencies(
    name: &str,
    addons: &HashMap<String, (PathBuf, TocFile)>,
    extra_dependencies: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let Some((_, toc)) = addons.get(name) else {
        return Vec::new();
    };

    let mut seen = HashSet::new();
    let mut deps = Vec::new();
    append_emit_dependencies(&mut deps, &mut seen, toc.dependencies(), addons);
    append_emit_dependencies(&mut deps, &mut seen, toc.optional_deps(), addons);
    append_emit_dependencies(
        &mut deps,
        &mut seen,
        extra_dependencies.get(name).cloned().unwrap_or_default(),
        addons,
    );
    deps
}

fn append_emit_dependencies(
    deps: &mut Vec<String>,
    seen: &mut HashSet<String>,
    candidates: Vec<String>,
    addons: &HashMap<String, (PathBuf, TocFile)>,
) {
    for dep in candidates {
        if addons.contains_key(&dep) && seen.insert(dep.clone()) {
            deps.push(dep);
        }
    }
}

/// Build reverse index: for each addon name, which addons have `LoadWith` pointing to it.
fn build_load_with_map(
    addons: &HashMap<String, (PathBuf, TocFile)>,
) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for (name, (_, toc)) in addons {
        for trigger in toc.load_with() {
            map.entry(trigger).or_default().push(name.clone());
        }
    }
    // Sort each list for deterministic order
    for list in map.values_mut() {
        list.sort();
    }
    map
}

/// After emitting an addon, emit any addons with `LoadWith` pointing to it.
/// Recurses to handle chained LoadWith triggers.
fn emit_load_with(
    just_loaded: &str,
    load_with_map: &HashMap<String, Vec<String>>,
    addons: &mut HashMap<String, (PathBuf, TocFile)>,
    result: &mut Vec<(String, PathBuf)>,
    loaded: &mut std::collections::HashSet<String>,
) {
    let Some(triggered) = load_with_map.get(just_loaded) else {
        return;
    };
    for name in triggered.clone() {
        if loaded.contains(&name) {
            continue;
        }
        if let Some((toc_path, _)) = addons.remove(&name) {
            result.push((name.clone(), toc_path));
            loaded.insert(name.clone());
            // Recurse: this addon may trigger further LoadWith addons
            emit_load_with(&name, load_with_map, addons, result, loaded);
        }
    }
}

/// Build a map of addon name -> list of available addon names it depends on.
/// Includes both required and optional dependencies (WoW loads optional deps
/// before the addon if they are present).
fn build_dependency_graph<'a>(
    addons: &'a HashMap<String, (PathBuf, TocFile)>,
    available: &std::collections::HashSet<&'a str>,
) -> HashMap<&'a str, Vec<&'a str>> {
    addons
        .iter()
        .map(|(name, (_, toc))| {
            let mut seen = HashSet::new();
            let mut deps: Vec<&str> = Vec::new();
            for dep in toc
                .dependencies()
                .iter()
                .filter_map(|d| available.get(d.as_str()).copied())
            {
                if seen.insert(dep) {
                    deps.push(dep);
                }
            }
            for d in toc.optional_deps() {
                if let Some(&dep) = available.get(d.as_str())
                    && seen.insert(dep)
                {
                    deps.push(dep);
                }
            }
            (name.as_str(), deps)
        })
        .collect()
}

fn build_load_first_set<'a>(addons: &'a HashMap<String, (PathBuf, TocFile)>) -> HashSet<&'a str> {
    addons
        .iter()
        .filter_map(|(name, (_, toc))| toc.is_load_first().then_some(name.as_str()))
        .collect()
}

fn addon_priority_cmp(a: &str, b: &str, load_first: &HashSet<&str>) -> Ordering {
    match (load_first.contains(a), load_first.contains(b)) {
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        _ => b.cmp(a),
    }
}

fn insert_by_priority<'a>(queue: &mut Vec<&'a str>, name: &'a str, load_first: &HashSet<&'a str>) {
    let pos = queue.partition_point(|&existing| {
        addon_priority_cmp(existing, name, load_first) == Ordering::Less
    });
    queue.insert(pos, name);
}

/// Run Kahn's algorithm on a dependency graph. Returns names in topological order.
/// Ties are broken by `LoadFirst`, then alphabetically. If the remaining graph
/// contains a cycle, we still emit every addon by breaking the cycle using the
/// same priority order.
fn kahns_sort<'a>(
    deps: &HashMap<&'a str, Vec<&'a str>>,
    count: usize,
    load_first: &HashSet<&'a str>,
) -> Vec<&'a str> {
    let mut in_degree: HashMap<&str, usize> = deps.keys().map(|&n| (n, 0)).collect();
    let mut dependents: HashMap<&str, Vec<&str>> = HashMap::new();
    for (&node, reqs) in deps {
        *in_degree.entry(node).or_default() = reqs.len();
        for &r in reqs {
            dependents.entry(r).or_default().push(node);
        }
    }

    let mut queue = build_zero_degree_queue(&in_degree, load_first);

    let mut result = Vec::with_capacity(count);
    let mut emitted: HashSet<&str> = HashSet::new();
    while result.len() < count {
        let Some(name) = next_kahn_node(&mut queue, &in_degree, &emitted, load_first) else {
            break;
        };

        if !emitted.insert(name) {
            continue;
        }
        result.push(name);
        for &dep in dependents.get(name).unwrap_or(&Vec::new()) {
            if let Some(deg) = in_degree.get_mut(dep) {
                *deg = deg.saturating_sub(1);
                if *deg == 0 {
                    insert_by_priority(&mut queue, dep, load_first);
                }
            }
        }
    }

    result
}

fn build_zero_degree_queue<'a>(
    in_degree: &HashMap<&'a str, usize>,
    load_first: &HashSet<&'a str>,
) -> Vec<&'a str> {
    // Seed queue with zero-dependency addons, sorted descending (pop takes last = smallest)
    let mut queue: Vec<&str> = in_degree
        .iter()
        .filter(|&(_, deg)| *deg == 0)
        .map(|(&name, _)| name)
        .collect();
    queue.sort_by(|a, b| addon_priority_cmp(a, b, load_first));
    queue
}

fn next_kahn_node<'a>(
    queue: &mut Vec<&'a str>,
    in_degree: &HashMap<&'a str, usize>,
    emitted: &HashSet<&'a str>,
    load_first: &HashSet<&'a str>,
) -> Option<&'a str> {
    queue.pop().or_else(|| {
        in_degree
            .keys()
            .filter(|name| !emitted.contains(**name))
            .max_by(|a, b| addon_priority_cmp(a, b, load_first))
            .copied()
    })
}

#[cfg(test)]
mod tests;
