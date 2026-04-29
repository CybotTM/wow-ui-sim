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

fn glue_collections_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_GlueCollections")
}

fn glue_collections_toc() -> PathBuf {
    glue_collections_dir().join("Blizzard_GlueCollections.toc")
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
fn blizzard_glue_collections_toc_declares_glue_only_with_shared_xml_and_paged_content_deps() {
    let toc = TocFile::from_file(&glue_collections_toc())
        .expect("Blizzard_GlueCollections TOC should parse");
    assert!(
        !toc.is_load_on_demand(),
        "Blizzard_GlueCollections is non-LoadOnDemand — it auto-loads on the glue screens \
         after Blizzard_SharedXML and Blizzard_PagedContent so the warband-scene picker can \
         attach to CharacterSelectUI.CollectionsFrame as soon as that surface exists"
    );
    assert!(
        !toc.is_secure_env(),
        "Blizzard_GlueCollections does not declare UseSecureEnvironment"
    );
    let deps = toc.dependencies();
    assert_eq!(
        deps,
        vec![
            "Blizzard_SharedXML".to_string(),
            "Blizzard_PagedContent".to_string(),
        ],
        "Blizzard_GlueCollections should declare exactly two dependencies in order: \
         Blizzard_SharedXML (publishes WarbandSceneEntryMixin / WarbandSceneTemplate that \
         WarbandSceneGlueEntryMixin extends via CreateFromMixins) and Blizzard_PagedContent \
         (publishes PagedNaturalSizeGridContentFrameTemplate / PagingControlsHorizontalTemplate \
         that the journal embeds for the icon grid)"
    );
}

#[test]
fn blizzard_glue_collections_toc_declares_glue_screen_and_mainline_only() {
    let toc_text = std::fs::read_to_string(glue_collections_toc())
        .expect("Blizzard_GlueCollections TOC should read");
    assert!(
        toc_text.contains("## AllowLoad: Glue"),
        "Blizzard_GlueCollections declares `## AllowLoad: Glue` (capital G — glue-screen \
         only). The Game-screen warband-scene picker is in Blizzard_Collections/Mainline/\
         Blizzard_WarbandSceneCollection.lua and is independent"
    );
    assert!(
        toc_text.contains("## AllowLoadGameType: mainline"),
        "Blizzard_GlueCollections declares `## AllowLoadGameType: mainline` so the addon \
         loads on retail only — Classic flavors do not ship the warband-scene system"
    );
}

#[test]
fn blizzard_glue_collections_lists_two_lua_and_two_xml_files_with_warband_scene_first() {
    let toc_text = std::fs::read_to_string(glue_collections_toc())
        .expect("Blizzard_GlueCollections TOC should read");
    let body_lines: Vec<&str> = toc_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();

    assert_eq!(
        body_lines,
        vec![
            "GlueWarbandSceneCollection.lua",
            "GlueWarbandSceneCollection.xml",
            "GlueCollections.lua",
            "GlueCollections.xml",
        ],
        "Blizzard_GlueCollections lists exactly four files with WarbandScene first: the \
         WarbandSceneGlueEntryMixin / GlueWarbandSceneJounalMixin (sic — the source has the \
         `Jounal` typo) must publish before GlueCollections.xml's `mixin=\
         GlueCollectionsMixin inherits=PortraitFrameTemplate` template tries to bind to them"
    );
}

#[test]
fn blizzard_glue_collections_appears_in_character_select_discovery() {
    let addons =
        discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::CharacterSelect);
    let in_char_select = addons
        .iter()
        .any(|(name, _)| name == "Blizzard_GlueCollections");
    assert!(
        in_char_select,
        "Blizzard_GlueCollections (## AllowLoad: Glue) should appear in CharacterSelect-screen \
         auto-discovery — that's the glue screen where players open the warband-scene \
         backdrop picker via the collections journal button"
    );
}

#[test]
fn blizzard_glue_collections_appears_in_login_discovery() {
    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Login);
    let in_login = addons
        .iter()
        .any(|(name, _)| name == "Blizzard_GlueCollections");
    assert!(
        in_login,
        "Blizzard_GlueCollections also appears in Login-screen auto-discovery — `## AllowLoad: \
         Glue` covers every glue screen (Login + CharacterSelect + CharacterCreate); the Lua \
         only matters once CharacterSelectUI.CollectionsFrame exists, but the addon loads \
         eagerly across all three"
    );
}

