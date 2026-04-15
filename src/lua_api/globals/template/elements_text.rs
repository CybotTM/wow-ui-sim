//! FontString and button-related template element creation helpers.

use crate::loader::chunk_cache;
use crate::loader::helpers::resolve_lua_escapes;
use crate::lua_api::SimState;
use crate::lua_api::frame::methods::{
    apply_font_inherit, button_texture_should_show, register_child_widget, set_button_texture_field,
};
use crate::render::BlendMode;
use crate::widget::{Color, Frame, TextJustify, WidgetType};
use mlua::Lua;
use std::cell::RefCell;
use std::rc::Rc;

use super::elements::{
    AnchorParentContext, append_anchors_and_parent_refs, append_key_values,
    append_texture_properties, apply_region_layout, apply_region_visibility,
    apply_texture_visuals_direct, resolve_state_frame_id, sync_region_parent_refs,
};
use super::{escape_lua_string, get_size_values, lua_global_ref, rand_id};

/// Create a fontstring from template XML.
///
/// `subst_parent` is the name used for `$parent` substitution (propagated
/// through anonymous frames).
pub(super) fn create_fontstring_from_template(
    lua: &Lua,
    fontstring: &crate::xml::FontStringXml,
    parent_name: &str,
    subst_parent: &str,
    draw_layer: &str,
) {
    let child_name = template_fontstring_name(fontstring, subst_parent);

    #[cfg(test)]
    super::test_counters::record_fontstring_create();

    let mut code = build_fontstring_create_preamble(
        parent_name,
        &child_name,
        draw_layer,
        fontstring.inherits.as_deref(),
    );
    append_fontstring_template_properties(&mut code, fontstring, parent_name);
    append_fontstring_visibility(&mut code, fontstring.hidden, fontstring.alpha);
    code.push_str("        end\n");
    let _ = chunk_cache::exec(lua, &code, "template-elements");
}

fn template_fontstring_name(fontstring: &crate::xml::FontStringXml, subst_parent: &str) -> String {
    fontstring
        .name
        .as_ref()
        .map(|name| name.replace("$parent", subst_parent))
        .unwrap_or_else(|| format!("__fs_{}", rand_id()))
}

fn build_fontstring_create_preamble(
    parent_name: &str,
    child_name: &str,
    draw_layer: &str,
    inherits: Option<&str>,
) -> String {
    format!(
        r#"
        local parent = {}
        if parent then
            local fs = parent:CreateFontString("{}", "{}", {})
        "#,
        lua_global_ref(parent_name),
        escape_lua_string(child_name),
        draw_layer,
        quoted_fontstring_inherits(inherits)
    )
}

fn quoted_fontstring_inherits(inherits: Option<&str>) -> String {
    inherits
        .filter(|inherits| !inherits.is_empty())
        .map(|inherits| format!("\"{}\"", inherits))
        .unwrap_or_else(|| "nil".to_string())
}

fn append_fontstring_template_properties(
    code: &mut String,
    fontstring: &crate::xml::FontStringXml,
    parent_name: &str,
) {
    append_fontstring_size_and_text(code, fontstring);
    append_fontstring_justify_and_color(code, fontstring);
    append_fontstring_shadow(code, fontstring);
    append_anchors_and_parent_refs(
        code,
        &fontstring.anchors,
        fontstring.set_all_points,
        AnchorParentContext {
            parent_key: &fontstring.parent_key,
            parent_array: &fontstring.parent_array,
            var: "fs",
            parent_var: "parent",
            parent_name,
        },
    );
    append_fontstring_wrap_and_lines(code, fontstring);
    append_key_values(code, fontstring.key_values.as_ref(), "fs");
}

fn append_fontstring_visibility(code: &mut String, hidden: Option<bool>, alpha: Option<f32>) {
    if hidden == Some(true) {
        code.push_str("            fs:Hide()\n");
    }
    if let Some(alpha) = alpha {
        code.push_str(&format!("            fs:SetAlpha({alpha})\n"));
    }
}

