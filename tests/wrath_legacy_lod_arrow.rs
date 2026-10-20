#![cfg(feature = "client-wrath")]
use crate::wrath_legacy_lod_apis::original_lines;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn arrow_original_49_and_lazy_children() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("BattlefieldMinimap = CreateFrame('Frame', 'BattlefieldMinimap'); WorldMapParty1 = CreateFrame('Frame', 'WorldMapParty1')").unwrap();
    env.exec(&original_lines("Blizzard_BattlefieldMinimap", 38, 55))
        .unwrap();
    env.exec(r#"
        BattlefieldMinimap_OnLoad(BattlefieldMinimap)
        assert(PlayerMiniArrowFrame:GetObjectType() == 'Model')
        assert(PlayerMiniArrowEffectFrame:GetObjectType() == 'Model')
        assert(PlayerMiniArrowFrame:GetParent() == BattlefieldMinimap)
        assert(PlayerMiniArrowEffectFrame:GetParent() == BattlefieldMinimap)
        local arrow, effect = PlayerMiniArrowFrame, PlayerMiniArrowEffectFrame
        local other = CreateFrame('Frame')
        assert(select('#', CreateMiniWorldMapArrowFrame(other)) == 0)
        assert(arrow == PlayerMiniArrowFrame and effect == PlayerMiniArrowEffectFrame)
        assert(arrow:GetParent() == BattlefieldMinimap and effect:GetParent() == BattlefieldMinimap)
        local cases = {
            {false, 'Usage: CreateMiniWorldMapArrowFrame(parent)'},
            {{}, "CreateMiniWorldMapArrowFrame(): Couldn't find 'this' in parent object"},
            {BattlefieldMinimap:CreateTexture(), 'CreateMiniWorldMapArrowFrame(): Wrong object type, expected frame'},
        }
        for _, case in ipairs(cases) do
            local ok, err = pcall(CreateMiniWorldMapArrowFrame, case[1])
            assert(not ok and string.find(err, case[2], 1, true), tostring(err))
        end
    "#).unwrap();
}
