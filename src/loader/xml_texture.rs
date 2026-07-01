//! Texture creation from XML definitions.

use crate::lua_api::LoaderEnv;
use crate::xml::collect_texture_mixins;

use super::helpers::{
    escape_lua_string, generate_set_point_code, get_size_values, lua_global_ref,
    lua_table_field_ref,
};
use super::helpers_anim::generate_animation_group_code;

fn emit_file_code(texture: &crate::xml::TextureXml) -> String {
    if let Some(file) = &texture.file {
        return format!(
            r#"
        tex:SetTexture("{}")
        "#,
            escape_lua_string(file)
        );
    }
    String::new()
}

fn emit_atlas_code(texture: &crate::xml::TextureXml, is_mask: bool) -> String {
    if let Some(atlas) = &texture.atlas {
        let use_atlas_size = texture.use_atlas_size.unwrap_or(is_mask);
        return format!(
            r#"
        tex:SetAtlas("{}", {})
        "#,
            escape_lua_string(atlas),
            use_atlas_size
        );
    }
    String::new()
}

fn emit_tex_coords_code(texture: &crate::xml::TextureXml) -> String {
    if let Some(tc) = &texture.tex_coords {
        let left = tc.left.unwrap_or(0.0);
        let right = tc.right.unwrap_or(1.0);
        let top = tc.top.unwrap_or(0.0);
        let bottom = tc.bottom.unwrap_or(1.0);
        return format!(
            r#"
        tex:SetTexCoord({}, {}, {}, {})
        "#,
            left, right, top, bottom
        );
    }
    String::new()
}

fn emit_size_code(texture: &crate::xml::TextureXml) -> String {
    if let Some(size) = &texture.size {
        let (x, y) = get_size_values(size);
        return match (x, y) {
            (Some(x), Some(y)) => format!("\n        tex:SetSize({}, {})\n        ", x, y),
            (Some(x), None) => format!("\n        tex:SetWidth({})\n        ", x),
            (None, Some(y)) => format!("\n        tex:SetHeight({})\n        ", y),
            _ => String::new(),
        };
    }
    String::new()
}

/// Generate Lua code for texture source (file or atlas) and size.
///
/// `is_mask`: MaskTextures default to `useAtlasSize=true` when not explicit,
/// matching WoW behavior where masks auto-size from their atlas.  This matters
/// because the mask frame must be larger than the icon so the icon samples only
/// the opaque center of the mask texture.
fn generate_texture_source_code(texture: &crate::xml::TextureXml, is_mask: bool) -> String {
    let mut code = String::new();
    code.push_str(&emit_file_code(texture));
    code.push_str(&emit_atlas_code(texture, is_mask));
    code.push_str(&emit_tex_coords_code(texture));
    code.push_str(&emit_size_code(texture));
    code
}

fn texture_color_method(texture: &crate::xml::TextureXml) -> &'static str {
    if texture.file.is_some() || texture.atlas.is_some() {
        "SetVertexColor"
    } else {
        "SetColorTexture"
    }
}

fn emit_named_color_code(name: &str, color_method: &str) -> String {
    format!(
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
    )
}

fn emit_rgba_color_code(color: &crate::xml::ColorXml, color_method: &str) -> String {
    format!(
        r#"
        tex:{color_method}({}, {}, {}, {})
        "#,
        color.r.unwrap_or(1.0),
        color.g.unwrap_or(1.0),
        color.b.unwrap_or(1.0),
        color.a.unwrap_or(1.0),
        color_method = color_method
    )
}

fn emit_color_code(texture: &crate::xml::TextureXml) -> String {
    let Some(color) = &texture.color else {
        return String::new();
    };
    let color_method = texture_color_method(texture);

    if let Some(name) = &color.color {
        emit_named_color_code(name, color_method)
    } else {
        emit_rgba_color_code(color, color_method)
    }
}

fn emit_gradient_code(texture: &crate::xml::TextureXml) -> String {
    let Some(gradient) = &texture.gradient else {
        return String::new();
    };
    let orientation = gradient.orientation.as_deref().unwrap_or("VERTICAL");
    let min = gradient.min_color.as_ref();
    let max = gradient.max_color.as_ref();
    format!(
        r#"
        tex:SetGradient("{}", {{ r = {}, g = {}, b = {}, a = {} }}, {{ r = {}, g = {}, b = {}, a = {} }})
        "#,
        escape_lua_string(orientation),
        min.and_then(|color| color.r).unwrap_or(1.0),
        min.and_then(|color| color.g).unwrap_or(1.0),
        min.and_then(|color| color.b).unwrap_or(1.0),
        min.and_then(|color| color.a).unwrap_or(1.0),
        max.and_then(|color| color.r).unwrap_or(1.0),
        max.and_then(|color| color.g).unwrap_or(1.0),
        max.and_then(|color| color.b).unwrap_or(1.0),
        max.and_then(|color| color.a).unwrap_or(1.0),
    )
}

