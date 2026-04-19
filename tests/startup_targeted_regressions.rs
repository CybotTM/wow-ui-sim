mod common;

use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn load_and_startup_collect_messages() -> Vec<String> {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons(&ui);
    let mut messages = Vec::new();

    for (name, toc_path) in &addons {
        match load_addon(&env.loader_env(), toc_path) {
            Ok(result) => {
                for warning in result.warnings {
                    messages.push(format!("[load {name}] {warning}"));
                }
            }
            Err(error) => messages.push(format!("[load {name}] FAILED: {error}")),
        }
    }

    env.apply_post_load_workarounds();
    common::install_error_collector(&env, "__targeted_startup_errors");

    common::fire_addon_loaded(&env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        env.fire_event(event).ok();
        messages.extend(common::drain_string_table(
            &env,
            "__targeted_startup_errors",
        ));
    }

    env.fire_edit_mode_layouts_updated().ok();
    messages.extend(common::drain_string_table(
        &env,
        "__targeted_startup_errors",
    ));

    common::call_global_if_present(&env, "RequestTimePlayed");
    common::fire_player_entering_world(&env, true, false);
    messages.extend(common::drain_string_table(
        &env,
        "__targeted_startup_errors",
    ));

    for event in [
        "UNIT_AURA",
        "BAG_UPDATE_DELAYED",
        "QUEST_LOG_UPDATE",
        "GROUP_ROSTER_UPDATE",
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
        "UPDATE_CHAT_WINDOWS",
    ] {
        env.fire_event(event).ok();
        messages.extend(common::drain_string_table(
            &env,
            "__targeted_startup_errors",
        ));
    }

    env.fire_on_update(0.016).ok();
    messages.extend(common::drain_string_table(
        &env,
        "__targeted_startup_errors",
    ));
    messages
}

#[test]
fn startup_omits_targeted_missing_global_errors() {
    test_timeout! {
        let messages = load_and_startup_collect_messages();
        let targeted: Vec<String> = messages
            .into_iter()
            .filter(|message| {
                message.contains("GetQuestLink")
                    || message.contains("GetWorldPVPQueueStatus")
                    || message.contains("CanHearthAndResurrectFromArea")
                    || message.contains("UnitIsOtherPlayersPet")
                    || message.contains("SupportsClipCursor")
                    || message.contains("GetNumBattlefieldFlagPositions")
                    || message.contains("GetWorldMapActionButtonSpellInfo")
                    || message.contains("PlayerIsPVPInactive")
                    || message.contains("GetMouseFoci")
                    || message.contains("QuestOfferDataProvider.lua:174")
                    || message.contains("ContentTrackingDataProvider.lua:51")
                    || message.contains("DigSiteDataProvider.lua:17")
                    || message.contains("GarrisonPlotDataProvider.lua:12")
                    || message.contains("DungeonEntranceDataProvider.lua:34")
                    || message.contains("BannerDataProvider.lua:12")
                    || message.contains("MapLinkDataProvider.lua:12")
                    || message.contains("SelectableGraveyardDataProvider.lua:30")
                    || message.contains("AreaPOIEventDataProvider.lua:46")
                    || message.contains("DelveEntranceDataProvider.lua:32")
                    || message.contains("EncounterJournalDataProvider.lua:35")
                    || message.contains("invalid script handler 'OnCooldownDone'")
                    || message.contains("attempt to index field 'savedVars' (a nil value)")
                    || message.contains("attempt to call local '(for index)'")
                    || message.contains("attempt to call global 'date'")
            })
            .collect();

        assert!(
            targeted.is_empty(),
            "Startup should not report the targeted missing-global regressions:\n  {}",
            targeted.join("\n  ")
        );
    }
}

#[test]
fn blizzard_console_saved_variables_machine_seed_without_saved_vars_manager() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);

        let toc_path = blizzard_ui_dir().join("Blizzard_Console/Blizzard_Console.toc");
        load_addon(&env.loader_env(), &toc_path)
            .expect("Blizzard_Console should load without a saved vars manager");

        let saved_vars_type: String = env
            .eval("return type(Blizzard_Console_SavedVars)")
            .expect("saved vars probe should run");

        assert_eq!(
            saved_vars_type, "table",
            "SavedVariablesMachine globals should still be seeded when persistence is disabled"
        );
    }
}

#[test]
fn damage_meter_saved_variables_default_without_partial_empty_seed() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);

        let edit_mode_toc = blizzard_ui_dir().join("Blizzard_EditMode/Blizzard_EditMode.toc");
        load_addon(&env.loader_env(), &edit_mode_toc).expect("Blizzard_EditMode should load");

        let toc_path = blizzard_ui_dir().join("Blizzard_DamageMeter/Blizzard_DamageMeter.toc");
        load_addon(&env.loader_env(), &toc_path)
            .expect("Blizzard_DamageMeter should load without a saved vars manager");

        let (saved_vars_type, window_data_list_type): (String, String) = env
            .eval(
                r#"
                local saved_type = type(DamageMeterPerCharacterSettings)
                local list_type = "missing"
                if saved_type == "table" then
                    list_type = type(DamageMeterPerCharacterSettings.windowDataList)
                end
                return saved_type, list_type
                "#,
            )
            .expect("damage meter saved vars probe should run");

        assert!(
            saved_vars_type == "nil" || window_data_list_type == "table",
            "DamageMeter saved vars should stay nil or expose windowDataList, not a partially-seeded table"
        );
    }
}
