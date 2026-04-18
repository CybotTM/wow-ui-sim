//! rilua RustFn equivalents for methods_text/, methods_attribute.rs, and methods_event.rs.
//!
//! Each function is a `RustFn` (`fn(&mut LuaState) -> LuaResult<u32>`) that mirrors
//! the corresponding mlua method. Complex operations are stubbed with TODO.

mod attributes;
mod callbacks;
mod events;
mod helpers;
mod text;

use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;

/// Register all text, attribute, and event RustFn methods on the given table.
pub fn register_all(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    register_text_methods(state, table)?;
    register_attribute_methods(state, table)?;
    register_event_methods(state, table)
}

fn register_text_methods(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    register_plain_text(state, table)?;
    register_font_methods(state, table)?;
    register_text_layout(state, table)?;
    register_styled_text(state, table)
}

fn register_plain_text(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "SetText", text::set_text)?;
    table_set_rust_fn_static(state, table, "GetText", text::get_text)?;
    table_set_rust_fn_static(state, table, "ClearText", text::clear_text)?;
    table_set_rust_fn_static(state, table, "SetFormattedText", text::set_formatted_text)?;
    table_set_rust_fn_static(state, table, "ApplyDefaultText", text::apply_default_text)?;
    table_set_rust_fn_static(
        state,
        table,
        "TryApplyDefaultText",
        text::try_apply_default_text,
    )?;
    Ok(())
}

fn register_font_methods(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "SetFont", text::set_font)?;
    table_set_rust_fn_static(state, table, "GetFont", text::get_font)?;
    table_set_rust_fn_static(state, table, "SetFontObject", text::set_font_object)?;
    table_set_rust_fn_static(
        state,
        table,
        "SetFontObjectsToTry",
        text::set_font_objects_to_try,
    )?;
    table_set_rust_fn_static(state, table, "GetFontObject", text::get_font_object)?;
    table_set_rust_fn_static(state, table, "SetFontHeight", text::set_font_height)?;
    table_set_rust_fn_static(state, table, "SetTextHeight", text::set_text_height)?;
    table_set_rust_fn_static(state, table, "GetFontHeight", text::get_font_height)?;
    Ok(())
}

fn register_text_layout(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "GetStringWidth", text::get_string_width)?;
    table_set_rust_fn_static(state, table, "GetStringHeight", text::get_string_height)?;
    table_set_rust_fn_static(state, table, "GetTextWidth", text::get_text_width)?;
    table_set_rust_fn_static(state, table, "GetLineHeight", text::get_line_height)?;
    table_set_rust_fn_static(state, table, "IsTruncated", text::is_truncated)?;
    table_set_rust_fn_static(
        state,
        table,
        "GetUnboundedStringWidth",
        text::get_unbounded_string_width,
    )?;
    table_set_rust_fn_static(state, table, "SetJustifyH", text::set_justify_h)?;
    table_set_rust_fn_static(state, table, "GetJustifyH", text::get_justify_h)?;
    table_set_rust_fn_static(state, table, "SetJustifyV", text::set_justify_v)?;
    table_set_rust_fn_static(state, table, "GetJustifyV", text::get_justify_v)?;
    table_set_rust_fn_static(state, table, "SetWordWrap", text::set_word_wrap)?;
    table_set_rust_fn_static(state, table, "GetWordWrap", text::get_word_wrap)?;
    table_set_rust_fn_static(state, table, "CanWordWrap", text::can_word_wrap)?;
    table_set_rust_fn_static(state, table, "SetMaxLines", text::set_max_lines)?;
    table_set_rust_fn_static(state, table, "GetMaxLines", text::get_max_lines)?;
    table_set_rust_fn_static(state, table, "SetNonSpaceWrap", text::set_non_space_wrap)?;
    table_set_rust_fn_static(state, table, "CanNonSpaceWrap", text::can_non_space_wrap)?;
    table_set_rust_fn_static(state, table, "GetTextScale", text::get_text_scale)?;
    table_set_rust_fn_static(state, table, "SetTextScale", text::set_text_scale)?;
    table_set_rust_fn_static(state, table, "SetTextToFit", text::set_text_to_fit)?;
    table_set_rust_fn_static(state, table, "ScaleTextToFit", text::scale_text_to_fit)?;
    Ok(())
}

fn register_styled_text(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "SetTextColor", text::set_text_color)?;
    table_set_rust_fn_static(state, table, "GetTextColor", text::get_text_color)?;
    table_set_rust_fn_static(
        state,
        table,
        "SetHyperlinksEnabled",
        text::set_hyperlinks_enabled,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetHyperlinksEnabled",
        text::get_hyperlinks_enabled,
    )?;
    Ok(())
}

