use super::{elements, elements_text};
use crate::lua_api::SimState;
use crate::xml::{FrameXml, LayerElement, LayerXml, TextureXml};
use mlua::Lua;
use std::cell::RefCell;
use std::rc::Rc;

/// Apply layers (textures and fontstrings) from a template.
///
/// `subst_parent` is the name used for `$parent` substitution in child names.
/// For anonymous frames, this propagates from the nearest named ancestor.
pub(super) fn apply_layers(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    template: &FrameXml,
    frame_name: &str,
    subst_parent: &str,
    use_direct_creation: bool,
) {
    let region_ctx = RegionCreateContext {
        lua,
        state,
        frame_name,
        subst_parent,
    };
    for layers in template.layers() {
        for layer in &layers.layers {
            apply_layer_block(region_ctx, layer, use_direct_creation);
        }
    }
}

/// Apply button textures (NormalTexture, PushedTexture, etc.) from a frame/template.
pub(super) fn apply_button_texture_set(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame: &FrameXml,
    frame_name: &str,
    subst_parent: &str,
    use_direct_creation: bool,
) {
    let ctx = ButtonTextureContext {
        region: RegionCreateContext {
            lua,
            state,
            frame_name,
            subst_parent,
        },
        use_direct_creation,
    };
    for (parent_key, setter, tex_opt) in button_texture_specs(frame) {
        let Some(texture) = tex_opt else {
            continue;
        };
        apply_button_texture_spec(ctx, texture, parent_key, setter);
    }
}

#[derive(Clone, Copy)]
struct RegionCreateContext<'a> {
    lua: &'a Lua,
    state: &'a Rc<RefCell<SimState>>,
    frame_name: &'a str,
    subst_parent: &'a str,
}

#[derive(Clone, Copy)]
struct LayerCreateContext<'a> {
    region: RegionCreateContext<'a>,
    draw_layer: &'a str,
    use_direct_creation: bool,
}

#[derive(Clone, Copy)]
struct ButtonTextureContext<'a> {
    region: RegionCreateContext<'a>,
    use_direct_creation: bool,
}

fn apply_layer_block(region: RegionCreateContext<'_>, layer: &LayerXml, use_direct_creation: bool) {
    let draw_layer = layer.level.as_deref().unwrap_or("ARTWORK");
    let ctx = LayerCreateContext {
        region,
        draw_layer,
        use_direct_creation,
    };
    for element in &layer.elements {
        apply_layer_element(ctx, element);
    }
}

fn apply_layer_element(ctx: LayerCreateContext<'_>, element: &LayerElement) {
    match element {
        LayerElement::Texture(texture) => create_texture_layer(ctx, texture, false, false),
        LayerElement::Line(texture) => create_texture_layer(ctx, texture, false, true),
        LayerElement::MaskTexture(texture) => create_texture_layer(ctx, texture, true, false),
        LayerElement::FontString(fontstring) => create_fontstring_layer(ctx, fontstring),
    }
}

fn create_texture_layer(
    ctx: LayerCreateContext<'_>,
    texture: &TextureXml,
    is_mask: bool,
    is_line: bool,
) {
    let used_direct = ctx.use_direct_creation
        && elements::create_texture_from_template_direct(
            ctx.region.lua,
            ctx.region.state,
            texture,
            ctx.region.frame_name,
            ctx.region.subst_parent,
            ctx.draw_layer,
            is_mask,
            is_line,
        )
        .is_ok();
    if used_direct {
        return;
    }

    elements::create_texture_from_template(
        ctx.region.lua,
        texture,
        ctx.region.frame_name,
        ctx.region.subst_parent,
        ctx.draw_layer,
        is_mask,
        is_line,
    );
}

fn create_fontstring_layer(ctx: LayerCreateContext<'_>, fontstring: &crate::xml::FontStringXml) {
    let used_direct = ctx.use_direct_creation
        && elements_text::create_fontstring_from_template_direct(
            ctx.region.lua,
            ctx.region.state,
            fontstring,
            ctx.region.frame_name,
            ctx.region.subst_parent,
            ctx.draw_layer,
        )
        .is_ok();
    if used_direct {
        return;
    }

    elements_text::create_fontstring_from_template(
        ctx.region.lua,
        fontstring,
        ctx.region.frame_name,
        ctx.region.subst_parent,
        ctx.draw_layer,
    );
}

fn button_texture_specs(
    frame: &FrameXml,
) -> [(&'static str, &'static str, Option<&TextureXml>); 6] {
    [
        ("Normal", "SetNormalTexture", frame.normal_texture()),
        ("Pushed", "SetPushedTexture", frame.pushed_texture()),
        ("Disabled", "SetDisabledTexture", frame.disabled_texture()),
        (
            "Highlight",
            "SetHighlightTexture",
            frame.highlight_texture(),
        ),
        ("Checked", "SetCheckedTexture", frame.checked_texture()),
        (
            "DisabledChecked",
            "SetDisabledCheckedTexture",
            frame.disabled_checked_texture(),
        ),
    ]
}

fn apply_button_texture_spec(
    ctx: ButtonTextureContext<'_>,
    texture: &TextureXml,
    parent_key: &str,
    setter: &str,
) {
    let used_direct = ctx.use_direct_creation
        && elements_text::create_button_texture_from_template_direct(
            ctx.region.lua,
            ctx.region.state,
            texture,
            ctx.region.frame_name,
            ctx.region.subst_parent,
            parent_key,
        )
        .is_ok();
    if used_direct {
        return;
    }

    elements_text::create_button_texture_from_template(
        ctx.region.lua,
        texture,
        ctx.region.frame_name,
        ctx.region.subst_parent,
        parent_key,
        setter,
    );
}