#[test]
fn blizzard_glue_collections_is_absent_from_game_screen_discovery() {
    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    let in_game = addons
        .iter()
        .any(|(name, _)| name == "Blizzard_GlueCollections");
    assert!(
        !in_game,
        "Blizzard_GlueCollections must NOT appear in Game-screen auto-discovery — the \
         in-game warband-scene UI lives in Blizzard_Collections/Mainline/\
         Blizzard_WarbandSceneCollection.lua, which is a separate Game-only surface"
    );
}

#[test]
fn blizzard_glue_collections_loads_without_addon_specific_errors() {
    let env = load_character_select_screen();

    let collections_errors: Vec<String> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| {
            message.contains("GlueCollections")
                || message.contains("GlueWarbandScene")
                || message.contains("WarbandSceneGlueEntryMixin")
                || message.contains("GlueWarbandSceneJounalMixin")
        })
        .cloned()
        .collect();
    assert!(
        collections_errors.is_empty(),
        "Blizzard_GlueCollections emitted Lua errors during CharacterSelect-screen load:\n  {}",
        collections_errors.join("\n  ")
    );
}

#[test]
fn blizzard_glue_collections_publishes_glue_collections_mixin_with_three_handlers() {
    let env = load_character_select_screen();

    let mixin_present: bool = env
        .eval("return type(GlueCollectionsMixin) == 'table'")
        .expect("GlueCollectionsMixin query should succeed");
    assert!(
        mixin_present,
        "Blizzard_GlueCollections.lua line 1 publishes `GlueCollectionsMixin = {{}}` as a \
         global table — the mixin attaches three Frame handlers (OnShow / OnHide / OnKeyDown) \
         that GlueCollectionsTemplate binds via `mixin=GlueCollectionsMixin`"
    );

    for handler in ["OnShow", "OnHide", "OnKeyDown"] {
        let has_method: bool = env
            .eval(&format!(
                "return type(GlueCollectionsMixin.{handler}) == 'function'"
            ))
            .expect("mixin handler query should succeed");
        assert!(
            has_method,
            "GlueCollectionsMixin.{handler} should be a function after load — it implements \
             the GlueCollectionsTemplate frame's `<Scripts><{handler}/></Scripts>` binding"
        );
    }
}

#[test]
fn blizzard_glue_collections_publishes_warband_scene_glue_entry_mixin_extending_shared_xml() {
    let env = load_character_select_screen();

    let mixin_present: bool = env
        .eval("return type(WarbandSceneGlueEntryMixin) == 'table'")
        .expect("WarbandSceneGlueEntryMixin query should succeed");
    assert!(
        mixin_present,
        "GlueWarbandSceneCollection.lua line 1 publishes `WarbandSceneGlueEntryMixin = \
         CreateFromMixins(WarbandSceneEntryMixin)` — extends the SharedXML base mixin with \
         glue-screen-specific OnClick / Init / SetSelectedState overrides"
    );

    let inherits_shared_init: bool = env
        .eval(
            "return type(WarbandSceneGlueEntryMixin.OnMouseUp) == 'function' \
             or type(WarbandSceneGlueEntryMixin.GetIsOwned) == 'function'",
        )
        .expect("inherited-method query should succeed");
    assert!(
        inherits_shared_init,
        "WarbandSceneGlueEntryMixin should inherit at least one method from \
         WarbandSceneEntryMixin (Blizzard_SharedXML/Mainline/SharedCollectionTemplates.lua) \
         via CreateFromMixins — the glue extension only overrides OnClick / Init / \
         SetSelectedState, leaving OnMouseUp and GetIsOwned to come from the parent"
    );

    for method in ["OnClick", "Init", "SetSelectedState"] {
        let has_method: bool = env
            .eval(&format!(
                "return type(WarbandSceneGlueEntryMixin.{method}) == 'function'"
            ))
            .expect("entry-mixin method query should succeed");
        assert!(
            has_method,
            "WarbandSceneGlueEntryMixin.{method} should be a function after load — it \
             implements the glue-screen WarbandSceneGlueTemplate handlers (OnClick selects, \
             Init chains to parent.Init then sets selected state, SetSelectedState toggles \
             HighlightTexture)"
        );
    }
}

