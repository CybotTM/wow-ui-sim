//! Initial absent-record boundaries over existing supplied-set APIs and cached consumers.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use crate::common;
use common::blizzard_addon_harness::with_blizzard_addon_closure;
use wow_ui_sim::c_api::ui_widget_manager::{
    TextureAndTextRecord, WidgetSetEntry, WidgetSetRecord, WidgetVisualization,
};

#[path = "widget_two_records/helpers.rs"]
mod helpers;
#[path = "widget_two_records/texture.rs"]
mod records;
use helpers::{
    assert_fresh_snapshot, assert_lua, assert_replacement_snapshot, assert_selector_policy,
    flush_counts, install_snapshot_assertion,
};
use wow_ui_sim::lua_api::WowLuaEnv;

const SET_ID: f64 = 432.0;
const WIDGET_ID: f64 = 761312.0;
const WIDGET_TYPE: u32 = 12;

fn inject(env: &WowLuaEnv, id: f64, record: TextureAndTextRecord) {
    env.update_ui_widget(
        SET_ID,
        WidgetSetEntry {
            widget_id: id,
            widget_type: WIDGET_TYPE,
            unit_token: None,
        },
        WidgetVisualization::TextureAndText(record),
    )
    .expect("ordinary typed feed stores texture/text input before notification");
}

fn initialize_set(env: &WowLuaEnv) {
    env.replace_ui_widget_set(SET_ID, Some(supplied_set_without_visualization()))
        .expect("supply independent membership");
}

#[test]
fn texture_and_text_complete_19_field_snapshots_are_fresh_and_replacements_isolated() {
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

#[cfg(feature = "retail-12-1-0")]
#[test]
fn texture_and_text_secret_selectors_preserve_wrappers_and_deny_tainted_callers() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        initialize_set(&env);
        inject(&env, WIDGET_ID, records::supplied());
        install_snapshot_assertion(&env, records::GETTER, records::EXPECTED);
        let (sets, records) = {
            let state = env.state();
            let state = state.borrow();
            (
                state.ui_widget_inputs.sets.clone(),
                state.ui_widget_inputs.texture_and_text.clone(),
            )
        };
        helpers::assert_secret_selector_contract(&env, WIDGET_ID);
        let state = env.state();
        let state = state.borrow();
        assert_eq!(state.ui_widget_inputs.sets, sets);
        assert_eq!(state.ui_widget_inputs.texture_and_text, records);
    });
}

#[test]
fn texture_and_text_supplied_records_are_environment_local() {
    common::with_timeout(88, || {
        let first = WowLuaEnv::new().unwrap();
        let second = WowLuaEnv::new().unwrap();
        initialize_set(&first);
        initialize_set(&second);
        inject(&first, WIDGET_ID, records::supplied());
        assert_lua(
            &second,
            "unseeded environment",
            "assert(C_UIWidgetManager.GetTextureAndTextVisualizationInfo(761312) == nil)",
        );
        inject(&second, WIDGET_ID, records::replacement());
        install_snapshot_assertion(&first, records::GETTER, records::EXPECTED);
        install_snapshot_assertion(&second, records::GETTER, records::REPLACEMENT_EXPECTED);
        assert_lua(
            &first,
            "first environment",
            "AssertSuppliedSnapshot(FixtureGetter(761312))",
        );
        assert_lua(
            &second,
            "second environment",
            "AssertSuppliedSnapshot(FixtureGetter(761312))",
        );
        inject(&first, WIDGET_ID, records::replacement());
        install_snapshot_assertion(&first, records::GETTER, records::REPLACEMENT_EXPECTED);
        inject(&second, WIDGET_ID, records::supplied());
        install_snapshot_assertion(&second, records::GETTER, records::EXPECTED);
        assert_lua(
            &first,
            "second feed cannot mutate first",
            "AssertSuppliedSnapshot(FixtureGetter(761312))",
        );
        assert_lua(
            &second,
            "first feed cannot mutate second",
            "AssertSuppliedSnapshot(FixtureGetter(761312))",
        );
    });
}

#[test]
fn texture_and_text_selectors_follow_shared_required_number_policy() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        initialize_set(&env);
        inject(&env, WIDGET_ID + 0.5, records::supplied());
        install_snapshot_assertion(&env, records::GETTER, records::EXPECTED);
        assert_selector_policy(&env, WIDGET_ID + 0.5);
    });
}

