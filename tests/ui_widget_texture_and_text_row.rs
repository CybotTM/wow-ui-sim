//! Private authored input-stage tests: execute RED before adding any row projector.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use crate::common;
use common::blizzard_addon_harness::with_blizzard_addon_closure;
use wow_ui_sim::lua_api::WowLuaEnv;

#[path = "widget_row_records/fixtures.rs"]
mod fixtures;
use fixtures::{
    SET_ID, WIDGET_ID, assert_lua, consumer_record, flush_counts, initialize_set, inject,
    install_snapshot_assertions, replacement, supplied,
};

#[test]
fn texture_and_text_row_absent_returns_exactly_one_nil() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        initialize_set(&env);
        assert_lua(
            &env,
            "callable absent getter with nonempty supplied membership",
            r#"
            local getter = C_UIWidgetManager.GetTextureAndTextRowVisualizationInfo
            assert(type(getter) == 'function')
            local function oneNil(...)
                assert(select('#', ...) == 1, 'exactly one result, not zero or multiple')
                assert(select(1, ...) == nil, 'no inferred row record')
            end
            oneNil(getter(761315))
            oneNil(getter(761316))
            local members = C_UIWidgetManager.GetAllWidgetsBySetID(435)
            assert(#members == 1 and members[1].widgetID == 761315 and members[1].widgetType == 15)
            "#,
        );
    });
}

#[test]
fn texture_and_text_row_full_snapshots_have_fresh_nested_tables_and_isolated_mutations() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        initialize_set(&env);
        inject(&env, supplied());
        install_snapshot_assertions(&env);
        let stored_before = env
            .state()
            .borrow()
            .ui_widget_inputs
            .texture_and_text_row
            .clone();
        assert_lua(
            &env,
            "eighteen fields, ordered two-field entries, nested freshness and caller mutations",
            r#"
            local first = AssertRowSnapshot(RowExpected, RowGetter(761315))
            local retained = AssertRowSnapshot(RowExpected, RowGetter(761315))
            assert(first ~= retained and first.entries ~= retained.entries)
            assert(first.entries[1] ~= first.entries[2], 'entries must not alias each other')
            for i = 1, 2 do
                for j = 1, 2 do
                    assert(first.entries[i] ~= retained.entries[j], 'cross-query entry alias')
                end
            end
            first.entries[1].text = 'caller text corruption'
            first.entries[1].tooltip = 'caller tooltip corruption'
            first.entries[1].extra = true
            first.entries[2] = nil
            first.entries[3] = {text = 'caller append', tooltip = 'caller append tooltip'}
            first.entries.extra = 'caller array mutation'
            first.widgetTag = 'caller outer corruption'
            first.fixedWidth = nil
            first.extra = 'caller outer extra'
            AssertRowSnapshot(RowExpected, retained)
            local again = AssertRowSnapshot(RowExpected, RowGetter(761315))
            assert(again ~= first and again ~= retained)
            assert(again.entries ~= first.entries and again.entries ~= retained.entries)
            for i = 1, 2 do
                for j = 1, 2 do
                    assert(again.entries[i] ~= retained.entries[j])
                end
                assert(again.entries[i] ~= first.entries[1] and again.entries[i] ~= first.entries[3])
            end
            assert(select('#', RowGetter(761316)) == 1 and RowGetter(761316) == nil)
            "#,
        );
        assert_eq!(
            env.state().borrow().ui_widget_inputs.texture_and_text_row,
            stored_before,
            "Lua result mutation cannot mutate supplied Rust input"
        );
    });
}

