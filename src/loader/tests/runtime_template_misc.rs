use super::*;

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
fn test_runtime_template_anchor_keeps_direct_offset_attributes() {
    let t = load_test_xml(
        "runtime-template-direct-offset",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="DirectOffsetTemplate" virtual="true">
                <Frames>
                    <Frame parentKey="Child">
                        <Size x="10" y="10"/>
                        <Anchors>
                            <Anchor point="BOTTOMLEFT" relativePoint="BOTTOMLEFT">
                                <Offset x="19" y="-30"/>
                            </Anchor>
                        </Anchors>
                    </Frame>
                </Frames>
            </Frame>
        </Ui>
        "#,
    );

    t.env
        .exec(
            r#"
            local parent = CreateFrame("Frame", "DirectOffsetTemplateParent", UIParent, "DirectOffsetTemplate")
            local point, relativeTo, relativePoint, x, y = parent.Child:GetPoint(1)
            assert(point == "BOTTOMLEFT", "point=" .. tostring(point))
            assert(relativePoint == "BOTTOMLEFT", "relativePoint=" .. tostring(relativePoint))
            assert(x == 19, "x=" .. tostring(x))
            assert(y == -30, "y=" .. tostring(y))
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
