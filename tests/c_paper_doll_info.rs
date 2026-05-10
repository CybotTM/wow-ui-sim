use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn armor_effectiveness_returns_number_for_paper_doll_math() {
    let env = env();
    let effectiveness: f64 = env
        .eval(
            r#"
            local value = C_PaperDollInfo.GetArmorEffectiveness(1000, 80)
            assert(type(value) == "number", "effectiveness must be numeric")
            return value
            "#,
        )
        .unwrap();

    assert!(
        (0.0..=0.85).contains(&effectiveness),
        "effectiveness should be clamped to a sane damage-reduction range, got {effectiveness}"
    );
}

#[test]
fn armor_effectiveness_against_target_allows_no_target_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_PaperDollInfo.GetArmorEffectivenessAgainstTarget(1000) == nil")
        .unwrap();

    assert!(is_nil);
}

#[test]
fn stagger_percentage_returns_number_and_nil_target_value_without_target() {
    let env = env();
    let (stagger, against_target_is_nil): (f64, bool) = env
        .eval(
            r#"
            local stagger, staggerAgainstTarget = C_PaperDollInfo.GetStaggerPercentage("player")
            return stagger, staggerAgainstTarget == nil
            "#,
        )
        .unwrap();

    assert_eq!(stagger, 0.0);
    assert!(against_target_is_nil);
}

#[test]
fn inventory_slot_enabled_matches_known_inventory_slots() {
    let env = env();
    let (head_enabled, main_hand_enabled, bogus_disabled): (bool, bool, bool) = env
        .eval(
            r#"
            return C_PaperDollInfo.IsInventorySlotEnabled("HeadSlot"),
                   C_PaperDollInfo.IsInventorySlotEnabled("MainHandSlot"),
                   not C_PaperDollInfo.IsInventorySlotEnabled("NotASlot")
            "#,
        )
        .unwrap();

    assert!(head_enabled);
    assert!(main_hand_enabled);
    assert!(bogus_disabled);
}
