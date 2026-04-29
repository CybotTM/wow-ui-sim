//! Blizzard and third-party addon loading with timing/summary.

use std::path::{Path, PathBuf};
use wow_ui_sim::client_profile;
use wow_ui_sim::loader::{
    LoadResult, LoadTiming, discover_blizzard_addons_for_screen, load_addon,
    load_addon_with_saved_vars,
};
use wow_ui_sim::logging;
use wow_ui_sim::lua_api::{AddonInfo, WowLuaEnv};
use wow_ui_sim::saved_variables::SavedVariablesManager;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

pub const USER_ADDONS_PATH: &str = "./Interface/AddOns";
pub const TEST_ADDONS_PATH: &str = "./Interface/TestAddOns";

/// Addon names that are test-only and should not be loaded in GUI mode.
pub const TEST_ADDONS: &[&str] = &["Wowless", "WowlessData", "WowBehaviorTest", "WowDiscovery"];

/// Load Blizzard SharedXML and base UI addons (auto-discovered, dependency-sorted).
pub fn load_blizzard_addons(env: &WowLuaEnv, screen: ScreenKind) {
    let profile = client_profile::ACTIVE;
    let addons_dir = client_profile::blizzard_ui_addons_dir();
    if !addons_dir.exists() {
        eprintln!(
            "FATAL: {} is missing: Blizzard UI source not set up for {:?} profile.\n\
             Run from the repo root:\n\
             \t./scripts/setup-blizzard-ui.sh {}",
            addons_dir.display(),
            profile,
            profile.subdir().to_lowercase(),
        );
        std::process::exit(1);
    }

    let verbose = std::env::var("WOW_SIM_VERBOSE").is_ok();
    let blizzard_start = std::time::Instant::now();
    let mut total_timing = LoadTiming::default();

    // Stop GC during bulk loading — collect once at the end instead of
    // incremental sweeps on every allocation.
    env.gc_stop();

    // Wrath ships a top-level FrameXML.toc that pre-dates the retail Blizzard_*
    // split. Load it as a synthetic addon before iterating Blizzard_* dirs so
    // global UI templates (UIParent, GameTooltip, etc.) register first.
    if let Some(framexml_toc) = client_profile::blizzard_ui_framexml_toc() {
        load_one_blizzard_addon(env, "FrameXML", &framexml_toc, verbose, &mut total_timing);
    }

    let addons = discover_blizzard_addons_for_screen(&addons_dir, screen);
    logging::println_elapsed(&format!("Loading {} Blizzard addons...", addons.len()));

    for (name, toc_path) in &addons {
        load_one_blizzard_addon(env, name, toc_path, verbose, &mut total_timing);
        if name == "Blizzard_EnvironmentCleanup" {
            env.restore_post_cleanup_globals();
        }
    }

    let gc_start = std::time::Instant::now();
    env.gc_restart();
    env.gc_collect();
    let gc_dur = gc_start.elapsed();

    print_blizzard_summary(blizzard_start.elapsed(), &total_timing, gc_dur);
}

fn load_one_blizzard_addon(
    env: &WowLuaEnv,
    name: &str,
    toc_path: &Path,
    verbose: bool,
    timing: &mut LoadTiming,
) {
    match load_addon(&env.loader_env(), toc_path) {
        Ok(r) => record_blizzard_addon_success(env, name, verbose, timing, r),
        Err(e) => println!("{} failed: {}", name, e),
    }
}

fn record_blizzard_addon_success(
    env: &WowLuaEnv,
    name: &str,
    verbose: bool,
    timing: &mut LoadTiming,
    result: LoadResult,
) {
    print_verbose_blizzard_status(name, verbose, &result);
    print_nil_global_warnings(&result);
    fire_addon_loaded(env, name);
    timing.accumulate(&result.timing);
}

