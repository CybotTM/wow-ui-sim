use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, find_toc_file, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}

fn glue_xml_base_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_GlueXMLBase")
}

fn glue_xml_base_mainline_toc() -> PathBuf {
    glue_xml_base_dir().join("Blizzard_GlueXMLBase_Mainline.toc")
}

fn glue_xml_base_mists_toc() -> PathBuf {
    glue_xml_base_dir().join("Blizzard_GlueXMLBase_Mists.toc")
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
fn blizzard_glue_xml_base_find_toc_resolves_mainline_variant() {
    let resolved =
        find_toc_file(&glue_xml_base_dir()).expect("Blizzard_GlueXMLBase TOC should resolve");
    assert_eq!(
        resolved,
        glue_xml_base_mainline_toc(),
        "Blizzard_GlueXMLBase ships two flavor TOC variants (`_Mainline.toc` for retail and \
         `_Mists.toc` for the Mists of Pandaria classic flavor) — `find_toc_file` \
         (src/loader/mod.rs:65) prefers `_Mainline.toc` on the first lookup. The Mists TOC \
         additionally declares `## AllowLoadGameType: mists` which `is_game_type_restricted()` \
         filters out of mainline auto-discovery"
    );
}

#[test]
fn blizzard_glue_xml_base_mainline_toc_declares_load_first_glue_with_two_deps() {
    let toc = TocFile::from_file(&glue_xml_base_mainline_toc())
        .expect("Blizzard_GlueXMLBase_Mainline TOC should parse");
    assert!(
        !toc.is_load_on_demand(),
        "Blizzard_GlueXMLBase is non-LoadOnDemand — the base glue templates (GlueButtonTemplate, \
         GlueCheckButtonTemplate, GlueContextMenuTemplate) and shared constants (GLUE_AMBIENCE_TRACKS, \
         CREDITS_SCROLL_RATE_*, HTML_START) must auto-load eagerly so downstream glue addons \
         (Blizzard_GlueXML, Blizzard_GlueParent, Blizzard_GlueMenuFrame) can inherit and reference \
         them during their own load pass"
    );
    assert!(
        toc.is_load_first(),
        "Blizzard_GlueXMLBase declares `## LoadFirst: 1` — the base templates and constants must \
         install before the bulk of the glue-screen UI surface (Blizzard_GlueParent depends on \
         Blizzard_GlueXMLBase to provide CallbackRegistrantTemplate references and the legacy \
         expansion-data helpers consumed by GlueParentMixin's lifecycle path)"
    );
    assert!(
        !toc.is_secure_env(),
        "Blizzard_GlueXMLBase does not declare `## UseSecureEnvironment` — these are pure \
         template / constant declarations with no protected actions"
    );
    assert_eq!(
        toc.dependencies(),
        vec![
            "Blizzard_SharedXML".to_string(),
            "Blizzard_ScriptErrorsFrame".to_string(),
        ],
        "Blizzard_GlueXMLBase_Mainline declares exactly 2 deps in this order: Blizzard_SharedXML \
         (provides SharedButtonTemplate / SharedButtonSmallTemplate / SharedGoldRedButtonTemplate \
         / TooltipBackdropTemplate that GlueButtons.xml + GlueContextMenu.xml inherit from, plus \
         the CreateColor / CreateFramePool / wipe primitives the constants and context-menu Lua \
         consume) and Blizzard_ScriptErrorsFrame (the script-error display the glue-screen flow \
         routes Lua errors to before any other glue addon emits an error)"
    );
}

#[test]
fn blizzard_glue_xml_base_mainline_toc_declares_glue_screen_mainline_only() {
    let toc_text = std::fs::read_to_string(glue_xml_base_mainline_toc())
        .expect("Blizzard_GlueXMLBase_Mainline TOC should read");
    assert!(
        toc_text.contains("## AllowLoad: Glue"),
        "Blizzard_GlueXMLBase_Mainline declares `## AllowLoad: Glue` (capital G — \
         glue-screen-only). Loads on Login + CharacterSelect + CharacterCreate, absent from Game \
         where the in-game template surface is owned by separate Game-screen addons"
    );
    assert!(
        toc_text.contains("## AllowLoadGameType: mainline"),
        "Blizzard_GlueXMLBase_Mainline declares `## AllowLoadGameType: mainline` so retail-only"
    );
    assert!(
        toc_text.contains("## DefaultState: enabled"),
        "Blizzard_GlueXMLBase declares `## DefaultState: enabled` — the base glue templates must \
         always be active so downstream glue addon inheritance never errors"
    );
}

#[test]
fn blizzard_glue_xml_base_mainline_toc_lists_files_with_constants_first() {
    let toc = TocFile::from_file(&glue_xml_base_mainline_toc())
        .expect("Blizzard_GlueXMLBase_Mainline TOC should parse");
    let files: Vec<String> = toc
        .files
        .iter()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .collect();
    assert_eq!(
        files,
        vec![
            "Mainline/Constants.lua".to_string(),
            "Mainline/Localization.lua".to_string(),
            "Mainline/GlueTemplates.lua".to_string(),
            "Mainline/GlueTemplates.xml".to_string(),
            "Mainline/GlueButtons.xml".to_string(),
            "Shared/GlueContextMenu.lua".to_string(),
            "Shared/GlueContextMenu.xml".to_string(),
        ],
        "Blizzard_GlueXMLBase_Mainline TOC enumerates exactly 7 files in this exact order: \
         Constants.lua first (publishes GLUE_AMBIENCE_TRACKS / CREDITS_SCROLL_RATE_* / \
         HTML_START etc. that the rest of the glue-screen surface reads at load time), \
         Localization.lua (locale strings), GlueTemplates.lua (10 GlueTemplates_* tab helper \
         functions consumed by GlueTemplates.xml's tab buttons), GlueTemplates.xml \
         (GlueCheckButtonTemplate), GlueButtons.xml (5 button templates), GlueContextMenu.lua \
         (GlueContextMenuMixin + 4 GlobalGlueContextMenu_* functions), and GlueContextMenu.xml \
         (GlueContextMenuButtonTemplate + GlueContextMenuTemplate + the GlueContextMenu instance)"
    );
}

#[test]
fn blizzard_glue_xml_base_mists_toc_declares_mists_game_type_only() {
    let toc = TocFile::from_file(&glue_xml_base_mists_toc())
        .expect("Blizzard_GlueXMLBase_Mists TOC should parse");
    assert!(
        toc.is_load_first(),
        "Blizzard_GlueXMLBase_Mists also declares `## LoadFirst: 1` — same load-order priority"
    );
    assert!(
        toc.is_game_type_restricted(),
        "Blizzard_GlueXMLBase_Mists declares `## AllowLoadGameType: mists` which \
         `is_game_type_restricted()` (src/toc.rs:294) treats as a non-mainline restriction — so \
         this TOC is filtered out of mainline auto-discovery"
    );
    assert_eq!(
        toc.dependencies(),
        vec!["Blizzard_SharedXML".to_string()],
        "Blizzard_GlueXMLBase_Mists declares only 1 dep — the Mists classic flavor strips \
         Blizzard_ScriptErrorsFrame (Classic builds use a different script-error display)"
    );
    let mists_text = std::fs::read_to_string(glue_xml_base_mists_toc())
        .expect("Blizzard_GlueXMLBase_Mists TOC should read");
    assert!(
        mists_text.contains("## AllowLoadGameType: mists"),
        "Blizzard_GlueXMLBase_Mists raw TOC must contain `## AllowLoadGameType: mists`"
    );
    assert!(
        mists_text.contains("## AllowLoad: Glue"),
        "Blizzard_GlueXMLBase_Mists also targets glue screens, just gated by game type"
    );
}

#[test]
fn blizzard_glue_xml_base_directory_ships_two_tocs_and_three_subdirs() {
    let dir = glue_xml_base_dir();
    let mut entries: Vec<String> = std::fs::read_dir(&dir)
        .expect("Blizzard_GlueXMLBase directory should exist")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    entries.sort();
    assert_eq!(
        entries,
        vec![
            "Blizzard_GlueXMLBase_Mainline.toc".to_string(),
            "Blizzard_GlueXMLBase_Mists.toc".to_string(),
            "Mainline".to_string(),
            "Mists".to_string(),
            "Shared".to_string(),
        ],
        "Blizzard_GlueXMLBase directory ships exactly 5 entries: 2 flavor TOCs (Mainline + \
         Mists) plus 3 subdirectories (Mainline/ for retail-specific source files, Mists/ for \
         Mists-flavor sources, Shared/ for cross-flavor GlueContextMenu sources)"
    );
}

#[test]
fn blizzard_glue_xml_base_appears_in_all_three_glue_screen_discoveries() {
    let ui = blizzard_ui_dir();
    for screen in [
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        let addons = discover_blizzard_addons_for_screen(&ui, screen);
        let entries: Vec<&(String, PathBuf)> = addons
            .iter()
            .filter(|(name, _)| name == "Blizzard_GlueXMLBase")
            .collect();
        assert_eq!(
            entries.len(),
            1,
            "Blizzard_GlueXMLBase should appear exactly once in {screen:?} auto-discovery — \
             `find_toc_file` resolves to the `_Mainline.toc` variant; the `_Mists.toc` variant \
             is filtered out by `is_game_type_restricted()`. Got entries: {entries:?}"
        );
        assert_eq!(
            entries[0].1,
            glue_xml_base_mainline_toc(),
            "Blizzard_GlueXMLBase on {screen:?} should resolve to the `_Mainline.toc` variant"
        );
    }
}

#[test]
fn blizzard_glue_xml_base_absent_from_game_screen_discovery() {
    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);
    let discovered = addons
        .iter()
        .any(|(name, _)| name == "Blizzard_GlueXMLBase");
    assert!(
        !discovered,
        "Blizzard_GlueXMLBase MUST NOT appear in Game-screen auto-discovery — `## AllowLoad: \
         Glue` is glue-only. The in-game button / template surface is loaded by separate \
         Game-screen addons"
    );
}

