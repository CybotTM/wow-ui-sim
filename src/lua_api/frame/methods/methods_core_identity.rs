//! Core frame identity and type-query methods.

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::get_sim_state;

pub(super) fn add_identity_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_get_name(methods);
    add_get_debug_name(methods);
    add_get_object_type(methods);
    add_is_object_type(methods);
}

fn add_get_name<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetName", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .and_then(|frame| frame.name.clone()))
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
        if let Some(parent_id) = frame.parent_id
            && let Some(parent) = state.widgets.get(parent_id)
        {
            for (key, &child_id) in &parent.children_keys {
                if child_id == id {
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
        let object_type = state
            .widgets
            .get(this.0)
            .map(|frame| {
                frame
                    .object_type_name
                    .as_deref()
                    .unwrap_or(frame.widget_type.as_str())
            })
            .unwrap_or("Frame");
        Ok(object_type.to_string())
    });
}

fn add_is_object_type<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsObjectType", |lua, this, type_name: String| {
        use crate::widget::WidgetType;

        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let frame = state.widgets.get(this.0);
        let widget_type = frame
            .map(|frame| frame.widget_type)
            .unwrap_or(WidgetType::Frame);

        if let Some(object_type_name) = frame.and_then(|frame| frame.object_type_name.as_deref()) {
            if object_type_name.eq_ignore_ascii_case(&type_name) {
                return Ok(true);
            }
            if is_anim_type(object_type_name) {
                return Ok(anim_object_type_is_a(object_type_name, &type_name));
            }
        }

        Ok(widget_type_is_a(widget_type, &type_name))
    });
}

/// Check if an object_type_name belongs to the animation/actor/controlpoint family.
pub(crate) fn is_anim_type(object_type_name: &str) -> bool {
    matches!(
        object_type_name,
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
/// - AnimationGroup -> UIObject only (NOT Frame, NOT Region)
/// - Animation subtypes -> their type + parent chain + Animation + UIObject
///   - LineScale -> Scale -> Animation
///   - LineTranslation -> Translation -> Animation
///   - All others -> Animation directly
/// - ControlPoint -> UIObject only
/// - Actor -> UIObject only
fn anim_object_type_is_a(object_type: &str, query: &str) -> bool {
    if query.eq_ignore_ascii_case("object") {
        return true;
    }
    if query.eq_ignore_ascii_case("region") || query.eq_ignore_ascii_case("frame") {
        return false;
    }

    match object_type {
        "AnimationGroup" | "ControlPoint" | "Actor" | "ModelSceneActor" => false,
        "LineScale" => {
            query.eq_ignore_ascii_case("scale") || query.eq_ignore_ascii_case("animation")
        }
        "LineTranslation" => {
            query.eq_ignore_ascii_case("translation") || query.eq_ignore_ascii_case("animation")
        }
        _ => query.eq_ignore_ascii_case("animation"),
    }
}

/// Check if a widget type is or inherits from the given type name.
/// WorldFrame is special: GetObjectType() returns "Frame" but IsObjectType("Frame") is false.
fn widget_type_is_a(widget_type: crate::widget::WidgetType, type_name: &str) -> bool {
    use crate::widget::WidgetType;

    if widget_type == WidgetType::WorldFrame {
        return type_name.eq_ignore_ascii_case("worldframe")
            || type_name.eq_ignore_ascii_case("region");
    }
    if widget_type.as_str().eq_ignore_ascii_case(type_name) {
        return true;
    }

    match type_name.to_ascii_lowercase().as_str() {
        "object" | "region" => true,
        "frame" => !matches!(
            widget_type,
            WidgetType::FontString | WidgetType::Texture | WidgetType::Line
        ),
        "texture" => matches!(widget_type, WidgetType::Texture | WidgetType::Line),
        "line" => matches!(widget_type, WidgetType::Line),
        "button" => matches!(widget_type, WidgetType::Button | WidgetType::CheckButton),
        "model" => matches!(widget_type, WidgetType::Model | WidgetType::PlayerModel),
        "playermodel" => matches!(widget_type, WidgetType::PlayerModel),
        _ => false,
    }
}
