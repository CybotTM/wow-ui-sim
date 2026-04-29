//! Tests for Blizzard addon load order.
//!
//! Verifies that transitive dependencies are loaded before the addons that
//! need them, even when the dependency chain crosses base UI addon boundaries.

mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
}

/// Blizzard_ObjectAPI (which defines ItemMixin) must load before Blizzard_FrameXML
/// (which uses ItemMixin in EventToastManager.lua:669).
#[test]
fn test_object_api_loads_before_frame_xml() {
    test_timeout! {
        let ui = blizzard_ui_dir();
        let addons = discover_blizzard_addons(&ui);

        let names: Vec<&str> = addons.iter().map(|(n, _)| n.as_str()).collect();

        let obj_api_pos = names.iter().position(|&n| n == "Blizzard_ObjectAPI");
        let frame_xml_pos = names.iter().position(|&n| n == "Blizzard_FrameXML");

        assert!(
            obj_api_pos.is_some(),
            "Blizzard_ObjectAPI should be in the addon list"
        );
        assert!(
            frame_xml_pos.is_some(),
            "Blizzard_FrameXML should be in the addon list"
        );

        assert!(
            obj_api_pos.unwrap() < frame_xml_pos.unwrap(),
            "Blizzard_ObjectAPI (pos {}) must load before Blizzard_FrameXML (pos {})\n\
             Load order: {:?}",
            obj_api_pos.unwrap(),
            frame_xml_pos.unwrap(),
            &names[..std::cmp::min(names.len(), 10)],
        );
    }
}

/// ItemMixin (from Blizzard_ObjectAPI) must be defined when Blizzard_FrameXML loads.
/// EventToastManager.lua:669 does `CreateFromMixins(..., ItemMixin)` at file scope.
#[test]
fn test_item_mixin_available_for_event_toast_manager() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        let ui = blizzard_ui_dir();
        let addons = discover_blizzard_addons(&ui);

        for (name, toc_path) in &addons {
            load_addon(&env.loader_env(), toc_path).ok();
            if name == "Blizzard_FrameXML" {
                break;
            }
        }

        let has_item_mixin: bool = env
            .eval("return type(ItemMixin) == 'table'")
            .unwrap_or(false);
        assert!(
            has_item_mixin,
            "ItemMixin should be defined before Blizzard_FrameXML finishes loading"
        );
    }
}

/// Blizzard_UIPanels_Game (which defines PaperDollItemSlotButton_OnLoad) must
/// load before Blizzard_MainMenuBarBagButtons (whose OnLoad calls it).
/// ActionBar depends on UIPanels_Game, and ActionBar sorts before BagButtons
/// alphabetically, so the dependency chain pulls UIPanels_Game in first.
#[test]
fn test_uipanels_game_loads_before_bag_buttons() {
    test_timeout! {
        let ui = blizzard_ui_dir();
        let addons = discover_blizzard_addons(&ui);

        let names: Vec<&str> = addons.iter().map(|(n, _)| n.as_str()).collect();

        let uipanels_pos = names.iter().position(|&n| n == "Blizzard_UIPanels_Game");
        let bags_pos = names.iter().position(|&n| n == "Blizzard_MainMenuBarBagButtons");

        assert!(uipanels_pos.is_some(), "Blizzard_UIPanels_Game should be in the addon list");
        assert!(bags_pos.is_some(), "Blizzard_MainMenuBarBagButtons should be in the addon list");

        assert!(
            uipanels_pos.unwrap() < bags_pos.unwrap(),
            "Blizzard_UIPanels_Game (pos {}) must load before Blizzard_MainMenuBarBagButtons (pos {})",
            uipanels_pos.unwrap(),
            bags_pos.unwrap(),
        );
    }
}

