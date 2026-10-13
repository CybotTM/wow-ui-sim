use super::discrete_fixtures::assert_lua;
use wow_ui_sim::c_api::ui_widget_manager::{TextWithStateRecord, WidgetId, WidgetSetEntry};
use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) const ID: f64 = 760_801.0;
pub(super) const SET_ID: f64 = 428.0;

/// Explicit host-input test policy, not a native catalog record.
/// Enum choices read from both active Retail/PTR cached documentation.
pub(super) fn supplied_record() -> TextWithStateRecord {
    TextWithStateRecord {
        shown_state: 1,   // WidgetShownState.Shown
        enabled_state: 3, // WidgetEnabledState.White
        text: "Supplied state text".into(),
        tooltip: "Supplied text tooltip".into(),
        text_size_type: 1, // UIWidgetTextSizeType.Medium16Pt
        font_type: 0,      // UIWidgetFontType.Normal
        bottom_padding: 2.5,
        tooltip_loc: 4, // UIWidgetTooltipLocation.Top
        h_align: 0,     // WidgetTextHorizontalAlignmentType.Left
        widget_size_setting: 180.0,
        texture_kit: String::new(),
        frame_texture_kit: String::new(),
        has_timer: false,
        order_index: 12.5,
        widget_tag: "text-state-supplied".into(),
        in_anim_type: 0, // WidgetAnimationType.None
        out_anim_type: 0,
        widget_scale: 0,      // UIWidgetScale.OneHundred
        layout_direction: 1,  // UIWidgetLayoutDirection.Vertical
        model_scene_layer: 0, // UIWidgetModelSceneLayer.None
        scripted_animation_effect_id: 0.0,
    }
}

pub(super) fn entry() -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id: ID,
        widget_type: 8,
        unit_token: None,
    }
}

pub(super) fn seed_record(env: &WowLuaEnv, id: f64, record: TextWithStateRecord) {
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .text_with_state
        .insert(WidgetId::from_number(id), record);
}

pub(super) fn assert_initial_snapshot(env: &WowLuaEnv) {
    // Result boundary precedes cache loading, consumer construction, and security helpers.
    assert_lua(
        env,
        "TextWithState first supplied getter result (unexecuted RED candidate)",
        r#"
        local get = C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo
        assert(type(get) == 'function', 'missing TextWithState getter')
        local first = get(760801)
        assert(type(first) == 'table', 'missing supplied TextWithState record')
        expectedTextState = {
            shownState = 1, enabledState = 3, text = 'Supplied state text',
            tooltip = 'Supplied text tooltip', textSizeType = 1, fontType = 0,
            bottomPadding = 2.5, tooltipLoc = 4, hAlign = 0, widgetSizeSetting = 180,
            textureKit = '', frameTextureKit = '', hasTimer = false,
            orderIndex = 12.5, widgetTag = 'text-state-supplied', inAnimType = 0,
            outAnimType = 0, widgetScale = 0, layoutDirection = 1,
            modelSceneLayer = 0, scriptedAnimationEffectID = 0,
        }
        function AssertTextStateSnapshot(actual, expected)
            assert(type(actual) == 'table', 'complete supplied table')
            local count = 0
            for key, value in pairs(expected) do
                assert(type(actual[key]) == type(value), 'field type: ' .. key)
                assert(actual[key] == value, 'field value: ' .. key)
            end
            for key in pairs(actual) do
                count = count + 1
                assert(expected[key] ~= nil, 'unexpected field: ' .. tostring(key))
            end
            assert(count == 21, 'exact complete21 snapshot')
        end
        AssertTextStateSnapshot(first, expectedTextState)
        savedTextState = first
    "#,
    );
}
