use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn test_spell_static_fallback_shims_return_inert_values() {
    let env = WowLuaEnv::new().expect("WowLuaEnv init");
    let (override_id, maw_atlas_is_nil, current, max, start, duration, mod_rate): (
        i64,
        bool,
        i64,
        i64,
        i64,
        i64,
        f64,
    ) = env
        .eval(
            "local charges = C_Spell.GetSpellCharges(116)
             return C_Spell.GetOverrideSpell(116),
                    C_Spell.GetMawPowerBorderAtlasBySpellID(116) == nil,
                    charges.currentCharges,
                    charges.maxCharges,
                    charges.cooldownStartTime,
                    charges.cooldownDuration,
                    charges.chargeModRate",
        )
        .unwrap();

    assert_eq!(override_id, 116);
    assert!(maw_atlas_is_nil);
    assert_eq!(current, 0);
    assert_eq!(max, 0);
    assert_eq!(start, 0);
    assert_eq!(duration, 0);
    assert!((mod_rate - 1.0).abs() < 0.001);
}