fn emit_tiling_flags_code(texture: &crate::xml::TextureXml) -> String {
    let mut code = String::new();
    if texture.wants_horiz_tile() {
        code.push_str("\n        tex:SetHorizTile(true)\n        ");
    }
    if texture.wants_vert_tile() {
        code.push_str("\n        tex:SetVertTile(true)\n        ");
    }
    if texture.set_all_points == Some(true) {
        code.push_str("\n        tex:SetAllPoints(true)\n        ");
    }
    code
}

fn emit_parent_key_code(texture: &crate::xml::TextureXml) -> String {
    let mut code = String::new();
    if let Some(key) = &texture.parent_key {
        let parent_field = lua_table_field_ref("parent", key);
        code.push_str(&format!(
            r#"
        {} = tex
        "#,
            parent_field
        ));
    }
    if let Some(parent_array) = &texture.parent_array {
        let array_ref = lua_table_field_ref("parent", parent_array);
        code.push_str(&format!(
            r#"
        {array_ref} = {array_ref} or {{}}
        table.insert({array_ref}, tex)
        "#,
        ));
    }
    code
}

/// Generate Lua code for texture visual properties (color, tiling, parentKey).
fn generate_texture_visual_code(texture: &crate::xml::TextureXml) -> String {
    let mut code = String::new();
    code.push_str(&emit_color_code(texture));
    code.push_str(&emit_gradient_code(texture));
    code.push_str(&emit_tiling_flags_code(texture));
    code.push_str(&emit_parent_key_code(texture));
    code
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

fn emit_create_header(
    tex_name: &str,
    parent_name: &str,
    create_method: &str,
    draw_layer: &str,
    sub_level: i32,
) -> String {
    let mut code = format!(
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
        code.push_str(&format!(
            "\n        tex:SetDrawLayer(\"{}\", {})\n        ",
            draw_layer, sub_level
        ));
    }
    code
}

fn emit_line_thickness(texture: &crate::xml::TextureXml, is_line: bool) -> String {
    if is_line {
        if let Some(t) = texture.thickness {
            return format!("\n        tex:SetThickness({})\n        ", t);
        }
    }
    String::new()
}

fn emit_texture_visibility(texture: &crate::xml::TextureXml) -> String {
    let mut code = String::new();
    if texture.hidden == Some(true) {
        code.push_str("\n        tex:Hide()\n        ");
    }
    if let Some(a) = texture.alpha {
        code.push_str(&format!("\n        tex:SetAlpha({})\n        ", a));
    }
    if let Some(mode) = texture.effective_blend_mode() {
        code.push_str(&format!(
            "\n        tex:SetBlendMode(\"{}\")\n        ",
            mode
        ));
    }
    code
}

fn emit_masked_textures(texture: &crate::xml::TextureXml, is_mask: bool) -> String {
    if !is_mask {
        return String::new();
    }
    let mut code = String::new();
    if let Some(ref masked) = texture.masked_textures {
        for entry in &masked.entries {
            if let Some(ref key) = entry.child_key {
                let parent_field = lua_table_field_ref("parent", key);
                code.push_str(&format!(
                    r#"
        if {parent_field} then {parent_field}:AddMaskTexture(tex) end
        "#,
                ));
            }
        }
    }
    code
}

/// Build the Lua code string that creates and configures a texture.
pub(super) fn build_texture_lua(
    tex_name: &str,
    texture: &crate::xml::TextureXml,
    parent_name: &str,
    subst_parent_name: &str,
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
    let mut code = emit_create_header(tex_name, parent_name, create_method, draw_layer, sub_level);
    code.push_str(&generate_mixin_code(texture));
    code.push_str(&emit_line_thickness(texture, is_line));
    code.push_str(&generate_texture_source_code(texture, is_mask));
    code.push_str(&generate_texture_visual_code(texture));
    append_texture_anchors(&mut code, texture, subst_parent_name);
    code.push_str(&emit_texture_visibility(texture));
    code.push_str(&emit_masked_textures(texture, is_mask));
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
