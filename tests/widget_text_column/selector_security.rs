//! Numeric and real host-secret selectors over distinct supplied TextColumnRow records.
use super::fixtures::{self, ID, SET_ID, TYPE};
use super::selector_shared;
use wow_ui_sim::c_api::ui_widget_manager::{WidgetId, WidgetSetEntry, WidgetVisualization};
use wow_ui_sim::lua_api::WowLuaEnv;

const FRACTION_ID: f64 = ID + 0.25;

fn publish_distinct_records(env: &WowLuaEnv) {
    fixtures::supply_empty_set(env);
    for (id, record) in [
        (ID, fixtures::supplied()),
        (FRACTION_ID, fixtures::replacement()),
    ] {
        let membership = WidgetSetEntry {
            widget_id: id,
            widget_type: TYPE,
            unit_token: None,
        };
        env.update_ui_widget(
            SET_ID,
            membership,
            WidgetVisualization::TextColumnRow(record),
        )
        .expect("normal typed integer/fraction TextColumnRow feed");
    }
    fixtures::install_assertions(env);
    selector_shared::assert_lua(
        env,
        "local complete deep assertion and retained feed snapshots",
        include_str!("selector_security.lua"),
    );
}

fn assert_exact_supplied_records(env: &WowLuaEnv) {
    let state = env.state().borrow();
    let rows = &state.ui_widget_inputs.text_column_rows;
    assert_eq!(rows.len(), 2, "only explicitly supplied records");
    assert_eq!(
        rows.get(&WidgetId::from_number(ID)),
        Some(&fixtures::supplied())
    );
    assert_eq!(
        rows.get(&WidgetId::from_number(FRACTION_ID)),
        Some(&fixtures::replacement())
    );
}

#[test]
fn text_column_additional_numeric_selectors_preserve_distinct_normal_feed_state() {
    let env = WowLuaEnv::new().unwrap();
    publish_distinct_records(&env);
    assert_exact_supplied_records(&env);
    let before_rows = env
        .state()
        .borrow()
        .ui_widget_inputs
        .text_column_rows
        .clone();
    let before_sets = env.state().borrow().ui_widget_inputs.sets.clone();

    selector_shared::assert_selector_policy(&env, ID);
    selector_shared::assert_lua(
        &env,
        "select distinct fractional expectation",
        "FixtureExpected = ColumnReplacement",
    );
    selector_shared::assert_selector_policy(&env, FRACTION_ID);
    selector_shared::assert_lua(
        &env,
        "retained snapshots and deep nonalias caller mutation isolation",
        "AssertColumnMutationIsolation()",
    );

    assert_exact_supplied_records(&env);
    assert_eq!(
        env.state().borrow().ui_widget_inputs.text_column_rows,
        before_rows
    );
    assert_eq!(env.state().borrow().ui_widget_inputs.sets, before_sets);
}

#[cfg(all(
    any(feature = "client-retail", feature = "client-ptr"),
    feature = "retail-12-1-0"
))]
#[test]
fn text_column_additional_host_secret_numbers_enforce_caller_security_and_isolation() {
    let env = WowLuaEnv::new().unwrap();
    publish_distinct_records(&env);
    assert_exact_supplied_records(&env);
    let before_rows = env
        .state()
        .borrow()
        .ui_widget_inputs
        .text_column_rows
        .clone();
    let before_sets = env.state().borrow().ui_widget_inputs.sets.clone();

    // Canonical helper creates real host-secret NUMBER values, not Lua stand-ins.
    selector_shared::assert_secret_selector_contract(&env, ID);
    selector_shared::assert_lua(
        &env,
        "select distinct fractional expectation",
        "FixtureExpected = ColumnReplacement",
    );
    selector_shared::assert_secret_selector_contract(&env, FRACTION_ID);
    selector_shared::assert_lua(
        &env,
        "recovered outer context and retained deep public snapshots",
        "assert(debug.getstacktaint() == nil); AssertColumnMutationIsolation()",
    );

    assert_exact_supplied_records(&env);
    assert_eq!(
        env.state().borrow().ui_widget_inputs.text_column_rows,
        before_rows
    );
    assert_eq!(env.state().borrow().ui_widget_inputs.sets, before_sets);
}
