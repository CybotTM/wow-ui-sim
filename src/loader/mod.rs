//! Addon loader - loads addons from TOC files.

mod addon;
pub(crate) mod button;
pub(crate) mod bytecode_cache;
pub(crate) mod chunk_cache;
mod error;
pub(crate) mod helpers;
pub(crate) mod helpers_anim;
pub(crate) mod lua_file;
pub(crate) mod precompiled;
pub(crate) mod bytecode;
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
pub use xml_frame::create_frame_from_xml;

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
    let toc = TocFile::from_file(toc_path)?;
    load_addon_from_toc(env, &toc)
}

/// Load an addon from its TOC file with saved variables support.
pub fn load_addon_with_saved_vars(
    env: &LoaderEnv<'_>,
    toc_path: &Path,
    saved_vars_mgr: &mut SavedVariablesManager,
) -> Result<LoadResult, LoadError> {
    let toc = TocFile::from_file(toc_path)?;
    load_addon_from_toc_with_saved_vars(env, &toc, saved_vars_mgr)
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
    let entries = match std::fs::read_dir(blizzard_ui_dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    // Parse all addon TOCs into two pools: normal and load-on-demand
    let mut addons: HashMap<String, (PathBuf, TocFile)> = HashMap::new();
    let mut lod_pool: HashMap<String, (PathBuf, TocFile)> = HashMap::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let dir_name = path.file_name().unwrap().to_str().unwrap().to_string();
        if !dir_name.starts_with("Blizzard_") {
            continue;
        }
        if excluded_addons_for_screen(screen).contains(&dir_name.as_str()) {
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
        if toc.is_load_on_demand() {
            lod_pool.insert(dir_name, (toc_path, toc));
        } else {
            addons.insert(dir_name, (toc_path, toc));
        }
    }

    // Pull LOD addons that are required by non-LOD addons
    pull_required_lod_addons(&mut addons, &mut lod_pool);

    topological_sort_addons(addons)
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
fn topological_sort_addons(
    mut addons: HashMap<String, (PathBuf, TocFile)>,
) -> Vec<(String, PathBuf)> {
    let load_with_map = build_load_with_map(&addons);
    let mut result = Vec::with_capacity(addons.len());
    let mut loaded: HashSet<String> = HashSet::new();
    let mut visiting: HashSet<String> = HashSet::new();
    let ctx = (&load_with_map, &mut result, &mut loaded, &mut visiting);

    emit_early_addons(&mut addons, ctx.0, ctx.1, ctx.2, ctx.3);
    emit_remaining_addons(&mut addons, ctx.0, ctx.1, ctx.2, ctx.3);
    result
}

fn emit_early_addons(
    addons: &mut HashMap<String, (PathBuf, TocFile)>,
    load_with_map: &HashMap<String, Vec<String>>,
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
        emit_addon_recursive(&name, addons, load_with_map, result, loaded, visiting);
    }
}

fn emit_remaining_addons(
    addons: &mut HashMap<String, (PathBuf, TocFile)>,
    load_with_map: &HashMap<String, Vec<String>>,
    result: &mut Vec<(String, PathBuf)>,
    loaded: &mut HashSet<String>,
    visiting: &mut HashSet<String>,
) {
    let mut remaining: Vec<String> = addons.keys().cloned().collect();
    remaining.sort();
    for name in remaining {
        emit_addon_recursive(&name, addons, load_with_map, result, loaded, visiting);
    }
}

fn emit_addon_recursive(
    name: &str,
    addons: &mut HashMap<String, (PathBuf, TocFile)>,
    load_with_map: &HashMap<String, Vec<String>>,
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

    let deps = addons
        .get(name)
        .map(|(_, toc)| {
            let mut deps = toc.dependencies();
            for dep in toc.optional_deps() {
                if addons.contains_key(&dep) && !deps.contains(&dep) {
                    deps.push(dep);
                }
            }
            deps
        })
        .unwrap_or_default();

    for dep in deps {
        emit_addon_recursive(&dep, addons, load_with_map, result, loaded, visiting);
    }

    visiting.remove(name);

    if let Some((toc_path, _)) = addons.remove(name) {
        result.push((name.to_string(), toc_path));
        loaded.insert(name.to_string());
        emit_load_with(name, load_with_map, addons, result, loaded);
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
            let mut deps: Vec<&str> = toc
                .dependencies()
                .iter()
                .filter_map(|d| available.get(d.as_str()).copied())
                .collect();
            for d in toc.optional_deps() {
                if let Some(&dep) = available.get(d.as_str())
                    && !deps.contains(&dep)
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
