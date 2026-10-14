use super::fixtures::{self, GETTER, ID};
use super::helpers::{assert_secret_selector_contract, install_snapshot_assertion};
use crate::common;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn text_with_subtext_actual_secret_selectors_allow_secure_deny_tainted_preserve_state() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::supply_empty_set(&env);
        fixtures::supply(&env, fixtures::supplied());
        install_snapshot_assertion(&env, GETTER, fixtures::EXPECTED);
        let (sets, records) = {
            let state = env.state();
            let state = state.borrow();
            (
                state.ui_widget_inputs.sets.clone(),
                state.ui_widget_inputs.text_with_subtext.clone(),
            )
        };
        assert_secret_selector_contract(&env, ID);
        let state = env.state();
        let state = state.borrow();
        assert_eq!(state.ui_widget_inputs.sets, sets);
        assert_eq!(state.ui_widget_inputs.text_with_subtext, records);
    });
}
