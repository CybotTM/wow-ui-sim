//! Tests for HookScript return value and GetScript integration.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn test_hook_script_returns_true() {
    let env = WowLuaEnv::new().unwrap();
    let result: bool = env
        .eval(
            r#"
        local f = CreateFrame("Frame")
        f:SetScript("OnShow", function() end)
        return f:HookScript("OnShow", function() end)
    "#,
        )
        .unwrap();
    assert!(result, "HookScript should return true");
}

#[test]
fn test_get_script_returns_different_function_after_hook() {
    let env = WowLuaEnv::new().unwrap();
    let changed: bool = env
        .eval(
            r#"
        local f = CreateFrame("Frame")
        f:SetScript("OnShow", function() end)
        local original = f:GetScript("OnShow")
        f:HookScript("OnShow", function() end)
        return original ~= f:GetScript("OnShow")
    "#,
        )
        .unwrap();
    assert!(changed, "GetScript should return a different function after HookScript");
}

#[test]
fn test_hook_script_chains_original_then_hook() {
    let env = WowLuaEnv::new().unwrap();
    let order: Vec<String> = env
        .eval(
            r#"
        local f = CreateFrame("Frame")
        local order = {}
        f:SetScript("OnShow", function() table.insert(order, "original") end)
        f:HookScript("OnShow", function() table.insert(order, "hook") end)
        local handler = f:GetScript("OnShow")
        handler(f)
        return order
    "#,
        )
        .unwrap();
    assert_eq!(order, vec!["original", "hook"]);
}

#[test]
fn test_hook_script_with_no_existing_handler() {
    let env = WowLuaEnv::new().unwrap();
    let called: bool = env
        .eval(
            r#"
        local f = CreateFrame("Frame")
        local called = false
        f:HookScript("OnShow", function() called = true end)
        local handler = f:GetScript("OnShow")
        handler(f)
        return called
    "#,
        )
        .unwrap();
    assert!(called, "Hook should be callable via GetScript when no prior handler exists");
}

#[test]
fn test_multiple_hook_scripts_chain_in_order() {
    let env = WowLuaEnv::new().unwrap();
    let order: Vec<String> = env
        .eval(
            r#"
        local f = CreateFrame("Frame")
        local order = {}
        f:SetScript("OnShow", function() table.insert(order, "original") end)
        f:HookScript("OnShow", function() table.insert(order, "hook1") end)
        f:HookScript("OnShow", function() table.insert(order, "hook2") end)
        local handler = f:GetScript("OnShow")
        handler(f)
        return order
    "#,
        )
        .unwrap();
    assert_eq!(order, vec!["original", "hook1", "hook2"]);
}
