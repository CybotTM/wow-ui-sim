mod common;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::panic::{self, AssertUnwindSafe};
use std::path::PathBuf;
use wow_ui_sim::loader::{discover_all_blizzard_addons, discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_errors::grouped_errors_by_addon;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;
use wow_ui_sim::toc::TocFile;
use wow_ui_sim::xml::{
    FrameXml, clear_templates, get_template, register_intrinsic_templates, register_template,
};

const KNOWN_ERRORS: &[(&str, usize)] = &[
    ("Blizzard_AccountSaveUI", 3),
    ("Blizzard_AchievementUI", 28),
    ("Blizzard_ActionBar", 3),
    ("Blizzard_ActionBarController", 3),
    ("Blizzard_ActionStatus", 2),
    ("Blizzard_AlliedRacesUI", 2),
    ("Blizzard_AnimaDiversionUI", 3),
    ("Blizzard_ArchaeologyUI", 10),
    ("Blizzard_ArdenwealdGardening", 1),
    ("Blizzard_ArrowCalloutFrame", 3),
    ("Blizzard_ArtifactUI", 5),
    ("Blizzard_AuctionHouseUI", 63),
    ("Blizzard_AzeriteEssenceUI", 4),
    ("Blizzard_AzeriteRespecUI", 1),
    ("Blizzard_AzeriteUI", 3),
    ("Blizzard_BarbershopUI", 2),
    ("Blizzard_BattlefieldMap", 4),
    ("Blizzard_BlackMarketUI", 9),
    ("Blizzard_BoostTutorial", 6),
    ("Blizzard_Calendar", 14),
    ("Blizzard_CatalogShopSharedTemplates", 1),
    ("Blizzard_ChallengesUI", 1),
    ("Blizzard_CharacterCreate", 13),
    ("Blizzard_CharacterCustomize", 5),
    ("Blizzard_ChatFrameBase", 2),
    ("Blizzard_ClickBindingUI", 4),
    ("Blizzard_Collections", 76),
    ("Blizzard_CombatLog", 5),
    ("Blizzard_CombatText", 2),
    ("Blizzard_Commentator", 2),
    ("Blizzard_Communities", 6),
    ("Blizzard_Console", 4),
    ("Blizzard_CovenantSanctum", 1),
    ("Blizzard_CustomizationUI", 7),
    ("Blizzard_DeathRecap", 2),
    ("Blizzard_DebugTools", 4),
    ("Blizzard_DelvesDifficultyPicker", 8),
    ("Blizzard_Deprecated", 1),
    ("Blizzard_DeprecatedActionBar", 1),
    ("Blizzard_DeprecatedAutoComplete", 1),
    ("Blizzard_DeprecatedBattleNet", 1),
    ("Blizzard_DeprecatedChatInfo", 2),
    ("Blizzard_DeprecatedCombatLog", 1),
    ("Blizzard_DeprecatedCurrencyScript", 1),
    ("Blizzard_DeprecatedGlue", 1),
    ("Blizzard_DeprecatedGuildScript", 1),
    ("Blizzard_DeprecatedHousingCatalog", 1),
    ("Blizzard_DeprecatedInstanceEncounter", 1),
    ("Blizzard_DeprecatedItemScript", 1),
    ("Blizzard_DeprecatedItemSocketInfo", 1),
    ("Blizzard_DeprecatedLFG", 1),
    ("Blizzard_DeprecatedPetInfo", 1),
    ("Blizzard_DeprecatedPvpScript", 1),
    ("Blizzard_DeprecatedSoundScript", 1),
    ("Blizzard_DeprecatedSpecialization", 2),
    ("Blizzard_DeprecatedSpellBook", 1),
    ("Blizzard_DeprecatedSpellScript", 1),
    ("Blizzard_DeprecatedTradeInfo", 1),
    ("Blizzard_DeprecatedUnitScript", 1),
    ("Blizzard_DeprecatedWorldElapsedTimerTypes", 1),
    ("Blizzard_EncounterJournal", 21),
    ("Blizzard_EndOfMatchUI", 2),
    ("Blizzard_EventTrace", 4),
    ("Blizzard_ExpansionTrial", 2),
    ("Blizzard_FlightMap", 11),
    ("Blizzard_FrameXMLUtil", 8),
    ("Blizzard_GarrisonTemplates", 4),
    ("Blizzard_GarrisonUI", 486),
    ("Blizzard_GenericShoppingCart", 3),
    ("Blizzard_GlueXML", 18),
    ("Blizzard_GuildBankUI", 12),
    ("Blizzard_GuildControlUI", 11),
    ("Blizzard_HelpPlate", 2),
    ("Blizzard_HouseEditor", 103),
    ("Blizzard_HouseList", 2),
    ("Blizzard_HousingBulletinBoard", 13),
    ("Blizzard_HousingCharter", 2),
    ("Blizzard_HousingControls", 3),
    ("Blizzard_HousingCreateNeighborhood", 4),
    ("Blizzard_HousingDashboard", 23),
    ("Blizzard_HousingEventHandler", 1),
    ("Blizzard_HousingHouseFinder", 15),
    ("Blizzard_HousingHouseSettings", 4),
    ("Blizzard_HousingInspectModeUI", 4),
    ("Blizzard_HousingMarketCart", 1),
    ("Blizzard_HousingModelPreview", 6),
    ("Blizzard_HousingTemplates", 5),
    ("Blizzard_HybridMinimap", 3),
    ("Blizzard_InspectUI", 42),
    ("Blizzard_IslandsPartyPoseUI", 8),
    ("Blizzard_IslandsQueueUI", 2),
    ("Blizzard_ItemBeltFrame", 3),
    ("Blizzard_ItemInteractionUI", 5),
    ("Blizzard_ItemUpgradeUI", 14),
    ("Blizzard_Kiosk", 10),
    ("Blizzard_MacroUI", 7),
    ("Blizzard_MainMenuBarBagButtons", 25),
    ("Blizzard_MatchCelebrationPartyPoseUI", 8),
    ("Blizzard_MicroMenu", 57),
    ("Blizzard_MovePad", 16),
    ("Blizzard_NewPlayerExperienceGuide", 2),
    ("Blizzard_ObliterumUI", 2),
    ("Blizzard_OrderHallUI", 1),
    ("Blizzard_PTRFeedback", 5),
    ("Blizzard_PTRFeedbackGlue", 5),
    ("Blizzard_PVPUI", 15),
    ("Blizzard_PagedContent", 4),
    ("Blizzard_PartyPoseUI", 2),
    ("Blizzard_PerksProgram", 27),
    ("Blizzard_PetBattleUI", 35),
    ("Blizzard_PhotoSharing", 4),
    ("Blizzard_PlayerChoice", 2),
    ("Blizzard_PlayerSpells", 6),
    ("Blizzard_PlunderstormBasics", 2),
    ("Blizzard_PlunderstormPrematchUI", 10),
    ("Blizzard_PrivateAurasUI", 4),
    ("Blizzard_Professions", 4),
    ("Blizzard_ProfessionsBook", 26),
    ("Blizzard_ProfessionsCustomerOrders", 15),
    ("Blizzard_ProfessionsTemplates", 11),
    ("Blizzard_QuestNavigation", 1),
    ("Blizzard_RPE_TurnStrafe", 1),
    ("Blizzard_RaidUI", 1),
    ("Blizzard_RecentAllies", 1),
    ("Blizzard_ReforgingUI", 2),
    ("Blizzard_RemixArtifactTutorialUI", 3),
    ("Blizzard_ReportFrame", 2),
    ("Blizzard_ReportFrameGlue", 2),
    ("Blizzard_RuneforgeUI", 24),
    ("Blizzard_ScrappingMachineUI", 1),
    ("Blizzard_SharedMapDataProviders", 84),
    ("Blizzard_SpectateFrame", 3),
    ("Blizzard_SpellSearch", 2),
    ("Blizzard_SubscriptionInterstitialUI", 6),
    ("Blizzard_TimeManager", 6),
    ("Blizzard_TimerunningCharacterCreate", 3),
    ("Blizzard_Transmog", 18),
    ("Blizzard_WarfrontsPartyPoseUI", 8),
    ("Blizzard_WorldMap", 5),
];

const KNOWN_LOAD_ON_DEMAND_RUNTIME_ERRORS: &[(&str, usize)] = &[
    ("Blizzard_AzeriteEssenceUI", 6),
    ("Blizzard_BoostTutorial", 8),
    ("Blizzard_EventTrace", 4),
    ("Blizzard_ExpansionTrial", 4),
    ("Blizzard_ItemBeltFrame", 4),
    ("Blizzard_ItemInteractionUI", 6),
    ("Blizzard_Professions", 16),
    ("Blizzard_ScrappingMachineUI", 2),
    ("Blizzard_TimerunningCharacterCreate", 4),
];

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
    ("Blizzard_FrameXML", "Blizzard_FrameXML_Mainline.toc"),
    (
        "Blizzard_UIPanels_Game",
        "Blizzard_UIPanels_Game_Mainline.toc",
    ),
];

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
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
    KNOWN_ERRORS
        .iter()
        .map(|(addon_name, count)| ((*addon_name).to_string(), *count))
        .collect()
}