pub(super) fn create_fontstring_from_template_direct(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    fontstring: &crate::xml::FontStringXml,
    parent_name: &str,
    subst_parent: &str,
    draw_layer: &str,
) -> mlua::Result<()> {
    ensure_direct_fontstring_supported(fontstring)?;
    let parent_id = resolve_direct_region_parent(state, parent_name, "fontstring")?;
    let child_name = template_fontstring_name(fontstring, subst_parent);
    let child = build_direct_fontstring(lua, fontstring, parent_id, &child_name, draw_layer);
    let child_id = child.id;
    register_direct_fontstring(lua, parent_id, child, &child_name)?;
    sync_region_parent_refs(
        lua,
        state,
        parent_id,
        child_id,
        fontstring.parent_key.as_deref(),
        fontstring.parent_array.as_deref(),
    )?;
    apply_region_layout(
        state,
        child_id,
        &fontstring.anchors,
        fontstring.set_all_points,
        parent_name,
        false,
    );
    apply_region_visibility(state, child_id, fontstring.hidden, fontstring.alpha);
    Ok(())
}

fn resolve_direct_region_parent(
    state: &Rc<RefCell<SimState>>,
    parent_name: &str,
    region_kind: &str,
) -> mlua::Result<u64> {
    resolve_state_frame_id(state, parent_name).ok_or_else(|| {
        mlua::Error::runtime(format!(
            "missing parent '{}' for direct {region_kind} create",
            parent_name,
        ))
    })
}

fn build_direct_fontstring(
    lua: &Lua,
    fontstring: &crate::xml::FontStringXml,
    parent_id: u64,
    child_name: &str,
    draw_layer: &str,
) -> Frame {
    let mut child = Frame::new(
        WidgetType::FontString,
        Some(child_name.to_string()),
        Some(parent_id),
    );
    if let Some(layer) = crate::widget::DrawLayer::from_str(draw_layer) {
        child.draw_layer = layer;
    }
    if let Some(parent_key) = fontstring.parent_key.as_ref() {
        child.parent_key = Some(parent_key.clone());
    }
    apply_font_inherit(lua, &mut child, fontstring.inherits.as_deref());
    apply_fontstring_fields_direct(&mut child, fontstring);
    child
}

fn register_direct_fontstring(
    lua: &Lua,
    parent_id: u64,
    child: Frame,
    child_name: &str,
) -> mlua::Result<()> {
    register_child_widget(lua, parent_id, child, &Some(child_name.to_string())).map(|_| ())
}

fn append_fontstring_size_and_text(code: &mut String, fontstring: &crate::xml::FontStringXml) {
    if let Some(size) = fontstring.size.last() {
        let (width, height) = get_size_values(size);
        match (width, height) {
            (Some(width), Some(height)) => {
                code.push_str(&format!("            fs:SetSize({}, {})\n", width, height));
            }
            (Some(width), None) => {
                code.push_str(&format!("            fs:SetWidth({})\n", width));
            }
            (None, Some(height)) => {
                code.push_str(&format!("            fs:SetHeight({})\n", height));
            }
            _ => {}
        }
    }
    if let Some(text_key) = &fontstring.text {
        let raw_text =
            crate::global_strings::get_global_string(text_key).unwrap_or(text_key.as_str());
        let resolved_text = resolve_lua_escapes(raw_text);
        code.push_str(&format!(
            "            fs:SetText(\"{}\")\n",
            escape_lua_string(&resolved_text)
        ));
    }
}

