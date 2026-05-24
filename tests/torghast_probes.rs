use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::TorghastState;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn default_returns_false() {
    let env = env();
    let in_tower: bool = env.eval("return IsInJailersTower()").unwrap();
    let on_ground_floor: bool = env.eval("return IsOnGroundFloorInJailersTower()").unwrap();
    assert!(
        !in_tower,
        "should return false when no Torghast run is active"
    );
    assert!(
        !on_ground_floor,
        "should return false when no Torghast run is active"
    );
}

#[test]
fn active_floor_1_returns_true() {
    let env = env();
    {
        let state = env.state();
        let mut s = state.borrow_mut();
        s.torghast = TorghastState {
            active: true,
            floor: 1,
        };
    }
    let in_tower: bool = env.eval("return IsInJailersTower()").unwrap();
    let on_ground_floor: bool = env.eval("return IsOnGroundFloorInJailersTower()").unwrap();
    assert!(in_tower, "active run should return true");
    assert!(on_ground_floor, "active run on floor 1 should return true");
}

#[test]
fn active_floor_2_returns_false() {
    let env = env();
    {
        let state = env.state();
        let mut s = state.borrow_mut();
        s.torghast = TorghastState {
            active: true,
            floor: 2,
        };
    }
    let in_tower: bool = env.eval("return IsInJailersTower()").unwrap();
    let on_ground_floor: bool = env.eval("return IsOnGroundFloorInJailersTower()").unwrap();
    assert!(in_tower, "active run should return true");
    assert!(
        !on_ground_floor,
        "active run on floor 2 should return false"
    );
}

#[test]
fn inactive_floor_1_returns_false() {
    let env = env();
    {
        let state = env.state();
        let mut s = state.borrow_mut();
        s.torghast = TorghastState {
            active: false,
            floor: 1,
        };
    }
    let result: bool = env.eval("return IsOnGroundFloorInJailersTower()").unwrap();
    assert!(!result, "inactive run even on floor 1 should return false");
}

#[test]
fn mutation_reflects_live() {
    let env = env();

    let before: bool = env.eval("return IsOnGroundFloorInJailersTower()").unwrap();
    assert!(!before, "should start false");

    {
        let state = env.state();
        let mut s = state.borrow_mut();
        s.torghast = TorghastState {
            active: true,
            floor: 1,
        };
    }

    let after: bool = env.eval("return IsOnGroundFloorInJailersTower()").unwrap();
    assert!(after, "should reflect state mutation");
}
