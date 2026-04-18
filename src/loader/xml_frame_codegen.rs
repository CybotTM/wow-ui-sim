//! Lua code generation for XML frame creation.
//!
//! Builds the Lua source string that `create_frame_from_xml` executes to
//! instantiate a frame: CreateFrame call, parentKey, mixins, KeyValues,
//! attributes, and script handlers.

use super::helpers::{
    escape_lua_string, generate_scripts_code, lua_global_ref, lua_table_field_ref,
};

/// Build the complete Lua code string for creating a frame from XML.
pub(super) fn build_frame_lua_code(
    widget_type: &str,
    name: &str,
    explicit_parent: Option<&str>,
    inherits: &str,
    frame: &crate::xml::FrameXml,
    parent: &str,
) -> String {
    let mut lua_code = build_create_frame_code(widget_type, name, explicit_parent, inherits);
    append_parent_key_code(&mut lua_code, frame, inherits, parent);
    append_mixins_code(&mut lua_code, frame, inherits);
    append_key_values_code(&mut lua_code, frame, inherits);
    append_xml_attributes_code(&mut lua_code, frame);
    // SetID must be in the Lua chunk (not deferred to Rust direct-set) because
    // template child OnLoad handlers may call GetParent():GetID() during
    // fire_deferred_child_onloads, which runs before apply_xml_properties_direct.
    if let Some(id) = frame.xml_id {
        lua_code.push_str(&format!("\n        frame:SetID({})", id));
    }
    append_scripts_code(&mut lua_code, frame);
    lua_code
}

/// Build the initial `CreateFrame(...)` Lua code.
fn build_create_frame_code(
    widget_type: &str,
    name: &str,
    parent: Option<&str>,
    inherits: &str,
) -> String {
    let inherits_arg = if inherits.is_empty() {
        "nil".to_string()
    } else {
        format!("\"{}\"", inherits)
    };
    // Engine-root frames (e.g. UIParent) are pre-created without a parent.
    // When XML defines them, name == default parent, which would self-parent.
    // Reuse the existing engine frame instead.
    if let Some(p) = parent {
        if name == p {
            return format!(
                r#"
        local frame = _G["{name}"]
        "#,
            );
        }
    }
    let parent_arg = match parent {
        Some(p) => format!("{} or UIParent", lua_global_ref(p)),
        // Lua CreateFrame defaults nil parent to UIParent, so pass UIParent
        // here and orphan the frame with SetParent(nil) afterwards.
        None => "UIParent".to_string(),
    };
    let orphan_code = if parent.is_none() {
        // In WoW, top-level XML frames without a parent attribute are created
        // as orphans (no parent). Our Lua CreateFrame always defaults to
        // UIParent, so we create with UIParent then immediately orphan.
        "\n        frame:SetParent(nil)"
    } else {
        ""
    };
    format!(
        r#"
        local frame = CreateFrame("{widget_type}", "{name}", {parent_arg}, {inherits_arg}){orphan_code}
        "#,
    )
}

/// Append parentKey assignment so sibling frames can reference this frame.
///
/// Handles `$parent` prefix in parentKey (e.g. `$parent.CloseButton`)
/// which navigates up from the direct parent before setting the key.
fn append_parent_key_code(
    lua_code: &mut String,
    frame: &crate::xml::FrameXml,
    inherits: &str,
    parent: &str,
) {
    if let Some(parent_key) = resolve_inherited_string(frame, inherits, |f| f.parent_key.as_ref()) {
        let parent_ref = lua_global_ref(parent);
        if let Some(key) = parent_key.strip_prefix("$parent.") {
            let parent_field = lua_table_field_ref("__pk", key);
            lua_code.push_str(&format!(
                r#"
        do local __pk = {}:GetParent(); if __pk then {} = frame end end
        "#,
                parent_ref, parent_field
            ));
        } else {
            let parent_field = lua_table_field_ref(&parent_ref, &parent_key);
            lua_code.push_str(&format!(
                r#"
        {} = frame
        "#,
                parent_field
            ));
        }
    }
    append_parent_array_code(lua_code, frame, inherits, parent);
}

/// Append parentArray insertion when the attribute is directly on this frame.
///
/// Template-inherited parentArray is handled by `apply_parent_array_from_template`
/// inside `CreateFrame`, so we only handle the direct-attribute case here.
fn append_parent_array_code(
    lua_code: &mut String,
    frame: &crate::xml::FrameXml,
    inherits: &str,
    parent: &str,
) {
    if let Some(parent_array) =
        resolve_inherited_string(frame, inherits, |f| f.parent_array.as_ref())
    {
        let parent_ref = lua_global_ref(parent);
        let array_ref = lua_table_field_ref(&parent_ref, &parent_array);
        lua_code.push_str(&format!(
            "\n        {array_ref} = {array_ref} or {{}}\n        \
             table.insert({array_ref}, frame)\n        ",
        ));
    }
}

