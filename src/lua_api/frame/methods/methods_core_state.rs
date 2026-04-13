use super::super::handle::FrameRef;
use super::methods_core::lockdown_blocked;
use crate::lua_api::frame::handle::get_sim_state;

pub(super) fn add_core_state_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_visibility_methods(methods);
    add_strata_level_methods(methods);
    add_mouse_input_methods(methods);
    add_scale_methods(methods);
    super::methods_core_region::add_region_query_methods(methods);
}

fn add_visibility_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    super::methods_visibility::add_show_hide_methods(methods);
    add_is_visible(methods);
    add_is_shown(methods);
    add_collapse_layout_methods(methods);
}

fn add_is_visible<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsVisible", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.is_ancestor_visible(this.0))
    });
}

fn add_is_shown<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsShown", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.visible)
            .unwrap_or(false))
    });
}

fn add_collapse_layout_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetCollapsesLayout", |lua, this, val: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.collapses_layout = val;
        }
        Ok(())
    });

    methods.add_method("CollapsesLayout", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.collapses_layout)
            .unwrap_or(false))
    });

    methods.add_method("IsCollapsed", |lua, this, ()| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        is_collapsed_impl(&state, id)
    });
}

fn is_collapsed_impl(state: &crate::lua_api::SimState, id: u64) -> mlua::Result<bool> {
    let frame = match state.widgets.get(id) {
        Some(f) => f,
        None => return Ok(false),
    };
    if !frame.collapses_layout {
        return Ok(false);
    }
    let mut visible = frame.visible;
    let mut cur_parent = frame.parent_id;
    while visible {
        match cur_parent.and_then(|pid| state.widgets.get(pid)) {
            Some(p) if p.visible => cur_parent = p.parent_id,
            Some(_) => {
                visible = false;
            }
            None => break,
        }
    }
    Ok(!visible)
}

fn add_strata_level_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_alpha_methods(methods);
    add_strata_methods(methods);
    add_level_methods(methods);
    add_toplevel_methods(methods);
}

fn add_toplevel_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetToplevel", |lua, this, toplevel: bool| {
        let id = this.0;
        if lockdown_blocked(lua, id, "SetToplevel") {
            return Ok(());
        }
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(f) = state.widgets.get_mut(id) {
            f.toplevel = toplevel;
        }
        Ok(())
    });

    methods.add_method("IsToplevel", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        Ok(state_rc
            .borrow()
            .widgets
            .get(this.0)
            .map(|f| f.toplevel)
            .unwrap_or(false))
    });
}

fn add_alpha_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_alpha(methods);
    add_get_alpha_methods(methods);
    add_set_alpha_from_boolean(methods);
    add_ignore_parent_alpha_methods(methods);
}

fn add_set_alpha<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetAlpha", |lua, this, alpha: f32| {
        let id = this.0;
        let clamped = alpha.clamp(0.0, 1.0);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let changed = state
            .widgets
            .get(id)
            .map(|f| f.alpha != clamped)
            .unwrap_or(false);
        if changed {
            let parent_eff = state
                .widgets
                .get(id)
                .and_then(|f| f.parent_id)
                .and_then(|pid| state.widgets.get(pid))
                .map(|p| p.effective_alpha)
                .unwrap_or(1.0);
            if let Some(frame) = state.widgets.get_mut_visual(id) {
                frame.alpha = clamped;
            }
            state.widgets.propagate_effective_alpha(id, parent_eff);
        }
        Ok(())
    });
}

fn add_get_alpha_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetAlpha", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(|f| f.alpha).unwrap_or(1.0))
    });

    methods.add_method("GetEffectiveAlpha", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.effective_alpha)
            .unwrap_or(1.0))
    });
}

fn add_set_alpha_from_boolean<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetAlphaFromBoolean", |lua, this, flag: bool| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let new_alpha = if flag { 1.0 } else { 0.0 };
        let parent_eff = state
            .widgets
            .get(id)
            .and_then(|f| f.parent_id)
            .and_then(|pid| state.widgets.get(pid))
            .map(|p| p.effective_alpha)
            .unwrap_or(1.0);
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.alpha = new_alpha;
        }
        state.widgets.propagate_effective_alpha(id, parent_eff);
        Ok(())
    });
}

fn add_ignore_parent_alpha_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetIgnoreParentAlpha", |lua, this, ignore: bool| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let parent_eff = state
            .widgets
            .get(id)
            .and_then(|f| f.parent_id)
            .and_then(|pid| state.widgets.get(pid))
            .map(|p| p.effective_alpha)
            .unwrap_or(1.0);
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.ignore_parent_alpha = ignore;
        }
        state.widgets.propagate_effective_alpha(id, parent_eff);
        Ok(())
    });
    add_bool_frame_getter(methods, "GetIgnoreParentAlpha", |f| f.ignore_parent_alpha);
    add_bool_frame_getter(methods, "IsIgnoringParentAlpha", |f| f.ignore_parent_alpha);
}

