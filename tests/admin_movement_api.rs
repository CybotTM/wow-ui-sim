//! Tests for A_Admin movement state API.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ============================================================================
// SetMoving — IsPlayerMoving()
// ============================================================================

#[test]
fn test_set_moving_true_is_player_moving() {
    let env = env();
    let moving: bool = env
        .eval(
            r#"
            A_Admin.SetMoving(true)
            return IsPlayerMoving()
            "#,
        )
        .unwrap();
    assert!(
        moving,
        "IsPlayerMoving() should return true after SetMoving(true)"
    );
}

#[test]
fn test_set_moving_false_is_player_moving() {
    let env = env();
    let moving: bool = env
        .eval(
            r#"
            A_Admin.SetMoving(true)
            A_Admin.SetMoving(false)
            return IsPlayerMoving()
            "#,
        )
        .unwrap();
    assert!(
        !moving,
        "IsPlayerMoving() should return false after SetMoving(false)"
    );
}

#[test]
fn test_player_not_moving_by_default() {
    let env = env();
    let moving: bool = env.eval("return IsPlayerMoving()").unwrap();
    assert!(!moving, "IsPlayerMoving() should be false in default state");
}

// ============================================================================
// SetMounted — IsMounted()
// ============================================================================

#[test]
fn test_set_mounted_true() {
    let env = env();
    let mounted: bool = env
        .eval(
            r#"
            A_Admin.SetMounted(true)
            return IsMounted()
            "#,
        )
        .unwrap();
    assert!(
        mounted,
        "IsMounted() should return true after SetMounted(true)"
    );
}

#[test]
fn test_set_mounted_false() {
    let env = env();
    let mounted: bool = env
        .eval(
            r#"
            A_Admin.SetMounted(true)
            A_Admin.SetMounted(false)
            return IsMounted()
            "#,
        )
        .unwrap();
    assert!(
        !mounted,
        "IsMounted() should return false after SetMounted(false)"
    );
}

#[test]
fn test_not_mounted_by_default() {
    let env = env();
    let mounted: bool = env.eval("return IsMounted()").unwrap();
    assert!(!mounted, "IsMounted() should be false in default state");
}

// ============================================================================
// SetFlying — IsFlying()
// ============================================================================

#[test]
fn test_set_flying_true() {
    let env = env();
    let flying: bool = env
        .eval(
            r#"
            A_Admin.SetFlying(true)
            return IsFlying()
            "#,
        )
        .unwrap();
    assert!(
        flying,
        "IsFlying() should return true after SetFlying(true)"
    );
}

#[test]
fn test_set_flying_false() {
    let env = env();
    let flying: bool = env
        .eval(
            r#"
            A_Admin.SetFlying(true)
            A_Admin.SetFlying(false)
            return IsFlying()
            "#,
        )
        .unwrap();
    assert!(
        !flying,
        "IsFlying() should return false after SetFlying(false)"
    );
}

#[test]
fn test_not_flying_by_default() {
    let env = env();
    let flying: bool = env.eval("return IsFlying()").unwrap();
    assert!(!flying, "IsFlying() should be false in default state");
}

// ============================================================================
// SetFalling — IsFalling()
// ============================================================================

#[test]
fn test_set_falling_true() {
    let env = env();
    let falling: bool = env
        .eval(
            r#"
            A_Admin.SetFalling(true)
            return IsFalling()
            "#,
        )
        .unwrap();
    assert!(
        falling,
        "IsFalling() should return true after SetFalling(true)"
    );
}

#[test]
fn test_set_falling_false() {
    let env = env();
    let falling: bool = env
        .eval(
            r#"
            A_Admin.SetFalling(true)
            A_Admin.SetFalling(false)
            return IsFalling()
            "#,
        )
        .unwrap();
    assert!(
        !falling,
        "IsFalling() should return false after SetFalling(false)"
    );
}

#[test]
fn test_not_falling_by_default() {
    let env = env();
    let falling: bool = env.eval("return IsFalling()").unwrap();
    assert!(!falling, "IsFalling() should be false in default state");
}

// ============================================================================
// SetSwimming — IsSwimming()
// ============================================================================

#[test]
fn test_set_swimming_true() {
    let env = env();
    let swimming: bool = env
        .eval(
            r#"
            A_Admin.SetSwimming(true)
            return IsSwimming()
            "#,
        )
        .unwrap();
    assert!(
        swimming,
        "IsSwimming() should return true after SetSwimming(true)"
    );
}

#[test]
fn test_set_swimming_false() {
    let env = env();
    let swimming: bool = env
        .eval(
            r#"
            A_Admin.SetSwimming(true)
            A_Admin.SetSwimming(false)
            return IsSwimming()
            "#,
        )
        .unwrap();
    assert!(
        !swimming,
        "IsSwimming() should return false after SetSwimming(false)"
    );
}

#[test]
fn test_not_swimming_by_default() {
    let env = env();
    let swimming: bool = env.eval("return IsSwimming()").unwrap();
    assert!(!swimming, "IsSwimming() should be false in default state");
}

// ============================================================================
// Movement states are independent
// ============================================================================

#[test]
fn test_movement_states_are_independent() {
    let env = env();
    // Setting one movement state should not affect others.
    let (moving, mounted, flying, falling, swimming): (bool, bool, bool, bool, bool) = env
        .eval(
            r#"
            A_Admin.SetMoving(true)
            return IsPlayerMoving(), IsMounted(), IsFlying(), IsFalling(), IsSwimming()
            "#,
        )
        .unwrap();
    assert!(moving);
    assert!(!mounted);
    assert!(!flying);
    assert!(!falling);
    assert!(!swimming);
}

#[test]
fn unit_position_defaults_to_origin_until_world_coordinates_are_modeled() {
    let env = env();
    let position: (f64, f64, f64, f64) = env.eval(r#"return UnitPosition("player")"#).unwrap();
    assert_eq!(position, (0.0, 0.0, 0.0, 0.0));
}