#[test]
fn texture_and_text_row_full_replacement_preserves_retained_snapshot() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        initialize_set(&env);
        inject(&env, supplied());
        install_snapshot_assertions(&env);
        assert_lua(
            &env,
            "retain full supplied snapshot",
            "RetainedRow = AssertRowSnapshot(RowExpected, RowGetter(761315))",
        );
        inject(&env, replacement());
        assert_lua(
            &env,
            "all eighteen outer fields and both entries replaced without modifying retained tables",
            r#"
            local current = AssertRowSnapshot(RowReplacementExpected, RowGetter(761315))
            assert(current ~= RetainedRow and current.entries ~= RetainedRow.entries)
            for i = 1, 2 do
                for j = 1, 2 do assert(current.entries[i] ~= RetainedRow.entries[j]) end
            end
            AssertRowSnapshot(RowExpected, RetainedRow)
            assert(current.fixedWidth == nil, 'replacement None must not retain old width')
            local members = C_UIWidgetManager.GetAllWidgetsBySetID(435)
            assert(#members == 1 and members[1].widgetID == 761315 and members[1].widgetType == 15)
            "#,
        );
    });
}

#[test]
fn texture_and_text_row_fixed_width_none_some_zero_some_fraction_none() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        initialize_set(&env);
        install_snapshot_assertions(&env);
        for width in [None, Some(0.0), Some(318.125), None] {
            let mut record = supplied();
            record.fixed_width = width;
            inject(&env, record);
            let expected_width = width.map_or_else(|| "nil".into(), |value| value.to_string());
            assert_lua(
                &env,
                "only nullable field round-trips without zero/default coercion",
                &format!(
                    "RowExpected.fixedWidth = {expected_width}; AssertRowSnapshot(RowExpected, RowGetter(761315))"
                ),
            );
        }
    });
}

