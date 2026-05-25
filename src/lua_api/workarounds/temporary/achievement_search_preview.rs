//! Temporary AchievementUI search/summary surface repairs.
//!
//! Blizzard_AchievementUI expects the search-preview array and summary empty
//! text state to be ready after the addon loads. Keep the repair in this
//! addon-specific temporary owner instead of the generic runtime bootstrap.

const ACHIEVEMENT_SEARCH_PREVIEW_LUA: &str = r##"
local function ensure_search_previews()
    local frame = AchievementFrame
    local container = frame and frame.SearchPreviewContainer
    if type(container) ~= "table" and type(container) ~= "userdata" then
        return
    end

    local previews = container.searchPreviews
    if type(previews) ~= "table" then
        previews = {}
        container.searchPreviews = previews
    end

    local count = ACHIEVEMENT_FRAME_NUM_SEARCH_PREVIEWS or 5
    for index = 1, count do
        if previews[index] == nil then
            previews[index] = container["SearchPreview" .. index]
        end
    end
end

local function patch_search_preview_selection()
    if rawget(_G, "__wow_achievement_search_preview_patched") then
        return
    end
    if type(AchievementFrame_SetSearchPreviewSelection) ~= "function" then
        return
    end

    local original = AchievementFrame_SetSearchPreviewSelection
    AchievementFrame_SetSearchPreviewSelection = function(selectedIndex)
        ensure_search_previews()
        return original(selectedIndex)
    end
    __wow_achievement_search_preview_patched = true
end

local function patch_summary_empty_text_overlap()
    if rawget(_G, "__wow_achievement_summary_empty_text_patched") then
        return
    end
    if type(AchievementFrameSummary_UpdateAchievements) ~= "function" then
        return
    end

    local original = AchievementFrameSummary_UpdateAchievements
    AchievementFrameSummary_UpdateAchievements = function(...)
        local numAchievements = select(1, ...)
        local results = { original(...) }

        local emptyText = rawget(_G, "AchievementFrameSummaryAchievementsEmptyText")
        local summary = rawget(_G, "AchievementFrameSummaryAchievements")
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

    __wow_achievement_summary_empty_text_patched = true
end

local function patch_achievement_ui()
    ensure_search_previews()
    patch_search_preview_selection()
    patch_summary_empty_text_overlap()
end

if C_AddOns and type(C_AddOns.LoadAddOn) == "function" then
    hooksecurefunc(C_AddOns, "LoadAddOn", function(addonName)
        if addonName == "Blizzard_AchievementUI" then
            patch_achievement_ui()
        end
    end)
end
"##;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(ACHIEVEMENT_SEARCH_PREVIEW_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn patches_achievement_search_preview_after_addon_load() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                local calls = {}
                C_AddOns.LoadAddOn("NotAchievementUI")

                AchievementFrame = {
                    SearchPreviewContainer = {
                        SearchPreview1 = "one",
                        SearchPreview2 = "two",
                    },
                }
                ACHIEVEMENT_FRAME_NUM_SEARCH_PREVIEWS = 2
                AchievementFrame_SetSearchPreviewSelection = function(selectedIndex)
                    calls[#calls + 1] = selectedIndex
                    return "selected"
                end
                AchievementFrameSummaryAchievementsEmptyText = {
                    shown = nil,
                    SetShown = function(self, shown) self.shown = shown end,
                }
                AchievementFrameSummaryAchievements = { buttons = {} }
                AchievementFrameSummary_UpdateAchievements = function(numAchievements)
                    return numAchievements
                end

                C_AddOns.LoadAddOn("Blizzard_AchievementUI")
                local selectionResult = AchievementFrame_SetSearchPreviewSelection(2)
                local summaryResult = AchievementFrameSummary_UpdateAchievements(0)
                local previews = AchievementFrame.SearchPreviewContainer.searchPreviews

                if selectionResult ~= "selected" or summaryResult ~= 0 then return "return" end
                if calls[1] ~= 2 then return "selection" end
                if previews[1] ~= "one" or previews[2] ~= "two" then return "previews" end
                if AchievementFrameSummaryAchievementsEmptyText.shown ~= true then return "empty" end
                if not __wow_achievement_search_preview_patched then return "search_patch" end
                if not __wow_achievement_summary_empty_text_patched then return "summary_patch" end

                return "ok"
                "#,
            )
            .expect("achievement search preview probe should run");

        assert_eq!(result, "ok");
    }
}
