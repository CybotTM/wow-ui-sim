//! Integration tests for `C_TransmogOutfitInfo` lock probes in
//! `src/lua_api/globals/transmog_outfit_info.rs`:
//! `IsLockedOutfit` and `IsEquippedGearOutfitLocked` driven by
//! `state.transmog_outfit_locks` / `state.equipped_outfit_locked`.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn is_locked_outfit_is_false_when_state_empty() {
    let env = WowLuaEnv::new().expect("env");
    let result: bool = env
        .eval("return C_TransmogOutfitInfo.IsLockedOutfit(42)")
        .unwrap();
    assert!(!result);
}

#[test]
fn is_locked_outfit_reads_state() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().transmog_outfit_locks.insert(42);
    let listed: bool = env
        .eval("return C_TransmogOutfitInfo.IsLockedOutfit(42)")
        .unwrap();
    let unlisted: bool = env
        .eval("return C_TransmogOutfitInfo.IsLockedOutfit(99)")
        .unwrap();
    assert!(listed);
    assert!(!unlisted);
}

#[test]
fn is_locked_outfit_returns_false_when_id_missing() {
    let env = WowLuaEnv::new().expect("env");
    let no_arg: bool = env
        .eval("return C_TransmogOutfitInfo.IsLockedOutfit()")
        .unwrap();
    let nil_arg: bool = env
        .eval("return C_TransmogOutfitInfo.IsLockedOutfit(nil)")
        .unwrap();
    assert!(!no_arg);
    assert!(!nil_arg);
}

#[test]
fn is_equipped_gear_outfit_locked_defaults_false() {
    let env = WowLuaEnv::new().expect("env");
    let result: bool = env
        .eval("return C_TransmogOutfitInfo.IsEquippedGearOutfitLocked()")
        .unwrap();
    assert!(!result);
}

#[test]
fn is_equipped_gear_outfit_locked_reflects_state() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().equipped_outfit_locked = true;
    let result: bool = env
        .eval("return C_TransmogOutfitInfo.IsEquippedGearOutfitLocked()")
        .unwrap();
    assert!(result);
}

#[test]
fn action_button_update_usable_branches_use_lock_probes() {
    // Mirrors the ActionBarButtonMixin:UpdateUsable check at
    // vendor/wow-ui-source/Interface/AddOns/Blizzard_ActionBar/Shared/ActionButton.lua:1316-1317.
    let env = WowLuaEnv::new().expect("env");
    {
        let mut state = env.state().borrow_mut();
        state.transmog_outfit_locks.insert(7);
        state.equipped_outfit_locked = true;
    }
    env.exec(
        r#"
        local function lockedFlags(actionType, actionID, isEquippedGearAction)
            local isLockedOutfit = actionType == "outfit"
                and C_TransmogOutfitInfo.IsLockedOutfit(actionID)
            local isLockedEquippedGear = isEquippedGearAction
                and C_TransmogOutfitInfo.IsEquippedGearOutfitLocked()
            return isLockedOutfit, isLockedEquippedGear
        end
        outfitLocked, gearLocked = lockedFlags("outfit", 7, true)
        outfitFree, gearFree = lockedFlags("spell", 7, false)
    "#,
    )
    .unwrap();
    let outfit_locked: bool = env.eval("return outfitLocked").unwrap();
    let gear_locked: bool = env.eval("return gearLocked").unwrap();
    let outfit_free: bool = env.eval("return outfitFree").unwrap();
    let gear_free: bool = env.eval("return gearFree").unwrap();
    assert!(outfit_locked);
    assert!(gear_locked);
    assert!(!outfit_free);
    assert!(!gear_free);
}
