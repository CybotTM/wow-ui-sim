//! FontString and button-related template element creation helpers.

use crate::loader::chunk_cache;
use crate::loader::helpers::resolve_lua_escapes;
use mlua::Lua;

use super::elements::{
    AnchorParentContext, append_anchors_and_parent_refs, append_key_values,
    append_texture_properties,
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
    let child_name = fontstring
        .name
        .as_ref()
        .map(|name| name.replace("$parent", subst_parent))
        .unwrap_or_else(|| format!("__fs_{}", rand_id()));

    let inherits = fontstring.inherits.as_deref().unwrap_or("");

    let mut code = format!(
        r#"
        local parent = {}
        if parent then
            local fs = parent:CreateFontString("{}", "{}", {})
        "#,
        lua_global_ref(parent_name),
        escape_lua_string(&child_name),
        draw_layer,
        if inherits.is_empty() {
            "nil".to_string()
        } else {
            format!("\"{}\"", inherits)
        }
    );

    append_fontstring_size_and_text(&mut code, fontstring);
    append_fontstring_justify_and_color(&mut code, fontstring);
    append_fontstring_shadow(&mut code, fontstring);
    append_anchors_and_parent_refs(
        &mut code,
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
    append_fontstring_wrap_and_lines(&mut code, fontstring);
    append_key_values(&mut code, fontstring.key_values.as_ref(), "fs");

    if fontstring.hidden == Some(true) {
        code.push_str("            fs:Hide()\n");
    }
    if let Some(alpha) = fontstring.alpha {
        code.push_str(&format!("            fs:SetAlpha({})\n", alpha));
    }

    code.push_str("        end\n");
    let _ = chunk_cache::exec(lua, &code, "template-elements");
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
    let child_name = bar
        .name
        .as_ref()
        .map(|name| name.replace("$parent", subst_parent))
        .unwrap_or_else(|| format!("__bar_{}", rand_id()));

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
    let child_name = thumb
        .name
        .as_ref()
        .map(|name| name.replace("$parent", subst_parent))
        .unwrap_or_else(|| format!("__thumb_{}", rand_id()));

    let mut code = format!(
        r#"
        local parent = {}
        if parent and parent.SetThumbTexture then
            local thumb = parent:CreateTexture("{}", "ARTWORK")
        "#,
        lua_global_ref(parent_name),
        escape_lua_string(&child_name),
    );

    if let Some(size) = &thumb.size {
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
    if let Some(file) = &thumb.file {
        code.push_str(&format!(
            "            thumb:SetTexture(\"{}\")\n",
            escape_lua_string(file)
        ));
    }

    code.push_str("            parent:SetThumbTexture(thumb)\n");
    if let Some(parent_key) = &thumb.parent_key {
        code.push_str(&format!(
            "            parent[\"{}\"] = thumb\n",
            escape_lua_string(parent_key)
        ));
    } else {
        code.push_str("            parent[\"ThumbTexture\"] = thumb\n");
    }

    code.push_str("        end\n");
    let _ = chunk_cache::exec(lua, &code, "template-elements");
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

    let code = build_button_texture_code(
        texture,
        parent_name,
        setter_method,
        actual_parent_key,
        &texture_name,
    );
    let _ = chunk_cache::exec(lua, &code, "template-elements");
}

fn build_button_texture_code(
    texture: &crate::xml::TextureXml,
    parent_name: &str,
    setter_method: &str,
    actual_parent_key: &str,
    texture_name: &str,
) -> String {
    let key_escaped = escape_lua_string(actual_parent_key);
    let mut code = format!(
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
    );
    append_texture_properties(&mut code, texture, "tex", false);
    let no_parent_key = None;
    let no_parent_array = None;
    append_anchors_and_parent_refs(
        &mut code,
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
    if texture.hidden == Some(true) {
        code.push_str("            tex:Hide()\n");
    }

    code.push_str(&format!("            parent[\"{key_escaped}\"] = tex\n"));
    code.push_str(&format!("            parent:{}(tex)\n", setter_method));

    code.push_str("        end\n");
    code
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
