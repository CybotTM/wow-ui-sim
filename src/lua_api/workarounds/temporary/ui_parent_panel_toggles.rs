//! Temporary UIParent panel toggle helpers.
//!
//! Several Blizzard panel entry points are normally installed by lazy-loaded
//! addons. The simulator still needs these compatibility toggles during
//! startup and runtime addon loading until panel ownership is modeled more
//! directly.

use crate::lua_api::{LoaderEnv, WowLuaEnv};

const GETGLOBAL_HELPER_LUA: &str = r#"
local function __wow_getglobal(name)
    return getglobal(name)
end
_G.__wow_panel_getglobal = __wow_getglobal
"#;

const TOGGLE_ACHIEVEMENT_FRAME_LUA: &str = r#"
if __wow_panel_getglobal ~= nil then
    local __wow_getglobal = __wow_panel_getglobal

    local function __wow_patch_summary_empty_text_overlap()
        if rawget(_G, "__wow_achievement_summary_empty_text_patched") then
            return
        end
        if type(AchievementFrameSummary_UpdateAchievements) ~= "function" then
            return
        end

        local original = AchievementFrameSummary_UpdateAchievements
        AchievementFrameSummary_UpdateAchievements = function(...)
            local numAchievements = select('#', ...)
            local results = { original(...) }
            local emptyText = __wow_getglobal("AchievementFrameSummaryAchievementsEmptyText")
            local summary = __wow_getglobal("AchievementFrameSummaryAchievements")
            local buttons = summary and summary.buttons
            local hasVisibleSummaryButton = false

            if type(buttons) == "table" then
                for _, button in ipairs(buttons) do
                    if (type(button) == "table" or type(button) == "userdata")
                        and type(button.IsShown) == "function"
                        and button:IsShown()
                    then
                        hasVisibleSummaryButton = true
                        break
                    end
                end
            end

            if (type(emptyText) == "table" or type(emptyText) == "userdata")
                and type(emptyText.SetShown) == "function"
            then
                emptyText:SetShown(numAchievements == 0 and not hasVisibleSummaryButton)
            end

            return unpack(results)
        end

        rawset(_G, "__wow_achievement_summary_empty_text_patched", true)
    end

    function ToggleAchievementFrame(stats, toggleGuildView)
        local kiosk = __wow_getglobal("Kiosk")
        if ( (kiosk and kiosk.IsEnabled and kiosk.IsEnabled()) or __wow_getglobal("DISALLOW_FRAME_TOGGLING") ) then
            return;
        end

        local cAddOns = __wow_getglobal("C_AddOns")
        if cAddOns and cAddOns.LoadAddOn and cAddOns.IsAddOnLoaded and not cAddOns.IsAddOnLoaded("Blizzard_AchievementUI") then
            cAddOns.LoadAddOn("Blizzard_AchievementUI");
        end
        __wow_patch_summary_empty_text_overlap()

        local achievementFrame = __wow_getglobal("AchievementFrame")
        if not achievementFrame then
            return;
        end

        local achievementToggle = __wow_getglobal("AchievementFrame_ToggleAchievementFrame")
        if type(achievementToggle) == "function" then
            return achievementToggle(stats, toggleGuildView)
        end

        local requestedTab = stats and 3 or 1
        if achievementFrame:IsShown() and achievementFrame.selectedTab == requestedTab then
            local hideUIPanel = __wow_getglobal("HideUIPanel")
            if type(hideUIPanel) == "function" then
                hideUIPanel(achievementFrame)
            else
                achievementFrame:Hide();
            end
        else
            achievementFrame.selectedTab = requestedTab
            local showUIPanel = __wow_getglobal("ShowUIPanel")
            if type(showUIPanel) == "function" then
                showUIPanel(achievementFrame)
            else
                achievementFrame:Show();
            end
        end
    end
end
"#;

const TOGGLE_ENCOUNTER_JOURNAL_LUA: &str = r#"
function ToggleEncounterJournal()
    if DISALLOW_FRAME_TOGGLING then
        return
    end
    if not EncounterJournal and type(EncounterJournal_LoadUI) == "function" then
        EncounterJournal_LoadUI()
    end
    if not EncounterJournal and type(C_AddOns) == "table" and type(C_AddOns.LoadAddOn) == "function" then
        C_AddOns.LoadAddOn("Blizzard_EncounterJournal")
    end
    if EncounterJournal then
        if EncounterJournal:IsShown() then
            if type(HideUIPanel) == "function" then
                HideUIPanel(EncounterJournal)
            else
                EncounterJournal:Hide()
            end
        else
            if type(ShowUIPanel) == "function" then
                ShowUIPanel(EncounterJournal)
            else
                EncounterJournal:Show()
            end
        end
        return true;
    end
    return false;
end
"#;

