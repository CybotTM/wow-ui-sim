//! StatusBar widget methods.

use super::super::handle::FrameRef;
use super::widget_tooltip::val_to_f32;
use crate::lua_api::frame::handle::{frame_ref, get_sim_state};
use crate::widget::Color;
use mlua::Value;

pub fn add_statusbar_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_statusbar_texture_methods(methods);
    add_statusbar_color_methods(methods);
    add_statusbar_fill_methods(methods);
    add_statusbar_desaturate_methods(methods);
    add_statusbar_stubs(methods);
}

fn add_statusbar_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetInterpolatedValue", |_, _, ()| Ok(0.0_f64));
    methods.add_method("GetStatusBarDesaturation", |_, _, ()| Ok(0.0_f64));
    methods.add_method("GetTimerDuration", |_, _, ()| Ok(0.0_f64));
    methods.add_method("IsInterpolating", |_, _, ()| Ok(false));
    methods.add_method("IsStatusBarDesaturated", |_, _, ()| Ok(false));
    methods.add_method(
        "SetStatusBarDesaturation",
        |_, _, _: mlua::Variadic<Value>| Ok(()),
    );
    methods.add_method("SetTimerDuration", |_, _, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("SetToTargetValue", |_, _, _: mlua::Variadic<Value>| Ok(()));
}

fn add_statusbar_texture_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_statusbar_texture(methods);
    add_get_statusbar_texture(methods);
    methods.add_method("SetRotatesTexture", |_, _this, _rotates: bool| Ok(()));
}

fn add_set_statusbar_texture<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetStatusBarTexture", |lua, this, texture: Value| {
        let id = this.0;
        let (path, bar_id) = match &texture {
            Value::String(s) => (Some(s.to_string_lossy().to_string()), None),
            Value::UserData(_) => (None, crate::lua_api::frame::extract_frame_id(&texture)),
            _ => (None, None),
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.statusbar_texture_path = path.clone();
            if matches!(&texture, Value::Nil) {
                frame.statusbar_bar_id = None;
            } else if let Some(bid) = bar_id {
                frame.statusbar_bar_id = Some(bid);
            }
        }
        if let Some(ref tex_str) = path {
            apply_statusbar_texture_path(lua, &mut state, id, tex_str);
        }
        if let Some(bid) = bar_id {
            anchor_bar_to_parent(&mut state.widgets, bid, id);
        }
        Ok(true)
    });
}

fn add_get_statusbar_texture<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetStatusBarTexture", |lua, this, ()| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let bar_id = state.widgets.get(id).and_then(|f| f.statusbar_bar_id);
        if let Some(bar_id) = bar_id {
            if state
                .widgets
                .get(bar_id)
                .is_some_and(|f| f.parent_id == Some(id))
            {
                return frame_ref(lua, bar_id);
            }
        }
        Ok(Value::Nil)
    });
}

fn add_statusbar_color_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_statusbar_color(methods);
    add_get_statusbar_color(methods);
}

fn add_set_statusbar_color<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetStatusBarColor", |lua, this, args: mlua::MultiValue| {
        let color = parse_statusbar_color(args);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(bar_id) = statusbar_child_id(&state.widgets, this.0)
            && let Some(bar) = state.widgets.get_mut_visual(bar_id)
        {
            bar.vertex_color = Some(color);
        }
        Ok(())
    });
}

fn add_get_statusbar_color<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetStatusBarColor", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(bar_id) = statusbar_child_id(&state.widgets, this.0)
            && let Some(c) = state
                .widgets
                .get(bar_id)
                .and_then(|f| f.vertex_color.as_ref())
        {
            return Ok((c.r, c.g, c.b, c.a));
        }
        Ok((1.0_f32, 1.0_f32, 1.0_f32, 1.0_f32))
    });
}

fn parse_statusbar_color(args: mlua::MultiValue) -> Color {
    let mut it = args.into_iter();
    let r = val_to_f32(it.next(), 1.0);
    let g = val_to_f32(it.next(), 1.0);
    let b = val_to_f32(it.next(), 1.0);
    let a = val_to_f32(it.next(), 1.0);
    Color::new(r, g, b, a)
}

