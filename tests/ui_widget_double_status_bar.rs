//! Initial absent-record boundaries over existing supplied-set APIs and cached consumers.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use crate::common;
use common::blizzard_addon_harness::with_blizzard_addon_closure;
use wow_ui_sim::c_api::ui_widget_manager::{
    DoubleStatusBarRecord, WidgetSetEntry, WidgetSetRecord, WidgetVisualization,
};

#[path = "widget_two_records/helpers.rs"]
mod helpers;
#[path = "widget_two_records/double.rs"]
mod records;
use helpers::{
    assert_fresh_snapshot, assert_lua, assert_replacement_snapshot, assert_selector_policy,
    flush_counts, install_snapshot_assertion,
};
use wow_ui_sim::lua_api::WowLuaEnv;

const SET_ID: f64 = 431.0;
const WIDGET_ID: f64 = 761301.0;
const WIDGET_TYPE: u32 = 3;

fn inject(env: &WowLuaEnv, id: f64, record: DoubleStatusBarRecord) {
    env.update_ui_widget(
        SET_ID,
        WidgetSetEntry {
            widget_id: id,
            widget_type: WIDGET_TYPE,
            unit_token: None,
        },
        WidgetVisualization::DoubleStatusBar(record),
    )
    .expect("ordinary typed feed stores double-bar input before notification");
}

fn initialize_set(env: &WowLuaEnv) {
    env.replace_ui_widget_set(SET_ID, Some(supplied_set_without_visualization()))
        .expect("supply independent membership");
}

#[test]
fn double_status_bar_complete_26_field_snapshots_are_fresh_and_replacements_isolated() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        initialize_set(&env);
        inject(&env, WIDGET_ID, records::supplied());
        install_snapshot_assertion(&env, records::GETTER, records::EXPECTED);
        assert_fresh_snapshot(&env, WIDGET_ID);
        inject(&env, WIDGET_ID, records::replacement());
        install_snapshot_assertion(&env, records::GETTER, records::REPLACEMENT_EXPECTED);
        assert_replacement_snapshot(&env, WIDGET_ID);
    });
}

#[test]
fn double_status_bar_supplied_records_are_environment_local() {
    common::with_timeout(88, || {
        let first = WowLuaEnv::new().unwrap();
        let second = WowLuaEnv::new().unwrap();
        initialize_set(&first);
        initialize_set(&second);
        inject(&first, WIDGET_ID, records::supplied());
        assert_lua(
            &second,
            "unseeded environment",
            "assert(C_UIWidgetManager.GetDoubleStatusBarWidgetVisualizationInfo(761301) == nil)",
        );
        inject(&second, WIDGET_ID, records::replacement());
        install_snapshot_assertion(&first, records::GETTER, records::EXPECTED);
        install_snapshot_assertion(&second, records::GETTER, records::REPLACEMENT_EXPECTED);
        assert_lua(
            &first,
            "first environment",
            "AssertSuppliedSnapshot(FixtureGetter(761301))",
        );
        assert_lua(
            &second,
            "second environment",
            "AssertSuppliedSnapshot(FixtureGetter(761301))",
        );
        inject(&first, WIDGET_ID, records::replacement());
        install_snapshot_assertion(&first, records::GETTER, records::REPLACEMENT_EXPECTED);
        inject(&second, WIDGET_ID, records::supplied());
        install_snapshot_assertion(&second, records::GETTER, records::EXPECTED);
        assert_lua(
            &first,
            "second feed cannot mutate first",
            "AssertSuppliedSnapshot(FixtureGetter(761301))",
        );
        assert_lua(
            &second,
            "first feed cannot mutate second",
            "AssertSuppliedSnapshot(FixtureGetter(761301))",
        );
    });
}

#[test]
fn double_status_bar_selectors_follow_shared_required_number_policy() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        initialize_set(&env);
        // A supplied fractional identity is valid under the existing shared f64 policy.
        inject(&env, WIDGET_ID + 0.5, records::supplied());
        install_snapshot_assertion(&env, records::GETTER, records::EXPECTED);
        assert_selector_policy(&env, WIDGET_ID + 0.5);
    });
}

