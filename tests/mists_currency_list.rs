#![cfg(feature = "client-mists")]

use wow_ui_sim::lua_api::WowLuaEnv;

const TOKEN_UI_CATA_LUA: &str =
    include_str!("../Interface/BlizzardUI/Mists/AddOns/Blizzard_TokenUI/Cata/Blizzard_TokenUI.lua");

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

    env.exec(TOKEN_UI_CATA_LUA)
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
