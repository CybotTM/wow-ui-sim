//! Exact negative return shape and explicit temporary missing-effect-data policy.
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
fn g1_temporary_occupied_spell_without_effect_types_is_non_attack() {
    let env = WowLuaEnv::new().unwrap();
    {
        let mut sim = env.state().borrow_mut();
        sim.action_macros.remove(&7);
        sim.action_outfits.remove(&7);
        // Synthetic supplied spell, not an authenticated native attack/non-attack ID.
        sim.action_bars.insert(7, 1_000_001);
    }
    env.exec(r#"
        assert(IsAttackAction(7)==nil, 'temporary: absent effect types treats occupied slots as non-attack')
        assert(select('#', IsAttackAction(7))==1)
        -- Simulator namespace preservation, not a 12340 namespace parity claim.
        assert(C_ActionBar.IsAttackAction(7)==false)
    "#).unwrap();
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
