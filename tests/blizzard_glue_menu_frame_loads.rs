#![cfg(feature = "client-retail")]
use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, find_toc_file, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
}

fn glue_menu_frame_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_GlueMenuFrame")
}

fn glue_menu_frame_mainline_toc() -> PathBuf {
    glue_menu_frame_dir().join("Blizzard_GlueMenuFrame_Mainline.toc")
}

fn glue_menu_frame_classic_toc() -> PathBuf {
    glue_menu_frame_dir().join("Blizzard_GlueMenuFrame_Classic.toc")
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
fn blizzard_glue_menu_frame_find_toc_picks_mainline_variant() {
    let resolved =
        find_toc_file(&glue_menu_frame_dir()).expect("Blizzard_GlueMenuFrame TOC should resolve");
    assert_eq!(
        resolved,
        glue_menu_frame_mainline_toc(),
        "Blizzard_GlueMenuFrame ships both `_Mainline.toc` and `_Classic.toc` variants — \
         `find_toc_file` (src/loader/mod.rs:65) prefers the `_Mainline.toc` suffix on the first \
         pass so the simulator (which targets retail mainline) ignores the Classic-flavor TOC \
         that swaps Blizzard_GlueXMLBase for Blizzard_LoginWarningDialogs / \
         Blizzard_CharacterSelectNavBar"
    );
}

#[test]
fn blizzard_glue_menu_frame_mainline_toc_declares_load_first_glue_with_glue_xml_base_dep() {
    let toc = TocFile::from_file(&glue_menu_frame_mainline_toc())
        .expect("Blizzard_GlueMenuFrame_Mainline TOC should parse");
    assert!(
        !toc.is_load_on_demand(),
        "Blizzard_GlueMenuFrame is non-LoadOnDemand — the menu frame must exist before any \
         glue-screen Lua tries to call GlueMenuFrameUtil.ToggleMenu (e.g. ESC keybinding)"
    );
    assert!(
        toc.is_load_first(),
        "Blizzard_GlueMenuFrame declares `## LoadFirst: 1` so it loads before the bulk of the \
         glue-screen addons — Blizzard_GlueXML's CharacterSelect.lua / AccountLogin.lua call \
         `GlueMenuFrame:Hide()` directly during their OnLoad path, so the GlueMenuFrame global \
         must already be defined when those addons' XML instantiates"
    );
    assert!(
        !toc.is_secure_env(),
        "Blizzard_GlueMenuFrame does not declare UseSecureEnvironment — glue-screen menus do \
         not interact with the protected combat surface"
    );
    let deps = toc.dependencies();
    assert_eq!(
        deps,
        vec![
            "Blizzard_GlueXMLBase".to_string(),
            "Blizzard_GlueParent".to_string(),
        ],
        "Blizzard_GlueMenuFrame_Mainline declares exactly two deps: Blizzard_GlueXMLBase \
         (provides GlueDialog / SocialContract / TimerunningSelect / WarningDialog support) \
         and Blizzard_GlueParent (provides the `GlueParent` parent frame the GlueMenuFrame \
         attaches to via `parent=GlueParent`, plus GlueParent_AddModalFrame / \
         GlueParent_GetCurrentScreen / GlueParent_ShowOptionsScreen used by the mixin)"
    );
}

#[test]
fn blizzard_glue_menu_frame_mainline_toc_declares_glue_screen_and_mainline_only() {
    let toc_text = std::fs::read_to_string(glue_menu_frame_mainline_toc())
        .expect("Blizzard_GlueMenuFrame_Mainline TOC should read");
    assert!(
        toc_text.contains("## AllowLoad: Glue"),
        "Blizzard_GlueMenuFrame_Mainline declares `## AllowLoad: Glue` (capital G) so the \
         addon appears in glue-screen auto-discovery only — the in-game ESC menu equivalent \
         is Blizzard_GameMenuUI / GameMenuFrame, an entirely different surface"
    );
    assert!(
        toc_text.contains("## AllowLoadGameType: mainline"),
        "Blizzard_GlueMenuFrame_Mainline declares `## AllowLoadGameType: mainline` — Classic \
         flavors load the sibling `_Classic.toc` instead, which hooks a different dep chain"
    );
    assert!(
        toc_text.contains("## DefaultState: enabled"),
        "Blizzard_GlueMenuFrame declares `## DefaultState: enabled` so the addon is enabled by \
         default in the addon manager — the ESC menu is core glue-screen functionality"
    );
}

#[test]
fn blizzard_glue_menu_frame_classic_toc_swaps_dependency_chain_and_uses_blizzard_glue_xml_title() {
    let toc = TocFile::from_file(&glue_menu_frame_classic_toc())
        .expect("Blizzard_GlueMenuFrame_Classic TOC should parse");
    let deps = toc.dependencies();
    assert_eq!(
        deps,
        vec![
            "Blizzard_LoginWarningDialogs".to_string(),
            "Blizzard_CharacterSelectNavBar".to_string(),
        ],
        "The Classic-flavor TOC swaps the dep chain entirely — Classic builds do not ship \
         Blizzard_GlueXMLBase (the Mainline-only base) so the menu frame instead pulls \
         Blizzard_LoginWarningDialogs (legacy Classic warning dialogs) plus \
         Blizzard_CharacterSelectNavBar (which on Classic owns the bulk of the character-select \
         layout that Mainline splits into Blizzard_GlueXMLBase)"
    );

    let toc_text = std::fs::read_to_string(glue_menu_frame_classic_toc())
        .expect("Blizzard_GlueMenuFrame_Classic TOC should read");
    assert!(
        toc_text.contains("## Title: Blizzard_GlueXML"),
        "The Classic-flavor TOC declares `## Title: Blizzard_GlueXML` (the legacy umbrella \
         name on Classic), even though the directory + file paths still target the \
         Blizzard_GlueMenuFrame addon — a quirk of the upstream Blizzard source the test must \
         not silently normalize"
    );
    assert!(
        toc_text.contains("## AllowLoadGameType: classic"),
        "Classic TOC carries `## AllowLoadGameType: classic` so the Mainline-targeted \
         simulator never picks it (find_toc_file's `_Mainline.toc` preference handles this)"
    );
}

#[test]
fn blizzard_glue_menu_frame_mainline_toc_lists_three_files_with_shared_util_first() {
    let toc_text = std::fs::read_to_string(glue_menu_frame_mainline_toc())
        .expect("Blizzard_GlueMenuFrame_Mainline TOC should read");
    let body_lines: Vec<&str> = toc_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();

    assert_eq!(
        body_lines,
        vec![
            "GlueMenuFrameUtil.lua",
            "Mainline\\GlueMenuFrame.lua",
            "Mainline\\GlueMenuFrame.xml",
        ],
        "Blizzard_GlueMenuFrame_Mainline TOC body lists exactly 3 files in this order: the \
         flavor-shared `GlueMenuFrameUtil.lua` at the addon root MUST come first because \
         Mainline\\GlueMenuFrame.xml's KeyValue references `GlueMenuFrameUtil.GlueMenuContextKey`, \
         then the Mainline-flavored mixin Lua, then the XML that binds the mixin"
    );
}

#[test]
fn blizzard_glue_menu_frame_appears_in_character_select_and_login_discovery() {
    let cs_addons =
        discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::CharacterSelect);
    let in_cs = cs_addons
        .iter()
        .any(|(name, _)| name == "Blizzard_GlueMenuFrame");
    assert!(
        in_cs,
        "Blizzard_GlueMenuFrame (## AllowLoad: Glue) should appear in CharacterSelect-screen \
         auto-discovery — that's the screen where the ESC menu shows the InitCharacterSelectButtons \
         layout (Options, Store, AddOns, Credits, Cinematics, Exit)"
    );

    let login_addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Login);
    let in_login = login_addons
        .iter()
        .any(|(name, _)| name == "Blizzard_GlueMenuFrame");
    assert!(
        in_login,
        "Blizzard_GlueMenuFrame should also appear in Login-screen auto-discovery — that's \
         the screen where the ESC menu shows the InitAccountLoginButtons layout (Options, \
         Credits, Cinematics, Manage Account, Community Site, Exit)"
    );
}

