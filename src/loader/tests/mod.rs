//! Tests for the addon loader.

mod account_store;
mod screen_selection;
mod xml_basics;

use super::addon::AddonContext;
use super::lua_file::load_lua_file;
use super::xml_file::load_xml_file;
use super::*;
use crate::lua_api::WowLuaEnv;

/// Test context holding environment and temp directory for cleanup.
struct TestCtx {
    env: WowLuaEnv,
    temp_dir: PathBuf,
}

impl TestCtx {
    /// Assert a Lua expression evaluates to true.
    fn assert_lua_true(&self, expr: &str, msg: &str) {
        let result: bool = self.env.eval(expr).unwrap();
        assert!(result, "{}", msg);
    }

    /// Assert a Lua expression returns the expected string.
    fn assert_lua_str(&self, expr: &str, expected: &str) {
        let result: String = self.env.eval(expr).unwrap();
        assert_eq!(result, expected);
    }

    /// Assert that a script handler is set on a frame.
    fn assert_script_set(&self, frame: &str, handler: &str) {
        let expr = format!("return {}:GetScript('{}') ~= nil", frame, handler);
        let msg = format!("{} should be set on {}", handler, frame);
        self.assert_lua_true(&expr, &msg);
    }
}

impl Drop for TestCtx {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.temp_dir);
    }
}

/// Create a test environment, write XML content, load it, return context.
fn load_test_xml(dir_suffix: &str, xml_content: &str) -> TestCtx {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join(format!("wow-sim-{}", dir_suffix));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let xml_path = temp_dir.join("test.xml");
    std::fs::write(&xml_path, xml_content).unwrap();

    let addon_table = env.create_addon_table().unwrap();
    let ctx = AddonContext {
        name: "TestAddon",
        table: addon_table,
        addon_root: &temp_dir,
        use_secure_env: false,
        taint: false,
    };
    load_xml_file(
        &env.loader_env(),
        &xml_path,
        &ctx,
        &mut LoadTiming::default(),
    )
    .unwrap();

    TestCtx { env, temp_dir }
}

/// Create a test environment, write a Lua file, load it, return context + addon table.
fn load_test_lua(dir_suffix: &str, lua_content: &str) -> (TestCtx, mlua::Table) {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join(format!("wow-sim-{}", dir_suffix));
    std::fs::create_dir_all(&temp_dir).unwrap();
    let lua_path = temp_dir.join("test.lua");
    std::fs::write(&lua_path, lua_content).unwrap();

    register_loading_test_addon(&env);

    let addon_table = env.create_addon_table().unwrap();
    let ctx = AddonContext {
        name: "TestAddon",
        table: addon_table.clone(),
        addon_root: &temp_dir,
        use_secure_env: false,
        taint: false,
    };
    load_lua_file(
        &env.loader_env(),
        &lua_path,
        &ctx,
        &mut LoadTiming::default(),
    )
    .unwrap();

    (TestCtx { env, temp_dir }, addon_table)
}

fn register_loading_test_addon(env: &WowLuaEnv) {
    env.register_addon(crate::lua_api::AddonInfo {
        folder_name: "TestAddon".to_string(),
        title: "TestAddon".to_string(),
        enabled: true,
        loaded: true,
        ..Default::default()
    });
    set_loading_addon_index(env, "TestAddon");
}

fn set_loading_addon_index(env: &WowLuaEnv, addon_name: &str) {
    let mut s = env.state().borrow_mut();
    let idx = s
        .addons
        .iter()
        .position(|a| a.folder_name == addon_name)
        .unwrap();
    s.loading_addon_index = Some(idx as u16);
}

#[test]
fn test_load_lua_file() {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join("wow-sim-test");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let lua_path = temp_dir.join("test.lua");
    std::fs::write(&lua_path, "TEST_VAR = 42").unwrap();

    let addon_table = env.create_addon_table().unwrap();
    let ctx = AddonContext {
        name: "TestAddon",
        table: addon_table,
        addon_root: &temp_dir,
        use_secure_env: false,
        taint: false,
    };
    load_lua_file(
        &env.loader_env(),
        &lua_path,
        &ctx,
        &mut LoadTiming::default(),
    )
    .unwrap();

    let value: i32 = env.eval("return TEST_VAR").unwrap();
    assert_eq!(value, 42);
    std::fs::remove_file(&lua_path).ok();
}

#[test]
fn test_local_function_closures() {
    let (_t, addon_table) = load_test_lua(
        "test-closures",
        r#"
            local _, addon = ...
            local function innerFunc(x) return x * 2 end
            local function outerFunc(x)
                if not innerFunc then error("innerFunc is nil!") end
                return innerFunc(x)
            end
            addon.result = outerFunc(21)
            function addon:CreateSomething() return outerFunc(10) end
        "#,
    );

    assert_eq!(addon_table.get::<i32>("result").unwrap(), 42);
    let create_something: mlua::Function = addon_table.get("CreateSomething").unwrap();
    assert_eq!(
        create_something.call::<i32>(addon_table.clone()).unwrap(),
        20
    );
}

