//! Wrath payload proof, original Stopwatch preservation and uncounted spellbook diagnostics.
pub fn assert_reload_pew_arity() {
    let mut env = super::original::load_original_wrath_startup();
    env.set_reload_loader(|env, _| {
        super::original::load_original_wrath_ui(env);
        env.exec(
            r#"
            ReloadPewArity = -1
            local observer = CreateFrame('Frame')
            observer:RegisterEvent('PLAYER_ENTERING_WORLD')
            observer:SetScript('OnEvent', function(self, event, ...)
                ReloadPewArity = select('#', ...)
            end)
        "#,
        )
    });
    env.exec("ReloadUI()").unwrap();
    assert!(env.service_headless_reload(&mut None).unwrap());
    assert_eq!(
        env.eval::<i32>("return ReloadPewArity").unwrap(),
        0,
        "12340 PEW has no initial/reload booleans"
    );
    env.exec("assert(Blizzard_CombatLog_Filters ~= nil, 'original UIParent LOGIN consumer ran in fresh UI')").unwrap();
}

pub fn assert_original_stopwatch_reload_paths() {
    let mut env = super::original::load_original_wrath_startup();
    env.exec("TimeManager_LoadUI()").unwrap();
    assert_stopwatch_show_hide(&env);
    env.exec("OldStopwatchFrame = StopwatchFrame; ReloadUI()")
        .unwrap();
    env.set_reload_loader(|env, _| {
        super::original::load_original_wrath_ui(env);
        Ok(())
    });
    assert!(env.service_headless_reload(&mut None).unwrap());
    env.exec("TimeManager_LoadUI()").unwrap();
    env.exec("assert(OldStopwatchFrame == nil and StopwatchFrame ~= nil, 'new original panel in fresh VM')").unwrap();
    assert_stopwatch_show_hide(&env);
    observe_original_spellbook_gap();
}

fn assert_stopwatch_show_hide(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let before = env.state().borrow().lua_errors.len();
    env.exec("StopwatchFrame:Hide()").unwrap();
    wow_ui_sim::startup::run_lua_error_update_ticks(env);
    env.exec("StopwatchFrame:Show(); assert(StopwatchFrame:IsShown()); assert(TimeManagerStopwatchCheck:GetChecked(), 'original OnShow checks the stopwatch toggle')").unwrap();
    wow_ui_sim::startup::run_lua_error_update_ticks(env);
    env.exec("StopwatchFrame:Hide(); assert(not StopwatchFrame:IsShown()); assert(not TimeManagerStopwatchCheck:GetChecked(), 'original OnHide clears the toggle'); assert(StopwatchTabFrame:GetAlpha() == 0)").unwrap();
    wow_ui_sim::startup::run_lua_error_update_ticks(env);
    assert_eq!(
        env.state().borrow().lua_errors.len(),
        before,
        "original Stopwatch show/hide and ticks emit no errors"
    );
}

// Diagnostic observation only: never counted as a passing preservation case.
fn observe_original_spellbook_gap() {
    let env = super::original::load_original_wrath_startup();
    let before = env.state().borrow().lua_errors.len();
    env.exec(r#"
        SpellBookReloadSignals = 0
        local observer = CreateFrame('Frame')
        observer:RegisterEvent('SPELLS_CHANGED')
        observer:SetScript('OnEvent', function() SpellBookReloadSignals = SpellBookReloadSignals + 1 end)
        SpellBookFrame:Hide()
        SpellBookFrame:Show()
        assert(SpellBookFrame:IsShown())
        SpellBookFrame:Hide()
        assert(not SpellBookFrame:IsShown())
    "#).unwrap();
    let state = env.state().borrow();
    let errors = &state.lua_errors[before..];
    eprintln!(
        "EXPECTED-FAILURE OBSERVATION, excluded from acceptance totals: original SpellBook show/hide errors={}, first={:?}",
        errors.len(),
        errors.first()
    );
    drop(state);
    eprintln!(
        "original SpellBook hidden/show observed SPELLS_CHANGED={}; native occurrence/order remains UNKNOWN",
        env.eval::<i32>("return SpellBookReloadSignals").unwrap()
    );
}
