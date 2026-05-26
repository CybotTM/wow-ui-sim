use super::*;

#[test]
fn inherited_layer_parent_keys_exist_before_inherited_onload() {
    let t = load_test_xml(
        "inherited-layer-parent-keys-before-onload",
        r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Script>
                InheritedLayerTemplateMixin = {}
                function InheritedLayerTemplateMixin:OnLoad()
                    self.sawBackgroundOnLoad = self.Background ~= nil
                    self.sawBackgroundArrayOnLoad = self.specBackgrounds ~= nil and #self.specBackgrounds == 2
                end
                InheritedLayerChildMixin = {}
                function InheritedLayerChildMixin:OnLoad()
                    local parent = self:GetParent()
                    parent.sawBackgroundArrayBeforeChildOnLoad = parent.specBackgrounds ~= nil and #parent.specBackgrounds == 2
                end
            </Script>
            <Frame name="InheritedLayerTemplate" mixin="InheritedLayerTemplateMixin" virtual="true">
                <Layers>
                    <Layer level="BACKGROUND">
                        <Texture parentKey="Background" parentArray="specBackgrounds"/>
                        <Texture parentKey="OverlayBackground" parentArray="specBackgrounds"/>
                    </Layer>
                </Layers>
                <Frames>
                    <Frame parentKey="ChildObserver" mixin="InheritedLayerChildMixin">
                        <Scripts>
                            <OnLoad method="OnLoad"/>
                        </Scripts>
                    </Frame>
                </Frames>
                <Scripts>
                    <OnLoad method="OnLoad"/>
                </Scripts>
            </Frame>
            <Frame name="InheritedLayerInstance" parent="UIParent" inherits="InheritedLayerTemplate"/>
            <Frame name="InheritedLayerHost" parent="UIParent">
                <Frames>
                    <Frame parentKey="Child" inherits="InheritedLayerTemplate" hidden="true" frameLevel="100"/>
                </Frames>
            </Frame>
        </Ui>
        "#,
    );

    t.assert_lua_true(
        "return InheritedLayerInstance.Background ~= nil",
        "inherited layer parentKey should attach to the XML instance",
    );
    let array_signature: String = t
        .env
        .eval(
            r#"
            return type(InheritedLayerInstance.specBackgrounds)
                .. ":" .. tostring(InheritedLayerInstance.specBackgrounds and #InheritedLayerInstance.specBackgrounds)
            "#,
        )
        .unwrap();
    assert_eq!(
        array_signature, "table:2",
        "inherited layer parentArray should attach to the XML instance"
    );
    t.assert_lua_true(
        "return InheritedLayerInstance.sawBackgroundOnLoad and InheritedLayerInstance.sawBackgroundArrayOnLoad",
        "inherited OnLoad should run after inherited layer parentKey and parentArray wiring",
    );
    t.assert_lua_true(
        "return InheritedLayerInstance.sawBackgroundArrayBeforeChildOnLoad",
        "inherited child OnLoad should run after parent inherited layer parentArray wiring",
    );
    t.assert_lua_true(
        "return InheritedLayerHost.Child.Background ~= nil",
        "anonymous XML child should retain inherited layer parentKeys through its parentKey reference",
    );
    t.assert_lua_true(
        "return InheritedLayerHost.Child.specBackgrounds ~= nil and #InheritedLayerHost.Child.specBackgrounds == 2",
        "anonymous XML child should retain inherited layer parentArrays through its parentKey reference",
    );
}