/// Load multiple Lua files in sequence with a shared addon table.
fn load_test_lua_files(
    dir_suffix: &str,
    addon_name: &str,
    files: &[(&str, &str)],
) -> (TestCtx, mlua::Table) {
    let env = WowLuaEnv::new().unwrap();
    let temp_dir = std::env::temp_dir().join(format!("wow-sim-{}", dir_suffix));
    std::fs::create_dir_all(&temp_dir).unwrap();

    let addon_table = env.create_addon_table().unwrap();
    let ctx = AddonContext {
        name: addon_name,
        table: addon_table.clone(),
        addon_root: &temp_dir,
        use_secure_env: false,
        taint: false,
    };

    for (filename, content) in files {
        let path = temp_dir.join(filename);
        std::fs::write(&path, content).unwrap();
        load_lua_file(&env.loader_env(), &path, &ctx, &mut LoadTiming::default())
            .unwrap_or_else(|e| panic!("{} should load: {}", filename, e));
    }

    (TestCtx { env, temp_dir }, addon_table)
}

#[test]
fn test_multi_file_closures() {
    let (_t, addon_table) = load_test_lua_files(
        "test-multifile",
        "MultiFileTest",
        &[
            ("widgets.lua", MULTI_FILE_WIDGETS_LUA),
            ("button.lua", MULTI_FILE_BUTTON_LUA),
            ("addon.lua", MULTI_FILE_ADDON_LUA),
        ],
    );

    let test_button: mlua::Table = addon_table
        .get("testButton")
        .expect("testButton should exist");
    let result: String = test_button.get("result").expect("result should be set");
    assert!(
        result.starts_with("updated:"),
        "updateKeyDirection should have been called, got: {}",
        result
    );
}

const MULTI_FILE_WIDGETS_LUA: &str = r#"
    local _, addon = ...
    local function updateKeyDirection(self) return "updated: " .. tostring(self) end
    local function onCVarUpdate(self, cvar)
        if cvar == "TestCVar" then
            if not updateKeyDirection then error("updateKeyDirection is nil!") end
            self.result = updateKeyDirection(self)
        end
    end
    function addon:CreateButton(name)
        local button = { name = name }
        onCVarUpdate(button, "TestCVar")
        return button
    end
"#;

const MULTI_FILE_BUTTON_LUA: &str = r#"
    local _, addon = ...
    function addon:CreateExtraButton(name) return addon:CreateButton(name .. "_extra") end
"#;

const MULTI_FILE_ADDON_LUA: &str = r#"
    local _, addon = ...
    local button = addon:CreateExtraButton("test")
    addon.testButton = button
"#;

