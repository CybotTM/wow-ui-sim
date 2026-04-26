//! `C_PvP.IsActiveBattlefield()` reads `state.is_active_battlefield`. The
//! `StatusTrackingManager` checks this to hide XP / honor bars while the
//! player is queued into a battlefield.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn defaults_false() {
    let env = WowLuaEnv::new().expect("env");
    let result: bool = env.eval("return C_PvP.IsActiveBattlefield()").unwrap();
    assert!(!result);
}

#[test]
fn reflects_state_flag() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().is_active_battlefield = true;
    let result: bool = env.eval("return C_PvP.IsActiveBattlefield()").unwrap();
    assert!(result);
}

#[test]
fn status_tracking_branches_use_probe() {
    // Mirrors StatusTrackingManager: if the player is in a battlefield,
    // the experience/honor bars must hide regardless of XP gain state.
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        local function shouldShowExperienceBar(xpDisabled)
            if C_PvP.IsActiveBattlefield() then
                return false
            end
            return not xpDisabled
        end
        free = shouldShowExperienceBar(false)
        in_battlefield = nil
    "#,
    )
    .unwrap();
    env.state().borrow_mut().is_active_battlefield = true;
    env.exec(
        r#"
        local function shouldShowExperienceBar(xpDisabled)
            if C_PvP.IsActiveBattlefield() then
                return false
            end
            return not xpDisabled
        end
        in_battlefield = shouldShowExperienceBar(false)
    "#,
    )
    .unwrap();
    let free: bool = env.eval("return free").unwrap();
    let in_battlefield: bool = env.eval("return in_battlefield").unwrap();
    assert!(free);
    assert!(!in_battlefield);
}
