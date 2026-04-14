use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::{extract_frame_id, get_sim_state};
use mlua::Value;

pub(super) fn add_texture_visual_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_vertex_color_methods(methods);
    add_gradient_methods(methods);
    add_tex_coord_methods(methods);
    add_mask_methods(methods);
    add_rotation_methods(methods);
    add_draw_layer_methods(methods);
    add_visual_methods(methods);
}

fn add_vertex_color_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_vertex_color(methods);
    add_get_vertex_color(methods);
    methods.add_method("SetCenterColor", |_, _this, _args: mlua::MultiValue| Ok(()));
}

fn add_set_vertex_color<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "SetVertexColor",
        |lua, this, (r, g, b, a): (Option<f32>, Option<f32>, Option<f32>, Option<f32>)| {
            let Some(new_color) = vertex_color_from_args(r, g, b, a) else {
                return Ok(());
            };
            let state_rc = get_sim_state(lua);
            if vertex_color_matches(&state_rc.borrow(), this.0, &new_color) {
                return Ok(());
            }
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(this.0) {
                apply_vertex_color(frame, new_color);
            }
            Ok(())
        },
    );
}

fn add_get_vertex_color<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetVertexColor", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(read_vertex_color(&state, this.0))
    });
}

fn vertex_color_from_args(
    r: Option<f32>,
    g: Option<f32>,
    b: Option<f32>,
    a: Option<f32>,
) -> Option<crate::widget::Color> {
    Some(crate::widget::Color::new(r?, g?, b?, a.unwrap_or(1.0)))
}

fn vertex_color_matches(
    state: &crate::lua_api::SimState,
    id: u64,
    new_color: &crate::widget::Color,
) -> bool {
    state
        .widgets
        .get(id)
        .and_then(|frame| frame.vertex_color.as_ref())
        .map(|color| same_vertex_color(color, new_color))
        .unwrap_or(false)
}

fn same_vertex_color(a: &crate::widget::Color, b: &crate::widget::Color) -> bool {
    a.r == b.r && a.g == b.g && a.b == b.b && a.a == b.a
}

fn apply_vertex_color(frame: &mut crate::widget::Frame, new_color: crate::widget::Color) {
    frame.vertex_color = Some(new_color);
    frame.alpha = new_color.a;
}

fn read_vertex_color(state: &crate::lua_api::SimState, id: u64) -> (f32, f32, f32, f32) {
    state
        .widgets
        .get(id)
        .and_then(|frame| frame.vertex_color.as_ref())
        .map(|color| (color.r, color.g, color.b, color.a))
        .unwrap_or((1.0, 1.0, 1.0, 1.0))
}

fn add_gradient_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetGradient", |lua, this, args: mlua::MultiValue| {
        let args_vec: Vec<mlua::Value> = args.into_iter().collect();
        if args_vec.is_empty() {
            return Ok(());
        }
        let orientation = match &args_vec[0] {
            mlua::Value::String(s) => s
                .to_str()
                .map(|s| s.to_uppercase())
                .unwrap_or_else(|_| "VERTICAL".to_string()),
            _ => "VERTICAL".to_string(),
        };
        let vertical = orientation != "HORIZONTAL";
        let (min_color, max_color) = if args_vec.len() >= 3 {
            (
                extract_color_from_value(&args_vec[1]),
                extract_color_from_value(&args_vec[2]),
            )
        } else {
            return Ok(());
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.gradient = Some(crate::widget::Gradient {
                vertical,
                min_color,
                max_color,
            });
        }
        Ok(())
    });
}

fn extract_color_from_value(val: &mlua::Value) -> crate::widget::Color {
    match val {
        mlua::Value::Table(t) => {
            let r = t.get::<f32>("r").unwrap_or(0.0);
            let g = t.get::<f32>("g").unwrap_or(0.0);
            let b = t.get::<f32>("b").unwrap_or(0.0);
            let a = t.get::<f32>("a").unwrap_or(1.0);
            crate::widget::Color::new(r, g, b, a)
        }
        _ => crate::widget::Color::new(0.0, 0.0, 0.0, 1.0),
    }
}

fn add_tex_coord_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetTexCoord", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0)
            && let Some((left, right, top, bottom)) = frame.tex_coords
        {
            return Ok((left, top, left, bottom, right, top, right, bottom));
        }
        Ok((
            0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32, 1.0_f32, 0.0_f32, 1.0_f32, 1.0_f32,
        ))
    });
    methods.add_method("SetTexCoord", |lua, this, args: mlua::MultiValue| {
        let args_vec: Vec<Value> = args.into_iter().collect();
        let (raw_quad, left, right, top, bottom) = parse_tex_coord_args(&args_vec);
        let (Some(left), Some(right), Some(top), Some(bottom)) = (left, right, top, bottom) else {
            return Ok(());
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.tex_coords = Some(remap_tex_coords(
                frame.atlas_tex_coords,
                left,
                right,
                top,
                bottom,
            ));
            frame.tex_coords_quad = raw_quad;
        }
        Ok(())
    });
}

fn parse_tex_coord_args(
    args_vec: &[Value],
) -> (
    Option<[f32; 8]>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
) {
    if args_vec.len() >= 8 {
        parse_tex_coord_8_args(args_vec)
    } else if args_vec.len() >= 4 {
        let coords = (
            value_to_f32(&args_vec[0], 0.0),
            value_to_f32(&args_vec[1], 1.0),
            value_to_f32(&args_vec[2], 0.0),
            value_to_f32(&args_vec[3], 1.0),
        );
        (
            None,
            Some(coords.0),
            Some(coords.1),
            Some(coords.2),
            Some(coords.3),
        )
    } else {
        (None, None, None, None, None)
    }
}

