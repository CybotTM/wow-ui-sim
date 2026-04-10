use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn trade_info_warning_query_exists_and_defaults_false() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if type(C_TradeInfo) ~= "table" then
                return "missing_trade_info_namespace"
            end
            if type(C_TradeInfo.ShouldShowTradeOfferWarning) ~= "function" then
                return "missing_should_show_trade_offer_warning"
            end
            if C_TradeInfo.ShouldShowTradeOfferWarning() ~= false then
                return "trade_offer_warning_should_default_false"
            end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "C_TradeInfo.ShouldShowTradeOfferWarning should exist and default to false"
    );
}
