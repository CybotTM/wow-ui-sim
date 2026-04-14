//! Template element creation: textures, fontstrings, thumb/button textures.

use crate::event::ScriptHandler;
use crate::loader::chunk_cache;
use crate::loader::helpers::{generate_scripts_code, generate_set_point_code};
use crate::loader::helpers_anim::generate_animation_group_code;
use crate::lua_api::SimState;
use crate::lua_api::frame::get_sim_state;
use crate::lua_api::frame::methods::{apply_atlas_to_frame, register_child_widget};
use crate::lua_api::frame::{frame_ref, sync_child_to_lua};
use crate::lua_api::script_helpers::set_script;
use crate::render::BlendMode;
use crate::widget::{Color, Frame, Gradient, WidgetType};
use mlua::Lua;
use std::cell::RefCell;
use std::rc::Rc;

use super::{escape_lua_string, get_size_values, lua_global_ref, rand_id};

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
    let handlers = [
        ("OnLoad", scripts.on_load.last()),
        ("OnEvent", scripts.on_event.last()),
        ("OnUpdate", scripts.on_update.last()),
        ("OnClick", scripts.on_click.last()),
        ("PreClick", scripts.pre_click.last()),
        ("PostClick", scripts.post_click.last()),
        ("OnShow", scripts.on_show.last()),
        ("OnHide", scripts.on_hide.last()),
        ("OnEnter", scripts.on_enter.last()),
        ("OnLeave", scripts.on_leave.last()),
        ("OnMouseDown", scripts.on_mouse_down.last()),
        ("OnMouseUp", scripts.on_mouse_up.last()),
        ("OnMouseWheel", scripts.on_mouse_wheel.last()),
        ("OnDragStart", scripts.on_drag_start.last()),
        ("OnDragStop", scripts.on_drag_stop.last()),
        ("OnReceiveDrag", scripts.on_receive_drag.last()),
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
    ];

    let mut result = Vec::new();
    for (handler_name, script) in handlers {
        let Some(script) = script else {
            continue;
        };
        if script.intrinsic_order.is_some() || script.inherit.is_some() || script.function.is_some()
        {
            return None;
        }
        let Some(method_name) = script.method.as_deref() else {
            return None;
        };
        if script
            .body
            .as_deref()
            .is_some_and(|body| !body.trim().is_empty())
        {
            return None;
        }
        result.push((handler_name, method_name));
    }
    Some(result)
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

/// Create a texture from template XML.
///
/// `parent_name` is the actual Lua frame name (for parent reference).
/// `subst_parent` is the name used for `$parent` substitution in child names
/// (propagated through anonymous frames to the nearest named ancestor).
pub(super) fn create_texture_from_template(
    lua: &Lua,
    texture: &crate::xml::TextureXml,
    parent_name: &str,
    subst_parent: &str,
    draw_layer: &str,
    is_mask: bool,
    is_line: bool,
) {
    let resolved = crate::xml::resolve_texture_inheritance(texture);
    let child_name = resolved
        .name
        .as_ref()
        .map(|n| n.replace("$parent", subst_parent))
        .unwrap_or_else(|| format!("__tex_{}", rand_id()));

    #[cfg(test)]
    super::test_counters::record_texture_create();

    let code = build_template_texture_lua(
        &resolved,
        parent_name,
        &child_name,
        draw_layer,
        is_mask,
        is_line,
    );
    if let Err(e) = chunk_cache::exec(lua, &code, "template-elements") {
        eprintln!(
            "[create_texture] failed for '{}' on '{}': {}",
            child_name, parent_name, e
        );
    }
    apply_texture_animations(lua, &resolved, &child_name);
}