/// Blizzard_ItemButton currently loads before Blizzard_FrameXMLUtil, so
/// ItemButtonMixin:PostOnShow can run before ItemButtonUtil exists.
#[test]
fn test_item_button_loads_before_framexmlutil() {
    test_timeout! {
        let ui = blizzard_ui_dir();
        let addons = discover_blizzard_addons(&ui);

        let names: Vec<&str> = addons.iter().map(|(n, _)| n.as_str()).collect();

        let item_button_pos = names.iter().position(|&n| n == "Blizzard_ItemButton");
        let framexmlutil_pos = names.iter().position(|&n| n == "Blizzard_FrameXMLUtil");

        assert!(
            item_button_pos.is_some(),
            "Blizzard_ItemButton should be in the addon list"
        );
        assert!(
            framexmlutil_pos.is_some(),
            "Blizzard_FrameXMLUtil should be in the addon list"
        );

        assert!(
            item_button_pos.unwrap() < framexmlutil_pos.unwrap(),
            "Blizzard_ItemButton (pos {}) must currently load before Blizzard_FrameXMLUtil (pos {})",
            item_button_pos.unwrap(),
            framexmlutil_pos.unwrap(),
        );
    }
}

/// When Blizzard_ItemButton has loaded but Blizzard_FrameXMLUtil has not,
/// ItemButtonMixin exists but ItemButtonUtil still does not.
#[test]
fn test_item_button_mixin_exists_before_item_button_util() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        let ui = blizzard_ui_dir();
        let addons = discover_blizzard_addons(&ui);

        for (name, toc_path) in &addons {
            load_addon(&env.loader_env(), toc_path).ok();
            if name == "Blizzard_ItemButton" {
                break;
            }
        }

        let (has_item_button_mixin, has_item_button_util): (bool, bool) = env
            .eval(
                r#"
                return type(ItemButtonMixin) == "table",
                    type(ItemButtonUtil) == "table"
                "#,
            )
            .unwrap();
        assert!(has_item_button_mixin);
        assert!(
            !has_item_button_util,
            "ItemButtonUtil should still be unavailable when Blizzard_ItemButton finishes loading"
        );
    }
}

