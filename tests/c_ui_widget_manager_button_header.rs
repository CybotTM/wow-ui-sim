//! Supplied ButtonHeader projection and cached-provider behavior.
//! Input scaffold staged; query and executed behavioral proof remain pending.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use std::path::Path;

#[cfg(feature = "retail-12-1-0")]
use rilua::LuaApiMut;
#[cfg(feature = "retail-12-1-0")]
use rilua::table_security::wrap_host_secret_number;
use wow_ui_sim::c_api::ui_widget_manager::{ButtonHeaderRecord, WidgetId, WidgetSpellButtonRecord};
use wow_ui_sim::lua_api::WowLuaEnv;

const FIRST_ID: f64 = 760_041.0;
const SECOND_ID: f64 = 760_042.0;
const HIDDEN_ID: f64 = 760_043.0;
const EMPTY_ID: f64 = 760_044.0;

// Explicit supplied keys/data, not a native catalog or manufactured defaults.
fn first_record() -> ButtonHeaderRecord {
    ButtonHeaderRecord {
        shown_state: 1, // Shown
        header_text: "Supplied north header".into(),
        tooltip: "Supplied north tooltip".into(),
        tooltip_loc: 7, // BottomRight
        buttons: north_buttons(),
        widget_size_setting: 23.5,
        texture_kit: "supplied-north-content-kit".into(),
        frame_texture_kit: "evergreen-scenario-widget-short".into(),
        has_timer: true,
        order_index: 37.25,
        widget_tag: "supplied-north-tag".into(),
        in_anim_type: 1,      // Fade
        out_anim_type: 0,     // None, explicitly supplied
        widget_scale: 7,      // OneHundredTwenty
        layout_direction: 2,  // Horizontal
        model_scene_layer: 1, // Front
        scripted_animation_effect_id: 91_007.0,
    }
}

fn north_buttons() -> Vec<WidgetSpellButtonRecord> {
    vec![
        WidgetSpellButtonRecord {
            spell_id: 190_001.0,
            tooltip: "First supplied button".into(),
            icon: 3,          // Checkmark
            enabled_state: 1, // Enabled
            cooldown_type: 2, // ShowCooldownAndDisableOnCooldown
        },
        WidgetSpellButtonRecord {
            spell_id: 190_002.0,
            tooltip: "Second supplied button".into(),
            icon: 4,          // RedX
            enabled_state: 0, // Disabled
            cooldown_type: 1, // ShowCooldown
        },
    ]
}

fn second_record() -> ButtonHeaderRecord {
    ButtonHeaderRecord {
        shown_state: 1,
        header_text: "Supplied south header".into(),
        tooltip: "Supplied south tooltip".into(),
        tooltip_loc: 2, // Left
        buttons: vec![WidgetSpellButtonRecord {
            spell_id: 290_003.0,
            tooltip: "South supplied button".into(),
            icon: 1, // Speak
            enabled_state: 1,
            cooldown_type: 0, // HideCooldown
        }],
        widget_size_setting: 46.5,
        texture_kit: "supplied-south-content-kit".into(),
        frame_texture_kit: "supplied-south-frame-kit".into(),
        has_timer: false,
        order_index: 73.75,
        widget_tag: "supplied-south-tag".into(),
        in_anim_type: 0,
        out_anim_type: 1,
        widget_scale: 3,      // Seventy
        layout_direction: 1,  // Vertical
        model_scene_layer: 2, // Back
        scripted_animation_effect_id: 92_008.0,
    }
}

fn supply_record(env: &WowLuaEnv, widget_id: f64, record: ButtonHeaderRecord) {
    env.state()
        .borrow_mut()
        .button_header_inputs
        .widgets
        .insert(WidgetId::from_number(widget_id), record);
}

