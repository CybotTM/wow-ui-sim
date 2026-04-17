//! Integration tests for `src/lua_api/globals/movement_verbs.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

fn fired(env: &WowLuaEnv, name: &str) -> bool {
    env.state()
        .borrow()
        .events
        .pending()
        .iter()
        .any(|e| e.name == name)
}

#[test]
fn move_forward_start_flips_moving_and_fires_event() {
    let env = env();
    env.exec("MoveForwardStart()").unwrap();
    assert!(env.state().borrow().player.movement.moving);
    assert!(fired(&env, "PLAYER_STARTED_MOVING"));
}

#[test]
fn move_forward_stop_flips_moving_and_fires_event() {
    let env = env();
    env.state().borrow_mut().player.movement.moving = true;
    env.exec("MoveForwardStop()").unwrap();
    assert!(!env.state().borrow().player.movement.moving);
    assert!(fired(&env, "PLAYER_STOPPED_MOVING"));
}

#[test]
fn start_stop_cycle_toggles_flag() {
    let env = env();
    env.exec(
        "MoveForwardStart()
         MoveForwardStop()
         MoveForwardStart()",
    )
    .unwrap();
    assert!(env.state().borrow().player.movement.moving);
}
