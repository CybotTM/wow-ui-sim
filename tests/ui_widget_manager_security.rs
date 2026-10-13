//! Actual host-secret wrappers and caller-taint policy over supplied widget state.
//! Simulator contract; no native-parity claim.
#![cfg(all(
    any(feature = "client-retail", feature = "client-ptr"),
    feature = "retail-12-1-0"
))]

use rilua::LuaApiMut;
use rilua::table_security::wrap_host_secret_number;
use wow_ui_sim::c_api::ui_widget_manager::{
    IconAndTextRecord, WidgetId, WidgetSetEntry, WidgetSetRecord,
};
use wow_ui_sim::lua_api::WowLuaEnv;

const SET_ID: f64 = 417.0;
const ICON_ID: f64 = 812.0;

fn supplied_set() -> WidgetSetRecord {
    WidgetSetRecord {
        layout_direction: 1,
        vertical_padding: 6.25,
        widgets: vec![WidgetSetEntry {
            widget_id: ICON_ID,
            widget_type: 2,
            unit_token: Some("player".into()),
        }],
    }
}

fn supplied_icon() -> IconAndTextRecord {
    IconAndTextRecord {
        state: 1,
        text: "Supplied icon text".into(),
        tooltip: "Supplied tooltip".into(),
        dynamic_tooltip: "Supplied dynamic tooltip".into(),
        tooltip_loc: 7,
        shift_text_type: 1,
        widget_size_setting: 23.5,
        texture_kit: "supplied-content-kit".into(),
        frame_texture_kit: "supplied-frame-kit".into(),
        has_timer: true,
        order_index: 37.25,
        widget_tag: "supplied-tag".into(),
        in_anim_type: 1,
        out_anim_type: 0,
        widget_scale: 7,
        layout_direction: 2,
        model_scene_layer: 1,
        scripted_animation_effect_id: 91007.0,
    }
}

fn supply_inputs(env: &WowLuaEnv, set: WidgetSetRecord, icon: IconAndTextRecord) {
    let state = env.state();
    let mut state = state.borrow_mut();
    state
        .ui_widget_inputs
        .sets
        .insert(WidgetId::from_number(SET_ID), set);
    state
        .ui_widget_inputs
        .icon_and_text
        .insert(WidgetId::from_number(ICON_ID), icon);
}

fn install_rooted_host_secret_ids(env: &WowLuaEnv) {
    let loader = env.loader_env();
    let mut lua = loader.rilua_mut();
    rilua::table_security::register_table_security(&mut lua).expect("VM security helpers");
    for (name, number) in [
        ("WidgetDraftSet", SET_ID),
        ("WidgetDraftIcon", ICON_ID),
        ("WidgetDraftAbsentSet", 418.0),
        ("WidgetDraftAbsentIcon", 813.0),
    ] {
        let value = wrap_host_secret_number(lua.state_mut(), number);
        lua.state_mut().push(value);
        let inserted = lua.set_global_val(name, value);
        lua.state_mut().pop();
        inserted.expect("root actual secret NUMBER");
    }
}