#[test]
fn texture_and_text_row_normal_feeds_are_two_environment_isolated() {
    common::with_timeout(88, || {
        let first = WowLuaEnv::new().unwrap();
        let second = WowLuaEnv::new().unwrap();
        initialize_set(&first);
        initialize_set(&second);
        install_snapshot_assertions(&first);
        install_snapshot_assertions(&second);
        inject(&first, supplied());
        assert_lua(
            &second,
            "first environment feed cannot infer a second environment record",
            "assert(select('#', RowGetter(761315)) == 1 and RowGetter(761315) == nil)",
        );
        inject(&second, replacement());
        assert_lua(
            &first,
            "first supplied row",
            "AssertRowSnapshot(RowExpected, RowGetter(761315))",
        );
        assert_lua(
            &second,
            "second replacement row",
            "AssertRowSnapshot(RowReplacementExpected, RowGetter(761315))",
        );
        inject(&first, replacement());
        assert_lua(
            &second,
            "first replacement cannot change second",
            "AssertRowSnapshot(RowReplacementExpected, RowGetter(761315))",
        );
        inject(&second, supplied());
        assert_lua(
            &first,
            "second replacement cannot change first",
            "AssertRowSnapshot(RowReplacementExpected, RowGetter(761315))",
        );
        assert_lua(
            &second,
            "second now supplied",
            "AssertRowSnapshot(RowExpected, RowGetter(761315))",
        );
        assert!(first.remove_ui_widget(SET_ID, WIDGET_ID).unwrap());
        assert_lua(
            &second,
            "first membership removal cannot alter second membership or snapshot",
            r#"
            assert(#C_UIWidgetManager.GetAllWidgetsBySetID(435) == 1)
            AssertRowSnapshot(RowExpected, RowGetter(761315))
            "#,
        );
    });
}

#[test]
fn cached_texture_and_text_row_normal_feed_activation_update_hide_reacquire_membership_remove() {
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
                install_snapshot_assertions(env);
                assert_lua(
                    env,
                    "install real-entry assertions",
                    fixtures::CONSUMER_ASSERTIONS,
                );
                assert_lua(
                    env,
                    "actual profile-cached provider and absent-member registration",
                    r#"
                    assert(Enum.UIWidgetVisualizationType.TextureAndTextRow == 15)
                    assert(Enum.WidgetShownState.Hidden == 0 and Enum.WidgetShownState.Shown == 1)
                    assert(Enum.WidgetAnimationType.None == 0 and Enum.UIWidgetScale.Ninty == 1)
                    assert(Enum.UIWidgetHorizontalDirection.LeftToRight == 0 and Enum.UIWidgetHorizontalDirection.RightToLeft == 1)
                    assert(Enum.UIWidgetTooltipLocation.TopLeft == 3 and Enum.UIWidgetTooltipLocation.TopRight == 5)
                    local provider = assert(UIWidgetManager:GetWidgetTypeInfo(15))
                    assert(type(provider.visInfoDataFunction) == 'function')
                    assert(provider.templateInfo.frameTemplate == 'UIWidgetTemplateTextureAndTextRow')
                    RowContainer = CreateFrame('Frame', nil, UIParent, 'UIWidgetContainerTemplate')
                    RowContainer:SetPoint('CENTER', UIParent, 'CENTER')
                    RowContainer:RegisterForWidgetSet(435)
                    assert(RowContainer:IsRegisteredForWidgetSet(435))
                    assert(UIWidgetManager.registeredWidgetContainers[RowContainer])
                    assert(RowContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
                    assert(RowContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
                    local members = C_UIWidgetManager.GetAllWidgetsBySetID(435)
                    assert(#members == 1 and members[1].widgetID == 761315 and members[1].widgetType == 15)
                    assert(members[1].unitToken == nil)
                    assert(next(RowContainer.widgetFrames) == nil)
                    RowExpected.textureKit = ''; RowExpected.frameTextureKit = ''
                    RowExpected.scriptedAnimationEffectID = 0; RowExpected.fixedWidth = nil
                    "#,
                );
                flush_counts(env, 0);
                inject(env, consumer_record());
                flush_counts(env, 1);
                assert_lua(
                    env,
                    "activation through real row pool and base entry Setup",
                    r#"
                    ActiveRow = assert(RowContainer.widgetFrames[761315])
                    assert(ActiveRow.widgetID == 761315 and ActiveRow.widgetSetID == 435)
                    assert(ActiveRow.orderIndex == 37.5 and ActiveRow.widgetTag == 'row-contract-731')
                    assert(math.abs(ActiveRow:GetScale() - 0.9) < 0.0001)
                    assert(ActiveRow.fixedWidth == nil and ActiveRow.spacing == 17.25)
                    assert(ActiveRow.childLayoutDirection == 'leftToRight')
                    AssertActualRowEntries(ActiveRow, RowExpected.entries, 3)
                    AssertRowSnapshot(RowExpected, RowGetter(761315))
                    "#,
                );
                let mut updated = consumer_record();
                updated.entries.reverse();
                updated.entries[0].text = "Second now first updated 947".into();
                updated.entries[0].tooltip = "Second now first updated tooltip 947".into();
                updated.entries[1].text = "First now second updated 731".into();
                updated.entries[1].tooltip = "First now second updated tooltip 731".into();
                updated.tooltip_loc = 5;
                updated.group_alignment = 1;
                updated.fixed_width = Some(318.125);
                updated.widget_size_setting = 29.5;
                updated.widget_scale = 0;
                inject(env, updated.clone());
                flush_counts(env, 1);
                assert_lua(
                    env,
                    "incremental same-row update replaces ordered entry content and nullable width",
                    r#"
                    assert(RowContainer.widgetFrames[761315] == ActiveRow)
                    assert(ActiveRow.fixedWidth == 318.125 and ActiveRow.spacing == 29.5)
                    assert(ActiveRow.childLayoutDirection == 'rightToLeft' and ActiveRow:GetScale() == 1)
                    RowExpected.entries = {
                        {text = 'Second now first updated 947', tooltip = 'Second now first updated tooltip 947'},
                        {text = 'First now second updated 731', tooltip = 'First now second updated tooltip 731'},
                    }
                    RowExpected.tooltipLoc = 5; RowExpected.groupAlignment = 1
                    RowExpected.fixedWidth = 318.125; RowExpected.widgetSizeSetting = 29.5; RowExpected.widgetScale = 0
                    AssertActualRowEntries(ActiveRow, RowExpected.entries, 5)
                    AssertRowSnapshot(RowExpected, RowGetter(761315))
                    EntriesBeforeHide = {}
                    for entry in ActiveRow.entryPool:EnumerateActive() do table.insert(EntriesBeforeHide, entry) end
                    assert(#EntriesBeforeHide == 2)
                    "#,
                );
                let mut hidden = updated.clone();
                hidden.shown_state = 0;
                inject(env, hidden);
                flush_counts(env, 0);
                assert_lua(
                    env,
                    "hidden provider releases actual row and its nested entries, not membership",
                    r#"
                    assert(RowContainer.widgetFrames[761315] == nil and not ActiveRow:IsShown())
                    assert(ActiveRow.entryPool:GetNumActive() == 0 and ActiveRow.fixedWidth == nil)
                    for _, entry in ipairs(EntriesBeforeHide) do assert(not entry:IsShown()) end
                    assert(#C_UIWidgetManager.GetAllWidgetsBySetID(435) == 1)
                    RowExpected.shownState = 0
                    AssertRowSnapshot(RowExpected, RowGetter(761315))
                    "#,
                );
                updated.fixed_width = None;
                inject(env, updated.clone());
                flush_counts(env, 1);
                assert_lua(
                    env,
                    "ordinary feed reacquires real row and two correct entries without stale width",
                    r#"
                    ReacquiredRow = assert(RowContainer.widgetFrames[761315])
                    assert(ReacquiredRow.fixedWidth == nil and ReacquiredRow.childLayoutDirection == 'rightToLeft')
                    RowExpected.shownState = 1; RowExpected.fixedWidth = nil
                    AssertActualRowEntries(ReacquiredRow, RowExpected.entries, 5)
                    AssertRowSnapshot(RowExpected, RowGetter(761315))
                    "#,
                );
                assert!(env.remove_ui_widget(SET_ID, WIDGET_ID).unwrap());
                flush_counts(env, 0);
                assert_lua(
                    env,
                    "membership removal releases row and nested pool but retains independent record",
                    r#"
                    assert(#C_UIWidgetManager.GetAllWidgetsBySetID(435) == 0)
                    assert(next(RowContainer.widgetFrames) == nil and not ReacquiredRow:IsShown())
                    assert(ReacquiredRow.entryPool:GetNumActive() == 0)
                    AssertRowSnapshot(RowExpected, RowGetter(761315))
                    "#,
                );
                inject(env, updated);
                flush_counts(env, 1);
                assert_lua(
                    env,
                    "active unregister releases actual row and nested entries while preserving supplied membership",
                    r#"
                    local live = assert(RowContainer.widgetFrames[761315])
                    AssertActualRowEntries(live, RowExpected.entries, 5)
                    local entries = {}
                    for entry in live.entryPool:EnumerateActive() do table.insert(entries, entry) end
                    assert(#entries == 2 and live:IsShown())
                    RowContainer:UnregisterForWidgetSet()
                    assert(not RowContainer:IsRegisteredForWidgetSet(435))
                    assert(not UIWidgetManager.registeredWidgetContainers[RowContainer])
                    assert(not RowContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
                    assert(not RowContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
                    assert(next(RowContainer.widgetFrames) == nil and not live:IsShown())
                    assert(live.entryPool:GetNumActive() == 0)
                    for _, entry in ipairs(entries) do assert(not entry:IsShown()) end
                    assert(#C_UIWidgetManager.GetAllWidgetsBySetID(435) == 1)
                    AssertRowSnapshot(RowExpected, RowGetter(761315))
                    "#,
                );
                assert!(env.remove_ui_widget(SET_ID, WIDGET_ID).unwrap());
                assert!(!env.remove_ui_widget(SET_ID, WIDGET_ID).unwrap());
            },
        );
    });
}
