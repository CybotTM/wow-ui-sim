use super::discrete_fixtures::assert_lua;
use wow_ui_sim::c_api::ui_widget_manager::{
    SpellDisplayRecord, WidgetId, WidgetSetEntry, WidgetSpellInfoRecord,
};
use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) const ID: f64 = 761_301.0;
pub(super) const SET_ID: f64 = 431.0;

/// Explicit host fixture. Enums literal in BOTH cached Retail/PTR declarations.
/// 35395 is existing data/spells.rs:16092 Crusader Strike, icon135891; not a widget catalog.
pub(super) fn supplied_record() -> SpellDisplayRecord {
    SpellDisplayRecord {
        spell_info: WidgetSpellInfoRecord {
            spell_id: 35395.0,
            shown_state: 1,
            enabled_state: 3,
            tooltip: "Supplied spell tooltip".into(),
            text: "Supplied spell".into(),
            stack_display: 3.0,
            icon_size_type: 2,
            icon_display_type: 0,
            text_shown_state: 0,
            border_color: 8,
            text_font_type: 0,
            text_size_type: 1,
            h_align_type: 0,
            tint: 0,
            show_glow_state: 0,
            show_as_earned: false,
        },
        tooltip_loc: 4,
        widget_size_setting: 180.0,
        texture_kit: String::new(),
        frame_texture_kit: String::new(),
        has_timer: false,
        order_index: 12.5,
        widget_tag: "spell-display-supplied".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 0,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 0.0,
    }
}

pub(super) fn entry() -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id: ID,
        widget_type: 13,
        unit_token: None,
    }
}

pub(super) fn seed_record(env: &WowLuaEnv, id: f64, record: SpellDisplayRecord) {
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .spell_display
        .insert(WidgetId::from_number(id), record);
}

pub(super) fn assert_initial_snapshot(env: &WowLuaEnv) {
    // First supplied getter TABLE boundary, before cached construction or secret helpers.
    assert_lua(
        env,
        "SpellDisplay supplied table boundary",
        r#"
        local get = C_UIWidgetManager.GetSpellDisplayVisualizationInfo
        assert(type(get) == 'function', 'missing SpellDisplay getter')
        local first = get(761301)
        assert(type(first) == 'table', 'missing supplied SpellDisplay record')
        expectedSpell = {
            spellInfo = { spellID=35395, shownState=1, enabledState=3,
                tooltip='Supplied spell tooltip', text='Supplied spell', stackDisplay=3,
                iconSizeType=2, iconDisplayType=0, textShownState=0, borderColor=8,
                textFontType=0, textSizeType=1, hAlignType=0, tint=0,
                showGlowState=0, showAsEarned=false },
            tooltipLoc=4, widgetSizeSetting=180, textureKit='', frameTextureKit='',
            hasTimer=false, orderIndex=12.5, widgetTag='spell-display-supplied',
            inAnimType=0, outAnimType=0, widgetScale=0, layoutDirection=1,
            modelSceneLayer=0, scriptedAnimationEffectID=0,
        }
        function AssertSpellSnapshot(actual, expected)
            assert(type(actual) == 'table', 'complete supplied table')
            local count = 0
            for key, value in pairs(expected) do
                assert(type(actual[key]) == type(value), 'field type: '..key)
                if type(value) == 'table' then
                    AssertSpellSnapshot(actual[key], value)
                else
                    assert(actual[key] == value, 'field value: '..key)
                end
            end
            for key in pairs(actual) do
                count = count + 1
                assert(expected[key] ~= nil, 'unexpected field: '..tostring(key))
            end
            local expectedCount = expected.spellInfo and 14 or 16
            assert(count == expectedCount, 'exact complete nested snapshot')
        end
        AssertSpellSnapshot(first, expectedSpell)
        savedSpell = first
    "#,
    );
}
