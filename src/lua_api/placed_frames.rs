//! Wrath character-local placed-frame snapshots, in simulator layout units.
use super::WowLuaEnv;
use crate::saved_variables::SavedVariablesManager;
use crate::widget::{AnchorPoint, Frame};
use crate::{LayoutRect, Result};

mod format;

const POINTS: [AnchorPoint; 9] = [
    AnchorPoint::TopLeft,
    AnchorPoint::Top,
    AnchorPoint::TopRight,
    AnchorPoint::Left,
    AnchorPoint::Center,
    AnchorPoint::Right,
    AnchorPoint::BottomLeft,
    AnchorPoint::Bottom,
    AnchorPoint::BottomRight,
];

struct Placement {
    name: String,
    level: i32,
    point: AnchorPoint,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

pub(super) fn save(env: &WowLuaEnv, manager: &SavedVariablesManager) -> Result<()> {
    let Some(records) = capture_placements(env) else {
        return Ok(());
    };
    let directory = manager.character_storage_dir();
    std::fs::create_dir_all(&directory).map_err(|error| {
        crate::Error::Other(format!(
            "create placed-layout directory {}: {error}",
            directory.display()
        ))
    })?;
    let path = directory.join("layout-local.txt");
    std::fs::write(&path, format::write_records(&records)).map_err(|error| {
        crate::Error::Other(format!("write placed layout {}: {error}", path.display()))
    })
}

fn capture_placements(env: &WowLuaEnv) -> Option<Vec<Placement>> {
    let mut state = env.state().borrow_mut();
    state.ensure_layout_rects();
    let root = state.widgets.get_by_name("UIParent")?.layout_rect?;
    let mut records: Vec<_> = state
        .widgets
        .iter_ids()
        .filter_map(|id| state.widgets.get(id))
        .filter_map(|frame| capture_frame(frame, root))
        .collect();
    records.sort_by(|a, b| a.name.cmp(&b.name));
    Some(records)
}

fn capture_frame(frame: &Frame, root: LayoutRect) -> Option<Placement> {
    let name = frame.name.as_ref().filter(|name| !name.is_empty())?;
    let can_move_or_resize = frame.movable || frame.resizable;
    let queryable = crate::widget::has_queryable_rect(frame, frame.id);
    if !frame.user_placed || frame.dont_save_position || !can_move_or_resize || !queryable {
        return None;
    }
    let rect = frame.layout_rect?;
    let (point, x, y) = nearest_corresponding_point(rect, root);
    let scale = frame.effective_scale;
    Some(Placement {
        name: name.clone(),
        level: frame.frame_level,
        point,
        x: round_frame_units(x, scale),
        y: round_frame_units(y, scale),
        width: round_frame_units(rect.width, scale),
        height: round_frame_units(rect.height, scale),
    })
}

fn round_frame_units(value: f32, effective_scale: f32) -> i32 {
    (value / effective_scale).round_ties_even() as i32
}

fn nearest_corresponding_point(rect: LayoutRect, root: LayoutRect) -> (AnchorPoint, f32, f32) {
    let mut nearest = (POINTS[0], 0.0, 0.0);
    let mut minimum = f32::INFINITY;
    for point in POINTS {
        let (x, y) = corresponding_displacement(rect, root, point);
        let distance = x * x + y * y;
        if distance < minimum {
            minimum = distance;
            nearest = (point, x, y);
        }
    }
    nearest
}

fn corresponding_displacement(
    rect: LayoutRect,
    root: LayoutRect,
    point: AnchorPoint,
) -> (f32, f32) {
    let horizontal = point.horizontal_factor();
    let vertical = match point {
        AnchorPoint::TopLeft | AnchorPoint::Top | AnchorPoint::TopRight => 0.0,
        AnchorPoint::Left | AnchorPoint::Center | AnchorPoint::Right => 0.5,
        AnchorPoint::BottomLeft | AnchorPoint::Bottom | AnchorPoint::BottomRight => 1.0,
    };
    let x = rect.x - root.x + (rect.width - root.width) * horizontal;
    let y = root.y - rect.y + (root.height - rect.height) * vertical;
    (x, y)
}

pub(super) fn load(env: &WowLuaEnv, manager: &SavedVariablesManager) -> Result<()> {
    let path = manager.character_storage_dir().join("layout-local.txt");
    let text = std::fs::read_to_string(&path).map_err(|error| {
        crate::Error::Other(format!("read placed layout {}: {error}", path.display()))
    })?;
    for record in format::read_records(&text) {
        apply_placement(env, record)?;
    }
    env.state().borrow_mut().ensure_layout_rects();
    Ok(())
}

fn apply_placement(env: &WowLuaEnv, record: Placement) -> Result<()> {
    let name = crate::loader::helpers::escape_lua_string(&record.name);
    let point = record.point.as_str();
    env.exec(&format!(
        r#"
        local f = _G["{name}"]
        if f then
            if f:IsMovable() then
                f:ClearAllPoints()
                f:SetPoint('{point}', UIParent, '{point}', {x}, {y})
                f:SetUserPlaced(true)
            end
            if f:IsResizable() then
                f:SetSize({width}, {height})
                f:SetUserPlaced(true)
            end
            if {level} >= 0 then
                f:SetFrameLevel(math.min({level}, f:GetFrameLevel() + 128))
            end
            f:GetRect()
        end
    "#,
        x = record.x,
        y = record.y,
        width = record.width,
        height = record.height,
        level = record.level
    ))
}
