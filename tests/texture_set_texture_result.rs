//! Shared SetTexture application-result contract, without GPU or pixel loading.
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn set_texture_reports_one_boolean_for_known_and_missing_paths() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local texture = UIParent:CreateTexture()
        local function expect(asset, expected)
            local values = {texture:SetTexture(asset)}
            assert(#values == 1, 'SetTexture must return exactly one result')
            assert(type(values[1]) == 'boolean' and values[1] == expected)
        end
        expect('Interface\\LFGFrame\\UI-LFG-BACKGROUND-HEROIC', true)
        assert(texture:GetTexture() == 340816)
        expect('Interface\\ResidualMissingTextureResult', false)
        expect('Interface\\LFGFrame\\UI-LFG-BACKGROUND-HEROIC', true)
        expect(nil, false)
        assert(texture:GetTexture() == nil)
        expect('', false)
        assert(texture:GetTexture() == nil)
    "#,
    )
    .unwrap();
}

#[test]
fn set_texture_reports_known_file_ids_and_rejects_unknown_ids() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local texture = UIParent:CreateTexture()
        assert(texture:SetTexture(340816) == true)
        assert(texture:GetTexture() == 340816)
        assert(texture:SetTexture('340816') == true)
        assert(texture:SetTexture(4294967295) == false)
        assert(texture:SetTexture(0) == false)
        assert(texture:GetTexture() == nil)
    "#,
    )
    .unwrap();
}

#[test]
fn set_texture_resolves_addon_files_without_reading_pixels() {
    let env = WowLuaEnv::new().unwrap();
    let root = tempfile::tempdir().unwrap();
    let addon = root.path().join("TextureResultProbe");
    std::fs::create_dir(&addon).unwrap();
    // Resolution tests existence, not decoding: no image pixels are needed.
    std::fs::write(addon.join("present.tga"), []).unwrap();
    std::fs::create_dir(addon.join("directory.tga")).unwrap();
    env.state()
        .borrow_mut()
        .addon_base_paths
        .push(root.path().to_path_buf());
    env.exec(
        r#"
        local texture = UIParent:CreateTexture()
        assert(texture:SetTexture('Interface\\AddOns\\TextureResultProbe\\present') == true)
        assert(texture:SetTexture('Interface\\AddOns\\TextureResultProbe\\missing') == false)
        assert(texture:SetTexture('Interface\\AddOns\\TextureResultProbe\\directory') == false)
    "#,
    )
    .unwrap();
}
