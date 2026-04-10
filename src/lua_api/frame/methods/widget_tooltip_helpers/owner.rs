use super::shared::{fire_tooltip_script, val_to_f32};
use super::{Anchor, AnchorPoint, Value, build_cursor_anchor, extract_frame_id, get_sim_state};

pub(crate) fn set_owner_impl(lua: &mlua::Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<()> {
    let (owner_id, anchor, x_offset, y_offset) = parse_set_owner_args(lua, args)?;
    {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(td) = state.tooltips.get_mut(&id) {
            td.lines.clear();
            td.spell_id = None;
            td.owner_id = owner_id;
            td.anchor_type = anchor.clone();
            td.anchor_x_offset = x_offset;
            td.anchor_y_offset = y_offset;
        }
        position_tooltip(&mut state, id, owner_id, &anchor, x_offset, y_offset);
        state.set_frame_visible(id, true);
    }
    fire_tooltip_script(lua, id, "OnTooltipCleared")?;
    Ok(())
}

pub(crate) fn set_anchor_type_impl(
    lua: &mlua::Lua,
    id: u64,
    args: mlua::Variadic<Value>,
) -> mlua::Result<()> {
    let (anchor, x_offset, y_offset) = parse_set_anchor_type_args(lua, args)?;
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let owner_id = match state.tooltips.get_mut(&id) {
        Some(td) => {
            td.anchor_type = anchor.clone();
            td.anchor_x_offset = x_offset;
            td.anchor_y_offset = y_offset;
            td.owner_id
        }
        None => return Ok(()),
    };
    position_tooltip(&mut state, id, owner_id, &anchor, x_offset, y_offset);
    Ok(())
}

pub(crate) fn set_object_tooltip_position_impl(lua: &mlua::Lua, id: u64) -> mlua::Result<()> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let owner_id = state.tooltips.get(&id).and_then(|td| td.owner_id);
    let anchor = owner_id.map(build_object_tooltip_anchor);
    apply_tooltip_anchor(&mut state, id, anchor);
    Ok(())
}

/// Parse SetOwner arguments: owner, anchor type (with validation), x/y offsets.
fn parse_set_owner_args(
    lua: &mlua::Lua,
    args: mlua::MultiValue,
) -> mlua::Result<(Option<u64>, String, f32, f32)> {
    let mut args_iter = args.into_iter();
    let owner_val = match args_iter.next() {
        Some(v) if extract_frame_id(&v).is_some() => v,
        _ => {
            return Err(mlua::Error::runtime(
                "Usage: GameTooltip:SetOwner(owner[, anchor])",
            ));
        }
    };
    let anchor = parse_anchor_type_or_default(lua, args_iter.next(), "SetOwner");
    let x_offset = val_to_f32(args_iter.next(), 0.0);
    let y_offset = val_to_f32(args_iter.next(), 0.0);
    Ok((extract_frame_id(&owner_val), anchor, x_offset, y_offset))
}

fn parse_set_anchor_type_args(
    lua: &mlua::Lua,
    args: mlua::Variadic<Value>,
) -> mlua::Result<(String, f32, f32)> {
    let mut args_iter = args.into_iter();
    let anchor = match args_iter.next() {
        Some(value @ Value::String(_)) => {
            parse_anchor_type_or_default(lua, Some(value), "SetAnchorType")
        }
        _ => {
            return Err(mlua::Error::runtime(
                "Usage: GameTooltip:SetAnchorType(anchor[, xOffset, yOffset])",
            ));
        }
    };
    let x_offset = val_to_f32(args_iter.next(), 0.0);
    let y_offset = val_to_f32(args_iter.next(), 0.0);
    Ok((anchor, x_offset, y_offset))
}

fn parse_anchor_type_or_default(
    lua: &mlua::Lua,
    value: Option<Value>,
    caller_name: &str,
) -> String {
    match value {
        Some(Value::String(s)) => {
            let anchor = s.to_string_lossy().to_string();
            if is_valid_anchor_type(&anchor) {
                return anchor;
            }
            crate::lua_api::script_helpers::call_error_handler(
                lua,
                &format!(
                    "{caller_name}: invalid anchor type '{}', defaulting to ANCHOR_LEFT",
                    anchor
                ),
            );
            "ANCHOR_LEFT".to_string()
        }
        _ => "ANCHOR_LEFT".to_string(),
    }
}

