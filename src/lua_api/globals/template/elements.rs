//! Template element creation: textures, fontstrings, thumb/button textures.

mod texture;

use super::lua_global_ref;
use crate::event::ScriptHandler;
use crate::loader::chunk_cache;
use crate::loader::helpers::generate_scripts_code;
use crate::lua_api::frame::get_sim_state;
use crate::lua_api::script_helpers::set_script;
use mlua::Lua;
pub(super) use texture::{
    AnchorParentContext, DirectTextureCreateContext, append_anchors_and_parent_refs,
    append_key_values, append_texture_properties, apply_deferred_mask_atlases, apply_region_layout,
    apply_region_visibility, apply_texture_visuals_direct, create_texture_from_template,
    create_texture_from_template_direct, resolve_state_frame_id, sync_region_parent_refs,
};

/// Apply scripts from template.
pub(super) fn apply_scripts_from_template(
    lua: &Lua,
    scripts: &crate::xml::ScriptsXml,
    frame_name: &str,
) {
    if apply_method_only_scripts_fast(lua, scripts, frame_name).unwrap_or(false) {
        return;
    }

    let handlers_code = generate_scripts_code(scripts);

    if !handlers_code.is_empty() {
        let frame_ref = lua_global_ref(frame_name);
        let code = format!(
            "\n        local frame = {frame_ref}\n        if frame then\n        {handlers_code}\n        end\n"
        );
        let _ = chunk_cache::exec(lua, &code, "template-elements");
    }
}

fn apply_method_only_scripts_fast(
    lua: &Lua,
    scripts: &crate::xml::ScriptsXml,
    frame_name: &str,
) -> mlua::Result<bool> {
    let Some(frame_id) = resolve_frame_id(lua, frame_name) else {
        return Ok(false);
    };
    let Some(handlers) = collect_method_only_handlers(scripts) else {
        return Ok(false);
    };
    if handlers.is_empty() {
        return Ok(true);
    }

    for (handler_name, method_name) in handlers {
        let func = build_method_handler(lua, method_name)?;
        set_script(lua, frame_id, handler_name, func);
        register_direct_script_handler(lua, frame_id, handler_name);
    }

    Ok(true)
}

fn collect_method_only_handlers(
    scripts: &crate::xml::ScriptsXml,
) -> Option<Vec<(&'static str, &str)>> {
    let mut result = Vec::new();
    collect_method_only_handler_group(&mut result, base_method_only_handlers(scripts))?;
    collect_method_only_handler_group(&mut result, pointer_method_only_handlers(scripts))?;
    collect_method_only_handler_group(&mut result, text_method_only_handlers(scripts))?;
    collect_method_only_handler_group(&mut result, state_method_only_handlers(scripts))?;
    Some(result)
}

type MethodOnlyScript<'a> = (&'static str, Option<&'a crate::xml::ScriptBodyXml>);

fn collect_method_only_handler_group<'a>(
    result: &mut Vec<(&'static str, &'a str)>,
    handlers: impl IntoIterator<Item = MethodOnlyScript<'a>>,
) -> Option<()> {
    for (handler_name, script) in handlers {
        let Some(script) = script else {
            continue;
        };
        if script.intrinsic_order.is_some() || script.inherit.is_some() || script.function.is_some()
        {
            return None;
        }
        let method_name = script.method.as_deref()?;
        if script
            .body
            .as_deref()
            .is_some_and(|body| !body.trim().is_empty())
        {
            return None;
        }
        result.push((handler_name, method_name));
    }
    Some(())
}

fn base_method_only_handlers(scripts: &crate::xml::ScriptsXml) -> [MethodOnlyScript<'_>; 8] {
    [
        ("OnLoad", scripts.on_load.last()),
        ("OnEvent", scripts.on_event.last()),
        ("OnUpdate", scripts.on_update.last()),
        ("OnClick", scripts.on_click.last()),
        ("PreClick", scripts.pre_click.last()),
        ("PostClick", scripts.post_click.last()),
        ("OnShow", scripts.on_show.last()),
        ("OnHide", scripts.on_hide.last()),
    ]
}

fn pointer_method_only_handlers(scripts: &crate::xml::ScriptsXml) -> [MethodOnlyScript<'_>; 8] {
    [
        ("OnEnter", scripts.on_enter.last()),
        ("OnLeave", scripts.on_leave.last()),
        ("OnMouseDown", scripts.on_mouse_down.last()),
        ("OnMouseUp", scripts.on_mouse_up.last()),
        ("OnMouseWheel", scripts.on_mouse_wheel.last()),
        ("OnDragStart", scripts.on_drag_start.last()),
        ("OnDragStop", scripts.on_drag_stop.last()),
        ("OnReceiveDrag", scripts.on_receive_drag.last()),
    ]
}