fn resolve_inherited_string(
    frame: &crate::xml::FrameXml,
    inherits: &str,
    getter: impl Fn(&crate::xml::FrameXml) -> Option<&String>,
) -> Option<String> {
    getter(frame).cloned().or_else(|| {
        if inherits.is_empty() {
            return None;
        }
        crate::xml::get_template_chain(inherits)
            .iter()
            .rev()
            .find_map(|entry| getter(&entry.frame).cloned())
    })
}

/// Append Mixin() calls for the frame's own (direct) mixins only.
///
/// Template-inherited mixins are already applied inside CreateFrame by
/// `apply_templates_from_registry` → `apply_single_template` → `apply_mixin`,
/// so we only need the frame's own mixin attribute here.
fn append_mixins_code(lua_code: &mut String, frame: &crate::xml::FrameXml, _inherits: &str) {
    let mut all_mixins: Vec<String> = Vec::new();

    // Only direct mixins — template mixins are applied during CreateFrame
    collect_mixins_from_attr(&mut all_mixins, frame.combined_mixin().as_deref());

    for m in &all_mixins {
        lua_code.push_str(&format!(
            "\n        do local m = {m} or (__secureenv and rawget(__secureenv, \"{m}\")) \
             if m then Mixin(frame, m) end end"
        ));
    }
}

/// Parse a comma-separated mixin attribute and append unique entries.
fn collect_mixins_from_attr(all_mixins: &mut Vec<String>, mixin_attr: Option<&str>) {
    if let Some(mixin) = mixin_attr {
        for m in mixin.split(',').map(|s| s.trim()) {
            if !m.is_empty() && !all_mixins.contains(&m.to_string()) {
                all_mixins.push(m.to_string());
            }
        }
    }
}

/// Append KeyValue assignments from the frame's own XML only.
///
/// Template-inherited key values are already applied inside CreateFrame by
/// `apply_templates_from_registry` → `apply_single_template` → `apply_key_values`.
fn append_key_values_code(lua_code: &mut String, frame: &crate::xml::FrameXml, _inherits: &str) {
    for kv in frame.all_key_values() {
        append_key_values_from_xml(lua_code, Some(kv));
    }
}

/// Append `frame.key = value` assignments for a KeyValues block.
fn append_key_values_from_xml(
    lua_code: &mut String,
    key_values: Option<&crate::xml::KeyValuesXml>,
) {
    if let Some(key_values) = key_values {
        for kv in &key_values.values {
            let value = format_key_value_lua(&kv.value, kv.value_type.as_deref());
            let field_ref = lua_table_field_ref("frame", &kv.key);
            lua_code.push_str(&format!(
                r#"
        {} = {}
        "#,
                field_ref, value
            ));
        }
    }
}

/// Generate `var.key = value` assignments for a KeyValues block.
pub(super) fn generate_key_values_code(
    key_values: Option<&crate::xml::KeyValuesXml>,
    var_name: &str,
) -> String {
    let Some(key_values) = key_values else {
        return String::new();
    };
    let mut code = String::new();
    for kv in &key_values.values {
        let value = format_key_value_lua(&kv.value, kv.value_type.as_deref());
        let field_ref = lua_table_field_ref(var_name, &kv.key);
        code.push_str(&format!("\n        {field_ref} = {value}\n        "));
    }
    code
}

/// Format a KeyValue's value as a Lua expression based on its type.
fn format_key_value_lua(value: &str, value_type: Option<&str>) -> String {
    match value_type {
        Some("number") => value.to_string(),
        Some("boolean") => value.to_lowercase(),
        Some("global") if !value.is_empty() => value.to_string(),
        Some("global") => "nil".to_string(),
        // Auto-detect numbers when type is not specified (WoW behavior)
        None if value.parse::<f64>().is_ok() => value.to_string(),
        _ => format!("\"{}\"", escape_lua_string(value)),
    }
}

/// Append SetAttribute calls for `<Attributes>` XML elements.
fn append_xml_attributes_code(lua_code: &mut String, frame: &crate::xml::FrameXml) {
    if let Some(attrs) = frame.xml_attributes() {
        for attr in &attrs.entries {
            let value = match attr.attr_type.as_deref() {
                Some("number") => attr.value.as_deref().unwrap_or("0").to_string(),
                Some("boolean") => attr.value.as_deref().unwrap_or("false").to_lowercase(),
                Some("nil") => "nil".to_string(),
                _ => format!(
                    "\"{}\"",
                    escape_lua_string(attr.value.as_deref().unwrap_or(""))
                ),
            };
            lua_code.push_str(&format!(
                "\n        frame:SetAttribute(\"{}\", {})",
                escape_lua_string(&attr.name),
                value
            ));
        }
    }
}

/// Append script handler registrations from the frame's Scripts element.
fn append_scripts_code(lua_code: &mut String, frame: &crate::xml::FrameXml) {
    if let Some(scripts) = frame.scripts() {
        lua_code.push_str(&generate_scripts_code(scripts));
    }
}
