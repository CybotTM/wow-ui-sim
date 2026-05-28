use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::loader::{
    BlizzardAddonOverride, discover_blizzard_addon_closure_for_screen,
    discover_blizzard_addon_closure_for_screen_with_overrides, discover_blizzard_addons_for_screen,
};
use crate::screen::ScreenKind;

fn blizzard_ui_dir() -> PathBuf {
    crate::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}

fn addon_names(screen: ScreenKind) -> Vec<String> {
    discover_blizzard_addons_for_screen(&blizzard_ui_dir(), screen)
        .into_iter()
        .map(|(name, _)| name)
        .collect()
}

#[test]
fn game_screen_discovers_game_addons_not_glue_base() {
    let addons = addon_names(ScreenKind::Game);
    assert!(addons.iter().any(|name| name == "Blizzard_UIParent"));
    assert!(!addons.iter().any(|name| name == "Blizzard_GlueParent"));
    assert!(!addons.iter().any(|name| name == "Blizzard_GlueXML"));
}

#[test]
fn login_screen_discovers_glue_addons_not_game_base() {
    let addons = addon_names(ScreenKind::Login);
    assert!(addons.iter().any(|name| name == "Blizzard_GlueParent"));
    assert!(addons.iter().any(|name| name == "Blizzard_GlueXML"));
    assert!(!addons.iter().any(|name| name == "Blizzard_UIParent"));
    assert!(!addons.iter().any(|name| name == "Blizzard_CharacterCreate"));
    assert!(
        !addons
            .iter()
            .any(|name| name == "Blizzard_CharacterCustomize")
    );
}

#[test]
fn character_select_screen_uses_glue_addon_set() {
    let addons = addon_names(ScreenKind::CharacterSelect);
    assert!(addons.iter().any(|name| name == "Blizzard_GlueParent"));
    assert!(addons.iter().any(|name| name == "Blizzard_GlueXML"));
    assert!(!addons.iter().any(|name| name == "Blizzard_UIParent"));
    assert!(addons.iter().any(|name| name == "Blizzard_CharacterCreate"));
}

#[test]
fn character_create_screen_uses_glue_addon_set() {
    let addons = addon_names(ScreenKind::CharacterCreate);
    assert!(addons.iter().any(|name| name == "Blizzard_GlueParent"));
    assert!(addons.iter().any(|name| name == "Blizzard_GlueXML"));
    assert!(!addons.iter().any(|name| name == "Blizzard_UIParent"));
    assert!(addons.iter().any(|name| name == "Blizzard_CharacterCreate"));
}

struct TempBlizzardUiDir {
    path: PathBuf,
}

impl TempBlizzardUiDir {
    fn new(suffix: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("wow-sim-blizzard-ui-{suffix}-{unique}"));
        std::fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    fn add_addon(&self, name: &str, toc_contents: &str) {
        let addon_dir = self.path.join(name);
        std::fs::create_dir_all(&addon_dir).unwrap();
        std::fs::write(addon_dir.join(format!("{name}.toc")), toc_contents).unwrap();
    }
}

impl Drop for TempBlizzardUiDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[test]
fn load_first_addons_sort_ahead_of_non_load_first_addons() {
    let ui = TempBlizzardUiDir::new("load-first-order");
    ui.add_addon(
        "Blizzard_Z_LoadFirst",
        r#"
## Title: Blizzard_Z_LoadFirst
## AllowLoad: Both
## LoadFirst: 1
Core.lua
"#,
    );
    ui.add_addon(
        "Blizzard_A_Normal",
        r#"
## Title: Blizzard_A_Normal
## AllowLoad: Both
Core.lua
"#,
    );

    let addons: Vec<String> = discover_blizzard_addons_for_screen(&ui.path, ScreenKind::Game)
        .into_iter()
        .map(|(name, _)| name)
        .collect();

    assert_eq!(
        addons,
        vec!["Blizzard_Z_LoadFirst", "Blizzard_A_Normal"],
        "LoadFirst addons should sort before normal addons",
    );
}

#[test]
fn cyclic_addons_still_emit_dependencies_before_load_first_addon() {
    let ui = TempBlizzardUiDir::new("load-first-cycle");
    ui.add_addon(
        "Blizzard_A_Normal",
        r#"
## Title: Blizzard_A_Normal
## AllowLoad: Both
## Dependencies: Blizzard_Z_LoadFirst
Core.lua
"#,
    );
    ui.add_addon(
        "Blizzard_Z_LoadFirst",
        r#"
## Title: Blizzard_Z_LoadFirst
## AllowLoad: Both
## LoadFirst: 1
## Dependencies: Blizzard_A_Normal
Core.lua
"#,
    );

    let addons: Vec<String> = discover_blizzard_addons_for_screen(&ui.path, ScreenKind::Game)
        .into_iter()
        .map(|(name, _)| name)
        .collect();

    assert_eq!(
        addons,
        vec!["Blizzard_A_Normal", "Blizzard_Z_LoadFirst"],
        "LoadFirst triggers first-pass loading, but declared dependencies still emit first",
    );
}

