//! Bounded empty-state consumers; no populated combo/overlay policy or native parity.
#![cfg(feature = "client-wrath")]
#[path = "wrath_unmasked_support/mod.rs"]
mod support;
use support::{env, load, run};

#[test]
fn wrath_unmasked_original_combo_frame_hides_without_target_points() {
    let env = env();
    load(&env, &["Constants.lua", "ComboFrame.lua"]);
    run(
        &env,
        r#"
        PlayerFrame = CreateFrame('Frame', 'ResidualPlayerFrame', UIParent)
        PlayerFrame.unit = 'player'
        ComboFrame = CreateFrame('Frame', 'ResidualComboFrame', PlayerFrame)
        ComboPoint1Highlight = ComboFrame:CreateTexture()
        ComboPoint1Shine = ComboFrame:CreateTexture()
        ComboPoint1Highlight:SetAlpha(1)
        ComboPoint1Shine:SetAlpha(1)
        ComboFrame:Show()
        assert(not UnitExists('target'))
        ComboFrame_OnEvent(ComboFrame, 'PLAYER_TARGET_CHANGED')
        assert(not ComboFrame:IsShown())
        assert(ComboPoint1Highlight:GetAlpha() == 0)
        assert(ComboPoint1Shine:GetAlpha() == 0)
        assert(COMBO_FRAME_LAST_NUM_POINTS == 0)
        assert(GetComboPoints('player', 'target') == 0)
        "#,
    );
}

#[test]
fn wrath_unmasked_original_worldmap_empty_overlays_hide_stale_textures() {
    let env = env();
    load(&env, &["Constants.lua", "WorldMapFrame.lua"]);
    run(
        &env,
        r#"
        WorldMapButton = CreateFrame('Button', 'ResidualWorldMapButton', UIParent)
        WorldMapDetailFrame = CreateFrame('Frame', 'ResidualWorldMapDetail', UIParent)
        OutlandButton = CreateFrame('Button', nil, WorldMapButton)
        AzerothButton = CreateFrame('Button', nil, WorldMapButton)
        WorldMapZoomOutButton = CreateFrame('Button', nil, WorldMapButton)
        for i=1, NUM_WORLDMAP_DETAIL_TILES do
            _G['WorldMapDetailTile'..i] = WorldMapDetailFrame:CreateTexture()
        end
        NUM_WORLDMAP_OVERLAYS = 2
        for i=1,2 do
            _G['WorldMapOverlay'..i] = WorldMapDetailFrame:CreateTexture()
            _G['WorldMapOverlay'..i]:Show()
        end
        -- Execute the complete unchanged function. The known next unsupported
        -- debug-map capability is not replaced or credited as repaired here.
        local ok, err = pcall(WorldMapFrame_Update)
        assert(not WorldMapOverlay1:IsShown(), tostring(err))
        assert(not WorldMapOverlay2:IsShown(), tostring(err))
        assert(GetNumMapOverlays() == 0)
        assert(not ok and string.find(tostring(err), 'HasDebugZoneMap', 1, true), tostring(err))
        "#,
    );
}
