//! Texture creation from XML definitions.

use crate::lua_api::LoaderEnv;
use crate::xml::collect_texture_mixins;

use super::helpers::{
    escape_lua_string, generate_set_point_code, get_size_values, lua_global_ref,
    lua_table_field_ref,
};
use super::helpers_anim::generate_animation_group_code;
use std::fmt::Write;

fn append_file_code(code: &mut String, texture: &crate::xml::TextureXml) {
    if let Some(file) = &texture.file {
        let _ = write!(
            code,
            r#"
        tex:SetTexture("{}")
        "#,
            escape_lua_string(file)
        );
    }
}

fn append_atlas_code(code: &mut String, texture: &crate::xml::TextureXml, is_mask: bool) {
    if let Some(atlas) = &texture.atlas {
        let use_atlas_size = texture.use_atlas_size.unwrap_or(is_mask);
        let _ = write!(
            code,
            r#"
        tex:SetAtlas("{}", {})
        "#,
            escape_lua_string(atlas),
            use_atlas_size
        );
    }
}

fn append_tex_coords_code(code: &mut String, texture: &crate::xml::TextureXml) {
    if let Some(tc) = &texture.tex_coords {
        let left = tc.left.unwrap_or(0.0);
        let right = tc.right.unwrap_or(1.0);
        let top = tc.top.unwrap_or(0.0);
        let bottom = tc.bottom.unwrap_or(1.0);
        let _ = write!(
            code,
            r#"
        tex:SetTexCoord({}, {}, {}, {})
        "#,
            left, right, top, bottom
        );
    }
}

fn append_size_code(code: &mut String, texture: &crate::xml::TextureXml) {
    if let Some(size) = &texture.size {
        let (x, y) = get_size_values(size);
        match (x, y) {
            (Some(x), Some(y)) => {
                let _ = write!(code, "\n        tex:SetSize({}, {})\n        ", x, y);
            }
            (Some(x), None) => {
                let _ = write!(code, "\n        tex:SetWidth({})\n        ", x);
            }
            (None, Some(y)) => {
                let _ = write!(code, "\n        tex:SetHeight({})\n        ", y);
            }
            _ => {}
        }
    }
}

/// Generate Lua code for texture source (file or atlas) and size.
///
/// `is_mask`: MaskTextures default to `useAtlasSize=true` when not explicit,
/// matching WoW behavior where masks auto-size from their atlas.  This matters
/// because the mask frame must be larger than the icon so the icon samples only
/// the opaque center of the mask texture.
fn append_texture_source_code(code: &mut String, texture: &crate::xml::TextureXml, is_mask: bool) {
    append_file_code(code, texture);
    append_atlas_code(code, texture, is_mask);
    append_tex_coords_code(code, texture);
    append_size_code(code, texture);
}

fn texture_color_method(texture: &crate::xml::TextureXml) -> &'static str {
    if texture.file.is_some() || texture.atlas.is_some() {
        "SetVertexColor"
    } else {
        "SetColorTexture"
    }
}

fn append_named_color_code(code: &mut String, name: &str, color_method: &str) {
    let _ = write!(
        code,
        r#"
        do
            local c = {name}
            if c then
                tex:{color_method}(c:GetRGBA())
            end
        end
        "#,
        name = name,
        color_method = color_method
    );
}

fn append_rgba_color_code(code: &mut String, color: &crate::xml::ColorXml, color_method: &str) {
    let _ = write!(
        code,
        r#"
        tex:{color_method}({}, {}, {}, {})
        "#,
        color.r.unwrap_or(1.0),
        color.g.unwrap_or(1.0),
        color.b.unwrap_or(1.0),
        color.a.unwrap_or(1.0),
        color_method = color_method
    );
}

fn append_color_code(code: &mut String, texture: &crate::xml::TextureXml) {
    let Some(color) = &texture.color else {
        return;
    };
    let color_method = texture_color_method(texture);

    if let Some(name) = &color.color {
        append_named_color_code(code, name, color_method);
    } else {
        append_rgba_color_code(code, color, color_method);
    }
}

fn append_tiling_flags_code(code: &mut String, texture: &crate::xml::TextureXml) {
    if texture.wants_horiz_tile() {
        code.push_str("\n        tex:SetHorizTile(true)\n        ");
    }
    if texture.wants_vert_tile() {
        code.push_str("\n        tex:SetVertTile(true)\n        ");
    }
    if texture.set_all_points == Some(true) {
        code.push_str("\n        tex:SetAllPoints(true)\n        ");
    }
}

fn append_parent_key_code(code: &mut String, texture: &crate::xml::TextureXml) {
    if let Some(key) = &texture.parent_key {
        let parent_field = lua_table_field_ref("parent", key);
        let _ = write!(
            code,
            r#"
        {} = tex
        "#,
            parent_field
        );
    }
    if let Some(parent_array) = &texture.parent_array {
        let array_ref = lua_table_field_ref("parent", parent_array);
        let _ = write!(
            code,
            r#"
        {array_ref} = {array_ref} or {{}}
        table.insert({array_ref}, tex)
        "#,
        );
    }
}

/// Generate Lua code for texture visual properties (color, tiling, parentKey).
fn append_texture_visual_code(code: &mut String, texture: &crate::xml::TextureXml) {
    append_color_code(code, texture);
    append_tiling_flags_code(code, texture);
    append_parent_key_code(code, texture);
}

