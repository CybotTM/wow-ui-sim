#![cfg(feature = "client-mists")]

use wow_ui_sim::lua_api::WowLuaEnv;

const PRODUCT_CHOICE_LUA: &str = include_str!(
    "../Interface/BlizzardUI/Mists/AddOns/Blizzard_UIPanels_Game/Classic/ProductChoice.lua"
);

#[test]
fn mists_product_choice_alerts_reproduce_nil_choices_length() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");
    env.exec(
        r#"
        C_ProductChoice = {
            GetChoices = function() return nil end,
        }
        "#,
    )
    .expect("install nil ProductChoice choices fixture");
    env.exec(PRODUCT_CHOICE_LUA)
        .expect("ProductChoice.lua should define functions before events fire");

    let (product_choice_type, get_choices_type, choices_type, ok, err): (
        String,
        String,
        String,
        bool,
        String,
    ) = env
        .eval(
            r#"
            local choices = C_ProductChoice.GetChoices()
            local ok, err = pcall(ProductChoiceFrame_ShowAlerts, {})
            return type(C_ProductChoice), type(C_ProductChoice.GetChoices), type(choices), ok, tostring(err)
            "#,
        )
        .expect("ProductChoiceFrame_ShowAlerts pcall should return a status");

    assert_eq!(product_choice_type, "table");
    assert_eq!(get_choices_type, "function");
    assert_eq!(
        choices_type, "nil",
        "the missing product-choice table is the C_ProductChoice.GetChoices() return value"
    );
    assert!(
        !ok,
        "nil ProductChoice choices should fail under length operator"
    );
    assert!(
        err.contains("length") || err.contains("nil"),
        "expected nil choices length failure, got: {err}"
    );
}

#[test]
fn mists_product_choice_empty_choices_are_available_data() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");
    env.exec(PRODUCT_CHOICE_LUA)
        .expect("ProductChoice.lua should define functions before events fire");

    let (choices_type, choices_count, ok, err): (String, i32, bool, String) = env
        .eval(
            r#"
            local choices = C_ProductChoice.GetChoices()
            local frame = {}
            local ok, err = pcall(ProductChoiceFrame_ShowAlerts, frame)
            return type(choices), #choices, ok, tostring(err)
            "#,
        )
        .expect("empty ProductChoice data should be safe for alert checks");

    assert_eq!(
        (choices_type, choices_count, ok),
        ("table".to_string(), 0, true),
        "Mists ProductChoice should expose no available choices as an empty table"
    );
    assert_eq!(err, "nil");
}

#[test]
fn mists_product_choice_api_defaults_expose_empty_tables() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let (choices_type, choices_count, products_type, products_count, suppressed): (
        String,
        i32,
        String,
        i32,
        i32,
    ) = env
        .eval(
            r#"
            local choices = C_ProductChoice.GetChoices()
            local products = C_ProductChoice.GetProducts(123)
            return type(choices), #choices, type(products), #products, C_ProductChoice.GetNumSuppressed()
            "#,
        )
        .expect("Mists ProductChoice API defaults should be callable");

    assert_eq!(
        (
            choices_type,
            choices_count,
            products_type,
            products_count,
            suppressed
        ),
        ("table".to_string(), 0, "table".to_string(), 0, 0),
        "Mists ProductChoice should expose empty data through the full API surface"
    );
}
