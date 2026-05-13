//! Behavior pin: new-action highlight marks drive ActionButton glow state.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBar";
const ACTION_ID: u32 = 1;
const SPELL_ID: u32 = 1234;

#[test]
fn new_action_highlight_shows_until_cleared() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_action_button_spell(env);

        mark_action_highlight(env);
        assert_highlight_visible(env);

        clear_action_highlight(env);
        assert_highlight_hidden(env);
    });
    }
}

fn seed_action_button_spell(env: &WowLuaEnv) {
    env.state()
        .borrow_mut()
        .action_bars
        .insert(ACTION_ID, SPELL_ID);
    let seeded: bool = env
        .eval(
            r#"
            ActionButton1:UpdateAction(true)
            local actionType, spellID = GetActionInfo(ActionButton1.action)
            return actionType == "spell" and spellID == 1234
            "#,
        )
        .expect("action button spell seed probe must run cleanly");
    assert!(seeded, "ActionButton1 must hold spell 1234");
}

fn mark_action_highlight(env: &WowLuaEnv) {
    let marked: bool = env
        .eval(
            r#"
            MarkNewActionHighlight(ActionButton1.action)
            ActionButton1:UpdateHighlightMark()
            return GetNewActionHighlightMark(ActionButton1.action) == true
            "#,
        )
        .expect("new action highlight mark must run cleanly");
    assert!(
        marked,
        "MarkNewActionHighlight must record ActionButton1.action"
    );
}

fn clear_action_highlight(env: &WowLuaEnv) {
    let cleared: bool = env
        .eval(
            r#"
            ClearNewActionHighlight(ActionButton1.action)
            ActionButton1:UpdateHighlightMark()
            return GetNewActionHighlightMark(ActionButton1.action) == nil
            "#,
        )
        .expect("new action highlight clear must run cleanly");
    assert!(
        cleared,
        "ClearNewActionHighlight must clear ActionButton1.action"
    );
}

fn assert_highlight_visible(env: &WowLuaEnv) {
    let shown: bool = env
        .eval("return ActionButton1.NewActionTexture:IsShown() == true")
        .expect("new action highlight visible probe must run cleanly");
    assert!(shown, "marked action button must show NewActionTexture");
}

fn assert_highlight_hidden(env: &WowLuaEnv) {
    let hidden: bool = env
        .eval("return ActionButton1.NewActionTexture:IsShown() == false")
        .expect("new action highlight hidden probe must run cleanly");
    assert!(hidden, "cleared action button must hide NewActionTexture");
}