#[test]
fn blizzard_glue_collections_publishes_journal_mixin_with_typo_preserved_and_apply_for_all_buffer()
{
    let env = load_character_select_screen();

    let mixin_present: bool = env
        .eval("return type(GlueWarbandSceneJounalMixin) == 'table'")
        .expect("GlueWarbandSceneJounalMixin query should succeed");
    assert!(
        mixin_present,
        "GlueWarbandSceneCollection.lua line 23 publishes `GlueWarbandSceneJounalMixin` — \
         note the typo `Jounal` (missing `r`) in the mixin name. The XML at \
         GlueWarbandSceneCollection.xml line 15 carries the same typo \
         (`mixin=GlueWarbandSceneJounalMixin`) so the binding still works. The template name \
         GlueWarbandSceneJournalTemplate (correctly spelled) is independent of the mixin name"
    );

    let buffer_value: f64 = env
        .eval("return GlueWarbandSceneJounalMixin.ApplyForAllCheckboxWidthBuffer")
        .expect("buffer-constant query should succeed");
    assert_eq!(
        buffer_value, 25.0,
        "GlueWarbandSceneJounalMixin.ApplyForAllCheckboxWidthBuffer is set to 25 in the \
         table literal at GlueWarbandSceneCollection.lua line 23 — used by SetupJournalEntries \
         to compute the apply-for-all checkbox horizontal offset relative to the apply button"
    );

    let methods = [
        "OnLoad",
        "OnShow",
        "OnEvent",
        "SetupJournalDropdown",
        "SetupJournalEntries",
        "SetJournalEntries",
        "UpdateWarbandScenes",
        "SelectWarbandScene",
        "GetSelectedStateForEntry",
    ];
    for method in methods {
        let has_method: bool = env
            .eval(&format!(
                "return type(GlueWarbandSceneJounalMixin.{method}) == 'function'"
            ))
            .expect("journal-mixin method query should succeed");
        assert!(
            has_method,
            "GlueWarbandSceneJounalMixin.{method} should be a function after load — it is \
             one of the nine handlers that drive the warband-scene picker journal lifecycle"
        );
    }
}

#[test]
fn blizzard_glue_collections_registers_confirm_warband_scenes_apply_all_static_popup() {
    let env = load_character_select_screen();

    let popup_registered: bool = env
        .eval(
            "return type(StaticPopupDialogs) == 'table' \
             and type(StaticPopupDialogs['CONFIRM_WARBAND_SCENES_APPLY_ALL']) == 'table'",
        )
        .expect("StaticPopupDialog registration query should succeed");
    assert!(
        popup_registered,
        "GlueWarbandSceneCollection.lua line 31 registers \
         `StaticPopupDialogs[\"CONFIRM_WARBAND_SCENES_APPLY_ALL\"] = {{ ... }}` — the \
         confirm-on-apply-for-all dialog the journal raises before broadcasting the warband \
         scene to every saved character"
    );
}

#[test]
fn blizzard_glue_collections_does_not_leak_virtual_templates_as_globals() {
    let env = load_character_select_screen();

    for template in [
        "GlueCollectionsTemplate",
        "WarbandSceneGlueTemplate",
        "GlueWarbandSceneJournalTemplate",
    ] {
        let leaked: bool = env
            .eval(&format!("return _G['{template}'] ~= nil"))
            .expect("global-template query should succeed");
        assert!(
            !leaked,
            "Virtual template `{template}` (declared with `virtual=\"true\"`) must not leak as \
             a `_G` global — it is only registered in the XML template registry for inheritance \
             and CreateFrame template lookup. A leak indicates the XML loader incorrectly \
             materialized a runtime frame for a virtual definition"
        );
    }
}

#[test]
fn blizzard_glue_collections_dir_ships_exactly_five_source_files() {
    let dir = glue_collections_dir();
    let mut entries: Vec<String> = std::fs::read_dir(&dir)
        .expect("Blizzard_GlueCollections dir should read")
        .flatten()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec![
            "Blizzard_GlueCollections.toc".to_string(),
            "GlueCollections.lua".to_string(),
            "GlueCollections.xml".to_string(),
            "GlueWarbandSceneCollection.lua".to_string(),
            "GlueWarbandSceneCollection.xml".to_string(),
        ],
        "Blizzard_GlueCollections ships exactly the TOC plus four source files — no \
         additional Mainline/ subdirectory, no localized strings file, no test fixtures. \
         Any extra entry suggests the addon has been extended in source without the test \
         keeping pace"
    );
}
