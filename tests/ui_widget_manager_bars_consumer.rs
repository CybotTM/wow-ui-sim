//! Cached Lua/XML consumers over supplied bar records; getters are the first RED boundary.
//! Retail/PTR fixtures load the active profile cache; source parity is not runtime proof.
//! Host bar feeds commit supplied records before notifying actual cached consumers.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use crate::common;
use common::blizzard_addon_harness::with_blizzard_addon_closure_in_dir;
use wow_ui_sim::c_api::ui_widget_manager::{
    CaptureBarRecord, StatusBarRecord, WidgetSetEntry, WidgetSetRecord, WidgetVisualization,
};
use wow_ui_sim::lua_api::WowLuaEnv;

use crate::ui_widget_manager_bar_fixtures::{supplied_capture, supplied_status};

const SET_ID: f64 = 419.0;
const STATUS_ID: f64 = 760_201.0;
const CAPTURE_ID: f64 = 760_202.0;
const STATUS_BAR: u32 = 2;
const CAPTURE_BAR: u32 = 1;

fn entry(widget_id: f64, widget_type: u32) -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id,
        widget_type,
        unit_token: None,
    }
}

fn bar_set() -> WidgetSetRecord {
    WidgetSetRecord {
        layout_direction: 0, // UIWidgetSetLayoutDirection.Vertical
        vertical_padding: 7.0,
        widgets: vec![entry(STATUS_ID, STATUS_BAR), entry(CAPTURE_ID, CAPTURE_BAR)],
    }
}

fn inject_status(env: &WowLuaEnv, record: StatusBarRecord) {
    env.update_ui_widget(
        SET_ID,
        entry(STATUS_ID, STATUS_BAR),
        WidgetVisualization::StatusBar(record),
    )
    .expect("host status feed commits record before incremental notification");
}

fn inject_capture(env: &WowLuaEnv, record: CaptureBarRecord) {
    env.update_ui_widget(
        SET_ID,
        entry(CAPTURE_ID, CAPTURE_BAR),
        WidgetVisualization::CaptureBar(record),
    )
    .expect("host capture feed commits record before incremental notification");
}

fn assert_lua(env: &WowLuaEnv, phase: &str, code: &str) {
    // Label fixture-local assertions without replacing vendor/global assert.
    let diagnostic_code = format!(
        r#"
        -- Widget dimensions and anchor offsets are f32; values remain f64.
        local geometryEpsilon = 0.0001
        local originalAssert = assert
        local assertionIndex = 0
        local function assert(value, message)
            assertionIndex = assertionIndex + 1
            return originalAssert(value, 'fixture assertion ' .. assertionIndex .. ': ' .. tostring(message))
        end
        {code}
        "#
    );
    env.exec(&diagnostic_code)
        .unwrap_or_else(|error| panic!("cached bar consumer [{phase}]: {error}"));
    let errors = env.state().borrow().lua_errors.clone();
    assert!(
        errors.is_empty(),
        "[{phase}] recorded Lua errors: {errors:?}"
    );
}

fn flush_and_assert_counts(env: &WowLuaEnv, showing: usize, timers: usize) {
    // Never use a pre-OnUpdate numWidgetsShowing as the result of an incremental event.
    env.fire_on_update(0.0)
        .expect("flush real cached container OnUpdate before layout counts");
    assert_lua(
        env,
        "post-OnUpdate layout/pool/timer counts",
        &format!(
            r#"
        local c = BarsContainer
        assert(not c.dirtyLayout, 'layout must be clean after flush')
        assert(c:GetNumWidgetsShowing() == {showing})
        assert(c.widgetPools:GetNumActive() == {showing})
        assert(StatusFrame.Bar.partitionPool:GetNumActive() == 0)
        assert(c.numTimers == {timers})
        assert((c.ticker ~= nil) == ({timers} > 0))
    "#
        ),
    );
}

fn supply_initial_set_and_records(env: &WowLuaEnv) {
    env.replace_ui_widget_set(SET_ID, Some(bar_set()))
        .expect("host feed supplies membership before real registration");
    inject_status(env, supplied_status());
    inject_capture(env, supplied_capture());
}