fn add_strata_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_frame_strata(methods);
    add_get_frame_strata(methods);
    add_fixed_frame_strata(methods);
}

fn add_set_frame_strata<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFrameStrata", |lua, this, strata: String| {
        let id = this.0;
        if lockdown_blocked(lua, id, "SetFrameStrata") {
            return Ok(());
        }
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let Some(s) = crate::widget::FrameStrata::from_str(&strata) else {
            return Ok(());
        };
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.frame_strata = s;
            frame.has_fixed_frame_strata = true;
        }
        let mut queue: Vec<u64> = state
            .widgets
            .get(id)
            .map(|f| f.children.clone())
            .unwrap_or_default();
        while let Some(child_id) = queue.pop() {
            let Some(child) = state.widgets.get_mut_visual(child_id) else {
                continue;
            };
            if child.has_fixed_frame_strata {
                continue;
            }
            child.frame_strata = s;
            queue.extend(child.children.iter().copied());
        }
        state.invalidate_strata_buckets();
        Ok(())
    });
}

fn add_get_frame_strata<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetFrameStrata", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let strata = state
            .widgets
            .get(this.0)
            .map(|f| f.frame_strata.as_str())
            .unwrap_or("MEDIUM");
        Ok(strata.to_string())
    });
}

fn add_fixed_frame_strata<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFixedFrameStrata", |lua, this, fixed: bool| {
        let id = this.0;
        if lockdown_blocked(lua, id, "SetFixedFrameStrata") {
            return Ok(());
        }
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.has_fixed_frame_strata = fixed;
        }
        Ok(())
    });

    methods.add_method("HasFixedFrameStrata", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.has_fixed_frame_strata)
            .unwrap_or(false))
    });
}

fn add_level_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_frame_level(methods);
    add_get_frame_level(methods);
    add_fixed_frame_level(methods);
}

fn add_set_frame_level<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFrameLevel", |lua, this, level: i32| {
        let id = this.0;
        if lockdown_blocked(lua, id, "SetFrameLevel") {
            return Ok(());
        }
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let Some(current_level) = state.widgets.get(id).map(|frame| frame.frame_level) else {
            return Ok(());
        };
        if current_level == level {
            return Ok(());
        }
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.frame_level = level;
        }
        super::methods_hierarchy::propagate_strata_level_pub(&mut state.widgets, id);
        state.invalidate_strata_buckets();
        Ok(())
    });
}

fn add_get_frame_level<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetFrameLevel", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.frame_level)
            .unwrap_or(0))
    });
}

fn add_fixed_frame_level<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFixedFrameLevel", |lua, this, fixed: bool| {
        let id = this.0;
        if lockdown_blocked(lua, id, "SetFixedFrameLevel") {
            return Ok(());
        }
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.has_fixed_frame_level = fixed;
        }
        Ok(())
    });

    methods.add_method("HasFixedFrameLevel", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.has_fixed_frame_level)
            .unwrap_or(false))
    });
}

fn add_mouse_input_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_id_methods(methods);
    add_mouse_enable_methods(methods);
    add_keyboard_methods(methods);
    methods.add_method(
        "RegisterForMouse",
        |_lua, _this, _args: mlua::MultiValue| Ok(()),
    );
    add_mouse_motion_methods(methods);
}

fn add_id_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetID", |lua, this, user_id: i32| {
        let state_rc = get_sim_state(lua);
        if let Some(f) = state_rc.borrow_mut().widgets.get_mut(this.0) {
            f.user_id = user_id;
        }
        Ok(())
    });

    methods.add_method("GetID", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        Ok(state_rc
            .borrow()
            .widgets
            .get(this.0)
            .map(|f| f.user_id)
            .unwrap_or(0))
    });

    add_map_id_methods(methods);
}

fn add_map_id_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetMapID", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .quest_blobs
            .get(&this.0)
            .map(|b| b.map_id as i32)
            .unwrap_or(0))
    });
    methods.add_method("SetMapID", |lua, this, map_id: i32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let blob = state.quest_blobs.entry(this.0).or_default();
        blob.map_id = map_id as u32;
        Ok(())
    });
}

fn add_mouse_enable_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_bool_frame_setter(methods, "EnableMouse", |frame, enable| {
        frame.mouse_enabled = enable;
    });
    add_bool_frame_getter(methods, "IsMouseEnabled", |frame| frame.mouse_enabled);
    add_bool_frame_setter(methods, "EnableMouseWheel", |frame, enable| {
        frame.mouse_wheel_enabled = enable;
    });
    add_bool_frame_getter(methods, "IsMouseWheelEnabled", |frame| {
        frame.mouse_wheel_enabled
    });
}