/// Test that GetAttribute supports multi-argument form (prefix, name, suffix)
/// and wildcard `*` prefix fallback, as required by SecureTemplates.lua.
#[test]
fn test_get_attribute_multi_arg_and_wildcard() {
    let t = load_test_xml(
        "test-getattr-multi",
        r#"<Ui>
            <Button name="TestSecureBtn" parent="UIParent">
                <Size x="100" y="30"/>
                <Anchors><Anchor point="CENTER"/></Anchors>
            </Button>
        </Ui>"#,
    );

    // Set attributes like SecureTemplates does
    t.env
        .eval::<()>(
            r#"
        TestSecureBtn:SetAttribute("*type1", "target")
        TestSecureBtn:SetAttribute("unit", "party1")
        TestSecureBtn:SetAttribute("type2", "menu")
    "#,
        )
        .unwrap();

    // Single-arg GetAttribute still works
    t.assert_lua_str(r#"return TestSecureBtn:GetAttribute("unit")"#, "party1");
    t.assert_lua_str(r#"return TestSecureBtn:GetAttribute("type2")"#, "menu");

    // Multi-arg form: GetAttribute(prefix, name, suffix) → concatenates
    t.assert_lua_str(
        r#"return TestSecureBtn:GetAttribute("", "type", "2")"#,
        "menu",
    );

    // Multi-arg with wildcard fallback: "type1" not found → falls back to "*type1"
    t.assert_lua_str(
        r#"return TestSecureBtn:GetAttribute("", "type", "1")"#,
        "target",
    );

    // Multi-arg with modifier prefix: "shift-type1" not found → "*type1"
    t.assert_lua_str(
        r#"return TestSecureBtn:GetAttribute("shift-", "type", "1")"#,
        "target",
    );

    // Unit attribute via multi-arg: bare name fallback (key 5)
    // SetAttribute("unit", "party1") found via GetAttribute("", "unit", "1") → tries
    // "unit1", "*unit1", "unit*", "*unit*", then "unit" (bare name) → found
    t.assert_lua_str(
        r#"return TestSecureBtn:GetAttribute("", "unit", "1")"#,
        "party1",
    );

    // Also works with empty suffix
    t.assert_lua_str(
        r#"return TestSecureBtn:GetAttribute("", "unit", "")"#,
        "party1",
    );

    // Non-existent attribute returns nil
    t.assert_lua_true(
        r#"return TestSecureBtn:GetAttribute("", "nosuch", "1") == nil"#,
        "non-existent multi-arg attribute should be nil",
    );
}

#[test]
fn test_set_get_hit_rect_insets() {
    let (t, _) = load_test_lua(
        "test-hit-rect-insets",
        r#"
        local f = CreateFrame("Frame", "HitRectTestFrame", UIParent)
        f:SetSize(200, 100)
        f:SetPoint("CENTER")

        -- Default insets should be zero
        local l, r, top, b = f:GetHitRectInsets()
        assert(l == 0 and r == 0 and top == 0 and b == 0,
            "default insets should be 0,0,0,0 but got " .. l .. "," .. r .. "," .. top .. "," .. b)

        -- Set and verify
        f:SetHitRectInsets(10, 20, 5, 15)
        local l2, r2, t2, b2 = f:GetHitRectInsets()
        assert(l2 == 10, "left inset should be 10, got " .. l2)
        assert(r2 == 20, "right inset should be 20, got " .. r2)
        assert(t2 == 5, "top inset should be 5, got " .. t2)
        assert(b2 == 15, "bottom inset should be 15, got " .. b2)

        -- Overwrite with new values
        f:SetHitRectInsets(0, 0, 0, 0)
        local l3, r3, t3, b3 = f:GetHitRectInsets()
        assert(l3 == 0 and r3 == 0 and t3 == 0 and b3 == 0,
            "reset insets should be 0,0,0,0")

        HIT_RECT_TEST_OK = true
    "#,
    );

    let ok: bool = t.env.eval("return HIT_RECT_TEST_OK == true").unwrap();
    assert!(ok, "SetHitRectInsets / GetHitRectInsets Lua test failed");
}

#[cfg(feature = "gui")]
#[test]
fn test_hit_rect_insets_shrinks_hittable_rect() {
    use crate::LayoutRect;
    use crate::iced_app::build_hittable_rects;
    use crate::iced_app::frame_collect::CollectedFrames;

    let mut registry = crate::widget::WidgetRegistry::new();
    let frame = crate::widget::Frame::default();
    let id = frame.id;
    registry.register(frame);
    let frame = registry.get_mut(id).unwrap();
    frame.hit_rect_insets = (10.0, 20.0, 5.0, 15.0);

    let collected = CollectedFrames {
        hittable: vec![(
            id,
            LayoutRect {
                x: 100.0,
                y: 50.0,
                width: 200.0,
                height: 100.0,
            },
        )],
    };

    let result = build_hittable_rects(&collected, &registry);
    assert_eq!(result.len(), 1);
    let (rid, rect) = &result[0];
    assert_eq!(*rid, id);

    let scale = crate::render::texture::UI_SCALE;
    let expected_x = (100.0 + 10.0) * scale;
    let expected_y = (50.0 + 5.0) * scale;
    let expected_w = (200.0 - 10.0 - 20.0) * scale;
    let expected_h = (100.0 - 5.0 - 15.0) * scale;
    assert!(
        (rect.x - expected_x).abs() < 0.01,
        "x: {} != {}",
        rect.x,
        expected_x
    );
    assert!(
        (rect.y - expected_y).abs() < 0.01,
        "y: {} != {}",
        rect.y,
        expected_y
    );
    assert!(
        (rect.width - expected_w).abs() < 0.01,
        "w: {} != {}",
        rect.width,
        expected_w
    );
    assert!(
        (rect.height - expected_h).abs() < 0.01,
        "h: {} != {}",
        rect.height,
        expected_h
    );
}

#[test]
fn test_xml_hit_rect_insets() {
    let t = load_test_xml(
        "test-xml-hit-rect-insets",
        r#"<Ui>
            <Frame name="HitRectXMLFrame" parent="UIParent">
                <Size x="200" y="100"/>
                <Anchors><Anchor point="CENTER"/></Anchors>
                <HitRectInsets left="10" right="20" top="5" bottom="15"/>
            </Frame>
        </Ui>"#,
    );

    t.assert_lua_true(
        "return HitRectXMLFrame ~= nil",
        "HitRectXMLFrame should exist",
    );
    let insets: (f64, f64, f64, f64) = t
        .env
        .eval("return HitRectXMLFrame:GetHitRectInsets()")
        .unwrap();
    assert_eq!(
        insets,
        (10.0, 20.0, 5.0, 15.0),
        "XML HitRectInsets should be applied: got {:?}",
        insets
    );
}

#[test]
fn test_is_mouse_over_uses_mouse_position_and_optional_offsets() {
    let (t, _) = load_test_lua(
        "test-is-mouse-over",
        r#"
        local f = CreateFrame("Frame", "MouseOverFrame", UIParent)
        f:SetSize(100, 100)
        f:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
        f:EnableMouse(true)
        local left, bottom, width, height = f:GetRect()
        assert(left and bottom and width and height, "GetRect should resolve layout before IsMouseOver")
    "#,
    );

    t.env.state().borrow_mut().mouse_position = Some((150.0, 150.0));
    t.assert_lua_true(
        "return MouseOverFrame:IsMouseOver()",
        "mouse inside frame rect should return true",
    );

    t.env.state().borrow_mut().mouse_position = Some((95.0, 150.0));
    let without_offsets: bool = t.env.eval("return MouseOverFrame:IsMouseOver()").unwrap();
    assert!(!without_offsets, "mouse left of frame should return false");

    t.assert_lua_true(
        "return MouseOverFrame:IsMouseOver(10, 0, 0, 0)",
        "left offset should expand the mouse-over area",
    );
}

#[test]
fn test_is_mouse_over_false_when_mouse_position_unknown() {
    let (t, _) = load_test_lua(
        "test-is-mouse-over-no-mouse",
        r#"
        local f = CreateFrame("Frame", "MouseOverUnknownMouseFrame", UIParent)
        f:SetSize(100, 100)
        f:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
        f:GetRect()
    "#,
    );

    let result: bool = t
        .env
        .eval("return MouseOverUnknownMouseFrame:IsMouseOver()")
        .unwrap();
    assert!(
        !result,
        "IsMouseOver should be false when no mouse position is available",
    );
}

#[test]
fn test_is_mouse_over_requires_mouse_enabled() {
    let (t, _) = load_test_lua(
        "test-is-mouse-over-disabled",
        r#"
        local f = CreateFrame("Frame", "MouseOverDisabledFrame", UIParent)
        f:SetSize(100, 100)
        f:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
        f:GetRect()
    "#,
    );

    t.env.state().borrow_mut().mouse_position = Some((150.0, 150.0));

    let initially_disabled: bool = t
        .env
        .eval("return MouseOverDisabledFrame:IsMouseOver()")
        .unwrap();
    assert!(
        !initially_disabled,
        "IsMouseOver should be false when mouse is not enabled on the frame",
    );

    t.env
        .eval::<()>("MouseOverDisabledFrame:EnableMouse(true)")
        .unwrap();
    t.assert_lua_true(
        "return MouseOverDisabledFrame:IsMouseOver()",
        "enabling mouse should allow IsMouseOver to return true inside the frame",
    );
}

#[test]
fn test_intersects_uses_resolved_layout_rects_for_overlapping_frames() {
    let (t, _) = load_test_lua(
        "test-intersects-overlap",
        r#"
        local a = CreateFrame("Frame", "IntersectFrameA", UIParent)
        a:SetSize(100, 100)
        a:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)

        local b = CreateFrame("Frame", "IntersectFrameB", UIParent)
        b:SetSize(100, 100)
        b:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 150, -150)

        INTERSECTS = a:Intersects(b)
    "#,
    );

    t.assert_lua_true(
        "return INTERSECTS",
        "overlapping frames should intersect even when layout was not pre-resolved",
    );
}

