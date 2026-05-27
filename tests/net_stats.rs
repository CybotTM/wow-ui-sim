//! `GetNetStats` + `A_Admin.SetNetStats` round-trip coverage.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn network_globals_are_classified_by_backing_model() {
    let globals_mod = include_str!("../src/lua_api/globals/mod.rs");
    let real_mod = include_str!("../src/lua_api/globals/real/mod.rs");
    let registrar = include_str!("../src/lua_api/globals/register.rs");
    let performance_defaults =
        include_str!("../src/lua_api/workarounds/temporary/performance_metric_defaults.rs");

    assert!(
        !globals_mod.contains("pub mod net_stats;"),
        "state-backed network stats should not live in the globals base module"
    );
    assert!(
        real_mod.contains("pub mod net_stats;"),
        "state-backed GetNetStats should be classified under globals::real"
    );
    assert!(
        registrar.contains("real::net_stats::register_all"),
        "global registrar should wire GetNetStats through globals::real"
    );
    assert!(
        performance_defaults.contains("GetDownloadedPercentage")
            && performance_defaults.contains("GetMovieDownloadProgress"),
        "unmodeled download pipeline defaults belong in temporary workarounds"
    );
}

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

#[test]
fn get_downloaded_percentage_defaults_to_fully_downloaded() {
    let env = WowLuaEnv::new().expect("env");
    let percent: f64 = env
        .eval("return GetDownloadedPercentage()")
        .expect("GetDownloadedPercentage should be callable");
    assert_eq!(percent, 1.0);
}

#[test]
fn get_movie_download_progress_defaults_to_no_active_download() {
    let env = WowLuaEnv::new().expect("env");
    let (in_progress, downloaded, total): (bool, f64, f64) = env
        .eval("return GetMovieDownloadProgress(1)")
        .expect("GetMovieDownloadProgress should be callable");
    assert!(!in_progress);
    assert_eq!(downloaded, 0.0);
    assert_eq!(total, 0.0);
}
