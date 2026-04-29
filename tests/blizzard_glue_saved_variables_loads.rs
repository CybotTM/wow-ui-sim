#![cfg(feature = "client-retail")]
use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, find_toc_file, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
}

fn glue_saved_variables_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_GlueSavedVariables")
}

fn glue_saved_variables_toc() -> PathBuf {
    glue_saved_variables_dir().join("Blizzard_GlueSavedVariables.toc")
}

#[test]
fn blizzard_glue_saved_variables_find_toc_resolves_bare_variant() {
    let resolved = find_toc_file(&glue_saved_variables_dir())
        .expect("Blizzard_GlueSavedVariables TOC should resolve");
    assert_eq!(
        resolved,
        glue_saved_variables_toc(),
        "Blizzard_GlueSavedVariables ships only a bare `Blizzard_GlueSavedVariables.toc` (no \
         `_Mainline.toc` / `_Classic.toc` flavor variants) — `find_toc_file` (src/loader/mod.rs:65) \
         falls through to the bare `.toc` suffix after the flavor-specific lookups miss"
    );
}

#[test]
fn blizzard_glue_saved_variables_toc_parses_minimal_metadata() {
    let toc = TocFile::from_file(&glue_saved_variables_toc())
        .expect("Blizzard_GlueSavedVariables TOC should parse");

    assert!(
        !toc.is_load_on_demand(),
        "Blizzard_GlueSavedVariables declares `## LoadOnDemand: 0` which is_load_on_demand() \
         (src/toc.rs:259) treats as false — the SavedVariables registration must run during the \
         normal glue-screen auto-discovery pass, not on-demand, so the 3 globals are restored \
         from disk before the consumer addons (Blizzard_GlueXML, ServerAlert.lua, \
         CharacterSelectTemplates.lua) read them"
    );
    assert!(
        !toc.is_load_first(),
        "Blizzard_GlueSavedVariables does not declare `## LoadFirst: 1` — it is just a \
         SavedVariables registration shell with no Lua/XML to publish, so no other addon depends \
         on its presence beyond the SavedVariables loader hooking up the 3 globals from disk"
    );
    assert!(
        !toc.is_secure_env(),
        "Blizzard_GlueSavedVariables does not declare `## UseSecureEnvironment` — there is no \
         executable code in this addon, so the secure-env distinction is moot"
    );
    assert!(
        toc.dependencies().is_empty(),
        "Blizzard_GlueSavedVariables declares no dependencies — the SavedVariables registration \
         is self-contained (the consumer addons declare their own deps separately)"
    );
}

#[test]
fn blizzard_glue_saved_variables_toc_declares_glue_screen_only() {
    let toc_text = std::fs::read_to_string(glue_saved_variables_toc())
        .expect("Blizzard_GlueSavedVariables TOC should read");
    assert!(
        toc_text.contains("## AllowLoad: Glue"),
        "Blizzard_GlueSavedVariables declares `## AllowLoad: Glue` (capital G — glue-screen-only). \
         The 3 globals it registers (g_collapsedServerAlert, g_characterSelectToolTrayCollapsed, \
         g_newGameModeAvailableAcknowledged) are read/written exclusively by the glue-screen \
         flow (server-alert dismissal, character-select tool-tray collapse state, new-game-mode \
         badge acknowledgement) — they have no game-screen consumers"
    );
    assert!(
        toc_text.contains("## LoadOnDemand: 0"),
        "Blizzard_GlueSavedVariables declares `## LoadOnDemand: 0` (literal `0`) — in_load_on_demand() \
         only treats `1` / `true` as on-demand, so `0` falls back to eager auto-loading"
    );
    assert!(
        !toc_text.contains("## AllowLoadGameType:"),
        "Blizzard_GlueSavedVariables omits `## AllowLoadGameType:` — the SavedVariables registration \
         is reused across mainline + classic flavors (the actual saved-variable values are stored \
         per-installation regardless of game type)"
    );
}

