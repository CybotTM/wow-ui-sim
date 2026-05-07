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
    let process_start = Instant::now();

    for element in &ui.elements {
        lua_count += process_element(env, element, xml_dir, ctx, timing).map_err(|e| {
            if !matches!(e, LoadError::Lua(_)) {
                let _ = env.with_state(|state| {
                    call_error_handler_state(state, &e.to_string());
                    Ok::<(), crate::Error>(())
                });
            }
            e
        })?;
    }
    timing.xml_process_time += process_start.elapsed();

    Ok(lua_count)
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
{name} = {
    __font = "{font_path}",
    __height = {font_height},
    __outline = "{font_outline}",
    __r = 1.0, __g = 1.0, __b = 1.0,
    __justifyH = "{justify_h}",
    __justifyV = "{justify_v}",
    SetTextColor = function(self, r, g, b)
        self.__r = r; self.__g = g; self.__b = b
    end,
    GetFont = function(self)
        return self.__font, self.__height, self.__outline
    end,
    SetFont = function(self, path, height, flags)
        self.__font = path
        if height then self.__height = height end
        if flags then self.__outline = flags end
    end,
    SetJustifyH = function(self, justify)
        self.__justifyH = justify
    end,
    GetJustifyH = function(self)
        return self.__justifyH
    end,
    SetJustifyV = function(self, justify)
        self.__justifyV = justify
    end,
    GetJustifyV = function(self)
        return self.__justifyV
    end,
    CopyFontObject = function(self, source)
        if source.__font then self.__font = source.__font end
        if source.__height then self.__height = source.__height end
        if source.__outline then self.__outline = source.__outline end
        if source.__r then self.__r = source.__r end
        if source.__g then self.__g = source.__g end
        if source.__b then self.__b = source.__b end
        if source.__justifyH then self.__justifyH = source.__justifyH end
        if source.__justifyV then self.__justifyV = source.__justifyV end
    end,
    SetFontObject = function(self, target)
        if type(target) == "string" then target = _G[target] end
        if target then self.__fontObject = target end
    end,
    GetFontObject = function(self)
        return self.__fontObject
    end,
    GetObjectType = function() return "Font" end,
    IsObjectType = function(_, t) return t == "Font" end,
}
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
    }
    if let Some(h) = font.height {
        copy_code.push_str(&format!("{name}.__height = {h}\n"));
    }
    if let Some(o) = &font.outline {
        copy_code.push_str(&format!("{name}.__outline = \"{o}\"\n"));
    }
    if let Some(jh) = &font.justify_h {
        copy_code.push_str(&format!("{name}.__justifyH = \"{jh}\"\n"));
    }
    if let Some(jv) = &font.justify_v {
        copy_code.push_str(&format!("{name}.__justifyV = \"{jv}\"\n"));
    }
}