fn known_load_on_demand_runtime_error_counts() -> BTreeMap<String, usize> {
    let mut counts = known_error_counts();
    for (addon_name, count) in KNOWN_LOAD_ON_DEMAND_RUNTIME_ERRORS {
        counts.insert((*addon_name).to_string(), *count);
    }
    counts
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

fn discover_blizzard_lod_addon_tocs() -> Vec<(String, TocFile)> {
    discover_all_blizzard_addons(&blizzard_ui_dir())
        .into_iter()
        .filter_map(|(name, toc_path)| {
            let toc = TocFile::from_file(&toc_path).ok()?;
            (toc.is_load_on_demand()
                && toc.allows_screen(ScreenKind::Game)
                && !toc.is_ptr_only()
                && !toc.is_game_type_restricted())
            .then_some((name, toc))
        })
        .collect()
}

fn discover_blizzard_lod_addon_families() -> Vec<Vec<String>> {
    let lod_tocs = discover_blizzard_lod_addon_tocs();
    let ordered_addons: Vec<_> = lod_tocs.iter().map(|(name, _)| name.clone()).collect();
    let addon_indices: HashMap<_, _> = ordered_addons
        .iter()
        .enumerate()
        .map(|(index, addon_name)| (addon_name.clone(), index))
        .collect();
    let addon_names: HashSet<_> = ordered_addons.iter().cloned().collect();
    let toc_by_addon: HashMap<_, _> = lod_tocs.into_iter().collect();
    let mut adjacency: HashMap<_, Vec<String>> = ordered_addons
        .iter()
        .cloned()
        .map(|addon_name| (addon_name, Vec::new()))
        .collect();

    for (addon_name, toc) in &toc_by_addon {
        let related_addons: HashSet<_> = toc
            .dependencies()
            .into_iter()
            .chain(toc.optional_deps())
            .chain(toc.load_with())
            .filter(|dependency| addon_names.contains(dependency))
            .collect();

        for related_addon in related_addons {
            adjacency
                .get_mut(addon_name)
                .expect("each load-on-demand addon should have an adjacency list")
                .push(related_addon.clone());
            adjacency
                .get_mut(&related_addon)
                .expect("each related load-on-demand addon should have an adjacency list")
                .push(addon_name.clone());
        }
    }

    let mut synthetic_family_members: HashMap<&'static str, Vec<String>> = HashMap::new();
    for addon_name in &ordered_addons {
        if let Some(family_key) = synthetic_load_on_demand_family_key(addon_name) {
            synthetic_family_members
                .entry(family_key)
                .or_default()
                .push(addon_name.clone());
        }
    }

    for family_members in synthetic_family_members.into_values() {
        if let Some(family_root) = family_members.first() {
            for related_addon in family_members.iter().skip(1) {
                adjacency
                    .get_mut(family_root)
                    .expect("synthetic family roots should have adjacency lists")
                    .push(related_addon.clone());
                adjacency
                    .get_mut(related_addon)
                    .expect("synthetic family members should have adjacency lists")
                    .push(family_root.clone());
            }
        }
    }

    let mut visited = HashSet::new();
    let mut families = Vec::new();
    for addon_name in ordered_addons {
        if !visited.insert(addon_name.clone()) {
            continue;
        }

        let mut family = vec![addon_name.clone()];
        let mut stack = vec![addon_name];
        while let Some(current) = stack.pop() {
            for related_addon in adjacency
                .get(&current)
                .into_iter()
                .flatten()
                .filter(|related_addon| visited.insert((*related_addon).clone()))
            {
                family.push(related_addon.clone());
                stack.push(related_addon.clone());
            }
        }

        family.sort_by_key(|related_addon| {
            *addon_indices
                .get(related_addon)
                .expect("family addons should preserve discovery order")
        });
        families.push(family);
    }

    families
}

fn synthetic_load_on_demand_family_key(addon_name: &str) -> Option<&'static str> {
    if addon_name.starts_with("Blizzard_Settings") {
        Some("settings")
    } else if addon_name.starts_with("Blizzard_Professions") {
        Some("professions")
    } else if addon_name == "Blizzard_Kiosk"
        || addon_name.starts_with("Blizzard_House")
        || addon_name.starts_with("Blizzard_Housing")
    {
        Some("housing")
    } else {
        None
    }
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

fn with_isolated_addon_coverage_state(f: impl FnOnce()) {
    reset_template_state();
    let result = panic::catch_unwind(AssertUnwindSafe(f));
    reset_template_state();
    if let Err(payload) = result {
        panic::resume_unwind(payload);
    }
}

fn load_startup_blizzard_ui(env: &WowLuaEnv) -> HashSet<String> {
    reset_template_state();
    let known_blizzard_addons: HashSet<_> = discover_all_blizzard_addons(&blizzard_ui_dir())
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    let startup_addons = discover_blizzard_addons(&blizzard_ui_dir());
    let mut load_failures = Vec::new();
    for (name, toc_path) in &startup_addons {
        if let Err(error) = load_addon(&env.loader_env(), toc_path) {
            load_failures.push(format!("{name}: {error}"));
        }
    }

    assert!(
        load_failures.is_empty(),
        "startup Blizzard addon load should not have hard TOC load failures:\n{}",
        load_failures.join("\n"),
    );

    env.apply_post_load_workarounds();
    settle_headless_startup(env);
    silence_lua_error_handler(env);
    known_blizzard_addons
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
    for (addon_name, toc_name) in PANEL_COVERAGE_ADDONS {
        let toc_path = ui.join(addon_name).join(toc_name);
        if !toc_path.exists() {
            continue;
        }
        if let Err(error) = load_addon(&env.loader_env(), &toc_path) {
            panic!("{addon_name} should load for the panel harness: {error}");
        }
    }

    env.apply_post_load_workarounds();
    fire_panel_harness_startup_events(env);
    silence_lua_error_handler(env);

    discover_all_blizzard_addons(&ui)
        .into_iter()
        .map(|(name, _)| name)
        .collect()
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

fn load_on_demand_shard_weight(
    addon_name: &str,
    known_runtime_counts: &BTreeMap<String, usize>,
) -> usize {
    known_runtime_counts
        .get(addon_name)
        .copied()
        .unwrap_or(0)
        .max(1)
}

fn shard_load_on_demand_addon_families(
    lod_families: &[Vec<String>],
    shard_count: usize,
    known_runtime_counts: &BTreeMap<String, usize>,
) -> Vec<Vec<Vec<String>>> {
    let mut weighted_families: Vec<_> = lod_families
        .iter()
        .enumerate()
        .map(|(original_index, family)| {
            let family_weight = family
                .iter()
                .map(|addon_name| load_on_demand_shard_weight(addon_name, known_runtime_counts))
                .sum::<usize>()
                .max(1);

            (original_index, family.clone(), family_weight)
        })
        .collect();

    weighted_families.sort_by(
        |(left_index, _, left_weight), (right_index, _, right_weight)| {
            right_weight
                .cmp(left_weight)
                .then_with(|| left_index.cmp(right_index))
        },
    );

    let mut shard_weights = vec![0usize; shard_count];
    let mut shards: Vec<Vec<(usize, Vec<String>)>> = vec![Vec::new(); shard_count];
    for (original_index, family, weight) in weighted_families {
        let shard_index = (0..shard_count)
            .min_by_key(|&index| (shard_weights[index], shards[index].len(), index))
            .expect("shard_count should be non-zero");
        shard_weights[shard_index] += weight;
        shards[shard_index].push((original_index, family));
    }

    shards
        .into_iter()
        .map(|mut shard| {
            shard.sort_by_key(|(original_index, _)| *original_index);
            shard.into_iter().map(|(_, family)| family).collect()
        })
        .collect()
}

#[test]
fn load_on_demand_runtime_baseline_overrides_force_load_counts() {
    let known_runtime_counts = known_load_on_demand_runtime_error_counts();

    assert_eq!(known_runtime_counts.get("Blizzard_EventTrace"), Some(&4));
    assert_eq!(known_runtime_counts.get("Blizzard_Professions"), Some(&16));
    assert_eq!(known_runtime_counts.get("Blizzard_WorldMap"), Some(&5));
}

#[test]
fn panel_open_runtime_baseline_overrides_known_side_loads() {
    let collections_case = PANEL_OPEN_COVERAGE_CASES
        .iter()
        .find(|case| case.name == "collections")
        .expect("collections coverage case should exist");
    let known_counts = known_panel_open_runtime_error_counts(collections_case);

    assert_eq!(known_counts.get("<unknown>"), Some(&29));
    assert_eq!(known_counts.get("Blizzard_Collections"), Some(&76));
}

#[test]
fn shard_load_on_demand_addons_spreads_heavy_addons_across_shards() {
    let lod_families = vec![
        vec!["Blizzard_Light".to_string()],
        vec!["Blizzard_HeavyA".to_string()],
        vec![
            "Blizzard_HeavyB".to_string(),
            "Blizzard_HeavyB_Dependency".to_string(),
        ],
        vec!["Blizzard_Medium".to_string()],
    ];
    let known_runtime_counts = BTreeMap::from([
        ("Blizzard_HeavyA".to_string(), 100),
        ("Blizzard_HeavyB".to_string(), 90),
        ("Blizzard_Medium".to_string(), 10),
    ]);

    let shards = shard_load_on_demand_addon_families(&lod_families, 2, &known_runtime_counts);

    assert_eq!(shards.len(), 2);
    assert!(
        shards[0]
            .iter()
            .flatten()
            .any(|addon_name| addon_name == "Blizzard_HeavyA")
    );
    assert!(
        shards[1]
            .iter()
            .flatten()
            .any(|addon_name| addon_name == "Blizzard_HeavyB")
    );
    assert!(
        shards.iter().any(|shard| shard.iter().any(|family| {
            family.contains(&"Blizzard_HeavyB".to_string())
                && family.contains(&"Blizzard_HeavyB_Dependency".to_string())
        })),
        "dependency families should stay together inside a single shard",
    );
}

fn first_unloaded_addon_in_family(env: &WowLuaEnv, family: &[String]) -> Option<String> {
    family
        .iter()
        .find(|addon_name| !is_addon_loaded(env, addon_name))
        .cloned()
}

#[test]
fn first_unloaded_addon_in_family_respects_family_order() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];
    let family = vec![
        "Blizzard_First".to_string(),
        "Blizzard_Second".to_string(),
        "Blizzard_Third".to_string(),
    ];

    let first = first_unloaded_addon_in_family(&env, &family);

    assert_eq!(first.as_deref(), Some("Blizzard_First"));
}

