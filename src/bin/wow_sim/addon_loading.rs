//! Blizzard and third-party addon loading with timing/summary.

use std::path::{Path, PathBuf};
use wow_ui_sim::loader::{
    LoadResult, LoadTiming, discover_blizzard_addons_for_screen, load_addon,
    load_addon_with_saved_vars,
};
use wow_ui_sim::logging;
use wow_ui_sim::lua_api::{AddonInfo, WowLuaEnv};
use wow_ui_sim::saved_variables::SavedVariablesManager;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

/// Addon names that are test-only and should not be loaded in GUI mode.
pub const TEST_ADDONS: &[&str] = &["Wowless", "WowlessData"];

/// Load Blizzard SharedXML and base UI addons (auto-discovered, dependency-sorted).
pub fn load_blizzard_addons(env: &WowLuaEnv, screen: ScreenKind) {
    let blizzard_ui_path = PathBuf::from("./Interface/BlizzardUI");
    if !blizzard_ui_path.exists() {
        return;
    }

    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_path, screen);
    let verbose = std::env::var("WOW_SIM_VERBOSE").is_ok();
    logging::println_elapsed(&format!("Loading {} Blizzard addons...", addons.len()));
    let blizzard_start = std::time::Instant::now();
    let mut total_timing = LoadTiming::default();

    // Stop GC during bulk loading — collect once at the end instead of
    // incremental sweeps on every allocation.
    env.lua().gc_stop();

    for (name, toc_path) in &addons {
        load_one_blizzard_addon(env, name, toc_path, verbose, &mut total_timing);
        if name == "Blizzard_EnvironmentCleanup" {
            env.restore_post_cleanup_globals();
        }
    }

    let gc_start = std::time::Instant::now();
    env.lua().gc_restart();
    let _ = env.lua().gc_collect();
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
        Ok(r) => {
            if verbose {
                let t = &r.timing;
                println!(
                    "{} loaded: {} Lua, {} XML, {} warnings ({:.1?}: xmlproc={:.1?} exec_lua={:.1?} lifecycle={:.1?} layers={:.1?} lua={:.1?} [compile={:.1?} call={:.1?}] frames={})",
                    name,
                    r.lua_files,
                    r.xml_files,
                    r.warnings.len(),
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
            if std::env::var("WOW_SIM_DEBUG_NIL_GLOBALS").is_ok() {
                for w in &r.warnings {
                    println!("  [!] {}", w);
                }
            }
            timing.accumulate(&r.timing);
        }
        Err(e) => println!("{} failed: {}", name, e),
    }
}

fn print_blizzard_summary(
    elapsed: std::time::Duration,
    t: &LoadTiming,
    gc_dur: std::time::Duration,
) {
    logging::println_elapsed(&blizzard_summary_line(elapsed, t, gc_dur));
    println!("{}", blizzard_frame_breakdown_line(t));
    println!("{}", blizzard_setup_breakdown_line(t));
    println!("{}", blizzard_finalize_breakdown_line(t));
}

fn blizzard_summary_line(
    elapsed: std::time::Duration,
    timing: &LoadTiming,
    gc_dur: std::time::Duration,
) -> String {
    format!(
        "Blizzard addons loaded in {elapsed:.2?} (io={:.2?} xml={:.2?} xmlproc={:.2?} frames⊂xmlproc={:.2?} lua={:.2?} [compile={:.2?} call={:.2?}] gc={gc_dur:.2?}{})",
        timing.io_time,
        timing.xml_parse_time,
        timing.xml_process_time,
        timing.xml_frame_create_time,
        timing.lua_exec_time,
        timing.lua_compile_time,
        timing.lua_call_time,
        blizzard_cache_info(timing),
    )
}

fn blizzard_cache_info(timing: &LoadTiming) -> String {
    let cache_total = timing.cache_hits + timing.cache_misses;
    if cache_total > 0 {
        format!(
            ", bytecode cache: {}/{} hits",
            timing.cache_hits, cache_total
        )
    } else {
        String::new()
    }
}

