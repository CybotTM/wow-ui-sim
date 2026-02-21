//! Tests for Slider widget methods.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

// ============================================================================
// GetThumbTexture / SetThumbTexture
// ============================================================================

#[test]
fn test_get_thumb_texture_nil_by_default() {
    let env = env();
    let is_nil: bool = env
        .eval(
            r#"
        local s = CreateFrame("Slider")
        return s:GetThumbTexture() == nil
    "#,
        )
        .unwrap();
    assert!(is_nil, "GetThumbTexture should return nil on a fresh slider");
}

#[test]
fn test_set_and_get_thumb_texture() {
    let env = env();
    let matches: bool = env
        .eval(
            r#"
        local s = CreateFrame("Slider")
        local t = s:CreateTexture()
        s:SetThumbTexture(t)
        return s:GetThumbTexture() == t
    "#,
        )
        .unwrap();
    assert!(matches, "GetThumbTexture should return the texture set via SetThumbTexture");
}

#[test]
fn test_set_thumb_texture_fileid_keeps_same_object() {
    let env = env();
    let still_same: bool = env
        .eval(
            r#"
        local s = CreateFrame("Slider")
        local t = s:CreateTexture()
        s:SetThumbTexture(t)
        s:SetThumbTexture(12345)
        return s:GetThumbTexture() == t
    "#,
        )
        .unwrap();
    assert!(still_same, "SetThumbTexture with fileID should keep same texture object");
}