pub(super) fn create_texture_from_template_direct(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    texture: &crate::xml::TextureXml,
    parent_name: &str,
    subst_parent: &str,
    draw_layer: &str,
    is_mask: bool,
    is_line: bool,
) -> mlua::Result<()> {
    let resolved = crate::xml::resolve_texture_inheritance(texture);
    ensure_direct_texture_supported(&resolved)?;

    let parent_id = resolve_state_frame_id(state, parent_name).ok_or_else(|| {
        mlua::Error::runtime(format!(
            "missing parent '{}' for direct texture create",
            parent_name
        ))
    })?;
    let child_name = resolved
        .name
        .as_ref()
        .map(|n| n.replace("$parent", subst_parent))
        .unwrap_or_else(|| format!("__tex_{}", rand_id()));

    let widget_type = if is_line {
        WidgetType::Line
    } else {
        WidgetType::Texture
    };
    let mut child = Frame::new(widget_type, Some(child_name.clone()), Some(parent_id));
    if let Some(layer) = crate::widget::DrawLayer::from_str(draw_layer) {
        child.draw_layer = layer;
    }
    if let Some(parent_key) = resolved.parent_key.as_ref() {
        child.parent_key = Some(parent_key.clone());
    }
    if is_mask {
        child.is_mask = true;
        child.object_type_name = Some("MaskTexture".to_string());
    }
    let child_id = child.id;
    register_child_widget(lua, parent_id, child, &Some(child_name))?;
    sync_region_parent_refs(
        lua,
        state,
        parent_id,
        child_id,
        resolved.parent_key.as_deref(),
        resolved.parent_array.as_deref(),
    )?;
    apply_texture_visuals_direct(state, child_id, &resolved, is_mask, is_line);
    apply_region_layout(
        state,
        child_id,
        &resolved.anchors,
        resolved.set_all_points,
        parent_name,
        resolved.anchors.is_none() && resolved.set_all_points != Some(true),
    );
    apply_region_visibility(state, child_id, resolved.hidden, resolved.alpha);
    apply_mask_wiring_direct(state, parent_id, child_id, &resolved)?;
    Ok(())
}

pub(super) fn resolve_state_frame_id(
    state: &Rc<RefCell<SimState>>,
    frame_name: &str,
) -> Option<u64> {
    state
        .borrow()
        .widgets
        .get_id_by_name(frame_name)
        .or_else(|| {
            frame_name
                .strip_prefix("__frame_")
                .and_then(|suffix| suffix.parse::<u64>().ok())
        })
}

pub(super) fn sync_region_parent_refs(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    parent_id: u64,
    child_id: u64,
    parent_key: Option<&str>,
    parent_array: Option<&str>,
) -> mlua::Result<()> {
    if let Some(parent_key) = parent_key {
        {
            let mut sim = state.borrow_mut();
            if let Some(parent) = sim.widgets.get_mut_visual(parent_id) {
                parent
                    .children_keys
                    .insert(parent_key.to_string(), child_id);
            }
        }
        sync_child_to_lua(lua, parent_id, parent_key, child_id)?;
    }

    let Some(parent_array) = parent_array else {
        return Ok(());
    };
    let parent_val = frame_ref(lua, parent_id)?;
    let child_val = frame_ref(lua, child_id)?;
    let mlua::Value::UserData(parent_ud) = parent_val else {
        return Ok(());
    };
    let fields: mlua::Table = parent_ud.user_value()?;
    let array = match fields.raw_get::<mlua::Value>(parent_array)? {
        mlua::Value::Table(existing) => existing,
        _ => {
            let created = lua.create_table()?;
            fields.raw_set(parent_array, created.clone())?;
            created
        }
    };
    let next_index = array.raw_len() + 1;
    array.raw_set(next_index, child_val)?;
    Ok(())
}

pub(super) fn apply_region_layout(
    state: &Rc<RefCell<SimState>>,
    region_id: u64,
    anchors: &Option<crate::xml::AnchorsXml>,
    set_all_points: Option<bool>,
    parent_name: &str,
    default_fill_parent: bool,
) {
    if let Some(frame) = state.borrow_mut().widgets.get_mut_visual(region_id) {
        frame.clear_all_points();
    }

    if let Some(anchors) = anchors {
        let mut sim = state.borrow_mut();
        for anchor in &anchors.anchors {
            super::direct::set_single_anchor(&mut sim, region_id, anchor, parent_name);
        }
    }
    if set_all_points == Some(true) || default_fill_parent {
        set_region_all_points(state, region_id);
    }
}

pub(super) fn apply_region_visibility(
    state: &Rc<RefCell<SimState>>,
    region_id: u64,
    hidden: Option<bool>,
    alpha: Option<f32>,
) {
    if hidden == Some(true) {
        state.borrow_mut().set_frame_visible(region_id, false);
    }
    if let Some(alpha) = alpha {
        super::direct::set_alpha(state, region_id, alpha);
    }
}