#[test]
fn generic_trait_ui_runtime_load_survives_prior_force_load_process_state() {
    common::with_perf_lock(|| {
        common::with_timeout(600, move || {
            let env = WowLuaEnv::new().expect("Failed to create Lua environment");
            env.set_screen_size(1024.0, 768.0);
            env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];

            for (_, toc_path) in discover_all_blizzard_addons(&blizzard_ui_dir()) {
                let _ = load_addon(&env.loader_env(), &toc_path);
            }
            drop(env);

            let env = WowLuaEnv::new().expect("Failed to create Lua environment");
            env.set_screen_size(1024.0, 768.0);
            env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];
            load_startup_blizzard_ui(&env);

            let was_loaded: bool = env
                .eval("return C_AddOns.IsAddOnLoaded(\"Blizzard_GenericTraitUI\")")
                .expect("precondition query should return");
            let (loaded, reason): (bool, Option<String>) = env
                .eval("return C_AddOns.LoadAddOn(\"Blizzard_GenericTraitUI\")")
                .expect("GenericTraitUI load should return");
            let now_loaded: bool = env
                .eval("return C_AddOns.IsAddOnLoaded(\"Blizzard_GenericTraitUI\")")
                .expect("postcondition query should return");

            assert!(
                loaded && now_loaded,
                "GenericTraitUI should load after a prior force-load pass; was_loaded={was_loaded}, loaded={loaded}, reason={reason:?}, now_loaded={now_loaded}",
            );
        })
    })
}

