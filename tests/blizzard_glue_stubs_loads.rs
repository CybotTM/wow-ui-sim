#![cfg(feature = "client-retail")]
use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
}

fn glue_stubs_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_GlueStubs")
}

fn glue_stubs_toc() -> PathBuf {
    glue_stubs_dir().join("Blizzard_GlueStubs.toc")
}

fn glue_stubs_xml() -> PathBuf {
    glue_stubs_dir().join("GlueStubs.xml")
}

fn load_character_select_screen() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::CharacterSelect);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    wow_ui_sim::xml::register_intrinsic_templates();

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::CharacterSelect);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("[load {name}] FAILED: {err}"));
    }

    env.apply_post_load_workarounds();
    settle_headless_startup(&env);
    env
}

#[test]
fn blizzard_glue_stubs_toc_declares_glue_only_mainline_with_glue_xml_dep() {
    let toc = TocFile::from_file(&glue_stubs_toc()).expect("Blizzard_GlueStubs TOC should parse");
    assert!(
        !toc.is_load_on_demand(),
        "Blizzard_GlueStubs is non-LoadOnDemand — the empty-virtual-template stubs must register \
         eagerly on the glue-screen discovery pass before any glue XML inherits from \
         ActionButtonTemplate / EditModeChatFrameSystemTemplate"
    );
    assert!(
        !toc.is_load_first(),
        "Blizzard_GlueStubs does not declare `## LoadFirst: 1` — the stubs are consumed only by \
         downstream inheritance lookups, not by the early-glue boot path"
    );
    assert!(
        !toc.is_secure_env(),
        "Blizzard_GlueStubs does not declare `## UseSecureEnvironment` — the stubs are pure XML \
         template placeholders with no executable code"
    );
    assert_eq!(
        toc.dependencies(),
        vec!["Blizzard_GlueXML".to_string()],
        "Blizzard_GlueStubs declares exactly one dep: Blizzard_GlueXML — so the stubs register \
         AFTER the bulk of glue-screen XML has already loaded, ensuring any inheritance lookup \
         that escapes through to Game-screen-only template names finds the empty placeholder \
         rather than erroring on a missing template"
    );
}

#[test]
fn blizzard_glue_stubs_toc_declares_glue_screen_mainline_only() {
    let toc_text =
        std::fs::read_to_string(glue_stubs_toc()).expect("Blizzard_GlueStubs TOC should read");
    assert!(
        toc_text.contains("## AllowLoad: glue"),
        "Blizzard_GlueStubs declares `## AllowLoad: glue` (lowercase variant — \
         allows_screen() (src/toc.rs:305) lowercases the value before matching, so both `glue` \
         and `Glue` route to the glue-screen branch). The placeholder templates are needed only \
         on glue screens where the Game-screen-only Blizzard_ActionBar / Blizzard_EditMode \
         addons that own the real ActionButtonTemplate / EditModeChatFrameSystemTemplate \
         definitions are not loaded"
    );
    assert!(
        toc_text.contains("## AllowLoadGameType: mainline"),
        "Blizzard_GlueStubs declares `## AllowLoadGameType: mainline` so the addon loads on \
         retail only — Classic builds use a different inheritance chain that does not need \
         these specific stubs"
    );
    assert!(
        toc_text.contains("## DefaultState: enabled"),
        "Blizzard_GlueStubs declares `## DefaultState: enabled` — the stub registration must \
         always be active so glue XML inheritance never errors on a missing template"
    );
}

#[test]
fn blizzard_glue_stubs_toc_lists_only_glue_stubs_xml() {
    let toc = TocFile::from_file(&glue_stubs_toc()).expect("Blizzard_GlueStubs TOC should parse");
    let files: Vec<String> = toc
        .files
        .iter()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .collect();
    assert_eq!(
        files,
        vec!["GlueStubs.xml".to_string()],
        "Blizzard_GlueStubs ships exactly one source file (GlueStubs.xml) — there is no Lua, no \
         localization, no Mainline/Classic subdirectory. The single XML body declares 2 \
         empty-virtual-template stubs"
    );
}

#[test]
fn blizzard_glue_stubs_directory_ships_only_toc_and_xml() {
    let dir = glue_stubs_dir();
    let mut entries: Vec<String> = std::fs::read_dir(&dir)
        .expect("Blizzard_GlueStubs directory should exist")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    entries.sort();
    assert_eq!(
        entries,
        vec![
            "Blizzard_GlueStubs.toc".to_string(),
            "GlueStubs.xml".to_string(),
        ],
        "Blizzard_GlueStubs directory ships exactly 2 entries: the TOC + the single XML file. \
         No Lua, no localization, no flavor variants"
    );
}

