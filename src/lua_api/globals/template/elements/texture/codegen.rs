use crate::loader::chunk_cache;
use crate::loader::helpers::generate_set_point_code;
use crate::loader::helpers_anim::generate_animation_group_code;
use mlua::Lua;

use super::super::super::{escape_lua_string, get_size_values, lua_global_ref};
use super::AnchorParentContext;

pub(crate) fn build_template_texture_lua(
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

fn append_template_texture_mixins(code: &mut String, texture: &crate::xml::TextureXml) {
    for m in &crate::xml::collect_texture_mixins(texture) {
        code.push_str(&format!("if {} then Mixin(tex, {}) end\n", m, m));
    }
}

pub(crate) fn apply_texture_animations(
    lua: &Lua,
    texture: &crate::xml::TextureXml,
    child_name: &str,
) {
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
        if key.contains('.') {
            code.push_str(&format!(
                "            C_Timer.After(0, function() {line} end)\n"
            ));
        } else {
            code.push_str(&format!("            {line}\n"));
        }
    }
}

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

fn append_gradient_code(code: &mut String, grad: &crate::xml::GradientXml, var: &str) {
    let orient = grad.orientation.as_deref().unwrap_or("VERTICAL");
    let [mr, mg, mb, ma] = extract_gradient_rgba(grad.min_color.as_ref(), 1.0);
    let [xr, xg, xb, xa] = extract_gradient_rgba(grad.max_color.as_ref(), 1.0);
    code.push_str(&format!(
        "            {var}:SetGradient(\"{orient}\", {{r={mr},g={mg},b={mb},a={ma}}}, {{r={xr},g={xg},b={xb},a={xa}}})\n"
    ));
}

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

pub(crate) fn append_texture_properties(
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

fn append_size_code(code: &mut String, size: &crate::xml::SizeXml, var: &str) {
    let (width, height) = get_size_values(size);
    match (width, height) {
        (Some(w), Some(h)) => code.push_str(&format!("            {var}:SetSize({w}, {h})\n")),
        (Some(w), None) => code.push_str(&format!("            {var}:SetWidth({w})\n")),
        (None, Some(h)) => code.push_str(&format!("            {var}:SetHeight({h})\n")),
        _ => {}
    }
}

pub(crate) fn append_key_values(
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

pub(crate) fn append_anchors_and_parent_refs(
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

pub(crate) fn apply_deferred_mask_atlases(
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

fn mask_key_matches_atlas_kv(parent_key: &str, kv_key: &str) -> bool {
    let Some(suffix) = kv_key.strip_suffix("Atlas") else {
        return false;
    };
    if lowercase_first(parent_key) == suffix {
        return true;
    }
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
