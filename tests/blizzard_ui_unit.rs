//! Blizzard UI component unit tests.

mod common;
mod tooltip_full_env_helpers;

use tooltip_full_env_helpers::setup_full_env;

fn open_character_panel(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        local btn = CharacterMicroButton
        assert(btn, "CharacterMicroButton should exist")
        local onclick = btn:GetScript("OnClick")
        assert(onclick, "CharacterMicroButton should have an OnClick handler")
        onclick(btn, "LeftButton", false)
        assert(CharacterFrame and CharacterFrame:IsShown(), "CharacterFrame should be shown")
        assert(CharacterHeadSlot ~= nil, "CharacterHeadSlot should exist")
        "#,
    )
    .expect("Failed to open character panel");
}

#[test]
fn character_panel_equipment_slots_match_inventory_or_background_textures() {
    test_timeout! {
        let env = setup_full_env();
        open_character_panel(&env);

        let result: String = env.eval(
            r#"
            local slotNames = {
                "CharacterHeadSlot",
                "CharacterNeckSlot",
                "CharacterShoulderSlot",
                "CharacterBackSlot",
                "CharacterChestSlot",
                "CharacterShirtSlot",
                "CharacterTabardSlot",
                "CharacterWristSlot",
                "CharacterHandsSlot",
                "CharacterWaistSlot",
                "CharacterLegsSlot",
                "CharacterFeetSlot",
                "CharacterFinger0Slot",
                "CharacterFinger1Slot",
                "CharacterTrinket0Slot",
                "CharacterTrinket1Slot",
                "CharacterMainHandSlot",
                "CharacterSecondaryHandSlot",
            }

            for _, frameName in ipairs(slotNames) do
                local slot = _G[frameName]
                if not slot then
                    return "missing_slot_" .. frameName
                end
                if not slot.icon then
                    return "missing_icon_" .. frameName
                end

                local expectedTexture = GetInventoryItemTexture("player", slot:GetID())
                local actualTexture = slot.icon:GetTexture()

                if expectedTexture ~= nil then
                    if actualTexture ~= expectedTexture then
                        return string.format(
                            "equipped_texture_mismatch_%s_expected_%s_actual_%s",
                            frameName,
                            tostring(expectedTexture),
                            tostring(actualTexture)
                        )
                    end
                else
                    if actualTexture ~= slot.backgroundTextureName then
                        return string.format(
                            "background_texture_mismatch_%s_expected_%s_actual_%s",
                            frameName,
                            tostring(slot.backgroundTextureName),
                            tostring(actualTexture)
                        )
                    end
                end
            end

            return "ok"
        "#,
        ).unwrap();

        assert_eq!(
            result,
            "ok",
            "Character equipment slot icons should match inventory item textures when equipped and slot backgrounds when empty: {result}"
        );
    }
}