fn append_fontstring_justify_and_color(code: &mut String, fontstring: &crate::xml::FontStringXml) {
    if let Some(justify_h) = &fontstring.justify_h {
        code.push_str(&format!("            fs:SetJustifyH(\"{}\")\n", justify_h));
    }
    if let Some(justify_v) = &fontstring.justify_v {
        code.push_str(&format!("            fs:SetJustifyV(\"{}\")\n", justify_v));
    }
    if let Some(color) = &fontstring.color {
        let r = color.r.unwrap_or(1.0);
        let g = color.g.unwrap_or(1.0);
        let b = color.b.unwrap_or(1.0);
        let a = color.a.unwrap_or(1.0);
        code.push_str(&format!(
            "            fs:SetTextColor({}, {}, {}, {})\n",
            r, g, b, a
        ));
    }
}

fn append_fontstring_shadow(code: &mut String, fontstring: &crate::xml::FontStringXml) {
    let Some(shadow) = &fontstring.shadow else {
        return;
    };
    if let Some(offset) = &shadow.offset {
        code.push_str(&format!(
            "            fs:SetShadowOffset({}, {})\n",
            offset.x(),
            offset.y()
        ));
    }
    if let Some(color) = &shadow.color {
        let r = color.r.unwrap_or(0.0);
        let g = color.g.unwrap_or(0.0);
        let b = color.b.unwrap_or(0.0);
        let a = color.a.unwrap_or(1.0);
        code.push_str(&format!(
            "            fs:SetShadowColor({}, {}, {}, {})\n",
            r, g, b, a
        ));
    }
}

fn append_fontstring_wrap_and_lines(code: &mut String, fontstring: &crate::xml::FontStringXml) {
    if fontstring.word_wrap == Some(false) {
        code.push_str("            fs:SetWordWrap(false)\n");
    }
    if let Some(max_lines) = fontstring.max_lines
        && max_lines > 0
    {
        code.push_str(&format!("            fs:SetMaxLines({})\n", max_lines));
    }
}

/// Create a bar texture from template XML (for StatusBars).
pub(super) fn create_bar_texture_from_template(
    lua: &Lua,
    bar: &crate::xml::TextureXml,
    parent_name: &str,
    subst_parent: &str,
) {
    let child_name = template_texture_name(bar, subst_parent, "__bar_");

    let mut code = format!(
        r#"
        local parent = {}
        if parent and parent.SetStatusBarTexture then
            local bar = parent:CreateTexture("{}", "ARTWORK")
        "#,
        lua_global_ref(parent_name),
        escape_lua_string(&child_name),
    );

    append_texture_properties(&mut code, bar, "bar", false);
    code.push_str("            parent:SetStatusBarTexture(bar)\n");
    let parent_key = bar.parent_key.as_deref().unwrap_or("Bar");
    code.push_str(&format!(
        "            parent[\"{}\"] = bar\n",
        escape_lua_string(parent_key)
    ));

    code.push_str("        end\n");
    let _ = chunk_cache::exec(lua, &code, "template-elements");
}

/// Create a thumb texture from template XML (for sliders).
pub(super) fn create_thumb_texture_from_template(
    lua: &Lua,
    thumb: &crate::xml::TextureXml,
    parent_name: &str,
    subst_parent: &str,
) {
    let child_name = template_texture_name(thumb, subst_parent, "__thumb_");
    let mut code = build_thumb_texture_create_preamble(parent_name, &child_name);
    append_thumb_texture_size_and_file(&mut code, thumb);
    code.push_str("            parent:SetThumbTexture(thumb)\n");
    append_thumb_texture_parent_binding(&mut code, thumb.parent_key.as_deref());

    code.push_str("        end\n");
    let _ = chunk_cache::exec(lua, &code, "template-elements");
}

fn template_texture_name(
    texture: &crate::xml::TextureXml,
    subst_parent: &str,
    anonymous_prefix: &str,
) -> String {
    texture
        .name
        .as_ref()
        .map(|name| name.replace("$parent", subst_parent))
        .unwrap_or_else(|| format!("{anonymous_prefix}{}", rand_id()))
}

fn build_thumb_texture_create_preamble(parent_name: &str, child_name: &str) -> String {
    format!(
        r#"
        local parent = {}
        if parent and parent.SetThumbTexture then
            local thumb = parent:CreateTexture("{}", "ARTWORK")
        "#,
        lua_global_ref(parent_name),
        escape_lua_string(child_name),
    )
}