#[test]
fn blizzard_glue_menu_frame_is_absent_from_game_screen_discovery() {
    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    let in_game = addons
        .iter()
        .any(|(name, _)| name == "Blizzard_GlueMenuFrame");
    assert!(
        !in_game,
        "Blizzard_GlueMenuFrame must NOT appear in Game-screen auto-discovery — the in-game \
         ESC menu is GameMenuFrame from a separate addon; loading the glue ESC menu in-game \
         would fight GameMenuFrame for the ESC keybinding"
    );
}

#[test]
fn blizzard_glue_menu_frame_loads_without_addon_specific_errors() {
    let env = load_character_select_screen();

    let menu_errors: Vec<String> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| {
            message.contains("GlueMenuFrame")
                || message.contains("GlueMenuFrameMixin")
                || message.contains("GlueMenuFrameUtil")
        })
        .cloned()
        .collect();
    assert!(
        menu_errors.is_empty(),
        "Blizzard_GlueMenuFrame emitted Lua errors during CharacterSelect-screen load:\n  {}",
        menu_errors.join("\n  ")
    );
}

#[test]
fn blizzard_glue_menu_frame_publishes_glue_menu_frame_util_namespace_with_three_functions() {
    let env = load_character_select_screen();

    let namespace_present: bool = env
        .eval("return type(GlueMenuFrameUtil) == 'table'")
        .expect("GlueMenuFrameUtil query should succeed");
    assert!(
        namespace_present,
        "GlueMenuFrameUtil.lua line 1 publishes `GlueMenuFrameUtil = {{}}` — the shared util \
         namespace consumed by both the Mainline and Classic mixin Lua"
    );

    let context_key: String = env
        .eval("return GlueMenuFrameUtil.GlueMenuContextKey")
        .expect("GlueMenuContextKey query should succeed");
    assert_eq!(
        context_key, "GlueMenuFrame",
        "GlueMenuFrameUtil.lua line 3 sets `GlueMenuContextKey = \"GlueMenuFrame\"` — the \
         string is passed as the first arg to GenerateFlatClosure(GlueParent_ShowOptionsScreen, \
         GlueMenuFrameUtil.GlueMenuContextKey) so GlueParent's screen-management code knows \
         which caller requested the screen change (used to dismiss the menu when the player \
         backs out of Options)"
    );

    for fn_name in ["ShowMenu", "HideMenu", "ToggleMenu"] {
        let has_fn: bool = env
            .eval(&format!(
                "return type(GlueMenuFrameUtil.{fn_name}) == 'function'"
            ))
            .expect("util-fn query should succeed");
        assert!(
            has_fn,
            "GlueMenuFrameUtil.{fn_name} should be a function after load — it is one of the \
             three keybinding-bound entry points (ShowMenu, HideMenu with an IG_MAINMENU_CONTINUE \
             sound, ToggleMenu that branches on GlueMenuFrame:IsShown())"
        );
    }
}

