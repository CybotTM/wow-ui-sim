//! Concrete synthetic host inputs, not native values or inferred catalog rows.
use wow_ui_sim::c_api::ui_widget_manager::{
    TextColumnRowEntryInfo, TextColumnRowVisualizationInfo, WidgetId, WidgetSetEntry,
    WidgetSetRecord, WidgetVisualization,
};
use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) const ID: f64 = 763_621.0;
pub(super) const SET_ID: f64 = 436.0;
pub(super) const TYPE: u32 = 21;

pub(super) fn supplied() -> TextColumnRowVisualizationInfo {
    TextColumnRowVisualizationInfo {
        shown_state: 1,
        entries: vec![
            entry_record("Alpha 731", 3, 0, 111.25),
            entry_record("", 0, 1, 87.5),
            entry_record("Gamma 731", 4, 2, 146.75),
        ],
        text_size_type: 1,
        font_type: 0,
        tooltip: "Column tooltip 731".into(),
        tooltip_loc: 3,
        bottom_padding: 6.5,
        widget_size_setting: 40.5,
        texture_kit: String::new(),
        frame_texture_kit: String::new(),
        has_timer: false,
        order_index: 37.5,
        widget_tag: "column-731".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 1,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 0.0,
    }
}

fn entry_record(text: &str, enabled: u32, align: u32, width: f64) -> TextColumnRowEntryInfo {
    TextColumnRowEntryInfo {
        text: text.into(),
        enabled_state: enabled,
        h_align: align,
        column_width: width,
    }
}

/// Every outer field and each retained entry field differs. Query-only asset/effect values.
pub(super) fn replacement() -> TextColumnRowVisualizationInfo {
    TextColumnRowVisualizationInfo {
        shown_state: 0,
        entries: vec![
            entry_record("Delta 947", 2, 2, 91.5),
            entry_record("Epsilon 947", 4, 0, 201.25),
        ],
        text_size_type: 4,
        font_type: 1,
        tooltip: "Column tooltip 947".into(),
        tooltip_loc: 5,
        bottom_padding: 8.25,
        widget_size_setting: 52.25,
        texture_kit: "query-only-column-kit".into(),
        frame_texture_kit: "query-only-frame-kit".into(),
        has_timer: true,
        order_index: -12.25,
        widget_tag: "column-947".into(),
        in_anim_type: 1,
        out_anim_type: 1,
        widget_scale: 0,
        layout_direction: 2,
        model_scene_layer: 1,
        scripted_animation_effect_id: 947.5,
    }
}

/// Explicit neutral effects/texture inputs keep consumer tests independent of a catalog.
pub(super) fn consumer_update() -> TextColumnRowVisualizationInfo {
    let mut record = replacement();
    record.shown_state = 1;
    record.texture_kit.clear();
    record.frame_texture_kit.clear();
    record.has_timer = false;
    record.in_anim_type = 0;
    record.out_anim_type = 0;
    record.layout_direction = 1;
    record.model_scene_layer = 0;
    record.scripted_animation_effect_id = 0.0;
    record
}

pub(super) fn seed(env: &WowLuaEnv, id: f64, record: TextColumnRowVisualizationInfo) {
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .text_column_rows
        .insert(WidgetId::from_number(id), record);
}

pub(super) fn membership() -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id: ID,
        widget_type: TYPE,
        unit_token: None,
    }
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
    .expect("explicit empty set");
}

pub(super) fn publish(env: &WowLuaEnv, record: TextColumnRowVisualizationInfo) {
    env.update_ui_widget(
        SET_ID,
        membership(),
        WidgetVisualization::TextColumnRow(record),
    )
    .expect("normal typed TextColumnRow feed");
}

pub(super) fn assert_lua(env: &WowLuaEnv, phase: &str, code: &str) {
    env.exec(code)
        .unwrap_or_else(|error| panic!("text-column [{phase}]: {error}"));
    let errors = env.state().borrow().lua_errors.clone();
    assert!(
        errors.is_empty(),
        "text-column [{phase}] Lua errors: {errors:?}"
    );
}

pub(super) fn install_assertions(env: &WowLuaEnv) {
    assert_lua(
        env,
        "observational contract assertions",
        include_str!("snapshot.lua"),
    );
}

pub(super) fn populated_boundary(env: &WowLuaEnv) {
    install_assertions(env);
    assert_lua(
        env,
        "populated getter boundary before consumer prerequisites",
        r#"
        ColumnRetained = ColumnOneSnapshot(ColumnExpected, ColumnGet(763621))
    "#,
    );
}
