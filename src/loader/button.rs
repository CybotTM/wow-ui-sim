//! Button texture and text application from XML.

use crate::lua_api::LoaderEnv;

use super::error::LoadError;
use super::helpers::{escape_lua_string, lua_global_ref};

/// Generate Lua code for a single button texture (atlas or file path),
/// and register the texture as a global if it has a `$parent`-prefixed name.
fn generate_button_texture_code(
    button_name: &str,
    method: &str,
    texture: &crate::xml::TextureXml,
) -> String {
    let getter = method.replace("Set", "Get");
    let register_global = texture.name.as_ref().map(|n| {
        let resolved = n.replace("$parent", button_name);
        format!(
            "do local t = {}:{}() if t then _G[\"{}\"] = t end end\n",
            lua_global_ref(button_name),
            getter,
            escape_lua_string(&resolved),
        )
    });

    let mut code = if let Some(atlas) = &texture.atlas {
        format!(
            r#"
        do
            local tex = {}:{}()
            if tex then tex:SetAtlas("{}") end
        end
        "#,
            lua_global_ref(button_name), getter, escape_lua_string(atlas)
        )
    } else if let Some(file) = &texture.file {
        format!(
            r#"
        {}:{}("{}")
        "#,
            lua_global_ref(button_name), method, escape_lua_string(file)
        )
    } else {
        String::new()
    };

    if let Some(reg) = register_global {
        code.push_str(&reg);
    }
    code
}

/// Apply button textures (NormalTexture, PushedTexture, etc.) from a FrameXml to a button.
///
/// First ensures texture children exist for ALL XML texture slots (atlas, file, or empty),
/// then generates Lua code to set atlas/file on them. This ordering is critical: Lua code
/// calls `Get*Texture()` which returns nil if the child doesn't exist yet.
pub fn apply_button_textures(
    env: &LoaderEnv<'_>,
    frame_xml: &crate::xml::FrameXml,
    button_name: &str,
) -> Result<(), LoadError> {
    let texture_slots: [(&str, &str, Option<&crate::xml::TextureXml>); 4] = [
        ("SetNormalTexture", "NormalTexture", frame_xml.normal_texture()),
        ("SetPushedTexture", "PushedTexture", frame_xml.pushed_texture()),
        ("SetHighlightTexture", "HighlightTexture", frame_xml.highlight_texture()),
        ("SetDisabledTexture", "DisabledTexture", frame_xml.disabled_texture()),
    ];

    // Create texture children for ALL slots BEFORE running Lua code.
    // The Lua atlas path calls Get*Texture() which needs the child to exist.
    ensure_button_texture_children(env, &texture_slots, button_name);

    let lua_code: String = texture_slots
        .iter()
        .filter_map(|(method, _, tex)| tex.map(|t| generate_button_texture_code(button_name, method, t)))
        .collect();

    if !lua_code.is_empty() {
        env.exec(&lua_code).map_err(|e| {
            LoadError::Lua(format!("Failed to apply button textures to {}: {}", button_name, e))
        })?;
    }

    Ok(())
}

/// Ensure texture children exist for all button XML texture slots.
/// Creates empty texture children for any slot that has an XML element,
/// regardless of whether it has atlas/file attributes.
fn ensure_button_texture_children(
    env: &LoaderEnv<'_>,
    slots: &[(&str, &str, Option<&crate::xml::TextureXml>); 4],
    button_name: &str,
) {
    use crate::lua_api::frame::methods::methods_helpers::get_or_create_button_texture;
    let mut state = env.state.borrow_mut();
    let button_id = state.widgets.get_id_by_name(button_name);
    let Some(button_id) = button_id else { return };
    for &(_, parent_key, tex_opt) in slots {
        if tex_opt.is_none() {
            continue;
        }
        get_or_create_button_texture(&mut state, button_id, parent_key);
    }
}

/// Apply button text from the text attribute and ButtonText child element.
pub fn apply_button_text(
    env: &LoaderEnv<'_>,
    frame_xml: &crate::xml::FrameXml,
    button_name: &str,
    inherits: &str,
) -> Result<(), LoadError> {
    // Create FontString from <ButtonText> child element if present.
    if let Some(bt) = frame_xml.button_text() {
        super::xml_fontstring::create_fontstring_from_xml(
            env, bt, button_name, "ARTWORK", 0,
        )?;
    }

    // Check for text attribute on the frame itself first
    let text = if let Some(t) = &frame_xml.text {
        Some(t.clone())
    } else if !inherits.is_empty() {
        // Check inherited templates for text attribute
        let template_chain = crate::xml::get_template_chain(inherits);
        template_chain
            .iter()
            .find_map(|entry| entry.frame.text.clone())
    } else {
        None
    };

    if let Some(text_key) = text {
        // In WoW, text attribute is a localization key or literal text.
        // We try to resolve it via global lookup (e.g., CANCEL -> "Cancel").
        // If not found, use the literal value.
        // Set text on both the button AND its Text fontstring child to ensure rendering works.
        let lua_code = format!(
            r#"
            local frame = {}
            if frame then
                local text = _G["{}"] or "{}"
                if frame.SetText then
                    frame:SetText(text)
                end
                -- Also set text directly on the Text fontstring child
                if frame.Text and frame.Text.SetText then
                    frame.Text:SetText(text)
                end
            end
            "#,
            lua_global_ref(button_name),
            escape_lua_string(&text_key),
            escape_lua_string(&text_key)
        );
        env.exec(&lua_code).ok(); // Ignore errors (SetText might not exist)
    }

    Ok(())
}