const EXPECTATIONS: &str = r#"
    local query = C_UIWidgetManager.GetButtonHeaderWidgetVisualizationInfo
    assert(type(query) == 'function', 'ButtonHeader query must be published')
    local expectedNorth = {
        shownState = 1, headerText = 'Supplied north header',
        tooltip = 'Supplied north tooltip', tooltipLoc = 7,
        buttons = {
            {spellID = 190001, tooltip = 'First supplied button', icon = 3,
             enabledState = 1, cooldownType = 2},
            {spellID = 190002, tooltip = 'Second supplied button', icon = 4,
             enabledState = 0, cooldownType = 1},
        },
        widgetSizeSetting = 23.5, textureKit = 'supplied-north-content-kit',
        frameTextureKit = 'evergreen-scenario-widget-short', hasTimer = true,
        orderIndex = 37.25, widgetTag = 'supplied-north-tag',
        inAnimType = 1, outAnimType = 0, widgetScale = 7,
        layoutDirection = 2, modelSceneLayer = 1,
        scriptedAnimationEffectID = 91007,
    }
    local expectedSouth = {
        shownState = 1, headerText = 'Supplied south header',
        tooltip = 'Supplied south tooltip', tooltipLoc = 2,
        buttons = {
            {spellID = 290003, tooltip = 'South supplied button', icon = 1,
             enabledState = 1, cooldownType = 0},
        },
        widgetSizeSetting = 46.5, textureKit = 'supplied-south-content-kit',
        frameTextureKit = 'supplied-south-frame-kit', hasTimer = false,
        orderIndex = 73.75, widgetTag = 'supplied-south-tag',
        inAnimType = 0, outAnimType = 1, widgetScale = 3,
        layoutDirection = 1, modelSceneLayer = 2,
        scriptedAnimationEffectID = 92008,
    }
    local function assertSameData(actual, expected, path)
        assert(type(actual) == type(expected), path .. ': wrong Lua type')
        if type(expected) ~= 'table' then
            assert(actual == expected, path .. ': supplied value changed')
            return
        end
        for key, value in pairs(expected) do
            assertSameData(actual[key], value, path .. '.' .. tostring(key))
        end
        for key in pairs(actual) do
            assert(expected[key] ~= nil, path .. ': unexpected projected key')
        end
    end
"#;

fn assert_query_behavior(env: &WowLuaEnv, body: &str) {
    env.exec(&format!("{EXPECTATIONS}\n{body}"))
        .expect("supplied ButtonHeader public-query behavior");
}

#[test]
fn button_header_unsupplied_ids_return_nil_without_id_range_defaults() {
    let env = WowLuaEnv::new().expect("fresh widget environment");
    assert_query_behavior(
        &env,
        r#"
        for _, id in ipairs({760041, 760042, 1000000, 2000001}) do
            assert(query(id) == nil, 'unsupplied widget must return nil')
        end
        "#,
    );
}

#[test]
fn button_header_preserves_all_seventeen_fields_and_five_button_fields_in_order() {
    let env = WowLuaEnv::new().expect("fresh widget environment");
    supply_record(&env, FIRST_ID, first_record());
    assert_query_behavior(
        &env,
        r#"
        local info = query(760041)
        assertSameData(info, expectedNorth, 'ButtonHeader')
        local order = {}
        for index, button in ipairs(info.buttons) do
            order[index] = button.spellID
        end
        assertSameData(order, {190001, 190002}, 'consumer sequence')
        "#,
    );
}

#[test]
fn button_header_preserves_a_supplied_empty_button_array() {
    let env = WowLuaEnv::new().expect("fresh widget environment");
    let mut record = first_record();
    record.buttons.clear();
    record.tooltip.clear();
    record.texture_kit.clear();
    supply_record(&env, EMPTY_ID, record);
    assert_query_behavior(
        &env,
        r#"
        expectedNorth.buttons = {}
        expectedNorth.tooltip = ''
        expectedNorth.textureKit = ''
        local info = query(760044)
        assertSameData(info, expectedNorth, 'empty-button record')
        assert(type(info.buttons) == 'table' and next(info.buttons) == nil)
        local consumed = 0
        for _ in ipairs(info.buttons) do consumed = consumed + 1 end
        assert(consumed == 0, 'empty array yields no button consumer entries')
        "#,
    );
}

#[test]
fn button_header_selects_the_exact_supplied_id_without_cross_record_data() {
    let env = WowLuaEnv::new().expect("fresh widget environment");
    supply_record(&env, FIRST_ID, first_record());
    supply_record(&env, SECOND_ID, second_record());
    assert_query_behavior(
        &env,
        r#"
        assertSameData(query(760042), expectedSouth, 'south')
        assertSameData(query(760041), expectedNorth, 'north')
        assert(query(760043) == nil, 'adjacent unsupplied ID stays absent')
        "#,
    );
}

