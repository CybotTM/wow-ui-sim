use super::{FrameRef, Value, frame_ref, get_sim_state};

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
        sync_line_fontstring(
            state,
            entry.left_id,
            Some(entry.left_text.as_str()),
            entry.left_color,
        );
        sync_line_fontstring(
            state,
            entry.right_id,
            entry.right_text.as_deref(),
            entry.right_color,
        );
    }
}

fn sync_line_fontstring(
    state: &mut crate::lua_api::state::SimState,
    fontstring_id: Option<u64>,
    text: Option<&str>,
    color: (f32, f32, f32),
) {
    let Some(id) = fontstring_id else {
        return;
    };
    let Some(fs) = state.widgets.get_mut_visual(id) else {
        return;
    };
    fs.text = text.map(str::to_string);
    let (r, g, b) = color;
    fs.text_color = crate::widget::Color::new(r, g, b, 1.0);
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