pub(super) fn apply_texture_visuals_direct(
    state: &Rc<RefCell<SimState>>,
    texture_id: u64,
    texture: &crate::xml::TextureXml,
    is_mask: bool,
    is_line: bool,
) {
    {
        let mut sim = state.borrow_mut();
        if let Some(frame) = sim.widgets.get_mut_visual(texture_id) {
            apply_texture_size_direct(frame, texture);
            if is_line && let Some(thickness) = texture.thickness {
                frame.line_thickness = thickness;
            }
            apply_texture_source_direct(frame, texture, is_mask);
            apply_texture_color_direct(frame, texture);
            apply_texture_gradient_direct(frame, texture.gradient.as_ref());
            if texture.horiz_tile == Some(true) {
                frame.horiz_tile = true;
            }
            if texture.vert_tile == Some(true) {
                frame.vert_tile = true;
            }
            if let Some(mode) = texture.effective_blend_mode() {
                frame.alpha_mode = Some(mode.to_string());
                frame.blend_mode = if mode.eq_ignore_ascii_case("ADD") {
                    BlendMode::Additive
                } else {
                    BlendMode::Alpha
                };
            }
        }
    }

    if let Some(atlas_name) = texture.atlas.as_deref()
        && let Some(lookup) = crate::atlas::get_render_atlas_info(atlas_name)
    {
        let mut sim = state.borrow_mut();
        apply_atlas_to_frame(
            &mut sim.widgets,
            texture_id,
            lookup.info,
            atlas_name,
            &lookup,
            texture.use_atlas_size.unwrap_or(is_mask),
        );
    }
}

fn ensure_direct_texture_supported(texture: &crate::xml::TextureXml) -> mlua::Result<()> {
    if !crate::xml::collect_texture_mixins(texture).is_empty() {
        return Err(mlua::Error::runtime(
            "direct texture create does not support texture mixins yet",
        ));
    }
    if texture.animations.is_some() {
        return Err(mlua::Error::runtime(
            "direct texture create does not support texture animations yet",
        ));
    }
    if texture
        .key_values
        .as_ref()
        .is_some_and(|values| !values.values.is_empty())
    {
        return Err(mlua::Error::runtime(
            "direct texture create does not support texture key values yet",
        ));
    }
    if texture
        .color
        .as_ref()
        .is_some_and(|color| color.color.is_some())
    {
        return Err(mlua::Error::runtime(
            "direct texture create does not support named color refs yet",
        ));
    }
    Ok(())
}

fn apply_texture_size_direct(frame: &mut Frame, texture: &crate::xml::TextureXml) {
    let Some(size) = texture.size.as_ref() else {
        return;
    };
    let (width, height) = super::get_size_values(size);
    match (width, height) {
        (Some(width), Some(height)) => frame.set_size(width, height),
        (Some(width), None) => frame.width = width,
        (None, Some(height)) => frame.height = height,
        (None, None) => {}
    }
}

fn apply_texture_source_direct(frame: &mut Frame, texture: &crate::xml::TextureXml, is_mask: bool) {
    if let Some(file) = texture.file.as_ref() {
        frame.texture = Some(file.clone());
    }
    if let Some(atlas_name) = texture.atlas.as_ref() {
        if crate::atlas::get_render_atlas_info(atlas_name).is_none() {
            frame.atlas = Some(atlas_name.clone());
            if texture.use_atlas_size.unwrap_or(is_mask) {
                frame.width = 0.0;
                frame.height = 0.0;
            }
        }
    }
    if let Some(tc) = texture.tex_coords.as_ref() {
        frame.tex_coords = Some((
            tc.left.unwrap_or(0.0),
            tc.right.unwrap_or(1.0),
            tc.top.unwrap_or(0.0),
            tc.bottom.unwrap_or(1.0),
        ));
    }
}

fn apply_texture_color_direct(frame: &mut Frame, texture: &crate::xml::TextureXml) {
    let Some(color) = texture.color.as_ref() else {
        return;
    };
    let rgba = Color::new(
        color.r.unwrap_or(1.0),
        color.g.unwrap_or(1.0),
        color.b.unwrap_or(1.0),
        color.a.unwrap_or(1.0),
    );
    if texture.file.is_some() || texture.atlas.is_some() {
        frame.vertex_color = Some(rgba);
    } else {
        frame.color_texture = Some(rgba);
        frame.texture = None;
        frame.texture_file_data_id = None;
    }
}

