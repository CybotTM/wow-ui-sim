//! Tests for Slider widget methods.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

// ============================================================================
// GetThumbTexture / SetThumbTexture
// ============================================================================

#[test]
fn test_get_thumb_texture_defaults_to_thumb_child() {
    let env = env();
    let matches: bool = env
        .eval(
            r#"
        local s = CreateFrame("Slider")
        return s:GetThumbTexture() == s.ThumbTexture
    "#,
        )
        .unwrap();
    assert!(
        matches,
        "GetThumbTexture should return the default thumb child"
    );
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
    assert!(
        matches,
        "GetThumbTexture should return the texture set via SetThumbTexture"
    );
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
    assert!(
        still_same,
        "SetThumbTexture with fileID should keep same texture object"
    );
}

#[test]
fn test_slider_set_thumb_texture_file_id_get_texture() {
    let env = env();
    let tex_id: i32 = env
        .eval(
            r#"
        local s = CreateFrame("Slider")
        s:SetThumbTexture(12345)
        local t = s:GetThumbTexture()
        return t:GetTexture()
    "#,
        )
        .unwrap();
    assert_eq!(tex_id, 12345);
}

#[test]
fn test_obey_step_on_drag_round_trip() {
    let env = env();
    let obeys: (bool, bool, bool) = env
        .eval(
            r#"
        local s = CreateFrame("Slider")
        local initial = s:GetObeyStepOnDrag()
        s:SetObeyStepOnDrag(true)
        local enabled = s:GetObeyStepOnDrag()
        s:SetObeyStepOnDrag(false)
        local disabled = s:GetObeyStepOnDrag()
        return initial, enabled, disabled
    "#,
        )
        .unwrap();
    assert_eq!(
        obeys,
        (false, true, false),
        "GetObeyStepOnDrag should round-trip the persisted slider flag"
    );
}
