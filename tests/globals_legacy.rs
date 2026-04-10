//! Tests for globals_legacy.rs: print, ipairs, getmetatable overrides.

use wow_ui_sim::iced_app::build_quad_batch_for_registry_with_quest_blobs;
use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn build_strata_buckets(env: &WowLuaEnv) -> Vec<Vec<u64>> {
    let mut state = env.state().borrow_mut();
    state.ensure_layout_rects();
    let _ = state.get_strata_buckets();
    state.strata_buckets.as_ref().unwrap().clone()
}

fn build_quads(env: &WowLuaEnv) -> wow_ui_sim::render::QuadBatch {
    let buckets = build_strata_buckets(env);
    let state = env.state().borrow();
    build_quad_batch_for_registry_with_quest_blobs(
        &state.widgets,
        (1024.0, 768.0),
        None,
        None,
        None,
        None,
        None,
        None,
        Some(&state.quest_blobs),
        &buckets,
    )
}

// ============================================================================
// print override
// ============================================================================

#[test]
fn test_print_nil() {
    let env = env();
    env.exec("print(nil)").unwrap();
    let output = &env.state().borrow().console_output;
    assert_eq!(output.last().unwrap(), "nil");
}

#[test]
fn test_print_boolean() {
    let env = env();
    env.exec("print(true, false)").unwrap();
    let output = &env.state().borrow().console_output;
    assert_eq!(output.last().unwrap(), "true\tfalse");
}

#[test]
fn test_print_numbers() {
    let env = env();
    env.exec("print(42, 3.14)").unwrap();
    let output = &env.state().borrow().console_output;
    assert_eq!(output.last().unwrap(), "42\t3.14");
}

#[test]
fn test_print_string() {
    let env = env();
    env.exec("print('hello world')").unwrap();
    let output = &env.state().borrow().console_output;
    assert_eq!(output.last().unwrap(), "hello world");
}

#[test]
fn test_print_table() {
    let env = env();
    env.exec("print({})").unwrap();
    let output = &env.state().borrow().console_output;
    assert_eq!(output.last().unwrap(), "table");
}

#[test]
fn test_print_function() {
    let env = env();
    env.exec("print(print)").unwrap();
    let output = &env.state().borrow().console_output;
    assert_eq!(output.last().unwrap(), "function");
}

#[test]
fn test_print_mixed_args_tab_separated() {
    let env = env();
    env.exec("print(1, 'two', true, nil)").unwrap();
    let output = &env.state().borrow().console_output;
    assert_eq!(output.last().unwrap(), "1\ttwo\ttrue\tnil");
}

#[test]
fn test_print_no_args() {
    let env = env();
    env.exec("print()").unwrap();
    let output = &env.state().borrow().console_output;
    assert_eq!(output.last().unwrap(), "");
}

#[test]
fn test_print_accumulates_in_console_buffer() {
    let env = env();
    env.exec("print('first')").unwrap();
    env.exec("print('second')").unwrap();
    let output = &env.state().borrow().console_output;
    assert_eq!(output.len(), 2);
    assert_eq!(output[0], "first");
    assert_eq!(output[1], "second");
}

// ============================================================================
// ipairs override (with frame userdata support)
// ============================================================================

#[test]
fn test_ipairs_table_still_works() {
    let env = env();
    let total: i32 = env
        .eval(
            r#"
            local sum = 0
            for i, v in ipairs({10, 20, 30}) do
                sum = sum + v
            end
            return sum
            "#,
        )
        .unwrap();
    assert_eq!(total, 60);
}

#[test]
fn test_ipairs_empty_table() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local n = 0
            for _ in ipairs({}) do n = n + 1 end
            return n
            "#,
        )
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn test_ipairs_frame_children() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "TestIpairsParent")
            CreateFrame("Frame", "TestIpairsChild1", parent)
            CreateFrame("Frame", "TestIpairsChild2", parent)
            local n = 0
            for i, child in ipairs(parent) do
                n = n + 1
            end
            return n
            "#,
        )
        .unwrap();
    assert_eq!(count, 2);
}

