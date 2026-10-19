//! Wrath payload proof plus unchanged SpellBook hidden/show preservation.
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

pub fn assert_original_spellbook_paths() {
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
    assert_eq!(
        env.state().borrow().lua_errors.len(),
        before,
        "original hidden/show consumer errors"
    );
    eprintln!(
        "original SpellBook hidden/show observed SPELLS_CHANGED={}; native occurrence/order remains UNKNOWN",
        env.eval::<i32>("return SpellBookReloadSignals").unwrap()
    );
}