/// Snapshot of the full resolved Blizzard addon load order.
///
/// If the topological sort algorithm changes and reorders addons, this test
/// catches it. Update the snapshot deliberately when the order changes for a
/// good reason (e.g. new addon added, wow-ui-source updated, dependency changed).
///
/// To regenerate: `cargo test --test load_order dump_load_order -- --ignored --nocapture`
#[test]
fn test_blizzard_addon_load_order_snapshot() {
    test_timeout! {
        let ui = blizzard_ui_dir();
        let addons = discover_blizzard_addons(&ui);
        let names: Vec<&str> = addons.iter().map(|(n, _)| n.as_str()).collect();

        #[rustfmt::skip]
        let expected: &[&str] = &[
            "Blizzard_LoadLocale",
            "Blizzard_Fonts_Shared",
            "Blizzard_ScriptErrors",
            "Blizzard_SharedXMLBase",
            "Blizzard_PrintHandler",
            "Blizzard_Menu",
            "Blizzard_Colors",
            "Blizzard_HelpPlate",
            "Blizzard_SharedXML",
            "Blizzard_AuthChallengeUI",
            "Blizzard_CatalogShopSharedUtil",
            "Blizzard_CatalogShopSharedTemplates",
            "Blizzard_CatalogShop",
            "Blizzard_AsyncRequest",
            "Blizzard_CatalogShopRefundFlow",
            "Blizzard_CatalogShopTopUpFlow",
            "Blizzard_ClassTrialSecure",
            "Blizzard_StoreUI",
            "Blizzard_Settings_Shared",
            "Blizzard_TextStatusBar",
            "Blizzard_AccessibilityTemplates",
            "Blizzard_SettingsDefinitions_Shared",
            "Blizzard_SettingsDefinitions_Frame",
            "Blizzard_QuickKeybind",
            "Blizzard_SharedXMLGame",
            "Blizzard_FrameXMLBase",
            "Blizzard_ObjectAPI",
            "Blizzard_UIParent",
            "Blizzard_UIParentPanelManager",
            "Blizzard_UIPanelTemplates",
            "Blizzard_EditMode",
            "Blizzard_ItemButton",
            "Blizzard_FrameXMLUtil",
            "Blizzard_GarrisonBase",
            "Blizzard_GameTooltip",
            "Blizzard_MoneyFrame",
            "Blizzard_StaticPopup",
            "Blizzard_AutoComplete",
            "Blizzard_StaticPopup_Game",
            "Blizzard_TransmogShared",
            "Blizzard_FrameXML",
            "Blizzard_POIButton",
            "Blizzard_UIPanels_Game",
            "Blizzard_Flyout",
            "Blizzard_MicroMenu",
            "Blizzard_ActionBar",
            "Blizzard_Minimap",
            "Blizzard_ClassTrial",
            "Blizzard_CombatLogBase",
            "Blizzard_CombatLogProcessor",
            "Blizzard_CommunitiesSecure",
            "Blizzard_BuffFrame",
            "Blizzard_SpellDiminishUI",
            "Blizzard_UnitFrame",
            "Blizzard_TimerunningUtil",
            "Blizzard_ChatFrameBase",
            "Blizzard_VoiceToggleButton",
            "Blizzard_ChatFrame",
            "Blizzard_RestrictedAddOnEnvironment",
            "Blizzard_EnvironmentCleanup",
            "Blizzard_MapCanvasSecureUtil",
            "Blizzard_PingUI",
            "Blizzard_PrivateAurasUI",
            "Blizzard_SecureTransferUI",
            "Blizzard_SimpleCheckout",
            "Blizzard_WowTokenUI",
            "Blizzard_OverrideActionBar",
            "Blizzard_ActionBarController",
            "Blizzard_ActionStatus",
            "Blizzard_AddOnList",
            "Blizzard_AddOnPerformance",
            "Blizzard_SocialToast",
            "Blizzard_BNet",
            "Blizzard_CUFProfiles",
            "Blizzard_Channels",
            "Blizzard_ChatFrameUtil",
            "Blizzard_ClassMenu",
            "Blizzard_ClientSavedVariables",
            "Blizzard_CodeOfConduct",
            "Blizzard_CombatAudioAlerts",
            "Blizzard_CommandLineUtil",
            "Blizzard_GuildControlUI",
            "Blizzard_Communities",
            "Blizzard_RecentAllies",
            "Blizzard_FriendsFrame",
            "Blizzard_RaidFrame",
            "Blizzard_CompactRaidFrames",
            "Blizzard_Console",
            "Blizzard_UIFrameManager",
            "Blizzard_MawBuffs",
            "Blizzard_SpellSearch",
            "Blizzard_SharedTalentUI",
            "Blizzard_TieredEntranceTraits",
            "Blizzard_UIWidgets",
            "Blizzard_ObjectiveTracker",
            "Blizzard_ContentTracking",
            "Blizzard_CooldownViewer",
            "Blizzard_CovenantToasts",
            "Blizzard_DamageMeter",
            "Blizzard_DeclensionFrame",
            "Blizzard_PagedContent",
            "Blizzard_DelvesCompanionConfiguration",
            "Blizzard_DelvesToast",
            "Blizzard_Deprecated",
            "Blizzard_DeprecatedActionBar",
            "Blizzard_DeprecatedAutoComplete",
            "Blizzard_DeprecatedBattleNet",
            "Blizzard_DeprecatedChatInfo",
            "Blizzard_DeprecatedCombatLog",
            "Blizzard_DeprecatedCurrencyScript",
            "Blizzard_DeprecatedGlue",
            "Blizzard_DeprecatedGuildScript",
            "Blizzard_DeprecatedHousingCatalog",
            "Blizzard_DeprecatedInstanceEncounter",
            "Blizzard_DeprecatedItemScript",
            "Blizzard_DeprecatedItemSocketInfo",
            "Blizzard_DeprecatedLFG",
            "Blizzard_DeprecatedPetInfo",
            "Blizzard_DeprecatedPvpScript",
            "Blizzard_DeprecatedSoundScript",
            "Blizzard_DeprecatedSpecialization",
            "Blizzard_DeprecatedSpellBook",
            "Blizzard_DeprecatedSpellScript",
            "Blizzard_DeprecatedTradeInfo",
            "Blizzard_DeprecatedUnitScript",
            "Blizzard_DeprecatedWorldElapsedTimerTypes",
            "Blizzard_DurabilityFrame",
            "Blizzard_Deprecated_ArenaUI",
            "Blizzard_Dispatcher",
            "Blizzard_EncounterTimeline",
            "Blizzard_EncounterWarnings",
            "Blizzard_FrameEffects",
            "Blizzard_FrameStack",
            "Blizzard_FramerateFrame",
            "Blizzard_GameMenu",
            "Blizzard_GlobalFXModelScenes",
            "Blizzard_GroupFinder",
            "Blizzard_GuildRename",
            "Blizzard_HelpFrame",
            "Blizzard_HousingEventHandler",
            "Blizzard_HousingTemplates",
            "Blizzard_TutorialManager",
            "Blizzard_Tutorials",
            "Blizzard_HousingTutorials",
            "Blizzard_IME",
            "Blizzard_MailFrame",
            "Blizzard_MainMenuBarBagButtons",
            "Blizzard_MajorFactions",
            "Blizzard_MapCanvas",
            "Blizzard_MatchmakingQueueDisplay",
            "Blizzard_MirrorTimer",
            "Blizzard_MoneyReceipt",
            "Blizzard_NamePlates",
            "Blizzard_Notification",
            "Blizzard_PVPMatch",
            "Blizzard_PerformanceBar",
            "Blizzard_PersonalResourceDisplay",
            "Blizzard_PetBattleUI",
            "Blizzard_PhotoSharing",
            "Blizzard_QuestNavigation",
            "Blizzard_QueueStatusFrame",
            "Blizzard_QuickJoin",
            "Blizzard_RPE_TurnStrafe",
            "Blizzard_RecruitAFriend",
            "Blizzard_ReportFrameShared",
            "Blizzard_ReportFrame",
            "Blizzard_SavedSets",
            "Blizzard_ScriptErrorsFrame",
            "Blizzard_SharedMapDataProviders",
            "Blizzard_SharedWidgetFrames",
            "Blizzard_StableUI",
            "Blizzard_Subtitles",
            "Blizzard_TokenUI",
            "Blizzard_TransformManipulator",
            "Blizzard_UnitPopupShared",
            "Blizzard_UnitPopup",
            "Blizzard_WeeklyRewardsUtil",
            "Blizzard_WorldMap",
            "Blizzard_ZoneAbility",
        ];

        assert_eq!(
            names, expected,
            "Blizzard addon load order changed. If intentional, update the snapshot.\n\
             To regenerate: cargo test --test load_order dump_load_order -- --ignored --nocapture"
        );
    }
}

/// Helper to regenerate the snapshot above.
#[test]
#[ignore]
fn dump_load_order() {
    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons(&ui);
    for (name, _) in &addons {
        eprintln!("    \"{name}\",");
    }
    eprintln!("Total: {} addons", addons.len());
    panic!("dump complete — copy output into test_blizzard_addon_load_order_snapshot");
}

/// PaperDollItemSlotButton_OnLoad must exist when Blizzard_MainMenuBarBagButtons loads.
#[test]
fn test_paperdoll_onload_exists_for_bag_buttons() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        let ui = blizzard_ui_dir();
        let addons = discover_blizzard_addons(&ui);

        for (name, toc_path) in &addons {
            load_addon(&env.loader_env(), toc_path).ok();
            if name == "Blizzard_MainMenuBarBagButtons" {
                break;
            }
        }

        let exists: bool = env
            .eval("return type(PaperDollItemSlotButton_OnLoad) == 'function'")
            .unwrap_or(false);
        assert!(
            exists,
            "PaperDollItemSlotButton_OnLoad should be defined before Blizzard_MainMenuBarBagButtons loads"
        );
    }
}