fn add_keyboard_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("EnableKeyboard", |lua, this, enable: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(f) = state.widgets.get_mut(this.0) {
            f.keyboard_enabled = enable;
        }
        Ok(())
    });

    methods.add_method("IsKeyboardEnabled", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.keyboard_enabled)
            .unwrap_or(false))
    });
}

fn add_mouse_motion_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_bool_frame_setter(methods, "EnableMouseMotion", |frame, enable| {
        frame.mouse_motion_enabled = enable;
    });
    add_bool_frame_getter(methods, "IsMouseMotionEnabled", |frame| {
        frame.mouse_motion_enabled
    });
    add_bool_frame_setter(methods, "SetMouseMotionEnabled", |frame, enable| {
        frame.mouse_motion_enabled = enable;
    });
    add_bool_frame_setter(methods, "SetMouseClickEnabled", |frame, enable| {
        frame.mouse_enabled = enable;
    });
    add_bool_frame_getter(methods, "IsMouseClickEnabled", |frame| frame.mouse_enabled);
}

fn add_bool_frame_setter<M, F>(methods: &mut M, name: &str, update: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::widget::Frame, bool) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, enable: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            update(frame, enable);
        }
        Ok(())
    });
}

fn add_bool_frame_getter<M, F>(methods: &mut M, name: &str, read: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&crate::widget::Frame) -> bool + Copy + 'static,
{
    methods.add_method(name, move |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(read).unwrap_or(false))
    });
}

fn add_scale_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_get_set_scale(methods);
    add_ignore_parent_scale_methods(methods);
}

fn add_get_set_scale<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_scale_getter(methods, "GetScale", |frame| frame.scale);
    add_set_scale(methods);
    add_scale_getter(methods, "GetEffectiveScale", |frame| frame.effective_scale);
}

fn add_scale_getter<M, F>(methods: &mut M, name: &str, read: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&crate::widget::Frame) -> f32 + Copy + 'static,
{
    methods.add_method(name, move |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(read).unwrap_or(1.0))
    });
}

fn add_set_scale<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetScale", |lua, this, scale: f32| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if scale <= 0.0 {
            return Err(mlua::Error::RuntimeError(
                "Frame:SetScale(): Scale must be > 0".into(),
            ));
        }
        let parent_eff_scale = state
            .widgets
            .get(id)
            .and_then(|f| f.parent_id)
            .and_then(|pid| state.widgets.get(pid))
            .map(|p| p.effective_scale)
            .unwrap_or(1.0);
        if let Some(f) = state.widgets.get_mut_visual(id) {
            f.scale = scale;
        }
        state
            .widgets
            .propagate_effective_scale(id, parent_eff_scale);
        state.widgets.mark_rect_dirty(id);
        Ok(())
    });
}

fn add_ignore_parent_scale_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetIgnoreParentScale", |lua, this, ignore: bool| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let parent_eff_scale = state
            .widgets
            .get(id)
            .and_then(|f| f.parent_id)
            .and_then(|pid| state.widgets.get(pid))
            .map(|p| p.effective_scale)
            .unwrap_or(1.0);
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.ignore_parent_scale = ignore;
        }
        state
            .widgets
            .propagate_effective_scale(id, parent_eff_scale);
        state.widgets.mark_rect_dirty(id);
        Ok(())
    });
    add_bool_frame_getter(methods, "GetIgnoreParentScale", |f| f.ignore_parent_scale);
    add_bool_frame_getter(methods, "IsIgnoringParentScale", |f| f.ignore_parent_scale);
}

#[cfg(test)]
mod tests {
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
        // Clear dirty tracking from initial setup.
        env.state().borrow_mut().widgets.take_rect_dirty_ids();

        env.exec(r#"ScaleNoopFrame:SetScale(0.75)"#)
            .expect("repeat SetScale should succeed");

        let dirty: bool = env
            .state()
            .borrow()
            .widgets
            .rect_dirty_contains(id);
        assert!(
            !dirty,
            "no-op SetScale should not mark the frame rect dirty"
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
        env.state().borrow_mut().widgets.take_rect_dirty_ids();

        env.exec(r#"ScaleChangeFrame:SetScale(2.0)"#)
            .expect("SetScale with new value should succeed");

        assert!(
            env.state().borrow().widgets.rect_dirty_contains(id),
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

        let (_, dirty_ids) = env
            .state()
            .borrow()
            .widgets
            .take_render_dirty_with_ids();
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
        env.state().borrow_mut().widgets.take_rect_dirty_ids();

        env.exec(r#"IgnoreScaleNoopFrame:SetIgnoreParentScale(true)"#)
            .unwrap();

        assert!(
            !env.state().borrow().widgets.rect_dirty_contains(id),
            "no-op SetIgnoreParentScale should not mark the frame rect dirty",
        );
    }
}