#[test]
fn cached_double_status_bar_feed_activates_updates_hides_and_removes_visible_bars() {
    common::with_timeout(88, || {
        with_blizzard_addon_closure(
            &[
                "Blizzard_SharedXMLGame",
                "Blizzard_Fonts_Shared",
                "Blizzard_FrameEffects",
                "Blizzard_UIWidgets",
            ],
            &[],
            |env, loaded| {
                assert!(loaded.iter().any(|name| name == "Blizzard_UIWidgets"));
                initialize_set(env);
                assert_lua(
                    env,
                    "real cached registration before record",
                    r#"
                    assert(UIWidgetManager:GetWidgetTypeInfo(3))
                    assert(Enum.WidgetShownState.Hidden == 0 and Enum.WidgetShownState.Shown == 1)
                    assert(Enum.WidgetAnimationType.None == 0 and Enum.UIWidgetMotionType.Instant == 0)
                    -- Empty kits leave these bars without a fill texture for the real Spark anchors.
                    -- Use an existing real atlas, never fabricate atlas info or replace Setup.
                    assert(C_Texture.GetAtlasInfo('widgetstatusbar-fill-white'), 'real bar fill atlas prerequisite')
                    FixtureContainer = CreateFrame('Frame', nil, UIParent, 'UIWidgetContainerTemplate')
                    FixtureContainer:SetPoint('CENTER', UIParent, 'CENTER')
                    FixtureContainer:RegisterForWidgetSet(431)
                    assert(FixtureContainer:IsRegisteredForWidgetSet(431))
                    assert(next(FixtureContainer.widgetFrames) == nil)
                "#,
                );
                flush_counts(env, 0);
                inject(env, WIDGET_ID, records::supplied());
                assert_lua(
                    env,
                    "actual visible bars and shared label",
                    r#"
                    FixtureFrame = assert(FixtureContainer.widgetFrames[761301])
                    assert(FixtureFrame:IsVisible())
                    assert(FixtureFrame.widgetID == 761301 and FixtureFrame.widgetSetID == 431)
                    assert(FixtureFrame.LeftBar:IsVisible() and FixtureFrame.RightBar:IsVisible())
                    local leftMin, leftMax = FixtureFrame.LeftBar:GetMinMaxValues()
                    local rightMin, rightMax = FixtureFrame.RightBar:GetMinMaxValues()
                    assert(leftMin == 10 and leftMax == 110 and FixtureFrame.LeftBar:GetValue() == 40.25)
                    assert(rightMin == -20 and rightMax == 80 and FixtureFrame.RightBar:GetValue() == 35.5)
                    assert(FixtureFrame.LeftBar:GetWidth() == 105 and FixtureFrame.RightBar:GetWidth() == 105)
                    assert(FixtureFrame.Label:IsVisible() and FixtureFrame.Label:GetText() == 'Double supplied label')
                    assert(FixtureFrame.LeftBar.tooltip == 'Left supplied tooltip')
                    assert(FixtureFrame.RightBar.tooltip == 'Right supplied tooltip')
                    assert(FixtureFrame.LeftBar.tooltipLoc == 3 and FixtureFrame.RightBar.tooltipLoc == 4)
                "#,
                );
                install_snapshot_assertion(env, records::GETTER, records::EXPECTED);
                assert_lua(
                    env,
                    "active complete snapshot",
                    "AssertSuppliedSnapshot(FixtureGetter(761301))",
                );
                flush_counts(env, 1);
                let mut updated = records::supplied();
                updated.left_bar_min = -7.5;
                updated.left_bar_max = 200.25;
                updated.left_bar_value = 91.5;
                updated.right_bar_min = -50.25;
                updated.right_bar_max = 150.5;
                updated.right_bar_value = -2.25;
                updated.text = "Updated double label".into();
                updated.left_bar_tooltip = "Updated left tooltip".into();
                updated.right_bar_tooltip = "Updated right tooltip".into();
                updated.left_bar_tooltip_loc = 4;
                updated.right_bar_tooltip_loc = 3;
                updated.widget_size_setting = 127.0;
                inject(env, WIDGET_ID, updated.clone());
                assert_lua(
                    env,
                    "incremental in-place update",
                    r#"
                    assert(FixtureContainer.widgetFrames[761301] == FixtureFrame)
                    assert(FixtureFrame:IsVisible() and FixtureFrame.LeftBar:IsVisible() and FixtureFrame.RightBar:IsVisible())
                    local leftMin, leftMax = FixtureFrame.LeftBar:GetMinMaxValues()
                    local rightMin, rightMax = FixtureFrame.RightBar:GetMinMaxValues()
                    assert(leftMin == -7.5 and leftMax == 200.25 and FixtureFrame.LeftBar:GetValue() == 91.5)
                    assert(rightMin == -50.25 and rightMax == 150.5 and FixtureFrame.RightBar:GetValue() == -2.25)
                    assert(FixtureFrame.LeftBar:GetWidth() == 127 and FixtureFrame.RightBar:GetWidth() == 127)
                    assert(FixtureFrame.Label:IsVisible() and FixtureFrame.Label:GetText() == 'Updated double label')
                    assert(FixtureFrame.LeftBar.tooltip == 'Updated left tooltip' and FixtureFrame.LeftBar.tooltipLoc == 4)
                    assert(FixtureFrame.RightBar.tooltip == 'Updated right tooltip' and FixtureFrame.RightBar.tooltipLoc == 3)
                    FixtureExpected.leftBarMin = -7.5; FixtureExpected.leftBarMax = 200.25; FixtureExpected.leftBarValue = 91.5
                    FixtureExpected.rightBarMin = -50.25; FixtureExpected.rightBarMax = 150.5; FixtureExpected.rightBarValue = -2.25
                    FixtureExpected.text = 'Updated double label'; FixtureExpected.widgetSizeSetting = 127
                    FixtureExpected.leftBarTooltip = 'Updated left tooltip'; FixtureExpected.rightBarTooltip = 'Updated right tooltip'
                    FixtureExpected.leftBarTooltipLoc = 4; FixtureExpected.rightBarTooltipLoc = 3
                    AssertSuppliedSnapshot(FixtureGetter(761301))
                "#,
                );
                flush_counts(env, 1);
                let mut hidden = updated.clone();
                hidden.shown_state = 0;
                inject(env, WIDGET_ID, hidden);
                flush_counts(env, 0);
                assert_lua(
                    env,
                    "hidden record releases cached frame",
                    r#"
                    assert(FixtureContainer.widgetFrames[761301] == nil and not FixtureFrame:IsShown())
                    assert(#C_UIWidgetManager.GetAllWidgetsBySetID(431) == 1)
                    FixtureExpected.shownState = 0
                    AssertSuppliedSnapshot(FixtureGetter(761301))
                "#,
                );
                inject(env, WIDGET_ID, updated);
                flush_counts(env, 1);
                assert_lua(
                    env,
                    "restore via feed",
                    r#"
                    RestoredFrame = assert(FixtureContainer.widgetFrames[761301])
                    assert(RestoredFrame:IsVisible())
                    assert(RestoredFrame.LeftBar:GetValue() == 91.5 and RestoredFrame.RightBar:GetValue() == -2.25)
                    assert(RestoredFrame.Label:GetText() == 'Updated double label')
                    FixtureExpected.shownState = 1
                    AssertSuppliedSnapshot(FixtureGetter(761301))
                "#,
                );
                assert!(env.remove_ui_widget(SET_ID, WIDGET_ID).unwrap());
                flush_counts(env, 0);
                assert_lua(
                    env,
                    "membership removal retains independent record and unregister cleans up",
                    r#"
                    assert(#C_UIWidgetManager.GetAllWidgetsBySetID(431) == 0)
                    assert(next(FixtureContainer.widgetFrames) == nil and not RestoredFrame:IsShown())
                    AssertSuppliedSnapshot(FixtureGetter(761301))
                    FixtureContainer:UnregisterForWidgetSet()
                    assert(not FixtureContainer:IsRegisteredForWidgetSet(431))
                    assert(not UIWidgetManager.registeredWidgetContainers[FixtureContainer])
                    assert(not FixtureContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
                    assert(not FixtureContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
                    assert(FixtureContainer.widgetPools:GetNumActive() == 0)
                "#,
                );
            },
        );
    });
}

fn supplied_set_without_visualization() -> WidgetSetRecord {
    WidgetSetRecord {
        layout_direction: 0, // UIWidgetSetLayoutDirection.Vertical
        vertical_padding: 7.0,
        widgets: vec![WidgetSetEntry {
            widget_id: WIDGET_ID,
            widget_type: WIDGET_TYPE,
            unit_token: None,
        }],
    }
}

#[test]
fn double_status_bar_getter_is_callable_and_absent_id_returns_exactly_one_nil() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().expect("initialize existing runtime API surface");
        env.exec(
            r#"
            local getter = C_UIWidgetManager.GetDoubleStatusBarWidgetVisualizationInfo
            assert(type(getter) == 'function', 'exact visualization getter must be callable')
            local function assertOneNil(...)
                assert(select('#', ...) == 1, 'absent record must return exactly one result')
                assert(select(1, ...) == nil, 'absent record result must be nil')
            end
            assertOneNil(getter(761301))
            "#,
        )
        .expect("exact getter callability and absent supplied-record result boundary");
    });
}

#[test]
fn cached_double_status_bar_consumer_skips_member_without_visualization() {
    common::with_timeout(88, || {
        with_blizzard_addon_closure(
            &[
                "Blizzard_SharedXMLGame",
                "Blizzard_Fonts_Shared",
                "Blizzard_FrameEffects",
                "Blizzard_UIWidgets",
            ],
            &[],
            |env, loaded| {
                assert!(loaded.iter().any(|name| name == "Blizzard_UIWidgets"));
                assert!(
                    env.state().borrow().lua_errors.is_empty(),
                    "closure load errors"
                );
                env.replace_ui_widget_set(SET_ID, Some(supplied_set_without_visualization()))
                    .expect("supply normal nonempty membership without any visualization record");
                env.exec(
                    r#"
                    local setInfo = C_UIWidgetManager.GetWidgetSetInfo(431)
                    assert(setInfo and setInfo.layoutDirection == Enum.UIWidgetSetLayoutDirection.Vertical)
                    assert(setInfo.verticalPadding == 7)
                    local members = C_UIWidgetManager.GetAllWidgetsBySetID(431)
                    assert(#members == 1, 'nonvacuity: supplied set must contain one member')
                    assert(members[1].widgetSetID == 431)
                    assert(members[1].widgetID == 761301 and members[1].widgetType == 3)
                    assert(members[1].unitToken == nil, 'normal member has no unit filter')
                    assert(Enum.UIWidgetVisualizationType.DoubleStatusBar == 3)
                    local provider = UIWidgetManager:GetWidgetTypeInfo(3)
                    assert(provider, 'nonvacuity: cached visualization provider must be registered')
                    assert(type(provider.visInfoDataFunction) == 'function')
                    AbsentVisualizationContainer = CreateFrame('Frame', nil, UIParent, 'UIWidgetContainerTemplate')
                    AbsentVisualizationContainer:SetPoint('CENTER', UIParent, 'CENTER')
                    -- No direct getter precheck here: the actual cached provider must call it.
                    AbsentVisualizationContainer:RegisterForWidgetSet(431)
                    assert(AbsentVisualizationContainer:IsRegisteredForWidgetSet(431))
                    assert(UIWidgetManager.registeredWidgetContainers[AbsentVisualizationContainer])
                    assert(AbsentVisualizationContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
                    assert(AbsentVisualizationContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
                    assert(AbsentVisualizationContainer.widgetFrames[761301] == nil)
                    assert(next(AbsentVisualizationContainer.widgetFrames) == nil)
                    assert(AbsentVisualizationContainer.widgetPools:GetNumActive() == 0)
                    assert(AbsentVisualizationContainer:GetNumWidgetsShowing() == 0)
                    "#,
                )
                .expect("real cached registration must safely skip absent visualization via its provider");
                assert!(
                    env.state().borrow().lua_errors.is_empty(),
                    "cached registration recorded errors: {:?}",
                    env.state().borrow().lua_errors
                );
                env.replace_ui_widget_set(SET_ID, Some(supplied_set_without_visualization()))
                    .expect("normal host feed must safely reconcile the same absent member");
                env.exec(
                    r#"
                    local members = C_UIWidgetManager.GetAllWidgetsBySetID(431)
                    assert(#members == 1 and members[1].widgetID == 761301 and members[1].widgetType == 3)
                    assert(AbsentVisualizationContainer:IsRegisteredForWidgetSet(431))
                    assert(next(AbsentVisualizationContainer.widgetFrames) == nil)
                    assert(AbsentVisualizationContainer.widgetPools:GetNumActive() == 0)
                    assert(AbsentVisualizationContainer:GetNumWidgetsShowing() == 0)
                    AbsentVisualizationContainer:UnregisterForWidgetSet()
                    assert(not AbsentVisualizationContainer:IsRegisteredForWidgetSet(431))
                    "#,
                )
                .expect("real cached full-set notification must preserve no acquisition");
                assert!(
                    env.state().borrow().lua_errors.is_empty(),
                    "cached reconciliation recorded errors: {:?}",
                    env.state().borrow().lua_errors
                );
            },
        );
    });
}
