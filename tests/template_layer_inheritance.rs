//! Template inheritance for textures defined in inherited `<Layers>` blocks.
//!
//! `ButtonFrameTemplate` inherits from `ButtonFrameBaseTemplate`, whose
//! `<Layers>` block declares `<Texture parentKey="TopTileStreaks" inherits="_UI-Frame-TopTileStreaks">`.
//! Frames using `ButtonFrameTemplate` (`AlliedRacesFrame`, `AchievementUI`,
//! `BlackMarketUI`, …) call `self.TopTileStreaks:Hide()` in `OnLoad`, so the
//! parentKey must reach the instance.

use crate::common;

use common::env_with_shared_xml;
use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn button_frame_template_exposes_inherited_top_tile_streaks() {
    let env = env_with_shared_xml();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestButtonFrameTopTileStreaks", UIParent, "ButtonFrameTemplate")
    "#,
    )
    .unwrap();

    let has_top_tile_streaks: bool = env
        .eval("return TestButtonFrameTopTileStreaks.TopTileStreaks ~= nil")
        .unwrap();
    assert!(
        has_top_tile_streaks,
        "ButtonFrameTemplate should expose its inherited TopTileStreaks texture"
    );

    let object_type: String = env
        .eval("return TestButtonFrameTopTileStreaks.TopTileStreaks:GetObjectType()")
        .unwrap();
    assert_eq!(object_type, "Texture");

    let parent_matches: bool = env
        .eval(
            "return TestButtonFrameTopTileStreaks.TopTileStreaks:GetParent() == TestButtonFrameTopTileStreaks",
        )
        .unwrap();
    assert!(
        parent_matches,
        "TopTileStreaks should be parented to the ButtonFrameTemplate instance"
    );
}

#[test]
fn portrait_frame_textured_template_exposes_inherited_top_tile_streaks() {
    let env = env_with_shared_xml();
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestPortraitFrameTexturedTopTileStreaks", UIParent, "PortraitFrameTexturedBaseTemplate")
    "#,
    )
    .unwrap();

    let has_top_tile_streaks: bool = env
        .eval("return TestPortraitFrameTexturedTopTileStreaks.TopTileStreaks ~= nil")
        .unwrap();
    assert!(
        has_top_tile_streaks,
        "PortraitFrameTexturedBaseTemplate should expose its TopTileStreaks texture"
    );
}

#[test]
fn button_frame_template_top_tile_streaks_can_be_hidden_in_on_load() {
    let env = env_with_shared_xml();
    env.exec(
        r#"
        AlliedRacesLikeMixin = {}
        function AlliedRacesLikeMixin:OnLoad()
            self.TopTileStreaks:Hide()
            ALLIED_RACES_LIKE_LOADED = true
        end
    "#,
    )
    .unwrap();

    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestAlliedRacesLike", UIParent, "ButtonFrameTemplate")
        Mixin(f, AlliedRacesLikeMixin)
        f:OnLoad()
    "#,
    )
    .unwrap();

    let loaded: bool = env.eval("return ALLIED_RACES_LIKE_LOADED == true").unwrap();
    assert!(
        loaded,
        "OnLoad should run without 'attempt to index field TopTileStreaks (a nil value)'"
    );

    let hidden: bool = env
        .eval("return TestAlliedRacesLike.TopTileStreaks:IsShown() == false")
        .unwrap();
    assert!(hidden, "TopTileStreaks should be hidden after OnLoad runs");
}

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}

fn load_addon_or_panic(env: &WowLuaEnv, addon: &str) {
    let toc = blizzard_ui_dir().join(format!("{addon}/{addon}.toc"));
    if !toc.exists() {
        panic!("missing TOC for {addon}: {}", toc.display());
    }
    load_addon(&env.loader_env(), &toc).unwrap_or_else(|err| panic!("{addon} load failed: {err}"));
}

/// Reproduce the AlliedRacesUI load path: a top-level XML frame defined via
/// `inherits="ButtonFrameTemplate"` calls `self.TopTileStreaks:Hide()` in its
/// OnLoad mixin. Confirms the static XML loader installs inherited
/// parentKey textures before OnLoad fires.
#[test]
fn allied_races_addon_loads_without_top_tile_streaks_error() {
    let env = env_with_shared_xml();
    load_addon_or_panic(&env, "Blizzard_AlliedRacesUI");

    let frame_exists: bool = env.eval("return AlliedRacesFrame ~= nil").unwrap_or(false);
    assert!(
        frame_exists,
        "AlliedRacesFrame should exist after addon load"
    );

    let has_top_tile_streaks: bool = env
        .eval("return AlliedRacesFrame.TopTileStreaks ~= nil")
        .unwrap();
    assert!(
        has_top_tile_streaks,
        "AlliedRacesFrame should expose its inherited TopTileStreaks texture"
    );

    let object_type: String = env
        .eval("return AlliedRacesFrame.TopTileStreaks:GetObjectType()")
        .unwrap();
    assert_eq!(object_type, "Texture");
}

/// Reproduce the full-Blizzard-load failure path: `discover_all_blizzard_addons`
/// must order foundational shared addons (`SharedXMLBase`, `SharedXML`,
/// `SharedXMLGame`) before LoadOnDemand frames that inherit their templates.
/// Without the ordering, `Blizzard_AlliedRacesUI` (alphabetically before
/// `Blizzard_SharedXML`) instantiates `AlliedRacesFrame` against an empty
/// `ButtonFrameTemplate` chain, and the `TopTileStreaks` parentKey texture is
/// dropped — `OnLoad` then errors with `attempt to index field 'TopTileStreaks'`.
#[test]
fn discover_all_blizzard_addons_loads_shared_xml_before_allied_races_ui() {
    let env = WowLuaEnv::new().expect("WowLuaEnv");
    let ui = blizzard_ui_dir();
    let addons = wow_ui_sim::loader::discover_all_blizzard_addons(&ui);

    let shared_index = addons
        .iter()
        .position(|(name, _)| name == "Blizzard_SharedXML")
        .expect("Blizzard_SharedXML should be present in the discovered addon list");
    let allied_index = addons
        .iter()
        .position(|(name, _)| name == "Blizzard_AlliedRacesUI")
        .expect("Blizzard_AlliedRacesUI should be present in the discovered addon list");
    assert!(
        shared_index < allied_index,
        "Blizzard_SharedXML (index {shared_index}) must load before Blizzard_AlliedRacesUI (index {allied_index})"
    );

    for (_, toc_path) in &addons {
        if let Some(stem) = toc_path.file_stem().and_then(|s| s.to_str())
            && (stem.starts_with("Blizzard_SharedXMLBase")
                || stem.starts_with("Blizzard_SharedXML")
                || stem.starts_with("Blizzard_AlliedRacesUI"))
        {
            load_addon(&env.loader_env(), toc_path)
                .unwrap_or_else(|err| panic!("{} load failed: {err}", toc_path.display()));
        }
    }

    let has_top_tile_streaks: bool = env
        .eval("return AlliedRacesFrame ~= nil and AlliedRacesFrame.TopTileStreaks ~= nil")
        .unwrap();
    assert!(
        has_top_tile_streaks,
        "AlliedRacesFrame.TopTileStreaks should resolve when SharedXML is ordered before AlliedRacesUI"
    );
}
