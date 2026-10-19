//! Temporary nonzero Wrath minimap coordinate policy.
//!
//! Missing minimap view->world transform (dimensions/radius/rotation) and
//! normalization. Preserve existing raw-position values only for nonzero view
//! inputs; they are NOT evidence of native normalized coordinates. Local event
//! delivery is modeled separately and must not be suppressed.
//! Retirement condition: model the minimap view->world transform.

pub(crate) fn retain_raw_view_coordinates(x: f32, y: f32) -> (f32, f32) {
    (x, y)
}
