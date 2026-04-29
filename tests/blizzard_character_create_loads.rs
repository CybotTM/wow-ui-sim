#![cfg(feature = "client-retail")]
use std::path::PathBuf;

use wow_ui_sim::loader::discover_blizzard_addons_for_screen;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
}

fn load_character_create_screen() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::CharacterCreate);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    wow_ui_sim::xml::register_intrinsic_templates();

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::CharacterCreate);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("[load {name}] FAILED: {err}"));
    }

    env.apply_post_load_workarounds();
    settle_headless_startup(&env);
    env
}

#[test]
fn blizzard_character_create_addon_is_discovered_for_glue_screen() {
    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::CharacterCreate);
    let names: Vec<&str> = addons.iter().map(|(name, _)| name.as_str()).collect();

    assert!(
        names.contains(&"Blizzard_CharacterCreate"),
        "Blizzard_CharacterCreate (## AllowLoad: Glue) should appear in the CharacterCreate screen addon set; got: {names:?}"
    );
    assert!(
        names.contains(&"Blizzard_CharacterCustomize"),
        "Blizzard_CharacterCustomize (RequiredDep of Blizzard_CharacterCreate) should also be discovered; got: {names:?}"
    );
}

#[test]
fn blizzard_character_create_loads_on_character_create_screen() {
    let env = load_character_create_screen();

    let mixins_present: bool = env
        .eval(
            "return type(CharacterCreateMixin) == 'table' \
                and type(CharacterCreateRaceAndClassMixin) == 'table' \
                and type(CharacterCreateNavButtonMixin) == 'table' \
                and type(CharacterCreateNavForwardButtonMixin) == 'table' \
                and type(CharacterCreateClassButtonMixin) == 'table' \
                and type(CharacterCreateRaceButtonMixin) == 'table' \
                and type(CharacterCreateSpecButtonMixin) == 'table' \
                and type(CharacterCreateFactionHeaderMixin) == 'table' \
                and type(ClassTrialCheckButtonMixin) == 'table' \
                and type(CharacterCreateFrameRacialAbilityMixin) == 'table' \
                and type(CharacterCreateRacialAbilityListMixin) == 'table' \
                and type(CharacterCreateEditBoxMixin) == 'table' \
                and type(CharacterCreateNameAvailabilityStateMixin) == 'table' \
                and type(CharacterCreateRandomNameButtonMixin) == 'table' \
                and type(CharacterCreateClassTrialSpecsMixin) == 'table' \
                and type(CharacterCreateZoneChoiceMixin) == 'table' \
                and type(CharacterCreateStartingZoneMixin) == 'table' \
                and type(CharacterCreateStartingZoneArtMixin) == 'table' \
                and type(CharacterCreateStartingZoneButtonMixin) == 'table'",
        )
        .expect("mixin query should succeed");
    assert!(
        mixins_present,
        "Blizzard_CharacterCreate mixin tables should be defined after load"
    );

    let tooltip_present: bool = env
        .eval("return CharCreateTooltip ~= nil")
        .expect("CharCreateTooltip query should succeed");
    assert!(
        tooltip_present,
        "CharCreateTooltip (declared in Blizzard_CharacterCreate.xml) should be defined"
    );
}
