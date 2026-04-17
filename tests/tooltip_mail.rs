use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn c_tooltip_info_mail_getters_reuse_item_tooltips() {
    let env = env();
    env.exec(
        r#"
        A_Admin.AddMail("AH", "Won", "", 0, {{ item_id = 6948, count = 1 }})
        "#,
    )
    .unwrap();
    {
        let mut st = env.state().borrow_mut();
        st.player.send_mail_items[0] = st.player.inbox[0].items.first().cloned();
    }

    let result: String = env
        .eval(
            r#"
            local baseline = C_TooltipInfo.GetItemByID(6948)
            local inboxTip = C_TooltipInfo.GetInboxItem(1, 1)
            local sendMailTip = C_TooltipInfo.GetSendMailItem(1)

            if inboxTip.lines[1].leftText ~= baseline.lines[1].leftText then
                return "inbox_item_should_match_item_tooltip"
            end
            if sendMailTip.lines[1].leftText ~= baseline.lines[1].leftText then
                return "send_mail_item_should_match_item_tooltip"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "Mail tooltip getters should reuse the normal item tooltip path"
    );
}

#[test]
fn game_tooltip_mail_item_setters_populate_lines() {
    let env = env();
    env.exec(
        r#"
        A_Admin.AddMail("AH", "Won", "", 0, {{ item_id = 6948, count = 1 }})
        "#,
    )
    .unwrap();
    {
        let mut st = env.state().borrow_mut();
        st.player.send_mail_items[0] = st.player.inbox[0].items.first().cloned();
    }

    env.exec(
        r#"
        GameTooltip:SetOwner(UIParent, "ANCHOR_NONE")
        GameTooltip:SetInboxItem(1, 1)
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
        GameTooltip:SetSendMailItem(1)
        "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let td = state.tooltips.get(&tooltip_id).expect("No tooltip data");
    assert_eq!(td.lines[0].left_text, "Hearthstone");
}