#[test]
fn button_header_returns_fresh_outer_array_and_button_tables() {
    let env = WowLuaEnv::new().expect("fresh widget environment");
    supply_record(&env, FIRST_ID, first_record());
    assert_query_behavior(
        &env,
        r#"
        local first, second = query(760041), query(760041)
        assert(not rawequal(first, second), 'fresh outer snapshot')
        assert(not rawequal(first.buttons, second.buttons), 'fresh button array')
        for index, button in ipairs(first.buttons) do
            assert(not rawequal(button, second.buttons[index]), 'fresh nested record')
        end
        assert(not rawequal(first.buttons[1], first.buttons[2]), 'distinct buttons')
        assertSameData(first, expectedNorth, 'first snapshot')
        assertSameData(second, expectedNorth, 'second snapshot')
        "#,
    );
}

#[test]
fn button_header_lua_mutation_cannot_change_supplied_state_or_other_snapshots() {
    let env = WowLuaEnv::new().expect("fresh widget environment");
    let supplied = first_record();
    supply_record(&env, FIRST_ID, supplied.clone());
    supply_record(&env, SECOND_ID, second_record());
    assert_query_behavior(
        &env,
        r#"
        local first, retained = query(760041), query(760041)
        first.headerText = 'caller changed header'
        first.hasTimer = false
        first.buttons[1].spellID = 999999
        first.buttons[1].tooltip = 'caller changed button'
        first.buttons[2] = nil
        first.buttons[3] = {spellID = 888888}
        assertSameData(retained, expectedNorth, 'retained snapshot')
        assertSameData(query(760041), expectedNorth, 'later snapshot')
        assertSameData(query(760042), expectedSouth, 'other supplied widget')
        "#,
    );
    assert_eq!(
        env.state()
            .borrow()
            .button_header_inputs
            .widgets
            .get(&WidgetId::from_number(FIRST_ID)),
        Some(&supplied),
        "Lua mutation leaves supplied Rust state intact"
    );
}

#[test]
fn button_header_supplied_records_are_isolated_between_live_environments() {
    let north = WowLuaEnv::new().expect("north environment");
    let south = WowLuaEnv::new().expect("south environment");
    let absent = WowLuaEnv::new().expect("unsupplied environment");
    supply_record(&north, FIRST_ID, first_record());
    supply_record(&south, FIRST_ID, second_record());
    assert_query_behavior(
        &north,
        "assertSameData(query(760041), expectedNorth, 'north env')",
    );
    assert_query_behavior(
        &south,
        "assertSameData(query(760041), expectedSouth, 'south env')",
    );
    assert_query_behavior(
        &absent,
        "assert(query(760041) == nil, 'no global record leakage')",
    );
    assert_query_behavior(
        &north,
        "assertSameData(query(760041), expectedNorth, 'north retained')",
    );
}

fn load_cached_lua(env: &WowLuaEnv, path: &Path) {
    let source = std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("read cached {}: {error}", path.display()));
    env.exec_named(&source, &path.display().to_string())
        .unwrap_or_else(|error| panic!("load cached {}: {error}", path.display()));
}

fn load_bounded_cached_button_header_provider(env: &WowLuaEnv) {
    let addons = wow_ui_sim::paths::default_blizzard_ui_addons_path()
        .expect("active Retail/PTR profile cache required; no alternate source");
    let widgets = addons.join("Blizzard_UIWidgets");
    // Exact existing draft sequence, no guessed additional prerequisites.
    // GREEN-side dependency sufficiency remains pending runtime evidence.
    load_cached_lua(env, &widgets.join("Blizzard_UIWidgetManager.lua"));
    env.exec("UIWidgetManager = CreateFromMixins(UIWidgetManagerMixin); UIWidgetManager:OnLoad()")
        .expect("initialize actual cached manager registration state");
    load_cached_lua(env, &widgets.join("Blizzard_UIWidgetTemplateBase.lua"));
    load_cached_lua(
        env,
        &widgets.join("Blizzard_UIWidgetTemplateButtonHeader.lua"),
    );
}

