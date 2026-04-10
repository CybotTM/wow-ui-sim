use super::{TooltipLine, Value, extract_frame_id, get_sim_state};

pub(crate) fn copy_tooltip_impl(
    _lua: &mlua::Lua,
    destination_id: u64,
    args: mlua::Variadic<Value>,
) -> mlua::Result<()> {
    let source_id = parse_copy_tooltip_source(args)?;
    let state_rc = get_sim_state(_lua);
    let mut state = state_rc.borrow_mut();
    let Some(snapshot) = read_tooltip_content_snapshot(&state, source_id) else {
        return Ok(());
    };
    apply_tooltip_content_snapshot(&mut state, destination_id, snapshot);
    Ok(())
}

fn parse_copy_tooltip_source(args: mlua::Variadic<Value>) -> mlua::Result<u64> {
    let source_id = args
        .into_iter()
        .next()
        .and_then(|value| extract_frame_id(&value));
    source_id.ok_or_else(|| mlua::Error::runtime("Usage: GameTooltip:CopyTooltip(sourceTooltip)"))
}

struct TooltipContentSnapshot {
    lines: Vec<TooltipLine>,
    min_width: f32,
    padding: f32,
    spell_id: Option<u32>,
    line_spacing: Option<f32>,
}

fn read_tooltip_content_snapshot(
    state: &crate::lua_api::state::SimState,
    source_id: u64,
) -> Option<TooltipContentSnapshot> {
    let td = state.tooltips.get(&source_id)?;
    Some(TooltipContentSnapshot {
        lines: td.lines.clone(),
        min_width: td.min_width,
        padding: td.padding,
        spell_id: td.spell_id,
        line_spacing: td.line_spacing,
    })
}

fn apply_tooltip_content_snapshot(
    state: &mut crate::lua_api::state::SimState,
    destination_id: u64,
    snapshot: TooltipContentSnapshot,
) {
    let Some(td) = state.tooltips.get_mut(&destination_id) else {
        return;
    };
    td.lines = snapshot.lines;
    td.min_width = snapshot.min_width;
    td.padding = snapshot.padding;
    td.spell_id = snapshot.spell_id;
    td.line_spacing = snapshot.line_spacing;
    td.left_line_ids.clear();
    td.right_line_ids.clear();
    state.widgets.mark_rect_dirty(destination_id);
    state.widgets.mark_visual_dirty(destination_id);
}
