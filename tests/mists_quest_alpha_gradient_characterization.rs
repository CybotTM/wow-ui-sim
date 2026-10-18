//! Explicit Mists-cache callback characterization, not automatic source selection.
#![cfg(feature = "client-mists")]

#[path = "common/quest_alpha_gradient.rs"]
mod support;

use wow_ui_sim::lua_api::WowLuaEnv;

fn create_env() -> WowLuaEnv {
    let env = support::create_env();
    support::load_cached_lua(
        &env,
        "mists/AddOns/Blizzard_SharedXML/Wrath/SoundKitConstants.lua",
        "184a74f6e0b7ae04646ce54db0ba37fb105217adf3a6fc500b1a348a772128bf",
    );
    support::load_cached_lua(
        &env,
        "mists/AddOns/Blizzard_UIPanels_Game/Classic/QuestFrame.lua",
        "0e44a7710ffc15a302a3ae061a2c6c7c534248b2b09399ba53cb1c5f786a69df",
    );
    support::load_cached_lua(
        &env,
        "mists/AddOns/Blizzard_UIPanels_Game/Wrath/QuestInfo.lua",
        "cf15335d06ebc347b9aa4e1052434a5ff6e926c17a8618e809565a5b09357f57",
    );
    support::exec(
        &env,
        include_str!("fixtures/quest_alpha_gradient/mists.lua"),
    );
    env
}

#[test]
fn cached_mists_gradual_callback_immediately_reveals_dependents_at_completion() {
    let env = create_env();
    support::exec(&env, "RunMistsGradualGradientCharacterization()");
}

#[test]
fn cached_mists_callback_characterizes_supplied_1024_progress() {
    let env = create_env();
    support::exec(&env, "RunMistsInstantGradientCharacterization()");
}
