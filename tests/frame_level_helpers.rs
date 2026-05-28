use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn raise_and_lower_frame_level_helpers_change_level_by_one() {
    let env = WowLuaEnv::new().unwrap();
    let level: i32 = env
        .eval(
            r#"
            local f = CreateFrame('Frame')
            f:SetFrameLevel(5)
            RaiseFrameLevel(f)
            LowerFrameLevel(f)
            RaiseFrameLevel(f)
            return f:GetFrameLevel()
            "#,
        )
        .unwrap();
    assert_eq!(level, 6);
}
