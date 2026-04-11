mod common;

use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use wow_ui_sim::loader::{discover_all_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_errors::grouped_errors_by_addon;

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
    ("Blizzard_AzeriteEssenceUI", 2),
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
    ("Blizzard_EventTrace", 1),
    ("Blizzard_ExpansionTrial", 1),
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

fn actual_error_counts(grouped_errors: &BTreeMap<String, Vec<String>>) -> BTreeMap<String, usize> {
    grouped_errors
        .iter()
        .map(|(addon_name, errors)| (addon_name.clone(), errors.len()))
        .collect()
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

#[test]
fn all_blizzard_addon_load_errors_are_tracked_per_addon_name() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);
        env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];

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
            .filter(|addon_name| addon_name.as_str() != "<unknown>" && !known_addons.contains(*addon_name))
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
        assert!(
            changes.decreased.is_empty(),
            "full Blizzard load decreased per-addon Lua errors; ratchet KNOWN_ERRORS down.\ndecreased: [{}]\nactual counts: [{}]\n{}",
            format_error_count_changes(&changes.decreased),
            format_error_count_map(&actual_counts),
            format_per_addon_report(&grouped_errors),
        );
    }
}
