use crate::lua_api::frame::handle::{FrameRef, extract_frame_id, frame_ref, get_sim_state};
use crate::lua_api::tooltip::TooltipLine;
use crate::widget::{Anchor, AnchorPoint};
use mlua::Value;

pub(crate) fn add_tooltip_info_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsOwned", |lua, this, frame: Value| {
        let check_id = extract_frame_id(&frame);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let owned = state
            .tooltips
            .get(&this.0)
            .is_some_and(|td| td.owner_id.is_some() && td.owner_id == check_id);
        Ok(owned)
    });

    methods.add_method("GetOwner", |lua, this, ()| {
        let owner_id = {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            state.tooltips.get(&this.0).and_then(|td| td.owner_id)
        };
        match owner_id {
            Some(oid) => frame_ref(lua, oid),
            None => Ok(Value::Nil),
        }
    });

    methods.add_method("GetAnchorType", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let anchor = state
            .tooltips
            .get(&this.0)
            .map(|td| td.anchor_type.clone())
            .unwrap_or_else(|| "ANCHOR_NONE".to_string());
        Ok(anchor)
    });
}

pub(crate) fn add_tooltip_state_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("FadeOut", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        state.set_frame_visible(this.0, false);
        if let Some(td) = state.tooltips.get_mut(&this.0) {
            td.owner_id = None;
        }
        Ok(())
    });
}

pub(crate) fn add_get_line_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetLeftLine", |lua, this, index: i32| {
        get_tooltip_line_fontstring(lua, this.0, index, Side::Left)
    });
    methods.add_method("GetRightLine", |lua, this, index: i32| {
        get_tooltip_line_fontstring(lua, this.0, index, Side::Right)
    });
}

enum Side {
    Left,
    Right,
}

fn get_tooltip_line_fontstring(
    lua: &mlua::Lua,
    tooltip_id: u64,
    index: i32,
    side: Side,
) -> mlua::Result<Value> {
    if index < 1 {
        return Ok(Value::Nil);
    }
    let idx = (index - 1) as usize;

    ensure_tooltip_fontstrings(lua, tooltip_id)?;

    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let td = match state.tooltips.get(&tooltip_id) {
        Some(td) => td,
        None => return Ok(Value::Nil),
    };

    let ids = match side {
        Side::Left => &td.left_line_ids,
        Side::Right => &td.right_line_ids,
    };

    match ids.get(idx) {
        Some(&fs_id) => frame_ref(lua, fs_id),
        None => Ok(Value::Nil),
    }
}

/// Create FontString children for any tooltip lines that don't have them yet,
/// and sync text/color from `td.lines` to the FontString widgets.
fn ensure_tooltip_fontstrings(lua: &mlua::Lua, tooltip_id: u64) -> mlua::Result<()> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();

    let (new_left, new_right) = create_missing_fontstrings(&mut state, tooltip_id);

    let td = state.tooltips.get_mut(&tooltip_id).unwrap();
    td.left_line_ids.extend(&new_left);
    td.right_line_ids.extend(&new_right);
    let sync_data = collect_line_sync_data(td);
    sync_fontstring_text(&mut state, &sync_data);

    drop(state);
    register_fontstring_globals(lua, &state_rc, &new_left, &new_right)
}

/// Create left and right FontString children for lines that don't have them yet.
fn create_missing_fontstrings(
    state: &mut crate::lua_api::state::SimState,
    tooltip_id: u64,
) -> (Vec<u64>, Vec<u64>) {
    let tooltip_name = state
        .widgets
        .get(tooltip_id)
        .and_then(|f| f.name.clone())
        .unwrap_or_default();

    let td = match state.tooltips.get(&tooltip_id) {
        Some(td) => td,
        None => return (Vec::new(), Vec::new()),
    };
    let line_count = td.lines.len();
    let existing_left = td.left_line_ids.len();
    let existing_right = td.right_line_ids.len();

    let new_left = create_line_fontstrings(
        state,
        tooltip_id,
        &tooltip_name,
        "TextLeft",
        existing_left,
        line_count,
    );
    let new_right = create_line_fontstrings(
        state,
        tooltip_id,
        &tooltip_name,
        "TextRight",
        existing_right,
        line_count,
    );
    (new_left, new_right)
}

