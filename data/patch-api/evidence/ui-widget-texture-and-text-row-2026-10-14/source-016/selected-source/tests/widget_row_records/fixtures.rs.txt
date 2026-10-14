//! Literal synthetic inputs and behavioral assertions, not catalog/default records.
use wow_ui_sim::c_api::ui_widget_manager::{
    TextureAndTextEntryRecord, TextureAndTextRowRecord, WidgetSetEntry, WidgetSetRecord,
    WidgetVisualization,
};
use wow_ui_sim::lua_api::WowLuaEnv;

pub const SET_ID: f64 = 435.0;
pub const WIDGET_ID: f64 = 761315.0;
pub const WIDGET_TYPE: u32 = 15;

pub fn supplied() -> TextureAndTextRowRecord {
    TextureAndTextRowRecord {
        shown_state: 1,
        entries: vec![
            TextureAndTextEntryRecord {
                text: "First row text 731".into(),
                tooltip: "First row tooltip 731".into(),
            },
            TextureAndTextEntryRecord {
                text: "Second row text 947".into(),
                tooltip: "Second row tooltip 947".into(),
            },
        ],
        text_size_type: 2,
        group_alignment: 0,
        fixed_width: Some(243.75),
        tooltip_loc: 3,
        widget_size_setting: 17.25,
        texture_kit: "row-foreground-fixture".into(),
        frame_texture_kit: "row-background-fixture".into(),
        has_timer: false,
        order_index: 37.5,
        widget_tag: "row-contract-731".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 1,
        layout_direction: 2,
        model_scene_layer: 0,
        scripted_animation_effect_id: 731.0,
    }
}

// Every outer field, both entries and optional width change; projection-only fixture.
pub fn replacement() -> TextureAndTextRowRecord {
    TextureAndTextRowRecord {
        shown_state: 0,
        entries: vec![
            TextureAndTextEntryRecord {
                text: "Replacement first text".into(),
                tooltip: "Replacement first tooltip".into(),
            },
            TextureAndTextEntryRecord {
                text: "Replacement second text".into(),
                tooltip: "Replacement second tooltip".into(),
            },
        ],
        text_size_type: 1,
        group_alignment: 1,
        fixed_width: None,
        tooltip_loc: 5,
        widget_size_setting: 29.5,
        texture_kit: "replacement-row-foreground".into(),
        frame_texture_kit: "replacement-row-background".into(),
        has_timer: true,
        order_index: -12.25,
        widget_tag: "replacement-row-contract".into(),
        in_anim_type: 1,
        out_anim_type: 2,
        widget_scale: 2,
        layout_direction: 1,
        model_scene_layer: 1,
        scripted_animation_effect_id: 947.5,
    }
}

/// Real cached consumer deliberately excludes atlas/animation/effect prerequisites.
pub fn consumer_record() -> TextureAndTextRowRecord {
    let mut record = supplied();
    record.texture_kit = "".into();
    record.frame_texture_kit = "".into();
    record.scripted_animation_effect_id = 0.0;
    record.fixed_width = None;
    record
}

pub fn initialize_set(env: &WowLuaEnv) {
    env.replace_ui_widget_set(
        SET_ID,
        Some(WidgetSetRecord {
            layout_direction: 0,
            vertical_padding: 7.0,
            widgets: vec![membership()],
        }),
    )
    .expect("normal full-set feed supplies nonempty membership without a row record");
}

fn membership() -> WidgetSetEntry {
    WidgetSetEntry {
        widget_id: WIDGET_ID,
        widget_type: WIDGET_TYPE,
        unit_token: None,
    }
}

pub fn inject(env: &WowLuaEnv, record: TextureAndTextRowRecord) {
    env.update_ui_widget(
        SET_ID,
        membership(),
        WidgetVisualization::TextureAndTextRow(record),
    )
    .expect("normal typed feed stores row input before synchronous notification");
}

pub fn assert_lua(env: &WowLuaEnv, phase: &str, code: &str) {
    env.exec(code)
        .unwrap_or_else(|error| panic!("TextureAndTextRow [{phase}]: {error}"));
    let errors = env.state().borrow().lua_errors.clone();
    assert!(
        errors.is_empty(),
        "[{phase}] recorded Lua errors: {errors:?}"
    );
}

pub fn install_snapshot_assertions(env: &WowLuaEnv) {
    assert_lua(
        env,
        "install exact deep snapshot assertions",
        SNAPSHOT_ASSERTIONS,
    );
}

pub fn flush_counts(env: &WowLuaEnv, phase: &str, count: usize) {
    env.fire_on_update(0.0)
        .expect("flush actual cached deferred layout without manual widget events");
    assert_lua(
        env,
        phase,
        &format!(
            r#"
            local active = RowContainer.widgetPools:GetNumActive()
            assert(active == {count}, 'active pool expected {count}, got ' .. tostring(active))
            local showing = RowContainer:GetNumWidgetsShowing()
            assert(showing == {count}, 'showing expected {count}, got ' .. tostring(showing))
            assert(RowContainer.numTimers == 0, 'timers expected 0, got ' .. tostring(RowContainer.numTimers))
            assert(RowContainer.ticker == nil, 'ticker expected nil, got ' .. tostring(RowContainer.ticker))
            "#
        ),
    );
}

