//! Blizzard UI addon-bootstrap lane.
//!
//! Keep this file for behaviors that only exist after the relevant Blizzard
//! addons and their startup sequence have loaded.

mod common;

use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use wow_ui_sim::loader::{
    discover_all_blizzard_addons, discover_blizzard_addon_closure_for_screen, load_addon,
};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_errors::grouped_errors_by_addon;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::xml::{clear_templates, register_intrinsic_templates};

struct PanelOpenCoverageCase {
    name: &'static str,
    open_lua: &'static str,
    expected_addon: &'static str,
    expected_frame: &'static str,
    expected_error_overrides: &'static [(&'static str, usize)],
}

const PANEL_OPEN_COVERAGE_CASES: &[PanelOpenCoverageCase] = &[
    PanelOpenCoverageCase {
        name: "achievement",
        open_lua: "ToggleAchievementFrame()",
        expected_addon: "Blizzard_AchievementUI",
        expected_frame: "AchievementFrame",
        expected_error_overrides: &[("<unknown>", 1)],
    },
    PanelOpenCoverageCase {
        name: "collections",
        open_lua: "ToggleCollectionsJournal(COLLECTIONS_JOURNAL_TAB_INDEX_MOUNTS)",
        expected_addon: "Blizzard_Collections",
        expected_frame: "CollectionsJournal",
        expected_error_overrides: &[("<unknown>", 29)],
    },
    PanelOpenCoverageCase {
        name: "encounter_journal",
        open_lua: "ToggleEncounterJournal()",
        expected_addon: "Blizzard_EncounterJournal",
        expected_frame: "EncounterJournal",
        expected_error_overrides: &[("<unknown>", 2)],
    },
];

const PANEL_COVERAGE_ADDONS: &[(&str, &str)] = &[
    ("Blizzard_SharedXMLBase", "Blizzard_SharedXMLBase.toc"),
    ("Blizzard_Colors", "Blizzard_Colors_Mainline.toc"),
    ("Blizzard_SharedXML", "Blizzard_SharedXML_Mainline.toc"),
    ("Blizzard_SharedXMLGame", "Blizzard_SharedXMLGame.toc"),
    (
        "Blizzard_UIPanelTemplates",
        "Blizzard_UIPanelTemplates_Mainline.toc",
    ),
    (
        "Blizzard_FrameXMLBase",
        "Blizzard_FrameXMLBase_Mainline.toc",
    ),
    ("Blizzard_FrameEffects", "Blizzard_FrameEffects.toc"),
    ("Blizzard_LoadLocale", "Blizzard_LoadLocale.toc"),
    ("Blizzard_Fonts_Shared", "Blizzard_Fonts_Shared.toc"),
    ("Blizzard_HelpPlate", "Blizzard_HelpPlate.toc"),
    (
        "Blizzard_AccessibilityTemplates",
        "Blizzard_AccessibilityTemplates.toc",
    ),
    ("Blizzard_ObjectAPI", "Blizzard_ObjectAPI_Mainline.toc"),
    ("Blizzard_UIParent", "Blizzard_UIParent_Mainline.toc"),
    ("Blizzard_TextStatusBar", "Blizzard_TextStatusBar.toc"),
    ("Blizzard_MoneyFrame", "Blizzard_MoneyFrame_Mainline.toc"),
    ("Blizzard_POIButton", "Blizzard_POIButton.toc"),
    ("Blizzard_Flyout", "Blizzard_Flyout.toc"),
    ("Blizzard_StoreUI", "Blizzard_StoreUI_Mainline.toc"),
    ("Blizzard_MicroMenu", "Blizzard_MicroMenu_Mainline.toc"),
    ("Blizzard_EditMode", "Blizzard_EditMode.toc"),
    ("Blizzard_GarrisonBase", "Blizzard_GarrisonBase.toc"),
    ("Blizzard_GameTooltip", "Blizzard_GameTooltip_Mainline.toc"),
    (
        "Blizzard_UIParentPanelManager",
        "Blizzard_UIParentPanelManager_Mainline.toc",
    ),
    (
        "Blizzard_Settings_Shared",
        "Blizzard_Settings_Shared_Mainline.toc",
    ),
    (
        "Blizzard_SettingsDefinitions_Shared",
        "Blizzard_SettingsDefinitions_Shared.toc",
    ),
    (
        "Blizzard_SettingsDefinitions_Frame",
        "Blizzard_SettingsDefinitions_Frame_Mainline.toc",
    ),
    ("Blizzard_FrameXMLUtil", "Blizzard_FrameXMLUtil.toc"),
    ("Blizzard_Menu", "Blizzard_Menu.toc"),
    ("Blizzard_Minimap", "Blizzard_Minimap_Mainline.toc"),
    ("Blizzard_StaticPopup", "Blizzard_StaticPopup.toc"),
    ("Blizzard_TimeManager", "Blizzard_TimeManager_Mainline.toc"),
    ("Blizzard_ItemButton", "Blizzard_ItemButton_Mainline.toc"),
    ("Blizzard_QuickKeybind", "Blizzard_QuickKeybind.toc"),
    ("Blizzard_Collections", "Blizzard_Collections_Mainline.toc"),
    ("Blizzard_FrameXML", "Blizzard_FrameXML_Mainline.toc"),
    (
        "Blizzard_UIPanels_Game",
        "Blizzard_UIPanels_Game_Mainline.toc",
    ),
];

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn panel_coverage_roots() -> Vec<&'static str> {
    PANEL_COVERAGE_ADDONS
        .iter()
        .map(|(addon_name, _)| *addon_name)
        .collect()
}