fn print_verbose_blizzard_status(name: &str, verbose: bool, result: &LoadResult) {
    if !verbose {
        return;
    }
    let t = &result.timing;
    println!(
        "{} loaded: {} Lua, {} XML, {} warnings ({:.1?}: xmlproc={:.1?} exec_lua={:.1?} lifecycle={:.1?} layers={:.1?} lua={:.1?} [compile={:.1?} call={:.1?}] frames={})",
        name,
        result.lua_files,
        result.xml_files,
        result.warnings.len(),
        t.total(),
        t.xml_process_time,
        t.frame_exec_lua_time,
        t.frame_lifecycle_time,
        t.frame_layer_children_time,
        t.lua_exec_time,
        t.lua_compile_time,
        t.lua_call_time,
        t.frame_count
    );
}

fn print_nil_global_warnings(result: &LoadResult) {
    if std::env::var("WOW_SIM_DEBUG_NIL_GLOBALS").is_err() {
        return;
    }
    for warning in &result.warnings {
        println!("  [!] {warning}");
    }
}

fn fire_addon_loaded(env: &WowLuaEnv, name: &str) {
    if let Err(e) = env.fire_event_with_args("ADDON_LOADED", &[env.lua_string(name)]) {
        logging::println_elapsed(&format!("Error firing ADDON_LOADED for {name}: {e}"));
    }
}

fn print_blizzard_frame_detail(t: &LoadTiming) {
    println!(
        "  frame breakdown: setup={:.2?} finalize={:.2?} ({} frames)",
        t.xml_frame_setup_time, t.xml_frame_finalize_time, t.frame_count
    );
    println!(
        "  setup: code_build={:.2?} exec_lua={:.2?} props={:.2?}",
        t.frame_code_build_time, t.frame_exec_lua_time, t.frame_apply_props_time
    );
    println!(
        "  finalize: layers={:.2?} ({} tex, {} fs) anim={:.2?} button={:.2?} lifecycle={:.2?} ({} fires)",
        t.frame_layer_children_time,
        t.texture_count,
        t.fontstring_count,
        t.frame_anim_time,
        t.frame_button_time,
        t.frame_lifecycle_time,
        t.lifecycle_fire_count
    );
}

fn print_blizzard_summary(
    elapsed: std::time::Duration,
    t: &LoadTiming,
    gc_dur: std::time::Duration,
) {
    let cache_total = t.cache_hits + t.cache_misses;
    let cache_info = if cache_total > 0 {
        format!(", bytecode cache: {}/{} hits", t.cache_hits, cache_total)
    } else {
        String::new()
    };
    logging::println_elapsed(&format!(
        "Blizzard addons loaded in {elapsed:.2?} (io={:.2?} xml={:.2?} xmlproc={:.2?} frames⊂xmlproc={:.2?} lua={:.2?} [compile={:.2?} call={:.2?}] gc={gc_dur:.2?}{cache_info})",
        t.io_time,
        t.xml_parse_time,
        t.xml_process_time,
        t.xml_frame_create_time,
        t.lua_exec_time,
        t.lua_compile_time,
        t.lua_call_time
    ));
    print_blizzard_frame_detail(t);
    if let Some(report) = wow_ui_sim::loader::fast_create_frame_profile_report() {
        println!("  {report}");
    }
    if let Some(report) = wow_ui_sim::loader::fast_create_frame_profile_body_report() {
        println!("  {report}");
    }
}

/// Scan, load, and register third-party addons; print summary.
pub fn load_third_party_addons(
    skip_addons: bool,
    is_test: bool,
    env: &WowLuaEnv,
    saved_vars: &mut Option<SavedVariablesManager>,
    screen: ScreenKind,
) {
    if skip_addons && !is_test {
        logging::println_elapsed("Addon loading disabled");
        return;
    }

    let user_addons_path = PathBuf::from(USER_ADDONS_PATH);
    let exclude = if is_test { &[][..] } else { TEST_ADDONS };
    let mut addons = scan_addons(&user_addons_path, exclude, screen);
    if is_test {
        let test_addons_path = PathBuf::from(TEST_ADDONS_PATH);
        addons.extend(scan_addons(&test_addons_path, &[], screen));
    }
    wow_ui_sim::loader::sort_addons_by_dependencies(&mut addons);
    if skip_addons {
        addons.retain(|(name, _)| TEST_ADDONS.iter().any(|t| t == name));
    }
    if addons.is_empty() {
        return;
    }

    logging::println_elapsed(&format!("Loading {} addons...", addons.len()));
    let mut stats = LoadStats::default();
    for (name, toc_path) in &addons {
        load_single_addon(env, name, toc_path, saved_vars, &mut stats);
    }
    print_load_summary(&addons, &stats);
}

