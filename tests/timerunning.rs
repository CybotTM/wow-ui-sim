//! `PlayerIsTimerunning` + `PlayerGetTimerunningSeasonID` round-trip coverage.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn timerunning_defaults_to_inactive() {
    let env = WowLuaEnv::new().expect("env");
    let (is_active, season_id): (bool, f64) = env
        .eval("return PlayerIsTimerunning(), PlayerGetTimerunningSeasonID()")
        .expect("both timerunning probes should return");
    assert!(!is_active, "default should not be in timerunning mode");
    assert_eq!(season_id, 0.0, "default season id should be 0");
}

#[test]
fn admin_set_timerunning_season_id_enables_and_clears() {
    let env = WowLuaEnv::new().expect("env");

    env.exec("A_Admin.SetTimerunningSeasonID(2)").unwrap();
    let (is_active, season_id): (bool, f64) = env
        .eval("return PlayerIsTimerunning(), PlayerGetTimerunningSeasonID()")
        .unwrap();
    assert!(is_active, "non-zero season id should mark player timerunning");
    assert_eq!(season_id, 2.0);

    env.exec("A_Admin.SetTimerunningSeasonID(0)").unwrap();
    let (is_active, season_id): (bool, f64) = env
        .eval("return PlayerIsTimerunning(), PlayerGetTimerunningSeasonID()")
        .unwrap();
    assert!(!is_active, "season id 0 should clear timerunning");
    assert_eq!(season_id, 0.0);
}

#[test]
fn admin_set_timerunning_season_id_accepts_nil_as_clear() {
    let env = WowLuaEnv::new().expect("env");
    env.exec("A_Admin.SetTimerunningSeasonID(5)").unwrap();
    env.exec("A_Admin.SetTimerunningSeasonID(nil)").unwrap();
    let (is_active, season_id): (bool, f64) = env
        .eval("return PlayerIsTimerunning(), PlayerGetTimerunningSeasonID()")
        .unwrap();
    assert!(!is_active);
    assert_eq!(season_id, 0.0);
}

#[test]
fn admin_set_timerunning_season_id_rejects_negative_as_clear() {
    let env = WowLuaEnv::new().expect("env");
    env.exec("A_Admin.SetTimerunningSeasonID(-1)").unwrap();
    let (is_active, season_id): (bool, f64) = env
        .eval("return PlayerIsTimerunning(), PlayerGetTimerunningSeasonID()")
        .unwrap();
    assert!(!is_active, "negative id should be treated as no season");
    assert_eq!(season_id, 0.0);
}