fn assert_supplied_query_boundary(env: &WowLuaEnv) {
    assert_lua(
        env,
        "query boundary: getter types and complete supplied records",
        r#"
        local function CheckRecord(getter, id, expected)
            assert(type(getter) == 'function', 'missing bar visualization getter')
            local record = getter(id)
            assert(type(record) == 'table', 'getter must return supplied bar record')
            for key, value in pairs(expected) do
                assert(type(record[key]) == type(value), 'wrong field type: ' .. key)
                if type(value) == 'table' then
                    assert(next(record[key]) == nil, 'supplied partition array must be empty')
                else
                    assert(record[key] == value, 'wrong supplied field: ' .. key)
                end
            end
            for key in pairs(record) do
                assert(expected[key] ~= nil, 'unexpected record field: ' .. tostring(key))
            end
        end
        CheckRecord(C_UIWidgetManager.GetStatusBarWidgetVisualizationInfo, 760201, {
            shownState = 1, barMin = 10, barMax = 110, barValue = 40.25,
            text = 'Supplied status label', tooltip = 'Supplied status tooltip',
            barValueTextType = 2, overrideBarText = 'Supplied override',
            overrideBarTextShownType = 0, colorTint = 2, partitionValues = {},
            tooltipLoc = 3, fillMotionType = 0, barTextEnabledState = 3,
            barTextFontType = 0, barTextSizeType = 4, textEnabledState = 3,
            textFontType = 0, textSizeType = 4, glowAnimType = 0, showGlowState = 0,
            fillMinOpacity = 0, fillMaxOpacity = 0, widgetSizeSetting = 215,
            textureKit = 'white', frameTextureKit = 'widgetstatusbar', hasTimer = true,
            orderIndex = 10, widgetTag = 'bars-status-consumer', inAnimType = 0,
            outAnimType = 0, widgetScale = 0, layoutDirection = 1, modelSceneLayer = 0,
            scriptedAnimationEffectID = 0,
        })
        CheckRecord(C_UIWidgetManager.GetCaptureBarWidgetVisualizationInfo, 760202, {
            shownState = 1, barValue = 50, barMinValue = 0, barMaxValue = 100,
            neutralZoneSize = 20, neutralZoneCenter = 50,
            tooltip = 'Supplied capture tooltip', glowAnimType = 0,
            fillDirectionType = 1, tooltipLoc = 3, widgetSizeSetting = 196,
            textureKit = 'bastionarmor', frameTextureKit = 'worldstate-capturebar',
            hasTimer = false, orderIndex = 20, widgetTag = 'bars-capture-consumer',
            inAnimType = 0, outAnimType = 0, widgetScale = 0, layoutDirection = 1,
            modelSceneLayer = 0, scriptedAnimationEffectID = 0,
        })
    "#,
    );
}