/// Create a FontFamily object in Lua from XML definition.
const FONT_FAMILY_LUA_TEMPLATE: &str = r#"
{name} = {
    __font = "Fonts/FRIZQT__.TTF",
    __height = 12.0,
    __outline = "",
    __r = 1.0, __g = 1.0, __b = 1.0,
    __justifyH = "CENTER",
    __justifyV = "MIDDLE",
    SetTextColor = function(self, r, g, b)
        self.__r = r; self.__g = g; self.__b = b
    end,
    GetTextColor = function(self)
        return self.__r, self.__g, self.__b
    end,
    SetFont = function(self, font, height, flags)
        if font then self.__font = font end
        if height then self.__height = height end
        if flags then self.__outline = flags end
    end,
    GetFont = function(self)
        return self.__font, self.__height, self.__outline
    end,
    SetJustifyH = function(self, justify)
        self.__justifyH = justify
    end,
    GetJustifyH = function(self)
        return self.__justifyH
    end,
    SetJustifyV = function(self, justify)
        self.__justifyV = justify
    end,
    GetJustifyV = function(self)
        return self.__justifyV
    end,
    CopyFontObject = function(self, source)
        if source.__font then self.__font = source.__font end
        if source.__height then self.__height = source.__height end
        if source.__outline then self.__outline = source.__outline end
        if source.__r then self.__r = source.__r end
        if source.__g then self.__g = source.__g end
        if source.__b then self.__b = source.__b end
    end,
}
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
    }
    if let Some(h) = font.height {
        code.push_str(&format!("{name}.__height = {h}\n"));
    }
    if let Some(o) = &font.outline {
        code.push_str(&format!("{name}.__outline = \"{o}\"\n"));
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
mod tests {
    use super::*;

    fn default_frame() -> FrameXml {
        FrameXml::default()
    }

    /// Helper: call resolve_frame_element and return (widget_type, intrinsic).
    fn resolve(elem: &XmlElement) -> Option<(&'static str, Option<&'static str>)> {
        resolve_frame_element(elem).map(|(_, wt, intr)| (wt, intr))
    }

    #[test]
    fn specialized_widget_types() {
        let f = default_frame();
        assert_eq!(
            resolve(&XmlElement::Frame(f.clone())),
            Some(("Frame", None))
        );
        assert_eq!(
            resolve(&XmlElement::Button(f.clone())),
            Some(("Button", None))
        );
        assert_eq!(
            resolve(&XmlElement::ItemButton(f.clone())),
            Some(("Button", Some("ItemButton")))
        );
        assert_eq!(
            resolve(&XmlElement::CheckButton(f.clone())),
            Some(("CheckButton", None))
        );
        assert_eq!(
            resolve(&XmlElement::EditBox(f.clone())),
            Some(("EditBox", None))
        );
        assert_eq!(
            resolve(&XmlElement::EventEditBox(f.clone())),
            Some(("EditBox", Some("EventEditBox")))
        );
        assert_eq!(
            resolve(&XmlElement::ScrollFrame(f.clone())),
            Some(("ScrollFrame", None))
        );
        assert_eq!(
            resolve(&XmlElement::EventScrollFrame(f.clone())),
            Some(("ScrollFrame", Some("EventScrollFrame")))
        );
        assert_eq!(
            resolve(&XmlElement::Slider(f.clone())),
            Some(("Slider", None))
        );
        assert_eq!(
            resolve(&XmlElement::StatusBar(f.clone())),
            Some(("StatusBar", None))
        );
        assert_eq!(
            resolve(&XmlElement::Cooldown(f.clone())),
            Some(("Cooldown", None))
        );
        assert_eq!(
            resolve(&XmlElement::GameTooltip(f.clone())),
            Some(("GameTooltip", None))
        );
        assert_eq!(
            resolve(&XmlElement::ColorSelect(f.clone())),
            Some(("ColorSelect", None))
        );
        assert_eq!(
            resolve(&XmlElement::Model(f.clone())),
            Some(("Model", None))
        );
        assert_eq!(
            resolve(&XmlElement::ModelScene(f.clone())),
            Some(("ModelScene", None))
        );
        assert_eq!(
            resolve(&XmlElement::SimpleHTML(f.clone())),
            Some(("SimpleHTML", None))
        );
        assert_eq!(
            resolve(&XmlElement::Minimap(f.clone())),
            Some(("Minimap", None))
        );
        assert_eq!(
            resolve(&XmlElement::MessageFrame(f.clone())),
            Some(("MessageFrame", None))
        );
    }

    #[test]
    fn player_model_variants_all_map_to_player_model() {
        let f = default_frame();
        assert_eq!(
            resolve(&XmlElement::PlayerModel(f.clone())),
            Some(("PlayerModel", None))
        );
        assert_eq!(
            resolve(&XmlElement::CinematicModel(f.clone())),
            Some(("PlayerModel", None))
        );
        assert_eq!(
            resolve(&XmlElement::TabardModel(f.clone())),
            Some(("PlayerModel", None))
        );
        assert_eq!(
            resolve(&XmlElement::DressUpModel(f.clone())),
            Some(("PlayerModel", None))
        );
    }

    #[test]
    fn button_intrinsic_variants() {
        let f = default_frame();
        // DropDownToggleButton and EventButton map to plain Button (no intrinsic) in XmlElement
        assert_eq!(
            resolve(&XmlElement::DropDownToggleButton(f.clone())),
            Some(("Button", None))
        );
        assert_eq!(
            resolve(&XmlElement::EventButton(f.clone())),
            Some(("Button", None))
        );
        // DropdownButton has intrinsic
        assert_eq!(
            resolve(&XmlElement::DropdownButton(f.clone())),
            Some(("Button", Some("DropdownButton")))
        );
        // ContainedAlertFrame has intrinsic
        assert_eq!(
            resolve(&XmlElement::ContainedAlertFrame(f.clone())),
            Some(("Button", Some("ContainedAlertFrame")))
        );
    }

    #[test]
    fn scrolling_message_frame_has_intrinsic() {
        let f = default_frame();
        assert_eq!(
            resolve(&XmlElement::ScrollingMessageFrame(f)),
            Some(("MessageFrame", Some("ScrollingMessageFrame")))
        );
    }

    #[test]
    fn frame_like_elements_preserve_supported_alias_types() {
        let f = default_frame();
        let preserved_aliases = [
            XmlElement::EventFrame(f.clone()),
            XmlElement::UnitPositionFrame(f.clone()),
            XmlElement::OffScreenFrame(f.clone()),
            XmlElement::Checkout(f.clone()),
            XmlElement::FogOfWarFrame(f.clone()),
            XmlElement::QuestPOIFrame(f.clone()),
            XmlElement::ArchaeologyDigSiteFrame(f.clone()),
            XmlElement::ScenarioPOIFrame(f.clone()),
            XmlElement::Browser(f.clone()),
            XmlElement::MovieFrame(f.clone()),
        ];
        for elem in &preserved_aliases {
            let (_, tag) = elem.as_frame_data().unwrap();
            assert_eq!(
                resolve(elem),
                Some((tag, None)),
                "Expected preserved type for {:?}",
                std::mem::discriminant(elem)
            );
        }
    }

    #[test]
    fn unsupported_frame_like_elements_still_fall_back_to_frame() {
        let f = default_frame();
        let frame_likes = [
            XmlElement::TaxiRouteFrame(f.clone()),
            XmlElement::ModelFFX(f.clone()),
            XmlElement::UiCamera(f.clone()),
            XmlElement::UIThemeContainerFrame(f.clone()),
            XmlElement::MapScene(f.clone()),
            XmlElement::Line(f.clone()),
            XmlElement::WorldFrame(f.clone()),
        ];
        for elem in &frame_likes {
            assert_eq!(
                resolve(elem),
                Some(("Frame", None)),
                "Expected Frame for {:?}",
                std::mem::discriminant(elem)
            );
        }
    }

    #[test]
    fn non_frame_elements_return_none() {
        use crate::xml::ScriptXml;
        assert_eq!(
            resolve(&XmlElement::Script(ScriptXml {
                file: None,
                inline: None
            })),
            None
        );
        assert_eq!(resolve(&XmlElement::Text("hello".into())), None);
        assert_eq!(resolve(&XmlElement::Unknown), None);
    }

    /// Document the differences between XmlElement and FrameElement mappings.
    /// ItemButton resolves as a Button with the ItemButton intrinsic base here,
    /// while FrameElement preserves the raw alias and inherits are resolved later.
    /// DropDownToggleButton/EventButton have no intrinsic here but do in FrameElement.
    #[test]
    fn xml_vs_frame_element_divergences() {
        let f = default_frame();
        // XmlElement::ItemButton -> ("Button", Some("ItemButton"))
        assert_eq!(
            resolve(&XmlElement::ItemButton(f.clone())),
            Some(("Button", Some("ItemButton")))
        );
        // XmlElement::DropDownToggleButton -> ("Button", None) — no intrinsic
        assert_eq!(
            resolve(&XmlElement::DropDownToggleButton(f.clone())),
            Some(("Button", None))
        );
        // XmlElement::EventButton -> ("Button", None) — no intrinsic
        assert_eq!(
            resolve(&XmlElement::EventButton(f.clone())),
            Some(("Button", None))
        );
    }

    #[test]
    fn roman_font_overrides_with_all_fields() {
        let ff = crate::xml::FontFamilyXml {
            name: Some("TestFont".to_string()),
            is_virtual: None,
            members: vec![crate::xml::FontFamilyMemberXml {
                alphabet: Some("roman".to_string()),
                font: Some(crate::xml::FontXml {
                    font: Some("Fonts\\Test.TTF".to_string()),
                    height: Some(14.0),
                    outline: Some("OUTLINE".to_string()),
                    ..Default::default()
                }),
            }],
        };
        let code = build_roman_font_overrides("TestFont", &ff);
        assert!(code.contains("TestFont.__font = \"Fonts/Test.TTF\""));
        assert!(code.contains("TestFont.__height = 14"));
        assert!(code.contains("TestFont.__outline = \"OUTLINE\""));
    }

    #[test]
    fn roman_font_overrides_no_roman_member() {
        let ff = crate::xml::FontFamilyXml {
            name: Some("TestFont".to_string()),
            is_virtual: None,
            members: vec![crate::xml::FontFamilyMemberXml {
                alphabet: Some("hangul".to_string()),
                font: Some(crate::xml::FontXml::default()),
            }],
        };
        let code = build_roman_font_overrides("TestFont", &ff);
        assert!(code.is_empty());
    }

    #[test]
    fn roman_font_overrides_partial_fields() {
        let ff = crate::xml::FontFamilyXml {
            name: Some("TestFont".to_string()),
            is_virtual: None,
            members: vec![crate::xml::FontFamilyMemberXml {
                alphabet: Some("roman".to_string()),
                font: Some(crate::xml::FontXml {
                    height: Some(16.0),
                    ..Default::default()
                }),
            }],
        };
        let code = build_roman_font_overrides("TestFont", &ff);
        assert!(!code.contains("__font"));
        assert!(code.contains("TestFont.__height = 16"));
        assert!(!code.contains("__outline"));
    }
}
