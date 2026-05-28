use std::path::Path;

const DEFAULT_GETHE_WOW_UI_SOURCE_BRANCHES: &[&str] = &[
    "live",
    "beta",
    "classic",
    "classic_era",
    "classic_anniversary",
    "classic_beta",
    "classic_ptr",
];

const MISTS_GETHE_WOW_UI_SOURCE_BRANCHES: &[&str] = &[
    "classic_ptr",
    "classic_anniversary",
    "classic_beta",
    "classic",
    "classic_era",
    "live",
    "beta",
];

pub(super) const MISTS_REQUIRED_PROFILE_CACHE_ENTRIES: &[&str] = &[
    "Blizzard_ActionBar/Classic/ActionButtonTemplate.xml",
    "Blizzard_ActionBar/Classic/ActionButtonUtilOverrides.lua",
    "Blizzard_ActionBar/Classic/ExpBar.xml",
    "Blizzard_ActionBar/Classic/ExpBarOverrides.lua",
    "Blizzard_ActionBar/Classic/MainActionBar.xml",
    "Blizzard_ActionBar/Classic/MainActionBarOverrides.lua",
    "Blizzard_ActionBar/Classic/MainMenuBar.lua",
    "Blizzard_ActionBar/Classic/MainMenuBar.xml",
    "Blizzard_ActionBar/Classic/PetActionBar.xml",
    "Blizzard_ActionBar/Classic/PossessActionBar.xml",
    "Blizzard_ActionBar/Classic/ReputationBarOverrides.lua",
    "Blizzard_ActionBar/Classic/StanceBar.xml",
    "Blizzard_ActionBar/Classic/StanceBarOverrides.lua",
    "Blizzard_ActionBar/Classic/StatusTrackingBar.xml",
    "Blizzard_ActionBar/Classic/StatusTrackingBarTemplate.xml",
    "Blizzard_ActionBar/Classic/StatusTrackingManagerOverrides.lua",
    "Blizzard_FrameXMLUtil/Blizzard_FrameXMLUtil_Classic.toc",
    "Blizzard_FrameXMLUtil/Classic/ArenaUtil.lua",
    "Blizzard_FrameXMLUtil/Classic/AuraUtil.lua",
    "Blizzard_FrameXMLUtil/Classic/Cooldown.xml",
    "Blizzard_FrameXMLUtil/Classic/MapUtil.lua",
    "Blizzard_FrameXMLUtil/Classic/QuestUtils.lua",
    "Blizzard_FrameXMLUtil/Classic/RaidWarning.lua",
    "Blizzard_FrameXMLUtil/Classic/TransmogUtil.lua",
    "Blizzard_MainMenuBarBagButtons/Classic/MainMenuBarBagButtons.lua",
    "Blizzard_SharedXML/Classic/ClassicCvarUtil.lua",
    "Blizzard_SharedXML/Classic/Dialog/DialogTemplates.xml",
    "Blizzard_SharedXML/Classic/Frame/MainMenuFrameTemplates.xml",
    "Blizzard_SharedXML/Classic/GameTooltipTemplate.xml",
    "Blizzard_SharedXML/Classic/GlueCheck.lua",
    "Blizzard_SharedXML/Classic/ModelFrames.lua",
    "Blizzard_SharedXML/Classic/ModelFrames.xml",
    "Blizzard_SharedXML/Classic/NineSliceLayouts.lua",
    "Blizzard_SharedXML/Classic/Scroll/TrimScrollBar.xml",
    "Blizzard_SharedXML/Classic/ScrollDefine.lua",
    "Blizzard_SharedXML/Classic/ScrollDefine.xml",
    "Blizzard_SharedXML/Classic/SecureCurrencyUtil.lua",
    "Blizzard_SharedXML/Classic/Selector/Blizzard_ScrollBoxSelector.xml",
    "Blizzard_SharedXML/Classic/SharedUIPanelTemplates.lua",
    "Blizzard_SharedXML/Classic/SharedUIPanelTemplates.xml",
    "Blizzard_SharedXML/Classic/SharedUtils.lua",
    "Blizzard_SharedXML/Classic/SliderTemplates.xml",
    "Blizzard_SharedXML/Classic/Sound.lua",
    "Blizzard_SharedXML/Classic/Stubs.lua",
    "Blizzard_SharedXML/Classic/Stubs.xml",
    "Blizzard_SharedXML/Classic/UIDropDownMenu.lua",
    "Blizzard_SharedXML/Classic/UIDropDownMenu.xml",
    "Blizzard_SharedXML/Classic/UIDropDownMenuTemplates.lua",
    "Blizzard_SharedXML/Classic/UIDropDownMenuTemplates.xml",
    "Blizzard_SharedXML/TBC/ClassColors.lua",
    "Blizzard_SharedXML/Wrath/SoundKitConstants.lua",
    "Blizzard_UIPanelTemplates/Blizzard_UIPanelTemplates_Classic.toc",
    "Blizzard_UIPanelTemplates/Classic/AutoCastTemplates.lua",
    "Blizzard_UIPanelTemplates/Classic/AutoCastTemplates.xml",
    "Blizzard_UIPanelTemplates/Classic/UIPanelTemplates.lua",
    "Blizzard_UIPanelTemplates/Classic/UIPanelTemplates.xml",
];

pub(super) fn required_profile_cache_entries() -> &'static [&'static str] {
    match crate::client_profile::ACTIVE {
        crate::client_profile::ClientProfile::Mists => MISTS_REQUIRED_PROFILE_CACHE_ENTRIES,
        _ => &[],
    }
}

pub(super) fn gethe_wow_ui_source_branches() -> &'static [&'static str] {
    match crate::client_profile::ACTIVE {
        crate::client_profile::ClientProfile::Mists => MISTS_GETHE_WOW_UI_SOURCE_BRANCHES,
        _ => DEFAULT_GETHE_WOW_UI_SOURCE_BRANCHES,
    }
}

pub(super) fn cache_entry_is_usable(entry: &str, path: &Path) -> bool {
    match crate::client_profile::ACTIVE {
        crate::client_profile::ClientProfile::Mists => mists_cache_entry_is_usable(entry, path),
        _ => true,
    }
}

fn mists_cache_entry_is_usable(entry: &str, path: &Path) -> bool {
    match entry {
        "Blizzard_ActionBar/Classic/ActionButtonTemplate.xml" => {
            file_contains(path, "ActionBarButtonTemplate")
                && file_contains(path, r#"parentKey="chargeCooldown""#)
        }
        _ => true,
    }
}

fn file_contains(path: &Path, needle: &str) -> bool {
    std::fs::read_to_string(path).is_ok_and(|contents| contents.contains(needle))
}