pub fn scan_addons(
    base_path: &Path,
    exclude: &[&str],
    screen: ScreenKind,
) -> Vec<(String, PathBuf)> {
    let mut addons = Vec::new();
    let Ok(entries) = std::fs::read_dir(base_path) else {
        return addons;
    };
    for entry in entries.flatten() {
        if let Some(addon) = scanned_addon(entry.path(), exclude, screen) {
            addons.push(addon);
        }
    }
    addons
}

fn scanned_addon(path: PathBuf, exclude: &[&str], screen: ScreenKind) -> Option<(String, PathBuf)> {
    if !path.is_dir() {
        return None;
    }
    let name = path.file_name()?.to_str()?.to_string();
    if should_skip_addon_dir(&name, exclude) {
        return None;
    }
    let toc_path = loadable_toc_path(&path, screen)?;
    Some((name, toc_path))
}

fn should_skip_addon_dir(name: &str, exclude: &[&str]) -> bool {
    name.starts_with('.') || name == "BlizzardUI" || exclude.contains(&name)
}

fn loadable_toc_path(path: &Path, screen: ScreenKind) -> Option<PathBuf> {
    let toc_path = wow_ui_sim::loader::find_toc_file(path)?;
    let toc = TocFile::from_file(&toc_path).ok()?;
    let supports_screen = toc.allows_screen(screen);
    let supported_game_type = !toc.is_ptr_only() && !toc.is_game_type_restricted();
    (supports_screen && supported_game_type).then_some(toc_path)
}

/// Accumulated statistics from loading addons.
#[derive(Default)]
struct LoadStats {
    total_lua: usize,
    total_xml: usize,
    total_warnings: usize,
    total_timing: LoadTiming,
    success_count: usize,
    fail_count: usize,
    addon_times: Vec<(String, std::time::Duration)>,
    cache_hits: u32,
    cache_misses: u32,
}

fn parse_addon_metadata(name: &str, toc_path: &Path) -> (String, String, bool) {
    let toc = TocFile::from_file(toc_path).ok();
    toc.as_ref()
        .map(|t| {
            let title = t
                .metadata
                .get("Title")
                .cloned()
                .unwrap_or_else(|| name.to_string());
            let notes = t.metadata.get("Notes").cloned().unwrap_or_default();
            let lod = t
                .metadata
                .get("LoadOnDemand")
                .map(|v| v == "1")
                .unwrap_or(false);
            (title, notes, lod)
        })
        .unwrap_or_else(|| (name.to_string(), String::new(), false))
}

fn load_single_addon(
    env: &WowLuaEnv,
    name: &str,
    toc_path: &Path,
    saved_vars: &mut Option<SavedVariablesManager>,
    stats: &mut LoadStats,
) {
    let (title, notes, load_on_demand) = parse_addon_metadata(name, toc_path);
    env.register_addon(AddonInfo {
        folder_name: name.to_string(),
        title,
        notes,
        enabled: true,
        loaded: false,
        load_on_demand,
        ..Default::default()
    });

    let result = match saved_vars.as_mut() {
        Some(sv) => load_addon_with_saved_vars(&env.loader_env(), toc_path, sv),
        None => load_addon(&env.loader_env(), toc_path),
    };
    match result {
        Ok(r) => {
            mark_addon_loaded(env, name, &r);
            fire_addon_loaded(env, name);
            record_addon_success(name, &r, stats);
        }
        Err(e) => {
            println!("✗ {} failed: {}", name, e);
            stats.fail_count += 1;
        }
    }
}

