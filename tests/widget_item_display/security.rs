use super::fixtures::{self, ID};
use super::helpers::assert_secret_selector_contract;
use crate::common;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn item_display_actual_secret_wrappers_allow_secure_deny_tainted_and_preserve_state() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::supply_empty_set(&env);
        fixtures::supply(&env, fixtures::supplied());
        fixtures::install_assertions(&env);
        let (sets, records) = {
            let state = env.state();
            let state = state.borrow();
            (
                state.ui_widget_inputs.sets.clone(),
                state.ui_widget_inputs.item_display.clone(),
            )
        };
        // Existing manager policy and actual rilua wrappers; no fabricated Lua secrets.
        // Nested AssertSuppliedSnapshot additionally checks every nested key/value is public.
        assert_secret_selector_contract(&env, ID);
        let state = env.state();
        let state = state.borrow();
        assert_eq!(state.ui_widget_inputs.sets, sets);
        assert_eq!(state.ui_widget_inputs.item_display, records);
    });
}