#[test]
fn contribution_runtime_load_survives_post_startup_state() {
    with_isolated_addon_coverage_state(|| {
        common::with_perf_lock(|| {
            common::with_timeout(600, move || {
                let env = WowLuaEnv::new().expect("Failed to create Lua environment");
                env.set_screen_size(1024.0, 768.0);
                env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];
                load_startup_blizzard_ui(&env);
                clear_lua_error_tracking(&env);

                let (collector_type, close_type): (String, String) = env
                    .eval(
                        "return type(C_ContributionCollector), type(C_ContributionCollector and C_ContributionCollector.Close)",
                    )
                    .expect("collector shape query should return");
                let (loaded, reason): (bool, Option<String>) = env
                    .eval("return C_AddOns.LoadAddOn(\"Blizzard_Contribution\")")
                    .expect("Blizzard_Contribution load should return");
                let state = env.state().borrow();
                let grouped_errors = grouped_errors_by_addon(&state);

                assert!(
                    loaded,
                    "Blizzard_Contribution should load after startup; collector_type={collector_type}, close_type={close_type}, reason={reason:?}, errors=\n{}",
                    format_per_addon_report(&grouped_errors),
                );
            })
        })
    });
}

#[test]
fn shard_14_runtime_load_survives_prior_runtime_shards_in_process() {
    for shard_index in 9..14 {
        run_load_on_demand_blizzard_addon_shard(shard_index, 16);
    }
}

