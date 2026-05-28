use std::path::PathBuf;

pub(super) fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}

pub(super) fn major_factions_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_MajorFactions")
}

pub(super) fn major_factions_toc() -> PathBuf {
    major_factions_dir().join("Blizzard_MajorFactions.toc")
}

pub(super) const MAJOR_FACTIONS_TOC_FILES: &[&str] = &[
    "Blizzard_MajorFactionsLandingTemplates.xml",
    "Blizzard_MajorFactionToasts.xml",
    "Blizzard_MajorFactionUnlockToast.lua",
    "Blizzard_MajorFactionUnlockToast.xml",
    "Blizzard_MajorFactionRenownToast.lua",
    "Blizzard_MajorFactionRenownToast.xml",
    "Localization.lua",
];

pub(super) const MAJOR_FACTION_LIST_MIXIN_METHODS: &[&str] = &[
    "OnLoad",
    "OnShow",
    "OnHide",
    "OnEvent",
    "Refresh",
    "SetExpansionFilter",
    "OnRenownTrackFactionChanged",
    "SetSelectedFaction",
    "ScrollToSelectedFaction",
];

pub(super) const MAJOR_FACTION_BUTTON_MIXIN_METHODS: &[&str] = &["Init", "UpdateState"];

pub(super) const MAJOR_FACTION_BUTTON_LOCKED_STATE_MIXIN_METHODS: &[&str] =
    &["OnEnter", "OnLeave", "Refresh"];

pub(super) const MAJOR_FACTION_BUTTON_UNLOCKED_STATE_MIXIN_METHODS: &[&str] = &[
    "Refresh",
    "OnShow",
    "OnHide",
    "OnEvent",
    "OnEnter",
    "OnLeave",
    "OnClick",
    "OnUpdate",
    "SetSelected",
    "RefreshTooltip",
    "ShowRenownRewardsTooltip",
    "ShowParagonRewardsTooltip",
    "PlayUnlockCelebration",
    "StopUnlockCelebration",
];

pub(super) const MAJOR_FACTION_WATCH_FACTION_BUTTON_MIXIN_METHODS: &[&str] = &[
    "OnLoad",
    "OnShow",
    "OnHide",
    "OnEvent",
    "UpdateState",
    "OnClick",
];

pub(super) const MAJOR_FACTION_UNLOCK_TOAST_MIXIN_METHODS: &[&str] = &[
    "OnLoad",
    "OnEvent",
    "OnHide",
    "PlayMajorFactionUnlockToast",
    "PlayBanner",
    "StopBanner",
    "OnAnimFinished",
];

pub(super) const MAJOR_FACTIONS_RENOWN_TOAST_MIXIN_METHODS: &[&str] = &[
    "OnLoad",
    "OnEvent",
    "OnHide",
    "ShowRenownLevelUpToast",
    "SetupRewardVisuals",
    "PlayBanner",
    "OnMouseEnter",
    "OnMouseLeave",
    "RefreshTooltip",
    "StopBanner",
    "OnAnimFinished",
];

pub(super) const NAMED_MAJOR_FACTION_FRAMES: &[&str] =
    &["MajorFactionUnlockToast", "MajorFactionsRenownToast"];
