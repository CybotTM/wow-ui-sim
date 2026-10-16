//! Grounding (Retail/PTR AddOns/Blizzard_UIWidgets/):
//! Blizzard_UIWidgetManager.lua:100-169,555-608 (layout, reconciliation, sorting).
//! Cached sorting/layout and full-set reconciliation, without replacing layoutFunc.
use super::fixtures::{self, ID, SET_ID};
use super::helpers::{assert_lua, flush_counts};
use wow_ui_sim::c_api::ui_widget_manager::{WidgetSetRecord, WidgetVisualization};
use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_vertical_order(env: &WowLuaEnv, first: f64, second: f64) {
    assert_lua(
        env,
        "cached sort order determines anchors and frame levels",
        &format!(
            r#"
            local first = assert(FixtureContainer.widgetFrames[{first}])
            local second = assert(FixtureContainer.widgetFrames[{second}])
            local _, firstRelative = first:GetPoint(1)
            local _, secondRelative = second:GetPoint(1)
            assert(firstRelative == FixtureContainer, 'first sorted widget anchors to container')
            assert(secondRelative == first, 'second sorted widget anchors to first')
            assert(first:GetFrameLevel() == FixtureContainer:GetFrameLevel() + 1)
            assert(second:GetFrameLevel() == FixtureContainer:GetFrameLevel() + 2)
            "#
        ),
    );
}

fn add_companion_and_reorder(env: &WowLuaEnv) {
    let mut companion = fixtures::entry();
    companion.widget_id = ID + 1.0;
    env.update_ui_widget(
        SET_ID,
        companion,
        WidgetVisualization::HorizontalCurrencies(fixtures::supplied()),
    )
    .expect("activate second type9 through normal host feed");
    flush_counts(env, 2);
    assert_vertical_order(env, ID, ID + 1.0);
    assert_lua(
        env,
        "companion content from normal feed",
        r#"
        AssertHorizontalInitial(FixtureContainer.widgetFrames[934210])
        AssertCurrencyAt(934210, CurrencyExpected)
    "#,
    );
    let mut moved = fixtures::updated();
    moved.order_index = 77.0;
    fixtures::supply(env, moved);
    flush_counts(env, 2);
    assert_vertical_order(env, ID + 1.0, ID);
    fixtures::supply(env, fixtures::updated());
    flush_counts(env, 2);
    assert_vertical_order(env, ID, ID + 1.0);
}

fn replace_set_dropping_companion(env: &WowLuaEnv) {
    assert_lua(
        env,
        "retain companion frame before full membership replacement",
        "CompanionFrame = assert(FixtureContainer.widgetFrames[934210])",
    );
    env.replace_ui_widget_set(
        SET_ID,
        Some(WidgetSetRecord {
            layout_direction: 0,
            vertical_padding: 7.0,
            widgets: vec![fixtures::entry()],
        }),
    )
    .expect("normal full-set feed reconciles cached consumers");
    flush_counts(env, 1);
    assert_lua(
        env,
        "full replacement releases companion, retains queryable records and survivor identity",
        r#"
        assert(FixtureContainer.widgetFrames[934210] == nil and not CompanionFrame:IsShown())
        assert(CompanionFrame.currencyPool:GetNumActive() == 0, 'full replacement releases nested rows')
        assert(FixtureContainer.widgetFrames[934209] == FixtureFrame)
        local members = C_UIWidgetManager.GetAllWidgetsBySetID(449)
        assert(#members == 1 and members[1].widgetID == 934209)
        AssertSuppliedSnapshot(FixtureGetter(934209))
        local currentExpected = FixtureExpected
        FixtureExpected = SavedHorizontalExpected
        AssertSuppliedSnapshot(FixtureGetter(934210))
        FixtureExpected = currentExpected
        "#,
    );
}

pub(super) fn assert_ordering_and_set_replacement(env: &WowLuaEnv) {
    add_companion_and_reorder(env);
    replace_set_dropping_companion(env);
}
