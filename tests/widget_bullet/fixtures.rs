use wow_ui_sim::c_api::ui_widget_manager::{
    BulletTextListRecord, WidgetId, WidgetSetEntry, WidgetSetRecord, WidgetVisualization,
};
use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) const ID: f64 = 760_401.0;
pub(super) const SET_ID: f64 = 431.0;

pub(super) fn supplied_record() -> BulletTextListRecord {
    BulletTextListRecord {
        shown_state: 1,
        enabled_state: 1,
        lines: vec!["first".into(), String::new(), "third".into()],
        widget_size_setting: 237.5,
        texture_kit: "supplied-kit".into(),
        frame_texture_kit: "supplied-frame-kit".into(),
        has_timer: true,
        order_index: -12.5,
        widget_tag: "bullet-supplied".into(),
        in_anim_type: 1,
        out_anim_type: 1,
        widget_scale: 6,
        layout_direction: 1,
        model_scene_layer: 2,
        scripted_animation_effect_id: 987.25,
    }
}

pub(super) fn consumer_record() -> BulletTextListRecord {
    BulletTextListRecord {
        widget_size_setting: 237.0,
        texture_kit: String::new(),
        frame_texture_kit: String::new(),
        has_timer: false,
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 0,
        model_scene_layer: 0,
        scripted_animation_effect_id: 0.0,
        ..supplied_record()
    }
}

pub(super) fn supply_record(env: &WowLuaEnv, id: f64, record: BulletTextListRecord) {
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .bullet_text_lists
        .insert(WidgetId::from_number(id), record);
}

pub(super) fn entry() -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id: ID,
        widget_type: 10,
        unit_token: None,
    }
}

pub(super) fn supply_empty_set(env: &WowLuaEnv) {
    env.replace_ui_widget_set(
        SET_ID,
        Some(WidgetSetRecord {
            layout_direction: 0,
            vertical_padding: 7.0,
            widgets: vec![],
        }),
    )
    .expect("explicit empty host set");
}

pub(super) fn update(env: &WowLuaEnv, record: BulletTextListRecord) {
    env.update_ui_widget(SET_ID, entry(), WidgetVisualization::BulletTextList(record))
        .expect("typed host feed");
}

pub(super) fn assert_lua(env: &WowLuaEnv, phase: &str, code: &str) {
    env.eval::<()>(code)
        .unwrap_or_else(|error| panic!("bullet-text-list [{phase}]: {error}"));
    let errors = env.state().borrow().lua_errors.clone();
    assert!(
        errors.is_empty(),
        "bullet-text-list [{phase}] Lua errors: {errors:?}"
    );
}

pub(super) fn install_snapshot_assertions(env: &WowLuaEnv) {
    assert_lua(
        env,
        "install test assertions, not replacement APIs",
        include_str!("snapshot_assertions.lua"),
    );
}

pub(super) fn assert_initial_snapshot(env: &WowLuaEnv) {
    install_snapshot_assertions(env);
    assert_lua(
        env,
        "first genuine positive-data getter RED",
        r#"
        local get = C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo
        assert(type(get) == 'function', 'missing BulletTextList getter')
        local row = get(760401)
        assert(type(row) == 'table', 'missing supplied BulletTextList record')
        AssertBulletSnapshot(row, expectedBullet)
        retainedBullet = row
    "#,
    );
}