/// Generate Lua Mixin() calls for texture mixins (from inherits and direct mixin attr).
fn generate_mixin_code(texture: &crate::xml::TextureXml) -> String {
    let mixins = collect_texture_mixins(texture);
    if mixins.is_empty() {
        return String::new();
    }
    let mut code = String::new();
    for m in &mixins {
        code.push_str(&format!(
            "\n        if {} then Mixin(tex, {}) end\n        ",
            m, m
        ));
    }
    code
}

fn append_create_header(
    code: &mut String,
    tex_name: &str,
    parent_name: &str,
    create_method: &str,
    draw_layer: &str,
    sub_level: i32,
) {
    let _ = write!(
        code,
        r#"
        local parent = {}
        local tex = parent:{}("{}", "{}")
        "#,
        lua_global_ref(parent_name),
        create_method,
        escape_lua_string(tex_name),
        escape_lua_string(draw_layer)
    );
    if sub_level != 0 {
        let _ = write!(
            code,
            "\n        tex:SetDrawLayer(\"{}\", {})\n        ",
            draw_layer, sub_level
        );
    }
}

fn append_line_thickness(code: &mut String, texture: &crate::xml::TextureXml, is_line: bool) {
    if is_line {
        if let Some(t) = texture.thickness {
            let _ = write!(code, "\n        tex:SetThickness({})\n        ", t);
        }
    }
}

fn append_texture_visibility(code: &mut String, texture: &crate::xml::TextureXml) {
    if texture.hidden == Some(true) {
        code.push_str("\n        tex:Hide()\n        ");
    }
    if let Some(a) = texture.alpha {
        let _ = write!(code, "\n        tex:SetAlpha({})\n        ", a);
    }
    if let Some(mode) = texture.effective_blend_mode() {
        let _ = write!(code, "\n        tex:SetBlendMode(\"{}\")\n        ", mode);
    }
}

fn append_masked_textures(code: &mut String, texture: &crate::xml::TextureXml, is_mask: bool) {
    if !is_mask {
        return;
    }
    if let Some(ref masked) = texture.masked_textures {
        for entry in &masked.entries {
            if let Some(ref key) = entry.child_key {
                let parent_field = lua_table_field_ref("parent", key);
                let _ = write!(
                    code,
                    r#"
        if {parent_field} then {parent_field}:AddMaskTexture(tex) end
        "#,
                );
            }
        }
    }
}

/// Build the Lua code string that creates and configures a texture.
pub(super) fn build_texture_lua(
    tex_name: &str,
    texture: &crate::xml::TextureXml,
    parent_name: &str,
    draw_layer: &str,
    is_mask: bool,
    is_line: bool,
    sub_level: i32,
) -> String {
    let create_method = if is_line {
        "CreateLine"
    } else if is_mask {
        "CreateMaskTexture"
    } else {
        "CreateTexture"
    };
    let mut code = String::with_capacity(1024);
    append_create_header(
        &mut code,
        tex_name,
        parent_name,
        create_method,
        draw_layer,
        sub_level,
    );
    code.push_str(&generate_mixin_code(texture));
    append_line_thickness(&mut code, texture, is_line);
    append_texture_source_code(&mut code, texture, is_mask);
    append_texture_visual_code(&mut code, texture);
    append_texture_anchors(&mut code, texture, parent_name);
    append_texture_visibility(&mut code, texture);
    append_masked_textures(&mut code, texture, is_mask);
    code.push_str(&super::xml_frame_codegen::generate_key_values_code(
        texture.key_values.as_ref(),
        "tex",
    ));
    code
}

/// Append anchor or SetAllPoints code for a texture.
fn append_texture_anchors(code: &mut String, texture: &crate::xml::TextureXml, parent_name: &str) {
    if let Some(anchors) = &texture.anchors {
        code.push_str(&generate_set_point_code(
            anchors,
            "tex",
            "parent",
            parent_name,
            "parent",
        ));
    } else if texture.set_all_points != Some(true) {
        code.push_str("\n        tex:SetAllPoints(true)\n        ");
    }
}

/// Process animation groups on a texture created from XML.
pub(super) fn apply_texture_animations_xml(
    env: &LoaderEnv<'_>,
    texture: &crate::xml::TextureXml,
    tex_name: &str,
) {
    let Some(anims) = &texture.animations else {
        return;
    };
    let mut anim_code = format!("local frame = {}\n", lua_global_ref(tex_name));
    for anim_group_xml in &anims.animations {
        if anim_group_xml.is_virtual == Some(true) {
            if let Some(ref name) = anim_group_xml.name {
                crate::xml::register_anim_group_template(name, anim_group_xml.clone());
            }
            continue;
        }
        anim_code.push_str(&generate_animation_group_code(anim_group_xml, "frame"));
    }
    env.exec(&anim_code).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texture_create_code_escapes_generated_child_names() {
        let code = build_texture_lua(
            r#"Parent-|TInterface\AddOns\Addon\Icon:0|t.__tex_1"#,
            &crate::xml::TextureXml::default(),
            "UIParent",
            "ARTWORK",
            false,
            false,
            0,
        );

        assert!(
            code.contains(r#"CreateTexture("Parent-|TInterface\\AddOns\\Addon\\Icon:0|t.__tex_1""#)
        );
    }
}
