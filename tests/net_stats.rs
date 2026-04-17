//! `GetNetStats` + `A_Admin.SetNetStats` round-trip coverage.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn get_net_stats_defaults_to_zero() {
    let env = WowLuaEnv::new().expect("env");
    let (bw_in, bw_out, lat_home, lat_world): (f64, f64, f64, f64) = env
        .eval("return GetNetStats()")
        .expect("GetNetStats should return four numbers");
    assert_eq!(bw_in, 0.0);
    assert_eq!(bw_out, 0.0);
    assert_eq!(lat_home, 0.0);
    assert_eq!(lat_world, 0.0);
}

#[test]
fn admin_set_net_stats_flows_through_to_get_net_stats() {
    let env = WowLuaEnv::new().expect("env");
    env.exec("A_Admin.SetNetStats(128, 64, 47, 89)")
        .expect("A_Admin.SetNetStats should succeed");
    let (bw_in, bw_out, lat_home, lat_world): (f64, f64, f64, f64) = env
        .eval("return GetNetStats()")
        .expect("GetNetStats should read admin-set values");
    assert_eq!(bw_in, 128.0);
    assert_eq!(bw_out, 64.0);
    assert_eq!(lat_home, 47.0);
    assert_eq!(lat_world, 89.0);
}

#[test]
fn admin_set_net_stats_accepts_missing_trailing_args() {
    let env = WowLuaEnv::new().expect("env");
    env.exec("A_Admin.SetNetStats(10, 20)")
        .expect("partial SetNetStats should succeed");
    let (bw_in, bw_out, lat_home, lat_world): (f64, f64, f64, f64) =
        env.eval("return GetNetStats()").expect("GetNetStats");
    assert_eq!(bw_in, 10.0);
    assert_eq!(bw_out, 20.0);
    assert_eq!(lat_home, 0.0, "missing latencyHome should default to 0");
    assert_eq!(lat_world, 0.0, "missing latencyWorld should default to 0");
}

#[test]
fn admin_set_net_stats_overwrites_previous_values() {
    let env = WowLuaEnv::new().expect("env");
    env.exec("A_Admin.SetNetStats(100, 200, 300, 400)").unwrap();
    env.exec("A_Admin.SetNetStats(1, 2, 3, 4)").unwrap();
    let (bw_in, bw_out, lat_home, lat_world): (f64, f64, f64, f64) =
        env.eval("return GetNetStats()").expect("GetNetStats");
    assert_eq!((bw_in, bw_out, lat_home, lat_world), (1.0, 2.0, 3.0, 4.0));
}
