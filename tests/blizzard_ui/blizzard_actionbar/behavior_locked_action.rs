//! Behavior pin: level-link action locks show the action-button overlay.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBar";
const ACTION_ID: u32 = 1;
const SPELL_ID: u32 = 1234;

#[test]
fn locked_action_shows_level_link_overlay_until_unlocked() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_action_button_spell(env);

        lock_action_slot(env);
        refresh_action_button(env);
        assert_level_link_overlay_visible(env);

        unlock_action_slot(env);
        refresh_action_button(env);
        assert_level_link_overlay_hidden(env);
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

fn lock_action_slot(env: &WowLuaEnv) {
    env.state().borrow_mut().locked_action_slots.insert(1);
    let locked: bool = env
        .eval("return C_LevelLink.IsActionLocked(ActionButton1.action) == true")
        .expect("level-link locked probe must run cleanly");
    assert!(
        locked,
        "C_LevelLink must report ActionButton1.action locked"
    );
}

fn unlock_action_slot(env: &WowLuaEnv) {
    env.state().borrow_mut().locked_action_slots.remove(&1);
    let unlocked: bool = env
        .eval("return C_LevelLink.IsActionLocked(ActionButton1.action) == false")
        .expect("level-link unlocked probe must run cleanly");
    assert!(
        unlocked,
        "C_LevelLink must report ActionButton1.action unlocked"
    );
}

fn refresh_action_button(env: &WowLuaEnv) {
    env.eval::<()>("ActionButton1:UpdateAction(true)")
        .expect("ActionButton1 update must run cleanly");
}

fn assert_level_link_overlay_visible(env: &WowLuaEnv) {
    let shown: bool = env
        .eval(
            r#"
            return ActionButton1.LevelLinkLockIcon ~= nil
                and ActionButton1.LevelLinkLockIcon:IsShown() == true
            "#,
        )
        .expect("level-link overlay visible probe must run cleanly");
    assert!(shown, "locked action button must show LevelLinkLockIcon");
}

fn assert_level_link_overlay_hidden(env: &WowLuaEnv) {
    let hidden: bool = env
        .eval(
            r#"
            return ActionButton1.LevelLinkLockIcon ~= nil
                and ActionButton1.LevelLinkLockIcon:IsShown() == false
            "#,
        )
        .expect("level-link overlay hidden probe must run cleanly");
    assert!(hidden, "unlocked action button must hide LevelLinkLockIcon");
}
