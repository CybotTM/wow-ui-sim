use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::loader::{
    BlizzardAddonOverride, discover_blizzard_addon_closure_for_screen,
    discover_blizzard_addon_closure_for_screen_with_overrides, discover_blizzard_addons_for_screen,
};
use crate::screen::ScreenKind;

fn blizzard_ui_dir() -> PathBuf {
    crate::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
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
#[cfg(feature = "client-mists")]
fn mists_profile_skips_retail_only_deprecated_arena_ui() {
    let ui = TempBlizzardUiDir::new("mists-retail-only-addon");
    ui.add_addon(
        "Blizzard_Deprecated_ArenaUI",
        r#"
## Title: Blizzard_Deprecated_ArenaUI
## AllowLoad: Game
Deprecated_ArenaUI.xml
"#,
    );
    ui.add_addon(
        "Blizzard_UnitFrame",
        r#"
## Title: Blizzard_UnitFrame
## AllowLoad: Game
UnitFrame.lua
"#,
    );

    let addons: Vec<String> = discover_blizzard_addons_for_screen(&ui.path, ScreenKind::Game)
        .into_iter()
        .map(|(name, _)| name)
        .collect();

    assert_eq!(
        addons,
        vec!["Blizzard_UnitFrame"],
        "Mists should not load Blizzard_Deprecated_ArenaUI from the retail cache because it is absent from the Classic PTR/Mists source tree"
    );
}

#[test]
fn deprecated_chat_and_combat_compat_addons_load_after_base_addons() {
    let ui = TempBlizzardUiDir::new("deprecated-compat-order");
    for addon in [
        "Blizzard_DeprecatedChatInfo",
        "Blizzard_ChatFrameBase",
        "Blizzard_DeprecatedCombatLog",
        "Blizzard_CombatLogBase",
    ] {
        ui.add_addon(addon, "## AllowLoad: Game\nCore.lua\n");
    }

    let addons: Vec<String> = discover_blizzard_addons_for_screen(&ui.path, ScreenKind::Game)
        .into_iter()
        .map(|(name, _)| name)
        .collect();

    let index = |name: &str| addons.iter().position(|addon| addon == name).unwrap();
    assert!(index("Blizzard_ChatFrameBase") < index("Blizzard_DeprecatedChatInfo"));
    assert!(index("Blizzard_CombatLogBase") < index("Blizzard_DeprecatedCombatLog"));
}

#[test]
fn deprecated_combat_log_pulls_load_on_demand_combat_log_base() {
    let ui = TempBlizzardUiDir::new("deprecated-combat-log-lod");
    ui.add_addon(
        "Blizzard_DeprecatedCombatLog",
        "## AllowLoad: Game\nDeprecated.lua\n",
    );
    ui.add_addon(
        "Blizzard_CombatLogBase",
        "## AllowLoad: Game\n## LoadOnDemand: 1\nBase.lua\n",
    );

    let addons: Vec<String> = discover_blizzard_addons_for_screen(&ui.path, ScreenKind::Game)
        .into_iter()
        .map(|(name, _)| name)
        .collect();

    assert_eq!(
        addons,
        vec!["Blizzard_CombatLogBase", "Blizzard_DeprecatedCombatLog"],
        "implicit deprecated-combat-log dependency should pull the LOD base addon before loading deprecated globals"
    );
}

#[test]
fn objective_tracker_pulls_poi_button_owner_templates() {
    let ui = TempBlizzardUiDir::new("objective-tracker-poi-button");
    ui.add_addon(
        "Blizzard_ObjectiveTracker",
        "## AllowLoad: Game\nObjectiveTracker.lua\n",
    );
    ui.add_addon(
        "Blizzard_POIButton",
        "## AllowLoad: Game\n## LoadOnDemand: 1\nPOIButtonOwner.xml\n",
    );

    let addons: Vec<String> = discover_blizzard_addons_for_screen(&ui.path, ScreenKind::Game)
        .into_iter()
        .map(|(name, _)| name)
        .collect();

    assert_eq!(
        addons,
        vec!["Blizzard_POIButton", "Blizzard_ObjectiveTracker"],
        "ObjectiveTracker inherits POIButtonOwnerTemplate without declaring the addon"
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