#[test]
fn widget_queries_deny_tainted_secrets_and_recover_public_isolated_snapshots() {
    let env = WowLuaEnv::new().expect("widget security environment");
    let set = supplied_set();
    let icon = supplied_icon();
    supply_inputs(&env, set.clone(), icon.clone());
    // Publication precedes secrets: missing registration cannot masquerade as denial.
    env.exec(
        r#"
        for _, name in ipairs({ 'GetWidgetSetInfo', 'GetAllWidgetsBySetID',
            'GetIconAndTextWidgetVisualizationInfo' }) do
            assert(type(C_UIWidgetManager[name]) == 'function', name .. ' unpublished')
        end
        "#,
    )
    .expect("three modeled queries published");
    install_rooted_host_secret_ids(&env);
    env.exec(
        r#"
        debug.settaintmode(true)
        assert(debug.getstacktaint() == nil)
        local metadata = C_UIWidgetManager.GetWidgetSetInfo
        local listing = C_UIWidgetManager.GetAllWidgetsBySetID
        local visualization = C_UIWidgetManager.GetIconAndTextWidgetVisualizationInfo
        local setID, iconID = WidgetDraftSet, WidgetDraftIcon
        local absentSet, absentIcon = WidgetDraftAbsentSet, WidgetDraftAbsentIcon
        local expectedSet = {layoutDirection = 1, verticalPadding = 6.25}
        local expectedList = {
            {widgetID = 812, widgetSetID = 417, widgetType = 2, unitToken = 'player'},
        }
        local expectedIcon = {
            state = 1, text = 'Supplied icon text', tooltip = 'Supplied tooltip',
            dynamicTooltip = 'Supplied dynamic tooltip', tooltipLoc = 7,
            shiftTextType = 1, widgetSizeSetting = 23.5,
            textureKit = 'supplied-content-kit', frameTextureKit = 'supplied-frame-kit',
            hasTimer = true, orderIndex = 37.25, widgetTag = 'supplied-tag',
            inAnimType = 1, outAnimType = 0, widgetScale = 7, layoutDirection = 2,
            modelSceneLayer = 1, scriptedAnimationEffectID = 91007,
        }
        local function assertPublicData(actual, expected)
            assert(not issecretvalue(actual), 'public snapshot value')
            assert(type(actual) == type(expected), 'snapshot Lua type')
            if type(expected) ~= 'table' then
                assert(actual == expected, 'supplied value preserved')
                return
            end
            for key, value in pairs(expected) do assertPublicData(actual[key], value) end
            for key in pairs(actual) do
                assert(not issecretvalue(key), 'public snapshot key')
                assert(expected[key] ~= nil, 'no extra projected fields')
            end
        end
        for _, id in ipairs({setID, iconID, absentSet, absentIcon}) do
            assert(issecretvalue(id), 'actual host secret NUMBER wrapper')
        end
        local retainedSet, retainedList, retainedIcon =
            metadata(setID), listing(setID), visualization(iconID)
        assertPublicData(retainedSet, expectedSet)
        assertPublicData(retainedList, expectedList)
        assertPublicData(retainedIcon, expectedIcon)
        local cases = {
            {query = metadata, supplied = setID, absent = absentSet, publicID = 417,
             expected = expectedSet},
            {query = listing, supplied = setID, absent = absentSet, publicID = 417,
             expected = expectedList},
            {query = visualization, supplied = iconID, absent = absentIcon, publicID = 812,
             expected = expectedIcon},
        }
        local function addon()
            assert(debug.getstacktaint() == 'WidgetSecurityDraftProbe')
            for _, case in ipairs(cases) do
                for _, id in ipairs({case.supplied, case.absent}) do
                    local ok, message = pcall(case.query, id)
                    assert(not ok and type(message) == 'string', 'secret input denied')
                    assert(string.find(message, 'untainted caller', 1, true),
                        'authentication precedes supplied/absent lookup: ' .. tostring(message))
                    assert(issecretvalue(id), 'wrapped input retained')
                    assert(not pcall(secretunwrap, id), 'tainted declassification denied')
                    assert(debug.getstacktaint() == 'WidgetSecurityDraftProbe')
                end
                assertPublicData(case.query(case.publicID), case.expected)
            end
            assertPublicData(retainedSet, expectedSet)
            assertPublicData(retainedList, expectedList)
            assertPublicData(retainedIcon, expectedIcon)
        end
        debug.setobjecttaint(addon, 'WidgetSecurityDraftProbe')
        addon()
        assert(debug.getstacktaint() == nil, 'outer caller restored')
        for _, case in ipairs(cases) do
            assert(issecretvalue(case.supplied) and issecretvalue(case.absent))
            assert(secretunwrap(case.supplied) == case.publicID)
            assertPublicData(case.query(case.supplied), case.expected)
        end
        assert(secretunwrap(absentSet) == 418 and secretunwrap(absentIcon) == 813)
        local changedSet, changedList, changedIcon =
            metadata(setID), listing(setID), visualization(iconID)
        assert(not rawequal(changedSet, retainedSet), 'fresh metadata')
        assert(not rawequal(changedList, retainedList), 'fresh list')
        assert(not rawequal(changedList[1], retainedList[1]), 'fresh nested list entry')
        assert(not rawequal(changedIcon, retainedIcon), 'fresh visualization')
        changedSet.verticalPadding = -1
        changedList[1].widgetID = 999
        changedList[1].unitToken = 'target'
        changedList[2] = {widgetID = 999}
        changedIcon.text = 'caller mutation'
        assertPublicData(retainedSet, expectedSet)
        assertPublicData(retainedList, expectedList)
        assertPublicData(retainedIcon, expectedIcon)
        assertPublicData(metadata(417), expectedSet)
        assertPublicData(listing(417), expectedList)
        assertPublicData(visualization(812), expectedIcon)
        "#,
    )
    .expect("three-query secret denial, public recovery and snapshot isolation");
    {
        let state = env.state();
        let state = state.borrow();
        assert_eq!(
            state
                .ui_widget_inputs
                .sets
                .get(&WidgetId::from_number(SET_ID)),
            Some(&set)
        );
        assert_eq!(
            state
                .ui_widget_inputs
                .icon_and_text
                .get(&WidgetId::from_number(ICON_ID)),
            Some(&icon)
        );
    }
    let other = WowLuaEnv::new().expect("second live environment");
    let mut other_set = set;
    other_set.vertical_padding = 19.5;
    other_set.widgets[0].unit_token = None;
    let mut other_icon = icon;
    other_icon.text = "Other environment".into();
    supply_inputs(&other, other_set, other_icon);
    other.exec(
        r#"
        assert(C_UIWidgetManager.GetWidgetSetInfo(417).verticalPadding == 19.5)
        local row = C_UIWidgetManager.GetAllWidgetsBySetID(417)[1]
        assert(row.widgetID == 812 and row.widgetSetID == 417 and row.unitToken == nil)
        assert(C_UIWidgetManager.GetIconAndTextWidgetVisualizationInfo(812).text == 'Other environment')
        "#,
    )
    .expect("same IDs select second environment inputs");
    env.exec(
        r#"
        assert(C_UIWidgetManager.GetWidgetSetInfo(417).verticalPadding == 6.25)
        assert(C_UIWidgetManager.GetAllWidgetsBySetID(417)[1].unitToken == 'player')
        assert(C_UIWidgetManager.GetIconAndTextWidgetVisualizationInfo(812).text == 'Supplied icon text')
        "#,
    )
    .expect("second environment does not replace first environment inputs");
}
