use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    let toc = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("Interface/AddOns/SimCommands/SimCommands.toc");
    load_addon(&env.loader_env(), &toc).expect("Failed to load SimCommands");
    env
}

#[test]
fn builtin_earn_achievement_registered() {
    let env = env();
    let found: bool = env
        .eval(
            r#"
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Earn Achievement" then return true end
            end
            return false
            "#,
        )
        .unwrap();
    assert!(found, "Earn Achievement command should be registered");
}

#[test]
fn builtin_earn_random_achievement_earns_and_notifies_toast_listener() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local randomAchievement
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Earn Random Achievement" then
                    randomAchievement = cmd
                    break
                end
            end
            if not randomAchievement then return "missing_command" end

            local originalRandom = math.random
            math.random = function(max)
                if max == nil then return 0.5 end
                return 3
            end

            local toast = CreateFrame("Frame", "RandomAchievementToastTest", UIParent)
            toast:RegisterEvent("ACHIEVEMENT_EARNED")
            toast:SetScript("OnEvent", function(self, event, achievementID)
                local _, name = GetAchievementInfo(achievementID)
                self.event = event
                self.achievementID = achievementID
                self.achievementName = name
            end)

            randomAchievement.action()
            math.random = originalRandom

            local _, name, _, completed = GetAchievementInfo(8)
            return table.concat({
                tostring(completed == true),
                tostring(toast.event),
                tostring(toast.achievementID),
                tostring(toast.achievementName),
                tostring(name),
            }, "|")
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "true|ACHIEVEMENT_EARNED|8|Level 30|Level 30",
        "random achievement command should earn a seeded achievement and notify the toast event listener"
    );
}

#[test]
fn builtin_toggle_debug_borders() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local first = A_Admin.ToggleDebugBorders()
            local second = A_Admin.ToggleDebugBorders()
            if first == true and second == false then return "ok" end
            return "first=" .. tostring(first) .. " second=" .. tostring(second)
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "ToggleDebugBorders should toggle: {result}");
}

#[test]
fn builtin_toggle_debug_commands_registered() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local borders, anchors = false, false
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Toggle Debug Borders" then borders = true end
                if cmd.name == "Toggle Debug Anchors" then anchors = true end
            end
            if borders and anchors then return "ok" end
            return "borders=" .. tostring(borders) .. " anchors=" .. tostring(anchors)
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "Debug toggle commands should be registered: {result}"
    );
}

#[test]
fn builtin_reload_ui_registered() {
    let env = env();
    let found: bool = env
        .eval(
            r#"
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Reload UI" then return true end
            end
            return false
            "#,
        )
        .unwrap();
    assert!(found, "Reload UI command should be registered");
}
