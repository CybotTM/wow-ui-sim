#![cfg(feature = "client-ptr")]

use std::path::Path;
use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn load_selected_consumers() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("create Lua environment");
    env.set_screen_mode(ScreenKind::Game);
    let addons_dir = wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )));
    env.state().borrow_mut().addon_base_paths = vec![addons_dir.clone()];
    wow_ui_sim::xml::register_intrinsic_templates();
    let addons = discover_blizzard_addons_for_screen(&addons_dir, ScreenKind::Game);
    for required in [
        "Blizzard_ChatFrame",
        "Blizzard_SocialToast",
        "Blizzard_FrameXML",
    ] {
        assert!(
            addons.iter().any(|(name, _)| name == required),
            "{required} not selected"
        );
    }
    for (name, toc) in addons {
        load_addon(&env.loader_env(), &toc).unwrap_or_else(|error| panic!("load {name}: {error}"));
    }
    env
}

fn assert_consumer_registers(mixin: &str, event: &str) {
    let env = load_selected_consumers();
    let lua = format!(
        r#"
        local frame = CreateFrame("Frame")
        local consumer = assert({mixin}, "consumer not loaded by selected TOC")
        local ok, trace = xpcall(function() consumer.OnLoad(frame) end, debug.traceback)
        assert(frame:IsEventRegistered("{event}"), trace or "consumer did not register event")
    "#
    );
    env.exec(&lua)
        .expect("original consumer must register its event");
}

#[test]
fn voice_transcription_registers_active_provider_event() {
    assert_consumer_registers(
        "VoiceChatTranscriptionButtonMixin",
        "VOICE_CHAT_ACTIVE_VOICE_PROVIDER_CHANGED",
    );
}

#[test]
fn shard_transfer_toast_registers_imminent_event() {
    assert_consumer_registers("ShardTransferImminentMixin", "SHARD_TRANSFER_IMMINENT");
}

#[test]
fn pet_happiness_indicator_registers_happiness_event() {
    assert_consumer_registers("PetHappinessIndicatorMixin", "UNIT_HAPPINESS");
}
