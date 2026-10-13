//! Positive spell-catalog contract; unknown spell behavior is not asserted.
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn zone_ability_icon_returns_catalog_file_id_for_known_spell() {
    const FIREBALL_SPELL_ID: u32 = 133;
    let spell = wow_ui_sim::spells::get_spell(FIREBALL_SPELL_ID)
        .expect("concrete Fireball fixture exists in spell catalog");
    let icon = spell.icon_file_data_id;
    let env = WowLuaEnv::new().unwrap();
    env.exec(&format!(
        r#"
        local icon = C_ZoneAbility.GetZoneAbilityIcon({FIREBALL_SPELL_ID})
        assert(type(icon) == 'number', 'zone ability icon must be numeric file ID')
        assert(icon == {icon}, 'zone ability icon must match supplied spell catalog')
        assert(select('#', C_ZoneAbility.GetZoneAbilityIcon({FIREBALL_SPELL_ID})) == 1)
        "#,
    ))
    .expect("known spell returns exactly one numeric catalog icon");
}