#[test]
fn test_intersects_returns_false_for_disjoint_frames() {
    let (t, _) = load_test_lua(
        "test-intersects-disjoint",
        r#"
        local a = CreateFrame("Frame", "DisjointFrameA", UIParent)
        a:SetSize(100, 100)
        a:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)

        local b = CreateFrame("Frame", "DisjointFrameB", UIParent)
        b:SetSize(100, 100)
        b:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 400, -400)

        INTERSECTS = a:Intersects(b)
    "#,
    );

    let result: bool = t.env.eval("return INTERSECTS").unwrap();
    assert!(!result, "disjoint frames should not intersect");
}

#[test]
fn test_draw_layer_enabled_round_trip_tracks_per_layer_frame_state() {
    let env = WowLuaEnv::new().unwrap();
    env.eval::<()>(
        r#"
        local f = CreateFrame("Frame", "LayerToggleFrame", UIParent)

        assert(f:IsDrawLayerEnabled("BACKGROUND") == true, "background should default enabled")
        assert(f:IsDrawLayerEnabled("BORDER") == true, "border should default enabled")

        f:SetDrawLayerEnabled("BORDER", false)
        assert(f:IsDrawLayerEnabled("BORDER") == false, "border should disable")
        assert(f:IsDrawLayerEnabled("BACKGROUND") == true, "background should stay enabled")

        f:SetDrawLayerEnabled("BORDER", true)
        assert(f:IsDrawLayerEnabled("BORDER") == true, "border should re-enable")

        DRAW_LAYER_ENABLED_TEST_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env
        .eval("return DRAW_LAYER_ENABLED_TEST_OK == true")
        .unwrap();
    assert!(
        ok,
        "SetDrawLayerEnabled / IsDrawLayerEnabled Lua round-trip failed",
    );
}

#[test]
fn test_draw_layer_legacy_toggle_methods_update_layer_state() {
    let env = WowLuaEnv::new().unwrap();
    env.eval::<()>(
        r#"
        local f = CreateFrame("Frame", "LegacyLayerToggleFrame", UIParent)

        assert(f:IsDrawLayerEnabled("ARTWORK") == true, "artwork should default enabled")

        f:DisableDrawLayer("ARTWORK")
        assert(f:IsDrawLayerEnabled("ARTWORK") == false, "DisableDrawLayer should disable artwork")

        f:EnableDrawLayer("ARTWORK")
        assert(f:IsDrawLayerEnabled("ARTWORK") == true, "EnableDrawLayer should re-enable artwork")

        DRAW_LAYER_LEGACY_TOGGLE_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env
        .eval("return DRAW_LAYER_LEGACY_TOGGLE_OK == true")
        .unwrap();
    assert!(
        ok,
        "EnableDrawLayer / DisableDrawLayer Lua round-trip failed"
    );
}

#[test]
fn test_frame_buffer_methods_persist_flag_and_rotate_child_textures() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "FrameBufferFrame", UIParent)
        local first = frame:CreateTexture(nil, "ARTWORK")
        local second = frame:CreateTexture(nil, "OVERLAY")

        assert(not frame:IsFrameBuffer(), "frame buffer flag should default false")

        frame:SetIsFrameBuffer(true)
        assert(frame:IsFrameBuffer(), "frame buffer flag should enable")

        frame:RotateTextures(math.pi / 2)
        assert(math.abs(first:GetRotation() - (math.pi / 2)) < 0.001, "first child texture should rotate")
        assert(math.abs(second:GetRotation() - (math.pi / 2)) < 0.001, "second child texture should rotate")

        frame:SetIsFrameBuffer(false)
        assert(not frame:IsFrameBuffer(), "frame buffer flag should disable")

        FRAME_BUFFER_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env.eval("return FRAME_BUFFER_OK == true").unwrap();
    assert!(ok, "frame buffer flag/rotation round-trip should succeed");
}

