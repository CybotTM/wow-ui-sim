//! Core frame methods: GetName, SetSize, Show/Hide, strata/level, mouse, scale, rect.

use super::super::handle::FrameRef;
use super::combat_lockdown;
use super::methods_helpers::{calculate_frame_height, calculate_frame_width};
use crate::lua_api::SimState;
use crate::lua_api::frame::handle::get_sim_state;
use mlua::Value;

/// Read screen dimensions from SimState.
pub(crate) fn screen_dims(state: &SimState) -> (f32, f32) {
    (state.screen_width, state.screen_height)
}

/// Check combat lockdown for `id` and fire ADDON_ACTION_BLOCKED if blocked.
/// Returns `true` when the caller should return early (call was blocked).
fn lockdown_blocked(lua: &mlua::Lua, id: u64, method_name: &str) -> bool {
    let state_rc = get_sim_state(lua);
    combat_lockdown::check_and_fire(lua, &state_rc, id, method_name)
}

/// Add core frame methods to the shared methods table.
pub fn add_core_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_identity_methods(methods);
    add_size_methods(methods);
    super::methods_rect::add_rect_methods(methods);
    add_visibility_methods(methods);
    add_strata_level_methods(methods);
    add_mouse_input_methods(methods);
    add_scale_methods(methods);
    add_region_query_methods(methods);
}

fn add_identity_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_get_name(methods);
    add_get_debug_name(methods);
    add_get_object_type(methods);
    add_is_object_type(methods);
}

fn add_get_name<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetName", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).and_then(|f| f.name.clone()))
    });
}

fn add_get_debug_name<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetDebugName", |lua, this, ()| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let Some(frame) = state.widgets.get(id) else {
            return Ok("[Unknown]".to_string());
        };
        if let Some(ref name) = frame.name {
            return Ok(name.clone());
        }
        if let Some(pid) = frame.parent_id
            && let Some(parent) = state.widgets.get(pid)
        {
            for (key, &cid) in &parent.children_keys {
                if cid == id {
                    let parent_name = parent.name.as_deref().unwrap_or("?");
                    return Ok(format!("{}.{}", parent_name, key));
                }
            }
        }
        Ok(format!("[{}]", frame.widget_type.as_str()))
    });
}

fn add_get_object_type<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetObjectType", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let obj_type = state
            .widgets
            .get(this.0)
            .map(|f| {
                f.object_type_name
                    .as_deref()
                    .unwrap_or(f.widget_type.as_str())
            })
            .unwrap_or("Frame");
        Ok(obj_type.to_string())
    });
}

fn add_is_object_type<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsObjectType", |lua, this, type_name: String| {
        use crate::widget::WidgetType;
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let frame = state.widgets.get(this.0);
        let wt = frame.map(|f| f.widget_type).unwrap_or(WidgetType::Frame);
        // Check object_type_name first (e.g., "ArchaeologyDigSiteFrame")
        if let Some(otn) = frame.and_then(|f| f.object_type_name.as_deref()) {
            if otn.eq_ignore_ascii_case(&type_name) {
                return Ok(true);
            }
            // Animation/Actor/ControlPoint types have their own hierarchy (not Frame)
            if is_anim_type(otn) {
                return Ok(anim_object_type_is_a(otn, &type_name));
            }
        }
        Ok(widget_type_is_a(wt, &type_name))
    });
}

fn add_size_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_size_getters(methods);
    add_size_setters(methods);
}

fn add_size_getters<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetWidth", |lua, this, ignore: Option<bool>| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        if ignore == Some(true) {
            return Ok(state_rc
                .borrow()
                .widgets
                .get(id)
                .map(|f| f.width)
                .unwrap_or(0.0));
        }
        let mut state = state_rc.borrow_mut();
        state.resolve_rect_if_dirty(id);
        Ok(calculate_frame_width(&state.widgets, id))
    });

    methods.add_method("GetHeight", |lua, this, ignore: Option<bool>| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        if ignore == Some(true) {
            return Ok(state_rc
                .borrow()
                .widgets
                .get(id)
                .map(|f| f.height)
                .unwrap_or(0.0));
        }
        let mut state = state_rc.borrow_mut();
        state.resolve_rect_if_dirty(id);
        Ok(calculate_frame_height(&state.widgets, id))
    });

    methods.add_method("GetSize", |lua, this, ignore: Option<bool>| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        if ignore == Some(true) {
            let state = state_rc.borrow();
            return Ok(state
                .widgets
                .get(id)
                .map(|f| (f.width, f.height))
                .unwrap_or((0.0, 0.0)));
        }
        let mut state = state_rc.borrow_mut();
        state.resolve_rect_if_dirty(id);
        let width = calculate_frame_width(&state.widgets, id);
        let height = calculate_frame_height(&state.widgets, id);
        Ok((width, height))
    });
}