fn apply_texture_gradient_direct(frame: &mut Frame, gradient: Option<&crate::xml::GradientXml>) {
    let Some(gradient) = gradient else {
        return;
    };
    let min = gradient.min_color.as_ref();
    let max = gradient.max_color.as_ref();
    frame.gradient = Some(Gradient {
        vertical: gradient
            .orientation
            .as_deref()
            .unwrap_or("VERTICAL")
            .eq_ignore_ascii_case("VERTICAL"),
        min_color: Color::new(
            min.and_then(|c| c.r).unwrap_or(0.0),
            min.and_then(|c| c.g).unwrap_or(0.0),
            min.and_then(|c| c.b).unwrap_or(0.0),
            min.and_then(|c| c.a).unwrap_or(1.0),
        ),
        max_color: Color::new(
            max.and_then(|c| c.r).unwrap_or(0.0),
            max.and_then(|c| c.g).unwrap_or(0.0),
            max.and_then(|c| c.b).unwrap_or(0.0),
            max.and_then(|c| c.a).unwrap_or(1.0),
        ),
    });
}

fn set_region_all_points(state: &Rc<RefCell<SimState>>, region_id: u64) {
    let parent_id = state
        .borrow()
        .widgets
        .get(region_id)
        .and_then(|frame| frame.parent_id);
    let mut sim = state.borrow_mut();
    if let Some(frame) = sim.widgets.get_mut_visual(region_id) {
        frame.clear_all_points();
        frame.set_point(
            crate::widget::AnchorPoint::TopLeft,
            parent_id.map(|id| id as usize),
            crate::widget::AnchorPoint::TopLeft,
            0.0,
            0.0,
        );
        frame.set_point(
            crate::widget::AnchorPoint::BottomRight,
            parent_id.map(|id| id as usize),
            crate::widget::AnchorPoint::BottomRight,
            0.0,
            0.0,
        );
    }
    sim.widgets.mark_rect_dirty(region_id);
}

fn apply_mask_wiring_direct(
    state: &Rc<RefCell<SimState>>,
    parent_id: u64,
    mask_id: u64,
    texture: &crate::xml::TextureXml,
) -> mlua::Result<()> {
    let Some(masked) = texture.masked_textures.as_ref() else {
        return Ok(());
    };
    let mut sim = state.borrow_mut();
    for entry in &masked.entries {
        let Some(key) = entry.child_key.as_deref() else {
            continue;
        };
        let target_id = resolve_mask_target_id(&sim, parent_id, key).ok_or_else(|| {
            mlua::Error::runtime(format!(
                "direct texture create could not resolve mask childKey '{}'",
                key
            ))
        })?;
        let Some(frame) = sim.widgets.get_mut_visual(target_id) else {
            continue;
        };
        let already_masked = frame
            .mask_textures
            .iter()
            .any(|existing| *existing == mask_id);
        if !already_masked {
            frame.mask_textures.push(mask_id);
        }
    }
    Ok(())
}

fn resolve_mask_target_id(state: &SimState, parent_id: u64, key: &str) -> Option<u64> {
    let mut current_id = parent_id;
    for segment in key.split('.') {
        let frame = state.widgets.get(current_id)?;
        current_id = *frame.children_keys.get(segment)?;
    }
    Some(current_id)
}

/// Build Lua code that creates and configures a texture from a template.
fn build_template_texture_lua(
    texture: &crate::xml::TextureXml,
    parent_name: &str,
    child_name: &str,
    draw_layer: &str,
    is_mask: bool,
    is_line: bool,
) -> String {
    let mut code = start_template_texture_lua(
        parent_name,
        child_name,
        draw_layer,
        template_texture_create_method(is_mask, is_line),
    );
    append_template_texture_mixins(&mut code, texture);
    append_line_texture_options(&mut code, texture, is_line);
    append_texture_properties(&mut code, texture, "tex", is_mask);
    append_anchors_and_parent_refs(
        &mut code,
        &texture.anchors,
        texture.set_all_points,
        AnchorParentContext {
            parent_key: &texture.parent_key,
            parent_array: &texture.parent_array,
            var: "tex",
            parent_var: "parent",
            parent_name,
        },
    );
    append_template_texture_defaults(&mut code, texture);
    append_mask_wiring(&mut code, is_mask, texture);
    code.push_str("end\n");
    code
}

