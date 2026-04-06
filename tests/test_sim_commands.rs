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
fn sim_commands_addon_loads() {
    let env = env();
    let exists: bool = env
        .eval("return type(SimCommands) == 'table'")
        .unwrap();
    assert!(exists, "SimCommands global should exist after addon load");
}

#[test]
fn sim_commands_register_and_list() {
    let env = env();
    let added: i32 = env
        .eval(
            r#"
            local before = #SimCommands:GetCommands()
            SimCommands:Register("Test Command", "A test", function() end, "Debug")
            SimCommands:Register("Another", "Second test", function() end)
            return #SimCommands:GetCommands() - before
            "#,
        )
        .unwrap();
    assert_eq!(added, 2, "Should have added 2 commands");
}

#[test]
fn sim_commands_entry_fields() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            SimCommands:Register("Test Entry", "A test entry", function() end, "Debug")
            local cmds = SimCommands:GetCommands()
            local cmd = cmds[#cmds]  -- last registered
            return cmd.name .. "|" .. cmd.description .. "|" .. cmd.category
            "#,
        )
        .unwrap();
    assert_eq!(result, "Test Entry|A test entry|Debug");
}

#[test]
fn sim_commands_default_category() {
    let env = env();
    let cat: String = env
        .eval(
            r#"
            SimCommands:Register("No Category", "desc", function() end)
            local cmds = SimCommands:GetCommands()
            return cmds[#cmds].category
            "#,
        )
        .unwrap();
    assert_eq!(cat, "General", "Default category should be 'General'");
}

#[test]
fn sim_commands_filter_by_name() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            SimCommands:Register("Zzz Open Alpha", "Show alpha UI", function() end)
            SimCommands:Register("Set Level", "Change player level", function() end)
            SimCommands:Register("Zzz Open Beta", "Show beta UI", function() end)
            -- Filter for "zzz open" to match only our test entries
            local matches = SimCommands:Filter("zzz open")
            return tostring(#matches)
            "#,
        )
        .unwrap();
    assert_eq!(result, "2", "Filter 'zzz open' should match 2 test commands");
}

#[test]
fn sim_commands_filter_by_description() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            SimCommands:Register("Do Thing", "xyzzy unique desc", function() end)
            SimCommands:Register("Other", "unrelated", function() end)
            return tostring(#SimCommands:Filter("xyzzy"))
            "#,
        )
        .unwrap();
    assert_eq!(result, "1", "Filter 'xyzzy' should match description");
}

#[test]
fn sim_commands_filter_empty_returns_all() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            local total = #SimCommands:GetCommands()
            local filtered = #SimCommands:Filter("")
            return total == filtered and total > 0
            "#,
        )
        .unwrap();
    assert!(result, "Empty filter should return all commands");
}

#[test]
fn sim_commands_toggle_shows_frame() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            SimCommands:Toggle()
            local shown = SimCommands:IsShown()
            SimCommands:Toggle()
            local hidden = not SimCommands:IsShown()
            if shown and hidden then return "ok" end
            return "shown=" .. tostring(shown) .. " hidden=" .. tostring(hidden)
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Toggle should show then hide: {result}");
}

#[test]
fn sim_commands_palette_frame_exists() {
    let env = env();
    let exists: bool = env
        .eval(
            r#"
            SimCommands:Show()
            return SimCommandsFrame ~= nil
            "#,
        )
        .unwrap();
    assert!(exists, "SimCommandsFrame should exist after Show()");
}

#[test]
fn sim_commands_search_box_exists() {
    let env = env();
    let exists: bool = env
        .eval(
            r#"
            SimCommands:Show()
            return SimCommandsSearchBox ~= nil
            "#,
        )
        .unwrap();
    assert!(exists, "SimCommandsSearchBox should exist after Show()");
}

#[test]
fn sim_commands_ctrl_p_toggles() {
    let env = env();
    env.send_key_press("CTRL-P", None)
        .expect("CTRL-P keybind failed");
    let shown: bool = env.eval("return SimCommands:IsShown()").unwrap();
    assert!(shown, "CTRL-P should open the command palette");

    env.send_key_press("CTRL-P", None)
        .expect("CTRL-P keybind failed");
    let hidden: bool = env.eval("return not SimCommands:IsShown()").unwrap();
    assert!(hidden, "CTRL-P again should close the command palette");
}

#[test]
fn sim_commands_minimap_button_exists() {
    let env = env();
    let exists: bool = env
        .eval("return SimCommandsMinimapButton ~= nil")
        .unwrap();
    assert!(exists, "Minimap button should be created at load time");
}

#[test]
fn sim_commands_minimap_button_toggles_palette() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local btn = SimCommandsMinimapButton
            if not btn then return "no_button" end
            local click = btn:GetScript("OnClick")
            if not click then return "no_onclick" end
            click(btn)
            if not SimCommands:IsShown() then return "not_shown" end
            click(btn)
            if SimCommands:IsShown() then return "not_hidden" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Minimap button click should toggle palette: {result}");
}

#[test]
fn builtin_open_mailbox_registered() {
    let env = env();
    let found: bool = env
        .eval(
            r#"
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Open Mailbox" then return true end
            end
            return false
            "#,
        )
        .unwrap();
    assert!(found, "Open Mailbox command should be registered");
}

#[test]
fn builtin_open_mailbox_fires_event() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("MAIL_SHOW")
            f:SetScript("OnEvent", function(self, event)
                if event == "MAIL_SHOW" then fired = true end
            end)
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Open Mailbox" then
                    cmd.action()
                    break
                end
            end
            return fired and "ok" or "not_fired"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Open Mailbox should fire MAIL_SHOW: {result}");
}

#[test]
fn builtin_open_bank_fires_event() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("BANKFRAME_OPENED")
            f:SetScript("OnEvent", function(self, event)
                if event == "BANKFRAME_OPENED" then fired = true end
            end)
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Open Bank" then
                    cmd.action()
                    break
                end
            end
            return fired and "ok" or "not_fired"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Open Bank should fire BANKFRAME_OPENED: {result}");
}
