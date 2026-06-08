#![cfg(any(feature = "client-era", feature = "client-anniversary"))]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn era_create_forbidden_frame_marks_frame_forbidden() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (String, String, bool) = env
        .eval(
            r#"
            local frame = CreateForbiddenFrame("Button", "EraForbiddenProbe", UIParent)
            return frame:GetObjectType(), frame:GetName(), frame:IsForbidden()
            "#,
        )
        .expect("CreateForbiddenFrame should create a real forbidden frame");

    assert_eq!(
        result,
        ("Button".to_string(), "EraForbiddenProbe".to_string(), true),
        "CreateForbiddenFrame should preserve CreateFrame semantics and mark the frame forbidden"
    );
}