fn text_method_only_handlers(scripts: &crate::xml::ScriptsXml) -> [MethodOnlyScript<'_>; 10] {
    [
        ("OnEnterPressed", scripts.on_enter_pressed.last()),
        ("OnEscapePressed", scripts.on_escape_pressed.last()),
        ("OnTabPressed", scripts.on_tab_pressed.last()),
        ("OnSpacePressed", scripts.on_space_pressed.last()),
        ("OnTextChanged", scripts.on_text_changed.last()),
        ("OnTextSet", scripts.on_text_set.last()),
        ("OnChar", scripts.on_char.last()),
        ("OnEditFocusGained", scripts.on_edit_focus_gained.last()),
        ("OnEditFocusLost", scripts.on_edit_focus_lost.last()),
        (
            "OnInputLanguageChanged",
            scripts.on_input_language_changed.last(),
        ),
    ]
}

fn state_method_only_handlers(scripts: &crate::xml::ScriptsXml) -> [MethodOnlyScript<'_>; 10] {
    [
        ("OnKeyDown", scripts.on_key_down.last()),
        ("OnKeyUp", scripts.on_key_up.last()),
        ("OnValueChanged", scripts.on_value_changed.last()),
        ("OnEnable", scripts.on_enable.last()),
        ("OnDisable", scripts.on_disable.last()),
        ("OnSizeChanged", scripts.on_size_changed.last()),
        ("OnAttributeChanged", scripts.on_attribute_changed.last()),
        ("OnHyperlinkClick", scripts.on_hyperlink_click.last()),
        ("OnHyperlinkEnter", scripts.on_hyperlink_enter.last()),
        ("OnHyperlinkLeave", scripts.on_hyperlink_leave.last()),
    ]
}

fn build_method_handler(lua: &Lua, method_name: &str) -> mlua::Result<mlua::Function> {
    chunk_cache::load_chunk(
        lua,
        r#"
            local method_name = ...
            return function(self, ...)
                return self[method_name](self, ...)
            end
        "#,
        "template-method-handler",
    )?
    .call(method_name)
}

fn resolve_frame_id(lua: &Lua, frame_name: &str) -> Option<u64> {
    get_sim_state(lua)
        .borrow()
        .widgets
        .get_id_by_name(frame_name)
        .or_else(|| {
            frame_name
                .strip_prefix("__frame_")
                .and_then(|value| value.parse::<u64>().ok())
        })
}

fn register_direct_script_handler(lua: &Lua, frame_id: u64, handler_name: &str) {
    let Some(handler) = ScriptHandler::from_str(handler_name) else {
        return;
    };

    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    state.scripts.set(frame_id, handler, 1);
    if handler == ScriptHandler::OnUpdate || handler == ScriptHandler::OnPostUpdate {
        state.on_update_frames.insert(frame_id);
        state.visible_on_update_cache = None;
    }
}

pub(super) fn apply_missing_scripts_from_template(
    lua: &Lua,
    scripts: &crate::xml::ScriptsXml,
    frame_name: &str,
) {
    let mut handlers_code = String::new();
    append_missing_method_handler(
        &mut handlers_code,
        "OnDragStart",
        scripts.on_drag_start.last(),
    );
    append_missing_method_handler(
        &mut handlers_code,
        "OnDragStop",
        scripts.on_drag_stop.last(),
    );
    append_missing_method_handler(
        &mut handlers_code,
        "OnReceiveDrag",
        scripts.on_receive_drag.last(),
    );
    if !handlers_code.is_empty() {
        let frame_ref = lua_global_ref(frame_name);
        let code = format!(
            "
        local frame = {frame_ref}
        if frame then
        {handlers_code}
        end
"
        );
        let _ = chunk_cache::exec(lua, &code, "template-elements");
    }
}

fn append_missing_method_handler(
    code: &mut String,
    handler_name: &str,
    script: Option<&crate::xml::ScriptBodyXml>,
) {
    let Some(method) = script.and_then(|script| script.method.as_deref()) else {
        return;
    };
    code.push_str(&format!(
        "if frame:GetScript(\"{handler_name}\") == nil then frame:SetScript(\"{handler_name}\", function(self, ...) self:{method}(...) end) end
"
    ));
}
