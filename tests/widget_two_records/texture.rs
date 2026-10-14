//! Explicit synthetic records, not native catalog defaults.
use wow_ui_sim::c_api::ui_widget_manager::TextureAndTextRecord;

pub const GETTER: &str = "GetTextureAndTextVisualizationInfo";
pub const EXPECTED: &str = r#"{
    shownState = 1, text = 'TextureText initial 731', tooltip = 'TextureText tooltip initial 731',
    tooltipLoc = 3, textSizeType = 2, textFormatType = 0, updateAnimType = 0,
    widgetSizeSetting = 137.25, textureKit = '', frameTextureKit = '', hasTimer = false,
    orderIndex = 37.5, widgetTag = 'texture-text-contract-731', inAnimType = 0,
    outAnimType = 0, widgetScale = 1, layoutDirection = 2, modelSceneLayer = 0,
    scriptedAnimationEffectID = 731,
}"#;

pub fn supplied() -> TextureAndTextRecord {
    TextureAndTextRecord {
        shown_state: 1,
        text: "TextureText initial 731".into(),
        tooltip: "TextureText tooltip initial 731".into(),
        tooltip_loc: 3,
        text_size_type: 2,
        text_format_type: 0,
        update_anim_type: 0,
        widget_size_setting: 137.25,
        texture_kit: "".into(),
        frame_texture_kit: "".into(),
        has_timer: false,
        order_index: 37.5,
        widget_tag: "texture-text-contract-731".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 1,
        layout_direction: 2,
        model_scene_layer: 0,
        scripted_animation_effect_id: 731.0,
    }
}

// Changes every field for projection-only tests; animation/assets are not exercised here.
pub fn replacement() -> TextureAndTextRecord {
    TextureAndTextRecord {
        shown_state: 0,
        text: "Replacement texture text".into(),
        tooltip: "Replacement texture tooltip".into(),
        tooltip_loc: 5,
        text_size_type: 1,
        text_format_type: 1,
        update_anim_type: 1,
        widget_size_setting: 211.5,
        texture_kit: "replacement-foreground".into(),
        frame_texture_kit: "replacement-background".into(),
        has_timer: true,
        order_index: -12.25,
        widget_tag: "replacement-texture-tag".into(),
        in_anim_type: 1,
        out_anim_type: 2,
        widget_scale: 2,
        layout_direction: 1,
        model_scene_layer: 1,
        scripted_animation_effect_id: 947.5,
    }
}

pub const REPLACEMENT_EXPECTED: &str = r#"{
    shownState = 0, text = 'Replacement texture text', tooltip = 'Replacement texture tooltip',
    tooltipLoc = 5, textSizeType = 1, textFormatType = 1, updateAnimType = 1,
    widgetSizeSetting = 211.5, textureKit = 'replacement-foreground',
    frameTextureKit = 'replacement-background', hasTimer = true, orderIndex = -12.25,
    widgetTag = 'replacement-texture-tag', inAnimType = 1, outAnimType = 2,
    widgetScale = 2, layoutDirection = 1, modelSceneLayer = 1, scriptedAnimationEffectID = 947.5,
}"#;
