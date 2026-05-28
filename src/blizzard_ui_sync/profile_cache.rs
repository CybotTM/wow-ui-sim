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
    "Blizzard_FrameXML/TBC/WorldStateFrame.lua",
    "Blizzard_MainMenuBarBagButtons/Classic/MainMenuBarBagButtons.lua",
    "Blizzard_MicroMenu/Blizzard_MicroMenu_Classic.toc",
    "Blizzard_MicroMenu/Shared/MicroMenuContainer.lua",
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
    "Blizzard_FrameXMLBase/Classic/Constants.lua",
    "Blizzard_FrameXMLBase/Classic/FrameLocks.lua",
    "Blizzard_FrameXMLBase/Classic/IconDataProvider.lua",
    "Blizzard_FrameXMLBase/TBC/Localization.lua",
    "Blizzard_FrameXMLBase/Wrath/Constants.lua",
    "Blizzard_FrameXML/Classic/AlertFrameSystems.lua",
    "Blizzard_FrameXML/Classic/AlertFrameSystems.xml",
    "Blizzard_FrameXML/Classic/AlertFrames.lua",
    "Blizzard_FrameXML/Classic/AlertFrames.xml",
    "Blizzard_FrameXML/Classic/CombatFeedback.lua",
    "Blizzard_BuffFrame/Classic/BuffFrame.lua",
    "Blizzard_GameTooltip/Blizzard_GameTooltip_Classic.toc",
    "Blizzard_GameTooltip/Classic/GameTooltip.lua",
    "Blizzard_GameTooltip/Classic/GameTooltip.xml",
    "Blizzard_ItemButton/Blizzard_ItemButton_Classic.toc",
    "Blizzard_ItemButton/Classic/ItemButtonTemplate.lua",
    "Blizzard_ItemButton/Classic/ItemButtonTemplate.xml",
    "Blizzard_LevelUpDisplay/Cata/LevelUpDisplay.lua",
    "Blizzard_NamePlates/Blizzard_NamePlates.toc",
    "Blizzard_Settings_Shared/Blizzard_Settings_Shared_Classic.toc",
    "Blizzard_Settings_Shared/Classic/AudioOverrides.lua",
    "Blizzard_Settings_Shared/Classic/Blizzard_SettingsPanelTemplates.xml",
    "Blizzard_Settings_Shared/Classic/GraphicsOverrides.lua",
    "Blizzard_UIPanelTemplates/Blizzard_UIPanelTemplates_Classic.toc",
    "Blizzard_UIPanelTemplates/Classic/AutoCastTemplates.lua",
    "Blizzard_UIPanelTemplates/Classic/AutoCastTemplates.xml",
    "Blizzard_UIPanelTemplates/Classic/UIPanelTemplates.lua",
    "Blizzard_UIPanelTemplates/Classic/UIPanelTemplates.xml",
    "Blizzard_UIPanels_Game/Blizzard_UIPanels_Game_Mists.toc",
    "Blizzard_UIPanels_Game/Classic/CastingBarFrame.lua",
    "Blizzard_UIPanels_Game/Classic/CastingBarFrame.xml",
    "Blizzard_UIPanels_Game/Shared/CastingBarFrame.lua",
    "Blizzard_UnitFrame/Cata/EclipseBarFrame.lua",
    "Blizzard_UnitFrame/Cata/EclipseBarFrame.xml",
    "Blizzard_UnitFrame/Cata/RuneFrame.lua",
    "Blizzard_UnitFrame/Cata/RuneFrame.xml",
    "Blizzard_UnitFrame/Classic/ComboFrame.lua",
    "Blizzard_UnitFrame/Classic/ComboFrame.xml",
    "Blizzard_UnitFrame/Classic/CompactUnitFrameOptions.lua",
    "Blizzard_UnitFrame/Classic/Localization.lua",
    "Blizzard_UnitFrame/Classic/PartyFrameTemplates.xml",
    "Blizzard_UnitFrame/Classic/PartyMemberFrame.lua",
    "Blizzard_UnitFrame/Classic/PetFrame.lua",
    "Blizzard_UnitFrame/Classic/PetFrame.xml",
    "Blizzard_UnitFrame/Classic/PlayerFrame.lua",
    "Blizzard_UnitFrame/Classic/PlayerFrame.xml",
    "Blizzard_UnitFrame/Classic/RuneFrame_Shared.lua",
    "Blizzard_UnitFrame/Classic/TargetFrame.lua",
    "Blizzard_UnitFrame/Classic/TargetFrame.xml",
    "Blizzard_UnitFrame/Classic/TotemFrame.lua",
    "Blizzard_UnitFrame/Classic/TotemFrame.xml",
    "Blizzard_UnitFrame/Classic/UnitFrame.lua",
    "Blizzard_UnitFrame/Classic/UnitPowerBarAlt.lua",
    "Blizzard_UnitFrame/Classic/UnitPowerBarAlt.xml",
    "Blizzard_UnitFrame/Mists/AlternatePowerBar.xml",
    "Blizzard_UnitFrame/Mists/MonkHarmonyBar.lua",
    "Blizzard_UnitFrame/Mists/MonkHarmonyBar.xml",
    "Blizzard_UnitFrame/Mists/MonkStaggerBar.lua",
    "Blizzard_UnitFrame/Mists/MonkStaggerBar.xml",
    "Blizzard_UnitFrame/Mists/PaladinPowerBar.lua",
    "Blizzard_UnitFrame/Mists/PaladinPowerBar.xml",
    "Blizzard_UnitFrame/Mists/PriestBar.xml",
    "Blizzard_UnitFrame/Mists/ShardBar.lua",
    "Blizzard_UnitFrame/Mists/ShardBar.xml",
    "Blizzard_UnitFrame/Mists/TotemFrame.lua",
    "Blizzard_UnitFrame/Wrath/AlternatePowerBar.lua",
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
        "Blizzard_MicroMenu/Blizzard_MicroMenu_Classic.toc" => file_contains(
            path,
            r#"Cata\MainMenuBarMicroButtons.xml [AllowLoadGameType mists]"#,
        ),
        "Blizzard_MicroMenu/Shared/MicroMenuContainer.lua" => {
            !file_contains(path, "PostAddButtonCallback")
        }
        "Blizzard_NamePlates/Blizzard_NamePlates.toc" => file_contains(
            path,
            "Blizzard_ClassNameplateBar.lua [AllowLoadGameType mainline]",
        ),
        "Blizzard_UIPanels_Game/Shared/CastingBarFrame.lua" => {
            file_contains(path, "function PlayerCastingBarMixin:OnShow()")
        }
        _ => true,
    }
}

fn file_contains(path: &Path, needle: &str) -> bool {
    std::fs::read_to_string(path).is_ok_and(|contents| contents.contains(needle))
}
