use crate::common;

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

fn frame_id_by_name(env: &WowLuaEnv, name: &str) -> u64 {
    env.state()
        .borrow()
        .widgets
        .get_id_by_name(name)
        .unwrap_or_else(|| panic!("frame {name} should exist"))
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
fn set_alpha_real_change_marks_parent_and_child_visual_dirty() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "NoopAlphaDirtyFrame", UIParent)
            local child = CreateFrame("Frame", "NoopAlphaDirtyChild", frame)
            frame:SetAlpha(1)
            "#,
        )
        .expect("initial alpha dirty setup should succeed");

        let frame_id = frame_id_by_name(&env, "NoopAlphaDirtyFrame");
        let child_id = frame_id_by_name(&env, "NoopAlphaDirtyChild");

        clear_dirty(&env);

        env.exec(r#"NoopAlphaDirtyFrame:SetAlpha(0.5)"#)
            .expect("changed SetAlpha should succeed");

        let (dirty_mask, dirty_ids) = env.state().borrow().widgets.take_render_dirty_with_ids();
        let dirty_ids = dirty_ids.unwrap_or_default();
        assert_ne!(dirty_mask, 0, "SetAlpha value change should dirty at least one strata");
        assert!(
            dirty_ids.contains(&frame_id),
            "SetAlpha value change should dirty the parent frame"
        );
        assert!(
            dirty_ids.contains(&child_id),
            "SetAlpha value change should dirty child effective alpha too"
        );
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
fn set_formatted_text_same_value_with_overridden_format_still_calls_format_and_stays_clean() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "NoopFormattedOverrideFrame", UIParent)
            local text = frame:CreateFontString("NoopFormattedOverrideFontString", "ARTWORK")
            text:SetWidth(120)
            text:SetHeight(20)
            text:SetFormattedText("%dm", 60)

            local original_format = format
            _G.__noop_format_calls = 0
            format = function(...)
                _G.__noop_format_calls = _G.__noop_format_calls + 1
                return original_format(...)
            end
            "#,
        )
        .expect("formatted override setup should succeed");

        clear_dirty(&env);

        env.exec(r#"NoopFormattedOverrideFontString:SetFormattedText("%dm", 60)"#)
            .expect("same-value SetFormattedText with override should succeed");

        let format_calls = env
            .eval::<f64>(r#"return _G.__noop_format_calls"#)
            .expect("format call counter should be readable");
        assert_eq!(
            format_calls as u32, 1,
            "SetFormattedText should still dispatch through overridden format()"
        );
        assert_no_visual_dirty(
            &env,
            "SetFormattedText(%dm, 60) with overridden format on identical output",
        );
        assert_no_rect_dirty(
            &env,
            "SetFormattedText(%dm, 60) with overridden format on identical output",
        );
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
fn set_point_same_value_implicit_parent_form_is_a_true_noop() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "NoopPointImplicitFrame", UIParent)
            frame:SetPoint("CENTER", 10, 20)
            "#,
        )
        .expect("initial implicit point setup should succeed");

        clear_dirty(&env);

        env.exec(r#"NoopPointImplicitFrame:SetPoint("CENTER", 10, 20)"#)
            .expect("same-value implicit SetPoint should succeed");

        assert_no_visual_dirty(&env, "SetPoint implicit-parent form on identical anchor");
        assert_no_rect_dirty(&env, "SetPoint implicit-parent form on identical anchor");
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
fn set_font_object_equivalent_snapshot_different_table_is_a_true_noop() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            local frame = CreateFrame("Frame", "NoopFontObjectSnapshotFrame", UIParent)
            local text = frame:CreateFontString("NoopFontObjectSnapshotString", "ARTWORK")
            text:SetFontObject(GameFontNormalMed1)

            local copy = {}
            for k, v in pairs(GameFontNormalMed1) do
                copy[k] = v
            end
            _G.__noop_font_object_snapshot_copy = copy
            "#,
        )
        .expect("font object snapshot setup should succeed");

        clear_dirty(&env);

        env.exec(r#"NoopFontObjectSnapshotString:SetFontObject(__noop_font_object_snapshot_copy)"#)
            .expect("equivalent snapshot SetFontObject should succeed");

        let stored_copy = env
            .eval::<bool>(
                r#"return NoopFontObjectSnapshotString:GetFontObject() == __noop_font_object_snapshot_copy"#,
            )
            .expect("font object identity check should succeed");
        assert!(
            stored_copy,
            "font object store should accept replacement table reference"
        );
        assert_no_visual_dirty(&env, "SetFontObject on equivalent snapshot table");
        assert_no_rect_dirty(&env, "SetFontObject on equivalent snapshot table");
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