fn append_thumb_texture_size_and_file(code: &mut String, thumb: &crate::xml::TextureXml) {
    append_thumb_texture_size(code, thumb.size.as_ref());
    append_thumb_texture_file(code, thumb.file.as_deref());
}

fn append_thumb_texture_size(code: &mut String, size: Option<&crate::xml::SizeXml>) {
    let Some(size) = size else {
        return;
    };
    let (width, height) = get_size_values(size);
    match (width, height) {
        (Some(width), Some(height)) => {
            code.push_str(&format!(
                "            thumb:SetSize({}, {})\n",
                width, height
            ));
        }
        (Some(width), None) => {
            code.push_str(&format!("            thumb:SetWidth({})\n", width));
        }
        (None, Some(height)) => {
            code.push_str(&format!("            thumb:SetHeight({})\n", height));
        }
        _ => {}
    }
}

fn append_thumb_texture_file(code: &mut String, file: Option<&str>) {
    let Some(file) = file else {
        return;
    };
    code.push_str(&format!(
        "            thumb:SetTexture(\"{}\")\n",
        escape_lua_string(file)
    ));
}

fn append_thumb_texture_parent_binding(code: &mut String, parent_key: Option<&str>) {
    let binding = parent_key.unwrap_or("ThumbTexture");
    code.push_str(&format!(
        "            parent[\"{}\"] = thumb\n",
        escape_lua_string(binding)
    ));
}

pub(super) fn create_button_texture_from_template_direct(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    texture: &crate::xml::TextureXml,
    parent_name: &str,
    subst_parent: &str,
    parent_key: &str,
) -> mlua::Result<()> {
    let parent_id = resolve_direct_region_parent(state, parent_name, "button texture")?;
    let binding = direct_button_texture_binding(texture, subst_parent, parent_key);
    let child = build_direct_button_texture(parent_id, &binding);
    let child_id = child.id;
    register_direct_button_texture(lua, parent_id, child, &binding.child_name)?;
    sync_region_parent_refs(
        lua,
        state,
        parent_id,
        child_id,
        Some(binding.actual_parent_key.as_str()),
        texture.parent_array.as_deref(),
    )?;
    apply_texture_visuals_direct(state, child_id, texture, false, false);
    apply_region_layout(
        state,
        child_id,
        &texture.anchors,
        texture.set_all_points,
        parent_name,
        false,
    );
    apply_region_visibility(state, child_id, texture.hidden, texture.alpha);
    sync_button_texture_slot(
        state,
        parent_id,
        child_id,
        binding.actual_parent_key.as_str(),
    );
    Ok(())
}

struct DirectButtonTextureBinding {
    child_name: String,
    actual_parent_key: String,
}

fn direct_button_texture_binding(
    texture: &crate::xml::TextureXml,
    subst_parent: &str,
    parent_key: &str,
) -> DirectButtonTextureBinding {
    let default_parent_key = format!("{parent_key}Texture");
    let actual_parent_key = texture.parent_key.clone().unwrap_or(default_parent_key);
    let child_name = texture
        .name
        .as_ref()
        .map(|name| name.replace("$parent", subst_parent))
        .unwrap_or_else(|| format!("__tex_{}", rand_id()));
    DirectButtonTextureBinding {
        child_name,
        actual_parent_key,
    }
}

fn build_direct_button_texture(parent_id: u64, binding: &DirectButtonTextureBinding) -> Frame {
    let mut child = Frame::new(
        WidgetType::Texture,
        Some(binding.child_name.clone()),
        Some(parent_id),
    );
    child.parent_key = Some(binding.actual_parent_key.clone());
    apply_button_slot_defaults(&mut child, binding.actual_parent_key.as_str());
    child
}