#[test]
fn cached_texture_and_text_feed_activates_updates_hides_and_removes_visible_text() {
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
                    assert(UIWidgetManager:GetWidgetTypeInfo(12))
                    assert(Enum.WidgetShownState.Hidden == 0 and Enum.WidgetShownState.Shown == 1)
                    assert(Enum.WidgetAnimationType.None == 0 and Enum.UIWidgetUpdateAnimType.None == 0)
                    assert(Enum.UIWidgetTextFormatType.None == 0)
                    assert(Enum.UIWidgetTooltipLocation.TopLeft == 3 and Enum.UIWidgetTooltipLocation.TopRight == 5)
                    FixtureContainer = CreateFrame('Frame', nil, UIParent, 'UIWidgetContainerTemplate')
                    FixtureContainer:SetPoint('CENTER', UIParent, 'CENTER')
                    FixtureContainer:RegisterForWidgetSet(432)
                    assert(FixtureContainer:IsRegisteredForWidgetSet(432))
                    assert(next(FixtureContainer.widgetFrames) == nil)
                "#,
                );
                flush_counts(env, 0);
                inject(env, WIDGET_ID, records::supplied());
                assert_lua(
                    env,
                    "actual visible text and tooltip metadata",
                    r#"
                    FixtureFrame = assert(FixtureContainer.widgetFrames[761312])
                    assert(FixtureFrame:IsVisible() and FixtureFrame.Text:IsVisible())
                    assert(FixtureFrame.widgetID == 761312 and FixtureFrame.widgetSetID == 432)
                    assert(FixtureFrame.Text:GetText() == 'TextureText initial 731')
                    assert(FixtureFrame.tooltip == 'TextureText tooltip initial 731')
                    assert(FixtureFrame.tooltipLoc == 3 and FixtureFrame.tooltipAnchor == 'ANCHOR_LEFT')
                    assert(FixtureFrame:IsMouseEnabled() and not FixtureFrame:IsMouseClickEnabled())
                    assert(math.abs(FixtureFrame:GetScale() - 0.9) < 0.0001)
                    assert(FixtureFrame.orderIndex == 37.5 and FixtureFrame.widgetTag == 'texture-text-contract-731')
                "#,
                );
                install_snapshot_assertion(env, records::GETTER, records::EXPECTED);
                assert_lua(
                    env,
                    "active complete snapshot",
                    "AssertSuppliedSnapshot(FixtureGetter(761312))",
                );
                flush_counts(env, 1);
                let mut updated = records::supplied();
                updated.text = "TextureText updated 947".into();
                updated.tooltip = "TextureText tooltip updated 947".into();
                updated.tooltip_loc = 5;
                updated.text_size_type = 1;
                updated.widget_scale = 0;
                inject(env, WIDGET_ID, updated.clone());
                assert_lua(
                    env,
                    "incremental in-place update",
                    r#"
                    assert(FixtureContainer.widgetFrames[761312] == FixtureFrame)
                    assert(FixtureFrame:IsVisible() and FixtureFrame.Text:IsVisible())
                    assert(FixtureFrame.Text:GetText() == 'TextureText updated 947')
                    assert(FixtureFrame.tooltip == 'TextureText tooltip updated 947')
                    assert(FixtureFrame.tooltipLoc == 5 and FixtureFrame.tooltipAnchor == 'ANCHOR_RIGHT')
                    assert(FixtureFrame:IsMouseEnabled() and not FixtureFrame:IsMouseClickEnabled())
                    assert(FixtureFrame:GetScale() == 1)
                    FixtureExpected.text = 'TextureText updated 947'
                    FixtureExpected.tooltip = 'TextureText tooltip updated 947'
                    FixtureExpected.tooltipLoc = 5; FixtureExpected.textSizeType = 1; FixtureExpected.widgetScale = 0
                    AssertSuppliedSnapshot(FixtureGetter(761312))
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
                    assert(FixtureContainer.widgetFrames[761312] == nil and not FixtureFrame:IsShown())
                    assert(#C_UIWidgetManager.GetAllWidgetsBySetID(432) == 1)
                    FixtureExpected.shownState = 0
                    AssertSuppliedSnapshot(FixtureGetter(761312))
                "#,
                );
                inject(env, WIDGET_ID, updated);
                flush_counts(env, 1);
                assert_lua(
                    env,
                    "restore via feed",
                    r#"
                    RestoredFrame = assert(FixtureContainer.widgetFrames[761312])
                    assert(RestoredFrame:IsVisible() and RestoredFrame.Text:IsVisible())
                    assert(RestoredFrame.Text:GetText() == 'TextureText updated 947')
                    assert(RestoredFrame.tooltip == 'TextureText tooltip updated 947' and RestoredFrame.tooltipLoc == 5)
                    FixtureExpected.shownState = 1
                    AssertSuppliedSnapshot(FixtureGetter(761312))
                "#,
                );
                assert!(env.remove_ui_widget(SET_ID, WIDGET_ID).unwrap());
                flush_counts(env, 0);
                assert_lua(
                    env,
                    "membership removal retains independent record and unregister cleans up",
                    r#"
                    assert(#C_UIWidgetManager.GetAllWidgetsBySetID(432) == 0)
                    assert(next(FixtureContainer.widgetFrames) == nil and not RestoredFrame:IsShown())
                    AssertSuppliedSnapshot(FixtureGetter(761312))
                    FixtureContainer:UnregisterForWidgetSet()
                    assert(not FixtureContainer:IsRegisteredForWidgetSet(432))
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
fn texture_and_text_getter_is_callable_and_absent_id_returns_exactly_one_nil() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().expect("initialize existing runtime API surface");
        env.exec(
            r#"
            local getter = C_UIWidgetManager.GetTextureAndTextVisualizationInfo
            assert(type(getter) == 'function', 'exact visualization getter must be callable')
            local function assertOneNil(...)
                assert(select('#', ...) == 1, 'absent record must return exactly one result')
                assert(select(1, ...) == nil, 'absent record result must be nil')
            end
            assertOneNil(getter(761312))
            "#,
        )
        .expect("exact getter callability and absent supplied-record result boundary");
    });
}

