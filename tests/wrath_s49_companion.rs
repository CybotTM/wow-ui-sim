#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::globals::real::wrath_legacy_state::CompanionMetadata;
use wow_ui_sim::lua_api::state::SpellCooldownState;

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
    let creature: f64 = env.eval("return GetCompanionInfo('mount', 1)").unwrap();
    assert_eq!(
        creature, 91001.0,
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
        env.state().borrow_mut().spell_cooldowns.insert(
            first_spell, SpellCooldownState { start: 90.25, duration: 300.5 },
        );
        assert_companion_cooldown(&env, 90.25, 300.5);
        env.state().borrow_mut().spell_cooldowns.insert(
            first_spell, SpellCooldownState { start: 95.0, duration: 400.0 },
        );
        assert_companion_cooldown(&env, 95.0, 400.0);
        env.state().borrow_mut().spell_cooldowns.clear();
        assert_companion_cooldown(&env, 0.0, 0.0);
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

// Request-11 Q5: native wrapper returns seconds and a numeric enable.
// Each selector loads runtime-selected original FrameXML, then observes widgets.
fn cooldown_env() -> (WowLuaEnv, u32) {
    let env = common::panel_fixtures::setup_env();
    let (spell, _) = seed_mount_inputs(&env);
    env.state().borrow_mut().start_time -= std::time::Duration::from_secs(100);
    env.exec(&format!("S49_FIRST_SPELL = {spell}"))
        .expect("publish fixture expectation");
    env.exec(include_str!("fixtures/wrath_s49/companion_list.lua"))
        .expect("selected original companion rows");
    (env, spell)
}

fn assert_companion_cooldown(env: &WowLuaEnv, start: f64, duration: f64) {
    env.exec(&format!(
        r#"
        PetPaperDollFrame_UpdateCompanionCooldowns()
        local cooldown = CompanionButton1Cooldown
        assert(cooldown:IsShown() == ({start} > 0 and {duration} > 0),
            "original cooldown visibility")
        if {duration} > 0 then
            local actualStart, actualDuration = cooldown:GetCooldownTimes()
            assert(actualStart == {start} * 1000, "original cooldown start")
            assert(actualDuration == {duration} * 1000, "original cooldown duration")
        end
        "#
    ))
    .expect("original selected consumer applies changing cooldown");
    // Direct API contract assertion, not the RED-through-consumer proof.

    let result: (f64, f64, f64) = env
        .eval("return GetCompanionCooldown('mount', '1')")
        .expect("three numeric cooldown results, including numeric-convertible index");
    assert_eq!(result, (start, duration, 1.0));
}

#[test]
fn companion_cooldown_own_timer_seconds_and_expiry() {
    test_timeout! {
    let (env, spell) = cooldown_env();
    assert_companion_cooldown(&env, 0.0, 0.0);
    env.state().borrow_mut().spell_cooldowns.insert(
        spell,
        SpellCooldownState {
            start: 90.25,
            duration: 300.5,
        },
    );
    env.state().borrow_mut().wrath_legacy.cooldowns_on_hold.insert(spell);
    env.exec("PetPaperDollFrame_UpdateCompanionCooldowns(); assert(not CompanionButton1Cooldown:IsShown())")
        .expect("original timer hides on-hold recovery");
    let held: (f64, f64, f64) = env.eval("return GetCompanionCooldown('MOUNT', 1)").unwrap();
    assert_eq!(held, (90.25, 300.5, 0.0), "direct numeric-enable contract");
    env.state().borrow_mut().wrath_legacy.cooldowns_on_hold.remove(&spell);
    // Cooldown lookup needs the selected spell, not creature-name metadata.
    env.state()
        .borrow_mut()
        .wrath_legacy
        .companion_metadata
        .clear();
    assert_companion_cooldown(&env, 90.25, 300.5);
    env.exec(
        r#"
        assert(select('#', GetCompanionCooldown('MOUNT', 1)) == 3)
        for _, args in ipairs({{}, {'other', 1}, {'MOUNT'}, {'MOUNT', 0}, {'MOUNT', 'bad'}}) do
            local ok, err = pcall(GetCompanionCooldown, unpack(args))
            assert(not ok and string.find(err, 'Usage: GetCompanionCooldown(type, index)', 1, true))
        end
        assert(select('#', GetCompanionCooldown('MOUNT', -1)) == 0)
        assert(select('#', GetCompanionCooldown('MOUNT', 2)) == 0)
        assert(select('#', GetCompanionCooldown('CRITTER', 1)) == 0)
    "#,
    )
    .expect("native validation and absent selected spell return counts");
    env.state().borrow_mut().spell_cooldowns.insert(
        spell,
        SpellCooldownState {
            start: 1.0,
            duration: 2.0,
        },
    );
    assert_companion_cooldown(&env, 0.0, 0.0);
    }
}

#[test]
fn companion_cooldown_shared_category_tracks_existing_spell_state() {
    test_timeout! {
    let (env, spell) = cooldown_env();
    // Synthetic host cooldown source/category, not a native companion fixture pair.
    let source_spell = 990001;
    {
        let mut sim = env.state().borrow_mut();
        sim.spell_cooldown_categories.insert(spell, 701);
        sim.spell_cooldown_categories.insert(source_spell, 702);
        sim.spell_cooldowns.insert(
            source_spell,
            SpellCooldownState {
                start: 80.5,
                duration: 250.25,
            },
        );
    }
    assert_companion_cooldown(&env, 0.0, 0.0);
    env.state()
        .borrow_mut()
        .spell_cooldown_categories
        .insert(source_spell, 701);
    assert_companion_cooldown(&env, 80.5, 250.25);
    env.state().borrow_mut().spell_cooldowns.insert(
        spell,
        SpellCooldownState {
            start: 95.0,
            duration: 400.0,
        },
    );
    assert_companion_cooldown(&env, 95.0, 400.0);
    env.state().borrow_mut().spell_cooldowns.remove(&spell);
    assert_companion_cooldown(&env, 80.5, 250.25);
    env.state().borrow_mut().spell_cooldowns.clear();
    assert_companion_cooldown(&env, 0.0, 0.0);
    }
}

#[test]
fn companion_cooldown_gcd_and_longer_spell_interval() {
    test_timeout! {
    let (env, spell) = cooldown_env();
    assert_companion_cooldown(&env, 0.0, 0.0);
    env.state().borrow_mut().gcd = Some((99.25, 120.5));
    assert_companion_cooldown(&env, 0.0, 0.0);
    {
        let mut sim = env.state().borrow_mut();
        sim.wrath_legacy.spell_start_recovery_categories.insert(spell, 801);
        sim.wrath_legacy.gcd_category = Some(802);
    }
    assert_companion_cooldown(&env, 0.0, 0.0);
    env.state().borrow_mut().wrath_legacy.gcd_category = Some(801);
    assert_companion_cooldown(&env, 99.25, 120.5);
    env.state().borrow_mut().spell_cooldowns.insert(
        spell,
        SpellCooldownState {
            start: 90.5,
            duration: 400.25,
        },
    );
    assert_companion_cooldown(&env, 90.5, 400.25);
    env.state().borrow_mut().spell_cooldowns.clear();
    assert_companion_cooldown(&env, 99.25, 120.5);
    env.state().borrow_mut().gcd = Some((1.0, 2.0));
    assert_companion_cooldown(&env, 0.0, 0.0);
    }
}
