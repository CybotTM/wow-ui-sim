#![cfg(feature = "client-ptr")]

use wow_ui_sim::lua_api::WowLuaEnv;

// Exact registration statements from the selected Blizzard PTR consumers.
#[test]
fn voice_transcription_registers_active_provider_event() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local self = CreateFrame("Frame")
        -- Blizzard_ChatFrame/Shared/VoiceChatTranscriptionButton.lua:4
        self:RegisterEvent("VOICE_CHAT_ACTIVE_VOICE_PROVIDER_CHANGED")
        assert(self:IsEventRegistered("VOICE_CHAT_ACTIVE_VOICE_PROVIDER_CHANGED"))
        self:UnregisterEvent("VOICE_CHAT_ACTIVE_VOICE_PROVIDER_CHANGED")
        assert(not self:IsEventRegistered("VOICE_CHAT_ACTIVE_VOICE_PROVIDER_CHANGED"))
        "#,
    )
    .unwrap();
}

#[test]
fn shard_transfer_toast_registers_imminent_event() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local self = CreateFrame("Frame")
        -- Blizzard_SocialToast/SocialToast.lua:36
        self:RegisterEvent("SHARD_TRANSFER_IMMINENT")
        assert(self:IsEventRegistered("SHARD_TRANSFER_IMMINENT"))
        self:UnregisterEvent("SHARD_TRANSFER_IMMINENT")
        assert(not self:IsEventRegistered("SHARD_TRANSFER_IMMINENT"))
        "#,
    )
    .unwrap();
}

#[test]
fn pet_happiness_indicator_registers_happiness_event() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local self = CreateFrame("Frame")
        -- Blizzard_FrameXML/PetHappiness.lua:30
        self:RegisterEvent("UNIT_HAPPINESS")
        assert(self:IsEventRegistered("UNIT_HAPPINESS"))
        self:UnregisterEvent("UNIT_HAPPINESS")
        assert(not self:IsEventRegistered("UNIT_HAPPINESS"))
        "#,
    )
    .unwrap();
}
