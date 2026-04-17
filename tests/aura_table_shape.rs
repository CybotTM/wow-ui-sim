//! Pin the field set emitted by `build_aura_table` (now split into
//! `write_aura_identity` + `write_aura_flags`) in
//! `src/lua_api/globals/auras.rs`. Exercises `C_UnitAuras.GetAuraDataBySlot`
//! against the fixture BUFF/DEBUFF slots.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

#[test]
fn buff_slot_table_has_every_documented_field() {
    let env = env();
    let (name, icon, duration, source, spell_id, is_helpful, is_harmful, applications, time_mod): (
        String,
        f64,
        f64,
        String,
        f64,
        bool,
        bool,
        f64,
        f64,
    ) = env
        .eval(
            r#"
            local aura = C_UnitAuras.GetAuraDataBySlot("player", 1)
            return aura.name, aura.icon, aura.duration, aura.sourceUnit,
                   aura.spellId, aura.isHelpful, aura.isHarmful,
                   aura.applications, aura.timeMod
            "#,
        )
        .unwrap();
    assert!(!name.is_empty());
    assert!(icon > 0.0);
    assert!(duration > 0.0);
    assert_eq!(source, "player");
    assert!(spell_id > 0.0);
    assert!(is_helpful);
    assert!(!is_harmful);
    assert!(applications >= 0.0);
    assert_eq!(time_mod, 1.0);
}

#[test]
fn debuff_slot_table_reports_harmful() {
    let env = env();
    let (is_helpful, is_harmful, is_raid): (bool, bool, bool) = env
        .eval(
            r#"
            local aura = C_UnitAuras.GetAuraDataBySlot("target", 2)
            return aura.isHelpful, aura.isHarmful, aura.isRaid
            "#,
        )
        .unwrap();
    assert!(!is_helpful);
    assert!(is_harmful);
    // isRaid mirrors isHelpful in build_aura_table.
    assert!(!is_raid);
}

#[test]
fn aura_table_exposes_boolean_flag_shape() {
    let env = env();
    let (
        can_apply,
        boss,
        from_player,
        nameplate_all,
        nameplate_personal,
        nameplate_only,
        stealable,
    ): (bool, bool, bool, bool, bool, bool, bool) = env
        .eval(
            r#"
            local aura = C_UnitAuras.GetAuraDataBySlot("player", 1)
            return aura.canApplyAura, aura.isBossAura, aura.isFromPlayerOrPlayerPet,
                   aura.nameplateShowAll, aura.nameplateShowPersonal,
                   aura.isNameplateOnly, aura.isStealable
            "#,
        )
        .unwrap();
    assert!(can_apply, "canApplyAura default is true");
    assert!(!boss);
    assert!(from_player);
    assert!(!nameplate_all);
    assert!(!nameplate_personal);
    assert!(!nameplate_only);
    assert!(!stealable);
}

#[test]
fn aura_applications_charges_stackcount_are_aliased() {
    let env = env();
    let (a, c, s): (f64, f64, f64) = env
        .eval(
            r#"
            local aura = C_UnitAuras.GetAuraDataBySlot("player", 1)
            return aura.applications, aura.charges, aura.stackCount
            "#,
        )
        .unwrap();
    assert_eq!(a, c, "applications and charges must be equal");
    assert_eq!(c, s, "charges and stackCount must be equal");
}

#[test]
fn aura_points_field_is_an_empty_table() {
    let env = env();
    let (is_table, count): (bool, i64) = env
        .eval(
            r#"
            local aura = C_UnitAuras.GetAuraDataBySlot("player", 1)
            return type(aura.points) == "table", #aura.points
            "#,
        )
        .unwrap();
    assert!(is_table);
    assert_eq!(count, 0);
}