fn add_size_setters<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_size(methods);
    add_set_width(methods);
    add_set_height(methods);
}

fn add_set_size<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetSize", |lua, this, (width, height): (f32, f32)| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let changed = state
            .widgets
            .get(id)
            .map(|f| f.width != width || f.height != height)
            .unwrap_or(false);
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.set_size(width, height);
            frame.width_is_text_auto = false;
        }
        if changed {
            state.widgets.mark_rect_dirty(id);
        }
        Ok(())
    });
}

fn add_set_width<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetWidth", |lua, this, width: f32| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let changed = state
            .widgets
            .get(id)
            .map(|f| f.width != width)
            .unwrap_or(false);
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.width = width;
            frame.width_is_text_auto = false;
        }
        if changed {
            state.widgets.mark_rect_dirty(id);
        }
        Ok(())
    });
}

fn add_set_height<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetHeight", |lua, this, height: f32| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let changed = state
            .widgets
            .get(id)
            .map(|f| f.height != height)
            .unwrap_or(false);
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.height = height;
        }
        if changed {
            state.widgets.mark_rect_dirty(id);
        }
        Ok(())
    });
}

fn add_visibility_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    super::methods_visibility::add_show_hide_methods(methods);
    add_is_visible(methods);
    add_is_shown(methods);
    add_collapse_layout_methods(methods);
}

fn add_is_visible<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsVisible", |lua, this, ()| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let mut cur = id;
        loop {
            match state.widgets.get(cur) {
                Some(f) if f.visible => match f.parent_id {
                    Some(pid) => cur = pid,
                    None => return Ok(true),
                },
                _ => return Ok(false),
            }
        }
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
        state.strata_buckets = None;
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
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.frame_level = level;
        }
        super::methods_hierarchy::propagate_strata_level_pub(&mut state.widgets, id);
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

    methods.add_method("GetMapID", |_lua, _this, ()| Ok(0));
    methods.add_method("SetMapID", |_lua, _this, _map_id: i32| Ok(()));
}

fn add_mouse_enable_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("EnableMouse", |lua, this, enable: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.mouse_enabled = enable;
        }
        Ok(())
    });

    methods.add_method("IsMouseEnabled", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.mouse_enabled)
            .unwrap_or(false))
    });

    methods.add_method("EnableMouseWheel", |_lua, _this, _enable: bool| Ok(()));
    methods.add_method("IsMouseWheelEnabled", |_lua, _this, ()| Ok(false));
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
    methods.add_method("EnableMouseMotion", |lua, this, enable: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.mouse_motion_enabled = enable;
        }
        Ok(())
    });

    methods.add_method("IsMouseMotionEnabled", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.mouse_motion_enabled)
            .unwrap_or(false))
    });

    methods.add_method("SetMouseMotionEnabled", |lua, this, enable: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.mouse_motion_enabled = enable;
        }
        Ok(())
    });

    methods.add_method("SetMouseClickEnabled", |lua, this, enable: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.mouse_enabled = enable;
        }
        Ok(())
    });

    methods.add_method("IsMouseClickEnabled", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.mouse_enabled)
            .unwrap_or(false))
    });
}

fn add_scale_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_get_set_scale(methods);
    add_scale_stubs(methods);
}

fn add_get_set_scale<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetScale", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(|f| f.scale).unwrap_or(1.0))
    });

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

    methods.add_method("GetEffectiveScale", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.effective_scale)
            .unwrap_or(1.0))
    });
}

fn add_scale_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetIgnoreParentScale", |_lua, _this, _ignore: bool| Ok(()));
    methods.add_method("GetIgnoreParentScale", |_lua, _this, ()| Ok(false));
    methods.add_method("SetIgnoreParentAlpha", |_lua, _this, _ignore: bool| Ok(()));
    methods.add_method("GetIgnoreParentAlpha", |_lua, _this, ()| Ok(false));
}

fn add_region_query_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_rect_query_methods(methods);
    add_region_stub_methods(methods);
}