fn parse_tex_coord_8_args(
    args_vec: &[Value],
) -> (
    Option<[f32; 8]>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
) {
    let ul_x = value_to_f32(&args_vec[0], 0.0);
    let ul_y = value_to_f32(&args_vec[1], 0.0);
    let ll_x = value_to_f32(&args_vec[2], 0.0);
    let ll_y = value_to_f32(&args_vec[3], 1.0);
    let ur_x = value_to_f32(&args_vec[4], 1.0);
    let ur_y = value_to_f32(&args_vec[5], 0.0);
    let lr_x = value_to_f32(&args_vec[6], 1.0);
    let lr_y = value_to_f32(&args_vec[7], 1.0);
    let raw_quad = Some([ul_x, ul_y, ll_x, ll_y, ur_x, ur_y, lr_x, lr_y]);
    let left = ul_x.min(ll_x).min(ur_x).min(lr_x);
    let right = ul_x.max(ll_x).max(ur_x).max(lr_x);
    let top = ul_y.min(ll_y).min(ur_y).min(lr_y);
    let bottom = ul_y.max(ll_y).max(ur_y).max(lr_y);
    (raw_quad, Some(left), Some(right), Some(top), Some(bottom))
}

fn value_to_f32(value: &Value, default: f32) -> f32 {
    match value {
        Value::Number(n) => *n as f32,
        Value::Integer(n) => *n as f32,
        _ => default,
    }
}

fn remap_tex_coords(
    atlas_tex_coords: Option<(f32, f32, f32, f32)>,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
) -> (f32, f32, f32, f32) {
    if let Some((al, ar, at, ab)) = atlas_tex_coords {
        let aw = ar - al;
        let ah = ab - at;
        (
            al + left * aw,
            al + right * aw,
            at + top * ah,
            at + bottom * ah,
        )
    } else {
        (left, right, top, bottom)
    }
}

fn add_mask_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("AddMaskTexture", |lua, this, mask: Value| {
        let mask_id = extract_frame_id(&mask);
        if let Some(mask_id) = mask_id {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(this.0) {
                if !frame.mask_textures.contains(&mask_id) {
                    frame.mask_textures.push(mask_id);
                }
            }
        }
        Ok(())
    });
    methods.add_method("RemoveMaskTexture", |lua, this, mask: Value| {
        let mask_id = extract_frame_id(&mask);
        if let Some(mask_id) = mask_id {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(this.0) {
                frame.mask_textures.retain(|&mid| mid != mask_id);
            }
        }
        Ok(())
    });
    methods.add_method("GetNumMaskTextures", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map_or(0, |f| f.mask_textures.len()))
    });
    methods.add_method("GetMaskTexture", |_, _this, _index: i32| Ok(Value::Nil));
}

fn add_rotation_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetRotation", |lua, this, radians: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.rotation = radians as f32;
        }
        Ok(())
    });
    methods.add_method("GetRotation", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|f| f.rotation as f64)
            .unwrap_or(0.0))
    });
}

fn add_draw_layer_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_draw_layer_method(methods);
    add_get_draw_layer_method(methods);
}

fn add_set_draw_layer_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetDrawLayer", |lua, this, args: mlua::MultiValue| {
        set_draw_layer(lua, this.0, args)
    });
}

fn set_draw_layer(lua: &mlua::Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<()> {
    let args_vec: Vec<Value> = args.into_iter().collect();
    let Some((layer, sub_layer)) = draw_layer_request_from_args(&args_vec) else {
        return Ok(());
    };
    let state_rc = get_sim_state(lua);
    if draw_layer_matches(&state_rc.borrow(), id, layer, sub_layer) {
        return Ok(());
    }

    apply_draw_layer_change(&state_rc, id, layer, sub_layer);
    Ok(())
}

fn apply_draw_layer_change(
    state_rc: &std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
    layer: crate::widget::DrawLayer,
    sub_layer: i32,
) {
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.draw_layer = layer;
        frame.draw_sub_layer = sub_layer;
    }
    state.invalidate_strata_buckets();
}

fn add_get_draw_layer_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetDrawLayer", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(read_draw_layer(&state, this.0))
    });
}

fn read_draw_layer(state: &crate::lua_api::SimState, id: u64) -> (String, i32) {
    state
        .widgets
        .get(id)
        .map(|frame| (frame.draw_layer.as_str().to_string(), frame.draw_sub_layer))
        .unwrap_or(("ARTWORK".to_string(), 0))
}

fn draw_layer_request_from_args(args: &[Value]) -> Option<(crate::widget::DrawLayer, i32)> {
    let Value::String(layer_value) = args.first()? else {
        return None;
    };
    let layer = crate::widget::DrawLayer::from_str(&layer_value.to_string_lossy())?;
    let sub_layer = match args.get(1) {
        Some(Value::Integer(n)) => *n as i32,
        Some(Value::Number(n)) => *n as i32,
        _ => 0,
    };
    Some((layer, sub_layer))
}

fn draw_layer_matches(
    state: &crate::lua_api::SimState,
    id: u64,
    layer: crate::widget::DrawLayer,
    sub_layer: i32,
) -> bool {
    state
        .widgets
        .get(id)
        .is_some_and(|frame| frame.draw_layer == layer && frame.draw_sub_layer == sub_layer)
}

fn add_visual_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetVisuals", |_, _this, _info: Value| Ok(()));
}