#[test]
fn test_ipairs_frame_children_index_starts_at_one() {
    let env = env();
    let first_idx: i32 = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "TestIpairsIdx")
            CreateFrame("Frame", "TestIpairsIdxChild", parent)
            local idx
            for i, child in ipairs(parent) do
                idx = i
                break
            end
            return idx
            "#,
        )
        .unwrap();
    assert_eq!(first_idx, 1);
}

// ============================================================================
// getmetatable override (frame userdata metatable)
// ============================================================================

#[test]
fn test_getmetatable_table_works() {
    let env = env();
    let has_mt: bool = env
        .eval(
            r#"
            local t = setmetatable({}, {__index = function() return 42 end})
            return getmetatable(t) ~= nil
            "#,
        )
        .unwrap();
    assert!(has_mt);
}

#[test]
fn test_getmetatable_frame_returns_table() {
    let env = env();
    let is_table: bool = env
        .eval(
            r#"
            local f = CreateFrame("Frame", "TestGetMTFrame")
            local mt = getmetatable(f)
            return type(mt) == "table"
            "#,
        )
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_getmetatable_frame_has_index_table() {
    let env = env();
    let is_table: bool = env
        .eval(
            r#"
            local f = CreateFrame("Frame", "TestGetMTIndex")
            local mt = getmetatable(f)
            return type(mt.__index) == "table"
            "#,
        )
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_getmetatable_frame_index_has_methods() {
    let env = env();
    let result: (bool, bool, bool) = env
        .eval(
            r#"
            local f = CreateFrame("Frame", "TestGetMTMethods")
            local mt = getmetatable(f)
            local idx = mt.__index
            return type(idx.GetName) == "function",
                   type(idx.Show) == "function",
                   type(idx.SetPoint) == "function"
            "#,
        )
        .unwrap();
    assert!(result.0, "GetName should be a function");
    assert!(result.1, "Show should be a function");
    assert!(result.2, "SetPoint should be a function");
}

#[test]
fn test_getmetatable_frame_index_iterable_with_pairs() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local f = CreateFrame("Frame", "TestGetMTIterable")
            local mt = getmetatable(f)
            local n = 0
            for name, func in pairs(mt.__index) do
                n = n + 1
            end
            return n
            "#,
        )
        .unwrap();
    // Should have many methods
    assert!(
        count > 50,
        "Expected many methods in __index, got {}",
        count
    );
}

#[test]
fn test_frame_runtime_lookup_filters_wrong_type_methods() {
    let env = env();
    let result: (bool, bool, bool, bool) = env
        .eval(
            r#"
            local button = CreateFrame("Button", "MethodFilterButton")
            local scroll = CreateFrame("ScrollFrame", "MethodFilterScroll")
            return button.GetScrollChild == nil,
                   button.GetVerticalScroll == nil,
                   type(scroll.GetScrollChild) == "function",
                   type(scroll.GetVerticalScroll) == "function"
            "#,
        )
        .unwrap();
    assert!(
        result.0,
        "Button should not expose ScrollFrame GetScrollChild at runtime"
    );
    assert!(
        result.1,
        "Button should not expose ScrollFrame GetVerticalScroll at runtime"
    );
    assert!(
        result.2,
        "ScrollFrame should still expose GetScrollChild at runtime"
    );
    assert!(
        result.3,
        "ScrollFrame should still expose GetVerticalScroll at runtime"
    );
}

