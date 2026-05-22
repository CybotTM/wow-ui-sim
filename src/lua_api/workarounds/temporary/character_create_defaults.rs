//! Temporary character creation default-state workaround.
//!
//! The character creation UI expects selected race/class/faction fields and a
//! few frame surfaces to be ready by the time creation mixins run. Seed them
//! here until the simulator models the full character-create state flow.

use crate::lua_api::WowLuaEnv;

const CHARACTER_CREATE_DEFAULTS_WORKAROUND_LUA: &str = r#"
local function __wow_character_create_defaults_frame()
    if type(CharacterCreateFrame) ~= "table" then
        return nil
    end
    return CharacterCreateFrame.RaceAndClassFrame
end

local function __wow_seed_character_create_defaults(frame)
    if type(frame) ~= "table" then
        return
    end

    local raceID = C_CharacterCreation and C_CharacterCreation.GetSelectedRace and C_CharacterCreation.GetSelectedRace() or 1
    if type(frame.selectedRaceData) ~= "table" then
        frame.selectedRaceData = C_CharacterCreation and C_CharacterCreation.GetRaceDataByID and C_CharacterCreation.GetRaceDataByID(raceID) or { enabled = true, isNeutralRace = false, factionInternalName = "Alliance" }
    end
    if type(frame.selectedClassData) ~= "table" then
        frame.selectedClassData = C_CharacterCreation and C_CharacterCreation.GetSelectedClass and C_CharacterCreation.GetSelectedClass() or { classID = 2, earlyFactionChoice = false }
    end
    if frame.selectedFaction == nil and C_CharacterCreation and C_CharacterCreation.GetFactionForRace then
        frame.selectedFaction = C_CharacterCreation.GetFactionForRace(raceID)
    end
end

local function __wow_seed_character_create_frame(frame)
    if type(frame) ~= "table" then
        return
    end

    if type(frame.BGTex) ~= "table" then
        frame.BGTex = {}
    end

    if type(frame.BackButton) == "table"
        and type(frame.BackButton.UpdateText) == "function"
        and type(frame.BackButton.GetText) == "function"
        and (frame.BackButton:GetText() == nil or frame.BackButton:GetText() == "")
    then
        frame.BackButton:UpdateText(BACK, BACKWARD_ARROW)
    end

    if type(frame.UpdateForwardButton) == "function" then
        frame:UpdateForwardButton()
    end
end

local characterCreateFrame = type(CharacterCreateFrame) == "table" and CharacterCreateFrame or nil
local raceAndClassFrame = characterCreateFrame and characterCreateFrame.RaceAndClassFrame or nil
if raceAndClassFrame ~= nil then
    __wow_seed_character_create_defaults(raceAndClassFrame)
end
if characterCreateFrame ~= nil then
    __wow_seed_character_create_frame(characterCreateFrame)
end

if type(CharacterCreateMixin) == "table" and type(CharacterCreateMixin.CreateCharacter) == "function" and not rawget(_G, "__wow_character_create_defaults_patched") then
    local originalCreateCharacter = CharacterCreateMixin.CreateCharacter
    function CharacterCreateMixin:CreateCharacter(...)
        __wow_seed_character_create_defaults(__wow_character_create_defaults_frame())
        __wow_seed_character_create_frame(self)
        if A_Admin and type(A_Admin.SetPlayerName) == "function" and type(self.GetSelectedName) == "function" then
            A_Admin.SetPlayerName(self:GetSelectedName())
        end
        return originalCreateCharacter(self, ...)
    end
    rawset(_G, "__wow_character_create_defaults_patched", true)
end

if type(CharacterCreateRaceAndClassMixin) == "table" and type(CharacterCreateRaceAndClassMixin.GetCreateCharacterFaction) == "function" and not rawget(_G, "__wow_character_create_faction_patched") then
    local originalGetCreateCharacterFaction = CharacterCreateRaceAndClassMixin.GetCreateCharacterFaction
    function CharacterCreateRaceAndClassMixin:GetCreateCharacterFaction()
        __wow_seed_character_create_defaults(self)
        return originalGetCreateCharacterFaction(self)
    end
    rawset(_G, "__wow_character_create_faction_patched", true)
end

if type(CharacterCreateRaceAndClassMixin) == "table" and type(CharacterCreateRaceAndClassMixin.UpdateState) == "function" and not rawget(_G, "__wow_character_create_update_patched") then
    local originalUpdateState = CharacterCreateRaceAndClassMixin.UpdateState
    function CharacterCreateRaceAndClassMixin:UpdateState(selectedFaction)
        __wow_seed_character_create_defaults(self)
        local result = originalUpdateState(self, selectedFaction)
        __wow_seed_character_create_frame(CharacterCreateFrame)
        return result
    end
    rawset(_G, "__wow_character_create_update_patched", true)
end

