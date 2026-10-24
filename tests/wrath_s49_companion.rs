#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::globals::real::wrath_legacy_state::CompanionMetadata;

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
    let first_spell = state.world.mounts[0].spell_id;
    let second_spell = state.world.mounts[1].spell_id;
    // Explicit fixture identities, NOT claims about native creatures for these spells.
    for (spell, creature_id, name) in [
        (first_spell, 91001, "Zulu S49"),
        (second_spell, 91002, "alpha S49"),
    ] {
        state.wrath_legacy.companion_metadata.insert(
            spell,
            CompanionMetadata {
                creature_id,
                name: Some(name.into()),
                icon_path: None,
                active: false,
            },
        );
    }
    (first_spell, second_spell)
}

#[test]
fn supplied_companion_metadata_preserves_identity_and_optional_slots() {
    let env = WowLuaEnv::new().expect("create explicit-input environment");
    let (first_spell, _) = seed_mount_inputs(&env);
    let creature: u32 = env.eval("return GetCompanionInfo('mount', 1)").unwrap();
    assert_eq!(
        creature, 91001,
        "supplied creature identity, not journal ID"
    );
    env.state()
        .borrow_mut()
        .wrath_legacy
        .companion_metadata
        .clear();
    env.exec("assert(GetNumCompanions('MOUNT') == 1); assert(select('#', GetCompanionInfo('MOUNT', 1)) == 0)")
        .expect("missing metadata never invents creature identity or drops known count");
    env.state()
        .borrow_mut()
        .wrath_legacy
        .companion_metadata
        .insert(
            first_spell,
            CompanionMetadata {
                creature_id: 92001,
                name: None,
                icon_path: Some(r"Interface\Icons\FixtureCompanion".into()),
                active: true,
            },
        );
    env.exec(&format!(
        r#"
        assert(select('#', GetCompanionInfo('MOUNT', 1)) == 5)
        local creature, name, spell, icon, active = GetCompanionInfo('MOUNT', 1)
        assert(creature == 92001 and name == nil and spell == {first_spell})
        assert(icon == [[Interface\Icons\FixtureCompanion]] and active == 1)
        assert(select('#', GetCompanionInfo('MOUNT', 2)) == 0)
    "#
    ))
    .expect("exact positional metadata and nil slots");
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
