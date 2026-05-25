use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("failed to create Lua environment")
}

#[test]
fn c_api_reorg_keeps_core_namespaces_registered() {
    let env = env();
    let namespaces: (
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            return type(C_AddOns) == "table",
                   type(C_Texture) == "table",
                   type(C_XMLUtil) == "table",
                   type(C_Item) == "table",
                   type(C_CurrencyInfo) == "table",
                   type(C_Container) == "table",
                   type(C_ItemUpgrade) == "table",
                   type(C_Spell) == "table",
                   type(C_SpellBook) == "table",
                   type(C_ModelInfo) == "table",
                   type(C_LFGInfo) == "table",
                   type(C_WowTokenSecure) == "table",
                   type(C_FogOfWar) == "table"
        "#,
        )
        .expect("failed to probe C_* namespace registration");

    assert!(namespaces.0, "C_AddOns should stay registered");
    assert!(namespaces.1, "C_Texture should stay registered");
    assert!(namespaces.2, "C_XMLUtil should stay registered");
    assert!(namespaces.3, "C_Item should stay registered");
    assert!(namespaces.4, "C_CurrencyInfo should stay registered");
    assert!(namespaces.5, "C_Container should stay registered");
    assert!(namespaces.6, "C_ItemUpgrade should stay registered");
    assert!(namespaces.7, "C_Spell should stay registered");
    assert!(namespaces.8, "C_SpellBook should stay registered");
    assert!(namespaces.9, "C_ModelInfo should stay registered");
    assert!(namespaces.10, "C_LFGInfo should stay registered");
    assert!(namespaces.11, "C_WowTokenSecure should stay registered");
    assert!(namespaces.12, "C_FogOfWar should stay registered");
}

#[test]
fn c_fog_of_war_unknown_id_keeps_default_shape() {
    let env = env();
    let info: (Option<String>, Option<String>, f64) = env
        .eval(
            r#"
            local info = C_FogOfWar.GetFogOfWarInfo(-1)
            return info.backgroundAtlas, info.maskAtlas, info.maskScalar
        "#,
        )
        .expect("failed to query C_FogOfWar");

    assert_eq!(info.0, None);
    assert_eq!(info.1, None);
    assert_eq!(info.2, 1.0);
}

#[test]
fn c_addons_scripts_disallowed_for_beta_defaults_false() {
    let env = env();
    let result: (String, bool) = env
        .eval(
            r#"
            return type(C_AddOns.GetScriptsDisallowedForBeta),
                   C_AddOns.GetScriptsDisallowedForBeta()
        "#,
        )
        .expect("failed to query C_AddOns.GetScriptsDisallowedForBeta");

    assert_eq!(result, ("function".to_string(), false));
}

#[test]
fn configuration_warnings_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_configuration_warnings"),
        "unmodeled C_ConfigurationWarnings defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_configuration_warnings"),
        "C_ConfigurationWarnings should not be wired through c_api registration"
    );
}

#[test]
fn item_targeting_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let item_spell = include_str!("../src/c_api/item_spell/mod.rs");

    assert!(
        !temporary_shims.contains("c_item_targeting"),
        "unmodeled C_Item targeting defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !item_spell.contains("c_item_targeting"),
        "C_Item IsHelpfulItem/IsHarmfulItem defaults should not be wired through c_api item registration"
    );
}

#[test]
fn spell_target_defaults_are_not_c_api_temporary_shims() {
    let temporary_shims = include_str!("../src/c_api/temporary_shims/mod.rs");
    let registration = include_str!("../src/c_api/registration.rs");

    assert!(
        !temporary_shims.contains("c_spell_target"),
        "unmodeled C_Spell target-spell metadata defaults belong in lua_api::workarounds::temporary"
    );
    assert!(
        !registration.contains("c_spell_target"),
        "C_Spell TargetSpell* defaults should not be wired through c_api registration"
    );
}