#[test]
fn button_header_actual_cached_provider_filters_hidden_but_preserves_visible_data() {
    let env = WowLuaEnv::new().expect("bounded provider environment");
    supply_record(&env, FIRST_ID, first_record());
    let mut hidden = second_record();
    hidden.shown_state = 0; // Hidden
    supply_record(&env, HIDDEN_ID, hidden);
    let mut empty = first_record();
    empty.buttons.clear();
    supply_record(&env, EMPTY_ID, empty);
    // Reach missing real query before any unverified cache loading boundary.
    assert_query_behavior(
        &env,
        "assertSameData(query(760041), expectedNorth, 'query before consumer')",
    );
    load_bounded_cached_button_header_provider(&env);
    assert_query_behavior(
        &env,
        r#"
        local registered = UIWidgetManager:GetWidgetTypeInfo(Enum.UIWidgetVisualizationType.ButtonHeader)
        assert(registered and type(registered.visInfoDataFunction) == 'function')
        local provider = registered.visInfoDataFunction
        local rawHidden = query(760043)
        assert(type(rawHidden) == 'table' and rawHidden.shownState == Enum.WidgetShownState.Hidden)
        expectedSouth.shownState = 0
        assertSameData(rawHidden, expectedSouth, 'raw hidden supplied record')
        assert(provider(760043) == nil, 'actual cached provider rejects hidden')
        assert(provider(760042) == nil, 'actual cached provider rejects unknown')
        assertSameData(provider(760041), expectedNorth, 'actual cached visible provider')
        expectedNorth.buttons = {}
        assertSameData(provider(760044), expectedNorth, 'actual cached empty visible provider')
        "#,
    );
}

#[cfg(feature = "retail-12-1-0")]
fn install_host_secret_ids(env: &WowLuaEnv) {
    let loader = env.loader_env();
    let mut lua = loader.rilua_mut();
    rilua::table_security::register_table_security(&mut lua).expect("VM security helpers");
    for (name, number) in [
        ("ButtonHeaderSupplied", FIRST_ID),
        ("ButtonHeaderAbsent", SECOND_ID),
    ] {
        let value = wrap_host_secret_number(lua.state_mut(), number);
        lua.state_mut().push(value);
        let inserted = lua.set_global_val(name, value);
        lua.state_mut().pop();
        inserted.expect("root actual secret NUMBER");
    }
}

#[test]
#[cfg(feature = "retail-12-1-0")]
fn button_header_documented_secret_arguments_require_an_untainted_caller() {
    let env = WowLuaEnv::new().expect("secret-policy widget environment");
    supply_record(&env, FIRST_ID, first_record());
    // Query publication first; absent query cannot masquerade as authentication RED.
    assert_query_behavior(&env, "");
    install_host_secret_ids(&env);
    assert_query_behavior(
        &env,
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil)
        local supplied, absent = ButtonHeaderSupplied, ButtonHeaderAbsent
        assert(issecretvalue(supplied) and issecretvalue(absent), 'actual VM wrappers')
        assertSameData(query(supplied), expectedNorth, 'untainted secret supplied ID')
        assert(query(absent) == nil, 'untainted secret unknown ID')
        local function addon()
            assert(debug.getstacktaint() == 'ButtonHeaderDraftProbe')
            for _, id in ipairs({supplied, absent}) do
                local ok, message = pcall(query, id)
                assert(not ok and type(message) == 'string', 'tainted secret input denied')
                -- Existing pinned rilua unwrap_secret guard, not an invented API message.
                assert(string.find(message, 'untainted caller', 1, true),
                    'authentication must precede lookup: ' .. tostring(message))
                assert(issecretvalue(id), 'query preserves wrapped input')
                assert(not pcall(secretunwrap, id), 'no tainted declassification')
                assert(debug.getstacktaint() == 'ButtonHeaderDraftProbe', 'caller taint preserved')
            end
            assertSameData(query(760041), expectedNorth, 'tainted ordinary numeric ID')
            assert(query(760042) == nil)
        end
        debug.setobjecttaint(addon, 'ButtonHeaderDraftProbe')
        addon()
        assert(debug.getstacktaint() == nil, 'outer caller restored')
        assert(issecretvalue(supplied) and issecretvalue(absent))
        assert(secretunwrap(supplied) == 760041 and secretunwrap(absent) == 760042)
        assertSameData(query(supplied), expectedNorth, 'secret query after restoration')
        assert(query(absent) == nil)
        "#,
    );
}
