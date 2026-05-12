//! Protected-frame attribute write enforcement.
//!
//! Real WoW rejects `SetAttribute` / `SetAttributeNoHandler` on protected
//! frames when the caller is tainted (insecure). Secure callers can write
//! freely; insecure callers silently drop the write.
//!
//! These tests simulate insecure context by stamping the running chunk with
//! an addon taint via `debug.setstacktaint`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn mark_frame_protected(env: &WowLuaEnv, frame_global: &str) {
    let name = frame_global.to_string();
    let state = env.state().borrow();
    let id = state
        .widgets
        .get_id_by_name(&name)
        .unwrap_or_else(|| panic!("frame {name} should exist"));
    drop(state);
    let mut state = env.state().borrow_mut();
    state
        .widgets
        .get_mut(id)
        .expect("widget exists")
        .is_protected = true;
}

#[test]
fn secure_caller_can_set_attribute_on_protected_frame() {
    let env = env();
    env.exec(
        r#"
        Protected = CreateFrame("Frame", "ProtectedSecureWrite", UIParent)
    "#,
    )
    .unwrap();
    mark_frame_protected(&env, "ProtectedSecureWrite");

    env.exec(
        r#"
        Protected:SetAttribute("foo", "bar")
    "#,
    )
    .unwrap();

    let got: String = env
        .eval(r#"return tostring(Protected:GetAttribute("foo"))"#)
        .unwrap();
    assert_eq!(
        got, "bar",
        "secure caller's write must land on the protected frame"
    );
}

#[test]
fn insecure_caller_cannot_set_attribute_on_protected_frame() {
    let env = env();
    env.exec(
        r#"
        Protected = CreateFrame("Frame", "ProtectedInsecureWrite", UIParent)
        A_Admin.SetInCombat(true)
    "#,
    )
    .unwrap();
    mark_frame_protected(&env, "ProtectedInsecureWrite");

    env.exec(
        r#"
        debug.setstacktaint("evil-addon")
        Protected:SetAttribute("foo", "attempted")
        debug.setstacktaint(nil)
        A_Admin.SetInCombat(false)
    "#,
    )
    .unwrap();

    let got: String = env
        .eval(r#"return tostring(Protected:GetAttribute("foo"))"#)
        .unwrap();
    assert_eq!(
        got, "nil",
        "insecure write on a protected frame must be dropped silently"
    );
}

#[test]
fn insecure_caller_can_set_attribute_on_protected_frame_out_of_combat() {
    let env = env();
    env.exec(
        r#"
        Protected = CreateFrame("Frame", "ProtectedOutOfCombatWrite", UIParent)
        A_Admin.SetInCombat(false)
    "#,
    )
    .unwrap();
    mark_frame_protected(&env, "ProtectedOutOfCombatWrite");

    env.exec(
        r#"
        debug.setstacktaint("evil-addon")
        Protected:SetAttribute("foo", "allowed")
        debug.setstacktaint(nil)
    "#,
    )
    .unwrap();

    let got: String = env
        .eval(r#"return tostring(Protected:GetAttribute("foo"))"#)
        .unwrap();
    assert_eq!(
        got, "allowed",
        "insecure write on a protected frame should be allowed out of combat"
    );
}

#[test]
fn insecure_caller_can_set_attribute_on_unprotected_frame() {
    let env = env();
    env.exec(
        r#"
        Unprotected = CreateFrame("Frame", "UnprotectedInsecureWrite", UIParent)
        debug.setstacktaint("evil-addon")
        Unprotected:SetAttribute("foo", "allowed")
        debug.setstacktaint(nil)
    "#,
    )
    .unwrap();

    let got: String = env
        .eval(r#"return tostring(Unprotected:GetAttribute("foo"))"#)
        .unwrap();
    assert_eq!(
        got, "allowed",
        "insecure write on an unprotected frame must succeed"
    );
}

#[test]
fn insecure_caller_cannot_set_attribute_no_handler_on_protected_frame() {
    let env = env();
    env.exec(
        r#"
        Protected = CreateFrame("Frame", "ProtectedNoHandlerBlock", UIParent)
        A_Admin.SetInCombat(true)
    "#,
    )
    .unwrap();
    mark_frame_protected(&env, "ProtectedNoHandlerBlock");

    env.exec(
        r#"
        debug.setstacktaint("evil-addon")
        Protected:SetAttributeNoHandler("foo", "attempted")
        debug.setstacktaint(nil)
        A_Admin.SetInCombat(false)
    "#,
    )
    .unwrap();

    let got: String = env
        .eval(r#"return tostring(Protected:GetAttribute("foo"))"#)
        .unwrap();
    assert_eq!(
        got, "nil",
        "insecure SetAttributeNoHandler on a protected frame must be dropped silently"
    );
}

#[test]
fn unchanged_scalar_attribute_does_not_refire_on_attribute_changed() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "UnchangedAttributeNoRefire", UIParent)
            local count = 0
            frame:SetScript("OnAttributeChanged", function()
                count = count + 1
            end)

            frame:SetAttribute("showgrid", 1)
            frame:SetAttribute("showgrid", 1)

            return count
        "#,
        )
        .unwrap();

    assert_eq!(
        count, 1,
        "unchanged scalar attributes should not re-fire OnAttributeChanged"
    );
}

#[test]
fn set_attribute_dispatches_direct_on_attribute_changed_method() {
    let env = env();
    let got: String = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "DirectAttributeChangedMethod", UIParent)

            function frame:OnAttributeChanged(name, value)
                self.result = name .. "=" .. tostring(value)
            end

            frame:SetAttribute("open-to-category", 7)

            return frame.result or "missing"
        "#,
        )
        .unwrap();

    assert_eq!(
        got, "open-to-category=7",
        "SetAttribute should dispatch direct OnAttributeChanged methods"
    );
}
