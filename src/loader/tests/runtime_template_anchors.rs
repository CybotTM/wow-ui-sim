use super::load_test_xml;

#[test]
fn test_xml_texture_gradient_sets_fade_stops() {
    let t = load_test_xml(
        "xml-texture-gradient-sets-fade-stops",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="GradientTextureHost">
                <Layers>
                    <Layer level="BORDER">
                        <Texture name="$parentGradient" parentKey="gradient">
                            <Size x="100" y="40"/>
                            <Color r="0.306" g="0.133" b="0.031"/>
                            <Gradient orientation="VERTICAL">
                                <MaxColor r="1" g="1" b="1" a="0.8"/>
                                <MinColor r="1" g="1" b="1" a="0.0"/>
                            </Gradient>
                        </Texture>
                    </Layer>
                </Layers>
            </Frame>
        </Ui>
        "#,
    );

    let state = t.env.state().borrow();
    let gradient_id = state
        .widgets
        .get_id_by_name("GradientTextureHostGradient")
        .expect("gradient texture should exist");
    let gradient_frame = state.widgets.get(gradient_id).unwrap();
    let gradient = gradient_frame
        .gradient
        .expect("XML Gradient should be applied to the texture");

    assert!(gradient.vertical, "orientation should be vertical");
    assert_eq!(gradient.min_color.a, 0.0);
    assert_eq!(gradient.max_color.a, 0.8);
}

#[test]
fn test_anonymous_layer_relative_to_uses_substituted_parent_name() {
    let t = load_test_xml(
        "anonymous-layer-relative-to-substituted-parent",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="AnonymousLayerAnchorHost">
                <Frames>
                    <Frame setAllPoints="true">
                        <Layers>
                            <Layer level="OVERLAY">
                                <Texture name="$parentBLCorner">
                                    <Size x="64" y="64"/>
                                    <Anchors>
                                        <Anchor point="BOTTOMLEFT" x="3" y="3"/>
                                    </Anchors>
                                </Texture>
                                <Texture name="$parentBottomLine">
                                    <Size x="80" y="43"/>
                                    <Anchors>
                                        <Anchor point="BOTTOMLEFT" relativeTo="$parentBLCorner" relativePoint="BOTTOMRIGHT" x="0" y="0"/>
                                    </Anchors>
                                </Texture>
                            </Layer>
                        </Layers>
                    </Frame>
                </Frames>
            </Frame>
        </Ui>
        "#,
    );

    t.env
        .exec(
            r#"
            assert(AnonymousLayerAnchorHostBLCorner ~= nil, "expected substituted corner global")
            assert(AnonymousLayerAnchorHostBottomLine ~= nil, "expected substituted bottom line global")
            local point, relativeTo, relativePoint, x, y = AnonymousLayerAnchorHostBottomLine:GetPoint(1)
            assert(point == "BOTTOMLEFT", "point=" .. tostring(point))
            assert(relativeTo == AnonymousLayerAnchorHostBLCorner, "bottom line should anchor to substituted corner texture")
            assert(relativePoint == "BOTTOMRIGHT", "relativePoint=" .. tostring(relativePoint))
            assert(x == 0, "x=" .. tostring(x))
            assert(y == 0, "y=" .. tostring(y))
        "#,
        )
        .unwrap();
}