#[test]
fn addon_closure_resolves_transitive_optional_dependencies_for_roots() {
    let ui = TempBlizzardUiDir::new("closure");
    ui.add_addon(
        "Blizzard_B",
        r#"
## Title: Blizzard_B
## AllowLoad: Both
Core.lua
"#,
    );
    ui.add_addon(
        "Blizzard_C",
        r#"
## Title: Blizzard_C
## AllowLoad: Both
Core.lua
"#,
    );
    ui.add_addon(
        "Blizzard_D",
        r#"
## Title: Blizzard_D
## AllowLoad: Glue
Core.lua
"#,
    );
    ui.add_addon(
        "Blizzard_A",
        r#"
## Title: Blizzard_A
## AllowLoad: Both
## Dependencies: Blizzard_B
## OptionalDeps: Blizzard_C, Blizzard_D
Core.lua
"#,
    );

    let addons =
        discover_blizzard_addon_closure_for_screen(&ui.path, ScreenKind::Game, &["Blizzard_A"]);
    let names: Vec<String> = addons.into_iter().map(|(name, _)| name).collect();

    assert_eq!(
        names,
        vec!["Blizzard_B", "Blizzard_C", "Blizzard_A"],
        "dependency closure should include required and screen-allowed optional TOC deps only",
    );
    assert!(
        !names.iter().any(|name| name == "Blizzard_D"),
        "screen-filtered optional dependencies should not be included in the closure"
    );
}

#[test]
fn addon_closure_includes_load_on_demand_roots_and_their_dependencies() {
    let ui = TempBlizzardUiDir::new("lod-closure");
    ui.add_addon(
        "Blizzard_Dependency",
        r#"
## Title: Blizzard_Dependency
## AllowLoad: Both
Core.lua
"#,
    );
    ui.add_addon(
        "Blizzard_Optional",
        r#"
## Title: Blizzard_Optional
## AllowLoad: Both
Core.lua
"#,
    );
    ui.add_addon(
        "Blizzard_LoadOnDemandRoot",
        r#"
## Title: Blizzard_LoadOnDemandRoot
## AllowLoad: Both
## LoadOnDemand: 1
## Dependencies: Blizzard_Dependency
## OptionalDeps: Blizzard_Optional
Core.lua
"#,
    );

    let addons = discover_blizzard_addon_closure_for_screen(
        &ui.path,
        ScreenKind::Game,
        &["Blizzard_LoadOnDemandRoot"],
    );
    let names: Vec<String> = addons.into_iter().map(|(name, _)| name).collect();

    assert_eq!(
        names,
        vec![
            "Blizzard_Dependency",
            "Blizzard_Optional",
            "Blizzard_LoadOnDemandRoot",
        ],
        "load-on-demand roots should resolve against the full screen-allowed TOC set",
    );
}

#[test]
fn addon_closure_applies_override_manifest_extras_transitively() {
    let ui = TempBlizzardUiDir::new("closure-overrides");
    ui.add_addon(
        "Blizzard_B",
        r#"
## Title: Blizzard_B
## AllowLoad: Both
Core.lua
"#,
    );
    ui.add_addon(
        "Blizzard_C",
        r#"
## Title: Blizzard_C
## AllowLoad: Both
Core.lua
"#,
    );
    ui.add_addon(
        "Blizzard_A",
        r#"
## Title: Blizzard_A
## AllowLoad: Both
## Dependencies: Blizzard_B
Core.lua
"#,
    );

    let overrides = &[BlizzardAddonOverride {
        addon: "Blizzard_A",
        extra_roots: &["Blizzard_C"],
    }];

    let addons = discover_blizzard_addon_closure_for_screen_with_overrides(
        &ui.path,
        ScreenKind::Game,
        &["Blizzard_A"],
        overrides,
    );
    let names: Vec<String> = addons.into_iter().map(|(name, _)| name).collect();

    assert_eq!(
        names,
        vec!["Blizzard_B", "Blizzard_C", "Blizzard_A"],
        "override manifest extras should participate in the explicit closure",
    );
}

#[test]
fn real_glue_load_first_addons_sort_before_non_load_first_addons() {
    let addons = addon_names(ScreenKind::Login);

    let glue_menu_frame = addons
        .iter()
        .position(|name| name == "Blizzard_GlueMenuFrame")
        .expect("Blizzard_GlueMenuFrame should be present");
    let login_warning_dialogs = addons
        .iter()
        .position(|name| name == "Blizzard_LoginWarningDialogs")
        .expect("Blizzard_LoginWarningDialogs should be present");

    assert!(
        glue_menu_frame < login_warning_dialogs,
        "Blizzard_GlueMenuFrame is tagged LoadFirst and should sort before comparable non-LoadFirst glue addons",
    );
}
