use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn c_tooltip_info_merchant_item_reuses_item_tooltips() {
    let env = env();
    env.state().borrow_mut().merchant_items = vec![6948, 229181];

    let result: String = env
        .eval(
            r#"
            local baselineOne = C_TooltipInfo.GetItemByID(6948)
            local baselineTwo = C_TooltipInfo.GetItemByID(229181)
            local merchantOne = C_TooltipInfo.GetMerchantItem(1)
            local merchantTwo = C_TooltipInfo.GetMerchantItem(2)

            if merchantOne.lines[1].leftText ~= baselineOne.lines[1].leftText then
                return "merchant_item_one_should_match_item_by_id"
            end
            if merchantTwo.lines[1].leftText ~= baselineTwo.lines[1].leftText then
                return "merchant_item_two_should_match_item_by_id"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "Merchant tooltip getter should reuse the normal item tooltip path"
    );
}