fn statusbar_child_id(widgets: &crate::widget::WidgetRegistry, id: u64) -> Option<u64> {
    let bar_id = widgets.get(id).and_then(|frame| frame.statusbar_bar_id)?;
    widgets
        .get(bar_id)
        .is_some_and(|frame| frame.parent_id == Some(id))
        .then_some(bar_id)
}

fn add_statusbar_fill_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFillStyle", |lua, this, style: String| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.statusbar_fill_style = style;
        }
        Ok(())
    });
    methods.add_method("SetReverseFill", |lua, this, reverse: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.statusbar_reverse_fill = reverse;
        }
        Ok(())
    });
}

fn add_statusbar_desaturate_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetStatusBarDesaturated", |_, _this, _desat: bool| Ok(()));
    methods.add_method("GetStatusBarDesaturated", |_, _this, ()| Ok(false));
    methods.add_method("SetStatusBarAtlas", |lua, this, atlas: String| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.statusbar_texture_path = Some(atlas.clone());
        }
        apply_statusbar_texture_path(lua, &mut state, id, &atlas);
        Ok(())
    });
    methods.add_method("GetFillStyle", |_, _this, ()| Ok("STANDARD"));
    methods.add_method("GetReverseFill", |_, _this, ()| Ok(false));
    methods.add_method("GetRotatesTexture", |_, _this, ()| Ok(false));
}

fn apply_statusbar_texture_path(
    lua: &mlua::Lua,
    state: &mut crate::lua_api::SimState,
    id: u64,
    tex_str: &str,
) {
    let bar_child_id = find_bar_texture_child(&state.widgets, id).unwrap_or_else(|| {
        super::methods_helpers::get_or_create_button_texture(lua, state, id, "StatusBarTexture")
    });
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.statusbar_bar_id = Some(bar_child_id);
    }
    apply_bar_texture(&mut state.widgets, bar_child_id, tex_str);
    anchor_bar_to_parent(&mut state.widgets, bar_child_id, id);
}

fn find_bar_texture_child(widgets: &crate::widget::WidgetRegistry, parent_id: u64) -> Option<u64> {
    let frame = widgets.get(parent_id)?;
    let candidate = frame
        .statusbar_bar_id
        .or_else(|| frame.children_keys.get("BarTexture").copied())
        .or_else(|| frame.children_keys.get("StatusBarTexture").copied())
        .or_else(|| frame.children_keys.get("Bar").copied());
    candidate.filter(|&id| {
        widgets
            .get(id)
            .is_some_and(|f| f.parent_id == Some(parent_id))
    })
}

fn apply_bar_texture(widgets: &mut crate::widget::WidgetRegistry, child_id: u64, tex_str: &str) {
    if let Some(lookup) = crate::atlas::get_atlas_info(tex_str) {
        let info = lookup.info;
        if let Some(frame) = widgets.get_mut_visual(child_id) {
            frame.texture = Some(info.file.to_string());
            let uvs = (
                info.left_tex_coord,
                info.right_tex_coord,
                info.top_tex_coord,
                info.bottom_tex_coord,
            );
            frame.atlas_tex_coords = Some(uvs);
            frame.tex_coords = Some(uvs);
            frame.horiz_tile = info.tiles_horizontally;
            frame.vert_tile = info.tiles_vertically;
            frame.atlas = Some(tex_str.to_string());
        }
    } else if let Some(frame) = widgets.get_mut_visual(child_id) {
        frame.texture = Some(tex_str.to_string());
        frame.atlas = None;
        frame.tex_coords = None;
        frame.tex_coords_quad = None;
        frame.atlas_tex_coords = None;
    }
}

fn anchor_bar_to_parent(widgets: &mut crate::widget::WidgetRegistry, bar_id: u64, parent_id: u64) {
    use crate::widget::{Anchor, AnchorPoint};
    if let Some(bar) = widgets.get_mut_visual(bar_id) {
        bar.anchors = vec![
            Anchor {
                point: AnchorPoint::TopLeft,
                relative_to: None,
                relative_to_id: Some(parent_id as usize),
                relative_point: AnchorPoint::TopLeft,
                x_offset: 0.0,
                y_offset: 0.0,
            },
            Anchor {
                point: AnchorPoint::BottomRight,
                relative_to: None,
                relative_to_id: Some(parent_id as usize),
                relative_point: AnchorPoint::BottomRight,
                x_offset: 0.0,
                y_offset: 0.0,
            },
        ];
    }
}
