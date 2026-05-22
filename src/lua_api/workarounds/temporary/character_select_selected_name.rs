//! Temporary character-select selected-name guard.
//!
//! The character-select name label is not always materialized when the
//! selected-character callback runs during simulator startup. Guard the
//! callback until character-select frame construction is modeled more closely.

use crate::lua_api::WowLuaEnv;

const CHARACTER_SELECT_SELECTED_NAME_LUA: &str = r#"
if type(CharacterSelect_SetSelectedCharacterName) == "function"
    and not rawget(_G, "__wow_character_select_selected_name_patched") then
    local original = CharacterSelect_SetSelectedCharacterName
    CharacterSelect_SetSelectedCharacterName = function(name, timerunningSeasonID)
        if type(CharSelectCharacterName) ~= "table" then
            return
        end
        return original(name, timerunningSeasonID)
    end
    rawset(_G, "__wow_character_select_selected_name_patched", true)
end
"#;

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(CHARACTER_SELECT_SELECTED_NAME_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwards_to_original_when_name_label_exists() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            calls = {}
            CharSelectCharacterName = {}
            CharacterSelect_SetSelectedCharacterName = function(name, timerunningSeasonID)
                table.insert(calls, {
                    name = name,
                    timerunningSeasonID = timerunningSeasonID,
                })
                return "selected"
            end
            "#,
        )
        .expect("character-select fixture should install");

        patch(&env);

        let (result, call_count, selected_name, season_id): (String, i64, String, i64) = env
            .eval(
                r#"
                local result = CharacterSelect_SetSelectedCharacterName("Haky", 12)
                return result, #calls, calls[1].name, calls[1].timerunningSeasonID
                "#,
            )
            .expect("selected-name wrapper should be callable");

        assert_eq!(result, "selected");
        assert_eq!(call_count, 1);
        assert_eq!(selected_name, "Haky");
        assert_eq!(season_id, 12);
    }

    #[test]
    fn skips_original_when_name_label_is_missing() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            calls = 0
            CharSelectCharacterName = nil
            CharacterSelect_SetSelectedCharacterName = function()
                calls = calls + 1
                error("missing name label should not call original")
            end
            "#,
        )
        .expect("character-select fixture should install");

        patch(&env);

        let (result_is_nil, call_count): (bool, i64) = env
            .eval(
                r#"
                local result = CharacterSelect_SetSelectedCharacterName("Haky", 12)
                return result == nil, calls
                "#,
            )
            .expect("selected-name wrapper should tolerate missing label");

        assert!(result_is_nil);
        assert_eq!(call_count, 0);
    }

    #[test]
    fn patch_is_idempotent() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            calls = 0
            CharSelectCharacterName = {}
            CharacterSelect_SetSelectedCharacterName = function()
                calls = calls + 1
            end
            "#,
        )
        .expect("character-select fixture should install");

        patch(&env);
        patch(&env);

        let call_count: i64 = env
            .eval(
                r#"
                CharacterSelect_SetSelectedCharacterName("Haky")
                return calls
                "#,
            )
            .expect("selected-name wrapper should stay single-wrapped");

        assert_eq!(call_count, 1);
    }
}
