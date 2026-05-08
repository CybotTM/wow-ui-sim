//! EditBox method registration.

use super::*;
use crate::lua_bridge::table_set_rust_fn;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::table::Table;

const EDITBOX_METHODS: &[(&str, rilua::vm::closure::RustFn)] = &[
    ("SetFocus", set_focus),
    ("ClearFocus", clear_focus),
    ("HasFocus", has_focus),
    ("HasText", has_text),
    ("SetCursorPosition", set_cursor_position),
    ("GetCursorPosition", get_cursor_position),
    ("GetNumLetters", get_num_letters),
    ("SetMaxLetters", set_max_letters),
    ("GetMaxLetters", get_max_letters),
    ("SetMultiLine", set_multi_line),
    ("IsMultiLine", is_multi_line),
    ("SetAutoFocus", set_auto_focus),
    ("IsAutoFocus", is_auto_focus),
    ("SetNumeric", set_numeric),
    ("IsNumeric", is_numeric),
    ("SetAlphabeticOnly", set_alphabetic_only),
    ("IsAlphabeticOnly", is_alphabetic_only),
    ("SetNumericFullRange", set_numeric_full_range),
    ("IsNumericFullRange", is_numeric_full_range),
    ("SetPassword", set_password),
    ("IsPassword", is_password),
    ("SetSecureText", set_secure_text),
    ("IsSecureText", is_secure_text),
    ("SetCountInvisibleLetters", set_count_invisible_letters),
    ("IsCountInvisibleLetters", is_count_invisible_letters),
    ("SetSecurityDisableSetText", set_security_disable_set_text),
    ("SetNumber", set_number),
    ("GetNumber", get_number),
    ("AddHistoryLine", add_history_line),
    ("GetHistoryLines", get_history_lines),
    ("SetHistoryLines", set_history_lines),
    ("ClearHistory", clear_history),
    ("GetInputLanguage", get_input_language),
    ("ToggleInputLanguage", toggle_input_language),
    ("ResetInputMode", reset_input_mode),
    ("SetTextInsets", set_text_insets),
    ("SetSpacing", set_spacing),
    ("GetSpacing", get_spacing),
    ("GetTextInsets", get_text_insets),
    ("GetDisplayText", get_display_text),
    ("SetVisibleTextByteLimit", set_visible_text_byte_limit),
    ("GetVisibleTextByteLimit", get_visible_text_byte_limit),
    ("SetSecurityDisablePaste", set_security_disable_paste),
    ("SetHighlightColor", set_highlight_color),
    ("GetHighlightColor", get_highlight_color),
    ("IsInIMECompositionMode", is_in_ime_composition_mode),
    ("GetUTF8CursorPosition", get_utf8_cursor_position),
    ("SetDesiredWidth", set_desired_width),
    ("GetDesiredWidth", get_desired_width),
    ("GetScaledDesiredWidth", get_scaled_desired_width),
    ("GetDesiredHeight", get_desired_height),
    ("GetScaledDesiredHeight", get_scaled_desired_height),
    ("UpdateWidth", update_width),
    ("OnTextScaleUpdated", on_text_scale_updated),
    ("Insert", insert),
    ("SetBlinkSpeed", set_blink_speed),
    ("GetBlinkSpeed", get_blink_speed),
    ("SetAltArrowKeyMode", set_alt_arrow_key_mode),
    ("GetAltArrowKeyMode", get_alt_arrow_key_mode),
    ("HighlightText", selection::highlight_text),
    ("ClearHighlightText", selection::clear_highlight_text),
];

pub(super) fn register_editbox(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    for (name, func) in EDITBOX_METHODS {
        table_set_rust_fn(state, metatable, name, *func)?;
    }
    Ok(())
}
