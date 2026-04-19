use std::io::Write;

use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::xml::clear_templates;

fn create_test_addon(xml: &str, addon_name: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let toc_path = dir.path().join(format!("{addon_name}.toc"));
    let xml_path = dir.path().join(format!("{addon_name}.xml"));
    let mut toc = std::fs::File::create(&toc_path).unwrap();
    writeln!(toc, "## Title: {addon_name}").unwrap();
    writeln!(toc, "{}.xml", addon_name).unwrap();
    std::fs::write(xml_path, xml).unwrap();
    dir
}

#[test]
fn xml_animation_group_onload_hides_target_textures() {
    clear_templates();
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r##"
        XmlAnimTargetMixin = {}
        function XmlAnimTargetMixin:Show()
            self:SetTargetsShown(true, self:GetAnimations())
        end
        function XmlAnimTargetMixin:Hide()
            self:SetTargetsShown(false, self:GetAnimations())
        end
        function XmlAnimTargetMixin:SetTargetsShown(shown, ...)
            for i = 1, select("#", ...) do
                local anim = select(i, ...)
                local target = anim and anim:GetTarget()
                if target and target.SetShown then
                    target:SetShown(shown)
                end
            end
        end
    "##,
    )
    .unwrap();

    let addon = create_test_addon(
        r#"<Ui>
        <AnimationGroup name="XmlAnimTargetTemplate" mixin="XmlAnimTargetMixin" virtual="true">
            <Scripts><OnLoad method="Hide"/></Scripts>
        </AnimationGroup>
        <Frame name="XmlAnimTargetFrame" parent="UIParent">
            <Layers>
                <Layer level="ARTWORK">
                    <Texture parentKey="Pulse" file="Interface\Icons\INV_Misc_QuestionMark" setAllPoints="true"/>
                </Layer>
            </Layers>
            <Animations>
                <AnimationGroup parentKey="PulseAnim" inherits="XmlAnimTargetTemplate">
                    <Alpha childKey="Pulse" order="1" fromAlpha="0" toAlpha="1" duration="1"/>
                </AnimationGroup>
            </Animations>
        </Frame>
    </Ui>"#,
        "XmlAnimTargetOnLoad",
    );

    load_addon(
        &env.loader_env(),
        &addon.path().join("XmlAnimTargetOnLoad.toc"),
    )
    .unwrap();

    let hidden: bool = env
        .eval("return XmlAnimTargetFrame.Pulse:IsShown() == false")
        .unwrap();
    assert!(
        hidden,
        "XML animation-group OnLoad should hide child targets before play"
    );
}