#[test]
fn test_fontstring_runtime_lookup_hides_extra_title_methods() {
    let env = env();
    let result: (bool, bool, bool) = env
        .eval(
            r#"
            local parent = CreateFrame("Frame", "FontStringMethodParent")
            local text = parent:CreateFontString("FontStringMethodText")
            return text.GetTitle == nil,
                   text.SetTitle == nil,
                   type(text.SetText) == "function"
            "#,
        )
        .unwrap();
    assert!(result.0, "FontString should not expose GetTitle");
    assert!(result.1, "FontString should not expose SetTitle");
    assert!(result.2, "FontString should still expose SetText");
}

#[test]
fn test_statusbar_runtime_lookup_hides_extra_methods() {
    let env = env();
    let result: (bool, bool, bool) = env
        .eval(
            r#"
            local sb = CreateFrame("StatusBar", "StatusBarMethodFilter")
            return sb.GetStatusBarDesaturated == nil,
                   sb.SetStatusBarAtlas == nil,
                   type(sb.SetStatusBarTexture) == "function"
            "#,
        )
        .unwrap();
    assert!(
        result.0,
        "StatusBar should not expose GetStatusBarDesaturated"
    );
    assert!(result.1, "StatusBar should not expose SetStatusBarAtlas");
    assert!(
        result.2,
        "StatusBar should still expose supported texture methods"
    );
}

#[test]
fn test_getmetatable_nil_returns_nil() {
    let env = env();
    let is_nil: bool = env.eval("return getmetatable(nil) == nil").unwrap();
    assert!(is_nil);
}

#[test]
fn test_getmetatable_string_has_metatable() {
    let env = env();
    // Lua strings have a metatable with __index = string library
    let is_table: bool = env
        .eval("return type(getmetatable('')) == 'table'")
        .unwrap();
    assert!(is_table);
}

// ============================================================================
// CreateFrame exists and works (delegated to sub-module)
// ============================================================================

#[test]
fn test_create_frame_exists() {
    let env = env();
    let is_func: bool = env.eval("return type(CreateFrame) == 'function'").unwrap();
    assert!(is_func);
}

// ============================================================================
// Sub-module registrations are in place
// ============================================================================

#[test]
fn test_submodule_apis_registered() {
    let env = env();
    // Spot-check a few functions/namespaces from sub-modules
    for name in &[
        "GetLocale",    // locale_api
        "GetNumAddOns", // addon_api
        "UnitName",     // unit_api
        "Mixin",        // mixin_api
        "strsplit",     // utility_api
        "GetCVar",      // cvar_api
        "CreateFont",   // font_api
    ] {
        let is_func: bool = env
            .eval(&format!("return type({}) == 'function'", name))
            .unwrap();
        assert!(is_func, "{} should be a function", name);
    }
}

#[test]
fn test_submodule_namespaces_registered() {
    let env = env();
    for name in &[
        "C_Timer",
        "C_Map",
        "C_QuestLog",
        "C_MountJournal",
        "C_Item",
        "Enum",
        "Settings",
    ] {
        let is_table: bool = env
            .eval(&format!("return type({}) == 'table'", name))
            .unwrap();
        assert!(is_table, "{} should be a table", name);
    }
}

// ============================================================================
// UI strings registered
// ============================================================================

#[test]
fn test_ui_strings_registered() {
    let env = env();
    // Some well-known UI string constants
    let is_string: bool = env.eval("return type(OKAY) == 'string'").unwrap();
    assert!(is_string);
}

// ============================================================================
// string.format error messages
// ============================================================================

#[test]
fn test_string_format_missing_string_arg_error_message() {
    let env = env();
    let err: String = env
        .eval("local ok, err = pcall(string.format, '%s'); return err")
        .unwrap();
    assert_eq!(
        err, "bad argument #2 to '?' (string expected, got no value)",
        "got: {}",
        err
    );
}

#[test]
fn test_string_format_nil_string_arg_error_message() {
    let env = env();
    let err: String = env
        .eval("local ok, err = pcall(string.format, '%s', nil); return err")
        .unwrap();
    assert_eq!(
        err, "bad argument #2 to '?' (string expected, got nil)",
        "got: {}",
        err
    );
}

