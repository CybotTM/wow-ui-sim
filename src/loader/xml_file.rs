//! XML file loading and element processing.

use crate::lua_api::LoaderEnv;
use crate::lua_api::globals::security::mark_secure_state;
use crate::lua_api::methods::create_string;
use crate::lua_api::script_helpers::call_error_handler_state;
use crate::xml::{FrameXml, XmlElement, parse_xml_file};
use rilua::{Function, LuaApiMut};
use std::path::Path;
use std::time::Instant;

use super::LoadTiming;
use super::addon::AddonContext;
use super::error::LoadError;
use super::helpers::resolve_path_with_fallback;
use super::lua_file::load_lua_file;
use super::xml_frame::create_frame_from_xml;

/// Load an XML file, processing its elements.
/// Returns the number of Lua files loaded from Script elements.
pub fn load_xml_file(
    env: &LoaderEnv<'_>,
    path: &Path,
    ctx: &AddonContext,
    timing: &mut LoadTiming,
) -> Result<usize, LoadError> {
    let _xml_load_addon_guard = super::enter_xml_load_addon_context(env);
    let xml_start = Instant::now();
    let ui = parse_xml_file(path).map_err(|e| {
        let _ = env.with_state(|state| {
            call_error_handler_state(state, &e.to_string());
            Ok::<(), crate::Error>(())
        });
        LoadError::Xml(e)
    })?;
    timing.xml_parse_time += xml_start.elapsed();

    let xml_dir = path.parent().unwrap_or(Path::new("."));
    let mut lua_count = 0;

    for element in &ui.elements {
        lua_count += process_element_with_exclusive_timing(env, element, xml_dir, ctx, timing)
            .map_err(|e| {
                if !matches!(e, LoadError::Lua(_)) {
                    let _ = env.with_state(|state| {
                        call_error_handler_state(state, &e.to_string());
                        Ok::<(), crate::Error>(())
                    });
                }
                e
            })?;
    }

    Ok(lua_count)
}

/// Attribute only time not already counted in another timing bucket (Lua
/// compile/exec, I/O, nested XML) to `xml_process_time`.
fn process_element_with_exclusive_timing(
    env: &LoaderEnv<'_>,
    element: &XmlElement,
    xml_dir: &Path,
    ctx: &AddonContext,
    timing: &mut LoadTiming,
) -> Result<usize, LoadError> {
    let counted_before = timing.independently_counted_time();
    let process_start = Instant::now();
    let result = process_element(env, element, xml_dir, ctx, timing);
    let counted_child_time = timing
        .independently_counted_time()
        .saturating_sub(counted_before);
    timing.xml_process_time += process_start.elapsed().saturating_sub(counted_child_time);
    result
}

/// Process a single top-level XML element.
/// Returns the number of Lua files loaded (0 or 1, or recursive count for includes).
fn process_element(
    env: &LoaderEnv<'_>,
    element: &XmlElement,
    xml_dir: &Path,
    ctx: &AddonContext,
    timing: &mut LoadTiming,
) -> Result<usize, LoadError> {
    match element {
        XmlElement::Script(s) | XmlElement::ScriptLower(s) => {
            process_script(env, s, xml_dir, ctx, timing)
        }
        XmlElement::Include(i) | XmlElement::IncludeLower(i) => {
            process_include(env, i, xml_dir, ctx, timing)
        }
        XmlElement::Font(font) => {
            create_font_object(env, font)?;
            Ok(0)
        }
        XmlElement::FontFamily(font_family) => {
            create_font_family_object(env, font_family)?;
            Ok(0)
        }
        XmlElement::ScopedModifier(scoped) => {
            process_scoped_modifier(env, scoped, xml_dir, ctx, timing)
        }
        XmlElement::Texture(tex) => {
            register_virtual_texture(tex);
            Ok(0)
        }
        XmlElement::FontString(fs) => {
            register_virtual_font_string(fs);
            Ok(0)
        }
        XmlElement::AnimationGroup(ag) => {
            register_virtual_anim_group(ag);
            Ok(0)
        }
        XmlElement::Animation(_) | XmlElement::Binding(_) | XmlElement::ModifiedClick(_) => Ok(0),
        _ => {
            let frame_start = Instant::now();
            process_frame_element(env, element, timing)?;
            timing.xml_frame_create_time += frame_start.elapsed();
            Ok(0)
        }
    }
}

