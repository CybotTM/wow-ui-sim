//! T6 documented service contracts; anonymous Forever timer attribution remains unknown.
#![cfg(not(feature = "client-wrath"))]

use crate::cfactory_support::{documented_env, load_original};

#[cfg(any(feature = "client-retail", feature = "client-ptr"))]
#[test]
fn cfactory_t6_error_messages_reach_console_sink() {
    let env = documented_env("LogDocumentation.lua", "LogErrorMessage");
    env.exec("C_Log.LogErrorMessage('cfactory service diagnostic')")
        .unwrap();
    assert!(
        env.state()
            .borrow()
            .console_output
            .iter()
            .any(|line| line == "cfactory service diagnostic")
    );
}

#[cfg(any(feature = "client-retail", feature = "client-ptr"))]
#[test]
fn cfactory_t6_secure_combat_filter_reset_contract() {
    let env = documented_env("CombatLogSecureDocumentation.lua", "ClearEventFilters");
    env.exec("assert(select('#', C_CombatLogSecure.ClearEventFilters()) == 0)")
        .unwrap();
}

#[cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever"
))]
#[test]
fn cfactory_t6_collectable_tracking_documented_boolean() {
    let env = documented_env(
        "ContentTrackingDocumentation.lua",
        "GetCollectableSourceTrackingEnabled",
    );
    env.exec("assert(type(C_ContentTracking.GetCollectableSourceTrackingEnabled()) == 'boolean')")
        .unwrap();
}

#[test]
fn cfactory_t6_tts_chat_type_disabled_without_preference_backend() {
    let env = documented_env("TTSSettingsDocumentation.lua", "GetChatTypeEnabled");
    env.exec("assert(C_TTSSettings.GetChatTypeEnabled('SAY') == false); assert(C_TTSSettings.GetChatTypeEnabled('PARTY') == false)").unwrap();
}

#[cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever"
))]
#[test]
fn cfactory_t6_voice_login_documented_boolean() {
    let env = documented_env("VoiceChatDocumentation.lua", "IsLoggedIn");
    env.exec("assert(type(C_VoiceChat.IsLoggedIn()) == 'boolean')")
        .unwrap();
}

#[cfg(any(feature = "client-era", feature = "client-anniversary"))]
#[test]
fn cfactory_t6_speak_for_me_reads_existing_activity() {
    let env = documented_env("VoiceChatDocumentation.lua", "IsSpeakForMeActive");
    env.exec("assert(C_VoiceChat.IsSpeakForMeActive() == false)")
        .unwrap();
    env.state().borrow_mut().voice_chat.is_speak_for_me_active = true;
    env.exec("assert(C_VoiceChat.IsSpeakForMeActive() == true)")
        .unwrap();
}

#[cfg(any(feature = "client-era", feature = "client-anniversary"))]
#[test]
fn cfactory_t6_corpse_position_absent_without_location_model() {
    let env = documented_env("DeathInfoDocumentation.lua", "GetCorpseMapPosition");
    env.exec("assert(C_DeathInfo.GetCorpseMapPosition(84) == nil)")
        .unwrap();
}

#[cfg(feature = "client-wowforever")]
#[test]
fn cfactory_t6_original_preferred_locale_dropdown_handles_no_catalog() {
    let env = wow_ui_sim::lua_api::WowLuaEnv::new().unwrap();
    load_original(&env, "Blizzard_Communities/GuildPreferredPlaySettings.lua");
    env.exec("CommunitiesGuildPreferredPlaySettingsFrameMixin.InitializeLocaleDropdown({}, 1)")
        .unwrap();
}

#[cfg(feature = "client-wowforever")]
#[test]
fn cfactory_t6_discord_voice_documented_boolean() {
    let env = documented_env("DiscordDocumentation.lua", "IsVoiceEnabled");
    env.exec("assert(type(C_Discord.IsVoiceEnabled()) == 'boolean')")
        .unwrap();
}

#[cfg(feature = "client-wowforever")]
#[test]
fn cfactory_t6_voice_provider_documented_enum_number() {
    let env = documented_env("VoiceChatDocumentation.lua", "GetActiveVoiceProviderID");
    env.exec("assert(type(C_VoiceChat.GetActiveVoiceProviderID()) == 'number')")
        .unwrap();
}
