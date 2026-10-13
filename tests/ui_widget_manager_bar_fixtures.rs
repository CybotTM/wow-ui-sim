//! Complete synthetic bar inputs for cached retail consumers; no native catalog inference.
use wow_ui_sim::c_api::ui_widget_manager::{CaptureBarRecord, StatusBarRecord};

// Values anchored to cached retail UIWidgetManager{,Shared}Documentation.lua.
// These are explicit consumer-valid inputs, not defaults or catalog-derived widgets.
pub fn supplied_status() -> StatusBarRecord {
    StatusBarRecord {
        shown_state: 1, // WidgetShownState.Shown
        bar_min: 10.0,
        bar_max: 110.0,
        bar_value: 40.25,
        text: "Supplied status label".into(),
        tooltip: "Supplied status tooltip".into(),
        bar_value_text_type: 2, // StatusBarValueTextType.Value (ceil -> "41")
        override_bar_text: "Supplied override".into(),
        override_bar_text_shown_type: 0, // Never
        color_tint: 2,                   // White
        partition_values: vec![],        // Ordered number array, no invented partition records.
        tooltip_loc: 3,                  // TopLeft
        fill_motion_type: 0,             // Instant; no smoothing/time approximation.
        bar_text_enabled_state: 3,       // WidgetEnabledState.White, not a boolean Enabled enum.
        bar_text_font_type: 0,           // Normal
        bar_text_size_type: 4,           // Standard14Pt
        text_enabled_state: 3,           // White
        text_font_type: 0,               // Normal
        text_size_type: 4,               // Standard14Pt
        glow_anim_type: 0,               // None
        show_glow_state: 0,              // HideGlow
        fill_min_opacity: 0,             // WidgetOpacityType.OneHundred, not alpha 0.
        fill_max_opacity: 0,
        widget_size_setting: 215.0,
        texture_kit: "white".into(),
        frame_texture_kit: "widgetstatusbar".into(),
        has_timer: true,
        order_index: 10.0,
        widget_tag: "bars-status-consumer".into(),
        in_anim_type: 0,      // None
        out_anim_type: 0,     // None; removal is synchronous.
        widget_scale: 0,      // OneHundred
        layout_direction: 1,  // UIWidgetLayoutDirection.Vertical
        model_scene_layer: 0, // None
        scripted_animation_effect_id: 0.0,
    }
}

pub fn supplied_capture() -> CaptureBarRecord {
    CaptureBarRecord {
        shown_state: 1, // Shown
        bar_value: 50.0,
        bar_min_value: 0.0,
        bar_max_value: 100.0,
        neutral_zone_size: 20.0,
        neutral_zone_center: 50.0,
        tooltip: "Supplied capture tooltip".into(),
        glow_anim_type: 0,                  // None
        fill_direction_type: 1,             // LeftToRight; RightToLeft is 0, NOT 1.
        tooltip_loc: 3,                     // TopLeft
        widget_size_setting: 196.0,         // Complete DTO; Capture Setup does not use this width.
        texture_kit: "bastionarmor".into(), // Explicit cached Setup branch, no factions query.
        frame_texture_kit: "worldstate-capturebar".into(),
        has_timer: false,
        order_index: 20.0,
        widget_tag: "bars-capture-consumer".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 0,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 0.0,
    }
}