/// Process a ScopedModifier element, temporarily setting forbidden state.
fn process_scoped_modifier(
    env: &LoaderEnv<'_>,
    scoped: &crate::xml::ScopedModifierXml,
    xml_dir: &Path,
    ctx: &AddonContext,
    timing: &mut LoadTiming,
) -> Result<usize, LoadError> {
    let prev_forbidden = env.state().borrow().loading_forbidden;
    if scoped.forbidden.unwrap_or(false) || scoped.full_lockdown.unwrap_or(false) {
        env.state().borrow_mut().loading_forbidden = true;
    }
    let mut count = 0;
    for child in &scoped.elements {
        count += process_element(env, child, xml_dir, ctx, timing)?;
    }
    env.state().borrow_mut().loading_forbidden = prev_forbidden;
    Ok(count)
}

/// Process a Script element (file reference or inline code).
fn process_script(
    env: &LoaderEnv<'_>,
    s: &crate::xml::ScriptXml,
    xml_dir: &Path,
    ctx: &AddonContext,
    timing: &mut LoadTiming,
) -> Result<usize, LoadError> {
    if let Some(file) = &s.file {
        let script_path = resolve_path_with_fallback(xml_dir, ctx.addon_root, file);
        load_lua_file(env, &script_path, ctx, timing)?;
        return Ok(1);
    }

    let Some(inline) = &s.inline else {
        return Ok(0);
    };

    run_inline_script(env, ctx, inline, timing)?;
    Ok(1)
}

fn run_inline_script(
    env: &LoaderEnv<'_>,
    ctx: &AddonContext,
    inline: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    let func = compile_inline_script(env, inline, timing)?;
    let call_start = Instant::now();
    mark_inline_script_secure(env, ctx, &func)?;
    call_inline_script(env, ctx, &func)?;
    record_inline_script_call_timing(timing, call_start);
    Ok(())
}

fn compile_inline_script(
    env: &LoaderEnv<'_>,
    inline: &str,
    timing: &mut LoadTiming,
) -> Result<Function, LoadError> {
    let compile_start = Instant::now();
    let func_result =
        env.with_state(|state| LuaApiMut::load_bytes(state, inline.as_bytes(), "@inline"));
    let compile_elapsed = compile_start.elapsed();
    timing.lua_compile_time += compile_elapsed;
    timing.lua_exec_time += compile_elapsed;

    func_result.map_err(|e| {
        report_inline_script_error(env, &e.to_string());
        LoadError::Lua(e.to_string())
    })
}

fn mark_inline_script_secure(
    env: &LoaderEnv<'_>,
    ctx: &AddonContext,
    func: &Function,
) -> Result<(), LoadError> {
    if !ctx.use_secure_env {
        return Ok(());
    }

    env.with_state(|state| {
        mark_secure_state(state, func).map_err(|e| {
            call_error_handler_state(state, &e.to_string());
            LoadError::Lua(e.to_string())
        })
    })
}

fn call_inline_script(
    env: &LoaderEnv<'_>,
    ctx: &AddonContext,
    func: &Function,
) -> Result<(), LoadError> {
    // In WoW, runtime errors in inline <Script> elements are caught by the
    // error handler and don't abort XML file processing.
    env.with_state(|state| {
        let addon_name = create_string(state, ctx.name);
        if let Err(e) = crate::lua_api::methods::call_function_state(
            state,
            rilua::Val::Function(func.gc_ref()),
            &[addon_name, ctx.table],
        ) {
            call_error_handler_state(state, &e.to_string());
            tracing::warn!("Inline script error: {}", e);
        }
        Ok::<(), LoadError>(())
    })
}

fn record_inline_script_call_timing(timing: &mut LoadTiming, call_start: Instant) {
    let call_elapsed = call_start.elapsed();
    timing.lua_call_time += call_elapsed;
    timing.lua_exec_time += call_elapsed;
}

fn report_inline_script_error(env: &LoaderEnv<'_>, error: &str) {
    let _ = env.with_state(|state| {
        call_error_handler_state(state, error);
        Ok::<(), crate::Error>(())
    });
}

/// Process an Include element (XML or Lua file).
fn process_include(
    env: &LoaderEnv<'_>,
    i: &crate::xml::IncludeXml,
    xml_dir: &Path,
    ctx: &AddonContext,
    timing: &mut LoadTiming,
) -> Result<usize, LoadError> {
    let include_path = resolve_path_with_fallback(xml_dir, ctx.addon_root, &i.file);
    if i.file.ends_with(".lua") {
        // In WoW, Lua errors in <Script file="..."> includes are caught and don't
        // abort XML file processing — same as inline <Script> elements.
        if let Err(e) = load_lua_file(env, &include_path, ctx, timing) {
            tracing::warn!("Script file include error ({}): {}", i.file, e);
        }
        Ok(1)
    } else {
        load_xml_file(env, &include_path, ctx, timing)
    }
}

