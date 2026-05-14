//! Runtime children expected by Blizzard dropdown button templates.

use super::helpers::get_global;
use crate::lua_api::methods::extract_frame_id;
use crate::widget::WidgetType;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(super) fn ensure_dropdown_button_children(
    state: &mut LuaState,
    btn_id: u64,
    btn_name: &str,
) -> LuaResult<()> {
    let color_swatch_name = format!("{btn_name}ColorSwatch");
    for child in dropdown_button_children(btn_name) {
        find_or_create_dropdown_child(state, btn_id, child)?;
    }
    ensure_dropdown_color_swatch_color(state, &color_swatch_name)?;
    Ok(())
}

fn dropdown_button_children(btn_name: &str) -> Vec<DropdownButtonChild> {
    vec![
        DropdownButtonChild::texture("Highlight", &format!("{btn_name}Highlight")),
        DropdownButtonChild::texture("Check", &format!("{btn_name}Check")),
        DropdownButtonChild::texture("UnCheck", &format!("{btn_name}UnCheck")),
        DropdownButtonChild::texture("Icon", &format!("{btn_name}Icon")),
        DropdownButtonChild::button("ColorSwatch", &format!("{btn_name}ColorSwatch")),
        DropdownButtonChild::button("ExpandArrow", &format!("{btn_name}ExpandArrow")),
        DropdownButtonChild::button("invisibleButton", &format!("{btn_name}InvisibleButton")),
        DropdownButtonChild::frame("NewFeature", &format!("{btn_name}NewFeature")),
        DropdownButtonChild::font_string("Text", &format!("{btn_name}NormalText")),
    ]
}

struct DropdownButtonChild {
    parent_key: &'static str,
    global_name: String,
    widget_type: WidgetType,
    frame_type: &'static str,
}

impl DropdownButtonChild {
    fn texture(parent_key: &'static str, global_name: &str) -> Self {
        Self::new(parent_key, global_name, WidgetType::Texture, "Texture")
    }

    fn button(parent_key: &'static str, global_name: &str) -> Self {
        Self::new(parent_key, global_name, WidgetType::Button, "Button")
    }

    fn frame(parent_key: &'static str, global_name: &str) -> Self {
        Self::new(parent_key, global_name, WidgetType::Frame, "Frame")
    }

    fn font_string(parent_key: &'static str, global_name: &str) -> Self {
        Self::new(
            parent_key,
            global_name,
            WidgetType::FontString,
            "FontString",
        )
    }

    fn new(
        parent_key: &'static str,
        global_name: &str,
        widget_type: WidgetType,
        frame_type: &'static str,
    ) -> Self {
        Self {
            parent_key,
            global_name: global_name.to_string(),
            widget_type,
            frame_type,
        }
    }
}

fn find_or_create_dropdown_child(
    state: &mut LuaState,
    parent_id: u64,
    child: DropdownButtonChild,
) -> LuaResult<u64> {
    let child_val = get_global(state, &child.global_name);
    let child_id = match extract_frame_id(state, child_val) {
        Some(child_id) => child_id,
        None => super::create_frame_instance(
            state,
            child.widget_type,
            child.frame_type,
            Some(child.global_name),
            Some(parent_id),
            true,
            None,
        )?,
    };
    crate::lua_api::globals::template::assign_parent_key(
        state,
        parent_id,
        child.parent_key,
        child_id,
    )?;
    Ok(child_id)
}

fn ensure_dropdown_color_swatch_color(
    state: &mut LuaState,
    color_swatch_name: &str,
) -> LuaResult<()> {
    let color_swatch_val = get_global(state, color_swatch_name);
    let Some(color_swatch_id) = extract_frame_id(state, color_swatch_val) else {
        return Ok(());
    };
    let color = DropdownButtonChild::texture("Color", &format!("{color_swatch_name}Color"));
    find_or_create_dropdown_child(state, color_swatch_id, color)?;
    Ok(())
}