#[test]
fn perf_lock_recovers_after_prior_panicking_holder() {
    let first = panic::catch_unwind(|| {
        common::with_perf_lock(|| panic!("intentional perf lock poison"));
    });
    assert!(first.is_err(), "first perf-lock holder should panic");

    let second = panic::catch_unwind(|| {
        common::with_perf_lock(|| {});
    });
    assert!(
        second.is_ok(),
        "perf lock should recover after a prior panic instead of poisoning later shards"
    );
}

#[test]
fn isolated_shard_runner_resets_template_state_after_panic() {
    let first = panic::catch_unwind(|| {
        with_isolated_addon_coverage_state(|| {
            register_template("PoisonTemplate", "Frame", FrameXml::default());
            assert!(
                get_template("PoisonTemplate").is_some(),
                "test setup should register the synthetic template before the panic"
            );
            panic!("intentional shard failure");
        });
    });
    assert!(first.is_err(), "first isolated shard should panic");

    with_isolated_addon_coverage_state(|| {
        assert!(
            get_template("PoisonTemplate").is_none(),
            "template registry should be reset before the next isolated shard runs"
        );
    });
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

fn load_first_unloaded_addon_in_family(
    env: &WowLuaEnv,
    family: &[String],
    load_failures: &mut Vec<String>,
) -> Option<String> {
    if let Some(addon_name) = first_unloaded_addon_in_family(env, family) {
        let (loaded, reason): (bool, Option<String>) = env
            .eval(&format!("return C_AddOns.LoadAddOn({addon_name:?})"))
            .unwrap_or_else(|error| {
                panic!("{addon_name}: C_AddOns.LoadAddOn should return: {error:?}")
            });

        if !loaded {
            load_failures.push(format!(
                "{addon_name}: LoadAddOn returned false ({})",
                reason.as_deref().unwrap_or("nil"),
            ));
        }

        Some(addon_name)
    } else {
        None
    }
}

fn run_load_on_demand_blizzard_addon_shard(shard_index: usize, shard_count: usize) {
    with_isolated_addon_coverage_state(|| {
        common::with_perf_lock(|| {
            common::with_timeout(600, move || {
                let env = WowLuaEnv::new().expect("Failed to create Lua environment");
                env.set_screen_size(1024.0, 768.0);
                env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];

                let known_blizzard_addons = load_startup_blizzard_ui(&env);
                let lod_families = discover_blizzard_lod_addon_families();
                let known_runtime_counts = known_load_on_demand_runtime_error_counts();
                let shard_families = shard_load_on_demand_addon_families(
                    &lod_families,
                    shard_count,
                    &known_runtime_counts,
                );
                let mut family_failures = Vec::new();
                let mut load_failures = Vec::new();

                for family in &shard_families[shard_index] {
                    clear_lua_error_tracking(&env);
                    let representative =
                        load_first_unloaded_addon_in_family(&env, family, &mut load_failures);
                    let Some(representative) = representative else {
                        continue;
                    };
                    let state = env.state().borrow();
                    let grouped_errors = grouped_errors_by_addon(&state);
                    let actual_counts = actual_error_counts(&grouped_errors);
                    let increases = classify_error_count_increases_from_baseline(
                        &known_runtime_counts,
                        &actual_counts,
                    );
                    let invalid_addons: Vec<_> = grouped_errors
                        .keys()
                        .filter(|addon_name| {
                            addon_name.as_str() != "<unknown>"
                                && !known_blizzard_addons.contains(*addon_name)
                        })
                        .cloned()
                        .collect();
                    let unknown_count = grouped_errors.get("<unknown>").map_or(0, Vec::len);
                    if unknown_count > 0 || !invalid_addons.is_empty() || !increases.is_empty() {
                        family_failures.push(format!(
                            "{representative}: increased [{}], invalid_addons={:?}, unknown_count={}, actual counts=[{}]\n{}",
                            format_error_count_changes(&increases),
                            invalid_addons,
                            unknown_count,
                            format_error_count_map(&actual_counts),
                            format_per_addon_report(&grouped_errors),
                        ));
                    }
                }

                assert!(
                    load_failures.is_empty(),
                    "runtime LoadAddOn should load every Blizzard LoD addon in shard {shard_index}/{shard_count} after startup:\n{}",
                    load_failures.join("\n"),
                );
                assert!(
                    family_failures.is_empty(),
                    "runtime LoadAddOn exceeded the known runtime per-addon Lua error baseline for at least one family in shard {shard_index}/{shard_count}:\n{}",
                    family_failures.join("\n\n"),
                );
            })
        })
    })
}

#[test]
fn load_on_demand_blizzard_addons_shard_1_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(0, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_2_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(1, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_3_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(2, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_4_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(3, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_5_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(4, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_6_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(5, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_7_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(6, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_8_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(7, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_9_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(8, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_10_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(9, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_11_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(10, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_12_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(11, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_13_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(12, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_14_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(13, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_15_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(14, 16);
}

#[test]
fn load_on_demand_blizzard_addons_shard_16_stays_within_known_error_baseline_after_startup() {
    run_load_on_demand_blizzard_addon_shard(15, 16);
}