fn register_direct_button_texture(
    lua: &Lua,
    parent_id: u64,
    child: Frame,
    child_name: &str,
) -> mlua::Result<()> {
    register_child_widget(lua, parent_id, child, &Some(child_name.to_string())).map(|_| ())
}

/// Create a button texture from template XML (NormalTexture, PushedTexture, etc.).
pub(super) fn create_button_texture_from_template(
    lua: &Lua,
    texture: &crate::xml::TextureXml,
    parent_name: &str,
    subst_parent: &str,
    parent_key: &str,
    setter_method: &str,
) {
    let default_parent_key = format!("{}Texture", parent_key);
    let actual_parent_key = texture.parent_key.as_deref().unwrap_or(&default_parent_key);

    let child_name = texture
        .name
        .as_ref()
        .map(|name| name.replace("$parent", subst_parent));
    let texture_name = child_name
        .clone()
        .unwrap_or_else(|| format!("__tex_{}", rand_id()));

    #[cfg(test)]
    super::test_counters::record_button_texture_create();

    let code = build_button_texture_code(
        texture,
        parent_name,
        setter_method,
        actual_parent_key,
        &texture_name,
    );
    let _ = chunk_cache::exec(lua, &code, "template-elements");
}

fn ensure_direct_fontstring_supported(fontstring: &crate::xml::FontStringXml) -> mlua::Result<()> {
    if fontstring
        .key_values
        .as_ref()
        .is_some_and(|values| !values.values.is_empty())
    {
        return Err(mlua::Error::runtime(
            "direct fontstring create does not support key values yet",
        ));
    }
    Ok(())
}

fn apply_fontstring_fields_direct(frame: &mut Frame, fontstring: &crate::xml::FontStringXml) {
    apply_fontstring_size(frame, fontstring);
    apply_fontstring_text(frame, fontstring);
    apply_fontstring_alignment(frame, fontstring);
    apply_fontstring_colors(frame, fontstring);
    apply_fontstring_limits(frame, fontstring);
}

fn apply_fontstring_size(frame: &mut Frame, fontstring: &crate::xml::FontStringXml) {
    if let Some(size) = fontstring.size.last() {
        let (width, height) = get_size_values(size);
        match (width, height) {
            (Some(width), Some(height)) => frame.set_size(width, height),
            (Some(width), None) => frame.width = width,
            (None, Some(height)) => frame.height = height,
            (None, None) => {}
        }
    }
}

fn apply_fontstring_text(frame: &mut Frame, fontstring: &crate::xml::FontStringXml) {
    if let Some(text_key) = fontstring.text.as_ref() {
        let raw_text =
            crate::global_strings::get_global_string(text_key).unwrap_or(text_key.as_str());
        frame.text = Some(resolve_lua_escapes(raw_text));
    }
}

fn apply_fontstring_alignment(frame: &mut Frame, fontstring: &crate::xml::FontStringXml) {
    if let Some(justify_h) = fontstring.justify_h.as_ref() {
        frame.justify_h = TextJustify::from_wow_str(justify_h);
    }
    if let Some(justify_v) = fontstring.justify_v.as_ref() {
        frame.justify_v = TextJustify::from_wow_str(justify_v);
    }
}

fn apply_fontstring_colors(frame: &mut Frame, fontstring: &crate::xml::FontStringXml) {
    if let Some(color) = fontstring.color.as_ref() {
        frame.text_color = Color::new(
            color.r.unwrap_or(1.0),
            color.g.unwrap_or(1.0),
            color.b.unwrap_or(1.0),
            color.a.unwrap_or(1.0),
        );
    }
    if let Some(shadow) = fontstring.shadow.as_ref() {
        if let Some(offset) = shadow.offset.as_ref() {
            frame.shadow_offset = (offset.x(), offset.y());
        }
        if let Some(color) = shadow.color.as_ref() {
            frame.shadow_color = Color::new(
                color.r.unwrap_or(0.0),
                color.g.unwrap_or(0.0),
                color.b.unwrap_or(0.0),
                color.a.unwrap_or(1.0),
            );
        }
    }
}

