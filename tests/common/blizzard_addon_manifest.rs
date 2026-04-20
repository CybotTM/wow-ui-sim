#![allow(dead_code)]

use wow_ui_sim::loader::BlizzardAddonOverride;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BlizzardAddonSmokeShape {
    MostlyFunctional,
    TemplateHeavy,
    LayoutHeavy,
    MultiAddonFlow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlizzardAddonSmokeTarget<'a> {
    pub name: &'a str,
    pub shape: BlizzardAddonSmokeShape,
    pub roots: &'a [&'a str],
    pub overrides: &'a [BlizzardAddonOverride<'a>],
    pub required_addons: &'a [&'a str],
}

/// Shared non-TOC overrides for the world-map voice-button harness.
///
/// `Blizzard_Channels` is not pulled in by the isolated world-map roots
/// themselves, but the combined voice-button render-order checks need it
/// so Blizzard's implicit channel UI path is present.
pub const WORLD_MAP_VOICE_CHAT_OVERRIDES: &[BlizzardAddonOverride<'static>] =
    &[BlizzardAddonOverride {
        addon: "Blizzard_WorldMap",
        extra_roots: &["Blizzard_ChatFrame", "Blizzard_Channels"],
    }];

pub const COLLECTIONS_SMOKE_ROOTS: &[&str] = &["Blizzard_Collections"];
pub const SETTINGS_PANEL_SMOKE_ROOTS: &[&str] = &["Blizzard_SettingsDefinitions_Frame"];
pub const WORLD_MAP_SMOKE_ROOTS: &[&str] = &["Blizzard_WorldMap"];

pub const BLIZZARD_ADDON_SMOKE_TARGETS: &[BlizzardAddonSmokeTarget<'static>] = &[
    BlizzardAddonSmokeTarget {
        name: "combat_log",
        shape: BlizzardAddonSmokeShape::MostlyFunctional,
        roots: &["Blizzard_CombatLog"],
        overrides: &[],
        required_addons: &["Blizzard_CombatLog"],
    },
    BlizzardAddonSmokeTarget {
        name: "panel_templates",
        shape: BlizzardAddonSmokeShape::TemplateHeavy,
        roots: &["Blizzard_UIPanelTemplates"],
        overrides: &[],
        required_addons: &["Blizzard_UIPanelTemplates"],
    },
    BlizzardAddonSmokeTarget {
        name: "world_map",
        shape: BlizzardAddonSmokeShape::LayoutHeavy,
        roots: WORLD_MAP_SMOKE_ROOTS,
        overrides: &[],
        required_addons: &[
            "Blizzard_MapCanvas",
            "Blizzard_SharedMapDataProviders",
            "Blizzard_WorldMap",
        ],
    },
    BlizzardAddonSmokeTarget {
        name: "settings_panel",
        shape: BlizzardAddonSmokeShape::MultiAddonFlow,
        roots: SETTINGS_PANEL_SMOKE_ROOTS,
        overrides: &[],
        required_addons: &[
            "Blizzard_Settings_Shared",
            "Blizzard_SettingsDefinitions_Shared",
            "Blizzard_SettingsDefinitions_Frame",
        ],
    },
];
