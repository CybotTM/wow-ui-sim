//! Resolve a pointer gesture against the same shaped document used for emission.
use crate::lua_api::SimState;
use crate::lua_api::simple_html::layout::{LinkRegion, layout_document};
use crate::render::WowFontSystem;
use crate::widget::WidgetType;
use iced::Point;

pub(super) fn find_clicked_link(
    state: &SimState,
    id: u64,
    fonts: &mut WowFontSystem,
    gesture: (Point, Point),
) -> Option<LinkRegion> {
    let frame = state.widgets.get(id)?;
    if frame.widget_type != WidgetType::SimpleHTML {
        return None;
    }
    let disabled = frame
        .simple_html
        .as_ref()
        .is_some_and(|data| !data.hyperlinks_enabled);
    if disabled {
        return None;
    }
    let rect = crate::layout::apply_scroll_offsets(&state.widgets, id, frame.layout_rect?);
    let scale = frame.effective_scale.max(f32::EPSILON);
    let down = local_point(gesture.0, rect, scale);
    let up = local_point(gesture.1, rect, scale);
    layout_document(frame, rect.width / scale, fonts)
        .links
        .into_iter()
        .find(|link| link.contains(up.x, up.y) && link.contains(down.x, down.y))
}

fn local_point(point: Point, rect: crate::LayoutRect, scale: f32) -> Point {
    let ui_scale = crate::render::texture::UI_SCALE;
    Point::new(
        (point.x / ui_scale - rect.x) / scale,
        (point.y / ui_scale - rect.y) / scale,
    )
}