fn apply_fontstring_limits(frame: &mut Frame, fontstring: &crate::xml::FontStringXml) {
    if fontstring.word_wrap == Some(false) {
        frame.word_wrap = false;
    }
    if let Some(max_lines) = fontstring.max_lines
        && max_lines > 0
    {
        frame.max_lines = max_lines;
    }
}

fn apply_button_slot_defaults(texture: &mut Frame, parent_key: &str) {
    if parent_key == "HighlightTexture" {
        texture.draw_layer = crate::widget::DrawLayer::Highlight;
        texture.alpha_mode = Some("ADD".to_string());
        texture.blend_mode = BlendMode::Additive;
    }
}

fn sync_button_texture_slot(
    state: &Rc<RefCell<SimState>>,
    button_id: u64,
    texture_id: u64,
    parent_key: &str,
) {
    let child_texture = {
        let sim = state.borrow();
        sim.widgets.get(texture_id).and_then(|frame| {
            frame.texture.clone().map(|path| {
                let tex_coords = frame
                    .atlas_tex_coords
                    .or(frame.tex_coords)
                    .unwrap_or((0.0, 1.0, 0.0, 1.0));
                (path, tex_coords)
            })
        })
    };

    let mut sim = state.borrow_mut();
    if let Some(button) = sim.widgets.get_mut_visual(button_id)
        && let Some((path, tex_coords)) = child_texture
    {
        set_button_texture_field(button, parent_key, path, tex_coords);
    }
    let visible = button_texture_should_show(&sim, button_id, parent_key);
    sim.widgets.set_visible(texture_id, visible);
}

fn build_button_texture_code(
    texture: &crate::xml::TextureXml,
    parent_name: &str,
    setter_method: &str,
    actual_parent_key: &str,
    texture_name: &str,
) -> String {
    let key_escaped = escape_lua_string(actual_parent_key);
    let mut code =
        build_button_texture_prefix(parent_name, setter_method, &key_escaped, texture_name);
    append_texture_properties(&mut code, texture, "tex", false);
    append_button_texture_anchors(&mut code, texture, parent_name);
    if texture.hidden == Some(true) {
        code.push_str("            tex:Hide()\n");
    }
    append_button_texture_suffix(&mut code, &key_escaped, setter_method);
    code.push_str("        end\n");
    code
}

fn build_button_texture_prefix(
    parent_name: &str,
    setter_method: &str,
    key_escaped: &str,
    texture_name: &str,
) -> String {
    format!(
        r#"
        local parent = {}
        if parent and parent.{} then
            local tex = parent["{}"]
            if tex == nil then
                tex = parent:CreateTexture("{}", "ARTWORK")
            end
        "#,
        lua_global_ref(parent_name),
        setter_method,
        key_escaped,
        escape_lua_string(texture_name),
    )
}

fn append_button_texture_anchors(
    code: &mut String,
    texture: &crate::xml::TextureXml,
    parent_name: &str,
) {
    let no_parent_key = None;
    let no_parent_array = None;
    append_anchors_and_parent_refs(
        code,
        &texture.anchors,
        texture.set_all_points,
        AnchorParentContext {
            parent_key: &no_parent_key,
            parent_array: &no_parent_array,
            var: "tex",
            parent_var: "parent",
            parent_name,
        },
    );
}

fn append_button_texture_suffix(code: &mut String, key_escaped: &str, setter_method: &str) {
    code.push_str(&format!("            parent[\"{key_escaped}\"] = tex\n"));
    code.push_str(&format!("            parent:{setter_method}(tex)\n"));
}

