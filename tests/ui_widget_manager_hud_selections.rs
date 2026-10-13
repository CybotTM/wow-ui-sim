//! Supplied standard HUD set selections; zero default is simulator policy.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_selections(env: &WowLuaEnv, expected: [f64; 4]) {
    let queries = [
        "GetTopCenterWidgetSetID",
        "GetBelowMinimapWidgetSetID",
        "GetPowerBarWidgetSetID",
        "GetObjectiveTrackerWidgetSetID",
    ];
    for (query, value) in queries.into_iter().zip(expected) {
        env.exec(&format!(
            r#"
            local function check(...)
                assert(select('#', ...) == 1, '{query}: exactly one return')
                local actual = ...
                assert(type(actual) == 'number', '{query}: numeric return')
                assert(actual == {value}, '{query}: exact supplied value')
            end
            check(C_UIWidgetManager.{query}())
            "#,
        ))
        .unwrap_or_else(|error| panic!("{query}: {error}"));
    }
}

fn supply_selections(env: &WowLuaEnv, values: [f64; 4]) {
    let mut state = env.state().borrow_mut();
    let inputs = &mut state.ui_widget_inputs;
    inputs.top_center_widget_set_id = values[0];
    inputs.below_minimap_widget_set_id = values[1];
    inputs.power_bar_widget_set_id = values[2];
    inputs.objective_tracker_widget_set_id = values[3];
}

#[test]
fn hud_selection_defaults_return_exactly_one_number_zero() {
    // Use initialized environment defaults, not replacement UiWidgetInputs.
    let env = WowLuaEnv::new().unwrap();
    assert_selections(&env, [0.0; 4]);
}

#[test]
fn hud_selection_queries_return_distinct_supplied_values_without_set_catalog() {
    let env = WowLuaEnv::new().unwrap();
    // Synthetic host inputs, not native IDs; no matching set records supplied.
    supply_selections(&env, [417.0, 529.0, 641.0, 753.0]);
    assert_selections(&env, [417.0, 529.0, 641.0, 753.0]);
}

#[test]
fn hud_selection_queries_read_live_state_and_isolate_environments() {
    let first = WowLuaEnv::new().unwrap();
    let second = WowLuaEnv::new().unwrap();
    supply_selections(&first, [417.0, 529.0, 641.0, 753.0]);
    supply_selections(&second, [853.0, 967.0, 1081.0, 1193.0]);
    assert_selections(&first, [417.0, 529.0, 641.0, 753.0]);
    assert_selections(&second, [853.0, 967.0, 1081.0, 1193.0]);
    supply_selections(&first, [0.0, 529.0, 1601.0, 753.0]);
    assert_selections(&first, [0.0, 529.0, 1601.0, 753.0]);
    assert_selections(&second, [853.0, 967.0, 1081.0, 1193.0]);
    let untouched = WowLuaEnv::new().unwrap();
    assert_selections(&untouched, [0.0; 4]);
}