fn is_valid_anchor_type(s: &str) -> bool {
    matches!(
        s,
        "ANCHOR_LEFT"
            | "ANCHOR_RIGHT"
            | "ANCHOR_TOP"
            | "ANCHOR_BOTTOM"
            | "ANCHOR_TOPLEFT"
            | "ANCHOR_TOPRIGHT"
            | "ANCHOR_BOTTOMLEFT"
            | "ANCHOR_BOTTOMRIGHT"
            | "ANCHOR_CURSOR"
            | "ANCHOR_PRESERVE"
            | "ANCHOR_NONE"
    )
}

fn position_tooltip(
    state: &mut crate::lua_api::state::SimState,
    tooltip_id: u64,
    owner_id: Option<u64>,
    anchor_type: &str,
    x_offset: f32,
    y_offset: f32,
) {
    if anchor_type == "ANCHOR_PRESERVE" {
        return;
    }
    let anchor = match anchor_type {
        "ANCHOR_CURSOR" => {
            let (mx, my) = state.mouse_position.unwrap_or((0.0, 0.0));
            Some(build_cursor_anchor(mx, my, x_offset, y_offset))
        }
        "ANCHOR_NONE" => None,
        _ => owner_id.map(|oid| build_owner_anchor(anchor_type, oid, x_offset, y_offset)),
    };
    apply_tooltip_anchor(state, tooltip_id, anchor);
}

fn apply_tooltip_anchor(
    state: &mut crate::lua_api::state::SimState,
    tooltip_id: u64,
    anchor: Option<Anchor>,
) {
    let Some(frame) = state.widgets.get_mut_visual(tooltip_id) else {
        return;
    };
    frame.anchors.clear();
    if let Some(anchor) = anchor {
        frame.anchors.push(anchor);
    }
    let _ = frame;
    state.widgets.mark_rect_dirty(tooltip_id);
    state.widgets.mark_visual_dirty(tooltip_id);
}

fn build_owner_anchor(anchor_type: &str, owner_id: u64, x_offset: f32, y_offset: f32) -> Anchor {
    let (tp, rp) = anchor_points_for_type(anchor_type);
    Anchor {
        point: tp,
        relative_to: None,
        relative_to_id: Some(owner_id as usize),
        relative_point: rp,
        x_offset,
        y_offset,
    }
}

fn build_object_tooltip_anchor(owner_id: u64) -> Anchor {
    Anchor {
        point: AnchorPoint::Bottom,
        relative_to: None,
        relative_to_id: Some(owner_id as usize),
        relative_point: AnchorPoint::Top,
        x_offset: 0.0,
        y_offset: 0.0,
    }
}

fn anchor_points_for_type(anchor_type: &str) -> (AnchorPoint, AnchorPoint) {
    match anchor_type {
        "ANCHOR_RIGHT" => (AnchorPoint::Left, AnchorPoint::Right),
        "ANCHOR_LEFT" => (AnchorPoint::Right, AnchorPoint::Left),
        "ANCHOR_TOP" => (AnchorPoint::Bottom, AnchorPoint::Top),
        "ANCHOR_BOTTOM" => (AnchorPoint::Top, AnchorPoint::Bottom),
        "ANCHOR_TOPLEFT" => (AnchorPoint::BottomLeft, AnchorPoint::TopLeft),
        "ANCHOR_TOPRIGHT" => (AnchorPoint::BottomRight, AnchorPoint::TopRight),
        "ANCHOR_BOTTOMLEFT" => (AnchorPoint::TopLeft, AnchorPoint::BottomLeft),
        "ANCHOR_BOTTOMRIGHT" => (AnchorPoint::TopRight, AnchorPoint::BottomRight),
        _ => (AnchorPoint::Right, AnchorPoint::Left),
    }
}