fn mark_addon_loaded(env: &WowLuaEnv, name: &str, r: &LoadResult) {
    let t = r.timing.total().as_secs_f64();
    let mut s = env.state().borrow_mut();
    if let Some(a) = s.addons.iter_mut().find(|a| a.folder_name == name) {
        a.loaded = true;
        a.load_time_secs = t;
    }
}

fn record_addon_success(name: &str, r: &LoadResult, stats: &mut LoadStats) {
    print_verbose_addon_status(name, r);
    stats.addon_times.push((name.to_string(), r.timing.total()));
    print_addon_warnings(name, &r.warnings);
    stats.total_lua += r.lua_files;
    stats.total_xml += r.xml_files;
    stats.total_warnings += r.warnings.len();
    stats.total_timing.accumulate(&r.timing);
    stats.cache_hits += r.timing.cache_hits;
    stats.cache_misses += r.timing.cache_misses;
    stats.success_count += 1;
}

fn print_verbose_addon_status(name: &str, r: &LoadResult) {
    if std::env::var("WOW_SIM_VERBOSE").is_err() {
        return;
    }
    let status = if r.warnings.is_empty() { "✓" } else { "⚠" };
    let t = &r.timing;
    println!(
        "{} {} loaded: {} Lua, {} XML, {} warnings ({:.1?} total: io={:.1?} xml={:.1?} xmlproc={:.1?} frames⊂xmlproc={:.1?} setup⊂frames={:.1?} finalize⊂frames={:.1?} lua={:.1?} [compile={:.1?} call={:.1?}] sv={:.1?})",
        status,
        name,
        r.lua_files,
        r.xml_files,
        r.warnings.len(),
        t.total(),
        t.io_time,
        t.xml_parse_time,
        t.xml_process_time,
        t.xml_frame_create_time,
        t.xml_frame_setup_time,
        t.xml_frame_finalize_time,
        t.lua_exec_time,
        t.lua_compile_time,
        t.lua_call_time,
        t.saved_vars_time
    );
}

const VERBOSE_WARNING_ADDONS: &[&str] = &[
    "BetterWardrobe",
    "Plumber",
    "BetterBlizzFrames",
    "Baganator",
    "Angleur",
    "ExtraQuestButton",
    "WaypointUI",
    "TomTom",
    "WorldQuestTracker",
    "SavedInstances",
    "Rarity",
    "SimpleItemLevel",
    "TalentLoadoutManager",
    "Simulationcraft",
    "TomCats",
    "RaiderIO",
    "!BugGrabber",
    "CraftSim",
    "AdvancedInterfaceOptions",
    "BlizzMove_Debug",
    "ClickableRaidBuffs",
    "Dejunk",
    "Cell",
    "AngryKeystones",
    "AutoPotion",
    "BigWigs_Plugins",
    "BugSack",
    "Clicked",
    "DeathNote",
    "DeModal",
    "ElvUI_OptionsUI",
    "DragonRaceTimes",
    "DynamicCam",
    "DialogueUI",
    "Chattynator",
    "AstralKeys",
    "Leatrix_Plus",
    "CooldownToGo_Options",
    "HousingItemTracker",
    "idTip",
    "Macroriffic",
    "NameplateSCT",
    "Krowi_ExtendedVendorUI",
    "OmniCD",
    "Auctionator",
    "EditModeExpanded",
    "GlobalIgnoreList",
    "AllTheThings",
    "BigWigs_KhazAlgar",
    "LegionRemixHelper",
    "Collectionator",
    "Syndicator",
    "BigWigs",
    "!KalielsTracker",
    "KRaidSkipTracker",
    "MacroToolkit",
    "MinimapButtonButton",
    "OribosExchange",
];

fn print_addon_warnings(name: &str, warnings: &[String]) {
    if warnings.is_empty() || !VERBOSE_WARNING_ADDONS.contains(&name) {
        return;
    }
    for (i, w) in warnings.iter().take(10).enumerate() {
        println!("  [{}] {}", i + 1, w);
    }
    if warnings.len() > 10 {
        println!("  ... and {} more", warnings.len() - 10);
    }
}

