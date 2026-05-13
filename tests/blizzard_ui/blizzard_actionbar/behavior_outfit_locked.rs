//! Behavior pin: locked equipped-gear outfit actions show the action-button overlay.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBar";
const ACTION_ID: u32 = 1;
const EQUIPPED_GEAR_OUTFIT_ID: i64 = 0;

#[test]
fn equipped_gear_outfit_lock_shows_autocast_overlay_until_unlocked() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_equipped_gear_outfit_action(env);

        lock_equipped_gear_outfit(env);
        refresh_action_button_flash(env);
        assert_outfit_overlay_visible(env);

        unlock_equipped_gear_outfit(env);
        refresh_action_button_flash(env);
        assert_outfit_overlay_hidden(env);
    });
    }
}

fn seed_equipped_gear_outfit_action(env: &WowLuaEnv) {
    {
        let mut state = env.state().borrow_mut();
        state
            .action_outfits
            .insert(ACTION_ID, EQUIPPED_GEAR_OUTFIT_ID);
        state.equipped_gear_outfit_action_slots.insert(ACTION_ID);
    }
    let seeded: bool = env
        .eval(
            r#"
            ActionButton1:UpdateAction(true)
            local actionType, outfitID = GetActionInfo(ActionButton1.action)
            return actionType == "outfit"
                and outfitID == 0
                and C_ActionBar.IsEquippedGearOutfitAction(ActionButton1.action) == true
            "#,
        )
        .expect("equipped-gear outfit action seed probe must run cleanly");
    assert!(
        seeded,
        "ActionButton1 must hold the equipped-gear outfit action"
    );
}

fn lock_equipped_gear_outfit(env: &WowLuaEnv) {
    env.state().borrow_mut().equipped_outfit_locked = true;
    let locked: bool = env
        .eval("return C_TransmogOutfitInfo.IsEquippedGearOutfitLocked() == true")
        .expect("equipped outfit locked probe must run cleanly");
    assert!(
        locked,
        "C_TransmogOutfitInfo must report equipped gear locked"
    );
}

fn unlock_equipped_gear_outfit(env: &WowLuaEnv) {
    env.state().borrow_mut().equipped_outfit_locked = false;
    let unlocked: bool = env
        .eval("return C_TransmogOutfitInfo.IsEquippedGearOutfitLocked() == false")
        .expect("equipped outfit unlocked probe must run cleanly");
    assert!(
        unlocked,
        "C_TransmogOutfitInfo must report equipped gear unlocked"
    );
}

fn refresh_action_button_flash(env: &WowLuaEnv) {
    env.eval::<()>("ActionButton1:UpdateFlash()")
        .expect("ActionButton1 flash refresh must run cleanly");
}

fn assert_outfit_overlay_visible(env: &WowLuaEnv) {
    let shown: bool = env
        .eval(
            r#"
            return ActionButton1.AutoCastOverlay ~= nil
                and ActionButton1.AutoCastOverlay:IsShown() == true
                and ActionButton1.AutoCastOverlay.autoCastEnabled == true
                and ActionButton1.AutoCastOverlay.Shine:IsShown() == true
            "#,
        )
        .expect("outfit overlay visible probe must run cleanly");
    assert!(
        shown,
        "locked outfit action must show enabled AutoCastOverlay"
    );
}

fn assert_outfit_overlay_hidden(env: &WowLuaEnv) {
    let hidden: bool = env
        .eval(
            r#"
            return ActionButton1.AutoCastOverlay ~= nil
                and ActionButton1.AutoCastOverlay:IsShown() == false
                and ActionButton1.AutoCastOverlay.autoCastEnabled == false
                and ActionButton1.AutoCastOverlay.Shine:IsShown() == false
            "#,
        )
        .expect("outfit overlay hidden probe must run cleanly");
    assert!(hidden, "unlocked outfit action must hide AutoCastOverlay");
}
