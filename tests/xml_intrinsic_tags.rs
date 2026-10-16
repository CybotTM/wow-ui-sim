//! XML tags must inherit the original cached intrinsic definition.
#![cfg(any(feature = "client-retail", feature = "client-wowforever"))]

use wow_ui_sim::lua_api::WowLuaEnv;

struct IntrinsicFixture(std::path::PathBuf);

impl Drop for IntrinsicFixture {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).expect("remove owned intrinsic fixture");
    }
}

fn load_intrinsic_fixture(env: &WowLuaEnv) -> IntrinsicFixture {
    let path = std::env::temp_dir().join(format!("wow-xml-intrinsic-{}", std::process::id()));
    std::fs::create_dir(&path).unwrap();
    let fixture = IntrinsicFixture(path);
    std::fs::write(
        fixture.0.join("IntrinsicTag.toc"),
        "## Title: IntrinsicTag\nIntrinsicTag.xml\n",
    )
    .unwrap();
    std::fs::write(fixture.0.join("IntrinsicTag.xml"), r#"
        <Ui xmlns="http://www.blizzard.com/wow/ui/">
            <Frame name="IntrinsicExplicitTemplate" virtual="true">
                <Size x="73" y="29"/>
            </Frame>
            <UIThemeContainerFrame name="IntrinsicTagTop" inherits="IntrinsicExplicitTemplate">
                <Scripts><OnLoad>self.instanceSawIntrinsic = type(self.fontStrings) == 'table'</OnLoad></Scripts>
            </UIThemeContainerFrame>
            <Frame name="IntrinsicTagParent">
                <Frames><UIThemeContainerFrame name="$parentChild" parentKey="Child"/></Frames>
            </Frame>
        </Ui>
    "#).unwrap();
    wow_ui_sim::loader::load_addon(&env.loader_env(), &fixture.0.join("IntrinsicTag.toc")).unwrap();
    fixture
}

#[test]
fn original_cached_intrinsic_tag_applies_mixin_and_preload_at_top_level_and_in_children() {
    let cache = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    let (env, loaded) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &cache,
        &["Blizzard_AccessibilityTemplates"],
        &[],
    );
    assert!(
        loaded
            .iter()
            .any(|name| name == "Blizzard_AccessibilityTemplates")
    );
    let _fixture = load_intrinsic_fixture(&env);
    env.exec(r#"
        assert(type(UIThemeContainerMixin.UpdateTheme) == 'function')
        assert(IntrinsicTagTop.UpdateTheme == UIThemeContainerMixin.UpdateTheme,
            'XML intrinsic tag must receive original cached mixin')
        assert(IntrinsicTagTop.cvar == 'questTextContrast')
        assert(type(IntrinsicTagTop.fontStrings) == 'table' and type(IntrinsicTagTop.frames) == 'table')
        assert(IntrinsicTagTop.instanceSawIntrinsic == true, 'intrinsic preload before instance OnLoad')
        assert(IntrinsicTagTop:GetWidth() == 73 and IntrinsicTagTop:GetHeight() == 29)
        assert(IntrinsicTagParent.Child == IntrinsicTagParentChild)
        assert(IntrinsicTagParentChild.UpdateTheme == UIThemeContainerMixin.UpdateTheme)
        assert(IntrinsicTagParentChild.cvar == 'questTextContrast')
        assert(type(IntrinsicTagParentChild.fontStrings) == 'table')
        local frame = CreateFrame('Frame', 'IntrinsicCreateFrameControl', UIParent, 'UIThemeContainerFrame')
        assert(frame.UpdateTheme == UIThemeContainerMixin.UpdateTheme)
        assert(frame.cvar == 'questTextContrast' and type(frame.fontStrings) == 'table')
    "#).unwrap();
}