fn register_attribute_methods(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    register_attribute_core(state, table)?;
    register_attribute_frame_refs(state, table)?;
    register_attribute_protection(state, table)?;
    register_attribute_behavior_flags(state, table)?;
    register_attribute_hit_rect(state, table)?;
    Ok(())
}

fn register_attribute_core(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "GetAttribute", attributes::get_attribute)?;
    table_set_rust_fn_static(state, table, "SetAttribute", attributes::set_attribute)?;
    table_set_rust_fn_static(
        state,
        table,
        "SetAttributeNoHandler",
        attributes::set_attribute_no_handler,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "ClearAttributes",
        attributes::clear_attributes,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "ExecuteAttribute",
        attributes::execute_attribute,
    )?;
    Ok(())
}

fn register_attribute_frame_refs(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "SetFrameRef", attributes::set_frame_ref)?;
    table_set_rust_fn_static(state, table, "GetFrameRef", attributes::get_frame_ref)?;
    Ok(())
}

fn register_attribute_protection(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "SetForbidden", attributes::set_forbidden)?;
    table_set_rust_fn_static(state, table, "IsForbidden", attributes::is_forbidden)?;
    table_set_rust_fn_static(
        state,
        table,
        "CanChangeProtectedState",
        attributes::can_change_protected_state,
    )?;
    Ok(())
}

fn register_attribute_behavior_flags(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    register_attribute_input_render_flags(state, table)?;
    register_attribute_motion_scripts(state, table)?;
    register_attribute_clip_children(state, table)?;
    Ok(())
}

fn register_attribute_input_render_flags(
    state: &mut LuaState,
    table: GcRef<Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "SetPassThroughButtons",
        attributes::set_pass_through_buttons,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "SetFlattensRenderLayers",
        attributes::set_flattens_render_layers,
    )?;
    Ok(())
}

fn register_attribute_motion_scripts(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "SetMotionScriptsWhileDisabled",
        attributes::set_motion_scripts_while_disabled,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetMotionScriptsWhileDisabled",
        attributes::get_motion_scripts_while_disabled,
    )?;
    Ok(())
}

fn register_attribute_clip_children(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "SetClipsChildren",
        attributes::set_clips_children,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "DoesClipChildren",
        attributes::does_clip_children,
    )?;
    Ok(())
}

fn register_attribute_hit_rect(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "SetHitRectInsets",
        attributes::set_hit_rect_insets,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetHitRectInsets",
        attributes::get_hit_rect_insets,
    )?;
    Ok(())
}

fn register_event_methods(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    register_event_registration(state, table)?;
    register_event_callbacks(state, table)?;
    register_event_keyboard_propagation(state, table)?;
    register_event_script_handlers(state, table)?;
    Ok(())
}

fn register_event_registration(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "RegisterEvent", events::register_event)?;
    table_set_rust_fn_static(
        state,
        table,
        "RegisterUnitEvent",
        events::register_unit_event,
    )?;
    table_set_rust_fn_static(state, table, "UnregisterEvent", events::unregister_event)?;
    table_set_rust_fn_static(
        state,
        table,
        "UnregisterAllEvents",
        events::unregister_all_events,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "RegisterAllEvents",
        events::register_all_events,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "IsEventRegistered",
        events::is_event_registered,
    )?;
    Ok(())
}

fn register_event_callbacks(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    register_event_callback_listeners(state, table)?;
    register_named_callback_table(state, table)?;
    Ok(())
}

fn register_event_callback_listeners(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "RegisterEventCallback",
        events::register_event_callback,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "RegisterUnitEventCallback",
        events::register_unit_event_callback,
    )?;
    Ok(())
}

fn register_named_callback_table(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "RegisterCallback",
        callbacks::register_callback,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "UnregisterCallback",
        callbacks::unregister_callback,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "TriggerEvent",
        callbacks::trigger_callback_event,
    )?;
    table_set_rust_fn_static(state, table, "SetupMenu", callbacks::setup_menu)?;
    Ok(())
}

fn register_event_keyboard_propagation(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "SetPropagateKeyboardInput",
        events::set_propagate_keyboard_input,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetPropagateKeyboardInput",
        events::get_propagate_keyboard_input,
    )?;
    Ok(())
}

fn register_event_script_handlers(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "SetScript", events::set_script)?;
    table_set_rust_fn_static(state, table, "GetScript", events::get_script)?;
    table_set_rust_fn_static(state, table, "HasScript", events::has_script)?;
    table_set_rust_fn_static(state, table, "HookScript", events::hook_script)?;
    Ok(())
}