#[test]
fn test_bounds_position_methods_use_geometry_and_persisted_insets() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "BoundsFrame", UIParent)
        frame:SetSize(120, 45)
        frame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 10, -20)

        local left0, bottom0, width0, height0 = frame:GetBoundsRect()
        assert(left0 == 10 and width0 == 120 and height0 == 45, "GetBoundsRect should reflect initial geometry")

        frame:SetPointsOffset(30, -40)
        local _, _, _, x, y = frame:GetPoint(1)
        assert(x == 30 and y == -40, "SetPointsOffset should overwrite anchor offsets")

        local left1, bottom1, width1, height1 = frame:GetBoundsRect()
        assert(left1 == 30 and width1 == 120 and height1 == 45, "GetBoundsRect should reflect updated anchor geometry")

        frame:SetClampRectInsets(1, 2, 3, 4)
        local l, r, t, b = frame:GetClampRectInsets()
        assert(l == 1 and r == 2 and t == 3 and b == 4, "GetClampRectInsets should return persisted inset values")

        BOUNDS_POSITION_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env.eval("return BOUNDS_POSITION_OK == true").unwrap();
    assert!(ok, "bounds/position geometry round-trip should succeed");
}

#[test]
fn test_drag_methods_transfer_and_clear_active_drag_frame() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        DragSourceFrame = CreateFrame("Frame", "DragSourceFrame", UIParent)
        DragDelegateFrame = CreateFrame("Frame", "DragDelegateFrame", UIParent)
    "#,
    )
    .unwrap();

    let source_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("DragSourceFrame")
        .unwrap();
    let delegate_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("DragDelegateFrame")
        .unwrap();

    env.state().borrow_mut().active_drag_frame = Some(source_id);

    let intercepted: bool = env
        .eval("return DragSourceFrame:InterceptStartDrag(DragDelegateFrame)")
        .unwrap();
    assert!(
        intercepted,
        "drag interception should succeed for an active source frame"
    );

    let source_dragging: bool = env.eval("return DragSourceFrame:IsDragging()").unwrap();
    let delegate_dragging: bool = env.eval("return DragDelegateFrame:IsDragging()").unwrap();
    assert!(
        !source_dragging,
        "source frame should stop reporting dragging after interception"
    );
    assert!(
        delegate_dragging,
        "delegate frame should report dragging after interception"
    );
    assert_eq!(
        env.state().borrow().active_drag_frame,
        Some(delegate_id),
        "delegate should become the active drag frame"
    );

    env.exec("DragDelegateFrame:AbortDrag()").unwrap();

    let delegate_dragging_after_abort: bool =
        env.eval("return DragDelegateFrame:IsDragging()").unwrap();
    assert!(
        !delegate_dragging_after_abort,
        "AbortDrag should clear dragging state for the active drag frame"
    );
    assert_eq!(
        env.state().borrow().active_drag_frame,
        None,
        "AbortDrag should clear the active drag frame"
    );
}

#[test]
fn test_propagation_methods_round_trip_frame_flags() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        PropagationFrame = CreateFrame("Frame", "PropagationFrame", UIParent)

        assert(not PropagationFrame:CanPropagateMouseClicks(), "mouse clicks should default false")
        assert(not PropagationFrame:CanPropagateMouseMotion(), "mouse motion should default false")
        assert(not PropagationFrame:DoesHyperlinkPropagateToParent(), "hyperlink propagation should default false")

        PropagationFrame:SetPropagateMouseClicks(true)
        PropagationFrame:SetPropagateMouseMotion(true)
        PropagationFrame:SetHyperlinkPropagateToParent(true)

        assert(PropagationFrame:CanPropagateMouseClicks(), "mouse clicks should enable")
        assert(PropagationFrame:CanPropagateMouseMotion(), "mouse motion should enable")
        assert(PropagationFrame:DoesHyperlinkPropagateToParent(), "hyperlink propagation should enable")

        PropagationFrame:SetPropagateMouseClicks(false)
        PropagationFrame:SetPropagateMouseMotion(false)
        PropagationFrame:SetHyperlinkPropagateToParent(false)

        assert(not PropagationFrame:CanPropagateMouseClicks(), "mouse clicks should disable")
        assert(not PropagationFrame:CanPropagateMouseMotion(), "mouse motion should disable")
        assert(not PropagationFrame:DoesHyperlinkPropagateToParent(), "hyperlink propagation should disable")

        PROPAGATION_FLAGS_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env.eval("return PROPAGATION_FLAGS_OK == true").unwrap();
    assert!(ok, "propagation flag round-trip should succeed");
}

#[test]
fn test_gamepad_methods_round_trip_frame_state() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        GamePadFrame = CreateFrame("Frame", "GamePadFrame", UIParent)

        assert(not GamePadFrame:IsGamePadButtonEnabled(), "gamepad button should default false")
        assert(not GamePadFrame:IsGamePadStickEnabled(), "gamepad stick should default false")
        assert(not GamePadFrame:ShouldButtonPassThrough("LeftButton"), "button passthrough should default false")

        GamePadFrame:EnableGamePadButton(true)
        GamePadFrame:EnableGamePadStick(true)
        GamePadFrame:SetPassThroughButtons("LeftButton", "RightButton")

        assert(GamePadFrame:IsGamePadButtonEnabled(), "gamepad button should enable")
        assert(GamePadFrame:IsGamePadStickEnabled(), "gamepad stick should enable")
        assert(GamePadFrame:ShouldButtonPassThrough("LeftButton"), "left button should pass through after configuration")
        assert(GamePadFrame:ShouldButtonPassThrough("RIGHTBUTTON"), "button passthrough should be case-insensitive")
        assert(not GamePadFrame:ShouldButtonPassThrough("MiddleButton"), "unconfigured buttons should not pass through")

        GamePadFrame:EnableGamePadButton(false)
        GamePadFrame:EnableGamePadStick(false)
        GamePadFrame:SetPassThroughButtons()

        assert(not GamePadFrame:IsGamePadButtonEnabled(), "gamepad button should disable")
        assert(not GamePadFrame:IsGamePadStickEnabled(), "gamepad stick should disable")
        assert(not GamePadFrame:ShouldButtonPassThrough("LeftButton"), "button passthrough should clear")

        GAMEPAD_FLAGS_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env.eval("return GAMEPAD_FLAGS_OK == true").unwrap();
    assert!(ok, "gamepad flag round-trip should succeed");
}

