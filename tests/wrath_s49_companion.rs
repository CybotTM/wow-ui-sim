#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::globals::real::wrath_legacy_state::{
    CompanionMetadata, CompanionRecovery,
};

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
        set_history(&env, vec![own_recovery(first_spell, 90_250, 300_500)]);
        assert_companion_cooldown(&env, 90.25, 300.5);
        set_history(&env, vec![own_recovery(first_spell, 95_000, 400_000)]);
        assert_companion_cooldown(&env, 95.0, 400.0);
        set_history(&env, vec![]);
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
        .eval("return GetCompanionCooldown(PetPaperDollFrameCompanionFrame.mode, '1')")
        .expect("three numeric cooldown results, including numeric-convertible index");
    assert_eq!(result, (start, duration, 1.0));
}

fn own_recovery(spell_id: u32, time_ms: u32, recovery_ms: u32) -> CompanionRecovery {
    CompanionRecovery {
        spell_id,
        time_ms,
        recovery_ms,
        ..Default::default()
    }
}

fn category_recovery(category: i32, time_ms: u32, recovery_ms: u32) -> CompanionRecovery {
    CompanionRecovery {
        category,
        category_time_ms: time_ms,
        category_recovery_ms: recovery_ms,
        ..Default::default()
    }
}

fn gcd_recovery(category: i32, time_ms: u32, recovery_ms: u32) -> CompanionRecovery {
    CompanionRecovery {
        start_recovery_category: category,
        time_ms,
        start_recovery_ms: recovery_ms,
        ..Default::default()
    }
}

fn set_history(env: &WowLuaEnv, history: Vec<CompanionRecovery>) {
    env.state()
        .borrow_mut()
        .wrath_legacy
        .companion_recovery_history = history;
}

