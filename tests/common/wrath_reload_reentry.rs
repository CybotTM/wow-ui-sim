//! Original fresh PlayerFrame observes the combat-bit-conditional entry notification.
pub fn assert_reentry(combat: bool) {
    let mut env = super::original::load_original_wrath_startup();
    env.state().borrow_mut().player.in_combat = combat;
    env.state().borrow_mut().in_world = true;
    env.set_reload_loader(|env, _| {
        super::original::load_original_wrath_ui(env);
        env.exec(
            r#"
            ReentryRegens = {}
            local observer = CreateFrame('Frame')
            observer:RegisterEvent('PLAYER_REGEN_DISABLED')
            observer:SetScript('OnEvent', function(self, event, ...)
                table.insert(ReentryRegens, { argc = select('#', ...) })
            end)
        "#,
        )
    });
    env.exec("ReloadUI()").unwrap();
    assert!(env.service_headless_reload(&mut None).unwrap());
    assert_eq!(env.state().borrow().player.in_combat, combat);
    assert!(env.state().borrow().in_world);
    assert!(!env.state().borrow().reload_allows_protected_actions);
    assert_eq!(
        env.eval::<i32>("return #ReentryRegens").unwrap(),
        i32::from(combat)
    );
    assert_eq!(
        env.eval::<bool>("return InCombatLockdown()").unwrap(),
        combat
    );
    if combat {
        env.exec("assert(ReentryRegens[1].argc == 0); assert(UnitAffectingCombat('player') and PlayerFrame.onHateList ~= nil, 'original fresh PlayerFrame combat entry presentation')").unwrap();
    } else {
        env.exec("assert(not InCombatLockdown()); assert(PlayerFrame.onHateList == nil)")
            .unwrap();
    }
}
