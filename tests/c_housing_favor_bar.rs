//! Integration tests for the favor-bar `C_Housing` lookups in
//! `src/lua_api/globals/housing.rs`:
//! `GetTrackedHouseGuid`, `GetCurrentHouseLevelFavor`,
//! `GetHouseLevelFavorForLevel`, and `GetMaxHouseLevel` driven by
//! `state.housing`.

use wow_ui_sim::lua_api::{HousingState, WowLuaEnv};

fn sample_state() -> HousingState {
    HousingState {
        tracked_house_guid: Some("house-guid-42".to_string()),
        current_level: 3,
        current_favor: 1_500,
        next_threshold: 2_500,
        max_level: 10,
        level_thresholds: vec![0, 500, 1_500, 4_000, 8_500],
        ..HousingState::default()
    }
}

#[test]
fn get_tracked_house_guid_is_nil_when_unset() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_Housing.GetTrackedHouseGuid() == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn get_tracked_house_guid_reads_state() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().housing = sample_state();
    let guid: String = env.eval("return C_Housing.GetTrackedHouseGuid()").unwrap();
    assert_eq!(guid, "house-guid-42");
}

#[test]
fn get_max_house_level_reads_state() {
    let env = WowLuaEnv::new().expect("env");
    let default_zero: f64 = env.eval("return C_Housing.GetMaxHouseLevel()").unwrap();
    assert!(default_zero.abs() < 1e-6);
    env.state().borrow_mut().housing = sample_state();
    let seeded: f64 = env.eval("return C_Housing.GetMaxHouseLevel()").unwrap();
    assert!((seeded - 10.0).abs() < 1e-6);
}

#[test]
fn get_current_house_level_favor_returns_zeros_when_guid_mismatch() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().housing = sample_state();
    env.exec(
        r#"
        level, favor, threshold = C_Housing.GetCurrentHouseLevelFavor("not-the-tracked-house")
    "#,
    )
    .unwrap();
    let level: f64 = env.eval("return level").unwrap();
    let favor: f64 = env.eval("return favor").unwrap();
    let threshold: f64 = env.eval("return threshold").unwrap();
    assert!(level.abs() < 1e-6);
    assert!(favor.abs() < 1e-6);
    assert!(threshold.abs() < 1e-6);
}

#[test]
fn get_current_house_level_favor_returns_state_when_guid_matches() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().housing = sample_state();
    env.exec(
        r#"
        level, favor, threshold = C_Housing.GetCurrentHouseLevelFavor("house-guid-42")
    "#,
    )
    .unwrap();
    let level: f64 = env.eval("return level").unwrap();
    let favor: f64 = env.eval("return favor").unwrap();
    let threshold: f64 = env.eval("return threshold").unwrap();
    assert!((level - 3.0).abs() < 1e-6);
    assert!((favor - 1_500.0).abs() < 1e-6);
    assert!((threshold - 2_500.0).abs() < 1e-6);
}

#[test]
fn get_house_level_favor_for_level_indexes_threshold_table() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().housing = sample_state();
    let level_one: f64 = env
        .eval("return C_Housing.GetHouseLevelFavorForLevel(1)")
        .unwrap();
    let level_three: f64 = env
        .eval("return C_Housing.GetHouseLevelFavorForLevel(3)")
        .unwrap();
    let level_five: f64 = env
        .eval("return C_Housing.GetHouseLevelFavorForLevel(5)")
        .unwrap();
    assert!(level_one.abs() < 1e-6);
    assert!((level_three - 1_500.0).abs() < 1e-6);
    assert!((level_five - 8_500.0).abs() < 1e-6);
}

#[test]
fn get_house_level_favor_for_level_returns_zero_for_out_of_range() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().housing = sample_state();
    let above: f64 = env
        .eval("return C_Housing.GetHouseLevelFavorForLevel(99)")
        .unwrap();
    let below: f64 = env
        .eval("return C_Housing.GetHouseLevelFavorForLevel(0)")
        .unwrap();
    assert!(above.abs() < 1e-6);
    assert!(below.abs() < 1e-6);
}

#[test]
fn favor_bar_update_uses_state_thresholds() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().housing = sample_state();
    env.exec(
        r#"
        local function houseFavorBarUpdate(houseFavor)
            local current, minBar, maxBar, level = 0, 0, 1, 1
            if houseFavor then
                current = houseFavor.houseFavor
                level = houseFavor.houseLevel
                minBar = C_Housing.GetHouseLevelFavorForLevel(level)
                maxBar = C_Housing.GetHouseLevelFavorForLevel(level + 1)
            end
            return current, minBar, maxBar, level
        end
        current, minBar, maxBar, level = houseFavorBarUpdate({
            houseGUID = "house-guid-42",
            houseLevel = 3,
            houseFavor = 1500,
        })
    "#,
    )
    .unwrap();
    let current: f64 = env.eval("return current").unwrap();
    let min_bar: f64 = env.eval("return minBar").unwrap();
    let max_bar: f64 = env.eval("return maxBar").unwrap();
    let level: f64 = env.eval("return level").unwrap();
    assert!((current - 1_500.0).abs() < 1e-6);
    assert!((min_bar - 1_500.0).abs() < 1e-6);
    assert!((max_bar - 4_000.0).abs() < 1e-6);
    assert!((level - 3.0).abs() < 1e-6);
}

#[test]
fn favor_bar_skips_set_bar_values_when_max_is_zero() {
    // The mixin only calls SetBarValues when maxBar != 0. With no thresholds
    // configured, GetHouseLevelFavorForLevel(level + 1) returns 0, so the bar
    // legitimately stays inert.
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        local _, _, maxBar = C_Housing.GetHouseLevelFavorForLevel(1), nil,
            C_Housing.GetHouseLevelFavorForLevel(2)
        max = maxBar
    "#,
    )
    .unwrap();
    let max_bar: f64 = env.eval("return max").unwrap();
    assert!(max_bar.abs() < 1e-6);
}
