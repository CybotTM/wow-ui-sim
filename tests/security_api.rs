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

#[test]
fn test_secure_handler_stubs_are_inert() {
    let env = env();
    let (ran_original, kept_attribute, did_not_store_ref): (bool, bool, bool) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            local ref = CreateFrame("Frame")
            local originalRan = false

            frame:SetAttribute("testAttr", "before")
            frame:SetScript("OnShow", function() originalRan = true end)
            frame:Hide()

            SecureHandlerSetFrameRef(frame, "target", ref)
            SecureHandlerExecute(frame, "self:SetAttribute('testAttr', 'after')")
            SecureHandlerWrapScript(frame, "OnShow", "self:SetAttribute('wrapped', true)")

            frame:Show()

            return originalRan, frame:GetAttribute("testAttr") == "before", frame:GetAttribute("_frame-target") == nil
            "#,
        )
        .unwrap();
    assert!(ran_original, "original script should still run");
    assert!(
        kept_attribute,
        "secure handler stubs should not mutate attributes"
    );
    assert!(
        did_not_store_ref,
        "SecureHandlerSetFrameRef should stay inert until restricted handlers exist"
    );
}

// ============================================================================
// forceinsecure + taint tracking
// ============================================================================

#[test]
fn test_forceinsecure() {
    let env = env();
    let secure_after: bool = env.eval("forceinsecure(); return issecure()").unwrap();
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

// ============================================================================
// issecretvalue / canaccessvalue / canaccessallvalues / canaccesstable
// ============================================================================

#[test]
fn test_issecretvalue_untainted() {
    let env = env();
    let result: bool = env.eval("return issecretvalue(print)").unwrap();
    assert!(!result, "engine function should not be secret");
}

#[test]
fn test_issecretvalue_tainted() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            local f = loadstring("return 1")
            return issecretvalue(f)
            "#,
        )
        .unwrap();
    assert!(result, "loadstring result should be secret");
}

#[test]
fn test_canaccessvalue_untainted() {
    let env = env();
    let result: bool = env.eval("return canaccessvalue(print)").unwrap();
    assert!(result, "engine function should be accessible");
}

#[test]
fn test_canaccessvalue_tainted() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            local f = loadstring("return 1")
            return canaccessvalue(f)
            "#,
        )
        .unwrap();
    assert!(!result, "loadstring result should not be accessible");
}

#[test]
fn test_canaccessallvalues_all_clean() {
    let env = env();
    let result: bool = env
        .eval("return canaccessallvalues(print, type, tostring)")
        .unwrap();
    assert!(result, "all engine values should be accessible");
}

#[test]
fn test_canaccessallvalues_one_tainted() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            local f = loadstring("return 1")
            return canaccessallvalues(print, f, type)
            "#,
        )
        .unwrap();
    assert!(!result, "mixed values should fail access check");
}

#[test]
fn test_canaccesstable_clean() {
    let env = env();
    let result: bool = env.eval("return canaccesstable({1, 2, 3})").unwrap();
    assert!(result, "engine-created table should be accessible");
}

#[test]
fn test_state_driver_stubs_are_inert() {
    let env = env();
    let (still_shown, no_state_attr): (bool, bool) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            frame:Show()

            RegisterStateDriver(frame, "visibility", "hide")
            RegisterAttributeDriver(frame, "state-custom", "active")
            UnregisterStateDriver(frame, "visibility")
            UnregisterAttributeDriver(frame, "state-custom")

            return frame:IsShown(), frame:GetAttribute("state-custom") == nil
            "#,
        )
        .unwrap();
    assert!(
        still_shown,
        "state driver stubs should not change visibility until protected drivers exist"
    );
    assert!(
        no_state_attr,
        "attribute driver stubs should not write attributes until protected drivers exist"
    );
}

#[test]
fn test_securecallmethod_returns_values() {
    let env = env();
    let result: i32 = env
        .eval(
            r#"
            local obj = { Add = function(self, a, b) return a + b end }
            return securecallmethod(obj, "Add", 3, 7)
            "#,
        )
        .unwrap();
    assert_eq!(result, 10);
}

#[test]
fn test_securecallmethod_swallows_errors() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            local obj = { Bad = function() error("boom") end }
            securecallmethod(obj, "Bad")
            return true
            "#,
        )
        .unwrap();
    assert!(result, "securecallmethod should swallow errors");
}

#[test]
fn test_securecallmethod_missing_method() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            local obj = {}
            local r = securecallmethod(obj, "Nope")
            return r == nil
            "#,
        )
        .unwrap();
    assert!(result, "missing method should return nil");
}

// ============================================================================
// CreateSecureDelegate
// ============================================================================

#[test]
fn test_create_secure_delegate_is_identity() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            local function myFunc() return 42 end
            local delegate = CreateSecureDelegate(myFunc)
            return delegate == myFunc
            "#,
        )
        .unwrap();
    assert!(
        result,
        "CreateSecureDelegate should return the function as-is"
    );
}

#[test]
fn test_create_secure_delegate_survives_nil() {
    let env = env();
    // Simulate what EnvironmentCleanup does, then restore
    let result: bool = env
        .eval(
            r#"
            local function myFunc() return 42 end
            -- EnvironmentCleanup nils it
            CreateSecureDelegate = nil
            assert(CreateSecureDelegate == nil, "should be nil after cleanup")
            return true
            "#,
        )
        .unwrap();
    assert!(result);

    // Restore it (as the loader does after EnvironmentCleanup)
    env.restore_post_cleanup_globals();

    let result: bool = env
        .eval(
            r#"
            local function myFunc() return 42 end
            local delegate = CreateSecureDelegate(myFunc)
            return delegate == myFunc
            "#,
        )
        .unwrap();
    assert!(
        result,
        "CreateSecureDelegate should work after restore_post_cleanup_globals"
    );
}