#[test]
fn test_alpha_gradient_surface_round_trip_frame_state() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        AlphaGradientFrame = CreateFrame("Frame", "AlphaGradientFrame", UIParent)

        assert(not AlphaGradientFrame:HasAlphaGradient(), "alpha gradient should default disabled")

        AlphaGradientFrame:SetAlphaGradient(2, { x = 0.25, y = 0.75 })
        assert(AlphaGradientFrame:HasAlphaGradient(), "alpha gradient should enable after SetAlphaGradient")

        AlphaGradientFrame:ClearAlphaGradient()
        assert(not AlphaGradientFrame:HasAlphaGradient(), "alpha gradient should clear after ClearAlphaGradient")

        ALPHA_GRADIENT_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env.eval("return ALPHA_GRADIENT_OK == true").unwrap();
    assert!(ok, "alpha gradient round-trip should succeed");
}

#[test]
fn test_font_string_set_alpha_gradient_accepts_legacy_arguments() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local frame = CreateFrame("Frame", nil, UIParent)
        local fs = frame:CreateFontString(nil, "OVERLAY")
        fs:SetText("Hello World")

        local ok, applied = pcall(function()
            return fs:SetAlphaGradient(0, 50)
        end)

        assert(ok, "FontString:SetAlphaGradient should not error for legacy arguments")
        assert(applied == true, "FontString:SetAlphaGradient should report success")

        FONTSTRING_ALPHA_GRADIENT_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env
        .eval("return FONTSTRING_ALPHA_GRADIENT_OK == true")
        .unwrap();
    assert!(ok, "FontString alpha gradient compatibility should succeed");
}

#[test]
fn test_frame_level_methods_follow_parent_level_and_raise_state() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        ParentLevelFrame = CreateFrame("Frame", "ParentLevelFrame", UIParent)
        ChildLevelFrame = CreateFrame("Frame", "ChildLevelFrame", ParentLevelFrame)
        GrandchildLevelFrame = CreateFrame("Frame", "GrandchildLevelFrame", ChildLevelFrame)
        SiblingLevelFrame = CreateFrame("Frame", "SiblingLevelFrame", ParentLevelFrame)

        ParentLevelFrame:SetFrameLevel(10)

        assert(ChildLevelFrame:IsUsingParentLevel(), "child should inherit parent level by default")
        assert(ChildLevelFrame:GetFrameLevel() == 11, "child should inherit parent level plus default offset")
        assert(GrandchildLevelFrame:GetFrameLevel() == 12, "grandchild should inherit recursively")
        assert(ParentLevelFrame:GetHighestFrameLevel() == 10, "default highest level should use self")
        assert(ParentLevelFrame:GetHighestFrameLevel(true) == 12, "highest level should include descendants when requested")

        ChildLevelFrame:SetUsingParentLevel(false)
        ChildLevelFrame:SetFrameLevel(30)

        assert(not ChildLevelFrame:IsUsingParentLevel(), "child should stop inheriting after SetUsingParentLevel(false)")
        assert(ChildLevelFrame:GetFrameLevel() == 30, "child should keep explicit fixed frame level")
        assert(ChildLevelFrame:GetHighestFrameLevel(true) == 31, "fixed child highest level should include descendant")
        assert(GrandchildLevelFrame:GetFrameLevel() == 31, "grandchild should inherit from fixed child level")

        ParentLevelFrame:SetFrameLevel(20)

        assert(ChildLevelFrame:GetFrameLevel() == 30, "fixed child level should survive parent level changes")
        assert(SiblingLevelFrame:GetFrameLevel() == 21, "sibling should continue inheriting updated parent level")
        assert(ParentLevelFrame:GetHighestFrameLevel(true) == 31, "highest level should reflect deepest descendant")

        ChildLevelFrame:Raise()
        assert(ChildLevelFrame:GetRaisedFrameLevel() > ChildLevelFrame:GetFrameLevel(), "raised frame level should include raise order")

        ChildLevelFrame:SetUsingParentLevel(true)

        assert(ChildLevelFrame:IsUsingParentLevel(), "child should resume inheriting after SetUsingParentLevel(true)")
        assert(ChildLevelFrame:GetFrameLevel() == 21, "child should snap back to parent-derived level")
        assert(ChildLevelFrame:GetHighestFrameLevel(true) == 22, "highest level should update after re-enabling inheritance")
        assert(GrandchildLevelFrame:GetFrameLevel() == 22, "grandchild should re-inherit from updated child level")

        FRAME_LEVEL_METHODS_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env.eval("return FRAME_LEVEL_METHODS_OK == true").unwrap();
    assert!(ok, "frame level method round-trip should succeed");
}

