//! AddonList character dropdown reset behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::AddonInfo;

const ROOT: &str = "Blizzard_AddOnList";
const PROBE_ADDON: &str = "AddonListClearCharacterProbe";

#[test]
fn clear_character_dropdown_resets_selected_character_to_all() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        seed_probe_addon(env);

        let probe: ClearCharacterProbe = env
            .eval(
                r#"
                local getAddOnEnableState = C_AddOns.GetAddOnEnableState
                local phase = "before"
                local beforeCharacter
                local afterCharacterWasNil = false
                C_AddOns.GetAddOnEnableState = function(addonIndex, character)
                    if phase == "before" then
                        beforeCharacter = character
                    else
                        afterCharacterWasNil = character == nil
                    end
                    return getAddOnEnableState(addonIndex, character)
                end

                AddonList_Update()
                AddonList_ClearCharacterDropdown()
                phase = "after"
                AddonList_Update()

                C_AddOns.GetAddOnEnableState = getAddOnEnableState

                return beforeCharacter,
                       UnitGUID("player"),
                       afterCharacterWasNil
                "#,
            )
            .expect("AddonList clear character dropdown probe must run cleanly");

        assert_clear_character_probe(probe);
    });
}

type ClearCharacterProbe = (String, String, bool);

fn seed_probe_addon(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.addons.clear();
    state.addons.push(AddonInfo {
        folder_name: PROBE_ADDON.into(),
        title: PROBE_ADDON.into(),
        enabled: true,
        loaded: false,
        ..Default::default()
    });
}

fn assert_clear_character_probe(probe: ClearCharacterProbe) {
    let (character_before_clear, expected_player_guid, character_cleared_to_all) = probe;

    assert_eq!(
        character_before_clear, expected_player_guid,
        "`AddonList` must start on the current player character out of glue"
    );
    assert!(
        character_cleared_to_all,
        "`AddonList_ClearCharacterDropdown` must reset `GetAddonCharacter()` to nil"
    );
}