#[test]
fn blizzard_glue_xml_base_loads_without_addon_specific_lua_errors() {
    let env = load_character_select_screen();

    let lua_errors: Vec<String> = env.state().borrow().lua_errors.clone();
    let related: Vec<&String> = lua_errors
        .iter()
        .filter(|e| {
            e.contains("Blizzard_GlueXMLBase/")
                || e.contains("Blizzard_GlueXMLBase\\")
                || e.contains("GlueContextMenuMixin")
                || e.contains("GlueTemplates_")
                || e.contains("GLUE_AMBIENCE_TRACKS")
        })
        .collect();
    assert!(
        related.is_empty(),
        "Blizzard_GlueXMLBase emitted addon-specific Lua errors during CharacterSelect-screen \
         load:\n  {}",
        related
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}

#[test]
fn blizzard_glue_xml_base_publishes_glue_template_tab_helpers() {
    let env = load_character_select_screen();

    for helper in [
        "GlueTemplates_TabResize",
        "GlueTemplates_SetTab",
        "GlueTemplates_GetSelectedTab",
        "GlueTemplates_UpdateTabs",
        "GlueTemplates_SetNumTabs",
        "GlueTemplates_DisableTab",
        "GlueTemplates_EnableTab",
        "GlueTemplates_DeselectTab",
        "GlueTemplates_SelectTab",
        "GlueTemplates_SetDisabledTabState",
    ] {
        let exists: bool = env
            .eval(&format!("return type(_G['{helper}']) == 'function'"))
            .expect("helper existence query should succeed");
        assert!(
            exists,
            "After CharacterSelect-screen load, `{helper}` should be published as a `_G` \
             function — Mainline/GlueTemplates.lua publishes 10 GlueTemplates_* tab-helper \
             globals consumed by glue-screen tab UIs (CharacterSelect, RealmList, etc.)"
        );
    }
}

#[test]
fn blizzard_glue_xml_base_publishes_global_glue_context_menu_helpers() {
    let env = load_character_select_screen();

    for helper in [
        "GlobalGlueContextMenu_GetOwner",
        "GlobalGlueContextMenu_Acquire",
        "GlobalGlueContextMenu_IsShown",
        "GlobalGlueContextMenu_Release",
    ] {
        let exists: bool = env
            .eval(&format!("return type(_G['{helper}']) == 'function'"))
            .expect("helper existence query should succeed");
        assert!(
            exists,
            "After CharacterSelect-screen load, `{helper}` should be published as a `_G` \
             function — Shared/GlueContextMenu.lua publishes 4 GlobalGlueContextMenu_* helpers \
             that wrap the singleton GlueContextMenu frame for cross-addon use"
        );
    }
}

#[test]
fn blizzard_glue_xml_base_publishes_glue_context_menu_mixin_and_frame() {
    let env = load_character_select_screen();

    let mixin_exists: bool = env
        .eval("return type(_G['GlueContextMenuMixin']) == 'table'")
        .expect("mixin existence query should succeed");
    assert!(
        mixin_exists,
        "GlueContextMenuMixin should publish as a `_G` table — Shared/GlueContextMenu.lua \
         declares the mixin with Initialize / OnUpdate / AddButton / GetMaximumButtonWidth / \
         RefreshSize / Reset methods"
    );

    for method in [
        "Initialize",
        "OnUpdate",
        "AddButton",
        "GetMaximumButtonWidth",
        "RefreshSize",
        "Reset",
    ] {
        let method_exists: bool = env
            .eval(&format!(
                "return type(_G['GlueContextMenuMixin']['{method}']) == 'function'"
            ))
            .expect("mixin method query should succeed");
        assert!(
            method_exists,
            "GlueContextMenuMixin.{method} should be a function — owned by \
             Shared/GlueContextMenu.lua"
        );
    }

    let frame_exists: bool = env
        .eval(
            "local f = _G['GlueContextMenu']; return type(f) == 'table' \
             and type(f.GetName) == 'function'",
        )
        .expect("frame existence query should succeed");
    assert!(
        frame_exists,
        "GlueContextMenu should publish as a global frame instance — XML line 17 of \
         Shared/GlueContextMenu.xml declares `<Frame name=\"GlueContextMenu\" \
         inherits=\"GlueContextMenuTemplate\" hidden=\"true\"/>` so it materializes as a \
         singleton frame"
    );
}

#[test]
fn blizzard_glue_xml_base_publishes_credits_scroll_rate_constants() {
    let env = load_character_select_screen();

    let pairs: Vec<(&str, f64)> = vec![
        ("CREDITS_SCROLL_RATE_REWIND", -160.0),
        ("CREDITS_SCROLL_RATE_PAUSE", 0.0),
        ("CREDITS_SCROLL_RATE_PLAY", 40.0),
        ("CREDITS_SCROLL_RATE_FASTFORWARD", 160.0),
        ("CREDITS_SCROLL_RATE", 40.0),
        ("CREDITS_FADE_RATE", 0.4),
        ("NUM_CREDITS_ART_TEXTURES_WIDE", 4.0),
        ("NUM_CREDITS_ART_TEXTURES_HIGH", 2.0),
        ("CACHE_WAIT_TIME", 0.5),
        ("AUTO_LOGIN_WAIT_TIME", 1.75),
    ];
    for (name, expected) in pairs {
        let actual: f64 = env
            .eval(&format!("return _G['{name}']"))
            .expect("constant query should succeed");
        assert!(
            (actual - expected).abs() < 1e-9,
            "Constant `{name}` should publish as {expected} — got {actual}"
        );
    }
}

#[test]
fn blizzard_glue_xml_base_publishes_html_block_constants() {
    let env = load_character_select_screen();

    for (name, expected) in [
        ("HTML_START", "<html><body><p>"),
        ("HTML_START_CENTERED", "<html><body><p align=\"center\">"),
        ("HTML_END", "</p></body></html>"),
    ] {
        let actual: String = env
            .eval(&format!("return _G['{name}']"))
            .expect("constant query should succeed");
        assert_eq!(
            actual, expected,
            "Constant `{name}` should publish exactly as `{expected}` — got `{actual}`"
        );
    }
}

#[test]
fn blizzard_glue_xml_base_publishes_safe_get_expansion_data_recursion() {
    let env = load_character_select_screen();

    let exists: bool = env
        .eval("return type(_G['SafeGetExpansionData']) == 'function'")
        .expect("function existence query should succeed");
    assert!(
        exists,
        "SafeGetExpansionData should publish as a `_G` function — Mainline/Constants.lua line 1 \
         declares the recursive expansion-level lookup helper that callers (login screen, \
         character-select expansion logo) use to walk down to the highest entry that exists in a \
         dataTable when the queried expansion level is missing"
    );

    let resolved_exact: f64 = env
        .eval("local t = {[3] = 33, [5] = 55}; return SafeGetExpansionData(t, 5)")
        .expect("exact-level recursion query should succeed");
    assert!(
        (resolved_exact - 55.0).abs() < 1e-9,
        "SafeGetExpansionData with an exact-level entry should return that entry — got {resolved_exact}"
    );

    let resolved_fallback: f64 = env
        .eval("local t = {[3] = 33, [5] = 55}; return SafeGetExpansionData(t, 7)")
        .expect("fallback recursion query should succeed");
    assert!(
        (resolved_fallback - 55.0).abs() < 1e-9,
        "SafeGetExpansionData with a missing higher level should recurse down to the next \
         existing entry — expected 55, got {resolved_fallback}"
    );
}

#[test]
fn blizzard_glue_xml_base_publishes_glue_ambience_tracks_table() {
    let env = load_character_select_screen();

    let is_table: bool = env
        .eval("return type(_G['GLUE_AMBIENCE_TRACKS']) == 'table'")
        .expect("table existence query should succeed");
    assert!(
        is_table,
        "GLUE_AMBIENCE_TRACKS should publish as a `_G` table — Mainline/Constants.lua line 10 \
         declares the race -> SOUNDKIT.AMB_GLUESCREEN_<race> mapping consumed by the glue-screen \
         music switcher"
    );

    for race_key in [
        "HUMAN",
        "ORC",
        "TROLL",
        "DWARF",
        "GNOME",
        "TAUREN",
        "PANDAREN",
        "DEMONHUNTER",
        "DRACTHYR",
        "EARTHENDWARF",
        "WARBANDS_MAPSCENE",
    ] {
        let key_exists: bool = env
            .eval(&format!("return GLUE_AMBIENCE_TRACKS['{race_key}'] ~= nil"))
            .expect("race-key lookup should succeed");
        assert!(
            key_exists,
            "GLUE_AMBIENCE_TRACKS should include the `{race_key}` race entry"
        );
    }
}

#[test]
fn blizzard_glue_xml_base_publishes_glue_backdrop_color_objects() {
    let env = load_character_select_screen();

    let backdrop_exists: bool = env
        .eval(
            "local c = _G['GLUE_BACKDROP_COLOR']; return type(c) == 'table' \
             and type(c.GetRGB) == 'function'",
        )
        .expect("backdrop color query should succeed");
    assert!(
        backdrop_exists,
        "GLUE_BACKDROP_COLOR should publish as a CreateColor result — Mainline/Constants.lua \
         line 70 declares `GLUE_BACKDROP_COLOR = CreateColor(0.09, 0.09, 0.09)` consumed by \
         glue-screen tooltip backdrop styling"
    );

    let border_exists: bool = env
        .eval(
            "local c = _G['GLUE_BACKDROP_BORDER_COLOR']; return type(c) == 'table' \
             and type(c.GetRGB) == 'function'",
        )
        .expect("border color query should succeed");
    assert!(
        border_exists,
        "GLUE_BACKDROP_BORDER_COLOR should publish as a CreateColor result — Mainline/Constants.lua \
         line 71 declares `GLUE_BACKDROP_BORDER_COLOR = CreateColor(0.8, 0.8, 0.8)` for the \
         glue-screen tooltip border"
    );
}

#[test]
fn blizzard_glue_xml_base_registers_button_and_check_templates() {
    let _env = load_character_select_screen();

    for template in [
        "GlueButtonTemplate",
        "GlueButtonBigTemplate",
        "GlueButtonSmallTemplate",
        "GlueGoldRedButtonTemplate",
        "GlueGoldRedButtonSmallTemplate",
        "GlueCheckButtonTemplate",
        "GlueContextMenuButtonTemplate",
        "GlueContextMenuTemplate",
    ] {
        assert!(
            wow_ui_sim::xml::get_template(template).is_some(),
            "After CharacterSelect-screen load, `{template}` should be registered in the XML \
             template registry — Blizzard_GlueXMLBase publishes 8 virtual templates: 5 button \
             templates (GlueButtonTemplate / GlueButtonBigTemplate / GlueButtonSmallTemplate / \
             GlueGoldRedButtonTemplate / GlueGoldRedButtonSmallTemplate from \
             Mainline/GlueButtons.xml), 1 check button template (GlueCheckButtonTemplate from \
             Mainline/GlueTemplates.xml), and 2 context-menu templates \
             (GlueContextMenuButtonTemplate + GlueContextMenuTemplate from \
             Shared/GlueContextMenu.xml)"
        );
    }
}

#[test]
fn blizzard_glue_xml_base_does_not_leak_virtual_templates_as_globals() {
    let env = load_character_select_screen();

    for template in [
        "GlueButtonTemplate",
        "GlueButtonBigTemplate",
        "GlueButtonSmallTemplate",
        "GlueGoldRedButtonTemplate",
        "GlueGoldRedButtonSmallTemplate",
        "GlueCheckButtonTemplate",
        "GlueContextMenuButtonTemplate",
        "GlueContextMenuTemplate",
    ] {
        let leaked: bool = env
            .eval(&format!("return _G['{template}'] ~= nil"))
            .expect("global-template query should succeed");
        assert!(
            !leaked,
            "Virtual template `{template}` (declared with `virtual=\"true\"`) must not leak as \
             a `_G` global — only the GlueContextMenu instance frame derived from \
             GlueContextMenuTemplate publishes as a global"
        );
    }
}
