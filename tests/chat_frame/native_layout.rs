use super::*;

#[test]
fn wrath_chat_preparation_preserves_native_layout_and_editbox_state() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("native chat environment");
        env.set_screen_size(1024.0, 768.0);
        env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];
        common::load_profile_framexml(&env);
        // Production prepares native chat before third-party addons and entry events.
        // PLAYER_ENTERING_WORLD auto-applies simulator post-event workarounds.
        let snapshot = r#"
            local point, relative, relativePoint, x, y = ChatFrame1:GetPoint(1)
            local editBox = assert(ChatFrame1EditBox)
            return table.concat({
                ChatFrame1:GetWidth(), ChatFrame1:GetHeight(),
                point, relative:GetName(), relativePoint, x, y,
                tostring(editBox:IsShown()), editBox:GetWidth(), editBox:GetHeight(),
            }, "|")
        "#;
        let native: String = env.eval(snapshot).expect("native chat surface");
        wow_ui_sim::lua_api::chat_init::prepare_for_third_party_addons(&env);
        let prepared: String = env.eval(snapshot).expect("prepared chat surface");
        assert_eq!(prepared, native, "addon preparation must preserve native chat geometry and editbox state");
        env.apply_post_load_workarounds();
        let post_load: String = env.eval(snapshot).expect("post-load chat surface");
        assert_eq!(post_load, native, "post-load chat activation must preserve native chat surface");
        env.apply_post_event_workarounds();
        let post_event: String = env.eval(snapshot).expect("post-event chat surface");
        assert_eq!(post_event, native, "post-event chat activation must preserve native chat surface");
    }
}
