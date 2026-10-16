//! Host-side passive controls only; no HorizontalCurrencies getter calls.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::c_api::ui_widget_manager::{
    HorizontalCurrenciesRecord, UiWidgetCurrencyRecord, WidgetId, WidgetSetEntry, WidgetSetRecord,
    WidgetVisualization,
};
use wow_ui_sim::lua_api::WowLuaEnv;

const SET: f64 = 449.0;
const ID: f64 = 934209.0;

fn record() -> HorizontalCurrenciesRecord {
    HorizontalCurrenciesRecord {
        shown_state: 1,
        currencies: vec![UiWidgetCurrencyRecord {
            icon_file_id: 134400,
            leading_text: "Earned".into(),
            text: "17 / 80".into(),
            tooltip: "Passive supplied row".into(),
            is_currency_maxed: false,
            text_font_type: 0,
            text_size_type: 0,
            text_enabled_state: 4,
            icon_size_type: 0,
            update_anim_type: 0,
        }],
        tooltip_loc: 3,
        widget_size_setting: 180.5,
        texture_kit: "passive-horizontal".into(),
        frame_texture_kit: "passive-frame".into(),
        has_timer: false,
        order_index: 37.5,
        widget_tag: "passive-horizontal-tag".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 1,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 0.0,
    }
}

fn entry() -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id: ID,
        widget_type: 9,
        unit_token: None,
    }
}

fn supply_empty_set(env: &WowLuaEnv) {
    env.replace_ui_widget_set(
        SET,
        Some(WidgetSetRecord {
            layout_direction: 0,
            vertical_padding: 7.0,
            widgets: vec![],
        }),
    )
    .unwrap();
}

#[test]
fn horizontal_currencies_passive_host_storage_feed_replacement_and_membership_removal() {
    let env = WowLuaEnv::new().unwrap();
    let other = WowLuaEnv::new().unwrap();
    assert!(
        env.state()
            .borrow()
            .ui_widget_inputs
            .horizontal_currencies
            .is_empty()
    );
    supply_empty_set(&env);
    let supplied = record();
    env.update_ui_widget(
        SET,
        entry(),
        WidgetVisualization::HorizontalCurrencies(supplied.clone()),
    )
    .unwrap();
    let retained = env
        .state()
        .borrow()
        .ui_widget_inputs
        .horizontal_currencies
        .clone();
    assert_eq!(retained.get(&WidgetId::from_number(ID)), Some(&supplied));
    assert_eq!(
        env.state().borrow().ui_widget_inputs.sets[&WidgetId::from_number(SET)].widgets,
        vec![entry()]
    );
    assert!(
        other
            .state()
            .borrow()
            .ui_widget_inputs
            .horizontal_currencies
            .is_empty()
    );
    let mut replacement = supplied.clone();
    replacement.currencies[0].text = "Replaced passive row".into();
    env.update_ui_widget(
        SET,
        entry(),
        WidgetVisualization::HorizontalCurrencies(replacement.clone()),
    )
    .unwrap();
    assert_eq!(
        env.state().borrow().ui_widget_inputs.horizontal_currencies[&WidgetId::from_number(ID)],
        replacement
    );
    assert_eq!(retained[&WidgetId::from_number(ID)], supplied);
    assert!(env.remove_ui_widget(SET, ID).unwrap());
    assert!(
        env.state().borrow().ui_widget_inputs.sets[&WidgetId::from_number(SET)]
            .widgets
            .is_empty()
    );
    assert_eq!(
        env.state().borrow().ui_widget_inputs.horizontal_currencies[&WidgetId::from_number(ID)],
        replacement
    );
    env.replace_ui_widget_set(SET, None).unwrap();
    assert!(env.state().borrow().ui_widget_inputs.sets.is_empty());
    assert_eq!(
        env.state().borrow().ui_widget_inputs.horizontal_currencies[&WidgetId::from_number(ID)],
        replacement
    );
}

#[test]
fn horizontal_currencies_passive_rejected_feed_preserves_host_state_and_sends_no_event() {
    let env = WowLuaEnv::new().unwrap();
    supply_empty_set(&env);
    env.update_ui_widget(
        SET,
        entry(),
        WidgetVisualization::HorizontalCurrencies(record()),
    )
    .unwrap();
    env.exec(
        r#"
        PassiveEvents = 0
        PassiveListener = CreateFrame('Frame')
        PassiveListener:RegisterEvent('UPDATE_UI_WIDGET')
        PassiveListener:RegisterEvent('UPDATE_ALL_UI_WIDGETS')
        PassiveListener:SetScript('OnEvent', function() PassiveEvents = PassiveEvents + 1 end)
    "#,
    )
    .unwrap();
    let sets = env.state().borrow().ui_widget_inputs.sets.clone();
    let rows = env
        .state()
        .borrow()
        .ui_widget_inputs
        .horizontal_currencies
        .clone();
    let mut wrong_type = entry();
    wrong_type.widget_type = 8;
    assert!(
        env.update_ui_widget(
            SET,
            wrong_type,
            WidgetVisualization::HorizontalCurrencies(record())
        )
        .is_err()
    );
    assert!(
        env.update_ui_widget(
            SET + 1.0,
            entry(),
            WidgetVisualization::HorizontalCurrencies(record())
        )
        .is_err()
    );
    assert!(!env.remove_ui_widget(SET, ID + 1.0).unwrap());
    assert_eq!(env.state().borrow().ui_widget_inputs.sets, sets);
    assert_eq!(
        env.state().borrow().ui_widget_inputs.horizontal_currencies,
        rows
    );
    env.exec("assert(PassiveEvents == 0)").unwrap();
    assert!(env.state().borrow().lua_errors.is_empty());
}
