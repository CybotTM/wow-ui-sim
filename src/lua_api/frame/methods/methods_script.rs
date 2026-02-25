//! Script handler methods: SetScript, GetScript, HookScript, HasScript, etc.

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::get_sim_state;
use crate::lua_api::script_helpers::{get_scripts_table, remove_script, set_script};
use mlua::Value;

/// Add script handler methods to the shared methods table.
pub fn add_script_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_script_methods(methods);
    add_get_script_method(methods);
    add_hook_and_wrap_methods(methods);
    add_clear_scripts_method(methods);
    add_has_script_method(methods);
}

fn add_set_script_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_script(methods);
    add_set_on_click_handler(methods);
}

fn add_set_script<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetScript", |lua, this, (handler, func): (String, Value)| {
        let id = this.0;
        let h = validate_script_handler(&handler)?;
        if let Value::Function(f) = func {
            set_script(lua, id, &handler, f);
            register_script_handler(lua, id, h);
        } else {
            remove_script(lua, id, &handler);
            deregister_script_handler(lua, id, h);
        }
        Ok(())
    });
}

fn validate_script_handler(handler: &str) -> mlua::Result<crate::event::ScriptHandler> {
    crate::event::ScriptHandler::from_str(handler)
        .ok_or_else(|| mlua::Error::RuntimeError(format!(
            "SetScript: `{}' is not a valid script handler for this widget type", handler
        )))
}

fn register_script_handler(lua: &mlua::Lua, id: u64, h: crate::event::ScriptHandler) {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    state.scripts.set(id, h, 1);
    if h == crate::event::ScriptHandler::OnUpdate || h == crate::event::ScriptHandler::OnPostUpdate {
        state.on_update_frames.insert(id);
        state.visible_on_update_cache = None;
    }
}

fn deregister_script_handler(lua: &mlua::Lua, id: u64, h: crate::event::ScriptHandler) {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    state.scripts.remove(id, h);
    if h == crate::event::ScriptHandler::OnUpdate || h == crate::event::ScriptHandler::OnPostUpdate {
        state.on_update_frames.remove(&id);
        state.visible_on_update_cache = None;
    }
}

fn add_set_on_click_handler<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetOnClickHandler", |lua, this, func: Value| {
        let id = this.0;
        if let Value::Function(f) = func {
            set_script(lua, id, "OnClick", f);
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            state.scripts.set(id, crate::event::ScriptHandler::OnClick, 1);
        }
        Ok(())
    });
}

fn add_get_script_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetScript", |lua, this, handler: String| {
        match crate::lua_api::script_helpers::get_script(lua, this.0, &handler) {
            Some(f) => Ok(Value::Function(f)),
            None => Ok(Value::Nil),
        }
    });
}

fn add_hook_and_wrap_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_hook_script(methods);
    methods.add_method("WrapScript", |_lua, _this, (_target, _script, _pre_body): (Value, String, String)| Ok(()));
    methods.add_method("UnwrapScript", |_lua, _this, (_target, _script): (Value, String)| Ok(()));
}

fn add_hook_script<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("HookScript", |lua, this, (handler, func): (String, Value)| {
        let id = this.0;
        if let Value::Function(hook_fn) = func {
            let combined = build_hooked_function(lua, id, &handler, hook_fn)?;
            set_script(lua, id, &handler, combined);
            if let Some(h) = crate::event::ScriptHandler::from_str(&handler) {
                register_script_handler(lua, id, h);
            }
        }
        Ok(Value::Boolean(true))
    });
}

fn build_hooked_function(
    lua: &mlua::Lua,
    id: u64,
    handler: &str,
    hook_fn: mlua::Function,
) -> mlua::Result<mlua::Function> {
    let old = crate::lua_api::script_helpers::get_script(lua, id, handler);
    match old {
        Some(old_fn) => lua.load(r#"
            local old, hook = ...
            return function(...)
                old(...)
                hook(...)
            end
        "#).call::<mlua::Function>((old_fn, hook_fn)),
        None => Ok(hook_fn),
    }
}

fn remove_keys_with_prefix(table: &mlua::Table, prefix: &str) {
    let keys: Vec<String> = table
        .pairs::<String, Value>()
        .filter_map(|pair| {
            if let Ok((k, _)) = pair && k.starts_with(prefix) {
                return Some(k);
            }
            None
        })
        .collect();
    for key in keys {
        let _ = table.set(key.as_str(), Value::Nil);
    }
}

fn add_clear_scripts_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("ClearScripts", |lua, this, ()| {
        let id = this.0;
        let prefix = format!("{}_", id);
        if let Some(table) = get_scripts_table(lua) {
            remove_keys_with_prefix(&table, &prefix);
        }
        if let Ok(table) = lua.named_registry_value::<mlua::Table>("__script_hooks") {
            remove_keys_with_prefix(&table, &prefix);
        }
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        state.scripts.remove_all(id);
        if state.on_update_frames.remove(&id) {
            state.visible_on_update_cache = None;
        }
        Ok(())
    });
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

const BUTTON_SCRIPTS: &[&str] = &[
    "OnClick", "PreClick", "PostClick", "OnPostClick", "OnDoubleClick",
];
const EDITBOX_SCRIPTS: &[&str] = &[
    "OnTextChanged", "OnCursorChanged", "OnEditFocusGained", "OnEditFocusLost",
    "OnEnterPressed", "OnEscapePressed", "OnTabPressed", "OnSpacePressed",
    "OnInputLanguageChanged", "OnChar",
];
const RANGE_SCRIPTS: &[&str] = &["OnValueChanged", "OnMinMaxChanged"];
const SCROLL_SCRIPTS: &[&str] = &[
    "OnHorizontalScroll", "OnVerticalScroll", "OnScrollRangeChanged",
];
const TOOLTIP_SCRIPTS: &[&str] = &[
    "OnTooltipSetDefaultAnchor", "OnTooltipSetItem", "OnTooltipSetUnit",
    "OnTooltipSetSpell", "OnTooltipCleared",
];
const MODEL_SCRIPTS: &[&str] = &["OnModelLoaded", "OnAnimFinished"];
const COOLDOWN_SCRIPTS: &[&str] = &["OnCooldownDone"];

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

fn script_supported(widget_type: crate::widget::WidgetType, script_type: &str) -> bool {
    let in_base = BASE_SCRIPTS.iter().any(|s| s.eq_ignore_ascii_case(script_type));
    if in_base { return true; }
    extra_scripts_for_type(widget_type)
        .iter()
        .any(|s| s.eq_ignore_ascii_case(script_type))
}

fn add_has_script_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("HasScript", |lua, this, script_type: String| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let widget_type = state.widgets.get(this.0)
            .map(|f| f.widget_type)
            .unwrap_or(crate::widget::WidgetType::Frame);
        Ok(script_supported(widget_type, &script_type))
    });
}