const SNAPSHOT_ASSERTIONS: &str = r#"
RowGetter = C_UIWidgetManager.GetTextureAndTextRowVisualizationInfo
RowExpected = {
    shownState = 1,
    entries = {
        {text = 'First row text 731', tooltip = 'First row tooltip 731'},
        {text = 'Second row text 947', tooltip = 'Second row tooltip 947'},
    },
    textSizeType = 2, groupAlignment = 0, fixedWidth = 243.75, tooltipLoc = 3,
    widgetSizeSetting = 17.25, textureKit = 'row-foreground-fixture',
    frameTextureKit = 'row-background-fixture', hasTimer = false, orderIndex = 37.5,
    widgetTag = 'row-contract-731', inAnimType = 0, outAnimType = 0, widgetScale = 1,
    layoutDirection = 2, modelSceneLayer = 0, scriptedAnimationEffectID = 731,
}
RowReplacementExpected = {
    shownState = 0,
    entries = {
        {text = 'Replacement first text', tooltip = 'Replacement first tooltip'},
        {text = 'Replacement second text', tooltip = 'Replacement second tooltip'},
    },
    textSizeType = 1, groupAlignment = 1, tooltipLoc = 5, widgetSizeSetting = 29.5,
    textureKit = 'replacement-row-foreground', frameTextureKit = 'replacement-row-background',
    hasTimer = true, orderIndex = -12.25, widgetTag = 'replacement-row-contract',
    inAnimType = 1, outAnimType = 2, widgetScale = 2, layoutDirection = 1,
    modelSceneLayer = 1, scriptedAnimationEffectID = 947.5,
}
function AssertExactRowTable(expected, actual, path)
    assert(type(actual) == 'table', path .. ' must be a table')
    local expectedCount = 0
    for key, value in pairs(expected) do
        expectedCount = expectedCount + 1
        local fieldPath = path .. '.' .. tostring(key)
        assert(type(actual[key]) == type(value), 'wrong type: ' .. fieldPath)
        if type(value) == 'table' then
            AssertExactRowTable(value, actual[key], fieldPath)
        else
            assert(actual[key] == value, 'wrong value: ' .. fieldPath)
        end
    end
    local actualCount = 0
    for key in pairs(actual) do
        actualCount = actualCount + 1
        assert(expected[key] ~= nil, 'unexpected field: ' .. path .. '.' .. tostring(key))
    end
    assert(actualCount == expectedCount, 'missing or extra fields: ' .. path)
    return actualCount
end
function AssertRowSnapshot(expected, ...)
    assert(select('#', ...) == 1, 'row getter must return exactly one result')
    local actual = ...
    local fieldCount = AssertExactRowTable(expected, actual, 'row')
    assert(fieldCount == (expected.fixedWidth == nil and 17 or 18))
    assert(actual.fixedWidth == expected.fixedWidth, 'None must be nil, Some must be supplied number')
    assert(#actual.entries == 2, 'ordered two-entry snapshot')
    for index = 1, 2 do
        assert(AssertExactRowTable(expected.entries[index], actual.entries[index], 'entry') == 2)
    end
    return actual
end
"#;

pub const CONSUMER_ASSERTIONS: &str = r#"
function AssertActualRowEntries(frame, expectedEntries, tooltipLoc)
    assert(frame:IsVisible(), 'row must be visible')
    assert(frame.entryPool:GetNumActive() == #expectedEntries, 'actual entry pool count')
    local seen = {}
    local activeCount = 0
    for entry in frame.entryPool:EnumerateActive() do
        activeCount = activeCount + 1
        local index = entry.layoutIndex
        assert(type(index) == 'number' and expectedEntries[index], 'entry order')
        assert(not seen[index], 'duplicate entry order')
        seen[index] = true
        assert(entry:IsVisible() and entry.Text:IsVisible(), 'visible real entry and FontString')
        assert(entry.Text:GetText() == expectedEntries[index].text, 'actual ordered entry text')
        assert(entry.tooltip == expectedEntries[index].tooltip, 'actual entry tooltip')
        assert(entry.tooltipLoc == tooltipLoc, 'row tooltip location reaches each entry')
        assert(entry.tooltipAnchor == (tooltipLoc == 3 and 'ANCHOR_LEFT' or 'ANCHOR_RIGHT'))
        assert(entry:IsMouseEnabled() and not entry:IsMouseClickEnabled())
    end
    assert(activeCount == #expectedEntries)
    for index = 1, #expectedEntries do assert(seen[index], 'missing ordered entry') end
end
"#;
