use super::*;

#[test]
fn damage_meter_seeded_spells_include_unit_details() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");

        let result: (bool, bool, bool, bool, bool, bool, bool) = env
            .eval(
                r#"
                local source = C_DamageMeter.GetCombatSessionSourceFromType(
                    Enum.DamageMeterSessionType.Current,
                    Enum.DamageMeterType.DamageDone,
                    "Player-1-00000001"
                )
                local spell = source and source.combatSpells and source.combatSpells[1]
                local details = spell and spell.combatSpellDetails
                return
                    type(source) == "table",
                    type(source.maxAmount) == "number",
                    type(spell) == "table",
                    type(spell.creatureName) == "string",
                    type(spell.overkillAmount) == "number",
                    type(details) == "table",
                    type(details.unitName) == "string"
                "#,
            )
            .expect("damage meter spell detail probe should run");

        assert_eq!(
            result,
            (true, true, true, true, true, true, true),
            "seeded C_DamageMeter rows must match the required combat spell shape"
        );
    }
}
