//! Only exact negative return shape is unblocked by current spell data.
#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn g1_empty_one_based_action_returns_one_nil_not_boolean_false() {
    let env = WowLuaEnv::new().unwrap();
    {
        let mut sim = env.state().borrow_mut();
        sim.action_bars.remove(&1);
        sim.action_macros.remove(&1);
        sim.action_outfits.remove(&1);
    }
    env.exec(
        r#"
        assert(IsAttackAction(1) == nil, "Wrath negative is nil, not false")
        assert(select('#', IsAttackAction(1)) == 1, "one nil result, not zero results")
    "#,
    )
    .unwrap();
}

#[test]
fn g1_zero_slot_is_not_a_one_based_attack_slot() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(IsAttackAction(0) == nil, "slot 0 has no resolved spell")
        assert(select('#', IsAttackAction(0)) == 1)
    "#,
    )
    .unwrap();
}
