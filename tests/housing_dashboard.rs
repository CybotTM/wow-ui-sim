mod common;

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::startup::process_pending_timers;

fn setup_env() -> WowLuaEnv {
    common::panel_fixtures::setup_env()
}

#[test]
fn start_tutorial_button_opens_house_finder() {
    test_timeout! {
        let env = setup_env();
        let result: String = env
            .eval(
                r#"
                    local tutorialsLoaded, tutorialsReason = C_AddOns.LoadAddOn("Blizzard_HousingTutorials")
                    if not tutorialsLoaded then
                        return "tutorials_load_failed:" .. tostring(tutorialsReason)
                    end

                    local loaded, reason = C_AddOns.LoadAddOn("Blizzard_HousingDashboard")
                    if not loaded then
                        return "dashboard_load_failed:" .. tostring(reason)
                    end

                    ShowUIPanel(HousingDashboardFrame)
                    local button = HousingDashboardFrame.HouseInfoContent.DashboardNoHousesFrame.NoHouseButton
                    if not button or button:GetText() ~= HOUSING_DASHBOARD_START_TUTORIAL_BUTTON_TEXT then
                        return "missing_start_tutorial_button"
                    end

                    local onclick = button:GetScript("OnClick")
                    if not onclick then
                        return "missing_onclick"
                    end

                    local ok, err = pcall(function()
                        onclick(button, "LeftButton", false)
                    end)
                    if not ok then
                        return "click_failed:" .. tostring(err)
                    end

                    if not HouseFinderFrame or not HouseFinderFrame:IsShown() then
                        return "house_finder_not_shown"
                    end

                    if HousingDashboardFrame:IsShown() then
                        return "dashboard_still_shown"
                    end

                    return "ok"
                "#,
            )
            .unwrap();
        assert_eq!(
            result, "ok",
            "Housing tutorial button should advance to the house finder: {result}"
        );
    }
}

#[test]
fn neighborhood_selector_populates_and_click_loads_selected_map() {
    test_timeout! {
        let env = setup_env();
        let click_result: String = env
            .eval(
                r#"
                    local tutorialsLoaded, tutorialsReason = C_AddOns.LoadAddOn("Blizzard_HousingTutorials")
                    if not tutorialsLoaded then
                        return "tutorials_load_failed:" .. tostring(tutorialsReason)
                    end

                    local loaded, reason = C_AddOns.LoadAddOn("Blizzard_HousingDashboard")
                    if not loaded then
                        return "dashboard_load_failed:" .. tostring(reason)
                    end

                    ShowUIPanel(HousingDashboardFrame)
                    local startButton = HousingDashboardFrame.HouseInfoContent.DashboardNoHousesFrame.NoHouseButton
                    local onclick = startButton and startButton:GetScript("OnClick")
                    if not onclick then
                        return "missing_start_tutorial_onclick"
                    end
                    local ok, err = pcall(function()
                        onclick(startButton, "LeftButton", false)
                    end)
                    if not ok then
                        return "start_tutorial_click_failed:" .. tostring(err)
                    end

                    local firstButton = nil
                    local secondButton = nil
                    for button in HouseFinderFrame.neighborhoodButtonPool:EnumerateActive() do
                        if button.layoutIndex == 1 then
                            firstButton = button
                        elseif button.layoutIndex == 2 then
                            secondButton = button
                        end
                    end

                    if not firstButton or not secondButton then
                        return "missing_neighborhood_buttons"
                    end
                    if firstButton.neighborhoodInfo.neighborhoodName ~= "Dawnmeadow" then
                        return "wrong_first_neighborhood:" .. tostring(firstButton.neighborhoodInfo.neighborhoodName)
                    end
                    if secondButton.neighborhoodInfo.neighborhoodName ~= "Umber Grove" then
                        return "wrong_second_neighborhood:" .. tostring(secondButton.neighborhoodInfo.neighborhoodName)
                    end

                    secondButton:OnClick()

                    if HouseFinderFrame.selectedNeighborhoodButton ~= secondButton then
                        return "second_neighborhood_not_selected"
                    end
                    if HouseFinderFrame.LoadingSpinnerMap:IsShown() then
                        return "clicked"
                    end

                    return "spinner_not_shown_after_click"
                "#,
            )
            .unwrap();
        assert_eq!(
            click_result, "clicked",
            "Neighborhood selector should request selected map data: {click_result}"
        );

        process_pending_timers(&env);

        let map_result: String = env
            .eval(
                r#"
                    if not HouseFinderFrame.HouseFinderMapCanvasFrame:IsShown() then
                        return "map_not_shown"
                    end
                    if HouseFinderFrame.LoadingSpinnerMap:IsShown() then
                        return "map_spinner_still_shown"
                    end
                    return "ok"
                "#,
            )
            .unwrap();
        assert_eq!(
            map_result, "ok",
            "Neighborhood selector should load selected map data: {map_result}"
        );
    }
}
