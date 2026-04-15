use crate::lua_api::WowLuaEnv;

#[test]
fn enable_mouse_wheel_updates_frame_state() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");

    let initially_disabled: bool = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "MouseWheelStateFrame", UIParent)
            return frame:IsMouseWheelEnabled()
            "#,
        )
        .expect("initial mouse wheel query should succeed");
    assert!(
        !initially_disabled,
        "frames should start with mouse wheel disabled"
    );

    let enabled: bool = env
        .eval(
            r#"
            local frame = MouseWheelStateFrame
            frame:EnableMouseWheel(true)
            return frame:IsMouseWheelEnabled()
            "#,
        )
        .expect("mouse wheel enable should succeed");
    assert!(
        enabled,
        "EnableMouseWheel(true) should persist on the frame"
    );

    let disabled_again: bool = env
        .eval(
            r#"
            local frame = MouseWheelStateFrame
            frame:EnableMouseWheel(false)
            return frame:IsMouseWheelEnabled()
            "#,
        )
        .expect("mouse wheel disable should succeed");
    assert!(
        !disabled_again,
        "EnableMouseWheel(false) should clear the frame state"
    );
}

#[test]
fn set_frame_level_same_value_preserves_strata_buckets() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");

    env.exec(
        r#"
        local frame = CreateFrame("Frame", "SameLevelFrame", UIParent)
        "#,
    )
    .expect("frame creation should succeed");

    {
        let mut state = env.state().borrow_mut();
        let _ = state.get_strata_buckets();
        assert!(
            state.strata_buckets.is_some(),
            "building buckets should populate the cache"
        );
    }

    env.exec(
        r#"
        SameLevelFrame:SetFrameLevel(SameLevelFrame:GetFrameLevel())
        "#,
    )
    .expect("setting the same frame level should succeed");

    assert!(
        env.state().borrow().strata_buckets.is_some(),
        "no-op SetFrameLevel should not invalidate cached strata buckets",
    );
}

#[test]
fn set_scale_same_value_does_not_mark_rect_dirty() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "ScaleNoopFrame", UIParent)
        frame:SetScale(0.75)
        "#,
    )
    .expect("frame creation and initial SetScale should succeed");

    let id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("ScaleNoopFrame")
        .expect("frame should exist");
    env.state().borrow_mut().widgets.drain_rect_dirty();

    env.exec(r#"ScaleNoopFrame:SetScale(0.75)"#)
        .expect("repeat SetScale should succeed");

    let dirty_ids = env.state().borrow_mut().widgets.drain_rect_dirty();
    assert!(
        !dirty_ids.contains(&id),
        "no-op SetScale should not mark the frame rect dirty (got {:?})",
        dirty_ids,
    );
}

#[test]
fn set_scale_different_value_marks_rect_dirty() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "ScaleChangeFrame", UIParent)
        frame:SetScale(1.0)
        "#,
    )
    .expect("frame creation and initial SetScale should succeed");

    let id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("ScaleChangeFrame")
        .expect("frame should exist");
    env.state().borrow_mut().widgets.drain_rect_dirty();

    env.exec(r#"ScaleChangeFrame:SetScale(2.0)"#)
        .expect("SetScale with new value should succeed");

    let dirty_ids = env.state().borrow_mut().widgets.drain_rect_dirty();
    assert!(
        dirty_ids.contains(&id),
        "SetScale with a different value should mark the frame rect dirty",
    );
}

#[test]
fn set_scale_rejects_non_positive_even_on_noop() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "ScaleValidationFrame", UIParent)
        frame:SetScale(1.0)
        "#,
    )
    .unwrap();

    let result = env.exec(r#"ScaleValidationFrame:SetScale(-1)"#);
    assert!(
        result.is_err(),
        "SetScale(-1) must error, even when the stored scale would be different",
    );
}

#[test]
fn set_alpha_from_boolean_same_value_does_not_propagate() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "AlphaBoolNoopFrame", UIParent)
        local child = CreateFrame("Frame", "AlphaBoolChild", frame)
        frame:SetAlphaFromBoolean(true)
        "#,
    )
    .unwrap();

    let (id, child_id) = {
        let state = env.state().borrow();
        (
            state.widgets.get_id_by_name("AlphaBoolNoopFrame").unwrap(),
            state.widgets.get_id_by_name("AlphaBoolChild").unwrap(),
        )
    };
    env.state()
        .borrow_mut()
        .widgets
        .take_render_dirty_with_ids();

    env.exec(r#"AlphaBoolNoopFrame:SetAlphaFromBoolean(true)"#)
        .unwrap();

    let (_, dirty_ids) = env.state().borrow().widgets.take_render_dirty_with_ids();
    let dirty_ids = dirty_ids.unwrap_or_default();
    assert!(
        !dirty_ids.contains(&id) && !dirty_ids.contains(&child_id),
        "no-op SetAlphaFromBoolean should not mark parent or child visually dirty",
    );
}

#[test]
fn set_ignore_parent_scale_same_value_does_not_mark_rect_dirty() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "IgnoreScaleNoopFrame", UIParent)
        frame:SetIgnoreParentScale(true)
        "#,
    )
    .unwrap();

    let id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("IgnoreScaleNoopFrame")
        .unwrap();
    env.state().borrow_mut().widgets.drain_rect_dirty();

    env.exec(r#"IgnoreScaleNoopFrame:SetIgnoreParentScale(true)"#)
        .unwrap();

    let dirty_ids = env.state().borrow_mut().widgets.drain_rect_dirty();
    assert!(
        !dirty_ids.contains(&id),
        "no-op SetIgnoreParentScale should not mark the frame rect dirty (got {:?})",
        dirty_ids,
    );
}
