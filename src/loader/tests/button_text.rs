use super::*;

#[test]
fn test_inherited_button_text_defaults_to_center_anchor_when_anchors_omitted() {
    let t = load_test_xml(
        "test-inherited-button-text-default-anchor",
        r#"<Ui>
            <Button name="ButtonTextCenterTemplate" virtual="true">
                <ButtonText name="$parentText"/>
            </Button>
            <Button name="ButtonTextCenterButton" parent="UIParent" inherits="ButtonTextCenterTemplate" text="APPLY">
                <Size x="160" y="24"/>
                <Anchors><Anchor point="CENTER"/></Anchors>
            </Button>
        </Ui>"#,
    );

    t.assert_lua_true(
        r#"
        return (function()
            local btn = ButtonTextCenterButton
            local fs = btn and btn:GetFontString()
            if not btn or not fs then return false end
            local point, relativeTo, relativePoint, x, y = fs:GetPoint(1)
            if point ~= "CENTER" or relativeTo ~= btn or relativePoint ~= "CENTER" then
                return false
            end
            if (x or 0) ~= 0 or (y or 0) ~= 0 then
                return false
            end
            local left, right = fs:GetLeft(), fs:GetRight()
            local bleft, bright = btn:GetLeft(), btn:GetRight()
            if not left or not right or not bleft or not bright then
                return false
            end
            local fsCenter = (left + right) / 2
            local btnCenter = (bleft + bright) / 2
            return math.abs(fsCenter - btnCenter) < 0.6
        end)()
        "#,
        "ButtonText without explicit anchors should default to centered anchoring on its button",
    );
}
