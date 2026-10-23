//! Generated-documentation contracts for the three explicitly temporary M1 conditions.
#![cfg(not(feature = "client-wrath"))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_documented_boolean(file: &str, namespace: &str, member: &str) {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        "APIDocumentation = { AddDocumentationTable = function(_, t) M1Documentation = t end }",
    )
    .unwrap();
    let path = wow_ui_sim::client_profile::blizzard_ui_addons_dir()
        .join("Blizzard_APIDocumentationGenerated")
        .join(file);
    let source = std::fs::read_to_string(path).expect("SETUP: active profile documentation");
    env.loader_env().exec(&source).unwrap();
    env.exec(&format!(
        r#"
        local declaration
        for _, entry in ipairs(M1Documentation.Functions) do
            if entry.Name == {member:?} then declaration = entry; break end
        end
        assert(declaration, 'SETUP: generated function declaration')
        assert(not declaration.Arguments or #declaration.Arguments == 0)
        assert(#declaration.Returns == 1)
        assert(declaration.Returns[1].Type == 'bool' and not declaration.Returns[1].Nilable)
        local values = {{_G[{namespace:?}][{member:?}]()}}
        assert(#values == 1 and type(values[1]) == 'boolean', 'documented nonnil boolean')
        assert(_G[{namespace:?}].M1UnknownMember == nil)
        assert(rawget(_G[{namespace:?}], 'M1UnknownMember') == nil)
        "#
    ))
    .unwrap();
}

#[test]
fn cfactory_m1_ranged_slot_documented_boolean() {
    assert_documented_boolean(
        "PaperDollInfoDocumentation.lua",
        "C_PaperDollInfo",
        "IsRangedSlotShown",
    );
}

#[cfg(any(
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary",
    feature = "client-wowforever"
))]
#[test]
fn cfactory_m1_hardcore_documented_boolean() {
    assert_documented_boolean(
        "GameRulesDocumentation.lua",
        "C_GameRules",
        "IsHardcoreActive",
    );
}

#[cfg(feature = "client-wowforever")]
#[test]
fn cfactory_m1_daytime_documented_boolean() {
    assert_documented_boolean("DateAndTimeDocumentation.lua", "C_DateAndTime", "IsDayTime");
}
