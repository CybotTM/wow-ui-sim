use super::shared::val_to_f32;
use super::{TooltipLine, Value, get_sim_state};

pub(crate) fn add_double_line_impl(
    lua: &mlua::Lua,
    id: u64,
    args: mlua::MultiValue,
) -> mlua::Result<()> {
    let Some((left, right, left_color, right_color)) = parse_double_line_args(args) else {
        return Ok(());
    };
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(td) = state.tooltips.get_mut(&id) {
        td.lines.push(TooltipLine {
            left_text: left,
            left_color,
            right_text: Some(right),
            right_color,
            wrap: false,
            texture: None,
        });
    }
    Ok(())
}

fn parse_double_line_args(
    args: mlua::MultiValue,
) -> Option<(String, String, (f32, f32, f32), (f32, f32, f32))> {
    let mut it = args.into_iter();
    let left = tooltip_arg_text(it.next())?;
    let right = tooltip_arg_text(it.next()).unwrap_or_default();
    let left_color = parse_rgb_triplet(&mut it);
    let right_color = parse_rgb_triplet(&mut it);
    Some((left, right, left_color, right_color))
}

fn tooltip_arg_text(value: Option<Value>) -> Option<String> {
    match value {
        Some(Value::String(s)) => Some(s.to_string_lossy().to_string()),
        Some(Value::Number(n)) => Some(n.to_string()),
        Some(Value::Integer(n)) => Some(n.to_string()),
        _ => None,
    }
}

fn parse_rgb_triplet(it: &mut impl Iterator<Item = Value>) -> (f32, f32, f32) {
    (
        val_to_f32(it.next(), 1.0),
        val_to_f32(it.next(), 1.0),
        val_to_f32(it.next(), 1.0),
    )
}
