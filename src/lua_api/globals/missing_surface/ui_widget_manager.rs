//! Small `C_UIWidgetManager` surface for widget-set driven quest pins.
//!
//! The Blizzard world-quest pin path only needs a narrow slice of the
//! widget manager contract:
//! - a widget-set info table so `UIWidgetContainerMixin:RegisterForWidgetSet`
//!   accepts the set
//! - a single `IconAndText` widget so the quest pin acquires the
//!   `Worldquest-icon` child texture the render-order tests look for

use super::ensure_namespace;
use crate::lua_api::methods::{create_string, create_table, table_set};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const QUEST_WIDGET_SET_BASE_ID: i32 = 1_000_000;
const WORLD_QUEST_WIDGET_TEXTURE_KIT: &str = "Worldquest";

pub(super) fn register_ui_widget_manager_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_UIWidgetManager")?;
    table_set_rust_fn_static(state, ns, "GetWidgetSetInfo", build_widget_set_info)?;
    table_set_rust_fn_static(state, ns, "GetAllWidgetsBySetID", build_widgets_for_set_id)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetIconAndTextWidgetVisualizationInfo",
        build_icon_and_text_widget_visualization_info,
    )?;
    Ok(())
}

fn build_widget_set_info(state: &mut LuaState) -> LuaResult<u32> {
    let widget_set_id = i32::from_stack(state, 1)?;
    if !is_world_quest_widget_set(widget_set_id) {
        return Ok(0);
    }

    let info = create_table(state);
    table_set(state, info, "layoutDirection", Val::Num(2.0));
    table_set(state, info, "verticalPadding", Val::Num(0.0));
    state.push(info);
    Ok(1)
}

fn build_widgets_for_set_id(state: &mut LuaState) -> LuaResult<u32> {
    let widget_set_id = i32::from_stack(state, 1)?;
    let result = create_table(state);
    let Val::Table(result_ref) = result else {
        unreachable!("create_table must return a table");
    };

    if is_world_quest_widget_set(widget_set_id) {
        let widget = create_table(state);
        table_set(state, widget, "widgetID", Val::Num(widget_set_id as f64));
        table_set(state, widget, "widgetType", Val::Num(0.0));
        table_set(state, widget, "orderIndex", Val::Num(1.0));
        table_set(state, widget, "widgetTag", Val::Nil);
        table_set(state, widget, "hasTimer", Val::Bool(false));
        table_set(state, widget, "inAnimType", Val::Num(0.0));
        table_set(state, widget, "outAnimType", Val::Num(0.0));
        table_set(state, widget, "layoutDirection", Val::Num(0.0));
        table_set(state, widget, "modelSceneLayer", Val::Num(0.0));
        table_set(state, widget, "scriptedAnimationEffectID", Val::Num(0.0));
        if let Some(table) = state.gc.tables.get_mut(result_ref) {
            let _ = table.raw_set(Val::Num(1.0), widget, &state.gc.string_arena);
        }
        state.gc.barrier_back(result_ref);
    }

    state.push(result);
    Ok(1)
}

fn build_icon_and_text_widget_visualization_info(state: &mut LuaState) -> LuaResult<u32> {
    let widget_id = i32::from_stack(state, 1)?;
    if !is_world_quest_widget_id(widget_id) {
        return Ok(0);
    }

    let info = create_table(state);
    write_icon_and_text_widget_scalar_fields(state, info);
    write_icon_and_text_widget_string_fields(state, info);
    state.push(info);
    Ok(1)
}

fn write_icon_and_text_widget_scalar_fields(state: &mut LuaState, info: Val) {
    table_set(state, info, "hasTimer", Val::Bool(false));
    table_set(state, info, "inAnimType", Val::Num(0.0));
    table_set(state, info, "layoutDirection", Val::Num(0.0));
    table_set(state, info, "modelSceneLayer", Val::Num(0.0));
    table_set(state, info, "orderIndex", Val::Num(1.0));
    table_set(state, info, "outAnimType", Val::Num(0.0));
    table_set(state, info, "scriptedAnimationEffectID", Val::Num(0.0));
    table_set(state, info, "shiftTextType", Val::Num(0.0));
    table_set(state, info, "state", Val::Num(1.0));
    table_set(state, info, "widgetScale", Val::Num(0.0));
    table_set(state, info, "widgetSizeSetting", Val::Num(0.0));
}

fn write_icon_and_text_widget_string_fields(state: &mut LuaState, info: Val) {
    let dynamic_tooltip = create_string(state, "");
    table_set(state, info, "dynamicTooltip", dynamic_tooltip);
    let frame_texture_kit = create_string(state, "");
    table_set(state, info, "frameTextureKit", frame_texture_kit);
    let text = create_string(state, "");
    table_set(state, info, "text", text);
    let texture_kit = create_string(state, WORLD_QUEST_WIDGET_TEXTURE_KIT);
    table_set(state, info, "textureKit", texture_kit);
    let tooltip = create_string(state, "");
    table_set(state, info, "tooltip", tooltip);
    table_set(state, info, "tooltipLoc", Val::Nil);
    let widget_tag = create_string(state, "");
    table_set(state, info, "widgetTag", widget_tag);
}

fn is_world_quest_widget_set(widget_set_id: i32) -> bool {
    widget_set_id >= QUEST_WIDGET_SET_BASE_ID
}

fn is_world_quest_widget_id(widget_id: i32) -> bool {
    widget_id >= QUEST_WIDGET_SET_BASE_ID
}