fn register_and_assert_actual_templates(env: &WowLuaEnv) {
    assert_lua(
        env,
        "cached registrations and real XML children",
        r#"
        assert(Enum.UIWidgetVisualizationType.StatusBar == 2)
        assert(Enum.UIWidgetVisualizationType.CaptureBar == 1)
        assert(Enum.WidgetShownState.Hidden == 0 and Enum.WidgetShownState.Shown == 1)
        assert(Enum.CaptureBarWidgetFillDirectionType.RightToLeft == 0)
        assert(Enum.CaptureBarWidgetFillDirectionType.LeftToRight == 1)
        assert(Enum.StatusBarValueTextType.Value == 2)
        assert(Enum.StatusBarOverrideBarTextShownType.Never == 0)
        assert(Enum.StatusBarOverrideBarTextShownType.Always == 1)
        assert(Enum.UIWidgetMotionType.Instant == 0)
        assert(Enum.WidgetEnabledState.White == 3)
        assert(Enum.UIWidgetFontType.Normal == 0)
        assert(Enum.UIWidgetTextSizeType.Standard14Pt == 4)
        assert(Enum.WidgetOpacityType.OneHundred == 0)
        assert(Enum.UIWidgetScale.OneHundred == 0)
        assert(Enum.UIWidgetSetLayoutDirection.Vertical == 0)
        assert(Enum.UIWidgetLayoutDirection.Vertical == 1)
        assert(Enum.UIWidgetModelSceneLayer.None == 0)
        assert(Enum.UIWidgetTooltipLocation.TopLeft == 3)
        assert(Enum.WidgetAnimationType.None == 0)
        assert(Enum.WidgetGlowAnimType.None == 0)
        assert(Enum.WidgetShowGlowState.HideGlow == 0)
        assert(Enum.StatusBarColorTintValue.White == 2)
        assert(UIWidgetManager:GetWidgetTypeInfo(2))
        assert(UIWidgetManager:GetWidgetTypeInfo(1))
        -- Prerequisites are queried, NEVER supplied or replaced by this fixture.
        assert(C_Texture.GetAtlasInfo('widgetstatusbar-fill-white'), 'real status fill atlas')
        assert(C_Texture.GetAtlasInfo('worldstate-capturebar-neutralfill-bastionarmor'))
        assert(C_Texture.GetAtlasInfo('worldstate-capturebar-neutralglow-bastionarmor'))
        assert(C_Texture.GetAtlasInfo('worldstate-capturebar-spark-neutral-bastionarmor'))
        BarsContainer = CreateFrame('Frame', nil, UIParent, 'UIWidgetContainerTemplate')
        BarsContainer:SetPoint('CENTER', UIParent, 'CENTER')
        BarsContainer:RegisterForWidgetSet(419)
        assert(BarsContainer:IsRegisteredForWidgetSet(419))
        assert(BarsContainer.widgetSetLayoutDirection == Enum.UIWidgetSetLayoutDirection.Vertical)
        assert(BarsContainer.verticalAnchorYOffset == 7)
        assert(UIWidgetManager.registeredWidgetContainers[BarsContainer])
        assert(BarsContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(BarsContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        StatusFrame = assert(BarsContainer.widgetFrames[760201])
        CaptureFrame = assert(BarsContainer.widgetFrames[760202])
        assert(StatusFrame:IsShown() and CaptureFrame:IsShown())
        assert(StatusFrame.Bar:GetObjectType() == 'StatusBar')
        assert(CaptureFrame.Bar:GetObjectType() == 'Texture')
        assert(StatusFrame.Label:GetText() == 'Supplied status label')
        assert(StatusFrame.Bar.Label:GetText() == '41')
        local minimum, maximum = StatusFrame.Bar:GetMinMaxValues()
        assert(minimum == 10 and maximum == 110)
        assert(StatusFrame.Bar:GetValue() == 40.25)
        assert(StatusFrame.Bar:GetWidth() == 215)
        assert(StatusFrame.Bar.tooltip == 'Supplied status tooltip')
        assert(StatusFrame.Bar:GetStatusBarTexture())
        assert(StatusFrame.Bar.Spark:IsShown())
        assert(not StatusFrame.Bar.GlowLeft:IsShown())
        assert(not StatusFrame.Bar.GlowPulseAnim:IsPlaying())
        assert(CaptureFrame.tooltip == 'Supplied capture tooltip')
        assert(CaptureFrame:GetHeight() == 37)
        assert(math.abs(CaptureFrame.NeutralBar:GetWidth() - 27.8) < geometryEpsilon, string.format('initial NeutralBar width: actual=%.17g expected=27.8', CaptureFrame.NeutralBar:GetWidth()))
        assert(math.abs(CaptureFrame.LeftBar:GetWidth() - 55.6) < geometryEpsilon, string.format('initial LeftBar width: actual=%.17g expected=55.6', CaptureFrame.LeftBar:GetWidth()))
        assert(math.abs(CaptureFrame.RightBar:GetWidth() - 55.6) < geometryEpsilon, string.format('initial RightBar width: actual=%.17g expected=55.6', CaptureFrame.RightBar:GetWidth()))
        local point, relative, relativePoint, x, y = CaptureFrame.Spark:GetPoint(1)
        assert(point == 'CENTER' and relative == CaptureFrame and relativePoint == 'LEFT', string.format('initial Spark anchor: actual=(%s, %s, %s) expected=(CENTER, %s, LEFT)', tostring(point), tostring(relative), tostring(relativePoint), tostring(CaptureFrame)))
        assert(x == 97.5 and y == 2, string.format('initial Spark offsets: actual=(%.17g, %.17g) expected=(97.5, 2)', x, y))
        local np, nr, nrp, nx, ny = CaptureFrame.NeutralBar:GetPoint(1)
        assert(np == 'CENTER' and nr == CaptureFrame and nrp == 'LEFT', string.format('initial NeutralBar anchor: actual=(%s, %s, %s) expected=(CENTER, %s, LEFT)', tostring(np), tostring(nr), tostring(nrp), tostring(CaptureFrame)))
        assert(nx == 97.5 and ny == 0, string.format('initial NeutralBar offsets: actual=(%.17g, %.17g) expected=(97.5, 0)', nx, ny))
        assert(CaptureFrame.NeutralBar:IsShown() and CaptureFrame.Glow3:IsShown(), string.format('initial neutral visibility: NeutralBar=%s Glow3=%s Glow3Atlas=%s SparkNeutral=%s SparkNeutralAtlas=%s', tostring(CaptureFrame.NeutralBar:IsShown()), tostring(CaptureFrame.Glow3:IsShown()), tostring(CaptureFrame.Glow3:GetAtlas()), tostring(CaptureFrame.SparkNeutral:IsShown()), tostring(CaptureFrame.SparkNeutral:GetAtlas())))
        assert(not CaptureFrame.Glow1:IsShown() and not CaptureFrame.Glow2:IsShown())
        assert(CaptureFrame.SparkNeutral:IsShown() and not CaptureFrame.Spark:IsShown())
        assert(not CaptureFrame.LeftLine:IsShown() and not CaptureFrame.RightLine:IsShown())
        assert(not CaptureFrame.Divider:IsShown())
        assert(not CaptureFrame.GlowPulseAnim:IsPlaying())
    "#,
    );
    flush_and_assert_counts(env, 2, 1);
}

fn update_and_assert_live_bars(env: &WowLuaEnv) -> (StatusBarRecord, CaptureBarRecord) {
    let mut status = supplied_status();
    status.bar_min = 20.0;
    status.bar_max = 220.0;
    status.bar_value = 137.25;
    status.text = "Updated status label".into();
    status.tooltip = "Updated status tooltip".into();
    status.override_bar_text = "Updated explicit override".into();
    status.override_bar_text_shown_type = 1; // Always
    status.widget_size_setting = 240.0;
    status.order_index = 30.0; // Force genuine incremental layout dirtiness.
    let mut capture = supplied_capture();
    capture.bar_max_value = 200.0;
    capture.bar_value = 75.0;
    capture.neutral_zone_size = 40.0;
    capture.neutral_zone_center = 80.0;
    capture.fill_direction_type = 0; // RightToLeft
    capture.tooltip = "Updated capture tooltip".into();
    capture.order_index = 5.0;
    inject_status(env, status.clone());
    inject_capture(env, capture.clone());
    assert_lua(
        env,
        "live fields update before deferred layout",
        r#"
        assert(BarsContainer.widgetFrames[760201] == StatusFrame)
        assert(BarsContainer.widgetFrames[760202] == CaptureFrame)
        local minimum, maximum = StatusFrame.Bar:GetMinMaxValues()
        assert(minimum == 20 and maximum == 220)
        assert(StatusFrame.Bar:GetValue() == 137.25)
        assert(StatusFrame.Label:GetText() == 'Updated status label')
        assert(StatusFrame.Bar.Label:GetText() == 'Updated explicit override')
        assert(StatusFrame.Bar:GetWidth() == 240)
        assert(StatusFrame.Bar.tooltip == 'Updated status tooltip')
        assert(CaptureFrame.tooltip == 'Updated capture tooltip')
        -- Range=200, value=75 -> .375, RightToLeft -> .625.
        -- Cached bastionarmor barOffset=28, barWidth=139, spark y=2.
        local _, relative, _, x, y = CaptureFrame.Spark:GetPoint(1)
        assert(relative == CaptureFrame and math.abs(x - 114.875) < geometryEpsilon and y == 2)
        local _, nr, _, nx, ny = CaptureFrame.NeutralBar:GetPoint(1)
        assert(nr == CaptureFrame and math.abs(nx - 111.4) < geometryEpsilon and ny == 0)
        assert(math.abs(CaptureFrame.NeutralBar:GetWidth() - 27.8) < geometryEpsilon)
        assert(math.abs(CaptureFrame.LeftBar:GetWidth() - 69.5) < geometryEpsilon)
        assert(math.abs(CaptureFrame.RightBar:GetWidth() - 41.7) < geometryEpsilon)
        assert(CaptureFrame.NeutralBar:IsShown() and CaptureFrame.Glow3:IsShown())
        assert(CaptureFrame.SparkNeutral:IsShown() and not CaptureFrame.Spark:IsShown())
    "#,
    );
    flush_and_assert_counts(env, 2, 1);
    assert_lua(
        env,
        "actual default layout observes supplied ordering",
        r#"
        local _, relative = StatusFrame:GetPoint(1)
        assert(relative == CaptureFrame, 'updated order places status below capture')
    "#,
    );
    (status, capture)
}

fn update_and_assert_zero_neutral_zone(env: &WowLuaEnv, capture: &mut CaptureBarRecord) {
    capture.bar_value = 180.0; // RTL .1: unambiguously left of neutral center .6.
    capture.neutral_zone_size = 0.0;
    inject_capture(env, capture.clone());
    assert_lua(
        env,
        "zero-zone indicators and out-of-neutral spark",
        r#"
        assert(not CaptureFrame.NeutralBar:IsShown())
        assert(not CaptureFrame.LeftLine:IsShown() and not CaptureFrame.RightLine:IsShown())
        assert(not CaptureFrame.Glow3:IsShown())
        assert(not CaptureFrame.SparkNeutral:IsShown() and CaptureFrame.Spark:IsShown())
        local _, relative, _, x, y = CaptureFrame.Spark:GetPoint(1)
        assert(relative == CaptureFrame and math.abs(x - 41.9) < geometryEpsilon and y == 2)
        assert(math.abs(CaptureFrame.LeftBar:GetWidth() - 83.4) < geometryEpsilon)
        assert(math.abs(CaptureFrame.RightBar:GetWidth() - 55.6) < geometryEpsilon)
    "#,
    );
    flush_and_assert_counts(env, 2, 1);
}

fn hide_restore_and_remove(
    env: &WowLuaEnv,
    status: &mut StatusBarRecord,
    capture: &mut CaptureBarRecord,
) {
    status.shown_state = 0;
    inject_status(env, status.clone());
    assert_lua(
        env,
        "hidden status provider releases real frame",
        r#"
        assert(C_UIWidgetManager.GetStatusBarWidgetVisualizationInfo(760201).shownState == 0)
        assert(BarsContainer.widgetFrames[760201] == nil)
        assert(not StatusFrame:IsShown() and StatusFrame:GetParent() == nil)
        assert(StatusFrame.Bar.displayedValue == nil)
    "#,
    );
    flush_and_assert_counts(env, 1, 0);
    capture.shown_state = 0;
    inject_capture(env, capture.clone());
    assert_lua(
        env,
        "hidden capture provider releases real frame",
        r#"
        assert(C_UIWidgetManager.GetCaptureBarWidgetVisualizationInfo(760202).shownState == 0)
        assert(BarsContainer.widgetFrames[760202] == nil)
        assert(not CaptureFrame:IsShown() and CaptureFrame:GetParent() == nil)
        assert(not CaptureFrame.LeftArrowAnim:IsPlaying())
        assert(not CaptureFrame.RightArrowAnim:IsPlaying())
        assert(not CaptureFrame.GlowPulseAnim:IsPlaying())
    "#,
    );
    flush_and_assert_counts(env, 0, 0);
    status.shown_state = 1;
    capture.shown_state = 1;
    inject_status(env, status.clone());
    inject_capture(env, capture.clone());
    assert_lua(
        env,
        "restored pooled frames use current supplied records",
        r#"
        StatusFrame = assert(BarsContainer.widgetFrames[760201])
        CaptureFrame = assert(BarsContainer.widgetFrames[760202])
        assert(StatusFrame:IsShown() and CaptureFrame:IsShown())
        assert(StatusFrame.Bar:GetValue() == 137.25)
        assert(StatusFrame.Label:GetText() == 'Updated status label')
        assert(StatusFrame.Bar.Label:GetText() == 'Updated explicit override')
        assert(not CaptureFrame.NeutralBar:IsShown())
    "#,
    );
    flush_and_assert_counts(env, 2, 1);
    assert!(
        env.remove_ui_widget(SET_ID, CAPTURE_ID)
            .expect("membership removal through existing feed")
    );
    assert_lua(
        env,
        "membership-only removal preserves query record",
        r#"
        assert(BarsContainer.widgetFrames[760202] == nil)
        assert(not CaptureFrame:IsShown() and CaptureFrame:GetParent() == nil)
        assert(C_UIWidgetManager.GetCaptureBarWidgetVisualizationInfo(760202).barValue == 180)
        assert(BarsContainer.widgetFrames[760201] == StatusFrame)
    "#,
    );
    flush_and_assert_counts(env, 1, 1);
    assert!(
        env.remove_ui_widget(SET_ID, STATUS_ID)
            .expect("remove timer status membership")
    );
    assert_lua(
        env,
        "status membership removal resets bar and preserves query",
        r#"
        assert(BarsContainer.widgetFrames[760201] == nil)
        assert(not StatusFrame:IsShown() and StatusFrame:GetParent() == nil)
        assert(StatusFrame.Bar.displayedValue == nil)
        assert(C_UIWidgetManager.GetStatusBarWidgetVisualizationInfo(760201).barValue == 137.25)
    "#,
    );
    flush_and_assert_counts(env, 0, 0);
    env.replace_ui_widget_set(SET_ID, Some(bar_set()))
        .expect("[membership restoration] existing host feed restores both bars");
    flush_and_assert_counts(env, 2, 1);
}

fn unregister_and_assert_pool_cleanup(env: &WowLuaEnv) {
    assert_lua(
        env,
        "unregister all real pools/events/timers",
        r#"
        local c = BarsContainer
        local status, capture = c.widgetFrames[760201], c.widgetFrames[760202]
        assert(status and capture)
        c:UnregisterForWidgetSet()
        assert(not c:IsRegisteredForWidgetSet() and not c:IsShown())
        assert(not status:IsShown() and not capture:IsShown())
        assert(status:GetParent() == nil and capture:GetParent() == nil)
        assert(status.Bar.displayedValue == nil)
        assert(next(c.widgetFrames) == nil)
        assert(next(c.timerWidgets) == nil)
        assert(not c:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(not c:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        assert(UIWidgetManager.registeredWidgetContainers[c] == nil)
        for _ in UIWidgetManager:EnumerateWidgetsByWidgetTag('bars-status-consumer') do
            error('unregistered status must not remain published')
        end
        for _ in UIWidgetManager:EnumerateWidgetsByWidgetTag('bars-capture-consumer') do
            error('unregistered capture must not remain published')
        end
    "#,
    );
    flush_and_assert_counts(env, 0, 0);
    assert_lua(
        env,
        "post-unregister row/partition pool cleanup",
        r#"
        assert(BarsContainer.horizontalRowContainerPool:GetNumActive() == 0)
    "#,
    );
    inject_status(env, supplied_status());
    env.fire_event("UPDATE_ALL_UI_WIDGETS")
        .expect("unregistered full refresh ignored");
    flush_and_assert_counts(env, 0, 0);
    assert_lua(
        env,
        "no post-unregister reacquisition",
        "assert(next(BarsContainer.widgetFrames) == nil)",
    );
}

#[test]
fn actual_cached_status_and_capture_bar_lifecycle() {
    common::with_timeout(88, || {
        wow_ui_sim::xml::register_intrinsic_templates();
        // Real TOC dependency closure only; no API/template/manager/pool/Setup overrides.
        let cache = wow_ui_sim::paths::default_blizzard_ui_addons_path()
            .expect("[active profile cache] synced Blizzard UI addon tree is required");
        eprintln!(
            "[ui-widget-cache] profile={} path={}",
            wow_ui_sim::client_profile::ACTIVE.cache_subdir(),
            cache.display()
        );
        with_blizzard_addon_closure_in_dir(
            &cache,
            &[
                "Blizzard_SharedXMLGame",
                "Blizzard_Fonts_Shared",
                "Blizzard_FrameEffects",
                "Blizzard_UIWidgets",
            ],
            &[],
            |env, loaded| {
                assert!(loaded.iter().any(|name| name == "Blizzard_UIWidgets"));
                supply_initial_set_and_records(env);
                assert_supplied_query_boundary(env);
                register_and_assert_actual_templates(env);
                let (mut status, mut capture) = update_and_assert_live_bars(env);
                update_and_assert_zero_neutral_zone(env, &mut capture);
                hide_restore_and_remove(env, &mut status, &mut capture);
                unregister_and_assert_pool_cleanup(env);
            },
        );
    });
}