#[test]
fn test_secret_and_protected_methods_reflect_frame_security_state() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        SecretValuesFrame = CreateFrame("Frame", "SecretValuesFrame", UIParent)
        ProtectedSecretFrame = CreateFrame("Frame", "ProtectedSecretFrame", UIParent)
        ForbiddenSecretFrame = CreateFrame("Frame", "ForbiddenSecretFrame", UIParent)

        assert(not SecretValuesFrame:HasAnySecretAspect(), "new frame should not have secret aspects by default")
        assert(not SecretValuesFrame:HasSecretValues(), "new frame should not have secret values by default")
        assert(not SecretValuesFrame:IsPreventingSecretValues(), "new frame should not prevent secret values by default")
        assert(not SecretValuesFrame:IsAnchoringSecret(), "new frame should not be anchoring secret by default")
        assert(not SecretValuesFrame:IsAnchoringRestricted(), "new frame should not be anchoring restricted by default")
        assert(not SecretValuesFrame:HasSecretAspect(Enum.SecretAspect.FrameLevel), "unrelated secret aspect should stay false")

        SecretValuesFrame:SetPreventSecretValues(true)

        assert(SecretValuesFrame:IsPreventingSecretValues(), "SetPreventSecretValues(true) should persist")
        assert(SecretValuesFrame:HasSecretValues(), "preventing secret values should mark the frame as having secret values")
        assert(SecretValuesFrame:HasAnySecretAspect(), "secret-valued frame should report a secret aspect")
        assert(SecretValuesFrame:HasSecretAspect(Enum.SecretAspect.ObjectSecrets), "object secret aspect should be present")
        assert(SecretValuesFrame:IsAnchoringSecret(), "secret-valued frame should be anchoring secret")
        assert(not SecretValuesFrame:IsAnchoringRestricted(), "secret-valued frame should not become anchoring restricted")

        SecretValuesFrame:SetPreventSecretValues(false)

        assert(not SecretValuesFrame:IsPreventingSecretValues(), "SetPreventSecretValues(false) should clear")
        assert(not SecretValuesFrame:HasSecretValues(), "clearing prevention should clear secret values")
        assert(not SecretValuesFrame:HasAnySecretAspect(), "clearing prevention should clear secret aspects")
        assert(not SecretValuesFrame:IsAnchoringSecret(), "clearing prevention should clear anchoring secret")

        A_Admin.SetFrameProtected("ProtectedSecretFrame", true)
        ForbiddenSecretFrame:SetForbidden(true)

        assert(ProtectedSecretFrame:IsAnchoringRestricted(), "protected frames should be anchoring restricted")
        assert(ProtectedSecretFrame:HasAnySecretAspect(), "protected frames should report a secret/security aspect")
        assert(ProtectedSecretFrame:HasSecretAspect(Enum.SecretAspect.ObjectSecurity), "protected frames should report object security aspect")
        assert(not ProtectedSecretFrame:HasSecretValues(), "protected frames should not imply secret values")
        assert(not ProtectedSecretFrame:IsAnchoringSecret(), "protected frames should not imply anchoring secret")

        assert(ForbiddenSecretFrame:IsAnchoringRestricted(), "forbidden frames should be anchoring restricted")
        assert(ForbiddenSecretFrame:HasAnySecretAspect(), "forbidden frames should report a secret/security aspect")
        assert(ForbiddenSecretFrame:HasSecretAspect(Enum.SecretAspect.ObjectSecurity), "forbidden frames should report object security aspect")

        SECRET_PROTECTED_STATE_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env
        .eval("return SECRET_PROTECTED_STATE_OK == true")
        .unwrap();
    assert!(
        ok,
        "secret/protected state should round-trip through frame methods"
    );
}

#[test]
fn test_flatten_render_methods_track_local_and_inherited_state() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        FlattenRootFrame = CreateFrame("Frame", "FlattenRootFrame", UIParent)
        FlattenParentFrame = CreateFrame("Frame", "FlattenParentFrame", FlattenRootFrame)
        FlattenChildFrame = CreateFrame("Frame", "FlattenChildFrame", FlattenParentFrame)

        assert(not FlattenRootFrame:GetFlattensRenderLayers(), "new frames should default flatten=false")
        assert(not FlattenRootFrame:GetEffectivelyFlattensRenderLayers(), "root should default effective flatten=false")
        assert(not FlattenChildFrame:GetFlattensRenderLayers(), "child local flatten should default false")
        assert(not FlattenChildFrame:GetEffectivelyFlattensRenderLayers(), "child effective flatten should default false")

        FlattenParentFrame:SetFlattensRenderLayers(true)

        assert(FlattenParentFrame:GetFlattensRenderLayers(), "local flatten flag should persist on the frame")
        assert(FlattenParentFrame:GetEffectivelyFlattensRenderLayers(), "frame should effectively flatten when local flag is enabled")
        assert(not FlattenChildFrame:GetFlattensRenderLayers(), "descendants should not inherit the local flatten flag itself")
        assert(FlattenChildFrame:GetEffectivelyFlattensRenderLayers(), "descendants should inherit effective flattening from ancestors")
        assert(not FlattenRootFrame:GetEffectivelyFlattensRenderLayers(), "ancestors should not inherit flattening upward")

        FlattenChildFrame:SetFlattensRenderLayers(true)
        FlattenParentFrame:SetFlattensRenderLayers(false)

        assert(FlattenChildFrame:GetFlattensRenderLayers(), "child local flatten flag should persist independently")
        assert(FlattenChildFrame:GetEffectivelyFlattensRenderLayers(), "child local flatten should keep effective flattening enabled")
        assert(not FlattenParentFrame:GetFlattensRenderLayers(), "parent local flatten flag should clear")
        assert(not FlattenParentFrame:GetEffectivelyFlattensRenderLayers(), "cleared parent should stop flattening effectively")

        FLATTEN_RENDER_METHODS_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env
        .eval("return FLATTEN_RENDER_METHODS_OK == true")
        .unwrap();
    assert!(
        ok,
        "flatten render layer methods should track local and inherited state"
    );
}