fn format_per_addon_report(grouped_errors: &BTreeMap<String, Vec<String>>) -> String {
    let mut rows: Vec<_> = grouped_errors.iter().collect();
    rows.sort_by(|(left_name, left_errors), (right_name, right_errors)| {
        right_errors
            .len()
            .cmp(&left_errors.len())
            .then_with(|| left_name.cmp(right_name))
    });

    rows.into_iter()
        .map(|(addon_name, errors)| {
            let sample = errors.first().map(String::as_str).unwrap_or("<no sample>");
            format!("{addon_name}: {} error(s); sample: {sample}", errors.len())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_full_per_addon_report(grouped_errors: &BTreeMap<String, Vec<String>>) -> String {
    format!(
        "Per-addon Lua error report (sorted by error count):\n{}",
        format_per_addon_report(grouped_errors)
    )
}

fn known_error_counts() -> BTreeMap<String, usize> {
    common::addon_coverage_baseline::KNOWN_ERRORS
        .iter()
        .map(|(addon_name, count)| ((*addon_name).to_string(), *count))
        .collect()
}

fn actual_error_counts(grouped_errors: &BTreeMap<String, Vec<String>>) -> BTreeMap<String, usize> {
    grouped_errors
        .iter()
        .map(|(addon_name, errors)| (addon_name.clone(), errors.len()))
        .collect()
}

fn known_panel_open_runtime_error_counts(case: &PanelOpenCoverageCase) -> BTreeMap<String, usize> {
    let mut counts = known_error_counts();
    for (addon_name, count) in case.expected_error_overrides {
        counts.insert((*addon_name).to_string(), *count);
    }
    counts
}

fn format_error_count_map(error_counts: &BTreeMap<String, usize>) -> String {
    error_counts
        .iter()
        .map(|(addon_name, count)| format!("(\"{addon_name}\", {count})"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[derive(Debug, PartialEq, Eq)]
struct ErrorCountChanges {
    increased: Vec<(String, usize, usize)>,
    decreased: Vec<(String, usize, usize)>,
}

fn classify_error_count_changes(
    known: &BTreeMap<String, usize>,
    actual: &BTreeMap<String, usize>,
) -> ErrorCountChanges {
    let mut increased = Vec::new();
    let mut decreased = Vec::new();

    for (addon_name, known_count) in known {
        let actual_count = actual.get(addon_name).copied().unwrap_or(0);
        match actual_count.cmp(known_count) {
            std::cmp::Ordering::Greater => {
                increased.push((addon_name.clone(), *known_count, actual_count));
            }
            std::cmp::Ordering::Less => {
                decreased.push((addon_name.clone(), *known_count, actual_count));
            }
            std::cmp::Ordering::Equal => {}
        }
    }

    ErrorCountChanges {
        increased,
        decreased,
    }
}

fn format_error_count_changes(changes: &[(String, usize, usize)]) -> String {
    changes
        .iter()
        .map(|(addon_name, old_count, new_count)| {
            format!("{addon_name}: {old_count} -> {new_count}")
        })
        .collect::<Vec<_>>()
        .join(", ")
}

#[test]
fn error_count_ratchet_detects_increases_and_decreases() {
    let known = BTreeMap::from([
        ("Blizzard_A".to_string(), 2),
        ("Blizzard_B".to_string(), 4),
        ("Blizzard_C".to_string(), 1),
    ]);
    let actual = BTreeMap::from([
        ("Blizzard_A".to_string(), 3),
        ("Blizzard_B".to_string(), 4),
        ("Blizzard_C".to_string(), 0),
    ]);

    let changes = classify_error_count_changes(&known, &actual);

    assert_eq!(changes.increased, vec![("Blizzard_A".to_string(), 2, 3)],);
    assert_eq!(changes.decreased, vec![("Blizzard_C".to_string(), 1, 0)],);
}

#[test]
fn full_per_addon_report_lists_highest_error_counts_first() {
    let grouped_errors = BTreeMap::from([
        ("Blizzard_B".to_string(), vec!["second".to_string()]),
        (
            "Blizzard_A".to_string(),
            vec!["first".to_string(), "another".to_string()],
        ),
        ("Blizzard_C".to_string(), vec!["third".to_string()]),
    ]);

    let report = format_full_per_addon_report(&grouped_errors);
    let lines: Vec<_> = report.lines().collect();

    assert_eq!(
        lines[0],
        "Per-addon Lua error report (sorted by error count):"
    );
    assert_eq!(lines[1], "Blizzard_A: 2 error(s); sample: first");
    assert_eq!(lines[2], "Blizzard_B: 1 error(s); sample: second");
    assert_eq!(lines[3], "Blizzard_C: 1 error(s); sample: third");
}

fn count_blizzard_directories() -> usize {
    std::fs::read_dir(blizzard_ui_dir())
        .expect("BlizzardUI directory should be readable")
        .flatten()
        .filter(|entry| {
            entry.path().is_dir()
                && entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with("Blizzard_"))
        })
        .count()
}

fn clear_lua_error_tracking(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.lua_errors.clear();
    state.lua_error_records.clear();
    state.lua_error_counts.clear();
}

fn silence_lua_error_handler(env: &WowLuaEnv) {
    env.exec("seterrorhandler(function() end)")
        .expect("seterrorhandler should accept a no-op test handler");
}

fn reset_template_state() {
    clear_templates();
    register_intrinsic_templates();
}

fn fire_panel_harness_startup_events(env: &WowLuaEnv) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    common::fire_player_entering_world(env, true, false);
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
    ] {
        let _ = env.fire_event(event);
    }
}

fn load_panel_harness_blizzard_ui(env: &WowLuaEnv) -> HashSet<String> {
    reset_template_state();
    let ui = blizzard_ui_dir();
    let roots = panel_coverage_roots();
    let closure = discover_blizzard_addon_closure_for_screen(&ui, ScreenKind::Game, &roots);
    for (addon_name, toc_path) in &closure {
        if let Err(error) = load_addon(&env.loader_env(), toc_path) {
            panic!("{addon_name} should load for the panel harness: {error}");
        }
    }

    env.apply_post_load_workarounds();
    fire_panel_harness_startup_events(env);
    silence_lua_error_handler(env);

    closure.into_iter().map(|(name, _)| name).collect()
}

fn frame_is_shown(env: &WowLuaEnv, frame_name: &str) -> bool {
    env.eval(&format!(
        "return _G[{frame_name:?}] ~= nil and _G[{frame_name:?}]:IsShown()"
    ))
    .expect("frame visibility query should return")
}

fn is_addon_loaded(env: &WowLuaEnv, addon_name: &str) -> bool {
    env.eval(&format!("return C_AddOns.IsAddOnLoaded({addon_name:?})"))
        .expect("C_AddOns.IsAddOnLoaded should return")
}

#[test]
fn all_blizzard_addon_load_errors_are_tracked_per_addon_name() {
    common::with_perf_lock(|| {
        common::with_timeout(600, move || {
            let env = WowLuaEnv::new().expect("Failed to create Lua environment");
            env.set_screen_size(1024.0, 768.0);
            env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];
            reset_template_state();

            assert_eq!(
                count_blizzard_directories(),
                315,
                "expected the current Blizzard UI checkout to contain 315 Blizzard_* directories"
            );

            let addons = discover_all_blizzard_addons(&blizzard_ui_dir());
            assert_eq!(
                addons.len(),
                313,
                "expected the current Blizzard UI checkout to expose 313 loadable Blizzard addons; Blizzard_LevelUpDisplay and Blizzard_TalentUI only ship legacy Mists TOCs"
            );

            let known_addons: HashSet<_> = addons.iter().map(|(name, _)| name.clone()).collect();
            let mut load_failures = Vec::new();

            for (name, toc_path) in &addons {
                if let Err(error) = load_addon(&env.loader_env(), toc_path) {
                    load_failures.push(format!("{name}: {error}"));
                }
            }

            assert!(
                load_failures.is_empty(),
                "force-loading all Blizzard addons should not have hard TOC load failures:\n{}",
                load_failures.join("\n"),
            );

            let state = env.state().borrow();
            let grouped_errors = grouped_errors_by_addon(&state);
            println!("{}", format_full_per_addon_report(&grouped_errors));
            let known_counts = known_error_counts();
            let actual_counts = actual_error_counts(&grouped_errors);
            let changes = classify_error_count_changes(&known_counts, &actual_counts);
            let unknown_count = grouped_errors.get("<unknown>").map_or(0, Vec::len);
            let invalid_addons: Vec<_> = grouped_errors
                .keys()
                .filter(|addon_name| {
                    addon_name.as_str() != "<unknown>" && !known_addons.contains(*addon_name)
                })
                .cloned()
                .collect();

            assert!(
                unknown_count == 0,
                "full Blizzard load should attribute Lua errors to addon names, not <unknown>.\n{}",
                format_per_addon_report(&grouped_errors),
            );
            assert!(
                invalid_addons.is_empty(),
                "full Blizzard load attributed Lua errors to names outside the 315 Blizzard addons: {:?}\n{}",
                invalid_addons,
                format_per_addon_report(&grouped_errors),
            );
            assert!(
                changes.increased.is_empty(),
                "full Blizzard load increased per-addon Lua errors.\nincreased: [{}]\nactual counts: [{}]\n{}",
                format_error_count_changes(&changes.increased),
                format_error_count_map(&actual_counts),
                format_per_addon_report(&grouped_errors),
            );
        })
    })
}

fn classify_error_count_increases_from_baseline(
    known: &BTreeMap<String, usize>,
    actual: &BTreeMap<String, usize>,
) -> Vec<(String, usize, usize)> {
    actual
        .iter()
        .filter_map(|(addon_name, actual_count)| {
            let known_count = known.get(addon_name).copied().unwrap_or(0);
            (actual_count > &known_count).then(|| (addon_name.clone(), known_count, *actual_count))
        })
        .collect()
}

#[test]
fn panel_open_runtime_baseline_overrides_known_side_loads() {
    let collections_case = PANEL_OPEN_COVERAGE_CASES
        .iter()
        .find(|case| case.name == "collections")
        .expect("collections coverage case should exist");
    let known_counts = known_panel_open_runtime_error_counts(collections_case);

    assert_eq!(known_counts.get("<unknown>"), Some(&29));
    assert_eq!(known_counts.get("Blizzard_Collections"), Some(&6));
}

#[test]
fn panel_open_runtime_paths_stay_within_known_error_baseline() {
    common::with_perf_lock(|| {
        common::with_timeout(600, move || {
            let env = WowLuaEnv::new().expect("Failed to create Lua environment");
            env.set_screen_size(1024.0, 768.0);
            env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];

            let known_blizzard_addons = load_panel_harness_blizzard_ui(&env);
            let mut case_failures = Vec::new();

            for case in PANEL_OPEN_COVERAGE_CASES {
                clear_lua_error_tracking(&env);
                env.exec(case.open_lua)
                    .unwrap_or_else(|error| panic!("{} opener should run: {error}", case.name));

                let addon_loaded = is_addon_loaded(&env, case.expected_addon);
                let frame_shown = frame_is_shown(&env, case.expected_frame);
                let known_counts = known_panel_open_runtime_error_counts(case);
                let state = env.state().borrow();
                let grouped_errors = grouped_errors_by_addon(&state);
                let actual_counts = actual_error_counts(&grouped_errors);
                let increases =
                    classify_error_count_increases_from_baseline(&known_counts, &actual_counts);
                let invalid_addons: Vec<_> = grouped_errors
                    .keys()
                    .filter(|addon_name| {
                        addon_name.as_str() != "<unknown>"
                            && !known_blizzard_addons.contains(*addon_name)
                    })
                    .cloned()
                    .collect();
                drop(state);

                if !addon_loaded
                    || !frame_shown
                    || !invalid_addons.is_empty()
                    || !increases.is_empty()
                {
                    case_failures.push(format!(
                        "{}: loaded={}, frame_shown={}, increased=[{}], invalid_addons={:?}, actual counts=[{}]\n{}",
                        case.name,
                        addon_loaded,
                        frame_shown,
                        format_error_count_changes(&increases),
                        invalid_addons,
                        format_error_count_map(&actual_counts),
                        format_per_addon_report(&grouped_errors),
                    ));
                }
            }

            assert!(
                case_failures.is_empty(),
                "panel-open runtime paths exceeded the known per-addon Lua error baseline:\n{}",
                case_failures.join("\n\n"),
            );
        })
    })
}
