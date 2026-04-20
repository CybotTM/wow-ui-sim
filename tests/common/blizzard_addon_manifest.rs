use wow_ui_sim::loader::BlizzardAddonOverride;

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
