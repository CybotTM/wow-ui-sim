//! Tests for secure state drivers and secure frame helper globals.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn test_state_drivers_apply_visibility_and_attributes() {
    let env = env();
    let (shown, state_hidden, custom_state, numeric_state, nil_state): (
        bool,
        bool,
        String,
        i32,
        bool,
    ) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            frame:Show()

            RegisterStateDriver(frame, "visibility", "hide")
            RegisterAttributeDriver(frame, "state-custom", "active")
            RegisterAttributeDriver(frame, "state-count", "17")
            RegisterAttributeDriver(frame, "state-empty", "nil")

            return frame:IsShown(),
                frame:GetAttribute("statehidden"),
                frame:GetAttribute("state-custom"),
                frame:GetAttribute("state-count"),
                frame:GetAttribute("state-empty") == nil
            "#,
        )
        .unwrap();
    assert!(
        !shown,
        "visibility drivers should immediately hide frames for the hide state"
    );
    assert!(
        state_hidden,
        "visibility drivers should mark hidden frames with statehidden"
    );
    assert!(
        custom_state == "active",
        "attribute drivers should resolve string values onto frame attributes"
    );
    assert_eq!(
        numeric_state, 17,
        "attribute drivers should coerce numeric strings to numbers"
    );
    assert!(
        nil_state,
        "attribute drivers should treat the literal nil token as an unset attribute"
    );
}

#[test]
fn test_unregister_state_drivers_leaves_last_resolved_values_in_place() {
    let env = env();
    let (shown, custom_state): (bool, String) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            frame:Show()

            RegisterStateDriver(frame, "visibility", "hide")
            RegisterAttributeDriver(frame, "state-custom", "active")
            UnregisterStateDriver(frame, "visibility")
            UnregisterAttributeDriver(frame, "state-custom")

            return frame:IsShown(), frame:GetAttribute("state-custom")
            "#,
        )
        .unwrap();
    assert!(
        !shown,
        "unregister should stop future updates, not force a visibility reset"
    );
    assert_eq!(
        custom_state, "active",
        "unregister should preserve the last resolved attribute value"
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