fn template_texture_create_method(is_mask: bool, is_line: bool) -> &'static str {
    if is_line {
        "CreateLine"
    } else if is_mask {
        "CreateMaskTexture"
    } else {
        "CreateTexture"
    }
}

fn start_template_texture_lua(
    parent_name: &str,
    child_name: &str,
    draw_layer: &str,
    create_method: &str,
) -> String {
    format!(
        "local parent = {}\nif parent then\nlocal tex = parent:{}(\"{}\", \"{}\")\n",
        lua_global_ref(parent_name),
        create_method,
        escape_lua_string(child_name),
        draw_layer,
    )
}

fn append_line_texture_options(code: &mut String, texture: &crate::xml::TextureXml, is_line: bool) {
    if !is_line {
        return;
    }
    if let Some(thickness) = texture.thickness {
        code.push_str(&format!("tex:SetThickness({thickness})\n"));
    }
}

fn append_template_texture_defaults(code: &mut String, texture: &crate::xml::TextureXml) {
    if texture.anchors.is_none() && texture.set_all_points != Some(true) {
        code.push_str("tex:SetAllPoints(true)\n");
    }
    if texture.hidden == Some(true) {
        code.push_str("tex:Hide()\n");
    }
}

/// Append Mixin() calls from inherited templates and direct mixin attribute.
fn append_template_texture_mixins(code: &mut String, texture: &crate::xml::TextureXml) {
    for m in &crate::xml::collect_texture_mixins(texture) {
        code.push_str(&format!("if {} then Mixin(tex, {}) end\n", m, m));
    }
}

/// Process animation groups on a texture.
fn apply_texture_animations(lua: &Lua, texture: &crate::xml::TextureXml, child_name: &str) {
    let Some(anims) = &texture.animations else {
        return;
    };
    let mut anim_code = format!("local frame = {}\n", lua_global_ref(child_name));
    for group in &anims.animations {
        if group.is_virtual == Some(true) {
            continue;
        }
        anim_code.push_str(&generate_animation_group_code(group, "frame"));
    }
    let _ = chunk_cache::exec(lua, &anim_code, "template-elements-anim");
}

/// Wire MaskedTextures: call AddMaskTexture on each referenced sibling.
///
/// Tries synchronous wiring first (matching the XML loader). For dotted
/// paths (e.g. `HealthBar.MyHealPredictionBar.Fill`) where the target may
/// not exist yet, defers via `C_Timer.After(0, ...)` as a fallback.
fn append_mask_wiring(code: &mut String, is_mask: bool, texture: &crate::xml::TextureXml) {
    if !is_mask {
        return;
    }
    let Some(ref masked) = texture.masked_textures else {
        return;
    };
    for entry in &masked.entries {
        let Some(ref key) = entry.child_key else {
            continue;
        };
        let line = safe_add_mask_texture_code("parent", key);
        // Simple keys (same-layer siblings) — wire synchronously.
        // Dotted keys (nested children) — defer in case the target isn't created yet.
        if key.contains('.') {
            code.push_str(&format!(
                "            C_Timer.After(0, function() {line} end)\n"
            ));
        } else {
            code.push_str(&format!("            {line}\n"));
        }
    }
}

/// Generate Lua code that safely navigates a dotted childKey path and calls AddMaskTexture.
///
/// For `root="parent"` and `key="HealthBar.MyHealPredictionBar.Fill"`, produces:
/// `if parent.HealthBar and parent.HealthBar.MyHealPredictionBar and parent.HealthBar.MyHealPredictionBar.Fill then parent.HealthBar.MyHealPredictionBar.Fill:AddMaskTexture(tex) end`
fn safe_add_mask_texture_code(root: &str, key: &str) -> String {
    let parts: Vec<&str> = key.split('.').collect();
    let full_path = format!("{root}.{key}");
    if parts.len() <= 1 {
        return format!("if {full_path} then {full_path}:AddMaskTexture(tex) end");
    }
    let mut guards = Vec::new();
    let mut path = root.to_string();
    for part in &parts {
        path = format!("{path}.{part}");
        guards.push(path.clone());
    }
    let guard_str = guards.join(" and ");
    format!("if {guard_str} then {full_path}:AddMaskTexture(tex) end")
}