// ============================================================================
// Standard font objects created
// ============================================================================

#[test]
fn test_standard_font_objects_created() {
    let env = env();
    let exists: bool = env.eval("return GameFontNormal ~= nil").unwrap();
    assert!(exists, "GameFontNormal should exist");
}

// ============================================================================
// Quest log selection and description text
// ============================================================================

#[test]
fn test_quest_log_set_get_selected() {
    let env = env();
    let (before, after): (i32, i32) = env
        .eval(
            r#"
            local before = C_QuestLog.GetSelectedQuest()
            C_QuestLog.SetSelectedQuest(80000)
            return before, C_QuestLog.GetSelectedQuest()
        "#,
        )
        .unwrap();
    assert_eq!(before, 0, "initially no quest selected");
    assert_eq!(after, 80000, "SetSelectedQuest stores the ID");
}

#[test]
fn test_get_quest_log_quest_text_returns_description() {
    let env = env();
    let (desc, obj): (String, String) = env
        .eval(
            r#"
            C_QuestLog.SetSelectedQuest(80000)
            return GetQuestLogQuestText()
        "#,
        )
        .unwrap();
    assert!(
        desc.contains("Ironforge expedition"),
        "description should contain quest text, got: {desc}"
    );
    assert!(
        obj.contains("Ironforge Relics"),
        "objectives should contain objective text, got: {obj}"
    );
}

#[test]
fn test_get_quest_log_quest_text_no_selection() {
    let env = env();
    let (desc, obj): (String, String) = env
        .eval(
            r#"
            C_QuestLog.SetSelectedQuest(0)
            return GetQuestLogQuestText()
        "#,
        )
        .unwrap();
    assert_eq!(desc, "", "no quest selected → empty description");
    assert_eq!(obj, "", "no quest selected → empty objectives");
}

#[test]
fn test_get_quest_poi_blob_count_known_quest() {
    let env = env();
    let count: i32 = env.eval("return GetQuestPOIBlobCount(80000)").unwrap();
    assert_eq!(count, 1, "Quest 80000 should have 1 blob");
}

#[test]
fn test_get_quest_poi_blob_count_unknown_quest() {
    let env = env();
    let count: i32 = env.eval("return GetQuestPOIBlobCount(99999)").unwrap();
    assert_eq!(count, 0, "Unknown quest should have 0 blobs");
}

#[test]
fn test_draw_blob_stores_quest_id() {
    let env = env();

    env.exec(
        r#"
        local poi = CreateFrame("Frame", "TestPOI", UIParent)
        poi:SetMapID(2248)
        poi:DrawBlob(80000, true)
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let poi_id = state.widgets.get_id_by_name("TestPOI").unwrap();
    let blob = state.quest_blobs.get(&poi_id).unwrap();
    assert_eq!(blob.map_id, 2248);
    assert_eq!(blob.active_quests, vec![80000]);
}

#[test]
fn test_draw_blob_emits_fill_geometry_into_render_batch() {
    let env = env();

    env.exec(
        r#"
        local poi = CreateFrame("QuestPOIFrame", "RenderPOI", UIParent)
        poi:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -120)
        poi:SetSize(200, 180)
        poi:SetMapID(2248)
        poi:SetFillTexture("Interface/QuestBlobFill")
        poi:SetFillAlpha(0.75)
        poi:DrawBlob(80000, true)
    "#,
    )
    .unwrap();

    let batch = build_quads(&env);
    assert!(
        batch
            .texture_requests
            .iter()
            .any(|request| request.path == "Interface/QuestBlobFill"),
        "DrawBlob should enqueue the configured fill texture for rendering"
    );
    assert!(
        batch
            .texture_requests
            .iter()
            .filter(|request| request.path == "Interface/QuestBlobFill")
            .all(|request| request.vertex_count > 0),
        "DrawBlob should emit blob-specific geometry for the configured fill texture"
    );
}