#[test]
fn blizzard_glue_stubs_xml_declares_two_empty_virtual_templates() {
    let xml_text = std::fs::read_to_string(glue_stubs_xml())
        .expect("Blizzard_GlueStubs/GlueStubs.xml should read");
    assert!(
        xml_text.contains(r#"<Button name="ActionButtonTemplate" virtual="true"/>"#),
        "GlueStubs.xml line 3 declares `<Button name=\"ActionButtonTemplate\" virtual=\"true\"/>` \
         — an empty virtual template stub. The real ActionButtonTemplate is defined by \
         Blizzard_ActionBar/Mainline/ActionButtonTemplate.xml on the Game screen. The glue-screen \
         stub exists so XML inheritance lookups that escape from glue XML to a Game-screen-only \
         template name don't error during glue load"
    );
    assert!(
        xml_text.contains(r#"<Frame name="EditModeChatFrameSystemTemplate" virtual="true"/>"#),
        "GlueStubs.xml line 4 declares `<Frame name=\"EditModeChatFrameSystemTemplate\" \
         virtual=\"true\"/>` — an empty virtual template stub. The real \
         EditModeChatFrameSystemTemplate is defined by \
         Blizzard_EditMode/Shared/EditModeSystemTemplates.xml line 161 (inheriting from \
         EditModeSystemTemplate with EditModeChatFrameSystemMixin). The glue-screen stub provides \
         a placeholder so glue-screen XML doesn't error when EditMode-related inheritance is \
         walked"
    );
}

#[test]
fn blizzard_glue_stubs_appears_in_all_three_glue_screen_discoveries() {
    let ui = blizzard_ui_dir();
    for screen in [
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        let addons = discover_blizzard_addons_for_screen(&ui, screen);
        let discovered = addons.iter().any(|(name, _)| name == "Blizzard_GlueStubs");
        assert!(
            discovered,
            "Blizzard_GlueStubs should appear in {screen:?} auto-discovery — `## AllowLoad: glue` \
             covers all three glue screens. The stub templates must be registered on every glue \
             screen so any glue XML inheritance lookup resolves cleanly"
        );
    }
}

#[test]
fn blizzard_glue_stubs_absent_from_game_screen_discovery() {
    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);
    let discovered = addons.iter().any(|(name, _)| name == "Blizzard_GlueStubs");
    assert!(
        !discovered,
        "Blizzard_GlueStubs MUST NOT appear in Game-screen auto-discovery — `## AllowLoad: glue` \
         is glue-only. On the Game screen the real ActionButtonTemplate (from Blizzard_ActionBar) \
         and EditModeChatFrameSystemTemplate (from Blizzard_EditMode) are loaded with their \
         actual bodies; the glue stubs would clobber those if installed on the Game screen"
    );
}

#[test]
fn blizzard_glue_stubs_loads_via_loader_without_addon_specific_errors() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::CharacterSelect);
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }
    wow_ui_sim::xml::register_intrinsic_templates();

    load_addon(&env.loader_env(), &glue_stubs_toc())
        .expect("Blizzard_GlueStubs should load via Rust loader");

    let lua_errors: Vec<String> = env.state().borrow().lua_errors.clone();
    let related: Vec<&String> = lua_errors
        .iter()
        .filter(|e| {
            e.contains("GlueStubs")
                || e.contains("ActionButtonTemplate")
                || e.contains("EditModeChatFrameSystemTemplate")
        })
        .collect();
    assert!(
        related.is_empty(),
        "Blizzard_GlueStubs emitted addon-specific Lua errors during load:\n  {}",
        related
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}

#[test]
fn blizzard_glue_stubs_registers_action_button_template_in_template_registry() {
    let _env = load_character_select_screen();

    assert!(
        wow_ui_sim::xml::get_template("ActionButtonTemplate").is_some(),
        "After CharacterSelect-screen load, `ActionButtonTemplate` should be present in the XML \
         template registry — Blizzard_GlueStubs/GlueStubs.xml line 3 registers an empty virtual \
         template under that name as a placeholder for glue-screen XML inheritance lookups"
    );
}

#[test]
fn blizzard_glue_stubs_registers_edit_mode_chat_frame_system_template_in_template_registry() {
    let _env = load_character_select_screen();

    assert!(
        wow_ui_sim::xml::get_template("EditModeChatFrameSystemTemplate").is_some(),
        "After CharacterSelect-screen load, `EditModeChatFrameSystemTemplate` should be present \
         in the XML template registry — Blizzard_GlueStubs/GlueStubs.xml line 4 registers an \
         empty virtual template under that name as a placeholder for glue-screen XML \
         inheritance lookups"
    );
}

#[test]
fn blizzard_glue_stubs_does_not_leak_virtual_templates_as_globals() {
    let env = load_character_select_screen();

    for template in ["ActionButtonTemplate", "EditModeChatFrameSystemTemplate"] {
        let leaked: bool = env
            .eval(&format!("return _G['{template}'] ~= nil"))
            .expect("global-template query should succeed");
        assert!(
            !leaked,
            "Stub virtual template `{template}` (declared with `virtual=\"true\"`) must not leak \
             as a `_G` global — it is only registered in the XML template registry for \
             inheritance and CreateFrame template lookup. A leak indicates the XML loader \
             incorrectly materialized a runtime frame for a virtual definition"
        );
    }
}
