#![cfg(feature = "client-mists")]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn mists_create_forbidden_frame_hides_from_enumerate_frames() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (bool, bool, bool) = env
        .eval(
            r#"
            local forbidden = CreateForbiddenFrame("Button", "MistsHiddenForbiddenProbe", UIParent)
            local visible = CreateFrame("Button", "MistsVisibleEnumerationProbe", UIParent)

            local sawForbidden = false
            local sawVisible = false
            local object = EnumerateFrames()
            while object do
                if object == forbidden then
                    sawForbidden = true
                end
                if object == visible then
                    sawVisible = true
                end
                object = EnumerateFrames(object)
            end

            return sawForbidden, sawVisible, forbidden:IsForbidden()
            "#,
        )
        .expect("CreateForbiddenFrame should create an enumeration-hidden direct handle");

    assert_eq!(
        result,
        (false, true, true),
        "CreateForbiddenFrame should hide forbidden frames from EnumerateFrames"
    );
}
