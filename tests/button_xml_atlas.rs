//! Tests for button textures defined in XML with atlas attributes.
//!
//! When a button's XML defines `<NormalTexture atlas="..."/>`, the XML loader
//! must ensure the texture child exists BEFORE running Lua code that calls
//! `GetNormalTexture()`. Without this, `GetNormalTexture()` returns nil and
//! OnLoad code that does `self:GetNormalTexture():SetRotation(...)` errors.

use std::io::Write;

use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

/// Create a temporary addon with a Button that has NormalTexture with atlas.
/// Returns the path to the TOC file.
fn create_test_addon_with_atlas_button() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let addon_dir = dir.path();

    // TOC file
    let toc_path = addon_dir.join("TestAtlasButton.toc");
    let mut toc = std::fs::File::create(&toc_path).unwrap();
    writeln!(toc, "## Title: TestAtlasButton").unwrap();
    writeln!(toc, "TestAtlasButton.xml").unwrap();

    // XML file with a Button that has NormalTexture, PushedTexture, HighlightTexture with atlas
    let xml_path = addon_dir.join("TestAtlasButton.xml");
    let mut xml = std::fs::File::create(&xml_path).unwrap();
    writeln!(
        xml,
        r#"<Ui>
    <Button name="TestAtlasBtn" parent="UIParent">
        <Size x="32" y="32"/>
        <NormalTexture atlas="bag-arrow"/>
        <PushedTexture atlas="bag-arrow"/>
        <HighlightTexture atlas="bag-arrow"/>
    </Button>
</Ui>"#
    )
    .unwrap();

    dir
}

/// Button loaded from XML with `<NormalTexture atlas="..."/>` should have
/// GetNormalTexture() return a valid texture object.
#[test]
fn test_xml_button_atlas_textures_exist() {
    let env = WowLuaEnv::new().unwrap();
    let dir = create_test_addon_with_atlas_button();
    let toc_path = dir.path().join("TestAtlasButton.toc");

    load_addon(&env.loader_env(), &toc_path).expect("addon load should succeed");

    // GetNormalTexture should return a valid texture, not nil
    let has_normal: bool = env
        .eval("return TestAtlasBtn:GetNormalTexture() ~= nil")
        .unwrap();
    assert!(
        has_normal,
        "XML Button with <NormalTexture atlas='...'/> should have GetNormalTexture() ~= nil"
    );

    let has_pushed: bool = env
        .eval("return TestAtlasBtn:GetPushedTexture() ~= nil")
        .unwrap();
    assert!(
        has_pushed,
        "XML Button with <PushedTexture atlas='...'/> should have GetPushedTexture() ~= nil"
    );

    let has_highlight: bool = env
        .eval("return TestAtlasBtn:GetHighlightTexture() ~= nil")
        .unwrap();
    assert!(
        has_highlight,
        "XML Button with <HighlightTexture atlas='...'/> should have GetHighlightTexture() ~= nil"
    );
}