#[test]
fn blizzard_glue_saved_variables_toc_publishes_three_machine_specific_globals() {
    let toc = TocFile::from_file(&glue_saved_variables_toc())
        .expect("Blizzard_GlueSavedVariables TOC should parse");

    let mut declared = toc.saved_variables();
    declared.sort();
    assert_eq!(
        declared,
        vec![
            "g_characterSelectToolTrayCollapsed".to_string(),
            "g_collapsedServerAlert".to_string(),
            "g_newGameModeAvailableAcknowledged".to_string(),
        ],
        "Blizzard_GlueSavedVariables declares exactly 3 SavedVariablesMachine globals — \
         saved_variables() (src/toc.rs:316) folds the SavedVariablesMachine list into the same \
         accessor as SavedVariables, so all 3 round-trip through the same SavedVariables loader \
         path. Consumers: g_collapsedServerAlert is read/written by Blizzard_GlueXML/ServerAlert.lua \
         to remember which server alert text the player has dismissed; \
         g_characterSelectToolTrayCollapsed is set by \
         Blizzard_GlueXML/Mainline/CharacterSelect/CharacterSelectTemplates.lua's tool-tray \
         expand toggle and read by Blizzard_GlueXML/Mainline/CharacterSelect.lua to restore the \
         tool-tray collapse state on screen show; g_newGameModeAvailableAcknowledged tracks \
         whether the new-game-mode badge has been acknowledged"
    );

    assert!(
        toc.saved_variables_per_character().is_empty(),
        "Blizzard_GlueSavedVariables declares no SavedVariablesPerCharacter globals — all 3 \
         entries are SavedVariablesMachine (per-installation), since glue-screen state is shared \
         across all characters on the machine"
    );
}

#[test]
fn blizzard_glue_saved_variables_toc_lists_zero_files() {
    let toc = TocFile::from_file(&glue_saved_variables_toc())
        .expect("Blizzard_GlueSavedVariables TOC should parse");
    assert!(
        toc.files.is_empty(),
        "Blizzard_GlueSavedVariables enumerates zero source files — the TOC is purely a \
         SavedVariables-registration shell. The 3 globals are populated by the SavedVariables \
         loader from disk during startup; the consumer addons (Blizzard_GlueXML's ServerAlert.lua \
         + CharacterSelectTemplates.lua) read/write them directly with `g_x = g_x or nil` \
         fallbacks. Got: {:?}",
        toc.files
    );
}

#[test]
fn blizzard_glue_saved_variables_directory_ships_only_the_toc() {
    let dir = glue_saved_variables_dir();
    let mut entries: Vec<String> = std::fs::read_dir(&dir)
        .expect("Blizzard_GlueSavedVariables directory should exist")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    entries.sort();
    assert_eq!(
        entries,
        vec!["Blizzard_GlueSavedVariables.toc".to_string()],
        "Blizzard_GlueSavedVariables directory ships exactly one entry: the bare TOC. There is \
         no Lua, no XML, no localization, no Mainline/Classic subdirectory — this addon exists \
         solely to register 3 SavedVariablesMachine globals with the SavedVariables loader"
    );
}

#[test]
fn blizzard_glue_saved_variables_appears_in_all_three_glue_screen_discoveries() {
    let ui = blizzard_ui_dir();
    for screen in [
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        let addons = discover_blizzard_addons_for_screen(&ui, screen);
        let discovered = addons
            .iter()
            .any(|(name, _)| name == "Blizzard_GlueSavedVariables");
        assert!(
            discovered,
            "Blizzard_GlueSavedVariables should appear in {screen:?} auto-discovery — \
             `## AllowLoad: Glue` covers all three glue screens (Login + CharacterSelect + \
             CharacterCreate). The SavedVariables persistence must be active on every glue \
             screen so the 3 globals stay live across screen transitions"
        );
    }
}

#[test]
fn blizzard_glue_saved_variables_absent_from_game_screen_discovery() {
    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);
    let discovered = addons
        .iter()
        .any(|(name, _)| name == "Blizzard_GlueSavedVariables");
    assert!(
        !discovered,
        "Blizzard_GlueSavedVariables MUST NOT appear in Game-screen auto-discovery — \
         `## AllowLoad: Glue` is glue-only, and the 3 globals it registers have no game-screen \
         consumers (the in-game equivalents like CHANNELPULLOUT_FADEFRAMES live in \
         Blizzard_ClientSavedVariables instead)"
    );
}

#[test]
fn blizzard_glue_saved_variables_loads_without_lua_errors() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::CharacterSelect);
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }
    wow_ui_sim::xml::register_intrinsic_templates();

    load_addon(&env.loader_env(), &glue_saved_variables_toc())
        .expect("Blizzard_GlueSavedVariables should load via Rust loader");

    let lua_errors: Vec<String> = env.state().borrow().lua_errors.clone();
    let related: Vec<&String> = lua_errors
        .iter()
        .filter(|e| {
            e.contains("GlueSavedVariables")
                || e.contains("g_collapsedServerAlert")
                || e.contains("g_characterSelectToolTrayCollapsed")
                || e.contains("g_newGameModeAvailableAcknowledged")
        })
        .collect();
    assert!(
        related.is_empty(),
        "Blizzard_GlueSavedVariables emitted addon-specific Lua errors during load:\n  {}",
        related
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}
