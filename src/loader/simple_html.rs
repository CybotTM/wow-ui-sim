//! SimpleHTML's direct XML font roles (not child FontString widgets).
use crate::loader::error::LoadError;
use crate::lua_api::LoaderEnv;
use crate::xml::{FrameChildElement, FrameXml};

pub(super) fn apply_simple_html_fonts(
    env: &LoaderEnv<'_>,
    xml: &FrameXml,
    id: u64,
) -> Result<(), LoadError> {
    let is_html = env
        .state()
        .borrow()
        .widgets
        .get(id)
        .is_some_and(|frame| frame.widget_type == crate::widget::WidgetType::SimpleHTML);
    if !is_html {
        return Ok(());
    }
    for child in &xml.children {
        let (role, font) = match child {
            FrameChildElement::FontString(font) => ("p", font),
            FrameChildElement::FontStringHeader1(font) => ("h1", font),
            FrameChildElement::FontStringHeader2(font) => ("h2", font),
            FrameChildElement::FontStringHeader3(font) => ("h3", font),
            _ => continue,
        };
        apply_font_role(env, id, role, font)?;
    }
    Ok(())
}

fn apply_font_role(
    env: &LoaderEnv<'_>,
    id: u64,
    role: &str,
    font: &crate::xml::FontStringXml,
) -> Result<(), LoadError> {
    use crate::loader::helpers::escape_lua_string;
    if let Some(inherits) = &font.inherits {
        env.exec(&format!(
            "__frame_{id}:SetFontObject('{role}', \"{}\")",
            escape_lua_string(inherits)
        ))
        .map_err(|error| LoadError::Lua(error.to_string()))?;
    }
    let mut sim = env.state().borrow_mut();
    let Some(frame) = sim.widgets.get_mut_visual(id) else {
        return Ok(());
    };
    let data = frame.simple_html.get_or_insert_with(Default::default);
    let style = data.text_styles.entry(role.to_string()).or_default();
    if let Some(path) = &font.font {
        style.font = Some(path.clone());
    }
    if let Some(height) = font.font_height.as_ref().and_then(|height| height.value()) {
        style.font_size = height;
    }
    if let Some(spacing) = font.spacing {
        style.spacing = spacing;
    }
    if let Some(justify) = &font.justify_h {
        style.justify_h = justify.clone();
    }
    if let Some(color) = &font.color {
        style.text_color = (
            color.r.unwrap_or(1.0),
            color.g.unwrap_or(1.0),
            color.b.unwrap_or(1.0),
            color.a.unwrap_or(1.0),
        );
    }
    Ok(())
}