/// Append texture source setters (file, atlas, texcoords) to Lua code.
fn append_texture_source(
    code: &mut String,
    texture: &crate::xml::TextureXml,
    var: &str,
    is_mask: bool,
) {
    if let Some(file) = &texture.file {
        code.push_str(&format!(
            "            {}:SetTexture(\"{}\")\n",
            var,
            escape_lua_string(file)
        ));
    }
    if let Some(atlas) = &texture.atlas {
        let use_atlas_size = texture.use_atlas_size.unwrap_or(is_mask);
        code.push_str(&format!(
            "            {}:SetAtlas(\"{}\", {})\n",
            var,
            escape_lua_string(atlas),
            use_atlas_size
        ));
    }
    if let Some(tc) = &texture.tex_coords {
        let left = tc.left.unwrap_or(0.0);
        let right = tc.right.unwrap_or(1.0);
        let top = tc.top.unwrap_or(0.0);
        let bottom = tc.bottom.unwrap_or(1.0);
        code.push_str(&format!(
            "            {}:SetTexCoord({}, {}, {}, {})\n",
            var, left, right, top, bottom
        ));
    }
}

/// Append texture tinting from a `<Color>` XML element.
///
/// Textured regions should keep their file/atlas and use `SetVertexColor`.
/// Untextured regions use `SetColorTexture` to become a solid fill.
fn append_color_code(
    code: &mut String,
    texture: &crate::xml::TextureXml,
    color: &crate::xml::ColorXml,
    var: &str,
) {
    let uses_texture_source = texture.file.is_some() || texture.atlas.is_some();
    if let Some(name) = &color.color {
        let color_method = if uses_texture_source {
            "SetVertexColor"
        } else {
            "SetColorTexture"
        };
        code.push_str(&format!(
            "            do local c = {name} if c then {var}:{color_method}(c:GetRGBA()) end end\n",
        ));
    } else {
        let (r, g, b, a) = (
            color.r.unwrap_or(1.0),
            color.g.unwrap_or(1.0),
            color.b.unwrap_or(1.0),
            color.a.unwrap_or(1.0),
        );
        let color_method = if uses_texture_source {
            "SetVertexColor"
        } else {
            "SetColorTexture"
        };
        code.push_str(&format!(
            "            {var}:{color_method}({r}, {g}, {b}, {a})\n"
        ));
    }
}

/// Append `SetGradient` from a `<Gradient>` XML element.
fn append_gradient_code(code: &mut String, grad: &crate::xml::GradientXml, var: &str) {
    let orient = grad.orientation.as_deref().unwrap_or("VERTICAL");
    let [mr, mg, mb, ma] = extract_gradient_rgba(grad.min_color.as_ref(), 1.0);
    let [xr, xg, xb, xa] = extract_gradient_rgba(grad.max_color.as_ref(), 1.0);
    code.push_str(&format!(
        "            {var}:SetGradient(\"{orient}\", {{r={mr},g={mg},b={mb},a={ma}}}, {{r={xr},g={xg},b={xb},a={xa}}})\n"
    ));
}

/// Extract RGBA from an optional ColorXml with defaults (0 for RGB, custom for alpha).
fn extract_gradient_rgba(color: Option<&crate::xml::ColorXml>, default_a: f32) -> [f32; 4] {
    match color {
        Some(c) => [
            c.r.unwrap_or(0.0),
            c.g.unwrap_or(0.0),
            c.b.unwrap_or(0.0),
            c.a.unwrap_or(default_a),
        ],
        None => [0.0, 0.0, 0.0, default_a],
    }
}

/// Append texture-specific property setters (size, source, color, tiling, etc.) to Lua code.
pub(super) fn append_texture_properties(
    code: &mut String,
    texture: &crate::xml::TextureXml,
    var: &str,
    is_mask: bool,
) {
    if let Some(size) = &texture.size {
        append_size_code(code, size, var);
    }
    append_texture_source(code, texture, var, is_mask);
    if let Some(color) = &texture.color {
        append_color_code(code, texture, color, var);
    }
    if let Some(ref grad) = texture.gradient {
        append_gradient_code(code, grad, var);
    }
    if texture.horiz_tile == Some(true) {
        code.push_str(&format!("            {}:SetHorizTile(true)\n", var));
    }
    if texture.vert_tile == Some(true) {
        code.push_str(&format!("            {}:SetVertTile(true)\n", var));
    }
    if let Some(a) = texture.alpha {
        code.push_str(&format!("            {}:SetAlpha({})\n", var, a));
    }
    if let Some(mode) = texture.effective_blend_mode() {
        code.push_str(&format!("            {}:SetBlendMode(\"{}\")\n", var, mode));
    }
    append_key_values(code, texture.key_values.as_ref(), var);
}

