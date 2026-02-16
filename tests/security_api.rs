//! Tests for security API functions (security_api.rs) and Elune taint tracking.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// SecureCmdOptionParse
// ============================================================================

#[test]
fn test_securecmdoptionparse_returns_last() {
    let env = env();
    let result: String = env
        .eval("return SecureCmdOptionParse('[mod:shift] action1; action2')")
        .unwrap();
    assert_eq!(result, "action2");
}

#[test]
fn test_securecmdoptionparse_single_option() {
    let env = env();
    let result: String = env
        .eval("return SecureCmdOptionParse('just_this')")
        .unwrap();
    assert_eq!(result, "just_this");
}

// ============================================================================
// hooksecurefunc
// ============================================================================

#[test]
fn test_hooksecurefunc_global() {
    let env = env();
    let (orig_ran, hook_ran): (bool, bool) = env
        .eval(
            r#"
            HOOK_TEST_ORIG = false
            HOOK_TEST_HOOK = false
            function MyTestFunc() HOOK_TEST_ORIG = true end
            hooksecurefunc("MyTestFunc", function() HOOK_TEST_HOOK = true end)
            MyTestFunc()
            return HOOK_TEST_ORIG, HOOK_TEST_HOOK
            "#,
        )
        .unwrap();
    assert!(orig_ran);
    assert!(hook_ran);
}

#[test]
fn test_hooksecurefunc_table() {
    let env = env();
    let (orig_ran, hook_ran): (bool, bool) = env
        .eval(
            r#"
            local t = {}
            HOOK_TABLE_ORIG = false
            HOOK_TABLE_HOOK = false
            function t.Foo() HOOK_TABLE_ORIG = true end
            hooksecurefunc(t, "Foo", function() HOOK_TABLE_HOOK = true end)
            t.Foo()
            return HOOK_TABLE_ORIG, HOOK_TABLE_HOOK
            "#,
        )
        .unwrap();
    assert!(orig_ran);
    assert!(hook_ran);
}

// ============================================================================
// securecall / securecallfunction
// ============================================================================

#[test]
fn test_securecall() {
    let env = env();
    let result: i32 = env
        .eval("return securecall(function(a) return a * 2 end, 5)")
        .unwrap();
    assert_eq!(result, 10);
}

#[test]
fn test_securecallfunction() {
    let env = env();
    let result: i32 = env
        .eval("return securecallfunction(function(a, b) return a + b end, 3, 4)")
        .unwrap();
    assert_eq!(result, 7);
}

// ============================================================================
// secureexecuterange
// ============================================================================

#[test]
fn test_secureexecuterange() {
    let env = env();
    let total: i32 = env
        .eval(
            r#"
            SECURE_TOTAL = 0
            local t = {10, 20, 30}
            secureexecuterange(t, function(key, value) SECURE_TOTAL = SECURE_TOTAL + value end)
            return SECURE_TOTAL
            "#,
        )
        .unwrap();
    assert_eq!(total, 60);
}

// ============================================================================
// issecure / issecurevariable
// ============================================================================

#[test]
fn test_issecure_returns_true() {
    let env = env();
    let val: bool = env.eval("return issecure()").unwrap();
    assert!(val);
}

#[test]
fn test_issecurevariable_returns_true() {
    let env = env();
    // Elune's issecurevariable takes ("variable") or (table, "variable"), not nil
    let val: bool = env
        .eval("local s, t = issecurevariable('print'); return s")
        .unwrap();
    assert!(val);
}

// ============================================================================
// SecureHandler stubs
// ============================================================================

#[test]
fn test_secure_handler_stubs_exist() {
    let env = env();
    for func in &[
        "SecureHandlerSetFrameRef",
        "SecureHandlerExecute",
        "SecureHandlerWrapScript",
        "RegisterStateDriver",
        "UnregisterStateDriver",
        "RegisterAttributeDriver",
        "UnregisterAttributeDriver",
    ] {
        let is_func: bool = env
            .eval(&format!("return type({}) == 'function'", func))
            .unwrap();
        assert!(is_func, "{} should be a function", func);
    }
}

// ============================================================================
// forceinsecure + taint tracking
// ============================================================================

#[test]
fn test_forceinsecure() {
    let env = env();
    let secure_after: bool = env
        .eval("forceinsecure(); return issecure()")
        .unwrap();
    assert!(!secure_after, "forceinsecure() should taint execution");
}

#[test]
fn test_securecall_restores_taint() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            local function inner()
                forceinsecure()
            end
            securecall(inner)
            return issecure()
            "#,
        )
        .unwrap();
    assert!(result, "securecall should restore taint state after call");
}

#[test]
fn test_loadstring_taints() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            local f = loadstring("return issecure()")
            return f()
            "#,
        )
        .unwrap();
    assert!(!result, "code from loadstring should be tainted");
}

#[test]
fn test_issecurevariable_detects_taint() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            loadstring("TAINTED_VAR = 1")()
            local secure = issecurevariable("TAINTED_VAR")
            return secure
            "#,
        )
        .unwrap();
    assert!(!result, "variable set by tainted code should be insecure");
}