#[test]
fn blizzard_glue_menu_frame_publishes_mixin_with_six_handlers_and_init_methods() {
    let env = load_character_select_screen();

    let mixin_present: bool = env
        .eval("return type(GlueMenuFrameMixin) == 'table'")
        .expect("GlueMenuFrameMixin query should succeed");
    assert!(
        mixin_present,
        "Mainline/GlueMenuFrame.lua line 1 publishes `GlueMenuFrameMixin = {{}}` — bound by \
         the GlueMenuFrame XML via `mixin=GlueMenuFrameMixin`"
    );

    let methods = [
        "OnShow",
        "OnHide",
        "InitButtons",
        "GenerateMenuCallback",
        "InitAccountLoginButtons",
        "InitCharacterSelectButtons",
    ];
    for method in methods {
        let has_method: bool = env
            .eval(&format!(
                "return type(GlueMenuFrameMixin.{method}) == 'function'"
            ))
            .expect("mixin-method query should succeed");
        assert!(
            has_method,
            "GlueMenuFrameMixin.{method} should be a function after load — the mixin owns 6 \
             methods total: 2 lifecycle hooks (OnShow chains BaseLayoutMixin.OnShow + adds \
             modal frame + calls InitButtons; OnHide removes modal frame), InitButtons branches \
             on GlueParent_GetCurrentScreen() between the two button-list builders, \
             GenerateMenuCallback returns a closure that hides the menu and invokes its arg, \
             and the two button-list builders InitAccountLoginButtons / InitCharacterSelectButtons"
        );
    }
}