fn print_load_summary(addons: &[(String, PathBuf)], stats: &LoadStats) {
    println!("\n=== Summary ===");
    println!("Loaded: {}/{} addons", stats.success_count, addons.len());
    println!("Failed: {}", stats.fail_count);
    println!(
        "Total: {} Lua files, {} XML files, {} warnings",
        stats.total_lua, stats.total_xml, stats.total_warnings
    );
    print_timing_breakdown(&stats.total_timing);
    print_cache_stats(stats.cache_hits, stats.cache_misses);
    print_slowest_addons(&stats.addon_times);
}

fn print_lua_timing_detail(t: &LoadTiming, pct: &dyn Fn(std::time::Duration) -> f64) {
    println!(
        "  Lua exec:   {:.2?} ({:.1}%)",
        t.lua_exec_time,
        pct(t.lua_exec_time)
    );
    println!(
        "    compile:  {:.2?} ({:.1}%, subset of Lua exec)",
        t.lua_compile_time,
        pct(t.lua_compile_time)
    );
    println!(
        "    call:     {:.2?} ({:.1}%, subset of Lua exec)",
        t.lua_call_time,
        pct(t.lua_call_time)
    );
    println!(
        "  SavedVars:  {:.2?} ({:.1}%)",
        t.saved_vars_time,
        pct(t.saved_vars_time)
    );
}

fn print_timing_breakdown(t: &LoadTiming) {
    let total_time = t.total();
    if total_time.is_zero() {
        return;
    }
    let pct = |d: std::time::Duration| 100.0 * d.as_secs_f64() / total_time.as_secs_f64();
    println!("Total time: {:.2?}", total_time);
    println!("  IO:         {:.2?} ({:.1}%)", t.io_time, pct(t.io_time));
    println!(
        "  XML parse:  {:.2?} ({:.1}%)",
        t.xml_parse_time,
        pct(t.xml_parse_time)
    );
    println!(
        "  XML proc:   {:.2?} ({:.1}%)",
        t.xml_process_time,
        pct(t.xml_process_time)
    );
    print_frame_timing_detail(t, &pct);
    print_lua_timing_detail(t, &pct);
}

fn print_frame_timing_detail(t: &LoadTiming, pct: &dyn Fn(std::time::Duration) -> f64) {
    println!(
        "  XML frames: {:.2?} ({:.1}%, subset of XML proc)",
        t.xml_frame_create_time,
        pct(t.xml_frame_create_time)
    );
    println!(
        "    setup:  {:.2?}  (code_build={:.2?} exec_lua={:.2?} props={:.2?})",
        t.xml_frame_setup_time,
        t.frame_code_build_time,
        t.frame_exec_lua_time,
        t.frame_apply_props_time
    );
    println!(
        "    finalize: {:.2?}  (layers={:.2?} anim={:.2?} button={:.2?} lifecycle={:.2?})",
        t.xml_frame_finalize_time,
        t.frame_layer_children_time,
        t.frame_anim_time,
        t.frame_button_time,
        t.frame_lifecycle_time
    );
    println!(
        "    {} frames, {} textures, {} fontstrings, {} lifecycle fires",
        t.frame_count, t.texture_count, t.fontstring_count, t.lifecycle_fire_count
    );
}

fn print_cache_stats(hits: u32, misses: u32) {
    if hits == 0 && misses == 0 {
        return;
    }
    let total = hits + misses;
    let pct = 100.0 * hits as f64 / total as f64;
    println!("Bytecode cache: {}/{} hits ({:.0}%)", hits, total, pct);
}

fn print_slowest_addons(addon_times: &[(String, std::time::Duration)]) {
    let mut sorted = addon_times.to_vec();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    println!("\nSlowest addons:");
    for (name, time) in sorted.iter().take(10) {
        println!("  {:>7.1?}  {}", time, name);
    }
}
