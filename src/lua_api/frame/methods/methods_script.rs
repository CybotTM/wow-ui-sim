//! Script handler methods: SetScript, GetScript, HookScript, HasScript, etc.

use crate::lua_api::frame::handle::{get_sim_state, lud_to_id};
use crate::lua_api::script_helpers::{get_scripts_table, remove_script, set_script};
use mlua::{LightUserData, Lua, Value};

/// Add script handler methods to the shared methods table.
pub fn add_script_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    add_set_script_methods(lua, methods)?;
    add_get_script_method(lua, methods)?;
    add_hook_and_wrap_methods(lua, methods)?;
    add_clear_scripts_method(lua, methods)?;
    add_has_script_method(lua, methods)?;
    Ok(())
}

/// SetScript(handler, func) and SetOnClickHandler(func)
fn add_set_script_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("SetScript", lua.create_function(|lua, (ud, handler, func): (LightUserData, String, Value)| {
        let id = lud_to_id(ud);
        let handler_type = crate::event::ScriptHandler::from_str(&handler);

        let h = match handler_type {
            Some(h) => h,
            None => return Err(mlua::Error::RuntimeError(format!(
                "SetScript: `{}' is not a valid script handler for this widget type",
                handler
            ))),
        };

        if let Value::Function(f) = func {
            set_script(lua, id, &handler, f);

            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            state.scripts.set(id, h, 1);

            if h == crate::event::ScriptHandler::OnUpdate || h == crate::event::ScriptHandler::OnPostUpdate {
                state.on_update_frames.insert(id);
                state.visible_on_update_cache = None;
            }
        } else {
            // nil func: remove the handler
            remove_script(lua, id, &handler);

            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            state.scripts.remove(id, h);

            if h == crate::event::ScriptHandler::OnUpdate || h == crate::event::ScriptHandler::OnPostUpdate {
                state.on_update_frames.remove(&id);
                state.visible_on_update_cache = None;
            }
        }
        Ok(())
    })?)?;

    // SetOnClickHandler(func) - WoW 10.0+ convenience for setting OnClick
    methods.set("SetOnClickHandler", lua.create_function(|lua, (ud, func): (LightUserData, Value)| {
        let id = lud_to_id(ud);
        if let Value::Function(f) = func {
            set_script(lua, id, "OnClick", f);

            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            state
                .scripts
                .set(id, crate::event::ScriptHandler::OnClick, 1);
        }
        Ok(())
    })?)?;

    Ok(())
}

/// GetScript(handler) - retrieve a stored script handler function.
fn add_get_script_method(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetScript", lua.create_function(|lua, (ud, handler): (LightUserData, String)| {
        let id = lud_to_id(ud);
        match crate::lua_api::script_helpers::get_script(lua, id, &handler) {
            Some(f) => Ok(Value::Function(f)),
            None => Ok(Value::Nil),
        }
    })?)?;

    Ok(())
}

/// HookScript, WrapScript, UnwrapScript - script chaining methods.
fn add_hook_and_wrap_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("HookScript", lua.create_function(|lua, (ud, handler, func): (LightUserData, String, Value)| {
        let id = lud_to_id(ud);
        if let Value::Function(hook_fn) = func {
            let old = crate::lua_api::script_helpers::get_script(lua, id, &handler);
            let combined = match old {
                Some(old_fn) => {
                    lua.load(r#"
                        local old, hook = ...
                        return function(...)
                            old(...)
                            hook(...)
                        end
                    "#).call::<mlua::Function>((old_fn, hook_fn))?
                }
                None => hook_fn,
            };
            set_script(lua, id, &handler, combined);

            if let Some(h) = crate::event::ScriptHandler::from_str(&handler) {
                let state_rc = get_sim_state(lua);
                let mut state = state_rc.borrow_mut();
                state.scripts.set(id, h, 1);
                if h == crate::event::ScriptHandler::OnUpdate || h == crate::event::ScriptHandler::OnPostUpdate {
                    state.on_update_frames.insert(id);
                    state.visible_on_update_cache = None;
                }
            }
        }
        Ok(Value::Boolean(true))
    })?)?;

    // WrapScript - stub for secure script wrapping
    methods.set("WrapScript", lua.create_function(|_, (_ud, _target, _script, _pre_body): (LightUserData, Value, String, String)| {
        Ok(())
    })?)?;

    // UnwrapScript - stub for removing script wrapping
    methods.set("UnwrapScript", lua.create_function(|_, (_ud, _target, _script): (LightUserData, Value, String)| {
        Ok(())
    })?)?;

    Ok(())
}

