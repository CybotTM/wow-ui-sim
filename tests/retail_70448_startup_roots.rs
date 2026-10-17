//! Producer regressions for original Retail 12.1.5.70448 startup consumers.
#![cfg(feature = "client-retail")]
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn original_shared_unit_frame_util_loads_with_real_texture_metatable() {
    let env = WowLuaEnv::new().unwrap();
    let root = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    let source = root.join("Blizzard_UnitFrame/Shared/UnitFrameUtil.lua");
    env.exec(&std::fs::read_to_string(source).unwrap()).unwrap();
    env.exec(
        r#"
        local texture = CreateFrame("Frame"):CreateTexture()
        local mt = GetTextureMetatable(texture)
        assert(mt == GetTextureMetatable())
        local methods = CopyTable(mt.__index)
        methods.SetTexture(texture, 134400)
        assert(texture:GetTexture() == 134400)
        assert(type(UnitFrameUtil.UpdateUnitFrameRoleIcon) == "function")
        assert(type(UnitFrameUtil.GetUnitPvPIndicatorDisplayInfo) == "function")
        "#,
    )
    .unwrap();
}

#[test]
fn original_arena_spec_display_accepts_secret_atlas_and_preserves_read_policy() {
    let env = WowLuaEnv::new().unwrap();
    let root = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    env.exec(
        &std::fs::read_to_string(root.join("Blizzard_UnitFrame/Shared/UnitFrameUtil.lua")).unwrap(),
    )
    .unwrap();
    env.exec(
        r#"
        local texture = CreateFrame("Frame"):CreateTexture()
        UnitFrameUtil.UpdateArenaOpponentSpecDisplay({roleIcon = texture}, 1)
        texture:SetAtlas(secretwrap("GM-icon-menu"))
        assert(texture:GetAtlas() == "GM-icon-menu")
        local wrapped = secretwrap("GM-icon-settings")
        local function addon()
            assert(not pcall(texture.SetAtlas, texture, wrapped))
            assert(not pcall(texture.GetAtlas, texture))
        end
        debug.setobjecttaint(addon, "SecretAtlasProbe")
        addon()
        assert(texture:GetAtlas() == "GM-icon-menu")
        texture:SetAtlas("GM-icon-settings")
        local function public() return texture:GetAtlas() end
        debug.setobjecttaint(public, "SecretAtlasProbe")
        assert(public() == "GM-icon-settings")
        "#,
    )
    .unwrap();
}

#[test]
fn combat_audio_pulse_range_matches_70448_documented_command_bounds() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local values = Enum.CombatAudioAlertPulsePercentValues
        local meta = Enum.CombatAudioAlertPulsePercentValuesMeta
        assert(meta.MinValue == 0 and meta.MaxValue == 9 and meta.NumValues == 10)
        assert(values.Off == 0 and values.Under10Percent == 9)
        local count = 0
        for _, value in pairs(values) do
            assert(value >= meta.MinValue and value <= meta.MaxValue)
            count = count + 1
        end
        assert(count == meta.NumValues)
        "#,
    )
    .unwrap();
}

#[test]
fn age_verification_event_registers_delivers_and_rejects_invented_suffix() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        received = 0
        listener = CreateFrame("Frame")
        listener:SetScript("OnEvent", function(self, event)
            assert(self == listener and event == "ALERT_AGE_VERIFICATION_RESTRICTED")
            received = received + 1
        end)
        assert(listener:RegisterEvent("ALERT_AGE_VERIFICATION_RESTRICTED"))
        assert(listener:IsEventRegistered("ALERT_AGE_VERIFICATION_RESTRICTED"))
        assert(not pcall(listener.RegisterEvent, listener, "ALERT_AGE_VERIFICATION_RESTRICTED_INVENTED"))
        "#,
    )
    .unwrap();
    env.fire_event("ALERT_AGE_VERIFICATION_RESTRICTED").unwrap();
    assert_eq!(env.eval::<i32>("return received").unwrap(), 1);
    env.exec("listener:UnregisterEvent('ALERT_AGE_VERIFICATION_RESTRICTED')")
        .unwrap();
    env.fire_event("ALERT_AGE_VERIFICATION_RESTRICTED").unwrap();
    assert_eq!(env.eval::<i32>("return received").unwrap(), 1);
}
