//! Initial absent-record boundaries over existing supplied-set APIs and cached consumers.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use crate::common;
use common::blizzard_addon_harness::with_blizzard_addon_closure;
use wow_ui_sim::c_api::ui_widget_manager::{WidgetSetEntry, WidgetSetRecord};
use wow_ui_sim::lua_api::WowLuaEnv;

const SET_ID: f64 = 432.0;
const WIDGET_ID: f64 = 761312.0;
const WIDGET_TYPE: u32 = 12;

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