const TOGGLE_COLLECTIONS_JOURNAL_LUA: &str = r#"
function ToggleCollectionsJournal(tabIndex)
    if DISALLOW_FRAME_TOGGLING then
        return
    end
    if not CollectionsJournal and type(CollectionsJournal_LoadUI) == "function" then
        CollectionsJournal_LoadUI()
    end
    if not CollectionsJournal and type(C_AddOns) == "table" and type(C_AddOns.LoadAddOn) == "function" then
        C_AddOns.LoadAddOn("Blizzard_Collections")
    end
    if not CollectionsJournal then
        return
    end

    if type(SetCollectionsJournalShown) == "function" then
        local tabMatches = not tabIndex or tabIndex == PanelTemplates_GetSelectedTab(CollectionsJournal)
        local isShown = CollectionsJournal:IsShown() and tabMatches
        SetCollectionsJournalShown(not isShown, tabIndex)
    elseif CollectionsJournal:IsShown() then
        if type(HideUIPanel) == "function" then
            HideUIPanel(CollectionsJournal)
        else
            CollectionsJournal:Hide()
        end
    else
        if type(ShowUIPanel) == "function" then
            ShowUIPanel(CollectionsJournal)
        else
            CollectionsJournal:Show()
        end
    end
end
"#;

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(GETGLOBAL_HELPER_LUA);
    let _ = env.exec(TOGGLE_ACHIEVEMENT_FRAME_LUA);
    let _ = env.exec(TOGGLE_ENCOUNTER_JOURNAL_LUA);
    let _ = env.exec(TOGGLE_COLLECTIONS_JOURNAL_LUA);
}

pub(crate) fn patch_collections_journal_loader(env: &LoaderEnv<'_>) {
    let _ = env.exec(TOGGLE_COLLECTIONS_JOURNAL_LUA);
}

pub(crate) fn patch_encounter_journal_loader(env: &LoaderEnv<'_>) {
    let _ = env.exec(TOGGLE_ENCOUNTER_JOURNAL_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_getglobal_helper_and_toggle_functions() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        patch(&env);

        let helper_and_toggles_installed: bool = env
            .eval(
                r#"
                ShowUIPanel = function() end
                return __wow_panel_getglobal("ShowUIPanel") == ShowUIPanel
                    and type(ToggleAchievementFrame) == "function"
                    and type(ToggleEncounterJournal) == "function"
                    and type(ToggleCollectionsJournal) == "function"
                "#,
            )
            .expect("panel toggle helpers should be readable");

        assert!(helper_and_toggles_installed);
    }

    #[test]
    fn toggle_achievement_frame_loads_and_shows_frame() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            loaded_addon = nil
            shown_frame = nil
            C_AddOns = {
                IsAddOnLoaded = function()
                    return false
                end,
                LoadAddOn = function(addonName)
                    loaded_addon = addonName
                    AchievementFrame = {
                        selectedTab = nil,
                        shown = false,
                        IsShown = function(self)
                            return self.shown
                        end,
                        Show = function(self)
                            self.shown = true
                        end,
                        Hide = function(self)
                            self.shown = false
                        end,
                    }
                end,
            }
            ShowUIPanel = function(frame)
                shown_frame = frame
                frame:Show()
            end
            "#,
        )
        .expect("achievement-frame fixture should install");

        patch(&env);

        let (loaded_addon, selected_tab, was_shown): (String, i64, bool) = env
            .eval(
                r#"
                ToggleAchievementFrame(true, false)
                return loaded_addon, AchievementFrame.selectedTab, shown_frame == AchievementFrame
                "#,
            )
            .expect("achievement toggle should load and show frame");

        assert_eq!(loaded_addon, "Blizzard_AchievementUI");
        assert_eq!(selected_tab, 3);
        assert!(was_shown);
    }

    #[test]
    fn toggle_encounter_journal_uses_panel_show_and_hide() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            EncounterJournal = {
                shown = false,
                IsShown = function(self)
                    return self.shown
                end,
                Show = function(self)
                    self.shown = true
                end,
                Hide = function(self)
                    self.shown = false
                end,
            }
            ShowUIPanel = function(frame)
                frame:Show()
            end
            HideUIPanel = function(frame)
                frame:Hide()
            end
            "#,
        )
        .expect("encounter-journal fixture should install");

        patch(&env);

        let (first_result, shown_after_first, second_result, shown_after_second): (
            bool,
            bool,
            bool,
            bool,
        ) = env
            .eval(
                r#"
                local firstResult = ToggleEncounterJournal()
                local shownAfterFirst = EncounterJournal:IsShown()
                local secondResult = ToggleEncounterJournal()
                return firstResult, shownAfterFirst, secondResult, EncounterJournal:IsShown()
                "#,
            )
            .expect("encounter toggle should show and hide frame");

        assert!(first_result);
        assert!(shown_after_first);
        assert!(second_result);
        assert!(!shown_after_second);
    }

    #[test]
    fn toggle_collections_journal_uses_collection_panel_state_helper() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            CollectionsJournal = {
                shown = false,
                IsShown = function(self)
                    return self.shown
                end,
            }
            PanelTemplates_GetSelectedTab = function()
                return 2
            end
            requested_shown = nil
            requested_tab = nil
            SetCollectionsJournalShown = function(shown, tabIndex)
                requested_shown = shown
                requested_tab = tabIndex
            end
            "#,
        )
        .expect("collections-journal fixture should install");

        patch(&env);

        let (requested_shown, requested_tab): (bool, i64) = env
            .eval(
                r#"
                ToggleCollectionsJournal(3)
                return requested_shown, requested_tab
                "#,
            )
            .expect("collections toggle should use shown helper");

        assert!(requested_shown);
        assert_eq!(requested_tab, 3);
    }
}
