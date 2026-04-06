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
