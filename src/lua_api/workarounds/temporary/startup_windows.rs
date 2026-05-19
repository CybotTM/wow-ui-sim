//! First-frame window cleanup for addon-triggered startup panels.
//!
//! Retirement path: model Blizzard LoadOnDemand panel registration closely
//! enough that third-party startup `LoadAddOn` calls can create panel frames
//! without displaying them. Do not generalize this into hiding arbitrary
//! unparented frames; many addons use visible zero-size frames as dispatchers.

const CLOSE_STARTUP_SPECIAL_WINDOWS_LUA: &str = r#"
if type(CloseAllWindows) == "function" then
    CloseAllWindows(1)
end

if type(CloseProfessionsItemFlyout) == "function" then
    pcall(CloseProfessionsItemFlyout)
end

for _, frameName in ipairs({
    "Baganator_WelcomeFrame",
    "Baganator_SingleViewBackpackViewFrameblizzard",
    "Baganator_CategoryViewBackpackViewFrameblizzard",
}) do
    local frame = _G[frameName]
    if frame and type(frame.Hide) == "function" then
        frame:Hide()
    end
end
"#;

pub(crate) fn close_before_first_frame(env: &crate::lua_api::WowLuaEnv) {
    let _ = env.exec(CLOSE_STARTUP_SPECIAL_WINDOWS_LUA);
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    use super::CLOSE_STARTUP_SPECIAL_WINDOWS_LUA;

    #[test]
    fn startup_window_cleanup_closes_professions_flyout() {
        let env = WowLuaEnv::new().expect("env should initialize");
        env.exec(
            r#"
            __professionsFlyoutClosed = false
            function CloseProfessionsItemFlyout()
                __professionsFlyoutClosed = true
            end
            "#,
        )
        .expect("fake professions flyout close surface should install");

        env.exec(CLOSE_STARTUP_SPECIAL_WINDOWS_LUA)
            .expect("startup window cleanup should run");

        let closed: bool = env
            .eval("return __professionsFlyoutClosed")
            .expect("close probe should run");
        assert!(
            closed,
            "startup cleanup should close professions item flyout"
        );
    }

    #[test]
    fn startup_window_cleanup_hides_baganator_onboarding_frames() {
        let env = WowLuaEnv::new().expect("env should initialize");
        env.exec(
            r#"
            Baganator_WelcomeFrame = CreateFrame("Frame", "Baganator_WelcomeFrame", UIParent)
            Baganator_SingleViewBackpackViewFrameblizzard = CreateFrame("Frame", "Baganator_SingleViewBackpackViewFrameblizzard", UIParent)
            Baganator_CategoryViewBackpackViewFrameblizzard = CreateFrame("Frame", "Baganator_CategoryViewBackpackViewFrameblizzard", UIParent)
            Baganator_WelcomeFrame:Show()
            Baganator_SingleViewBackpackViewFrameblizzard:Show()
            Baganator_CategoryViewBackpackViewFrameblizzard:Show()
            "#,
        )
        .expect("fake Baganator frames should install");

        env.exec(CLOSE_STARTUP_SPECIAL_WINDOWS_LUA)
            .expect("startup window cleanup should run");

        let any_shown: bool = env
            .eval(
                r#"
                return Baganator_WelcomeFrame:IsShown()
                    or Baganator_SingleViewBackpackViewFrameblizzard:IsShown()
                    or Baganator_CategoryViewBackpackViewFrameblizzard:IsShown()
                "#,
            )
            .expect("Baganator visibility probe should run");
        assert!(
            !any_shown,
            "startup cleanup should hide Baganator onboarding frames"
        );
    }
}