/// Create FontString children for line indices `existing..target`.
fn create_line_fontstrings(
    state: &mut crate::lua_api::state::SimState,
    parent_id: u64,
    tooltip_name: &str,
    suffix: &str,
    existing: usize,
    target: usize,
) -> Vec<u64> {
    (existing..target)
        .map(|i| {
            let name = format!("{}{}{}", tooltip_name, suffix, i + 1);
            let fs = crate::widget::Frame::new(
                crate::widget::WidgetType::FontString,
                Some(name),
                Some(parent_id),
            );
            let fs_id = fs.id;
            state.widgets.register(fs);
            state.widgets.add_child(parent_id, fs_id);
            fs_id
        })
        .collect()
}

struct LineSyncEntry {
    left_id: Option<u64>,
    right_id: Option<u64>,
    left_text: String,
    left_color: (f32, f32, f32),
    right_text: Option<String>,
    right_color: (f32, f32, f32),
}

fn collect_line_sync_data(td: &crate::lua_api::tooltip::TooltipData) -> Vec<LineSyncEntry> {
    td.lines
        .iter()
        .enumerate()
        .map(|(i, line)| LineSyncEntry {
            left_id: td.left_line_ids.get(i).copied(),
            right_id: td.right_line_ids.get(i).copied(),
            left_text: line.left_text.clone(),
            left_color: line.left_color,
            right_text: line.right_text.clone(),
            right_color: line.right_color,
        })
        .collect()
}

fn sync_fontstring_text(state: &mut crate::lua_api::state::SimState, entries: &[LineSyncEntry]) {
    for entry in entries {
        if let Some(id) = entry.left_id
            && let Some(fs) = state.widgets.get_mut_visual(id)
        {
            fs.text = Some(entry.left_text.clone());
            let (r, g, b) = entry.left_color;
            fs.text_color = crate::widget::Color::new(r, g, b, 1.0);
        }
        if let Some(id) = entry.right_id
            && let Some(fs) = state.widgets.get_mut_visual(id)
        {
            fs.text = entry.right_text.clone();
            let (r, g, b) = entry.right_color;
            fs.text_color = crate::widget::Color::new(r, g, b, 1.0);
        }
    }
}

fn register_fontstring_globals(
    lua: &mlua::Lua,
    state_rc: &std::rc::Rc<std::cell::RefCell<crate::lua_api::state::SimState>>,
    new_left: &[u64],
    new_right: &[u64],
) -> mlua::Result<()> {
    for &fs_id in new_left.iter().chain(new_right.iter()) {
        let ud = frame_ref(lua, fs_id)?;
        let name = {
            let state = state_rc.borrow();
            state.widgets.get(fs_id).and_then(|f| f.name.clone())
        };
        if let Some(n) = name {
            lua.globals().raw_set(n.as_str(), ud)?;
        }
    }
    Ok(())
}

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
        }
        position_tooltip(&mut state, id, owner_id, &anchor, x_offset, y_offset);
        state.set_frame_visible(id, true);
    }
    fire_tooltip_script(lua, id, "OnTooltipCleared")?;
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
    let anchor = match args_iter.next() {
        Some(Value::String(s)) => {
            let s = s.to_string_lossy().to_string();
            if is_valid_anchor_type(&s) {
                s
            } else {
                crate::lua_api::script_helpers::call_error_handler(
                    lua,
                    &format!(
                        "SetOwner: invalid anchor type '{}', defaulting to ANCHOR_LEFT",
                        s
                    ),
                );
                "ANCHOR_LEFT".to_string()
            }
        }
        _ => "ANCHOR_LEFT".to_string(),
    };
    let x_offset = val_to_f32(args_iter.next(), 0.0);
    let y_offset = val_to_f32(args_iter.next(), 0.0);
    Ok((extract_frame_id(&owner_val), anchor, x_offset, y_offset))
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

