//! Tests for the addon loader.

mod account_store;
mod screen_selection;
mod xml_basics;

use super::addon::AddonContext;
use super::lua_file::load_lua_file;
use super::xml_file::load_xml_file;
use super::*;
use crate::lua_api::WowLuaEnv;
use crate::lua_api::rilua_methods::{call_function as call_rilua_function, val_to_string};
use rilua::{LuaApi, LuaApiMut, Val};

/// Test context holding environment and temp directory for cleanup.
pub(super) struct TestCtx {
    pub(super) env: WowLuaEnv,
    pub(super) temp_dir: PathBuf,
}

impl TestCtx {
    /// Assert a Lua expression evaluates to true.
    pub(super) fn assert_lua_true(&self, expr: &str, msg: &str) {
        let result: bool = self.env.eval(expr).unwrap();
        assert!(result, "{}", msg);
    }

    /// Assert a Lua expression returns the expected string.
    pub(super) fn assert_lua_str(&self, expr: &str, expected: &str) {
        let result: String = self.env.eval(expr).unwrap();
        assert_eq!(result, expected);
    }

    /// Assert that a script handler is set on a frame.
    pub(super) fn assert_script_set(&self, frame: &str, handler: &str) {
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

fn table_get(env: &WowLuaEnv, table: Val, key: &str) -> Val {
    let Val::Table(table_ref) = table else {
        panic!("expected table for key lookup: {key}");
    };
    let mut lua = env.rilua_mut();
    let state = lua.state_mut();
    let key_ref = state.gc.intern_string(key.as_bytes());
    if let Some(table) = state.gc.tables.get(table_ref) {
        table.get_str(key_ref, &state.gc.string_arena)
    } else {
        Val::Nil
    }
}

fn val_to_i32(value: Val) -> i32 {
    match value {
        Val::Num(n) => {
            let int = n as i32;
            assert_eq!(int as f64, n, "expected integer value, got {n}");
            int
        }
        other => panic!("expected numeric value, got {}", other.type_name()),
    }
}

fn val_to_rust_string(env: &WowLuaEnv, value: Val) -> String {
    let lua = env.rilua();
    val_to_string(lua.state(), value)
        .unwrap_or_else(|| panic!("expected string value, got {}", value.type_name()))
}

/// Create a test environment, write XML content, load it, return context.
pub(super) fn load_test_xml(dir_suffix: &str, xml_content: &str) -> TestCtx {
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
pub(super) fn load_test_lua(dir_suffix: &str, lua_content: &str) -> (TestCtx, Val) {
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

pub(super) fn register_loading_test_addon(env: &WowLuaEnv) {
    env.register_addon(crate::lua_api::AddonInfo {
        folder_name: "TestAddon".to_string(),
        title: "TestAddon".to_string(),
        enabled: true,
        loaded: true,
        ..Default::default()
    });
    set_loading_addon_index(env, "TestAddon");
}

pub(super) fn set_loading_addon_index(env: &WowLuaEnv, addon_name: &str) {
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
    let (t, addon_table) = load_test_lua(
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

    assert_eq!(val_to_i32(table_get(&t.env, addon_table, "result")), 42);
    let create_something = table_get(&t.env, addon_table, "CreateSomething");
    let mut lua = t.env.rilua_mut();
    let result = call_rilua_function(&mut lua, create_something, &[addon_table]).unwrap();
    assert_eq!(val_to_i32(result), 20);
}

/// Load multiple Lua files in sequence with a shared addon table.
fn load_test_lua_files(
    dir_suffix: &str,
    addon_name: &str,
    files: &[(&str, &str)],
) -> (TestCtx, Val) {
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
    let (t, addon_table) = load_test_lua_files(
        "test-multifile",
        "MultiFileTest",
        &[
            ("widgets.lua", MULTI_FILE_WIDGETS_LUA),
            ("button.lua", MULTI_FILE_BUTTON_LUA),
            ("addon.lua", MULTI_FILE_ADDON_LUA),
        ],
    );

    let test_button = table_get(&t.env, addon_table, "testButton");
    let result = val_to_rust_string(&t.env, table_get(&t.env, test_button, "result"));
    assert!(
        result.starts_with("updated:"),
        "updateKeyDirection should have been called, got: {}",
        result
    );
}

#[test]
fn test_runtime_action_button_template_creates_named_children() {
    let t = load_test_xml(
        "runtime-action-button-template",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Cooldown name="CooldownFrameTemplate" hidden="true" setAllPoints="true" virtual="true"/>
            <CheckButton name="ActionButtonTemplate" virtual="true">
                <Frames>
                    <Frame parentKey="TextOverlayContainer">
                        <Size x="10" y="11"/>
                        <Anchors>
                            <Anchor point="CENTER"/>
                        </Anchors>
                        <Scripts>
                            <OnLoad>self.loaded = true;</OnLoad>
                        </Scripts>
                    </Frame>
                    <Cooldown name="$parentCooldown" parentKey="cooldown" inherits="CooldownFrameTemplate" id="17">
                        <Anchors>
                            <Anchor point="TOPLEFT"/>
                            <Anchor point="BOTTOMRIGHT"/>
                        </Anchors>
                    </Cooldown>
                </Frames>
            </CheckButton>
        </Ui>
        "#,
    );

    t.env
        .exec(
            r#"
            local button = CreateFrame("CheckButton", "ActionButtonFastPath", UIParent, "ActionButtonTemplate")
            assert(button.TextOverlayContainer ~= nil, "TextOverlayContainer should exist")
            assert(button.TextOverlayContainer.loaded == true, "child OnLoad should fire")
            assert(button.cooldown ~= nil, "cooldown child should exist")
            assert(ActionButtonFastPathCooldown == button.cooldown, "named cooldown global should resolve")
            assert(button.cooldown:GetParent() == button, "cooldown parent should be button")
            assert(button.cooldown:GetID() == 17, "cooldown xml id should be preserved")
            assert(not button.cooldown:IsShown(), "inherited hidden cooldown should stay hidden")
        "#,
        )
        .unwrap();
}

#[test]
fn test_runtime_spellfx_template_creates_nested_inherited_children() {
    let t = load_test_xml(
        "runtime-spellfx-template",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="ActionButtonInterruptTemplate" virtual="true">
                <Frames>
                    <Frame parentKey="Highlight" hidden="true">
                        <Size x="7" y="8"/>
                        <Anchors>
                            <Anchor point="CENTER"/>
                        </Anchors>
                        <Scripts>
                            <OnLoad>self.loaded = true;</OnLoad>
                        </Scripts>
                    </Frame>
                </Frames>
            </Frame>
            <Frame name="ActionButtonCastingAnimFrameTemplate" virtual="true">
                <Frames>
                    <Frame parentKey="Fill">
                        <Size x="9" y="10"/>
                        <Anchors>
                            <Anchor point="CENTER"/>
                        </Anchors>
                        <Scripts>
                            <OnLoad>self.loaded = true;</OnLoad>
                        </Scripts>
                    </Frame>
                </Frames>
            </Frame>
            <CheckButton name="ActionButtonSpellFXTemplate" virtual="true">
                <Frames>
                    <Frame parentKey="InterruptDisplay" inherits="ActionButtonInterruptTemplate" hidden="true"/>
                    <Frame parentKey="SpellCastAnimFrame" inherits="ActionButtonCastingAnimFrameTemplate" hidden="true"/>
                </Frames>
            </CheckButton>
        </Ui>
        "#,
    );

    t.env
        .exec(
            r#"
            local button = CreateFrame("CheckButton", "SpellFXFastPathButton", UIParent, "ActionButtonSpellFXTemplate")
            assert(button.InterruptDisplay ~= nil, "InterruptDisplay should exist")
            assert(button.SpellCastAnimFrame ~= nil, "SpellCastAnimFrame should exist")
            assert(not button.InterruptDisplay:IsShown(), "inherited hidden flag should be preserved")
            assert(not button.SpellCastAnimFrame:IsShown(), "spell cast child should inherit hidden state")

            assert(button.InterruptDisplay.Highlight ~= nil, "nested interrupt child should exist")
            assert(button.InterruptDisplay.Highlight.loaded == true, "nested interrupt OnLoad should fire")
            assert(button.InterruptDisplay.Highlight:GetParent() == button.InterruptDisplay, "nested interrupt child parent should match")

            assert(button.SpellCastAnimFrame.Fill ~= nil, "nested casting child should exist")
            assert(button.SpellCastAnimFrame.Fill.loaded == true, "nested casting OnLoad should fire")
            assert(button.SpellCastAnimFrame.Fill:GetParent() == button.SpellCastAnimFrame, "nested casting child parent should match")
        "#,
        )
        .unwrap();
}

#[test]
fn test_runtime_minimal_scrollbar_avoids_lua_createframe_for_nested_thumb() {
    let t = load_test_xml(
        "runtime-minimal-scrollbar-direct-grandchildren",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <EventFrame name="MinimalScrollBar" virtual="true">
                <Frames>
                    <Frame parentKey="Track">
                        <Frames>
                            <EventButton parentKey="Thumb">
                                <Scripts>
                                    <OnLoad>self.loaded = true;</OnLoad>
                                </Scripts>
                            </EventButton>
                        </Frames>
                    </Frame>
                </Frames>
            </EventFrame>
        </Ui>
        "#,
    );

    t.env
        .exec(
            r#"
            local originalCreateFrame = CreateFrame
            local createCount = 0
            CreateFrame = function(...)
                createCount = createCount + 1
                return originalCreateFrame(...)
            end

            local scrollbar = CreateFrame("EventFrame", "MinimalScrollBarFastPath", UIParent, "MinimalScrollBar")
            assert(scrollbar.Track ~= nil, "Track child should exist")
            assert(scrollbar.Track.Thumb ~= nil, "Thumb grandchild should exist")
            assert(scrollbar.Track.Thumb.loaded == true, "Thumb OnLoad should fire")
            assert(createCount == 1, "nested thumb should avoid Lua CreateFrame fallback, got " .. createCount)
        "#,
        )
        .unwrap();
}

#[test]
fn test_anonymous_runtime_template_uses_registry_frame_refs_without_global_alias() {
    let t = load_test_xml(
        "runtime-anon-template-registry-ref",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="AnonymousTemplate" virtual="true">
                <Frames>
                    <Frame parentKey="Child">
                        <Scripts>
                            <OnLoad>self.loaded = true;</OnLoad>
                        </Scripts>
                    </Frame>
                </Frames>
                <Scripts>
                    <OnLoad>self.loaded = true;</OnLoad>
                </Scripts>
            </Frame>
        </Ui>
        "#,
    );

    t.env
        .exec(
            r#"
            __test_frame = CreateFrame("Frame", nil, UIParent, "AnonymousTemplate")
            assert(__test_frame.loaded == true, "anonymous template OnLoad should fire")
            assert(__test_frame.Child ~= nil, "anonymous template child should exist")
            assert(__test_frame.Child.loaded == true, "anonymous template child OnLoad should fire")
        "#,
        )
        .unwrap();

    t.assert_lua_true(
        "return __test_frame ~= nil and __test_frame.Child ~= nil",
        "anonymous runtime template frame should stay reachable",
    );
}

#[test]
fn test_action_button_updates_use_registry_frame_refs_for_anonymous_buttons() {
    let t = load_test_xml(
        "runtime-anon-action-button-registry-ref",
        "<Ui xmlns=\"http://www.blizzard.com/wow/ui/\"/>",
    );

    t.env
        .exec(
            r#"
            __test_button = CreateFrame("Button", nil, UIParent)
            rawset(__test_button, "UpdateState", function(self)
                self.updateCalls = (self.updateCalls or 0) + 1
            end)
            SetActionUIButton(__test_button, 1)
        "#,
        )
        .unwrap();

    t.assert_lua_true(
        "return __test_button ~= nil",
        "anonymous action button should stay reachable",
    );

    crate::lua_api::globals::action_bar_api::push_action_button_state_update(
        t.env.state(),
        t.env.lua(),
    )
    .unwrap();

    t.assert_lua_true(
        "__test_button.updateCalls == 1",
        "anonymous action button UpdateState should run through registry frame refs",
    );
}

#[test]
fn test_runtime_template_mixin_and_key_values_apply() {
    let t = load_test_xml(
        "runtime-template-mixin-keyvalues",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="RuntimeTemplateTest" virtual="true" mixin="RuntimeTemplateMixin">
                <KeyValues>
                    <KeyValue key="myString" value="hello"/>
                    <KeyValue key="myNumber" value="42" type="number"/>
                    <KeyValue key="myBool" value="true" type="boolean"/>
                    <KeyValue key="myGlobal" value="RuntimeTemplateGlobals.Token" type="global"/>
                </KeyValues>
            </Frame>
        </Ui>
        "#,
    );

    t.env
        .exec(
            r#"
            RuntimeTemplateGlobals = { Token = "ready" }
            RuntimeTemplateMixin = {
                Describe = function(self)
                    return self.myString .. ":" .. tostring(self.myNumber) .. ":" .. tostring(self.myBool) .. ":" .. self.myGlobal
                end,
            }

            local frame = CreateFrame("Frame", "RuntimeTemplateInstance", UIParent, "RuntimeTemplateTest")
            assert(frame.myString == "hello", "string key value should apply")
            assert(frame.myNumber == 42, "numeric key value should apply")
            assert(frame.myBool == true, "boolean key value should apply")
            assert(frame.myGlobal == "ready", "dotted global key value should resolve")
            assert(frame:Describe() == "hello:42:true:ready", "mixin method should apply")
        "#,
        )
        .unwrap();
}

#[test]
fn test_runtime_template_method_scripts_apply() {
    let t = load_test_xml(
        "runtime-template-method-scripts",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="RuntimeMethodScriptTemplate" virtual="true" mixin="RuntimeMethodScriptMixin">
                <Scripts>
                    <OnLoad method="RuntimeMethodScriptTemplate_OnLoad"/>
                    <OnEvent method="RuntimeMethodScriptTemplate_OnEvent"/>
                </Scripts>
            </Frame>
        </Ui>
        "#,
    );

    t.env
        .exec(
            r#"
            RuntimeMethodScriptMixin = {
                RuntimeMethodScriptTemplate_OnLoad = function(self)
                    self.loadedByMethodScript = true
                end,
                RuntimeMethodScriptTemplate_OnEvent = function(self, event, payload)
                    self.lastMethodEvent = event .. ":" .. tostring(payload)
                end,
            }

            local frame = CreateFrame("Frame", "RuntimeMethodScriptFrame", UIParent, "RuntimeMethodScriptTemplate")
            assert(frame.loadedByMethodScript == true, "OnLoad method script should run")

            local onEvent = frame:GetScript("OnEvent")
            assert(type(onEvent) == "function", "OnEvent method script should be installed")
            onEvent(frame, "TEST_EVENT", "payload")
            assert(frame.lastMethodEvent == "TEST_EVENT:payload", "method script should dispatch through frame method")
        "#,
        )
        .unwrap();
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
fn test_is_mouse_over_clean_layout_does_not_require_mutable_state_borrow() {
    let (t, _) = load_test_lua(
        "test-is-mouse-over-clean-layout",
        r#"
        local f = CreateFrame("Frame", "MouseOverCleanLayoutFrame", UIParent)
        f:SetSize(100, 100)
        f:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
        f:EnableMouse(true)
        f:GetRect()
    "#,
    );

    t.env.state().borrow_mut().mouse_position = Some((150.0, 150.0));

    let state_borrow = t.env.state().borrow();
    assert_eq!(state_borrow.mouse_position, Some((150.0, 150.0)));

    let is_mouse_over: bool = t
        .env
        .eval("return MouseOverCleanLayoutFrame:IsMouseOver()")
        .unwrap();
    assert!(is_mouse_over, "mouse inside clean frame should return true");
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

mod frame_interaction;
mod frame_state;
mod global_frame_access;
mod inline_script;
mod layout_alpha;
mod layout_anchoring;
mod layout_positioning;
mod layout_scale;
mod layout_size;
mod minimap_specialized;
mod wow_api;
mod wow_api_globals;
mod wow_api_tooltip;