#[test]
fn test_window_display_methods_persist_window_and_dont_save_position() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        WindowOwnerFrame = CreateFrame("Frame", "WindowOwnerFrame", UIParent)
        local firstWindow = { name = "first" }
        local secondWindow = { name = "second" }

        assert(not WindowOwnerFrame:GetDontSavePosition(), "frames should default to saving their position")
        assert(WindowOwnerFrame:GetWindow() == nil, "frames should default to no associated window")

        WindowOwnerFrame:SetDontSavePosition(true)
        assert(WindowOwnerFrame:GetDontSavePosition(), "SetDontSavePosition(true) should persist")

        WindowOwnerFrame:SetWindow(firstWindow)
        assert(WindowOwnerFrame:GetWindow() == firstWindow, "SetWindow should persist the associated window object")

        WindowOwnerFrame:SetWindow(secondWindow)
        assert(WindowOwnerFrame:GetWindow() == secondWindow, "SetWindow should overwrite the previous window object")

        WindowOwnerFrame:SetWindow(nil)
        assert(WindowOwnerFrame:GetWindow() == nil, "SetWindow(nil) should clear the associated window")

        WindowOwnerFrame:SetDontSavePosition(false)
        assert(not WindowOwnerFrame:GetDontSavePosition(), "SetDontSavePosition(false) should clear the persisted flag")

        WINDOW_DISPLAY_METHODS_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env
        .eval("return WINDOW_DISPLAY_METHODS_OK == true")
        .unwrap();
    assert!(
        ok,
        "window display methods should persist associated window and dont-save-position state"
    );
}

#[test]
fn test_misc_visual_state_methods_persist_and_desaturate_hierarchy() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        MiscStateRootFrame = CreateFrame("Frame", "MiscStateRootFrame", UIParent)
        MiscStateChildTexture = MiscStateRootFrame:CreateTexture("MiscStateChildTexture", "ARTWORK")
        MiscStateGrandchildFrame = CreateFrame("Frame", "MiscStateGrandchildFrame", MiscStateRootFrame)
        MiscStateGrandchildTexture = MiscStateGrandchildFrame:CreateTexture("MiscStateGrandchildTexture", "ARTWORK")

        assert(not MiscStateRootFrame:IsHighlightLocked(), "highlight lock should default false")
        assert(not MiscStateRootFrame:IsIgnoringChildrenForBounds(), "ignore children for bounds should default false")
        assert(not MiscStateChildTexture:IsDesaturated(), "child texture should default not desaturated")
        assert(not MiscStateGrandchildTexture:IsDesaturated(), "grandchild texture should default not desaturated")

        MiscStateRootFrame:SetHighlightLocked(true)
        MiscStateRootFrame:SetIgnoringChildrenForBounds(true)
        MiscStateRootFrame:DesaturateHierarchy(1, true)

        assert(MiscStateRootFrame:IsHighlightLocked(), "highlight lock should persist")
        assert(MiscStateRootFrame:IsIgnoringChildrenForBounds(), "ignore children for bounds should persist")
        assert(MiscStateChildTexture:IsDesaturated(), "desaturate hierarchy should affect direct child textures")
        assert(MiscStateGrandchildTexture:IsDesaturated(), "desaturate hierarchy should affect descendant textures")

        MiscStateRootFrame:SetHighlightLocked(false)
        MiscStateRootFrame:SetIgnoringChildrenForBounds(false)
        MiscStateRootFrame:DesaturateHierarchy(0)

        assert(not MiscStateRootFrame:IsHighlightLocked(), "highlight lock should clear")
        assert(not MiscStateRootFrame:IsIgnoringChildrenForBounds(), "ignore children for bounds should clear")
        assert(not MiscStateChildTexture:IsDesaturated(), "desaturate hierarchy should clear child textures")
        assert(not MiscStateGrandchildTexture:IsDesaturated(), "desaturate hierarchy should clear descendant textures")

        MISC_VISUAL_STATE_METHODS_OK = true
    "#,
    )
    .unwrap();

    {
        let state = env.state().borrow();
        let root_id = state
            .widgets
            .get_id_by_name("MiscStateRootFrame")
            .expect("root frame should exist");
        let root = state
            .widgets
            .get(root_id)
            .expect("root frame should be readable");
        assert!(
            !root.desaturated,
            "excludeRoot=true should leave the root frame undessaturated"
        );
    }

    let ok: bool = env
        .eval("return MISC_VISUAL_STATE_METHODS_OK == true")
        .unwrap();
    assert!(
        ok,
        "misc visual state methods should persist booleans and desaturate descendants"
    );
}

mod global_frame_access;
mod inline_script;
mod layout_alpha;
mod layout_anchoring;
mod layout_positioning;
mod layout_scale;
mod layout_size;
mod wow_api;
mod wow_api_globals;
mod wow_api_tooltip;