/// Extract the FrameXml data, widget type, and optional intrinsic name from an XmlElement.
///
/// XmlElement-specific overrides vs the shared `widget_type_for_tag`:
/// - `DropDownToggleButton` and `EventButton` map to plain `"Button"` (no intrinsic)
fn resolve_frame_element(
    element: &XmlElement,
) -> Option<(&FrameXml, &'static str, Option<&'static str>)> {
    let (f, tag) = element.as_frame_data()?;
    let (wt, intrinsic) = match tag {
        "DropDownToggleButton" | "EventButton" => ("Button", None),
        _ => crate::xml::widget_type_for_tag(tag)?,
    };
    Some((f, wt, intrinsic))
}

/// Register a top-level virtual Texture template (e.g. TextStatusBarSparkTemplate).
fn register_virtual_texture(texture: &crate::xml::TextureXml) {
    if texture.is_virtual == Some(true)
        && let Some(ref name) = texture.name
    {
        crate::xml::register_texture_template(name, texture.clone());
    }
}

/// Register a top-level virtual FontString template (e.g.
/// `UserScaledFontStringTemplate`). FontString templates live in their
/// own registry — see `src/xml/template.rs::register_font_string_template`.
fn register_virtual_font_string(fontstring: &crate::xml::FontStringXml) {
    if fontstring.is_virtual == Some(true)
        && let Some(ref name) = fontstring.name
    {
        crate::xml::register_font_string_template(name, fontstring.clone());
    }
}

/// Register a top-level virtual AnimationGroup template.
fn register_virtual_anim_group(anim_group: &crate::xml::AnimationGroupXml) {
    if anim_group.is_virtual == Some(true)
        && let Some(ref name) = anim_group.name
    {
        crate::xml::register_anim_group_template(name, anim_group.clone());
    }
}

/// Process a frame-type XML element by dispatching to create_frame_from_xml.
fn process_frame_element(
    env: &LoaderEnv<'_>,
    element: &XmlElement,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    if let Some((frame_xml, widget_type, intrinsic)) = resolve_frame_element(element) {
        create_frame_from_xml(env, frame_xml, widget_type, None, None, intrinsic, timing)?;
    }
    Ok(())
}

/// Lua template for Font objects. Placeholders: {name}, {font_path}, {font_height},
/// {font_outline}, {justify_h}, {justify_v}.
const FONT_LUA_TEMPLATE: &str = r#"
{name} = CreateFont("{name}")
{name}:SetFont("{font_path}", {font_height}, "{font_outline}")
{name}:SetJustifyH("{justify_h}")
{name}:SetJustifyV("{justify_v}")
{name}.__font = "{font_path}"
{name}.__height = {font_height}
{name}.__outline = "{font_outline}"
{name}.__r = 1.0; {name}.__g = 1.0; {name}.__b = 1.0
"#;

/// Create a Font object in Lua from XML definition.
///
/// When `inherits` is set, copies properties from the parent font first,
/// then overrides with any explicitly specified attributes.
fn create_font_object(env: &LoaderEnv<'_>, font: &crate::xml::FontXml) -> Result<(), LoadError> {
    let Some(name) = &font.name else {
        return Ok(());
    };
    if name.is_empty() {
        return Ok(());
    }

    let font_path = font_path(font);
    let lua_code = build_font_lua_code(name, font, &font_path);
    env.exec(&lua_code)
        .map_err(|e| LoadError::Lua(format!("Failed to create font {}: {}", name, e)))?;

    // Apply inheritance: copy properties from parent, then re-apply explicit overrides.
    if let Some(parent) = &font.inherits {
        let copy_code = build_font_inheritance_code(name, parent, font, &font_path);
        let _ = env.exec(&copy_code);
    }

    Ok(())
}

fn font_path(font: &crate::xml::FontXml) -> String {
    font.font
        .as_deref()
        .unwrap_or("Fonts/FRIZQT__.TTF")
        .replace('\\', "/")
}

