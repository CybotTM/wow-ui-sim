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

#[test]
fn adding_visible_tooltip_line_dirties_tooltip_rect() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "VisibleTooltipLineOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_RIGHT")
        "#,
    )
    .unwrap();

    drain_render_dirty(&env);
    env.exec(r#"GameTooltip:AddLine("line changes tooltip size")"#)
        .unwrap();

    assert_render_dirty_contains(&env, "GameTooltip", "GameTooltip:AddLine");
}

#[test]
fn inventory_tooltip_population_dirties_tooltip_rect() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "InventoryTooltipOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_RIGHT")
        "#,
    )
    .unwrap();

    drain_render_dirty(&env);
    let has_item: bool = env
        .eval(r#"return GameTooltip:SetInventoryItem("player", 11)"#)
        .unwrap();

    assert!(has_item, "test inventory slot should populate tooltip lines");
    assert_render_dirty_contains(&env, "GameTooltip", "GameTooltip:SetInventoryItem");
}

fn drain_render_dirty(env: &WowLuaEnv) {
    let state = env.state().borrow();
    let _ = state.widgets.take_render_dirty_with_ids();
}

fn assert_render_dirty_contains(env: &WowLuaEnv, frame_name: &str, label: &str) {
    let frame_id = {
        let state = env.state().borrow();
        state
            .widgets
            .get_id_by_name(frame_name)
            .unwrap_or_else(|| panic!("{frame_name} should exist"))
    };
    let (dirty_mask, dirty_ids) = {
        let state = env.state().borrow();
        state.widgets.take_render_dirty_with_ids()
    };

    assert_ne!(dirty_mask, 0, "{label} should dirty strata");
    assert!(
        dirty_ids.is_some_and(|ids| ids.contains(&frame_id)),
        "{label} should mark {frame_name} dirty"
    );
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
