mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn load_blizzard_screen(screen: ScreenKind) -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(screen);

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, screen);
    for (name, toc_path) in &addons {
        if let Err(err) = load_addon(&env.loader_env(), toc_path) {
            panic!("[load {name}] FAILED: {err}");
        }
    }

    env.apply_post_load_workarounds();
    fire_startup_events_for_screen(&env, screen);
    env
}

#[test]
fn character_select_screen_shows_character_select_not_login() {
    test_timeout! {
        let env = load_blizzard_screen(ScreenKind::CharacterSelect);

        let init_errors: Vec<String> = env
            .state()
            .borrow()
            .lua_errors
            .iter()
            .filter(|msg| msg.contains("InitializeCharacterScreenData"))
            .cloned()
            .collect();
        assert!(
            init_errors.is_empty(),
            "character select boot should not error on InitializeCharacterScreenData: {init_errors:#?}"
        );

        let character_select_visible: bool = env
            .eval("return CharacterSelect ~= nil and CharacterSelect:IsShown()")
            .expect("CharacterSelect visibility should be queryable");
        assert!(
            character_select_visible,
            "character-select screen should show CharacterSelect"
        );

        let account_login_visible: bool = env
            .eval("return AccountLogin ~= nil and AccountLogin:IsShown()")
            .expect("AccountLogin visibility should be queryable");
        assert!(
            !account_login_visible,
            "character-select screen should not fall back to AccountLogin"
        );
    }
}

#[test]
fn character_select_boot_skips_core_missing_stub_errors() {
    test_timeout! {
        let env = load_blizzard_screen(ScreenKind::CharacterSelect);

        let errors = env.state().borrow().lua_errors.clone();
        let unexpected: Vec<String> = errors
            .into_iter()
            .filter(|msg| {
                msg.contains("KIOSK_ENABLED")
                    || msg.contains("ACCOUNT_DATA_INITIALIZED")
                    || msg.contains("CHARACTER_LIST_UPDATE")
                    || msg.contains("ACCOUNT_CHARACTER_LIST_RECIEVED")
                    || msg.contains("SHOULD_RECONNECT_TO_REALM_LIST")
                    || msg.contains("UPDATE_REALM_NAME_FOR_GUID")
                    || msg.contains("UPDATE_SELECTED_CHARACTER")
                    || msg.contains("CHAR_RESTORE_COMPLETE")
                    || msg.contains("RACE_FACTION_CHANGE_STARTED")
                    || msg.contains("CHARACTER_LIST_GROUP_CREATED")
                    || msg.contains("CHARACTER_LIST_RESTRICTIONS_RECEIVED")
                    || msg.contains("MAP_SCENE_CHARACTER_ON_MOUSE_ENTER")
                    || msg.contains("FORCE_RENAME_CHARACTER")
                    || msg.contains("ACCOUNT_DATA_RESTORED")
                    || msg.contains("RACE_FACTION_CHANGE_RESULT")
                    || msg.contains("CHAR_RENAME_IN_PROGRESS")
                    || msg.contains("KEY_BINDINGS_COPY_COMPLETE")
                    || msg.contains("CUSTOMIZE_CHARACTER_STARTED")
                    || msg.contains("SetWorldFrameStrata")
                    || msg.contains("GetMaxWarbandGroupCount")
                    || msg.contains("SetCharSelectModelFrame")
                    || msg.contains("SetCharSelectMapSceneFrame")
                    || msg.contains("SetInCharacterSelect")
                    || msg.contains("HasQueuedUpgrade")
                    || msg.contains("GetNumCharacters")
                    || msg.contains("CheckCharacterUndeleteCooldown")
                    || msg.contains("GetServerName")
                    || msg.contains("IsTimerunningEnabled")
                    || msg.contains("IsTrialBoostEnabled")
                    || msg.contains("GetCharacterTimerunningSeasonID")
                    || msg.contains("IsCharacterTimerunningConversionAllowed")
                    || msg.contains("GetLiveRegionCharacterCopySourceRegions")
                    || msg.contains("IsTimerunningSeasonActive")
                    || msg.contains("RequestAutoRealmJoin")
                    || msg.contains("RequestChangeRealmList")
                    || msg.contains("GetQueuedUpgradeGUID")
                    || msg.contains("ClearQueuedUpgrade")
                    || msg.contains("ApplyLevelUp")
                    || msg.contains("AssignUpgradeDistribution")
                    || msg.contains("DoesGUIDHavePendingFactionChange")
                    || msg.contains("GetSelectBackgroundModel")
                    || msg.contains("GetActiveClassTrialBoostType")
                    || msg.contains("GetAutomaticBoost")
                    || msg.contains("GetAutomaticBoostCharacter")
                    || msg.contains("GetCharacterServiceDisplayDataByVASType")
                    || msg.contains("CharacterCreateType")
                    || msg.contains("GetFactionGroupByIndex")
                    || msg.contains("RequestManualUnrevoke")
                    || msg.contains("SetAutomaticBoost")
                    || msg.contains("SetAutomaticBoostCharacter")
                    || msg.contains("TrialBoostCharacter")
                    || msg.contains("GetCharacterCreateType")
                    || msg.contains("GetRaceDataByID")
                    || msg.contains("GetRaceIDFromName")
            })
            .collect();

        assert!(
            unexpected.is_empty(),
            "character-select boot should not hit the current core stub/event gaps: {unexpected:#?}"
        );
    }
}

