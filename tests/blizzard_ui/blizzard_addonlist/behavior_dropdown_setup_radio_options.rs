//! AddonList character dropdown behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AddOnList";

#[test]
fn dropdown_setup_creates_all_and_player_radios_out_of_glue() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let (
            menu_tag,
            radio_count,
            all_radio_text,
            all_radio_value_is_all,
            player_radio_text,
            player_radio_value,
            expected_player_text,
            expected_player_value,
            selected_character,
            update_called,
        ): DropdownProbe = env
            .eval(
                r#"
                local capturedTag
                local radios = {}
                AddonList:GetScript("OnShow")(AddonList)

                local rootDescription = {
                    SetTag = function(_, tag)
                        capturedTag = tag
                    end,
                    CreateRadio = function(_, text, isSelected, setSelected, value)
                        table.insert(radios, {
                            text = text,
                            isSelected = isSelected,
                            setSelected = setSelected,
                            value = value,
                        })
                    end,
                }

                local generator = AddonList.Dropdown.menuGenerator
                generator(AddonList.Dropdown, rootDescription)

                local getAddOnEnableState = C_AddOns.GetAddOnEnableState
                local selectedCharacter
                local updateCalled = false
                C_AddOns.GetAddOnEnableState = function(addonIndex, character)
                    selectedCharacter = character
                    updateCalled = true
                    return getAddOnEnableState(addonIndex, character)
                end

                local playerRadio = radios[2]
                playerRadio.setSelected(playerRadio.value)
                C_AddOns.GetAddOnEnableState = getAddOnEnableState

                return capturedTag,
                       #radios,
                       radios[1].text,
                       radios[1].value == "All",
                       playerRadio.text,
                       playerRadio.value,
                       UnitName("player"),
                       UnitGUID("player"),
                       selectedCharacter,
                       updateCalled
                "#,
            )
            .expect("AddonList dropdown radio setup probe must run cleanly");

        assert_eq!(menu_tag, "MENU_ADDON_LIST");
        assert_eq!(
            radio_count, 2,
            "out-of-glue dropdown must only expose ALL plus the current player"
        );
        assert_eq!(all_radio_text, "All");
        assert!(
            all_radio_value_is_all,
            "the ALL radio must select the local ALL_CHARACTERS sentinel"
        );
        assert_eq!(player_radio_text, expected_player_text);
        assert_eq!(player_radio_value, expected_player_value);
        assert_eq!(
            selected_character, expected_player_value,
            "selecting the player radio must update AddOnList's selected character"
        );
        assert!(
            update_called,
            "selecting a dropdown radio must trigger `AddonList_Update`"
        );
    });
}

#[test]
fn dropdown_player_radio_uses_seeded_local_character_identity() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let (player_radio_text, player_radio_value): (String, String) = env
            .eval(
                r#"
                local radios = {}
                AddonList:GetScript("OnShow")(AddonList)

                local rootDescription = {
                    SetTag = function() end,
                    CreateRadio = function(_, text, isSelected, setSelected, value)
                        table.insert(radios, { text = text, value = value })
                    end,
                }

                AddonList.Dropdown.menuGenerator(AddonList.Dropdown, rootDescription)
                return radios[2].text, radios[2].value
                "#,
            )
            .expect("AddonList dropdown player identity probe must run cleanly");

        assert_eq!(player_radio_text, "Uther");
        assert_eq!(player_radio_value, "Player-1-00000001");
    });
}

type DropdownProbe = (
    String,
    i64,
    String,
    bool,
    String,
    String,
    String,
    String,
    String,
    bool,
);
