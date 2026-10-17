//! Empty original startup/refilter only; populated replay remains policy-blocked.
#![cfg(feature = "client-wrath")]
use wow_ui_sim::loader::{discover_blizzard_startup_addons_for_screen, load_startup_addon};
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;

#[test]
fn wrath_unmasked_original_empty_refilter_clears_and_completes() {
    let env = wow_ui_sim::lua_api::WowLuaEnv::new().unwrap();
    env.set_screen_size(1600.0, 1200.0);
    env.set_font_system(std::rc::Rc::new(std::cell::RefCell::new(
        wow_ui_sim::render::font::WowFontSystem::new_without_casc(),
    )));
    let ui = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    env.set_screen_mode(ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    for addon in discover_blizzard_startup_addons_for_screen(&ui, ScreenKind::Game) {
        load_startup_addon(&env.loader_env(), &addon.toc_path, addon.kind, None)
            .unwrap_or_else(|error| panic!("load {}: {error}", addon.name));
    }
    env.apply_post_load_workarounds();
    fire_startup_events_for_screen(&env, ScreenKind::Game);
    // Keep unrelated sound/loot startup failures visible but outside this
    // consumer's gate. Do not replace the formatter, history or callbacks.
    let history_errors: Vec<_> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|error| error.contains("Blizzard_CombatLog") && error.contains("Refilter"))
        .cloned()
        .collect();
    assert!(history_errors.is_empty(), "{history_errors:?}");
    env.exec(
        r#"
        assert(C_AddOns.IsAddOnLoaded('Blizzard_CombatLog'))
        assert(CombatLogGetNumEntries() == 0)
        C_CombatLog._state.currentEntry = 37
        assert(C_CombatLog.GetCurrentEntryInfo() == nil)
        ChatFrame2:AddMessage('stale sentinel')
        assert(ChatFrame2:GetNumMessages() > 0)
        Blizzard_CombatLog_Refilter()
        assert(ChatFrame2:GetNumMessages() == 0)
        assert(ChatFrame2:GetMaxLines() == 300)
        assert(CombatLogUpdateFrame.refiltering == false)
        assert(CombatLogUpdateFrame:GetScript('OnUpdate') == nil)
        assert(CombatLogGetCurrentEntry() == nil)
        assert(CombatLogAdvanceEntry(-1) == false)
        "#,
    )
    .expect("empty original refilter never formats a cursor as an event");
}