fn build_font_lua_code(name: &str, font: &crate::xml::FontXml, font_path: &str) -> String {
    FONT_LUA_TEMPLATE
        .replace("{name}", name)
        .replace("{font_path}", font_path)
        .replace("{font_height}", &font.height.unwrap_or(12.0).to_string())
        .replace("{font_outline}", font.outline.as_deref().unwrap_or(""))
        .replace("{justify_h}", font.justify_h.as_deref().unwrap_or("CENTER"))
        .replace("{justify_v}", font.justify_v.as_deref().unwrap_or("MIDDLE"))
}

fn build_font_inheritance_code(
    name: &str,
    parent: &str,
    font: &crate::xml::FontXml,
    font_path: &str,
) -> String {
    let mut copy_code = format!("if {parent} then {name}:CopyFontObject({parent}) end\n");
    append_font_override_lines(&mut copy_code, name, font, font_path);
    copy_code
}

fn append_font_override_lines(
    copy_code: &mut String,
    name: &str,
    font: &crate::xml::FontXml,
    font_path: &str,
) {
    // Re-apply explicit overrides from the XML so they win over inherited values.
    if font.font.is_some() {
        copy_code.push_str(&format!("{name}.__font = \"{font_path}\"\n"));
        copy_code.push_str(&format!("{name}.__fontPath = \"{font_path}\"\n"));
    }
    if let Some(h) = font.height {
        copy_code.push_str(&format!("{name}.__height = {h}\n"));
        copy_code.push_str(&format!("{name}.__fontHeight = {h}\n"));
    }
    if let Some(o) = &font.outline {
        copy_code.push_str(&format!("{name}.__outline = \"{o}\"\n"));
        copy_code.push_str(&format!("{name}.__fontFlags = \"{o}\"\n"));
    }
    if let Some(jh) = &font.justify_h {
        copy_code.push_str(&format!("{name}:SetJustifyH(\"{jh}\")\n"));
    }
    if let Some(jv) = &font.justify_v {
        copy_code.push_str(&format!("{name}:SetJustifyV(\"{jv}\")\n"));
    }
}

/// Create a FontFamily object in Lua from XML definition.
const FONT_FAMILY_LUA_TEMPLATE: &str = r#"
{name} = CreateFont("{name}")
{name}:SetFont("Fonts/FRIZQT__.TTF", 12.0, "")
{name}:SetTextColor(1.0, 1.0, 1.0)
{name}:SetJustifyH("CENTER")
{name}:SetJustifyV("MIDDLE")
{name}.__font = "Fonts/FRIZQT__.TTF"
{name}.__height = 12.0
{name}.__outline = ""
{name}.__r = 1.0; {name}.__g = 1.0; {name}.__b = 1.0
"#;

fn create_font_family_object(
    env: &LoaderEnv<'_>,
    font_family: &crate::xml::FontFamilyXml,
) -> Result<(), LoadError> {
    let Some(name) = &font_family.name else {
        return Ok(());
    };
    if name.is_empty() {
        return Ok(());
    }
    let lua_code = FONT_FAMILY_LUA_TEMPLATE.replace("{name}", name);
    env.exec(&lua_code)
        .map_err(|e| LoadError::Lua(format!("Failed to create font family {}: {}", name, e)))?;

    let overrides = build_roman_font_overrides(name, font_family);
    if !overrides.is_empty() {
        let _ = env.exec(&overrides);
    }
    Ok(())
}

/// Build Lua override statements from the roman member's font properties.
fn build_roman_font_overrides(name: &str, font_family: &crate::xml::FontFamilyXml) -> String {
    let font = match find_roman_font(font_family) {
        Some(f) => f,
        None => return String::new(),
    };
    let mut code = String::new();
    if let Some(path) = &font.font {
        let p = path.replace('\\', "/");
        code.push_str(&format!("{name}.__font = \"{p}\"\n"));
        code.push_str(&format!("{name}.__fontPath = \"{p}\"\n"));
    }
    if let Some(h) = font.height {
        code.push_str(&format!("{name}.__height = {h}\n"));
        code.push_str(&format!("{name}.__fontHeight = {h}\n"));
    }
    if let Some(o) = &font.outline {
        code.push_str(&format!("{name}.__outline = \"{o}\"\n"));
        code.push_str(&format!("{name}.__fontFlags = \"{o}\"\n"));
    }
    code
}

/// Find the roman alphabet member's font definition.
fn find_roman_font(font_family: &crate::xml::FontFamilyXml) -> Option<&crate::xml::FontXml> {
    font_family
        .members
        .iter()
        .find(|m| m.alphabet.as_deref() == Some("roman"))
        .and_then(|m| m.font.as_ref())
}

#[cfg(test)]
#[path = "xml_file_tests.rs"]
mod tests;
