mod common;

use wow_ui_sim::lua_api::WowLuaEnv;

fn clear_dirty(env: &WowLuaEnv) {
    let _ = env.state().borrow().widgets.take_render_dirty_with_ids();
    env.state().borrow_mut().widgets.drain_rect_dirty();
}

fn assert_no_visual_dirty(env: &WowLuaEnv, context: &str) {
    let (dirty_mask, dirty_ids) = env.state().borrow().widgets.take_render_dirty_with_ids();
    let dirty_ids = dirty_ids.unwrap_or_default();
    assert_eq!(
        dirty_mask, 0,
        "{context}: visual dirty mask should stay zero"
    );
    assert!(
        dirty_ids.is_empty(),
        "{context}: visual dirty ids should stay empty, got {:?}",
        dirty_ids
    );
}

fn assert_no_rect_dirty(env: &WowLuaEnv, context: &str) {
    let dirty_ids = env.state().borrow_mut().widgets.drain_rect_dirty();
    assert!(
        dirty_ids.is_empty(),
        "{context}: rect dirty ids should stay empty, got {:?}",
        dirty_ids
    );
}

#[test]
fn set_alpha_same_value_is_a_true_noop() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "NoopAlphaFrame", UIParent)
            local child = CreateFrame("Frame", "NoopAlphaChild", frame)
            frame:SetAlpha(1)
            "#,
        )
        .expect("initial alpha setup should succeed");

        clear_dirty(&env);

        env.exec(r#"NoopAlphaFrame:SetAlpha(1)"#)
            .expect("same-value SetAlpha should succeed");

        assert_no_visual_dirty(&env, "SetAlpha(1) on alpha=1");
        assert_no_rect_dirty(&env, "SetAlpha(1) on alpha=1");
    }
}

#[test]
fn set_alpha_clamped_same_value_is_a_true_noop() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "NoopAlphaClampedFrame", UIParent)
            frame:SetAlpha(1)
            "#,
        )
        .expect("initial clamped alpha setup should succeed");

        clear_dirty(&env);

        env.exec(r#"NoopAlphaClampedFrame:SetAlpha(2)"#)
            .expect("clamped SetAlpha should succeed");

        assert_no_visual_dirty(&env, "SetAlpha(2) on alpha=1");
        assert_no_rect_dirty(&env, "SetAlpha(2) on alpha=1");
    }
}

#[test]
fn set_text_same_value_is_a_true_noop() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "NoopTextFrame", UIParent)
            local text = frame:CreateFontString("NoopTextFontString", "ARTWORK")
            text:SetWidth(120)
            text:SetHeight(20)
            text:SetText("Hello")
            "#,
        )
        .expect("initial text setup should succeed");

        clear_dirty(&env);

        env.exec(r#"NoopTextFontString:SetText("Hello")"#)
            .expect("same-value SetText should succeed");

        assert_no_visual_dirty(&env, "SetText(\"Hello\") on identical text");
        assert_no_rect_dirty(&env, "SetText(\"Hello\") on identical text");
    }
}

#[test]
fn set_formatted_text_same_value_is_a_true_noop() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "NoopFormattedFrame", UIParent)
            local text = frame:CreateFontString("NoopFormattedFontString", "ARTWORK")
            text:SetWidth(120)
            text:SetHeight(20)
            text:SetFormattedText("%dm", 60)
            "#,
        )
        .expect("initial formatted text setup should succeed");

        clear_dirty(&env);

        env.exec(r#"NoopFormattedFontString:SetFormattedText("%dm", 60)"#)
            .expect("same-value SetFormattedText should succeed");

        assert_no_visual_dirty(&env, "SetFormattedText(%dm, 60) on identical output");
        assert_no_rect_dirty(&env, "SetFormattedText(%dm, 60) on identical output");
    }
}

#[test]
fn set_point_same_value_is_a_true_noop() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "NoopPointFrame", UIParent)
            frame:SetPoint("CENTER", UIParent, "CENTER", 10, 20)
            "#,
        )
        .expect("initial point setup should succeed");

        clear_dirty(&env);

        env.exec(r#"NoopPointFrame:SetPoint("CENTER", UIParent, "CENTER", 10, 20)"#)
            .expect("same-value SetPoint should succeed");

        assert_no_visual_dirty(&env, "SetPoint on identical anchor");
        assert_no_rect_dirty(&env, "SetPoint on identical anchor");
    }
}

#[test]
fn set_font_object_same_value_is_a_true_noop() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "NoopFontObjectFrame", UIParent)
            local text = frame:CreateFontString("NoopFontObjectString", "ARTWORK")
            text:SetFontObject("GameFontNormalMed1")
            "#,
        )
        .expect("initial font object setup should succeed");

        clear_dirty(&env);

        env.exec(r#"NoopFontObjectString:SetFontObject("GameFontNormalMed1")"#)
            .expect("same-value SetFontObject should succeed");

        assert_no_visual_dirty(&env, "SetFontObject on identical font snapshot");
        assert_no_rect_dirty(&env, "SetFontObject on identical font snapshot");
    }
}

#[test]
fn set_font_object_same_table_reference_is_a_true_noop() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "NoopFontObjectTableFrame", UIParent)
            local text = frame:CreateFontString("NoopFontObjectTableString", "ARTWORK")
            local font_obj = GameFontNormalMed1
            text:SetFontObject(font_obj)
            _G.__noop_font_object_table_ref = font_obj
            "#,
        )
        .expect("initial font object table setup should succeed");

        clear_dirty(&env);

        env.exec(r#"NoopFontObjectTableString:SetFontObject(__noop_font_object_table_ref)"#)
            .expect("same-table SetFontObject should succeed");

        assert_no_visual_dirty(&env, "SetFontObject on identical font table ref");
        assert_no_rect_dirty(&env, "SetFontObject on identical font table ref");
    }
}

#[test]
fn set_shown_same_value_is_a_true_noop() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "NoopShownFrame", UIParent)
            frame:Hide()
            frame:Show()
            "#,
        )
        .expect("initial shown-state setup should succeed");

        clear_dirty(&env);

        env.exec(r#"NoopShownFrame:SetShown(true)"#)
            .expect("same-value SetShown should succeed");

        assert_no_visual_dirty(&env, "SetShown(true) on shown frame");
        assert_no_rect_dirty(&env, "SetShown(true) on shown frame");
    }
}

#[test]
fn set_vertex_color_same_value_is_a_true_noop() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "NoopVertexColorFrame", UIParent)
            local texture = frame:CreateTexture("NoopVertexColorTexture", "ARTWORK")
            texture:SetVertexColor(0.5, 0.6, 0.7, 0.8)
            "#,
        )
        .expect("initial vertex color setup should succeed");

        clear_dirty(&env);

        env.exec(r#"NoopVertexColorTexture:SetVertexColor(0.5, 0.6, 0.7, 0.8)"#)
            .expect("same-value SetVertexColor should succeed");

        assert_no_visual_dirty(&env, "SetVertexColor on identical rgba");
        assert_no_rect_dirty(&env, "SetVertexColor on identical rgba");
    }
}
