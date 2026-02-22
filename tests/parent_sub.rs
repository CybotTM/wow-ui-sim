//! Tests for $parent name substitution (wowless ParentSub behavior).
//!
//! Rules (from wowless ParentSub):
//! 1. Pattern matches `^$[pP][aA][rR][eE][nN][tT]` — case-insensitive, start-of-string only
//! 2. Walk parent chain to find the first NAMED ancestor (skip unnamed/anonymous frames)
//! 3. Fallback to "Top" when no named ancestor exists
//! 4. Single replacement only (anchored to start of string)

use wow_ui_sim::lua_api::WowLuaEnv;

/// Basic case: $parent at start with named parent.
#[test]
fn test_parent_sub_basic() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "Moo", UIParent)
            local child = CreateFrame("Frame", "$parentWowlessCow", parent)
            return child:GetName()
            "#,
        )
        .unwrap();
    assert_eq!(name, "MooWowlessCow");
}

/// No named parent at all (parent_id=None): "Top" fallback is exercised by the
/// internal `apply_parent_sub` unit path. In Lua integration, nil parent → UIParent
/// (which IS named), so this test verifies the no-op path (name without $parent prefix).
#[test]
fn test_parent_sub_no_parent_top_fallback() {
    let env = WowLuaEnv::new().unwrap();
    // A frame name without $parent prefix is returned unchanged regardless of parent.
    let name: String = env
        .eval(
            r#"
            local child = CreateFrame("Frame", "NoSubstitution", UIParent)
            return child:GetName()
            "#,
        )
        .unwrap();
    assert_eq!(name, "NoSubstitution");
}

/// $parent not at start of string — no substitution (mid-string $parent unchanged).
#[test]
fn test_parent_sub_mid_string_no_change() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "Moo", UIParent)
            local child = CreateFrame("Frame", "Wowless$parentCow", parent)
            return child:GetName()
            "#,
        )
        .unwrap();
    // $parent not at start → no substitution
    assert_eq!(name, "Wowless$parentCow");
}

/// Case-insensitive matching: $pArEnT should work.
#[test]
fn test_parent_sub_case_insensitive() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "Moo", UIParent)
            local child = CreateFrame("Frame", "$pArEnTMixed", parent)
            return child:GetName()
            "#,
        )
        .unwrap();
    assert_eq!(name, "MooMixed");
}

/// Single replacement only: only the first ^$parent (at start) is replaced.
#[test]
fn test_parent_sub_single_replacement_only() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "Moo", UIParent)
            local child = CreateFrame("Frame", "$parent$parentWowless$parentCow", parent)
            return child:GetName()
            "#,
        )
        .unwrap();
    // Only the first ^$parent at start is replaced; the rest remain literally
    assert_eq!(name, "Moo$parentWowless$parentCow");
}

/// Skip anonymous ancestors: walk up the parent chain to find the first named ancestor.
#[test]
fn test_parent_sub_skip_anon_find_named() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env
        .eval(
            r#"
            local named = CreateFrame("Frame", "Moo", UIParent)
            local anon = CreateFrame("Frame", nil, named)
            local child = CreateFrame("Frame", "$parentIgnoreAnonSub", anon)
            return child:GetName()
            "#,
        )
        .unwrap();
    // anon parent has no name → walk up to "Moo"
    assert_eq!(name, "MooIgnoreAnonSub");
}

/// Anonymous chain rooted at UIParent: all intermediate frames are anon, but UIParent
/// is named, so it is found and used (not "Top").
#[test]
fn test_parent_sub_anon_chain_fallback_top() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env
        .eval(
            r#"
            local anon1 = CreateFrame("Frame", nil, UIParent)
            local anon2 = CreateFrame("Frame", nil, anon1)
            local child = CreateFrame("Frame", "$parentIgnoreAnonTop", anon2)
            return child:GetName()
            "#,
        )
        .unwrap();
    // anon2 and anon1 have no name; UIParent is skipped in $parent walk → falls back to "Top"
    assert_eq!(name, "TopIgnoreAnonTop");
}

/// CreateTexture with $parent: verify parent substitution works for child regions.
#[test]
fn test_parent_sub_create_texture() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "Moo", UIParent)
            local tex = parent:CreateTexture("$parentTex", "BACKGROUND")
            return tex:GetName()
            "#,
        )
        .unwrap();
    assert_eq!(name, "MooTex");
}

/// CreateTexture $parent with anonymous parent frame → walk to first named ancestor.
#[test]
fn test_parent_sub_create_texture_skip_anon() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env
        .eval(
            r#"
            local named = CreateFrame("Frame", "Moo", UIParent)
            local anon = CreateFrame("Frame", nil, named)
            local tex = anon:CreateTexture("$parentTex", "BACKGROUND")
            return tex:GetName()
            "#,
        )
        .unwrap();
    // anon has no name → walk up to "Moo"
    assert_eq!(name, "MooTex");
}

/// CreateFontString $parent substitution.
#[test]
fn test_parent_sub_create_fontstring() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "Moo", UIParent)
            local fs = parent:CreateFontString("$parentFS", "OVERLAY")
            return fs:GetName()
            "#,
        )
        .unwrap();
    assert_eq!(name, "MooFS");
}

/// $Parent (capital P) is also matched case-insensitively.
#[test]
fn test_parent_sub_dollar_parent_upper_p() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "Moo", UIParent)
            local child = CreateFrame("Frame", "$ParentCow", parent)
            return child:GetName()
            "#,
        )
        .unwrap();
    assert_eq!(name, "MooCow");
}

/// nil parent passed explicitly: $parent substitution should use "Top" fallback,
/// not "UIParent" (which is only the parenting default, not the $parent sub source).
#[test]
fn test_parent_sub_top_fallback_nil_parent() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            local f = CreateFrame("Frame", "$parentWowlessCow", nil)
            return f:GetName()
            "#,
        )
        .unwrap();
    assert_eq!(result, "TopWowlessCow");
}

/// No parent arg at all: $parent substitution should use "Top" fallback,
/// not "UIParent" (which is only the parenting default, not the $parent sub source).
#[test]
fn test_parent_sub_top_fallback_no_parent_arg() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            local f = CreateFrame("Frame", "$parentWowlessCow")
            return f:GetName()
            "#,
        )
        .unwrap();
    assert_eq!(result, "TopWowlessCow");
}
