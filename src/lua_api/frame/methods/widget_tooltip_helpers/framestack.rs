use super::shared::fire_tooltip_script_with_args;
use super::{TooltipLine, Value, frame_ref, get_sim_state};

pub(crate) fn set_frame_stack_impl(
    lua: &mlua::Lua,
    tooltip_id: u64,
    args: mlua::Variadic<Value>,
) -> mlua::Result<Value> {
    let (_, _, highlight_delta) = parse_frame_stack_args(args);
    let state_rc = get_sim_state(lua);
    let highlight_id = {
        let mut state = state_rc.borrow_mut();
        let stack = collect_frame_stack(&state);
        let Some(highlight_id) =
            select_frame_stack_highlight(&mut state, tooltip_id, &stack, highlight_delta)
        else {
            clear_tooltip_for_frame_stack(&mut state, tooltip_id);
            return Ok(Value::Nil);
        };
        write_frame_stack_lines(&mut state, tooltip_id, &stack, highlight_id);
        highlight_id
    };

    fire_tooltip_framestack_script(lua, tooltip_id, highlight_id)?;
    frame_ref(lua, highlight_id)
}

fn parse_frame_stack_args(args: mlua::Variadic<Value>) -> (bool, bool, i32) {
    let mut args = args.into_iter();
    let show_hidden = matches!(args.next(), Some(Value::Boolean(true)));
    let show_regions = matches!(args.next(), Some(Value::Boolean(true)));
    let highlight_delta = match args.next() {
        Some(Value::Integer(value)) => value as i32,
        Some(Value::Number(value)) => value as i32,
        _ => 0,
    };
    (show_hidden, show_regions, highlight_delta)
}

fn collect_frame_stack(state: &crate::lua_api::state::SimState) -> Vec<u64> {
    let mut stack = Vec::new();
    let mut current = state
        .hovered_frame
        .or_else(|| state.widgets.get_id_by_name("UIParent"));
    while let Some(id) = current {
        stack.push(id);
        current = state.widgets.get(id).and_then(|frame| frame.parent_id);
    }
    stack
}

fn select_frame_stack_highlight(
    state: &mut crate::lua_api::state::SimState,
    tooltip_id: u64,
    stack: &[u64],
    highlight_delta: i32,
) -> Option<u64> {
    let td = state.tooltips.get_mut(&tooltip_id)?;
    if stack.is_empty() {
        td.frame_stack_index = 0;
        return None;
    }

    let stack_len = stack.len() as i32;
    let current_index = td.frame_stack_index.min(stack.len() - 1) as i32;
    let next_index = (current_index + highlight_delta).rem_euclid(stack_len) as usize;
    td.frame_stack_index = next_index;
    stack.get(next_index).copied()
}

fn clear_tooltip_for_frame_stack(state: &mut crate::lua_api::state::SimState, tooltip_id: u64) {
    let Some(td) = state.tooltips.get_mut(&tooltip_id) else {
        return;
    };
    td.lines.clear();
    td.left_line_ids.clear();
    td.right_line_ids.clear();
    td.spell_id = None;
    state.widgets.mark_rect_dirty(tooltip_id);
    state.widgets.mark_visual_dirty(tooltip_id);
}

fn write_frame_stack_lines(
    state: &mut crate::lua_api::state::SimState,
    tooltip_id: u64,
    stack: &[u64],
    highlight_id: u64,
) {
    let line_texts = collect_frame_stack_line_texts(state, stack);
    let Some(td) = state.tooltips.get_mut(&tooltip_id) else {
        return;
    };
    td.lines = line_texts
        .into_iter()
        .map(|(frame_id, text)| TooltipLine {
            left_text: text,
            left_color: if frame_id == highlight_id {
                (1.0, 1.0, 1.0)
            } else {
                (0.8, 0.8, 0.8)
            },
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            wrap: false,
            texture: None,
        })
        .collect();
    td.left_line_ids.clear();
    td.right_line_ids.clear();
    td.spell_id = None;
    state.widgets.mark_rect_dirty(tooltip_id);
    state.widgets.mark_visual_dirty(tooltip_id);
}

fn collect_frame_stack_line_texts(
    state: &crate::lua_api::state::SimState,
    stack: &[u64],
) -> Vec<(u64, String)> {
    stack
        .iter()
        .filter_map(|&frame_id| {
            describe_frame_stack_entry(state, frame_id).map(|text| (frame_id, text))
        })
        .collect()
}

fn describe_frame_stack_entry(
    state: &crate::lua_api::state::SimState,
    frame_id: u64,
) -> Option<String> {
    state.widgets.get(frame_id)?;
    Some(frame_stack_debug_name(state, frame_id))
}

fn frame_stack_debug_name(state: &crate::lua_api::state::SimState, frame_id: u64) -> String {
    let Some(frame) = state.widgets.get(frame_id) else {
        return "[Unknown]".to_string();
    };
    if let Some(name) = &frame.name {
        return name.clone();
    }
    if let Some(parent_id) = frame.parent_id
        && let Some(parent) = state.widgets.get(parent_id)
    {
        for (key, &child_id) in &parent.children_keys {
            if child_id == frame_id {
                let parent_name = parent.name.as_deref().unwrap_or("?");
                return format!("{}.{}", parent_name, key);
            }
        }
    }
    format!("[{}]", frame.widget_type.as_str())
}

fn fire_tooltip_framestack_script(
    lua: &mlua::Lua,
    tooltip_id: u64,
    highlight_id: u64,
) -> mlua::Result<()> {
    let highlight = frame_ref(lua, highlight_id)?;
    fire_tooltip_script_with_optional_frame_arg(
        lua,
        tooltip_id,
        "OnTooltipSetFramestack",
        &highlight,
    )?;
    fire_tooltip_script_with_optional_frame_arg(
        lua,
        tooltip_id,
        "OnTooltipSetFrameStack",
        &highlight,
    )?;
    Ok(())
}

fn fire_tooltip_script_with_optional_frame_arg(
    lua: &mlua::Lua,
    frame_id: u64,
    handler: &str,
    extra_arg: &Value,
) -> mlua::Result<()> {
    fire_tooltip_script_with_args(lua, frame_id, handler, vec![extra_arg.clone()])
}
