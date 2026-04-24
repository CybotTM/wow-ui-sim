mod common;

use wow_ui_sim::lua_api::WowLuaEnv;

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