/// Remove matching keys from a Lua table by prefix.
fn remove_keys_with_prefix(table: &mlua::Table, prefix: &str) {
    let keys: Vec<String> = table
        .pairs::<String, Value>()
        .filter_map(|pair| {
            if let Ok((k, _)) = pair
                && k.starts_with(prefix) {
                    return Some(k);
                }
            None
        })
        .collect();
    for key in keys {
        let _ = table.set(key.as_str(), Value::Nil);
    }
}

/// ClearScripts() - remove all script handlers for this frame.
fn add_clear_scripts_method(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("ClearScripts", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let prefix = format!("{}_", id);

        if let Some(table) = get_scripts_table(lua) {
            remove_keys_with_prefix(&table, &prefix);
        }

        // Also clear from hooks table
        let hooks_table: Option<mlua::Table> =
            lua.named_registry_value("__script_hooks").ok();
        if let Some(table) = hooks_table {
            remove_keys_with_prefix(&table, &prefix);
        }

        // Clear script entries in state
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        state.scripts.remove_all(id);
        if state.on_update_frames.remove(&id) {
            state.visible_on_update_cache = None;
        }

        Ok(())
    })?)?;

    Ok(())
}

/// Base script handlers supported by all frame types.
const BASE_SCRIPTS: &[&str] = &[
    "OnShow", "OnHide", "OnUpdate", "OnEvent", "OnSizeChanged",
    "OnMouseDown", "OnMouseUp", "OnMouseWheel",
    "OnEnter", "OnLeave", "OnDragStart", "OnDragStop", "OnReceiveDrag",
    "OnKeyDown", "OnKeyUp", "OnAttributeChanged", "OnLoad",
    "OnEnable", "OnDisable",
    "OnPostUpdate", "OnPostShow", "OnPostHide",
];

/// Script handlers added by Button / CheckButton.
const BUTTON_SCRIPTS: &[&str] = &[
    "OnClick", "PreClick", "PostClick", "OnPostClick", "OnDoubleClick",
];

/// Script handlers added by EditBox.
const EDITBOX_SCRIPTS: &[&str] = &[
    "OnTextChanged", "OnCursorChanged", "OnEditFocusGained", "OnEditFocusLost",
    "OnEnterPressed", "OnEscapePressed", "OnTabPressed", "OnSpacePressed",
    "OnInputLanguageChanged", "OnChar",
];

/// Script handlers added by Slider and StatusBar.
const RANGE_SCRIPTS: &[&str] = &["OnValueChanged", "OnMinMaxChanged"];

/// Script handlers added by ScrollFrame.
const SCROLL_SCRIPTS: &[&str] = &[
    "OnHorizontalScroll", "OnVerticalScroll", "OnScrollRangeChanged",
];

/// Script handlers added by GameTooltip.
const TOOLTIP_SCRIPTS: &[&str] = &[
    "OnTooltipSetDefaultAnchor", "OnTooltipSetItem", "OnTooltipSetUnit",
    "OnTooltipSetSpell", "OnTooltipCleared",
];

/// Script handlers added by Model / PlayerModel.
const MODEL_SCRIPTS: &[&str] = &["OnModelLoaded", "OnAnimFinished"];

/// Script handlers added by Cooldown.
const COOLDOWN_SCRIPTS: &[&str] = &["OnCooldownDone"];

/// Return extra (non-base) handlers for the given widget type. Empty for plain frames.
fn extra_scripts_for_type(widget_type: crate::widget::WidgetType) -> &'static [&'static str] {
    use crate::widget::WidgetType::*;
    match widget_type {
        Button | CheckButton => BUTTON_SCRIPTS,
        EditBox => EDITBOX_SCRIPTS,
        Slider | StatusBar => RANGE_SCRIPTS,
        ScrollFrame => SCROLL_SCRIPTS,
        GameTooltip => TOOLTIP_SCRIPTS,
        Model | PlayerModel | ModelScene => MODEL_SCRIPTS,
        Cooldown => COOLDOWN_SCRIPTS,
        _ => &[],
    }
}

/// Return whether `script_type` is valid for the given widget type.
///
/// Matches WoW client behaviour: `HasScript("OnClick")` is false on a plain Frame.
fn script_supported(widget_type: crate::widget::WidgetType, script_type: &str) -> bool {
    let in_base = BASE_SCRIPTS.iter().any(|s| s.eq_ignore_ascii_case(script_type));
    if in_base {
        return true;
    }
    extra_scripts_for_type(widget_type)
        .iter()
        .any(|s| s.eq_ignore_ascii_case(script_type))
}

/// HasScript(scriptType) - check if frame supports a script handler type.
fn add_has_script_method(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("HasScript", lua.create_function(|lua, (ud, script_type): (LightUserData, String)| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let widget_type = state.widgets.get(id)
            .map(|f| f.widget_type)
            .unwrap_or(crate::widget::WidgetType::Frame);
        Ok(script_supported(widget_type, &script_type))
    })?)?;

    Ok(())
}
