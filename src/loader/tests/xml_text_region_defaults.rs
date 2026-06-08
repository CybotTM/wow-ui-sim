use super::*;

#[test]
fn button_text_without_anchors_uses_justify_h_default_point() {
    let ctx = load_test_xml(
        "button-text-justify-h-default-anchor",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Button name="DefaultButtonTextAnchorLeft" parent="UIParent">
                <ButtonText name="$parentText" inherits="GameFontNormal" text="BL" justifyH="LEFT" justifyV="TOP"/>
            </Button>
            <Button name="DefaultButtonTextAnchorCenter" parent="UIParent">
                <ButtonText name="$parentText" inherits="GameFontNormal" text="BC" justifyH="CENTER" justifyV="MIDDLE"/>
            </Button>
            <Button name="DefaultButtonTextAnchorRight" parent="UIParent">
                <ButtonText name="$parentText" inherits="GameFontNormal" text="BR" justifyH="RIGHT" justifyV="BOTTOM"/>
            </Button>
            <Button name="DefaultButtonTextAnchorTopOnly" parent="UIParent">
                <ButtonText name="$parentText" inherits="GameFontNormal" text="BT" justifyH="LEFT" justifyV="BOTTOM">
                    <Anchors><Anchor point="TOP"/></Anchors>
                </ButtonText>
            </Button>
        </Ui>
        "#,
    );

    ctx.assert_lua_str(
        r#"
        return (function()
            local cases = {
                { DefaultButtonTextAnchorLeftText, DefaultButtonTextAnchorLeft, "LEFT" },
                { DefaultButtonTextAnchorCenterText, DefaultButtonTextAnchorCenter, "CENTER" },
                { DefaultButtonTextAnchorRightText, DefaultButtonTextAnchorRight, "RIGHT" },
                { DefaultButtonTextAnchorTopOnlyText, DefaultButtonTextAnchorTopOnly, "TOP" },
            }

            local results = {}
            for index, case in ipairs(cases) do
                local region, parent, expected = case[1], case[2], case[3]
                local point, relativeTo, relativePoint, x, y = region:GetPoint(1)
                results[index] = tostring(
                    region:GetNumPoints() == 1
                        and point == expected
                        and relativeTo == parent
                        and relativePoint == expected
                        and x == 0
                        and y == 0
                )
            end

            return table.concat(results, "|")
        end)()
        "#,
        "true|true|true|true",
    );
}

#[test]
fn editbox_xml_text_insets_do_not_anchor_backing_fontstring() {
    let ctx = load_test_xml(
        "editbox-text-insets-fontstring-anchor",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <EditBox name="DefaultEditBoxTextInsets" parent="UIParent">
                <Size><AbsDimension x="180" y="32"/></Size>
                <TextInsets left="7" right="11" top="13" bottom="17"/>
                <FontString name="$parentText" inherits="GameFontNormal" text="EditText" justifyH="RIGHT"/>
            </EditBox>
        </Ui>
        "#,
    );

    ctx.assert_lua_str(
        r#"
        return (function()
            local left, right, top, bottom = DefaultEditBoxTextInsets:GetTextInsets()
            local region = DefaultEditBoxTextInsets:GetRegions()
            return table.concat({
                tostring(left == 7),
                tostring(right == 11),
                tostring(top == 13),
                tostring(bottom == 17),
                tostring(region ~= nil and region:GetObjectType() == "FontString"),
                tostring(region ~= nil and region:GetNumPoints() == 0),
                tostring(DefaultEditBoxTextInsetsText == nil),
            }, "|")
        end)()
        "#,
        "true|true|true|true|true|true|true",
    );
}
