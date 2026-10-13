//! Unchanged cached type8 template, real constructor, typed feeds and frame-event dispatch.
use super::discrete_fixtures::assert_lua;
use super::feed::supply_empty_set;
use super::fixtures::{ID, SET_ID, assert_initial_snapshot, entry, seed_record, supplied_record};
use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::c_api::ui_widget_manager::{TextWithStateRecord, WidgetVisualization};
use wow_ui_sim::lua_api::WowLuaEnv;

fn supply(env: &WowLuaEnv, record: TextWithStateRecord) {
    env.update_ui_widget(SET_ID, entry(), WidgetVisualization::TextWithState(record))
        .expect("typed text-state feed before actual cached event notification");
}

fn flush_counts(env: &WowLuaEnv, count: usize) {
    env.fire_on_update(0.0)
        .expect("actual cached layout OnUpdate");
    assert_lua(
        env,
        "TextWithState actual pool/layout counts",
        &format!(
            r#"
        assert(not TextStateContainer.dirtyLayout)
        assert(TextStateContainer:GetNumWidgetsShowing() == {count})
        assert(TextStateContainer.widgetPools:GetNumActive() == {count})
        assert(TextStateContainer.numTimers == 0 and TextStateContainer.ticker == nil)
    "#
        ),
    );
}