/// Append SetSize/SetWidth/SetHeight from a `<Size>` XML element.
fn append_size_code(code: &mut String, size: &crate::xml::SizeXml, var: &str) {
    let (width, height) = get_size_values(size);
    match (width, height) {
        (Some(w), Some(h)) => code.push_str(&format!("            {var}:SetSize({w}, {h})\n")),
        (Some(w), None) => code.push_str(&format!("            {var}:SetWidth({w})\n")),
        (None, Some(h)) => code.push_str(&format!("            {var}:SetHeight({h})\n")),
        _ => {}
    }
}

pub(super) fn append_key_values(
    code: &mut String,
    key_values: Option<&crate::xml::KeyValuesXml>,
    var: &str,
) {
    let Some(key_values) = key_values else {
        return;
    };
    for kv in &key_values.values {
        let value = match kv.value_type.as_deref() {
            Some("number") => kv.value.clone(),
            Some("boolean") => kv.value.to_lowercase(),
            Some("global") if !kv.value.is_empty() => kv.value.clone(),
            Some("global") => "nil".to_string(),
            _ => format!("\"{}\"", escape_lua_string(&kv.value)),
        };
        code.push_str(&format!("            {var}.{} = {value}\n", kv.key));
    }
}

pub(super) struct AnchorParentContext<'a> {
    pub parent_key: &'a Option<String>,
    pub parent_array: &'a Option<String>,
    pub var: &'a str,
    pub parent_var: &'a str,
    pub parent_name: &'a str,
}

/// Append anchors, setAllPoints, parentKey, and parentArray assignment to Lua code.
pub(super) fn append_anchors_and_parent_refs(
    code: &mut String,
    anchors: &Option<crate::xml::AnchorsXml>,
    set_all_points: Option<bool>,
    context: AnchorParentContext<'_>,
) {
    if let Some(anchors) = anchors {
        code.push_str(&generate_set_point_code(
            anchors,
            context.var,
            context.parent_var,
            context.parent_name,
            context.parent_var,
        ));
    }
    if set_all_points == Some(true) {
        code.push_str(&format!("            {}:SetAllPoints(true)\n", context.var));
    }
    if let Some(parent_key) = context.parent_key {
        let key_escaped = escape_lua_string(parent_key);
        code.push_str(&format!(
            "            {}[\"{key_escaped}\"] = {}\n",
            context.parent_var, context.var
        ));
    }
    if let Some(parent_array) = context.parent_array {
        let arr_escaped = escape_lua_string(parent_array);
        code.push_str(&format!(
            "            {}[\"{arr_escaped}\"] = {}[\"{arr_escaped}\"] or {{}}\n\
             table.insert({}[\"{arr_escaped}\"], {})\n",
            context.parent_var, context.parent_var, context.parent_var, context.var
        ));
    }
}

/// Apply deferred mask atlases from KeyValues after all templates are applied.
///
/// Some templates define MaskTexture children with `useAtlasSize="true"` but no
/// atlas in XML — the atlas name is stored in a KeyValue and applied by a
/// composite-mixin OnLoad that may not run in our simulator.  We scan the
/// template chain for this pattern and emit Lua code to apply the atlas.
pub(super) fn apply_deferred_mask_atlases(
    lua: &Lua,
    frame_name: &str,
    chain: &[crate::xml::TemplateEntry],
) {
    let atlas_kvs = collect_atlas_key_values(chain);
    let masks = collect_unatlased_masks(chain);
    if atlas_kvs.is_empty() || masks.is_empty() {
        return;
    }
    let frame_ref = lua_global_ref(frame_name);
    let mut code = format!("do local f = {frame_ref}\nif f then\n");
    for (parent_key, _) in &masks {
        for (kv_key, _) in &atlas_kvs {
            if mask_key_matches_atlas_kv(parent_key, kv_key) {
                code.push_str(&format!(
                    "if f.{parent_key} and f.{kv_key} and f.{kv_key} ~= \"\" \
                     and not f.{parent_key}:GetAtlas() then \
                     f.{parent_key}:SetAtlas(f.{kv_key}, true) end\n"
                ));
            }
        }
    }
    code.push_str("end\nend");
    let _ = chunk_cache::exec(lua, &code, "template-elements");
}

