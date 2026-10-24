#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;

use crate::common;

#[test]
fn toggle_character_relic_capability_controls_original_ammo_slot_visibility() {
    let env = common::panel_fixtures::setup_env();
    const PALADIN: i32 = 2;
    const WARRIOR: i32 = 1;

    // Reopen the original panel after state changes. Either constant boolean,
    // nil/no returns, and a cached initial class must fail this selector alone.
    for (class, ammo_visible) in [(PALADIN, false), (WARRIOR, true), (PALADIN, false)] {
        env.state().borrow_mut().player.class_index = class;
        reopen_paper_doll(&env);
        let (character, paper_doll, ranged, ammo_shown, ammo): (bool, bool, bool, bool, bool) = env
            .eval(
                "return CharacterFrame:IsVisible(), PaperDollFrame:IsVisible(), \
                 CharacterRangedSlot:IsVisible(), CharacterAmmoSlot:IsShown(), \
                 CharacterAmmoSlot:IsVisible()",
            )
            .expect("read original character equipment frames");
        assert!(
            character && paper_doll && ranged,
            "class {class}: panel and ranged slot visible"
        );
        assert_eq!(ammo_shown, ammo_visible, "class {class}: ammo shown state");
        assert_eq!(
            ammo, ammo_visible,
            "class {class}: effective ammo visibility"
        );
    }
}

fn reopen_paper_doll(env: &WowLuaEnv) {
    common::panel_fixtures::clear_recorded_lua_errors(env);
    env.exec("HideUIPanel(CharacterFrame); ToggleCharacter('PaperDollFrame')")
        .expect("open original PaperDollFrame through ToggleCharacter");
    let errors = common::panel_fixtures::recorded_lua_errors(env);
    assert!(errors.is_empty(), "ToggleCharacter Lua errors: {errors:?}");
}