fn register_consumer(env: &WowLuaEnv) {
    assert_lua(
        env,
        "TextWithState unchanged cached manager/template",
        r#"
        assert(Enum.UIWidgetVisualizationType.TextWithState == 8)
        assert(Enum.WidgetShownState.Shown == 1 and Enum.WidgetShownState.Hidden == 0)
        assert(Enum.WidgetEnabledState.White == 3 and Enum.WidgetEnabledState.Green == 4)
        assert(Enum.UIWidgetFontType.Normal == 0 and Enum.UIWidgetFontType.Shadow == 1)
        assert(Enum.UIWidgetTextSizeType.Medium16Pt == 1 and Enum.UIWidgetTextSizeType.Standard14Pt == 4)
        assert(Enum.WidgetTextHorizontalAlignmentType.Left == 0 and Enum.WidgetTextHorizontalAlignmentType.Right == 2)
        assert(Enum.UIWidgetTooltipLocation.Top == 4 and Enum.WidgetAnimationType.None == 0)
        assert(Enum.UIWidgetScale.OneHundred == 0 and Enum.UIWidgetLayoutDirection.Vertical == 1)
        assert(Enum.UIWidgetModelSceneLayer.None == 0 and Enum.UIWidgetSetLayoutDirection.Vertical == 0)
        assert(UIWidgetManager:GetWidgetTypeInfo(8), 'actual cached type8 registered')
        function AssertTextStateGeometry(frame, text, tooltip, width, padding, justify, enabled, font, color)
            assert(frame:IsShown())
            assert(frame.Text:GetText() == text and frame.tooltip == tooltip)
            assert(frame:GetWidth() == width and frame.Text:GetWidth() == width)
            local textHeight = frame.Text:GetStringHeight()
            assert(textHeight > 1, 'actual font measurement required')
            local expectedHeight = textHeight + Clamp(padding, 0, textHeight - 1)
            assert(math.abs(frame:GetHeight() - expectedHeight) < .0001)
            assert(frame.Text:GetJustifyH() == justify)
            assert(frame.Text.enabledState == enabled)
            local path, size, flags = frame.Text:GetFont()
            local expectedPath, expectedSize, expectedFlags = font:GetFont()
            assert(path == expectedPath and size == expectedSize and flags == expectedFlags)
            local r, g, b = frame.Text:GetTextColor()
            local er, eg, eb = color:GetRGB()
            assert(math.abs(r-er) < .0001 and math.abs(g-eg) < .0001 and math.abs(b-eb) < .0001)
            assert(frame.tooltipLoc == Enum.UIWidgetTooltipLocation.Top)
            assert(frame:GetScale() == 1 and frame.layoutDirection == Enum.UIWidgetLayoutDirection.Vertical)
        end
        TextStateContainer = CreateFrame('Frame', nil, UIParent, 'UIWidgetContainerTemplate')
        TextStateContainer:SetPoint('CENTER', UIParent, 'CENTER')
        TextStateContainer:RegisterForWidgetSet(428)
        assert(TextStateContainer:IsRegisteredForWidgetSet(428))
        assert(UIWidgetManager.registeredWidgetContainers[TextStateContainer])
        assert(TextStateContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(TextStateContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        assert(next(TextStateContainer.widgetFrames) == nil, 'initial registered empty set')
    "#,
    );
    flush_counts(env, 0);
    supply(env, supplied_record());
    assert_lua(
        env,
        "TextWithState activates through actual UPDATE_UI_WIDGET",
        r#"
        TextStateFrame = assert(TextStateContainer.widgetFrames[760801], 'real type8 template acquired by event')
        AssertTextStateGeometry(TextStateFrame, 'Supplied state text', 'Supplied text tooltip',
            180, 2.5, 'LEFT', Enum.WidgetEnabledState.White, SystemFont_Large, HIGHLIGHT_FONT_COLOR)
        assert(TextStateFrame.orderIndex == 12.5)
    "#,
    );
    flush_counts(env, 1);
}

fn update_hide_restore(env: &WowLuaEnv) {
    let mut record = supplied_record();
    record.text = "Updated state text".into();
    record.tooltip = "Updated text tooltip".into();
    record.widget_size_setting = 206.25;
    record.bottom_padding = -3.75;
    record.enabled_state = 4; // Green
    record.font_type = 1; // Shadow
    record.text_size_type = 4; // Standard14Pt
    record.h_align = 2; // Right
    record.order_index = 9.25;
    supply(env, record.clone());
    assert_lua(
        env,
        "TextWithState actual incremental update",
        r#"
        assert(TextStateContainer.widgetFrames[760801] == TextStateFrame)
        AssertTextStateGeometry(TextStateFrame, 'Updated state text', 'Updated text tooltip',
            206.25, -3.75, 'RIGHT', Enum.WidgetEnabledState.Green, SystemFont_Shadow_Med3, GREEN_FONT_COLOR)
        assert(TextStateFrame.orderIndex == 9.25)
        assert(savedTextState.text == 'Supplied state text', 'retained pre-consumer snapshot')
    "#,
    );
    flush_counts(env, 1);
    record.shown_state = 0; // Hidden
    supply(env, record.clone());
    assert_lua(
        env,
        "TextWithState Hidden releases actual template",
        r#"
        assert(TextStateContainer.widgetFrames[760801] == nil)
        assert(not TextStateFrame:IsShown())
        assert(C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo(760801).shownState == Enum.WidgetShownState.Hidden)
    "#,
    );
    flush_counts(env, 0);
    record.shown_state = 1; // Shown
    record.text = "Restored state text".into();
    record.bottom_padding = 2.5;
    supply(env, record);
    assert_lua(
        env,
        "TextWithState restoration through real pool",
        r#"
        TextStateRestored = assert(TextStateContainer.widgetFrames[760801])
        AssertTextStateGeometry(TextStateRestored, 'Restored state text', 'Updated text tooltip',
            206.25, 2.5, 'RIGHT', Enum.WidgetEnabledState.Green, SystemFont_Shadow_Med3, GREEN_FONT_COLOR)
    "#,
    );
    flush_counts(env, 1);
}

#[test]
fn text_state_cached_type8_activation_update_hidden_restore_membership_removal() {
    let cache = wow_ui_sim::paths::default_blizzard_ui_addons_path()
        .expect("active Retail/PTR cache path; no vendor override");
    // Reuse core isolated Game environment; no cache is loaded at this point.
    let env = new_blizzard_addon_env(&cache);
    seed_record(&env, ID, supplied_record());
    assert_initial_snapshot(&env);
    // Everything below is unreached until main proves the supplied getter result.
    supply_empty_set(&env);
    wow_ui_sim::xml::register_intrinsic_templates();
    let loaded = load_blizzard_addon_closure_into_env(
        &env,
        &cache,
        &[
            "Blizzard_SharedXMLGame",
            "Blizzard_Fonts_Shared",
            "Blizzard_FrameEffects",
            "Blizzard_UIWidgets",
        ],
        &[],
    );
    assert!(loaded.iter().any(|name| name == "Blizzard_UIWidgets"));
    register_consumer(&env);
    update_hide_restore(&env);
    assert!(
        env.remove_ui_widget(SET_ID, ID)
            .expect("host removal sends actual UPDATE_ALL_UI_WIDGETS")
    );
    assert_lua(
        &env,
        "TextWithState membership removal hides frame but retains record",
        r#"
        assert(next(TextStateContainer.widgetFrames) == nil)
        assert(not TextStateRestored:IsShown())
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(428) == 0)
        assert(C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo(760801).text == 'Restored state text')
        assert(savedTextState.text == 'Supplied state text')
    "#,
    );
    flush_counts(&env, 0);
}