/// Collect KeyValues whose key ends with "Atlas" and has a string type.
fn collect_atlas_key_values(chain: &[crate::xml::TemplateEntry]) -> Vec<(String, String)> {
    let mut result = Vec::new();
    for entry in chain {
        for kvs in entry.frame.all_key_values() {
            for kv in &kvs.values {
                if kv.key.ends_with("Atlas")
                    && kv.value_type.as_deref() == Some("string")
                    && !kv.value.is_empty()
                {
                    result.push((kv.key.clone(), kv.value.clone()));
                }
            }
        }
    }
    result
}

/// Collect MaskTextures with `useAtlasSize=true` and no atlas attribute.
fn collect_unatlased_masks(chain: &[crate::xml::TemplateEntry]) -> Vec<(String, bool)> {
    chain
        .iter()
        .flat_map(|entry| entry.frame.layers())
        .flat_map(|layers| &layers.layers)
        .flat_map(|layer| &layer.elements)
        .filter_map(|elem| match elem {
            crate::xml::LayerElement::MaskTexture(t) => Some(t),
            _ => None,
        })
        .filter(|t| t.atlas.is_none() && t.use_atlas_size == Some(true))
        .filter_map(|t| t.parent_key.as_ref().map(|pk| (pk.clone(), true)))
        .collect()
}

/// Check if a MaskTexture parentKey matches an atlas KeyValue key.
///
/// Pattern: `BorderSheenMask` matches `sheenMaskAtlas` by stripping the common
/// "Border" prefix, lowercasing the first char, and appending "Atlas".
fn mask_key_matches_atlas_kv(parent_key: &str, kv_key: &str) -> bool {
    let Some(suffix) = kv_key.strip_suffix("Atlas") else {
        return false;
    };
    // Direct: parentKey lowercased == suffix (e.g. "IconMask" → "iconMask")
    if lowercase_first(parent_key) == suffix {
        return true;
    }
    // Strip "Border" prefix: "BorderSheenMask" → "sheenMask"
    if let Some(stripped) = parent_key.strip_prefix("Border") {
        return lowercase_first(stripped) == suffix;
    }
    false
}

fn lowercase_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_lowercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::build_template_texture_lua;

    #[test]
    fn build_template_texture_lua_uses_line_creation_and_thickness() {
        let texture = crate::xml::TextureXml {
            thickness: Some(3.5),
            ..Default::default()
        };

        let code = build_template_texture_lua(
            &texture,
            "ParentFrame",
            "ParentFrameLine",
            "OVERLAY",
            false,
            true,
        );

        assert!(code.contains("CreateLine(\"ParentFrameLine\", \"OVERLAY\")"));
        assert!(code.contains("tex:SetThickness(3.5)"));
        assert!(code.contains("tex:SetAllPoints(true)"));
    }

    #[test]
    fn build_template_texture_lua_hides_and_defers_dotted_mask_wiring() {
        let ui = crate::xml::parse_xml(
            r#"
            <Ui>
                <Frame name="ParentFrame">
                    <Layers>
                        <Layer level="ARTWORK">
                            <MaskTexture name="ParentFrameMask" hidden="true">
                                <MaskedTextures>
                                    <MaskedTexture childKey="HealthBar.Fill"/>
                                </MaskedTextures>
                            </MaskTexture>
                        </Layer>
                    </Layers>
                </Frame>
            </Ui>
            "#,
        )
        .unwrap();
        let texture = match &ui.elements[0] {
            crate::xml::XmlElement::Frame(frame) => {
                match &frame.layers().next().unwrap().layers[0].elements[0] {
                    crate::xml::LayerElement::MaskTexture(texture) => texture.clone(),
                    other => panic!("expected mask texture, got {:?}", other),
                }
            }
            other => panic!("expected frame, got {:?}", other),
        };

        let code = build_template_texture_lua(
            &texture,
            "ParentFrame",
            "ParentFrameMask",
            "ARTWORK",
            true,
            false,
        );

        assert!(code.contains("CreateMaskTexture(\"ParentFrameMask\", \"ARTWORK\")"));
        assert!(code.contains("tex:Hide()"));
        assert!(code.contains("C_Timer.After(0, function()"));
        assert!(code.contains("parent.HealthBar.Fill:AddMaskTexture(tex)"));
    }
}
