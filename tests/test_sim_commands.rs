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
    let count: i32 = env
        .eval(
            r#"
            SimCommands:Register("Test Command", "A test", function() end, "Debug")
            SimCommands:Register("Another", "Second test", function() end)
            return #SimCommands:GetCommands()
            "#,
        )
        .unwrap();
    assert_eq!(count, 2, "Should have 2 registered commands");
}

#[test]
fn sim_commands_entry_fields() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            SimCommands:Register("Open Mail", "Open the mailbox", function() end, "UI Panels")
            local cmd = SimCommands:GetCommands()[1]
            return cmd.name .. "|" .. cmd.description .. "|" .. cmd.category
            "#,
        )
        .unwrap();
    assert_eq!(result, "Open Mail|Open the mailbox|UI Panels");
}

#[test]
fn sim_commands_default_category() {
    let env = env();
    let cat: String = env
        .eval(
            r#"
            SimCommands:Register("No Category", "desc", function() end)
            return SimCommands:GetCommands()[1].category
            "#,
        )
        .unwrap();
    assert_eq!(cat, "General", "Default category should be 'General'");
}

#[test]
fn sim_commands_filter_by_name() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            SimCommands:Register("Open Mailbox", "Show mail UI", function() end)
            SimCommands:Register("Set Level", "Change player level", function() end)
            SimCommands:Register("Open Bank", "Show bank UI", function() end)
            return #SimCommands:Filter("open")
            "#,
        )
        .unwrap();
    assert_eq!(count, 2, "Filter 'open' should match 2 commands");
}

#[test]
fn sim_commands_filter_by_description() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            SimCommands:Register("Do Thing", "mail related", function() end)
            SimCommands:Register("Other", "unrelated", function() end)
            return #SimCommands:Filter("mail")
            "#,
        )
        .unwrap();
    assert_eq!(count, 1, "Filter 'mail' should match description");
}

#[test]
fn sim_commands_filter_empty_returns_all() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            SimCommands:Register("A", "", function() end)
            SimCommands:Register("B", "", function() end)
            return #SimCommands:Filter("")
            "#,
        )
        .unwrap();
    assert_eq!(count, 2, "Empty filter should return all commands");
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
