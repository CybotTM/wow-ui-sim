//! Mists AuraUtil defensive classification through queued timers.
#![cfg(feature = "client-mists")]

use wow_ui_sim::c_api::c_unit_aura_classification::AuraSpellClassification;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn timer_fixture() -> WowLuaEnv {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &ui,
        &["Blizzard_FrameXMLUtil"],
        &[],
    );
    env.state()
        .borrow_mut()
        .aura_spell_classifications
        .spells
        .extend([
            (
                101,
                AuraSpellClassification {
                    is_big_defensive: true,
                    is_private: false,
                },
            ),
            (
                202,
                AuraSpellClassification {
                    is_big_defensive: false,
                    is_private: false,
                },
            ),
        ]);
    env
}

fn assert_timer_classification(spell_id: u32, expected: bool) {
    let env = timer_fixture();
    env.exec(&format!(
        "C_Timer.After(0, function() MistsDefensiveResult = AuraUtil.IsBigDefensive({{spellId = {spell_id}}}) end)"
    )).unwrap();
    assert!(env.process_timers().unwrap() > 0);
    env.exec(&format!(
        "assert(MistsDefensiveResult == {expected}, 'original AuraUtil timer classification')"
    ))
    .unwrap();
}

#[test]
fn stock_startup_timers_do_not_call_a_nil_function() {
    let env = WowLuaEnv::new().unwrap();
    let ui = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    crate::common::load_profile_framexml(&env);
    for (name, toc) in
        wow_ui_sim::loader::discover_blizzard_addons_for_screen(&ui, ScreenKind::Game)
    {
        wow_ui_sim::loader::load_addon(&env.loader_env(), &toc)
            .unwrap_or_else(|error| panic!("{name}: {error}"));
    }
    env.apply_post_load_workarounds();
    let before = env.state().borrow().lua_errors.len();
    wow_ui_sim::startup::collect_lua_error_startup(&env);
    let errors = env.state().borrow().lua_errors[before..].to_vec();
    assert!(
        !errors
            .iter()
            .any(|error| error.contains("attempt to call a nil value")),
        "startup timer errors: {errors:#?}"
    );
}

#[test]
fn timer_consumer_reads_supplied_defensive() {
    assert_timer_classification(101, true);
}

#[test]
fn timer_consumer_reads_supplied_non_defensive() {
    assert_timer_classification(202, false);
}