pub(crate) fn add_double_line_impl(
    lua: &mlua::Lua,
    id: u64,
    args: mlua::MultiValue,
) -> mlua::Result<()> {
    let mut it = args.into_iter();
    let left = match it.next() {
        Some(Value::String(s)) => s.to_string_lossy().to_string(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Integer(n)) => n.to_string(),
        _ => return Ok(()),
    };
    let right = match it.next() {
        Some(Value::String(s)) => s.to_string_lossy().to_string(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Integer(n)) => n.to_string(),
        _ => String::new(),
    };
    let lr = val_to_f32(it.next(), 1.0);
    let lg = val_to_f32(it.next(), 1.0);
    let lb = val_to_f32(it.next(), 1.0);
    let rr = val_to_f32(it.next(), 1.0);
    let rg = val_to_f32(it.next(), 1.0);
    let rb = val_to_f32(it.next(), 1.0);
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(td) = state.tooltips.get_mut(&id) {
        td.lines.push(TooltipLine {
            left_text: left,
            left_color: (lr, lg, lb),
            right_text: Some(right),
            right_color: (rr, rg, rb),
            wrap: false,
            texture: None,
        });
    }
    Ok(())
}

const DEFAULT_CURSOR_Y_OFFSET: f32 = 20.0;

fn position_tooltip(
    state: &mut crate::lua_api::state::SimState,
    tooltip_id: u64,
    owner_id: Option<u64>,
    anchor_type: &str,
    x_offset: f32,
    y_offset: f32,
) {
    let frame = match state.widgets.get_mut_visual(tooltip_id) {
        Some(f) => f,
        None => return,
    };
    if anchor_type == "ANCHOR_PRESERVE" {
        return;
    }
    frame.anchors.clear();
    let anchor = match anchor_type {
        "ANCHOR_CURSOR" => {
            let (mx, my) = state.mouse_position.unwrap_or((0.0, 0.0));
            Some(build_cursor_anchor(mx, my, x_offset, y_offset))
        }
        "ANCHOR_NONE" => None,
        _ => owner_id.map(|oid| build_owner_anchor(anchor_type, oid, x_offset, y_offset)),
    };
    if let Some(a) = anchor {
        frame.anchors.push(a);
    }
}

fn build_cursor_anchor(mx: f32, my: f32, x_offset: f32, y_offset: f32) -> Anchor {
    let cursor_y = if x_offset == 0.0 && y_offset == 0.0 {
        my + DEFAULT_CURSOR_Y_OFFSET
    } else {
        my + y_offset
    };
    Anchor {
        point: AnchorPoint::TopLeft,
        relative_to: None,
        relative_to_id: None,
        relative_point: AnchorPoint::TopLeft,
        x_offset: mx + x_offset,
        y_offset: cursor_y,
    }
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

// --- Shared helpers ---

/// Fire a script handler on a frame (e.g. OnTooltipCleared).
pub(crate) fn fire_tooltip_script(
    lua: &mlua::Lua,
    frame_id: u64,
    handler: &str,
) -> mlua::Result<()> {
    if let Some(func) = crate::lua_api::script_helpers::get_script(lua, frame_id, handler)
        && let Some(frame_ud) = crate::lua_api::script_helpers::get_frame_ref(lua, frame_id)
        && let Err(e) = func.call::<()>(frame_ud)
    {
        crate::lua_api::script_helpers::call_error_handler(lua, &e.to_string());
    }
    Ok(())
}

/// Extract f32 from a Lua Value, returning default if nil/absent.
pub(crate) fn val_to_f32(val: Option<Value>, default: f32) -> f32 {
    match val {
        Some(Value::Number(n)) => n as f32,
        Some(Value::Integer(n)) => n as f32,
        _ => default,
    }
}
