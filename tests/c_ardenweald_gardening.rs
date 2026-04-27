//! Integration tests for the `C_ArdenwealdGardening` surface registered
//! in `src/c_api/c_ardenweald_gardening.rs`. Drives the gating logic in
//! `LandingPageMixin:UpdateArdenwealdGardeningSection`
//! (`Blizzard_GarrisonLandingPage.lua:190`) and the OnEnter tooltip in
//! `ArdenwealdGardeningButtonMixin:OnEnter`
//! (`Blizzard_ArdenwealdGardening.lua:24-38`).

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn c_ardenweald_gardening_globals_are_registered() {
    let env = WowLuaEnv::new().expect("env");
    let namespace_kind: String = env.eval("return type(C_ArdenwealdGardening)").unwrap();
    assert_eq!(namespace_kind, "table");

    let get_kind: String = env
        .eval("return type(C_ArdenwealdGardening.GetGardenData)")
        .unwrap();
    assert_eq!(get_kind, "function");

    let accessible_kind: String = env
        .eval("return type(C_ArdenwealdGardening.IsGardenAccessible)")
        .unwrap();
    assert_eq!(accessible_kind, "function");
}

#[test]
fn is_garden_accessible_default_is_false() {
    let env = WowLuaEnv::new().expect("env");
    let accessible: bool = env
        .eval("return C_ArdenwealdGardening.IsGardenAccessible()")
        .unwrap();
    assert!(
        !accessible,
        "fresh simulator must keep the gardening panel hidden"
    );
}

#[test]
fn is_garden_accessible_reflects_state_mutation() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().gardenweald.accessible = true;

    let accessible: bool = env
        .eval("return C_ArdenwealdGardening.IsGardenAccessible()")
        .unwrap();
    assert!(accessible);
}

#[test]
fn is_garden_accessible_returns_one_value() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return select('#', C_ArdenwealdGardening.IsGardenAccessible())")
        .unwrap();
    assert_eq!(count, 1.0);
}

#[test]
fn get_garden_data_default_returns_zero_table() {
    let env = WowLuaEnv::new().expect("env");
    env.exec("data = C_ArdenwealdGardening.GetGardenData()")
        .unwrap();

    let kind: String = env.eval("return type(data)").unwrap();
    assert_eq!(kind, "table");

    let active: f64 = env.eval("return data.active").unwrap();
    let ready: f64 = env.eval("return data.ready").unwrap();
    let remaining: f64 = env.eval("return data.remainingSeconds").unwrap();
    assert_eq!(active, 0.0);
    assert_eq!(ready, 0.0);
    assert_eq!(remaining, 0.0);
}

#[test]
fn get_garden_data_reflects_state_mutation() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut state = env.state().borrow_mut();
        state.gardenweald.active = 3;
        state.gardenweald.ready = 1;
        state.gardenweald.remaining_seconds = 600;
    }

    env.exec("data = C_ArdenwealdGardening.GetGardenData()")
        .unwrap();
    let active: f64 = env.eval("return data.active").unwrap();
    let ready: f64 = env.eval("return data.ready").unwrap();
    let remaining: f64 = env.eval("return data.remainingSeconds").unwrap();
    assert_eq!(active, 3.0);
    assert_eq!(ready, 1.0);
    assert_eq!(remaining, 600.0);
}

#[test]
fn get_garden_data_returns_one_value() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return select('#', C_ArdenwealdGardening.GetGardenData())")
        .unwrap();
    assert_eq!(count, 1.0);
}

#[test]
fn on_enter_active_branch_runs_without_errors() {
    // Mirrors the `data.active > 0` branch in
    // `ArdenwealdGardeningButtonMixin:OnEnter`. Without the populated
    // table the handler errors on `data.active > 0` because data is nil.
    let env = WowLuaEnv::new().expect("env");
    {
        let mut state = env.state().borrow_mut();
        state.gardenweald.accessible = true;
        state.gardenweald.active = 2;
        state.gardenweald.ready = 0;
        state.gardenweald.remaining_seconds = 1_800;
    }

    env.exec(
        r#"
        local data = C_ArdenwealdGardening.GetGardenData()
        assert(data, "GetGardenData must return a table")
        assert(data.active > 0, "active branch should be reachable")
        ARDENWEALD_ACTIVE_BRANCH_OK = data.active == 2 and data.ready == 0 and data.remainingSeconds == 1800
        "#,
    )
    .unwrap();

    let ok: bool = env
        .eval("return ARDENWEALD_ACTIVE_BRANCH_OK == true")
        .unwrap();
    assert!(ok, "OnEnter active branch probe should succeed");
}
