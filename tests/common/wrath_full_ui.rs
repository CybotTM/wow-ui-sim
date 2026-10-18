//! Original Wrath startup ordering, shared by bounded consumer tests.
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

pub fn load_original_wrath_startup() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1600.0, 1200.0);
    env.set_font_system(std::rc::Rc::new(std::cell::RefCell::new(
        wow_ui_sim::render::font::WowFontSystem::new_without_casc(),
    )));
    env.set_screen_mode(ScreenKind::Game);
    let ui = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc().unwrap();
    wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    for addon in
        wow_ui_sim::loader::discover_blizzard_startup_addons_for_screen(&ui, ScreenKind::Game)
    {
        wow_ui_sim::loader::load_startup_addon(
            &env.loader_env(),
            &addon.toc_path,
            addon.kind,
            None,
        )
        .unwrap();
    }
    env.apply_post_load_workarounds();
    wow_ui_sim::startup::collect_lua_error_startup(&env);
    env
}