fn add_rect_query_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsRectValid", |lua, this, ()| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let has_anchors = state
            .widgets
            .get(id)
            .map(|f| !f.anchors.is_empty())
            .unwrap_or(false);
        if !has_anchors {
            return Ok(false);
        }
        Ok(!state.widgets.is_rect_dirty(id))
    });

    methods.add_method("IsMouseMotionFocus", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.hovered_frame == Some(this.0))
    });
}

fn add_region_stub_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsObjectLoaded", |_lua, _this, ()| Ok(true));
    methods.add_method("IsMouseOver", |_lua, _this, _args: mlua::MultiValue| {
        Ok(true)
    });
    methods.add_method("StopAnimating", |_lua, _this, ()| Ok(()));
    methods.add_method("GetSourceLocation", |_lua, _this, ()| Ok(Value::Nil));
    methods.add_method("Intersects", |_lua, _this, _region: Value| Ok(false));
    methods.add_method("IsDrawLayerEnabled", |_lua, _this, _layer: String| Ok(true));
    methods.add_method(
        "SetDrawLayerEnabled",
        |_lua, _this, (_layer, _enabled): (String, bool)| Ok(()),
    );
}

/// Check if an object_type_name belongs to the animation/actor/controlpoint family.
pub(crate) fn is_anim_type(otn: &str) -> bool {
    matches!(
        otn,
        "AnimationGroup"
            | "Animation"
            | "Alpha"
            | "Rotation"
            | "Scale"
            | "Translation"
            | "LineTranslation"
            | "LineScale"
            | "Path"
            | "FlipBook"
            | "VertexColor"
            | "TextureCoordTranslation"
            | "ControlPoint"
            | "Actor"
            | "ModelSceneActor"
    )
}

/// Check IsObjectType for animation/actor/controlpoint types using WoW's hierarchy.
///
/// Hierarchy:
/// - AnimationGroup → UIObject only (NOT Frame, NOT Region)
/// - Animation subtypes → their type + parent chain + Animation + UIObject
///   - LineScale → Scale → Animation
///   - LineTranslation → Translation → Animation
///   - All others → Animation directly
/// - ControlPoint → UIObject only
/// - Actor → UIObject only
fn anim_object_type_is_a(obj_type: &str, query: &str) -> bool {
    // "Object" is the root for everything
    if query.eq_ignore_ascii_case("object") {
        return true;
    }
    // Animation types are NOT Region or Frame
    if query.eq_ignore_ascii_case("region") || query.eq_ignore_ascii_case("frame") {
        return false;
    }
    match obj_type {
        // These only match themselves + UIObject
        "AnimationGroup" | "ControlPoint" | "Actor" | "ModelSceneActor" => false,
        // LineScale inherits Scale → Animation
        "LineScale" => {
            query.eq_ignore_ascii_case("scale") || query.eq_ignore_ascii_case("animation")
        }
        // LineTranslation inherits Translation → Animation
        "LineTranslation" => {
            query.eq_ignore_ascii_case("translation") || query.eq_ignore_ascii_case("animation")
        }
        // All other animation subtypes inherit Animation directly
        _ => query.eq_ignore_ascii_case("animation"),
    }
}

/// Check if a widget type is or inherits from the given type name.
/// WorldFrame is special: GetObjectType() returns "Frame" but IsObjectType("Frame") is false.
fn widget_type_is_a(wt: crate::widget::WidgetType, type_name: &str) -> bool {
    use crate::widget::WidgetType;
    // WorldFrame: IsObjectType("WorldFrame") → true, IsObjectType("Frame") → false
    if wt == WidgetType::WorldFrame {
        return type_name.eq_ignore_ascii_case("worldframe")
            || type_name.eq_ignore_ascii_case("region");
    }
    if wt.as_str().eq_ignore_ascii_case(type_name) {
        return true;
    }
    match type_name.to_ascii_lowercase().as_str() {
        "object" | "region" => true,
        "frame" => !matches!(
            wt,
            WidgetType::FontString | WidgetType::Texture | WidgetType::Line
        ),
        "texture" => matches!(wt, WidgetType::Texture | WidgetType::Line),
        "line" => matches!(wt, WidgetType::Line),
        "button" => matches!(wt, WidgetType::Button | WidgetType::CheckButton),
        "model" => matches!(wt, WidgetType::Model | WidgetType::PlayerModel),
        "playermodel" => matches!(wt, WidgetType::PlayerModel),
        _ => false,
    }
}
