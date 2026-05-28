use std::path::PathBuf;

use wow_ui_sim::loader::discover_blizzard_addons_for_screen;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}

fn load_character_select_screen() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::CharacterSelect);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    wow_ui_sim::xml::register_intrinsic_templates();

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::CharacterSelect);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("[load {name}] FAILED: {err}"));
    }

    env.apply_post_load_workarounds();
    settle_headless_startup(&env);
    env
}

#[test]
fn blizzard_character_select_nav_bar_is_discovered_for_glue_screens() {
    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::CharacterSelect);
    let names: Vec<&str> = addons.iter().map(|(name, _)| name.as_str()).collect();

    assert!(
        names.contains(&"Blizzard_CharacterSelectNavBar"),
        "Blizzard_CharacterSelectNavBar (## AllowLoad: Glue) should appear in the CharacterSelect screen addon set; got: {names:?}"
    );
    for required in [
        "Blizzard_SharedXML",
        "Blizzard_GameModeSelect",
        "Blizzard_GlueMenuFrame",
    ] {
        assert!(
            names.contains(&required),
            "{required} (declared dep of Blizzard_CharacterSelectNavBar) should also be discovered; got: {names:?}"
        );
    }
}

#[test]
fn blizzard_character_select_nav_bar_mixins_and_template_register() {
    let env = load_character_select_screen();

    let mixins_present: bool = env
        .eval(
            "return type(CharacterSelectNavBarMixin) == 'table' \
                and type(CharacterSelectNavBarButtonMixin) == 'table'",
        )
        .expect("mixin query should succeed");
    assert!(
        mixins_present,
        "Blizzard_CharacterSelectNavBar mixins should be defined after load"
    );

    let template_instantiates: bool = env
        .eval(
            "local nav = CreateFrame('Frame', nil, UIParent, 'CharacterSelectNavBarTemplate'); \
             return nav ~= nil",
        )
        .expect("CreateFrame should succeed");
    assert!(
        template_instantiates,
        "CharacterSelectNavBarTemplate should instantiate via CreateFrame"
    );
}
