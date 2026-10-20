use super::*;

#[test]
fn lifecycle_scripts_use_passed_frame_id_instead_of_name_lookup() {
    let t = load_test_xml(
        "lifecycle-frame-id",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="LifecycleRefFrame" parent="UIParent">
                <Scripts>
                    <OnLoad>LIFECYCLE_REF_ONLOAD = (LIFECYCLE_REF_ONLOAD or 0) + 1</OnLoad>
                    <OnShow>LIFECYCLE_REF_ONSHOW = (LIFECYCLE_REF_ONSHOW or 0) + 1</OnShow>
                </Scripts>
            </Frame>
        </Ui>
        "#,
    );

    let frame_id = t
        .env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("LifecycleRefFrame")
        .expect("LifecycleRefFrame should exist");

    fire_lifecycle_scripts(
        &t.env.loader_env(),
        frame_id,
        "DefinitelyNotTheFrameName",
        LifecycleScripts {
            on_load: true,
            on_show: true,
        },
    );

    let (on_load_count, on_show_count): (i32, i32) = t
        .env
        .eval("return LIFECYCLE_REF_ONLOAD or 0, LIFECYCLE_REF_ONSHOW or 0")
        .unwrap();

    assert_eq!(
        on_load_count, 2,
        "OnLoad should fire again via direct frame id"
    );
    assert_eq!(
        on_show_count, 2,
        "OnShow should fire again via direct frame id"
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
fn test_hidden_runtime_parent_does_not_hide_shown_template_children() {
    let t = load_test_xml(
        "runtime-hidden-parent-template-children",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="RuntimeVisibleChildTemplate" virtual="true">
                <Frames>
                    <Frame parentKey="ShownChild">
                        <Size x="10" y="10"/>
                    </Frame>
                </Frames>
            </Frame>
        </Ui>
        "#,
    );

    t.env
        .exec(
            r#"
            local parent = CreateFrame("Frame", "RuntimeHiddenTemplateParent", UIParent, "RuntimeVisibleChildTemplate")
            parent:Hide()
            assert(parent.ShownChild ~= nil, "template child should exist")
            assert(parent.ShownChild:IsShown(), "template child should keep its own shown flag while parent is hidden")
            assert(not parent.ShownChild:IsVisible(), "template child should not be effectively visible while parent is hidden")

            parent:Show()
            assert(parent.ShownChild:IsShown(), "template child shown flag should survive parent show")
            assert(parent.ShownChild:IsVisible(), "template child should become visible once parent is shown")
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
fn runtime_child_key_values_exist_before_inherited_onload() {
    let t = load_test_xml(
        "runtime-child-key-values-before-inherited-onload",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Button name="NeedsInstanceValueTemplate" virtual="true">
                <Scripts>
                    <OnLoad>
                        if self.slotName == nil then
                            error("missing slotName")
                        end
                        self.loadedSlotName = self.slotName
                    </OnLoad>
                </Scripts>
            </Button>

            <Frame name="RuntimeChildKeyValueParentTemplate" virtual="true">
                <Frames>
                    <ItemButton parentKey="Child" inherits="NeedsInstanceValueTemplate">
                        <KeyValues>
                            <KeyValue key="slotName" value="Prof0ToolSlot" type="string"/>
                        </KeyValues>
                    </ItemButton>
                    <ItemButton parentKey="SecondChild" inherits="NeedsInstanceValueTemplate">
                        <KeyValues>
                            <KeyValue key="slotName" value="Prof0Gear0Slot" type="string"/>
                        </KeyValues>
                    </ItemButton>
                    <DropdownButton parentKey="LinkButton"/>
                </Frames>
            </Frame>
        </Ui>
        "#,
    );

    t.env
        .exec(
            r#"
            local parent = CreateFrame("Frame", "RuntimeChildKeyValueParent", UIParent, "RuntimeChildKeyValueParentTemplate")
            assert(parent.Child ~= nil, "template child should exist")
            assert(parent.SecondChild ~= nil, "second template child should exist")
            assert(parent.LinkButton ~= nil, "dropdown template child should exist")
            assert(parent.Child:GetName() == nil, "anonymous template child GetName should be nil")
            assert(parent.Child.loadedSlotName == "Prof0ToolSlot", "inherited OnLoad should see child KeyValues")
            assert(parent.SecondChild.loadedSlotName == "Prof0Gear0Slot", "second inherited OnLoad should see child KeyValues")
        "#,
        )
        .unwrap();
}

#[test]
fn test_runtime_action_button_template_avoids_lua_layer_and_button_texture_methods() {
    let t = load_test_xml(
        "runtime-action-button-template-direct-layers",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <CheckButton name="ActionButtonTemplate" virtual="true">
                <Layers>
                    <Layer level="BACKGROUND">
                        <Texture name="$parentIcon" parentKey="icon" atlas="UI-HUD-ActionBar-IconFrame-Background">
                            <Anchors>
                                <Anchor point="CENTER"/>
                            </Anchors>
                        </Texture>
                    </Layer>
                    <Layer level="OVERLAY">
                        <FontString name="$parentName" parentKey="Name" inherits="GameFontHighlightSmallOutline" justifyH="RIGHT">
                            <Size x="36" y="10"/>
                            <Anchors>
                                <Anchor point="BOTTOM" x="0" y="2"/>
                            </Anchors>
                        </FontString>
                    </Layer>
                </Layers>
                <NormalTexture name="$parentNormalTexture" parentKey="NormalTexture" atlas="UI-HUD-ActionBar-IconFrame">
                    <Size x="46" y="45"/>
                    <Anchors>
                        <Anchor point="TOPLEFT"/>
                    </Anchors>
                </NormalTexture>
                <PushedTexture parentKey="PushedTexture" atlas="UI-HUD-ActionBar-IconFrame-Down">
                    <Size x="46" y="45"/>
                    <Anchors>
                        <Anchor point="TOPLEFT"/>
                    </Anchors>
                </PushedTexture>
            </CheckButton>
        </Ui>
        "#,
    );

    crate::lua_api::globals::template::test_counters::reset();

    t.env
        .exec(
            r#"
            local button = CreateFrame("CheckButton", "ActionButtonDirectLayerFastPath", UIParent, "ActionButtonTemplate")
            assert(button.icon ~= nil, "layer texture parentKey should exist")
            assert(button.Name ~= nil, "layer fontstring parentKey should exist")
            assert(button:GetNormalTexture() ~= nil, "normal texture should exist")
            assert(button:GetPushedTexture() ~= nil, "pushed texture should exist")
            assert(ActionButtonDirectLayerFastPathIcon == button.icon, "named layer texture global should resolve")
            assert(ActionButtonDirectLayerFastPathNormalTexture == button:GetNormalTexture(), "named button texture global should resolve")
            assert(button.icon:GetParent() == button, "layer texture parent should match")
            assert(button.Name:GetParent() == button, "layer fontstring parent should match")
            assert(button:GetNormalTexture():GetParent() == button, "normal texture parent should match")
            assert(button:GetPushedTexture():GetParent() == button, "pushed texture parent should match")
        "#,
        )
        .unwrap();

    let counts = crate::lua_api::globals::template::test_counters::snapshot();
    assert_eq!(
        counts.texture_creates, 0,
        "hot template layers should avoid Lua texture creation fallback, got {:?}",
        counts
    );
    assert_eq!(
        counts.fontstring_creates, 0,
        "hot template fontstrings should avoid Lua fontstring creation fallback, got {:?}",
        counts
    );
    assert_eq!(
        counts.button_texture_creates, 0,
        "hot template button textures should avoid Lua button texture creation fallback, got {:?}",
        counts
    );
}

#[test]
fn test_runtime_template_nested_anonymous_layers_keep_outer_parent_name() {
    let t = load_test_xml(
        "runtime-anon-wrapper-layer-names",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Button name="NestedLayerTemplate" virtual="true">
                <Frames>
                    <Frame>
                        <Frames>
                            <Frame>
                                <Layers>
                                    <Layer level="OVERLAY">
                                        <Texture name="$parentGlow"/>
                                    </Layer>
                                </Layers>
                            </Frame>
                        </Frames>
                    </Frame>
                </Frames>
            </Button>
        </Ui>
        "#,
    );

    t.env
        .exec(
            r#"
            local frame = CreateFrame("Button", "NestedLayerRuntimeProbe", UIParent, "NestedLayerTemplate")
            assert(NestedLayerRuntimeProbeGlow ~= nil, "nested layer child should keep the outer $parent name through anonymous wrappers")
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

    let test_frame: mlua::Value = t.env.lua().globals().get("__test_frame").unwrap();
    let frame_id = crate::lua_api::frame::extract_frame_id(&test_frame).unwrap();
    let expr = format!(
        "local reg = debug.getregistry(); return reg.__frame_refs[{id}] == __test_frame and _G[\"__frame_{id}\"] == nil",
        id = frame_id
    );
    t.assert_lua_true(
        &expr,
        "anonymous runtime template should use registry frame refs without leaking __frame globals",
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
            local env = debug.getfenv(__test_button)
            assert(env and env[1], "button env table should exist")
            rawset(env[1], "UpdateState", function(self)
                self.updateCalls = (self.updateCalls or 0) + 1
            end)
            SetActionUIButton(__test_button, 1)
        "#,
        )
        .unwrap();

    let test_button: mlua::Value = t.env.lua().globals().get("__test_button").unwrap();
    let frame_id = crate::lua_api::frame::extract_frame_id(&test_button).unwrap();
    let expr = format!(
        "local reg = debug.getregistry(); return reg.__frame_refs[{id}] == __test_button and _G[\"__frame_{id}\"] == nil",
        id = frame_id
    );
    t.assert_lua_true(
        &expr,
        "anonymous action button should stay out of _G and remain reachable through registry frame refs",
    );

    {
        let mut lua = t.env.rilua_mut();
        crate::lua_api::globals::action_bar_api::push_action_button_state_update(
            t.env.state(),
            &mut lua,
        )
        .unwrap();
    }

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
fn test_anonymous_runtime_template_mixin_and_key_values_apply() {
    let t = load_test_xml(
        "runtime-anon-template-mixin-keyvalues",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="AnonymousRuntimeTemplateTest" virtual="true" mixin="AnonymousRuntimeTemplateMixin">
                <KeyValues>
                    <KeyValue key="myString" value="hello"/>
                    <KeyValue key="myNumber" value="42" type="number"/>
                    <KeyValue key="myBool" value="true" type="boolean"/>
                    <KeyValue key="myGlobal" value="AnonymousRuntimeTemplateGlobals.Token" type="global"/>
                </KeyValues>
            </Frame>
        </Ui>
        "#,
    );

    t.env
        .exec(
            r#"
            AnonymousRuntimeTemplateGlobals = { Token = "ready" }
            AnonymousRuntimeTemplateMixin = {
                Describe = function(self)
                    return self.myString .. ":" .. tostring(self.myNumber) .. ":" .. tostring(self.myBool) .. ":" .. self.myGlobal
                end,
            }

            local frame = CreateFrame("Frame", nil, UIParent, "AnonymousRuntimeTemplateTest")
            assert(frame.myString == "hello", "anonymous string key value should apply")
            assert(frame.myNumber == 42, "anonymous numeric key value should apply")
            assert(frame.myBool == true, "anonymous boolean key value should apply")
            assert(frame.myGlobal == "ready", "anonymous dotted global key value should resolve")
            assert(frame:Describe() == "hello:42:true:ready", "anonymous mixin method should apply")
        "#,
        )
        .unwrap();
}

#[test]
fn test_runtime_template_local_mixin_and_key_value_apply() {
    let env = WowLuaEnv::new().unwrap();
    preload_shared_templates(&env);
    let temp_dir = std::env::temp_dir().join("wow-sim-runtime-template-local-source");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let lua_path = temp_dir.join("test.lua");
    let xml_path = temp_dir.join("test.xml");
    std::fs::write(
        &lua_path,
        r#"
        local _addonName, addon = ...
        addon.localToken = "from-local-keyvalue"
        addon.Nested = {}
        addon.Nested.LocalMixin = {
            DescribeLocal = function(self)
                return self.localToken
            end,
        }
        "#,
    )
    .unwrap();
    std::fs::write(
        &xml_path,
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="RuntimeTemplateLocalSource" virtual="true">
                <Mixins>
                    <Mixin key="Nested.LocalMixin" source="local"/>
                </Mixins>
                <KeyValues>
                    <KeyValue key="localToken" type="local"/>
                </KeyValues>
            </Frame>
        </Ui>
        "#,
    )
    .unwrap();

    register_loading_test_addon(&env);
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
    load_xml_file(
        &env.loader_env(),
        &xml_path,
        &ctx,
        &mut LoadTiming::default(),
    )
    .unwrap();

    let result: String = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "RuntimeTemplateLocalSourceInstance", UIParent, "RuntimeTemplateLocalSource")
            return frame:DescribeLocal()
            "#,
        )
        .unwrap();
    assert_eq!(result, "from-local-keyvalue");

    let _ = std::fs::remove_dir_all(&temp_dir);
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

#[test]
fn test_runtime_template_inherited_method_onload_append_chain_runs() {
    let t = load_test_xml(
        "runtime-template-inherited-method-onload-chain",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="RuntimeMethodScriptBaseTemplate" virtual="true" mixin="RuntimeMethodScriptBaseMixin">
                <Scripts>
                    <OnLoad method="OnLoad"/>
                </Scripts>
            </Frame>
            <Frame name="RuntimeMethodScriptChildTemplate" virtual="true" inherits="RuntimeMethodScriptBaseTemplate" mixin="RuntimeMethodScriptChildMixin">
                <Scripts>
                    <OnLoad method="OnLoad" inherit="append"/>
                </Scripts>
            </Frame>
        </Ui>
        "#,
    );

    t.env
        .exec(
            r#"
            RuntimeMethodScriptOrder = {}
            RuntimeMethodScriptBaseMixin = {
                OnLoad = function(self)
                    table.insert(RuntimeMethodScriptOrder, "base")
                end,
            }
            RuntimeMethodScriptChildMixin = {
                OnLoad = function(self)
                    table.insert(RuntimeMethodScriptOrder, "child")
                end,
            }

            local frame = CreateFrame("Frame", "RuntimeMethodScriptChainedFrame", UIParent, "RuntimeMethodScriptChildTemplate")
            collectgarbage("collect")
            local onLoad = frame:GetScript("OnLoad")
            assert(type(onLoad) == "function", "runtime template OnLoad handler should stay callable")
            onLoad(frame)
            assert(table.concat(RuntimeMethodScriptOrder, ",") == "child,base,child,base", "append chain should run child then base both on creation and direct call")
        "#,
        )
        .unwrap();
}
