use wow_ui_sim::lua_api::WowLuaEnv;

const WOWTOKEN_SECURE_SCRIPT: &str = r#"
    local events = {}
    local listener = CreateFrame("Frame")
    listener:SetScript("OnEvent", function(_, event)
        table.insert(events, event)
    end)

    for _, event in ipairs({
        "TOKEN_REDEEM_BALANCE_UPDATED",
        "TOKEN_REDEEM_GAME_TIME_UPDATED",
        "TOKEN_STATUS_CHANGED",
    }) do
        listener:RegisterEvent(event)
    end

    local tokenCount = C_WowTokenSecure.GetTokenCount()
    if tokenCount ~= 2 then
        return "wrong_initial_token_count:" .. tostring(tokenCount)
    end

    local currentBalance, addedBalance, canRedeem, cannotRedeemReason = C_WowTokenSecure.GetBalanceRedemptionInfo()
    if currentBalance ~= 2500 or addedBalance ~= 1500 or canRedeem ~= true or cannotRedeemReason ~= 0 then
        return "wrong_initial_balance_info:"
            .. tostring(currentBalance) .. ":"
            .. tostring(addedBalance) .. ":"
            .. tostring(canRedeem) .. ":"
            .. tostring(cannotRedeemReason)
    end

    if C_WowTokenSecure.GetBalanceRedeemAmount() ~= 1500 then
        return "wrong_initial_redeem_amount"
    end

    if C_WowTokenSecure.GetPriceLockDuration() ~= 900 then
        return "wrong_price_lock_duration"
    end

    if C_WowTokenSecure.WillKickFromWorld() ~= false then
        return "wrong_kick_flag"
    end

    local canRedeemResult = C_WowTokenSecure.CanRedeemForBalance()
    if canRedeemResult ~= Enum.CanRedeemTokenForBalanceResult.Ok then
        return "wrong_can_redeem_result:" .. tostring(canRedeemResult)
    end

    if events[1] ~= "TOKEN_REDEEM_BALANCE_UPDATED" then
        return "missing_balance_update_event"
    end

    local remainingBefore = select(2, C_WowTokenSecure.GetGameTimeRedemptionInfo())
    if remainingBefore ~= 1440 then
        return "wrong_initial_remaining_game_time:" .. tostring(remainingBefore)
    end

    local refreshedRemaining = C_WowTokenSecure.GetRemainingGameTime()
    if refreshedRemaining ~= 1440 or events[2] ~= "TOKEN_REDEEM_GAME_TIME_UPDATED" then
        return "wrong_game_time_refresh"
    end

    C_WowTokenSecure.SetBalanceAmountString("$20.00")
    if C_WowTokenSecure.GetBalanceRedeemAmount() ~= 2000 then
        return "balance_amount_string_not_applied"
    end

    if C_WowTokenSecure.RedeemToken(LE_TOKEN_REDEEM_TYPE_BALANCE) ~= true then
        return "balance_redeem_not_started"
    end

    if C_WowTokenSecure.IsRedemptionStillValid() ~= true then
        return "balance_redemption_not_valid"
    end

    if C_WowTokenSecure.RedeemTokenConfirm(LE_TOKEN_REDEEM_TYPE_BALANCE) ~= true then
        return "balance_redeem_confirm_failed"
    end

    local balanceAfterRedeem, _, _, _ = C_WowTokenSecure.GetBalanceRedemptionInfo()
    if balanceAfterRedeem ~= 4500 then
        return "wrong_balance_after_redeem:" .. tostring(balanceAfterRedeem)
    end

    if C_WowTokenSecure.GetTokenCount() ~= 1 then
        return "wrong_token_count_after_balance_redeem"
    end

    if events[3] ~= "TOKEN_STATUS_CHANGED" or events[4] ~= "TOKEN_REDEEM_BALANCE_UPDATED" then
        return "wrong_balance_confirm_events"
    end

    if C_WowTokenSecure.RedeemToken(LE_TOKEN_REDEEM_TYPE_GAME_TIME) ~= true then
        return "game_time_redeem_not_started"
    end

    C_WowTokenSecure.CancelRedeem()
    if C_WowTokenSecure.IsRedemptionStillValid() ~= false then
        return "cancel_redeem_not_cleared"
    end

    if C_WowTokenSecure.RedeemToken(LE_TOKEN_REDEEM_TYPE_GAME_TIME) ~= true then
        return "game_time_redeem_restart_failed"
    end

    if C_WowTokenSecure.RedeemTokenConfirm(LE_TOKEN_REDEEM_TYPE_GAME_TIME) ~= true then
        return "game_time_redeem_confirm_failed"
    end

    local isSub, remainingAfter = C_WowTokenSecure.GetGameTimeRedemptionInfo()
    if isSub ~= true or remainingAfter ~= 44640 then
        return "wrong_game_time_after_redeem:" .. tostring(isSub) .. ":" .. tostring(remainingAfter)
    end

    if C_WowTokenSecure.GetTokenCount() ~= 0 then
        return "wrong_token_count_after_game_time_redeem"
    end

    if events[5] ~= "TOKEN_STATUS_CHANGED" or events[6] ~= "TOKEN_REDEEM_GAME_TIME_UPDATED" then
        return "wrong_game_time_confirm_events"
    end

    if C_WowTokenSecure.ConfirmBuyToken(true) ~= true then
        return "confirm_buy_true_failed"
    end

    if C_WowTokenSecure.GetTokenCount() ~= 1 then
        return "confirm_buy_did_not_add_token"
    end

    if C_WowTokenSecure.ConfirmSellToken(true) ~= true then
        return "confirm_sell_true_failed"
    end

    if C_WowTokenSecure.GetTokenCount() ~= 0 then
        return "confirm_sell_did_not_remove_token"
    end

    if C_WowTokenSecure.ConfirmBuyToken(false) ~= false then
        return "confirm_buy_false_wrong_result"
    end

    if C_WowTokenSecure.ConfirmSellToken(false) ~= false then
        return "confirm_sell_false_wrong_result"
    end

    if events[7] ~= "TOKEN_STATUS_CHANGED" or events[8] ~= "TOKEN_STATUS_CHANGED" then
        return "wrong_confirm_events"
    end

    return "ok"
"#;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn wowtoken_secure_methods_track_redemption_and_token_state() {
    let env = env();
    let result: String = env
        .eval(WOWTOKEN_SECURE_SCRIPT)
        .expect("seeded C_WowTokenSecure flow should be queryable");

    assert_eq!(result, "ok");
}