#[test]
fn companion_cooldown_own_timer_seconds_and_expiry() {
    test_timeout! {
        let (env, spell) = cooldown_env();
        assert_companion_cooldown(&env, 0.0, 0.0);
        let mut held = own_recovery(spell, 90_250, 300_500);
        held.on_hold = true;
        set_history(&env, vec![held]);
        env.exec("PetPaperDollFrame_UpdateCompanionCooldowns(); assert(not CompanionButton1Cooldown:IsShown())")
            .expect("original timer hides held recovery");
        let (start, duration, enable): (f64, f64, f64) = env.eval("return GetCompanionCooldown('MOUNT', 1)").unwrap();
        assert!(start >= 100.0, "held own starts at sampled now");
        assert_eq!((duration, enable), (0.001, 0.0), "req-13 held own output, not comparison interval");
        set_history(&env, vec![own_recovery(spell, 90_250, 300_500)]);
        env.state().borrow_mut().wrath_legacy.companion_metadata.clear();
        assert_companion_cooldown(&env, 90.25, 300.5);
        env.state().borrow_mut().wrath_legacy.cooldown_excluded_spells.insert(spell);
        assert_companion_cooldown(&env, 0.0, 0.0);
        env.state().borrow_mut().wrath_legacy.cooldown_excluded_spells.remove(&spell);
        assert_companion_cooldown(&env, 90.25, 300.5);
        env.exec(r#"
            assert(select('#', GetCompanionCooldown('MOUNT', 1)) == 3)
            for _, args in ipairs({{}, {'other', 1}, {'MOUNT'}, {'MOUNT', 0}, {'MOUNT', 'bad'}}) do
                local ok, err = pcall(GetCompanionCooldown, unpack(args))
                assert(not ok and string.find(err, 'Usage: GetCompanionCooldown(type, index)', 1, true))
            end
            assert(select('#', GetCompanionCooldown('MOUNT', -1)) == 0)
            assert(select('#', GetCompanionCooldown('MOUNT', 2)) == 0)
            assert(select('#', GetCompanionCooldown('CRITTER', 1)) == 0)
        "#).expect("native validation and absent selected spell return counts");
        set_history(&env, vec![own_recovery(spell, 1_000, 2_000)]);
        assert_companion_cooldown(&env, 0.0, 0.0);
    }
}

#[test]
fn companion_cooldown_same_category_history() {
    test_timeout! {
        let (env, spell) = cooldown_env();
        env.state().borrow_mut().spell_cooldown_categories.insert(spell, 701);
        set_history(&env, vec![category_recovery(702, 80_500, 250_250)]);
        assert_companion_cooldown(&env, 0.0, 0.0);
        set_history(&env, vec![category_recovery(701, 80_500, 250_250)]);
        assert_companion_cooldown(&env, 80.5, 250.25);
        set_history(&env, vec![own_recovery(spell, 95_000, 400_000)]);
        assert_companion_cooldown(&env, 95.0, 400.0);
        set_history(&env, vec![category_recovery(701, 80_500, 250_250)]);
        assert_companion_cooldown(&env, 80.5, 250.25);
        set_history(&env, vec![]);
        assert_companion_cooldown(&env, 0.0, 0.0);
    }
}

#[test]
fn companion_cooldown_matching_gcd_and_separate_own_interval() {
    test_timeout! {
        let (env, spell) = cooldown_env();
        assert_companion_cooldown(&env, 0.0, 0.0);
        set_history(&env, vec![gcd_recovery(802, 99_250, 120_500)]);
        assert_companion_cooldown(&env, 0.0, 0.0);
        env.state().borrow_mut().wrath_legacy.spell_start_recovery_categories.insert(spell, 801);
        assert_companion_cooldown(&env, 0.0, 0.0);
        set_history(&env, vec![gcd_recovery(801, 99_250, 120_500)]);
        assert_companion_cooldown(&env, 99.25, 120.5);
        set_history(&env, vec![own_recovery(spell, 90_500, 400_250)]);
        assert_companion_cooldown(&env, 90.5, 400.25);
        set_history(&env, vec![gcd_recovery(801, 99_250, 120_500)]);
        assert_companion_cooldown(&env, 99.25, 120.5);
        env.state().borrow_mut().wrath_legacy.spell_start_recovery_categories.insert(spell, 0);
        set_history(&env, vec![gcd_recovery(0, 99_250, 120_500)]);
        assert_companion_cooldown(&env, 99.25, 120.5);
        env.state().borrow_mut().wrath_legacy.spell_start_recovery_categories.insert(spell, 801);
        set_history(&env, vec![gcd_recovery(801, 1_000, 2_000)]);
        assert_companion_cooldown(&env, 0.0, 0.0);
    }
}

#[test]
fn companion_cooldown_flagged_shared_category() {
    test_timeout! {
        let (env, spell) = cooldown_env();
        env.state().borrow_mut().spell_cooldown_categories.insert(spell, 701);
        set_history(&env, vec![category_recovery(702, 80_500, 250_250)]);
        assert_companion_cooldown(&env, 0.0, 0.0);
        env.state().borrow_mut().wrath_legacy.spell_category_flags.insert(702, 2);
        assert_companion_cooldown(&env, 80.5, 250.25);
        set_history(&env, vec![category_recovery(702, 95_000, 400_000)]);
        assert_companion_cooldown(&env, 95.0, 400.0);
        env.state().borrow_mut().wrath_legacy.spell_category_flags.insert(702, 1);
        assert_companion_cooldown(&env, 0.0, 0.0);
    }
}

#[test]
fn companion_cooldown_absent_player_preserves_original_timer() {
    test_timeout! {
        let (env, spell) = cooldown_env();
        assert_companion_cooldown(&env, 0.0, 0.0);
        set_history(&env, vec![own_recovery(spell, 80_500, 250_250)]);
        assert_companion_cooldown(&env, 80.5, 250.25);
        env.state().borrow_mut().wrath_legacy.companion_player_absent = true;
        set_history(&env, vec![own_recovery(spell, 95_000, 400_000)]);
        env.exec(r#"
            PetPaperDollFrame_UpdateCompanionCooldowns()
            assert(select('#', GetCompanionCooldown('MOUNT', 1)) == 0)
            local start, duration = CompanionButton1Cooldown:GetCooldownTimes()
            assert(CompanionButton1Cooldown:IsShown())
            assert(start == 80500 and duration == 250250,
                'original consumer skips timer mutation without an active player')
        "#).expect("zero returns skip original timer update, not idle/hide");
        env.state().borrow_mut().wrath_legacy.companion_player_absent = false;
        assert_companion_cooldown(&env, 95.0, 400.0);
        set_history(&env, vec![]);
        assert_companion_cooldown(&env, 0.0, 0.0);
    }
}

#[test]
fn companion_cooldown_known_critter_uses_supplied_spell() {
    test_timeout! {
        let (env, spell) = cooldown_env();
        {
            let mut sim = env.state().borrow_mut();
            assert!(!sim.world.pets.is_empty(), "fixture requires one catalog species");
            for pet in &mut sim.world.pets { pet.is_collected = false; }
            sim.world.pets[0].is_collected = true;
            let species = sim.world.pets[0].species_id as i32;
            sim.wrath_legacy.eligible_critters.clear();
            sim.wrath_legacy.eligible_critters.insert(species);
            // Synthetic join, not native creature/spell data or an ID alias.
            sim.wrath_legacy.critter_spells.insert(species, spell);
        }
        env.exec("PetPaperDollFrame_SetTab(2); assert(PetPaperDollFrameCompanionFrame.mode == 'CRITTER'); assert(CompanionButton1:IsEnabled())")
            .expect("original selected critter row");
        assert_companion_cooldown(&env, 0.0, 0.0);
        set_history(&env, vec![own_recovery(spell, 80_500, 250_250)]);
        assert_companion_cooldown(&env, 80.5, 250.25);
        set_history(&env, vec![own_recovery(spell, 95_000, 400_000)]);
        assert_companion_cooldown(&env, 95.0, 400.0);
        set_history(&env, vec![]);
        assert_companion_cooldown(&env, 0.0, 0.0);
        env.state().borrow_mut().wrath_legacy.companion_player_absent = true;
        env.exec("PetPaperDollFrame_UpdateCompanionCooldowns(); assert(select('#', GetCompanionCooldown('CRITTER', 1)) == 0); assert(not CompanionButton1Cooldown:IsShown())")
            .expect("no active player also has zero critter returns");
    }
}

#[test]
fn companion_cooldown_req13_endpoint_and_history_ties() {
    test_timeout! {
        let (env, spell) = cooldown_env();
        env.state().borrow_mut().spell_cooldown_categories.insert(spell, 701);
        env.state().borrow_mut().wrath_legacy.spell_start_recovery_categories.insert(spell, 801);
        assert_companion_cooldown(&env, 0.0, 0.0);
        // One node: own -> category -> GCD; equal endpoints overwrite each time.
        let mut node = own_recovery(spell, 80_000, 320_000);
        node.category = 701;
        node.category_time_ms = 90_000;
        node.category_recovery_ms = 310_000;
        set_history(&env, vec![node.clone()]);
        assert_companion_cooldown(&env, 90.0, 310.0);
        node.start_recovery_category = 801;
        node.start_recovery_ms = 320_000;
        set_history(&env, vec![node.clone()]);
        assert_companion_cooldown(&env, 80.0, 320.0);
        // A shorter-ending GCD loses even though it is visited after category.
        node.start_recovery_ms = 319_000;
        set_history(&env, vec![node.clone()]);
        assert_companion_cooldown(&env, 90.0, 310.0);
        let later = category_recovery(701, 100_000, 300_000);
        set_history(&env, vec![node.clone(), later.clone()]);
        assert_companion_cooldown(&env, 100.0, 300.0);
        set_history(&env, vec![later, node]);
        assert_companion_cooldown(&env, 90.0, 310.0);
        // A later endpoint wins even with a much shorter duration.
        set_history(&env, vec![own_recovery(spell, 80_000, 320_000),
            gcd_recovery(801, 390_000, 20_000)]);
        assert_companion_cooldown(&env, 390.0, 20.0);
        set_history(&env, vec![]);
        assert_companion_cooldown(&env, 0.0, 0.0);
    }
}

#[test]
fn companion_cooldown_req13_accepted_hold_enable_is_sticky() {
    test_timeout! {
        let (env, spell) = cooldown_env();
        env.state().borrow_mut().spell_cooldown_categories.insert(spell, 701);
        env.state().borrow_mut().wrath_legacy.spell_start_recovery_categories.insert(spell, 801);
        assert_companion_cooldown(&env, 0.0, 0.0);
        let winner = gcd_recovery(801, 390_000, 20_000);
        set_history(&env, vec![winner.clone()]);
        assert_companion_cooldown(&env, 390.0, 20.0);
        let mut long_held = own_recovery(spell, 0, 500_000);
        long_held.on_hold = true;
        set_history(&env, vec![long_held, winner.clone()]);
        env.exec("PetPaperDollFrame_UpdateCompanionCooldowns(); assert(not CompanionButton1Cooldown:IsShown())")
            .expect("held own comparison endpoint beats GCD despite emitted 1ms interval");
        let (start, duration, enable): (f64, f64, f64) = env.eval("return GetCompanionCooldown('MOUNT', 1)").unwrap();
        assert!(start >= 100.0);
        assert_eq!((duration, enable), (0.001, 0.0));
        let mut held = own_recovery(spell, 0, 10_000);
        held.on_hold = true;
        set_history(&env, vec![held.clone(), winner.clone()]);
        env.exec("PetPaperDollFrame_UpdateCompanionCooldowns(); assert(not CompanionButton1Cooldown:IsShown())")
            .expect("accepted held candidate disables a later winning GCD");
        let result: (f64, f64, f64) = env.eval("return GetCompanionCooldown('MOUNT', 1)").unwrap();
        assert_eq!(result, (390.0, 20.0, 0.0));
        set_history(&env, vec![winner.clone(), held]);
        assert_companion_cooldown(&env, 390.0, 20.0);
        let mut held_category = category_recovery(701, 0, 10_000);
        held_category.on_hold = true;
        set_history(&env, vec![held_category, winner.clone()]);
        env.exec("PetPaperDollFrame_UpdateCompanionCooldowns(); assert(not CompanionButton1Cooldown:IsShown())")
            .expect("accepted held category also disables later GCD");
        let result: (f64, f64, f64) = env.eval("return GetCompanionCooldown('MOUNT', 1)").unwrap();
        assert_eq!(result, (390.0, 20.0, 0.0));
        set_history(&env, vec![winner]);
        assert_companion_cooldown(&env, 390.0, 20.0);
    }
}
