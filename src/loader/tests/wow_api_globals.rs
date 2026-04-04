//! Tests for global WoW API functions and pre-created global frames.

use super::*;

// ---------------------------------------------------------------------------
// Global functions
// ---------------------------------------------------------------------------

#[test]
fn test_get_build_info() {
    let env = WowLuaEnv::new().unwrap();
    let (version, toc): (String, i32) = env
        .eval("local v,_,_,t = GetBuildInfo(); return v, t")
        .unwrap();
    assert!(!version.is_empty());
    assert!(toc > 0);
}

#[test]
fn test_get_locale() {
    let env = WowLuaEnv::new().unwrap();
    let locale: String = env.eval("return GetLocale()").unwrap();
    assert!(!locale.is_empty());
}

#[test]
fn test_unit_name_player() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env.eval("return UnitName('player')").unwrap();
    assert!(!name.is_empty());
}

#[test]
fn test_get_money() {
    let env = WowLuaEnv::new().unwrap();
    let money: i64 = env.eval("return GetMoney()").unwrap();
    assert!(money >= 0);
}

#[test]
fn test_in_combat_lockdown_false() {
    let env = WowLuaEnv::new().unwrap();
    let in_combat: bool = env.eval("return InCombatLockdown()").unwrap();
    assert!(!in_combat);
}

#[test]
fn test_wipe_function() {
    let (t, _) = load_test_lua(
        "test-wipe",
        r#"
        local t = {1, 2, 3, a = "b"}
        wipe(t)
        WIPE_LEN = #t
        WIPE_A_NIL = (t.a == nil)
    "#,
    );
    let len: i32 = t.env.eval("return WIPE_LEN").unwrap();
    assert_eq!(len, 0);
    t.assert_lua_true("return WIPE_A_NIL", "wipe should clear named keys");
}

#[test]
fn test_copy_table_deep() {
    let (t, _) = load_test_lua(
        "test-copytable",
        r#"
        local orig = {a = 1, b = {c = 2}}
        local copy = CopyTable(orig)
        COPY_A = copy.a
        COPY_BC = copy.b.c
        copy.a = 99
        ORIG_A = orig.a
    "#,
    );
    let copy_a: i32 = t.env.eval("return COPY_A").unwrap();
    assert_eq!(copy_a, 1);
    let copy_bc: i32 = t.env.eval("return COPY_BC").unwrap();
    assert_eq!(copy_bc, 2);
    let orig_a: i32 = t.env.eval("return ORIG_A").unwrap();
    assert_eq!(orig_a, 1, "original should be unmodified");
}

#[test]
fn test_strsplit() {
    let (t, _) = load_test_lua(
        "test-strsplit",
        r#"
        local a, b, c = strsplit(",", "one,two,three")
        SS_A, SS_B, SS_C = a, b, c
    "#,
    );
    t.assert_lua_str("return SS_A", "one");
    t.assert_lua_str("return SS_B", "two");
    t.assert_lua_str("return SS_C", "three");
}

#[test]
fn test_strtrim() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env.eval(r#"return strtrim("  hello  ")"#).unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_geterrorhandler() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return type(geterrorhandler())").unwrap();
    assert_eq!(ty, "function");
}

#[test]
fn test_hooksecurefunc() {
    let (t, _) = load_test_lua(
        "test-hooksecure",
        r#"
        local obj = { MyMethod = function() end }
        HOOK_CALLED = false
        hooksecurefunc(obj, "MyMethod", function() HOOK_CALLED = true end)
        obj:MyMethod()
    "#,
    );
    t.assert_lua_true("return HOOK_CALLED", "hook should fire");
}

#[test]
fn test_hooksecurefunc_on_frame_userdata() {
    let (t, _) = load_test_lua(
        "test-hooksecure-ud",
        r#"
        local f = CreateFrame("Frame", "HookSecureUDTest", UIParent)
        HOOK_CALLED = false
        hooksecurefunc(f, "SetAlpha", function() HOOK_CALLED = true end)
        f:SetAlpha(0.5)
    "#,
    );
    t.assert_lua_true("return HOOK_CALLED", "hook should fire on userdata frame");
}

#[test]
fn test_issecurevariable_on_frame_userdata() {
    let (t, _) = load_test_lua(
        "test-issecurevar-ud",
        r#"
        local f = CreateFrame("Frame", "IssecureVarUDTest", UIParent)
        -- issecurevariable(frame, "method") should not error on userdata
        local secure, taint = issecurevariable(f, "Show")
        SECURE_RESULT = secure
    "#,
    );
    t.assert_lua_true("return SECURE_RESULT", "native method should be secure");
}

#[test]
fn test_mixin() {
    let (t, _) = load_test_lua(
        "test-mixin",
        r#"
        local target = {}
        Mixin(target, {foo = 1, bar = "hello"})
        MIX_FOO = target.foo
        MIX_BAR = target.bar
    "#,
    );
    let foo: i32 = t.env.eval("return MIX_FOO").unwrap();
    assert_eq!(foo, 1);
    t.assert_lua_str("return MIX_BAR", "hello");
}

#[test]
fn test_global_functions_callable() {
    let env = WowLuaEnv::new().unwrap();
    for f in &[
        "BreakUpLargeNumbers",
        "PlaySound",
        "ReloadUI",
        "GetBindingKey",
        "SetOverrideBinding",
        "ClearOverrideBindings",
        "GetInventoryItemLink",
        "GetInventoryItemTexture",
        "GetInventorySlotInfo",
        "GetFramerate",
        "format",
        "strjoin",
    ] {
        let expr = format!("return type({})", f);
        let ty: String = env.eval(&expr).unwrap();
        assert_eq!(ty, "function", "{} should be function", f);
    }
}

// ---------------------------------------------------------------------------
// Global frames and tables
// ---------------------------------------------------------------------------

#[test]
fn test_uiparent_exists() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return UIParent:GetObjectType()").unwrap();
    assert_eq!(ty, "Frame");
}

#[test]
fn test_ui_special_frames_table() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return type(UISpecialFrames)").unwrap();
    assert_eq!(ty, "table");
}

// SOUNDKIT: from Blizzard_SharedXML/SoundKitConstants.lua
// Tested via Lua addon tests (run-tests).

#[test]
fn test_game_tooltip_methods() {
    let env = WowLuaEnv::new().unwrap();
    for m in &["SetOwner", "Show", "Hide"] {
        let expr = format!("return type(GameTooltip.{})", m);
        let ty: String = env.eval(&expr).unwrap();
        assert_eq!(ty, "function", "GameTooltip.{} should be function", m);
    }
}

#[test]
fn test_static_popup() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return type(StaticPopup_Show)").unwrap();
    assert_eq!(ty, "function");
    let ty2: String = env.eval("return type(StaticPopupDialogs)").unwrap();
    assert_eq!(ty2, "table");
}

// ContinuableContainer, ItemButtonUtil, ScrollUtil, CreateScrollBoxLinearView,
// MainMenuBarBackpackButton: all from Blizzard addon Lua/XML.
// Tested via Lua addon tests (run-tests).