#[test]
fn cached_texture_and_text_consumer_skips_member_without_visualization() {
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
                    local setInfo = C_UIWidgetManager.GetWidgetSetInfo(432)
                    assert(setInfo and setInfo.layoutDirection == Enum.UIWidgetSetLayoutDirection.Vertical)
                    assert(setInfo.verticalPadding == 7)
                    local members = C_UIWidgetManager.GetAllWidgetsBySetID(432)
                    assert(#members == 1, 'nonvacuity: supplied set must contain one member')
                    assert(members[1].widgetSetID == 432)
                    assert(members[1].widgetID == 761312 and members[1].widgetType == 12)
                    assert(members[1].unitToken == nil, 'normal member has no unit filter')
                    assert(Enum.UIWidgetVisualizationType.TextureAndText == 12)
                    local provider = UIWidgetManager:GetWidgetTypeInfo(12)
                    assert(provider, 'nonvacuity: cached visualization provider must be registered')
                    assert(type(provider.visInfoDataFunction) == 'function')
                    AbsentVisualizationContainer = CreateFrame('Frame', nil, UIParent, 'UIWidgetContainerTemplate')
                    AbsentVisualizationContainer:SetPoint('CENTER', UIParent, 'CENTER')
                    -- No direct getter precheck here: the actual cached provider must call it.
                    AbsentVisualizationContainer:RegisterForWidgetSet(432)
                    assert(AbsentVisualizationContainer:IsRegisteredForWidgetSet(432))
                    assert(UIWidgetManager.registeredWidgetContainers[AbsentVisualizationContainer])
                    assert(AbsentVisualizationContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
                    assert(AbsentVisualizationContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
                    assert(AbsentVisualizationContainer.widgetFrames[761312] == nil)
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
                    local members = C_UIWidgetManager.GetAllWidgetsBySetID(432)
                    assert(#members == 1 and members[1].widgetID == 761312 and members[1].widgetType == 12)
                    assert(AbsentVisualizationContainer:IsRegisteredForWidgetSet(432))
                    assert(next(AbsentVisualizationContainer.widgetFrames) == nil)
                    assert(AbsentVisualizationContainer.widgetPools:GetNumActive() == 0)
                    assert(AbsentVisualizationContainer:GetNumWidgetsShowing() == 0)
                    AbsentVisualizationContainer:UnregisterForWidgetSet()
                    assert(not AbsentVisualizationContainer:IsRegisteredForWidgetSet(432))
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
