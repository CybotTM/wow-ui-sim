#![cfg(feature = "client-retail")]
use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
}

fn declension_frame_glue_toc() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_DeclensionFrameGlue/Blizzard_DeclensionFrameGlue_Mainline.toc")
}

fn load_login_screen() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Login);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    wow_ui_sim::xml::register_intrinsic_templates();

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::Login);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("[load {name}] FAILED: {err}"));
    }

    env.apply_post_load_workarounds();
    env
}

#[test]
fn blizzard_declension_frame_glue_toc_declares_gluexml_dep_and_glue_only() {
    let toc = TocFile::from_file(&declension_frame_glue_toc())
        .expect("Blizzard_DeclensionFrameGlue_Mainline TOC should parse");
    assert!(
        !toc.is_load_on_demand(),
        "Blizzard_DeclensionFrameGlue is non-LOD — the glue-screen mainline stub auto-loads \
         after Blizzard_GlueXML so locale-specific overrides can install their declension UI \
         on the character-create / character-select screens"
    );
    assert!(
        !toc.is_secure_env(),
        "Blizzard_DeclensionFrameGlue does not declare UseSecureEnvironment"
    );
    let deps = toc.dependencies();
    assert!(
        deps.contains(&"Blizzard_GlueXML".to_string()),
        "Blizzard_DeclensionFrameGlue should declare `## Dependencies: Blizzard_GlueXML` (the \
         glue counterpart of Blizzard_UIParent — the locale overrides will reparent their \
         declension dialog to GlueParent / CharacterCreateFrame), got {deps:?}"
    );

    let toc_text = std::fs::read_to_string(declension_frame_glue_toc())
        .expect("Blizzard_DeclensionFrameGlue TOC should read");
    assert!(
        toc_text.contains("## AllowLoad: glue"),
        "Blizzard_DeclensionFrameGlue declares `## AllowLoad: glue` (glue-screen-only — does \
         NOT load on the in-game UIParent screen; that's Blizzard_DeclensionFrame's job)"
    );
    assert!(
        toc_text.contains("## AllowLoadGameType: mainline"),
        "Blizzard_DeclensionFrameGlue declares `## AllowLoadGameType: mainline` (Classic \
         flavors ship their own DeclensionFrameGlue variants — only mainline retail loads \
         this stub)"
    );
}

#[test]
fn blizzard_declension_frame_glue_appears_in_login_screen_discovery() {
    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Login);
    let in_login = addons
        .iter()
        .any(|(name, _)| name == "Blizzard_DeclensionFrameGlue");
    assert!(
        in_login,
        "Blizzard_DeclensionFrameGlue (non-LOD with `## AllowLoad: glue`) should appear in \
         Login-screen auto-discovery so the locale-specific overrides have a base addon to \
         attach to during the character-select / character-create flow where ruRU / koKR / \
         zhCN players name pets and characters"
    );
}

#[test]
fn blizzard_declension_frame_glue_is_absent_from_game_screen_discovery() {
    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    let in_game = addons
        .iter()
        .any(|(name, _)| name == "Blizzard_DeclensionFrameGlue");
    assert!(
        !in_game,
        "Blizzard_DeclensionFrameGlue carries `## AllowLoad: glue` so it must NOT appear in \
         Game-screen auto-discovery — the Game-screen counterpart is Blizzard_DeclensionFrame"
    );
}

#[test]
fn blizzard_declension_frame_glue_appears_in_character_select_discovery() {
    let addons =
        discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::CharacterSelect);
    let in_char_select = addons
        .iter()
        .any(|(name, _)| name == "Blizzard_DeclensionFrameGlue");
    assert!(
        in_char_select,
        "Blizzard_DeclensionFrameGlue should also appear in CharacterSelect-screen \
         auto-discovery — the screen where pet / character names are first declined"
    );
}

#[test]
fn blizzard_declension_frame_glue_loads_via_explicit_load_without_errors() {
    let env = load_login_screen();

    let glue_errors: Vec<String> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| message.contains("DeclensionFrameGlue"))
        .cloned()
        .collect();
    assert!(
        glue_errors.is_empty(),
        "Blizzard_DeclensionFrameGlue emitted Lua errors during Login-screen load:\n  {}",
        glue_errors.join("\n  ")
    );
}

#[test]
fn blizzard_declension_frame_glue_mainline_stub_files_are_intentionally_empty() {
    let lua_path =
        blizzard_ui_dir().join("Blizzard_DeclensionFrameGlue/Mainline/DeclensionFrame.lua");
    let xml_path =
        blizzard_ui_dir().join("Blizzard_DeclensionFrameGlue/Mainline/DeclensionFrame.xml");

    let lua_text = std::fs::read_to_string(&lua_path)
        .expect("Mainline/DeclensionFrame.lua (Glue) should read");
    let xml_text = std::fs::read_to_string(&xml_path)
        .expect("Mainline/DeclensionFrame.xml (Glue) should read");

    assert!(
        lua_text.contains("Overridden by the locale-specific versions"),
        "Glue Mainline/DeclensionFrame.lua is a single-line placeholder comment by design — \
         the real glue-side declension UI ships only with locale builds. Got:\n{lua_text}"
    );
    assert!(
        xml_text.contains("Overridden by the locale-specific versions"),
        "Glue Mainline/DeclensionFrame.xml is a body-less <Ui> document by design — the real \
         glue-side declension XML ships only with locale builds. Got:\n{xml_text}"
    );
}
