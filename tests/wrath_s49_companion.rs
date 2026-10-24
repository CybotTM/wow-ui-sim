#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;

use crate::common;

// A5 = laptop-labels-answers-20261018/laptop-answers-req-5.md.
// Catalog IDs below are simulator inputs, never a native character census.
fn seed_mount_inputs(env: &WowLuaEnv) -> (u32, u32) {
    let mut state = env.state().borrow_mut();
    assert!(
        state.world.mounts.len() >= 2,
        "S49 requires two catalog inputs"
    );
    for mount in &mut state.world.mounts {
        mount.is_collected = false;
    }
    state.world.mounts[0].name = "Zulu S49".into();
    state.world.mounts[1].name = "alpha S49".into();
    // Native-fact A5:25: legacy classification is not modern usability filtering.
    state.world.mounts[0].is_usable = false;
    state.world.mounts[1].is_usable = false;
    state.world.mounts[0].is_collected = true;
    state.wrath_legacy.eligible_mounts = state.world.mounts[..2]
        .iter()
        .map(|mount| mount.mount_id as i32)
        .collect();
    (
        state.world.mounts[0].spell_id,
        state.world.mounts[1].spell_id,
    )
}

#[test]
fn learned_mount_refreshes_original_companion_buttons_inline() {
    test_timeout! {
        let env = common::panel_fixtures::setup_env();
        let (first_spell, second_spell) = seed_mount_inputs(&env);
        env.exec(&format!("S49_FIRST_SPELL = {first_spell}; S49_SECOND_SPELL = {second_spell}"))
            .expect("publish fixture expectations, not API replacements");
        common::panel_fixtures::clear_recorded_lua_errors(&env);
        env.exec(include_str!("fixtures/wrath_s49/companion_list.lua"))
            .expect("one supplied mount reaches original companion frames");
        // Host state setup, not a claim that a journal mutation is native learning.
        env.state().borrow_mut().world.mounts[1].is_collected = true;
        env.fire_event("COMPANION_LEARNED")
            .expect("deliver zero-payload learned event inline");
        // No explicit consumer refresh, timer, or tick after the event.
        env.exec("S49_AssertCompanionRows({{S49_SECOND_SPELL, 'alpha S49'}, {S49_FIRST_SPELL, 'Zulu S49'}})")
            .expect("learned event refreshes and sorts real companion buttons before returning");
        env.state().borrow_mut().wrath_legacy.eligible_mounts.clear();
        env.fire_event("COMPANION_UNLEARNED")
            .expect("deliver removal event after explicit eligibility change");
        env.exec("S49_AssertCompanionRows({})")
            .expect("removed companions disable and clear original buttons");
        let errors = common::panel_fixtures::recorded_lua_errors(&env);
        assert!(errors.is_empty(), "S49 original consumer errors: {errors:?}");
    }
}
