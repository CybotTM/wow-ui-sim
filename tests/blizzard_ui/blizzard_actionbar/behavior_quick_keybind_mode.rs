//! Behavior pin: opening QuickKeybindFrame enters quick-keybind mode and
//! overlays action buttons with quick-keybind highlight surfaces.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBar";

#[test]
fn quick_keybind_mode_shows_button_highlights_until_closed() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        isolate_quick_keybind_close_side_effects(env);
        seed_visible_hotkeys(env);
        assert_quick_keybind_mode_exited(env);
        assert_button_highlights_hidden(env);

        show_quick_keybind_frame(env);
        assert_quick_keybind_mode_entered(env);
        assert_button_highlights_visible(env);

        hide_quick_keybind_frame(env);
        assert_quick_keybind_mode_exited(env);
        assert_button_highlights_hidden(env);
    });
    }
}

fn isolate_quick_keybind_close_side_effects(env: &WowLuaEnv) {
    env.exec(
        r#"
        EditModeManagerFrame.ShowIfActive = function() return true end
        "#,
    )
    .expect("quick-keybind close fixture must install cleanly");
}

fn seed_visible_hotkeys(env: &WowLuaEnv) {
    let hotkeys_visible: bool = env
        .eval(
            r#"
            ActionButton1:UpdateHotkeys()
            MultiBarBottomLeftButton1:UpdateHotkeys()
            MultiBarRightButton1:UpdateHotkeys()
            return ActionButton1.HotKey:IsShown()
                and MultiBarBottomLeftButton1.HotKey:IsShown()
                and MultiBarRightButton1.HotKey:IsShown()
                and ActionButton1.HotKey:GetText() ~= ""
                and MultiBarBottomLeftButton1.HotKey:GetText() ~= ""
                and MultiBarRightButton1.HotKey:GetText() ~= ""
            "#,
        )
        .expect("hotkey label seed probe must run cleanly");
    assert!(
        hotkeys_visible,
        "sample action buttons must expose keybind labels"
    );
}

fn show_quick_keybind_frame(env: &WowLuaEnv) {
    env.eval::<()>("QuickKeybindFrame:Show()")
        .expect("QuickKeybindFrame show must run cleanly");
}

fn hide_quick_keybind_frame(env: &WowLuaEnv) {
    env.eval::<()>("QuickKeybindFrame:Hide()")
        .expect("QuickKeybindFrame hide must run cleanly");
}

fn assert_quick_keybind_mode_entered(env: &WowLuaEnv) {
    let entered: bool = env
        .eval("return KeybindFrames_InQuickKeybindMode() == true")
        .expect("quick-keybind entered probe must run cleanly");
    assert!(
        entered,
        "QuickKeybindFrame:Show() must enter quick-keybind mode"
    );
}

fn assert_quick_keybind_mode_exited(env: &WowLuaEnv) {
    let exited: bool = env
        .eval("return not KeybindFrames_InQuickKeybindMode()")
        .expect("quick-keybind exited probe must run cleanly");
    assert!(
        exited,
        "QuickKeybindFrame must not be in quick-keybind mode"
    );
}

fn assert_button_highlights_visible(env: &WowLuaEnv) {
    let shown: bool = env
        .eval(
            r#"
            return ActionButton1.QuickKeybindHighlightTexture:IsShown()
                and MultiBarBottomLeftButton1.QuickKeybindHighlightTexture:IsShown()
                and MultiBarRightButton1.QuickKeybindHighlightTexture:IsShown()
            "#,
        )
        .expect("quick-keybind visible probe must run cleanly");
    assert!(shown, "quick-keybind mode must show action-button overlays");
}

fn assert_button_highlights_hidden(env: &WowLuaEnv) {
    let hidden: bool = env
        .eval(
            r#"
            return not ActionButton1.QuickKeybindHighlightTexture:IsShown()
                and not MultiBarBottomLeftButton1.QuickKeybindHighlightTexture:IsShown()
                and not MultiBarRightButton1.QuickKeybindHighlightTexture:IsShown()
            "#,
        )
        .expect("quick-keybind hidden probe must run cleanly");
    assert!(
        hidden,
        "leaving quick-keybind mode must hide action-button overlays"
    );
}