fn blizzard_frame_breakdown_line(timing: &LoadTiming) -> String {
    format!(
        "  frame breakdown: setup={:.2?} finalize={:.2?} ({} frames)",
        timing.xml_frame_setup_time, timing.xml_frame_finalize_time, timing.frame_count
    )
}

fn blizzard_setup_breakdown_line(timing: &LoadTiming) -> String {
    format!(
        "  setup: code_build={:.2?} exec_lua={:.2?} props={:.2?}",
        timing.frame_code_build_time, timing.frame_exec_lua_time, timing.frame_apply_props_time
    )
}

fn blizzard_finalize_breakdown_line(timing: &LoadTiming) -> String {
    format!(
        "  finalize: layers={:.2?} ({} tex, {} fs) anim={:.2?} button={:.2?} lifecycle={:.2?} ({} fires)",
        timing.frame_layer_children_time,
        timing.texture_count,
        timing.fontstring_count,
        timing.frame_anim_time,
        timing.frame_button_time,
        timing.frame_lifecycle_time,
        timing.lifecycle_fire_count
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blizzard_cache_info_omits_empty_cache_stats() {
        assert!(blizzard_cache_info(&LoadTiming::default()).is_empty());
    }

    #[test]
    fn blizzard_summary_line_includes_cache_stats_when_present() {
        let timing = LoadTiming {
            cache_hits: 3,
            cache_misses: 1,
            ..LoadTiming::default()
        };

        let line = blizzard_summary_line(
            std::time::Duration::from_secs(2),
            &timing,
            std::time::Duration::from_millis(250),
        );

        assert!(line.contains("Blizzard addons loaded in"));
        assert!(line.contains("gc=250.00ms"));
        assert!(line.contains("bytecode cache: 3/4 hits"));
    }

    #[test]
    fn timing_breakdown_lines_include_frame_and_lua_details() {
        let timing = LoadTiming {
            io_time: std::time::Duration::from_millis(10),
            xml_parse_time: std::time::Duration::from_millis(20),
            xml_process_time: std::time::Duration::from_millis(30),
            xml_frame_create_time: std::time::Duration::from_millis(12),
            xml_frame_setup_time: std::time::Duration::from_millis(5),
            xml_frame_finalize_time: std::time::Duration::from_millis(7),
            frame_code_build_time: std::time::Duration::from_millis(2),
            frame_exec_lua_time: std::time::Duration::from_millis(1),
            frame_apply_props_time: std::time::Duration::from_millis(2),
            frame_layer_children_time: std::time::Duration::from_millis(3),
            frame_anim_time: std::time::Duration::from_millis(1),
            frame_button_time: std::time::Duration::from_millis(1),
            frame_lifecycle_time: std::time::Duration::from_millis(2),
            frame_count: 4,
            texture_count: 6,
            fontstring_count: 2,
            lifecycle_fire_count: 3,
            lua_compile_time: std::time::Duration::from_millis(4),
            lua_call_time: std::time::Duration::from_millis(6),
            lua_exec_time: std::time::Duration::from_millis(10),
            saved_vars_time: std::time::Duration::from_millis(5),
            ..LoadTiming::default()
        };

        let lines = timing_breakdown_lines(&timing);

        assert_eq!(lines[0], "Total time: 75.00ms");
        assert!(
            lines
                .iter()
                .any(|line| line.contains("XML frames: 12.00ms"))
        );
        assert!(lines.iter().any(|line| line.contains("compile:  4.00ms")));
        assert!(lines.iter().any(|line| line.contains("4 frames, 6 textures, 2 fontstrings, 3 lifecycle fires")));
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
    let addons_path = PathBuf::from("./Interface/AddOns");
    if skip_addons && !is_test {
        logging::println_elapsed("Addon loading disabled");
        return;
    }

    let exclude = if is_test { &[][..] } else { TEST_ADDONS };
    let mut addons = scan_addons(&addons_path, exclude, screen);
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
    base_path: &PathBuf,
    exclude: &[&str],
    screen: ScreenKind,
) -> Vec<(String, PathBuf)> {
    let mut addons = Vec::new();
    if let Ok(entries) = std::fs::read_dir(base_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            if name.starts_with('.') || name == "BlizzardUI" {
                continue;
            }
            if exclude.iter().any(|e| *e == name) {
                continue;
            }
            if let Some(toc_path) = wow_ui_sim::loader::find_toc_file(&path)
                && let Ok(toc) = TocFile::from_file(&toc_path)
                && toc.allows_screen(screen)
                && !toc.is_ptr_only()
                && !toc.is_game_type_restricted()
            {
                addons.push((name, toc_path));
            }
        }
    }
    wow_ui_sim::loader::sort_addons_by_dependencies(&mut addons);
    addons
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

fn print_timing_breakdown(t: &LoadTiming) {
    for line in timing_breakdown_lines(t) {
        println!("{line}");
    }
}

fn timing_breakdown_lines(timing: &LoadTiming) -> Vec<String> {
    let total_time = timing.total();
    if total_time.is_zero() {
        return Vec::new();
    }

    let mut lines = vec![
        format!("Total time: {:.2?}", total_time),
        timing_breakdown_line("IO", timing.io_time, total_time),
        timing_breakdown_line("XML parse", timing.xml_parse_time, total_time),
        timing_breakdown_line("XML proc", timing.xml_process_time, total_time),
    ];
    lines.extend(frame_timing_detail_lines(timing, total_time));
    lines.push(timing_breakdown_line(
        "Lua exec",
        timing.lua_exec_time,
        total_time,
    ));
    lines.push(lua_compile_line(timing, total_time));
    lines.push(lua_call_line(timing, total_time));
    lines.push(timing_breakdown_line(
        "SavedVars",
        timing.saved_vars_time,
        total_time,
    ));
    lines
}

fn timing_breakdown_line(
    label: &str,
    duration: std::time::Duration,
    total_time: std::time::Duration,
) -> String {
    format!(
        "  {label:10} {:.2?} ({:.1}%)",
        duration,
        timing_percent(duration, total_time)
    )
}

fn frame_timing_detail_lines(timing: &LoadTiming, total_time: std::time::Duration) -> [String; 4] {
    [
        format!(
            "  XML frames: {:.2?} ({:.1}%, subset of XML proc)",
            timing.xml_frame_create_time,
            timing_percent(timing.xml_frame_create_time, total_time)
        ),
        format!(
            "    setup:  {:.2?}  (code_build={:.2?} exec_lua={:.2?} props={:.2?})",
            timing.xml_frame_setup_time,
            timing.frame_code_build_time,
            timing.frame_exec_lua_time,
            timing.frame_apply_props_time
        ),
        format!(
            "    finalize: {:.2?}  (layers={:.2?} anim={:.2?} button={:.2?} lifecycle={:.2?})",
            timing.xml_frame_finalize_time,
            timing.frame_layer_children_time,
            timing.frame_anim_time,
            timing.frame_button_time,
            timing.frame_lifecycle_time
        ),
        format!(
            "    {} frames, {} textures, {} fontstrings, {} lifecycle fires",
            timing.frame_count,
            timing.texture_count,
            timing.fontstring_count,
            timing.lifecycle_fire_count
        ),
    ]
}

fn lua_compile_line(timing: &LoadTiming, total_time: std::time::Duration) -> String {
    format!(
        "    compile:  {:.2?} ({:.1}%, subset of Lua exec)",
        timing.lua_compile_time,
        timing_percent(timing.lua_compile_time, total_time)
    )
}

fn lua_call_line(timing: &LoadTiming, total_time: std::time::Duration) -> String {
    format!(
        "    call:     {:.2?} ({:.1}%, subset of Lua exec)",
        timing.lua_call_time,
        timing_percent(timing.lua_call_time, total_time)
    )
}

fn timing_percent(duration: std::time::Duration, total_time: std::time::Duration) -> f64 {
    100.0 * duration.as_secs_f64() / total_time.as_secs_f64()
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
