//! T4 original Wrath callback coverage; unexecuted candidate, not native parity.
#![cfg(feature = "client-wrath")]

#[path = "common/quest_alpha_gradient.rs"]
mod support;

use wow_ui_sim::lua_api::WowLuaEnv;

fn create_env() -> WowLuaEnv {
    let env = support::create_env();
    support::load_cached_lua(
        &env,
        "wrath/FrameXML/QuestFrame.lua",
        "4b2221da7f962535796ed500fbd1fc2adf95020e4564217ef457c1be7f9935f9",
    );
    support::load_cached_lua(
        &env,
        "wrath/FrameXML/QuestInfo.lua",
        "3bfe691acdb2a095a6633c3ce7bef4a13f53f464d20781b2e850e442f9b715cd",
    );
    support::exec(
        &env,
        include_str!("fixtures/quest_alpha_gradient/wrath.lua"),
    );
    env
}

fn assert_write_quest_requested(env: &WowLuaEnv) {
    let state = env.state();
    let sim = state.borrow();
    assert_eq!(
        sim.last_sound_name_requested.as_deref(),
        Some(&b"WriteQuest"[..])
    );
    assert_eq!(sim.last_sound_kit_requested, Some(3093));
}

#[test]
fn original_wrath_gradual_progress_completes_with_immediate_dependent_reveal() {
    let env = create_env();
    support::exec(&env, "RunWrathGradualGradient()");
    assert_write_quest_requested(&env);
}

#[test]
fn original_wrath_1024_branch_completes_and_enables_accept() {
    let env = create_env();
    support::exec(&env, "RunWrathInstantGradient()");
    assert_write_quest_requested(&env);
}