/// Apply the `text=` attribute on a Button element via SetText.
pub fn apply_button_text_attribute(lua: &Lua, frame: &crate::xml::FrameXml, frame_name: &str) {
    let Some(text_key) = &frame.text else {
        return;
    };
    let resolved = crate::global_strings::get_global_string(text_key).unwrap_or(text_key);
    let escaped = escape_lua_string(resolved);
    let frame_ref = lua_global_ref(frame_name);
    let code = format!(
        "do local f = {frame_ref} if f then \
         if f.SetText then f:SetText(\"{escaped}\") end \
         if f.Text and f.Text.SetText then f.Text:SetText(\"{escaped}\") end \
         end end"
    );
    let _ = chunk_cache::exec(lua, &code, "template-elements");
}

#[cfg(test)]
mod tests {
    use super::{
        append_thumb_texture_parent_binding, build_direct_button_texture, build_direct_fontstring,
        build_fontstring_create_preamble, direct_button_texture_binding,
        quoted_fontstring_inherits,
    };
    use crate::widget::{DrawLayer, WidgetType};
    use mlua::Lua;

    #[test]
    fn quoted_fontstring_inherits_uses_nil_for_missing_or_empty_values() {
        assert_eq!(quoted_fontstring_inherits(None), "nil");
        assert_eq!(quoted_fontstring_inherits(Some("")), "nil");
        assert_eq!(
            quoted_fontstring_inherits(Some("GameFontNormal")),
            "\"GameFontNormal\""
        );
    }

    #[test]
    fn build_fontstring_create_preamble_uses_escaped_name_and_inherits() {
        let code = build_fontstring_create_preamble(
            "Parent.Frame",
            "Frame\"Title",
            "OVERLAY",
            Some("GameFontNormal"),
        );

        assert!(code.contains("local parent = _G[\"Parent.Frame\"]"));
        assert!(code.contains(
            "parent:CreateFontString(\"Frame\\\"Title\", \"OVERLAY\", \"GameFontNormal\")"
        ));
    }

    #[test]
    fn append_thumb_texture_parent_binding_defaults_to_thumb_texture() {
        let mut code = String::new();
        append_thumb_texture_parent_binding(&mut code, None);

        assert!(code.contains("parent[\"ThumbTexture\"] = thumb"));

        code.clear();
        append_thumb_texture_parent_binding(&mut code, Some("Thumb\"Key"));
        assert!(code.contains("parent[\"Thumb\\\"Key\"] = thumb"));
    }

    #[test]
    fn build_direct_fontstring_sets_layer_parent_key_and_text_fields() {
        let lua = Lua::new();
        let fontstring = crate::xml::FontStringXml {
            name: Some("$parentTitle".to_string()),
            parent_key: Some("Text".to_string()),
            text: Some("Hello".to_string()),
            justify_h: Some("CENTER".to_string()),
            ..Default::default()
        };

        let child = build_direct_fontstring(&lua, &fontstring, 7, "FrameTitle", "OVERLAY");

        assert_eq!(child.widget_type, WidgetType::FontString);
        assert_eq!(child.parent_id, Some(7));
        assert_eq!(child.name.as_deref(), Some("FrameTitle"));
        assert_eq!(child.parent_key.as_deref(), Some("Text"));
        assert_eq!(child.draw_layer, DrawLayer::Overlay);
        assert_eq!(child.text.as_deref(), Some("Hello"));
    }

    #[test]
    fn direct_button_texture_binding_and_builder_apply_parent_key_defaults() {
        let texture = crate::xml::TextureXml {
            name: Some("$parentNormal".to_string()),
            ..Default::default()
        };

        let binding = direct_button_texture_binding(&texture, "Frame", "Normal");
        assert_eq!(binding.child_name, "FrameNormal");
        assert_eq!(binding.actual_parent_key, "NormalTexture");

        let child = build_direct_button_texture(9, &binding);
        assert_eq!(child.widget_type, WidgetType::Texture);
        assert_eq!(child.parent_id, Some(9));
        assert_eq!(child.name.as_deref(), Some("FrameNormal"));
        assert_eq!(child.parent_key.as_deref(), Some("NormalTexture"));
    }
}
