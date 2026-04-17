use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::TradeState;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn c_tooltip_info_trade_getters_delegate_to_item_tooltips() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.active_trade = Some(TradeState {
            target: "Jaina".into(),
            player_slots: [6948, 0, 0, 0, 0, 0, 0],
            target_slots: [6948, 0, 0, 0, 0, 0, 0],
            ..TradeState::default()
        });
    }

    let result: String = env
        .eval(
            r#"
            local baseline = C_TooltipInfo.GetItemByID(6948)
            local playerTip = C_TooltipInfo.GetTradePlayerItem(1)
            local targetTip = C_TooltipInfo.GetTradeTargetItem(1)

            if playerTip.lines[1].leftText ~= baseline.lines[1].leftText then
                return "player_trade_item_should_match_item_tooltip"
            end
            if targetTip.lines[1].leftText ~= baseline.lines[1].leftText then
                return "target_trade_item_should_match_item_tooltip"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "Trade tooltip getters should reuse the normal item tooltip path"
    );
}

#[test]
fn game_tooltip_trade_item_setters_populate_lines() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.active_trade = Some(TradeState {
            target: "Jaina".into(),
            player_slots: [6948, 0, 0, 0, 0, 0, 0],
            target_slots: [6948, 0, 0, 0, 0, 0, 0],
            ..TradeState::default()
        });
    }

    env.exec(
        r#"
        GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
        GameTooltip:SetTradePlayerItem(1)
        "#,
    )
    .unwrap();

    let tooltip_id = {
        let state = env.state().borrow();
        state
            .widgets
            .get_id_by_name("GameTooltip")
            .expect("GameTooltip not found")
    };
    {
        let state = env.state().borrow();
        let td = state.tooltips.get(&tooltip_id).expect("No tooltip data");
        assert_eq!(td.lines[0].left_text, "Hearthstone");
    }

    env.exec(
        r#"
        GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
        GameTooltip:SetTradeTargetItem(1)
        "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let td = state.tooltips.get(&tooltip_id).expect("No tooltip data");
    assert_eq!(td.lines[0].left_text, "Hearthstone");
}
