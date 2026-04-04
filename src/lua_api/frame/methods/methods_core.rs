//! Core frame methods: GetName, SetSize, Show/Hide, strata/level, mouse, scale, rect.

use super::super::handle::FrameRef;
use super::combat_lockdown;
use super::methods_core_state;
use super::methods_helpers::{calculate_frame_height, calculate_frame_width};
use crate::lua_api::SimState;
use crate::lua_api::frame::handle::get_sim_state;

/// Read screen dimensions from SimState.
pub(crate) fn screen_dims(state: &SimState) -> (f32, f32) {
    (state.screen_width, state.screen_height)
}

/// Check combat lockdown for `id` and fire ADDON_ACTION_BLOCKED if blocked.
/// Returns `true` when the caller should return early (call was blocked).
pub(super) fn lockdown_blocked(lua: &mlua::Lua, id: u64, method_name: &str) -> bool {
    let state_rc = get_sim_state(lua);
    combat_lockdown::check_and_fire(lua, &state_rc, id, method_name)
}

/// Add core frame methods to the shared methods table.
pub fn add_core_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_identity_methods(methods);
    add_size_methods(methods);
    super::methods_rect::add_rect_methods(methods);
    methods_core_state::add_core_state_methods(methods);
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
    add_get_width(methods);
    add_get_height(methods);
    add_get_size(methods);
}

fn add_get_width<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetWidth", |lua, this, ignore: Option<bool>| {
        size_value_or_raw(
            lua,
            this.0,
            ignore,
            |f| f.width,
            |widgets, id| calculate_frame_width(widgets, id),
        )
    });
}

fn add_get_height<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetHeight", |lua, this, ignore: Option<bool>| {
        size_value_or_raw(
            lua,
            this.0,
            ignore,
            |f| f.height,
            |widgets, id| calculate_frame_height(widgets, id),
        )
    });
}

fn add_get_size<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetSize", |lua, this, ignore: Option<bool>| {
        if ignore == Some(true) {
            return Ok(raw_frame_size(lua, this.0));
        }
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        state.resolve_rect_if_dirty(this.0);
        Ok((
            calculate_frame_width(&state.widgets, this.0),
            calculate_frame_height(&state.widgets, this.0),
        ))
    });
}

fn size_value_or_raw<Raw, Resolved>(
    lua: &mlua::Lua,
    id: u64,
    ignore: Option<bool>,
    raw: Raw,
    resolved: Resolved,
) -> mlua::Result<f32>
where
    Raw: FnOnce(&crate::widget::Frame) -> f32,
    Resolved: FnOnce(&crate::widget::WidgetRegistry, u64) -> f32,
{
    if ignore == Some(true) {
        return Ok(raw_size_value(lua, id, raw));
    }
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    state.resolve_rect_if_dirty(id);
    Ok(resolved(&state.widgets, id))
}

fn raw_size_value<F>(lua: &mlua::Lua, id: u64, raw: F) -> f32
where
    F: FnOnce(&crate::widget::Frame) -> f32,
{
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state.widgets.get(id).map(raw).unwrap_or(0.0)
}

fn raw_frame_size(lua: &mlua::Lua, id: u64) -> (f32, f32) {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .widgets
        .get(id)
        .map(|f| (f.width, f.height))
        .unwrap_or((0.0, 0.0))
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