#[test]
fn blizzard_glue_menu_frame_publishes_glue_menu_frame_global_with_glue_parent() {
    let env = load_character_select_screen();

    let frame_present: bool = env
        .eval("return GlueMenuFrame ~= nil and type(GlueMenuFrame.IsShown) == 'function'")
        .expect("GlueMenuFrame frame query should succeed");
    assert!(
        frame_present,
        "Mainline/GlueMenuFrame.xml line 10 declares `<Frame name=\"GlueMenuFrame\">` so the \
         frame publishes as a global with all standard frame methods (IsShown / Show / Hide \
         / GetParent etc.)"
    );

    let parent_name: String = env
        .eval("return GlueMenuFrame:GetParent():GetName()")
        .expect("GlueMenuFrame parent query should succeed");
    assert_eq!(
        parent_name, "GlueParent",
        "Mainline/GlueMenuFrame.xml line 10 sets `parent=\"GlueParent\"` — the menu must dock \
         into the GlueParent surface (NOT UIParent) because the glue screen does not run \
         UIParent. GlueParent is provided by Blizzard_GlueParent which loads first via the \
         dependency chain"
    );

    let starts_hidden: bool = env
        .eval("return not GlueMenuFrame:IsShown()")
        .expect("GlueMenuFrame visibility query should succeed");
    assert!(
        starts_hidden,
        "Mainline/GlueMenuFrame.xml line 10 sets `hidden=\"true\"` — the menu must start \
         hidden and only show when the player presses ESC (or another caller invokes \
         GlueMenuFrameUtil.ShowMenu)"
    );
}

#[test]
fn blizzard_glue_menu_frame_does_not_leak_button_template_as_global() {
    let env = load_character_select_screen();

    let leaked: bool = env
        .eval("return _G['GlueMenuFrameButtonTemplate'] ~= nil")
        .expect("template-leak query should succeed");
    assert!(
        !leaked,
        "Virtual template `GlueMenuFrameButtonTemplate` (Mainline/GlueMenuFrame.xml line 4 — \
         `<Button name=\"GlueMenuFrameButtonTemplate\" inherits=\"MainMenuFrameButtonTemplate\" \
         virtual=\"true\">`) must not leak as a `_G` global — it is only an XML template \
         supplying the GlueFontNormal / GlueFontHighlight / GlueFontDisable font overrides for \
         the buttons spawned by GlueMenuFrame's MainMenuFrameMixin.AddButton helper. The menu \
         frame XML wires it via the `buttonTemplate` KeyValue, not by name lookup at runtime"
    );
}

#[test]
fn blizzard_glue_menu_frame_dir_ships_both_flavors_plus_shared_util() {
    let dir = glue_menu_frame_dir();
    let mut entries: Vec<String> = std::fs::read_dir(&dir)
        .expect("Blizzard_GlueMenuFrame dir should read")
        .flatten()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec![
            "Blizzard_GlueMenuFrame_Classic.toc".to_string(),
            "Blizzard_GlueMenuFrame_Mainline.toc".to_string(),
            "Classic".to_string(),
            "GlueMenuFrameUtil.lua".to_string(),
            "Mainline".to_string(),
        ],
        "Blizzard_GlueMenuFrame ships exactly: 2 flavor TOCs (Mainline + Classic) + 2 flavor \
         subdirectories with the per-flavor mixin Lua/XML + the flavor-shared GlueMenuFrameUtil.lua \
         at the addon root. Any extra entry suggests the addon has been extended in source \
         without the test keeping pace"
    );
}