#[test]
fn character_select_hides_chat_frames() {
    test_timeout! {
        let env = load_blizzard_screen(ScreenKind::CharacterSelect);

        let chat_frame_visible: bool = env
            .eval("return ChatFrame1 ~= nil and ChatFrame1:IsShown()")
            .expect("ChatFrame1 visibility should be queryable");
        assert!(
            !chat_frame_visible,
            "character-select screen should not show the front-end chat frame"
        );

        let chat_dock_visible: bool = env
            .eval("return GeneralDockManager ~= nil and GeneralDockManager:IsShown()")
            .expect("GeneralDockManager visibility should be queryable");
        assert!(
            !chat_dock_visible,
            "character-select screen should not show the chat dock"
        );
    }
}

#[test]
fn character_select_populates_character_roster() {
    test_timeout! {
        let env = load_blizzard_screen(ScreenKind::CharacterSelect);

        let num_characters: i32 = env
            .eval("return GetNumCharacters()")
            .expect("GetNumCharacters should be queryable");
        assert!(
            num_characters > 0,
            "character-select screen should expose at least one character"
        );

        let first_guid: Option<String> = env
            .eval("return GetCharacterGUID(1)")
            .expect("GetCharacterGUID(1) should be queryable");
        assert!(
            first_guid.is_some(),
            "character-select screen should expose a guid for the first character"
        );

        let first_name: Option<String> = env
            .eval(
                "local info = CharacterSelectUtil and CharacterSelectUtil.GetCharacterInfoTable(1); return info and info.name or nil",
            )
            .expect("CharacterSelectUtil.GetCharacterInfoTable(1) should be queryable");
        assert!(
            first_name.is_some(),
            "character-select screen should expose info for the first character"
        );
    }
}

#[test]
fn character_select_builds_scrollbox_entries() {
    test_timeout! {
        let env = load_blizzard_screen(ScreenKind::CharacterSelect);

        let entry_count: i32 = env
            .eval(
                r#"
                local count = 0
                if CharacterSelectCharacterFrame and CharacterSelectCharacterFrame.ScrollBox then
                    for _ in CharacterSelectCharacterFrame.ScrollBox:EnumerateDataProviderEntireRange() do
                        count = count + 1
                    end
                end
                return count
                "#,
            )
            .expect("character list data provider should be enumerable");
        assert!(
            entry_count > 0,
            "character-select screen should populate the character list"
        );
    }
}