#[test]
fn test_draw_blob_multiple_quests() {
    let env = env();

    env.exec(
        r#"
        local poi = CreateFrame("Frame", "TestPOI2", UIParent)
        poi:DrawBlob(80000, true)
        poi:DrawBlob(80001, true)
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let poi_id = state.widgets.get_id_by_name("TestPOI2").unwrap();
    let blob = state.quest_blobs.get(&poi_id).unwrap();
    assert_eq!(blob.active_quests, vec![80000, 80001]);
}

#[test]
fn test_draw_blob_no_duplicates() {
    let env = env();

    env.exec(
        r#"
        local poi = CreateFrame("Frame", "TestPOI3", UIParent)
        poi:DrawBlob(80000, true)
        poi:DrawBlob(80000, true)
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let poi_id = state.widgets.get_id_by_name("TestPOI3").unwrap();
    let blob = state.quest_blobs.get(&poi_id).unwrap();
    assert_eq!(
        blob.active_quests,
        vec![80000],
        "Should not duplicate quest IDs"
    );
}

#[test]
fn test_draw_none_clears_blobs() {
    let env = env();

    env.exec(
        r#"
        local poi = CreateFrame("Frame", "TestPOI4", UIParent)
        poi:DrawBlob(80000, true)
        poi:DrawBlob(80001, true)
        poi:DrawNone()
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let poi_id = state.widgets.get_id_by_name("TestPOI4").unwrap();
    let blob = state.quest_blobs.get(&poi_id).unwrap();
    assert!(
        blob.active_quests.is_empty(),
        "DrawNone should clear all blobs"
    );
}

#[test]
fn test_draw_none_clears_rendered_blob_geometry() {
    let env = env();

    env.exec(
        r#"
        local poi = CreateFrame("QuestPOIFrame", "RenderPOIClear", UIParent)
        poi:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -120)
        poi:SetSize(200, 180)
        poi:SetMapID(2248)
        poi:SetFillTexture("Interface/QuestBlobFill")
        poi:SetFillAlpha(0.75)
        poi:DrawBlob(80000, true)
        poi:DrawNone()
    "#,
    )
    .unwrap();

    let batch = build_quads(&env);
    assert!(
        !batch
            .texture_requests
            .iter()
            .any(|request| request.path == "Interface/QuestBlobFill"),
        "DrawNone should remove blob-specific fill geometry from the render batch"
    );
}

#[test]
fn test_quest_blob_render_inputs_persist_on_first_setter_use() {
    let env = env();

    env.exec(
        r#"
        local poi = CreateFrame("Frame", "TestPOIStyled", UIParent)
        poi:SetFillTexture("Interface\\WorldMap\\UI-QuestBlob-Inside")
        poi:SetBorderTexture("Interface\\WorldMap\\UI-QuestBlob-Outside")
        poi:SetFillAlpha(128)
        poi:SetBorderAlpha(192)
        poi:SetBorderScalar(1.0)
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let poi_id = state.widgets.get_id_by_name("TestPOIStyled").unwrap();
    let blob = state.quest_blobs.get(&poi_id).unwrap();
    assert!(blob.active_quests.is_empty());
    assert_eq!(
        blob.fill_texture.as_deref(),
        Some("Interface\\WorldMap\\UI-QuestBlob-Inside")
    );
    assert_eq!(
        blob.border_texture.as_deref(),
        Some("Interface\\WorldMap\\UI-QuestBlob-Outside")
    );
    assert_eq!(blob.fill_alpha, Some(128.0));
    assert_eq!(blob.border_alpha, Some(192.0));
    assert_eq!(blob.border_scalar, Some(1.0));
}

#[test]
fn test_set_map_id_and_get_map_id() {
    let env = env();

    env.exec(
        r#"
        local poi = CreateFrame("Frame", "TestPOI5", UIParent)
        poi:SetMapID(37)
    "#,
    )
    .unwrap();

    let map_id: i32 = env.eval("return TestPOI5:GetMapID()").unwrap();
    assert_eq!(map_id, 37);
}

#[test]
fn test_update_mouse_over_tooltip_hit() {
    let env = env();

    // Quest 80000 blob is on map 2248, centered around (0.45, 0.58)
    let quest_id: i32 = env
        .eval(
            r#"
        local poi = CreateFrame("Frame", "HitTestPOI", UIParent)
        poi:SetMapID(2248)
        poi:DrawBlob(80000, true)
        local qid, count = poi:UpdateMouseOverTooltip(0.45, 0.58)
        return qid or 0
    "#,
        )
        .unwrap();
    assert_eq!(quest_id, 80000, "Should hit quest 80000 blob");
}

#[test]
fn test_update_mouse_over_tooltip_miss() {
    let env = env();

    let is_nil: bool = env
        .eval(
            r#"
        local poi = CreateFrame("Frame", "MissPOI", UIParent)
        poi:SetMapID(2248)
        poi:DrawBlob(80000, true)
        local qid = poi:UpdateMouseOverTooltip(0.1, 0.1)
        return qid == nil
    "#,
        )
        .unwrap();
    assert!(is_nil, "Point outside blob should return nil");
}

#[test]
fn test_update_mouse_over_tooltip_no_blobs() {
    let env = env();

    let is_nil: bool = env
        .eval(
            r#"
        local poi = CreateFrame("Frame", "EmptyPOI", UIParent)
        local qid = poi:UpdateMouseOverTooltip(0.5, 0.5)
        return qid == nil
    "#,
        )
        .unwrap();
    assert!(is_nil, "No active blobs should return nil");
}

#[test]
fn test_get_tooltip_index_identity() {
    let env = env();

    let result: i32 = env
        .eval(
            r#"
        local poi = CreateFrame("Frame", "TooltipIdxPOI", UIParent)
        return poi:GetTooltipIndex(3)
    "#,
        )
        .unwrap();
    assert_eq!(result, 3, "GetTooltipIndex should return the input index");
}

#[test]
fn test_get_tooltip_index_first() {
    let env = env();

    let result: i32 = env
        .eval(
            r#"
        local poi = CreateFrame("Frame", "TooltipIdxPOI2", UIParent)
        return poi:GetTooltipIndex(1)
    "#,
        )
        .unwrap();
    assert_eq!(result, 1);
}

#[test]
fn test_get_cursor_position_reads_mouse_state() {
    let env = env();

    env.state().borrow_mut().mouse_position = Some((200.0, 300.0));

    let (x, y): (f64, f64) = env.eval("return GetCursorPosition()").unwrap();
    assert!((x - 200.0).abs() < 0.1, "x should be 200, got {x}");
    assert!((y - 300.0).abs() < 0.1, "y should be 300, got {y}");
}

#[test]
fn test_get_cursor_position_default_when_no_mouse() {
    let env = env();

    // mouse_position is None by default
    let (x, y): (f64, f64) = env.eval("return GetCursorPosition()").unwrap();
    assert!((x - 512.0).abs() < 0.1, "default x should be 512, got {x}");
    assert!((y - 384.0).abs() < 0.1, "default y should be 384, got {y}");
}

#[test]
fn test_get_cursor_position_updates_dynamically() {
    let env = env();

    env.state().borrow_mut().mouse_position = Some((100.0, 50.0));
    let (x1, _): (f64, f64) = env.eval("return GetCursorPosition()").unwrap();

    env.state().borrow_mut().mouse_position = Some((700.0, 500.0));
    let (x2, _): (f64, f64) = env.eval("return GetCursorPosition()").unwrap();

    assert!((x1 - 100.0).abs() < 0.1);
    assert!((x2 - 700.0).abs() < 0.1, "Should reflect updated position");
}