if type(CharacterCreateMixin) == "table" and type(CharacterCreateMixin.UpdateBackgroundOverlays) == "function" and not rawget(_G, "__wow_character_create_background_overlay_patched") then
    local originalUpdateBackgroundOverlays = CharacterCreateMixin.UpdateBackgroundOverlays
    function CharacterCreateMixin:UpdateBackgroundOverlays(selectedClassData, selectedRaceData)
        local ok = pcall(originalUpdateBackgroundOverlays, self, selectedClassData, selectedRaceData)
        if ok then
            return
        end

        local backgroundTextures = self and self.BGTex or nil
        if type(backgroundTextures) == "table" then
            local iter_ok, iter, state, first = pcall(ipairs, backgroundTextures)
            if iter_ok and type(iter) == "function" then
                local didSetAlpha = false
                for _, texture in iter, state, first do
                    if type(texture) == "table" and type(texture.SetAlpha) == "function" then
                        texture:SetAlpha(1)
                        didSetAlpha = true
                    end
                end
                if didSetAlpha then
                    return
                end
            end
        end

        if type(backgroundTextures) == "table" and type(backgroundTextures.SetAlpha) == "function" then
            backgroundTextures:SetAlpha(1)
        end
    end
    rawset(_G, "__wow_character_create_background_overlay_patched", true)
end
"#;

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(CHARACTER_CREATE_DEFAULTS_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_initial_character_create_state_and_buttons() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            BACK = "Back"
            BACKWARD_ARROW = "<"
            C_CharacterCreation = {
                GetSelectedRace = function()
                    return 9
                end,
                GetRaceDataByID = function(raceID)
                    return { raceID = raceID, enabled = true }
                end,
                GetSelectedClass = function()
                    return { classID = 5, earlyFactionChoice = false }
                end,
                GetFactionForRace = function(raceID)
                    return "RaceFaction" .. tostring(raceID)
                end,
            }
            CharacterCreateFrame = {
                RaceAndClassFrame = {},
                BackButton = {
                    text = "",
                    GetText = function(self)
                        return self.text
                    end,
                    UpdateText = function(self, text, arrow)
                        self.text = text
                        self.arrow = arrow
                    end,
                },
                UpdateForwardButton = function(self)
                    self.forwardUpdated = true
                end,
            }
            "#,
        )
        .expect("character-create test surface should install");

        patch(&env);

        let (race_id, class_id, faction, back_text, back_arrow, forward_updated): (
            i64,
            i64,
            String,
            String,
            String,
            bool,
        ) = env
            .eval(
                r#"
                local raceFrame = CharacterCreateFrame.RaceAndClassFrame
                return raceFrame.selectedRaceData.raceID,
                    raceFrame.selectedClassData.classID,
                    raceFrame.selectedFaction,
                    CharacterCreateFrame.BackButton.text,
                    CharacterCreateFrame.BackButton.arrow,
                    CharacterCreateFrame.forwardUpdated
                "#,
            )
            .expect("seeded character-create state should be readable");

        assert_eq!(race_id, 9);
        assert_eq!(class_id, 5);
        assert_eq!(faction, "RaceFaction9");
        assert_eq!(back_text, "Back");
        assert_eq!(back_arrow, "<");
        assert!(forward_updated);
    }

    #[test]
    fn create_character_wrapper_seeds_state_and_admin_name() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            BACK = "Back"
            BACKWARD_ARROW = "<"
            C_CharacterCreation = {
                GetSelectedRace = function()
                    return 1
                end,
                GetRaceDataByID = function(raceID)
                    return { raceID = raceID }
                end,
                GetSelectedClass = function()
                    return { classID = 2 }
                end,
                GetFactionForRace = function()
                    return "Alliance"
                end,
            }
            A_Admin = {
                SetPlayerName = function(name)
                    A_Admin.playerName = name
                end,
            }
            CharacterCreateMixin = {
                CreateCharacter = function(self)
                    self.created = true
                    return "created"
                end,
            }
            CharacterCreateFrame = {
                RaceAndClassFrame = {},
                GetSelectedName = function()
                    return "Calia"
                end,
                UpdateForwardButton = function() end,
            }
            "#,
        )
        .expect("create-character wrapper test surface should install");

        patch(&env);

        let (result, created, player_name, selected_faction): (String, bool, String, String) = env
            .eval(
                r#"
                local result = CharacterCreateMixin.CreateCharacter(CharacterCreateFrame)
                return result,
                    CharacterCreateFrame.created,
                    A_Admin.playerName,
                    CharacterCreateFrame.RaceAndClassFrame.selectedFaction
                "#,
            )
            .expect("wrapped CreateCharacter state should be readable");

        assert_eq!(result, "created");
        assert!(created);
        assert_eq!(player_name, "Calia");
        assert_eq!(selected_faction, "Alliance");
    }

    #[test]
    fn background_overlay_fallback_sets_array_or_single_texture_alpha() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            CharacterCreateMixin = {
                UpdateBackgroundOverlays = function()
                    error("missing race/class art")
                end,
            }
            CharacterCreateFrame = { RaceAndClassFrame = {} }
            "#,
        )
        .expect("background overlay test surface should install");

        patch(&env);

        let (array_alpha, single_alpha): (i64, i64) = env
            .eval(
                r#"
                local arrayTexture = {
                    SetAlpha = function(self, alpha)
                        self.alpha = alpha
                    end,
                }
                CharacterCreateMixin.UpdateBackgroundOverlays({ BGTex = { arrayTexture } })

                local singleTexture = {
                    SetAlpha = function(self, alpha)
                        self.alpha = alpha
                    end,
                }
                CharacterCreateMixin.UpdateBackgroundOverlays({ BGTex = singleTexture })

                return arrayTexture.alpha, singleTexture.alpha
                "#,
            )
            .expect("background overlay fallback alpha should be readable");

        assert_eq!(array_alpha, 1);
        assert_eq!(single_alpha, 1);
    }
}
