use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn hidden_cursor_anchored_tooltip_does_not_dirty_on_mouse_move() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "HiddenCursorTooltipOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_CURSOR")
        GameTooltip:Hide()
        "#,
    )
    .unwrap();

    drain_render_dirty(&env);
    env.state()
        .borrow_mut()
        .set_mouse_position(Some((200.0, 240.0)));

    assert_no_render_dirty(&env, "hidden cursor tooltip");
}

#[test]
fn unchanged_cursor_tooltip_anchor_does_not_dirty_on_mouse_move() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "VisibleCursorTooltipOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_CURSOR")
        "#,
    )
    .unwrap();

    env.state()
        .borrow_mut()
        .set_mouse_position(Some((200.0, 240.0)));
    drain_render_dirty(&env);
    env.state()
        .borrow_mut()
        .set_mouse_position(Some((200.0, 240.0)));

    assert_no_render_dirty(&env, "unchanged cursor tooltip anchor");
}

fn drain_render_dirty(env: &WowLuaEnv) {
    let state = env.state().borrow();
    let _ = state.widgets.take_render_dirty_with_ids();
}

fn assert_no_render_dirty(env: &WowLuaEnv, label: &str) {
    let (dirty_mask, dirty_ids) = {
        let state = env.state().borrow();
        state.widgets.take_render_dirty_with_ids()
    };

    assert_eq!(dirty_mask, 0, "{label} should not dirty strata");
    assert!(
        dirty_ids.is_some_and(|ids| ids.is_empty()),
        "{label} should not dirty frame IDs"
    );
}
