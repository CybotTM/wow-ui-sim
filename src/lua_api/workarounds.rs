//! Post-load workarounds that are still required on the live rilua path.

pub fn apply(env: &crate::lua_api::WowLuaEnv) {
    crate::lua_api::workarounds_editmode::patch_edit_mode_manager(env);
    crate::lua_api::workarounds_editmode::init_edit_mode_layout(env);
    patch_ui_parent_panel_toggles(env);
}

pub fn apply_post_event(_env: &crate::lua_api::WowLuaEnv) {}

pub fn apply_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if matches!(
        addon_name,
        "Blizzard_SharedTalentUI" | "Blizzard_PlayerSpells"
    ) {
        patch_shared_talent_util(env);
    }
}

fn patch_ui_parent_panel_toggles(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(GETGLOBAL_HELPER_LUA);
    let _ = env.exec(TOGGLE_ACHIEVEMENT_FRAME_LUA);
    let _ = env.exec(TOGGLE_ENCOUNTER_JOURNAL_LUA);
    let _ = env.exec(TOGGLE_COLLECTIONS_JOURNAL_LUA);
}

const GETGLOBAL_HELPER_LUA: &str = r#"
local function __wow_getglobal(name)
    return getglobal(name)
end
_G.__wow_panel_getglobal = __wow_getglobal
"#;

const TOGGLE_ACHIEVEMENT_FRAME_LUA: &str = r#"
if __wow_panel_getglobal ~= nil then
    local __wow_getglobal = __wow_panel_getglobal
    function ToggleAchievementFrame(stats)
        local kiosk = __wow_getglobal("Kiosk")
        if ( (kiosk and kiosk.IsEnabled and kiosk.IsEnabled()) or __wow_getglobal("DISALLOW_FRAME_TOGGLING") ) then
            return;
        end

        local cAddOns = __wow_getglobal("C_AddOns")
        if cAddOns and cAddOns.LoadAddOn and cAddOns.IsAddOnLoaded and not cAddOns.IsAddOnLoaded("Blizzard_AchievementUI") then
            cAddOns.LoadAddOn("Blizzard_AchievementUI");
        end
        local achievementFrame = __wow_getglobal("AchievementFrame")
        if not achievementFrame then
            return;
        end

        local requestedTab = stats and 3 or 1
        if achievementFrame:IsShown() and achievementFrame.selectedTab == requestedTab then
            achievementFrame:Hide();
        else
            achievementFrame.selectedTab = requestedTab
            achievementFrame:Show();
        end
    end
end
"#;

const TOGGLE_ENCOUNTER_JOURNAL_LUA: &str = r#"
if __wow_panel_getglobal ~= nil then
    local __wow_getglobal = __wow_panel_getglobal
    function ToggleEncounterJournal()
        local kiosk = __wow_getglobal("Kiosk")
        if ( (kiosk and kiosk.IsEnabled and kiosk.IsEnabled()) or __wow_getglobal("DISALLOW_FRAME_TOGGLING") ) then
            return;
        end

        if ( not __wow_getglobal("EncounterJournal") ) then
            local cAddOns = __wow_getglobal("C_AddOns")
            if cAddOns and cAddOns.LoadAddOn then
                cAddOns.LoadAddOn("Blizzard_EncounterJournal");
            end
        end
        local encounterJournal = __wow_getglobal("EncounterJournal")
        if ( encounterJournal ) then
            if encounterJournal:IsShown() then
                encounterJournal:Hide();
            else
                encounterJournal:Show();
            end
            return true;
        end
        return false;
    end
end
"#;

const TOGGLE_COLLECTIONS_JOURNAL_LUA: &str = r#"
if __wow_panel_getglobal ~= nil then
    local __wow_getglobal = __wow_panel_getglobal
    function ToggleCollectionsJournal(tabIndex)
        if __wow_getglobal("DISALLOW_FRAME_TOGGLING") then
            return;
        end

        local collectionsJournal = __wow_getglobal("CollectionsJournal")
        if not collectionsJournal then
            local cAddOns = __wow_getglobal("C_AddOns")
            if cAddOns and cAddOns.LoadAddOn then
                cAddOns.LoadAddOn("Blizzard_Collections");
            end
            collectionsJournal = __wow_getglobal("CollectionsJournal")
        end
        if not collectionsJournal then
            return
        end

        if collectionsJournal:IsShown() then
            collectionsJournal:Hide();
        else
            collectionsJournal:Show();
        end
    end
end
"#;

const SHARED_TALENT_UTIL_COMBINE_COST_ARRAYS_LUA: &str = r###"
if TalentUtil and type(TalentUtil.CombineCostArrays) == "function" and not TalentUtil.__wow_ui_sim_nil_safe_combine then
    local original = TalentUtil.CombineCostArrays
    function TalentUtil.CombineCostArrays(...)
        local combinedCostMap = {}
        for i = 1, select("#", ...) do
            local costArray = select(i, ...)
            if type(costArray) == "table" then
                for _, cost in ipairs(costArray) do
                    combinedCostMap[cost.ID] = (combinedCostMap[cost.ID] or 0) + cost.amount
                end
            end
        end

        local combinedCostArray = {}
        for ID, amount in pairs(combinedCostMap) do
            table.insert(combinedCostArray, { ID = ID, amount = amount })
        end
        return combinedCostArray
    end
    TalentUtil.__wow_ui_sim_nil_safe_combine = true
    TalentUtil.__wow_ui_sim_original_combine = original
end
"###;

fn patch_shared_talent_util(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = env.exec(SHARED_TALENT_UTIL_COMBINE_COST_ARRAYS_LUA);
}
