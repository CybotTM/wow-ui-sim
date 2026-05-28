#![cfg(feature = "client-mists")]

use wow_ui_sim::lua_api::WowLuaEnv;

fn token_ui_cata_lua() -> String {
    std::fs::read_to_string(
        wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
            "CARGO_MANIFEST_DIR"
        )))
        .join("Blizzard_TokenUI/Cata/Blizzard_TokenUI.lua"),
    )
    .expect("Mists TokenUI Lua should be available in the profile UI source")
}

#[test]
fn token_frame_update_reproduces_missing_currency_list_size() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    env.exec(
        r#"
        rawset(_G, "GetCurrencyListSize", nil)
        UIPanelWindows = {}
        CharacterFrameTab4 = {
            Hide = function() end,
            Show = function() end,
        }
        TokenFrameContainer = {}
        "#,
    )
    .expect("install TokenFrame reproduction fixtures");

    let source = token_ui_cata_lua();
    env.exec(&source)
        .expect("Mists Cata TokenUI Lua should define TokenFrame helpers");

    let (ok, err): (bool, String) = env
        .eval(
            r#"
            local ok, err = pcall(TokenFrame_Update)
            return ok, tostring(err)
            "#,
        )
        .expect("TokenFrame_Update pcall should return a status");

    assert!(!ok, "TokenFrame_Update should reproduce the nil global");
    assert!(
        err.contains("GetCurrencyListSize"),
        "expected GetCurrencyListSize nil failure, got: {err}"
    );
}

#[test]
fn legacy_currency_list_size_wraps_c_currency_info() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    env.exec(
        r#"
        UIPanelWindows = {}
        local tab_visible = nil
        CharacterFrameTab4 = {
            Hide = function() tab_visible = false end,
            Show = function() tab_visible = true end,
        }
        TokenFrameContainer = {}
        "#,
    )
    .expect("install TokenFrame compatibility fixtures");

    let source = token_ui_cata_lua();
    env.exec(&source)
        .expect("Mists Cata TokenUI Lua should define TokenFrame helpers");

    let (legacy_size, namespaced_size, update_ok, err): (i32, i32, bool, String) = env
        .eval(
            r#"
            local legacySize = GetCurrencyListSize()
            local namespacedSize = C_CurrencyInfo.GetCurrencyListSize()
            local ok, err = pcall(TokenFrame_Update)
            return legacySize, namespacedSize, ok, tostring(err)
            "#,
        )
        .expect("legacy currency wrapper should support TokenFrame_Update");

    assert_eq!(
        legacy_size, namespaced_size,
        "legacy GetCurrencyListSize should delegate to C_CurrencyInfo.GetCurrencyListSize"
    );
    assert!(
        update_ok,
        "TokenFrame_Update should use the legacy compatibility wrapper: {err}"
    );
}
