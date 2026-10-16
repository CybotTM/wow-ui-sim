//! Public schemas (Retail/PTR AddOns/Blizzard_APIDocumentationGenerated/):
//! UIWidgetManagerDocumentation.lua:170-181,1488-1507,2044-2057.
//! Synthetic host values, not native IDs or catalog fixtures.
//! Explicit synthetic 15+10-field inputs; not native catalog data.
use super::helpers::{assert_lua, install_snapshot_assertion};
use wow_ui_sim::c_api::ui_widget_manager::{
    HorizontalCurrenciesRecord, UiWidgetCurrencyRecord, WidgetId, WidgetSetEntry, WidgetSetRecord,
    WidgetVisualization,
};
use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) const ID: f64 = 934_209.0;
pub(super) const SET_ID: f64 = 449.0;
pub(super) const TYPE: u32 = 9;
pub(super) const GETTER: &str = "GetHorizontalCurrenciesWidgetVisualizationInfo";

fn supplied_currencies() -> Vec<UiWidgetCurrencyRecord> {
    vec![
        UiWidgetCurrencyRecord {
            icon_file_id: 134400,
            leading_text: "Earned".into(),
            text: "17 / 80".into(),
            tooltip: "First currency tooltip 731".into(),
            is_currency_maxed: false,
            text_font_type: 0,
            text_size_type: 0,
            text_enabled_state: 4,
            icon_size_type: 0,
            update_anim_type: 0,
        },
        UiWidgetCurrencyRecord {
            icon_file_id: 134414,
            leading_text: "".into(),
            text: "43".into(),
            tooltip: "Second currency tooltip 947".into(),
            is_currency_maxed: true,
            text_font_type: 1,
            text_size_type: 4,
            text_enabled_state: 2,
            icon_size_type: 2,
            update_anim_type: 0,
        },
    ]
}

pub(super) fn supplied() -> HorizontalCurrenciesRecord {
    HorizontalCurrenciesRecord {
        shown_state: 1,
        currencies: supplied_currencies(),
        tooltip_loc: 3,
        widget_size_setting: 180.5,
        texture_kit: "horizontalcurrencies-fixture-731".into(),
        frame_texture_kit: "horizontalcurrencies-frame-731".into(),
        has_timer: false,
        order_index: 37.5,
        widget_tag: "horizontalcurrencies-contract-731".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 1,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 0.0,
    }
}

// Projection only: never activate this timer/fade/effect record in the cached consumer.
fn replacement_currencies() -> Vec<UiWidgetCurrencyRecord> {
    vec![
        UiWidgetCurrencyRecord {
            icon_file_id: 134415,
            leading_text: "Spent".into(),
            text: "26 / 90".into(),
            tooltip: "Replacement first tooltip".into(),
            is_currency_maxed: true,
            text_font_type: 2,
            text_size_type: 1,
            text_enabled_state: 0,
            icon_size_type: 1,
            update_anim_type: 1,
        },
        UiWidgetCurrencyRecord {
            icon_file_id: 134416,
            leading_text: "Remaining".into(),
            text: "65".into(),
            tooltip: "Replacement second tooltip".into(),
            is_currency_maxed: false,
            text_font_type: 0,
            text_size_type: 2,
            text_enabled_state: 3,
            icon_size_type: 3,
            update_anim_type: 2,
        },
    ]
}

pub(super) fn replacement() -> HorizontalCurrenciesRecord {
    HorizontalCurrenciesRecord {
        shown_state: 0,
        currencies: replacement_currencies(),
        tooltip_loc: 6,
        widget_size_setting: 206.25,
        texture_kit: "replacement-currency-icon".into(),
        frame_texture_kit: "replacement-currency-frame".into(),
        has_timer: true,
        order_index: -12.25,
        widget_tag: "replacement-horizontalcurrencies-tag".into(),
        in_anim_type: 1,
        out_anim_type: 1,
        widget_scale: 0,
        layout_direction: 2,
        model_scene_layer: 1,
        scripted_animation_effect_id: 947.5,
    }
}

pub(super) fn updated() -> HorizontalCurrenciesRecord {
    let mut record = supplied();
    record.texture_kit = "horizontalcurrencies-fixture-947".into();
    record.widget_tag = "updated-horizontalcurrencies-tag".into();
    record.order_index = -12.25;
    record.widget_scale = 0;
    record.currencies.reverse();
    record.currencies[0].text = "Updated 43".into();
    record
}

pub(super) fn entry() -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id: ID,
        widget_type: TYPE,
        unit_token: None,
    }
}

pub(super) fn seed(env: &WowLuaEnv, id: f64, record: HorizontalCurrenciesRecord) {
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .horizontal_currencies
        .insert(WidgetId::from_number(id), record);
}

pub(super) fn supply_empty_set(env: &WowLuaEnv) {
    env.replace_ui_widget_set(
        SET_ID,
        Some(WidgetSetRecord {
            layout_direction: 0,
            vertical_padding: 7.0,
            widgets: Vec::new(),
        }),
    )
    .expect("normal empty set feed");
}

pub(super) fn supply(env: &WowLuaEnv, record: HorizontalCurrenciesRecord) {
    env.update_ui_widget(
        SET_ID,
        entry(),
        WidgetVisualization::HorizontalCurrencies(record),
    )
    .expect("normal type9 feed publishes complete state before event");
}

pub(super) fn install_assertions(env: &WowLuaEnv) {
    install_snapshot_assertion(env, GETTER, "{}");
    assert_lua(
        env,
        "install nested behavioral assertions",
        include_str!("snapshots.lua"),
    );
}

pub(super) fn expect(env: &WowLuaEnv, expression: &str) {
    assert_lua(
        env,
        "select independent expected snapshot",
        &format!("FixtureExpected = {expression}"),
    );
}
