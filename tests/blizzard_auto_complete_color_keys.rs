use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";

#[test]
fn blizzard_auto_complete_color_keys_are_keyed_by_priority_enum() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before color keys can be checked. Loaded set: {loaded:?}"
                );

                let (ok, message): (bool, Option<String>) = env
                    .eval(
                        r#"
                        local priorities = {
                            "Other",
                            "Interacted",
                            "InGroup",
                            "Guild",
                            "Friend",
                            "AccountCharacter",
                            "AccountCharacterSameRealm",
                        }

                        if type(AUTOCOMPLETE_COLOR_KEYS) ~= "table" then
                            return false, "AUTOCOMPLETE_COLOR_KEYS is not a table"
                        end

                        for _, priorityName in ipairs(priorities) do
                            local priority = Enum.AutoCompletePriority[priorityName]
                            if type(priority) ~= "number" then
                                return false, "Enum.AutoCompletePriority."..priorityName.." is not numeric"
                            end

                            local entry = AUTOCOMPLETE_COLOR_KEYS[priority]
                            if type(entry) ~= "table" then
                                return false, priorityName.." entry is not a table"
                            end
                            if entry.key == nil then
                                return false, priorityName..".key is nil"
                            end
                            if entry.text == nil then
                                return false, priorityName..".text is nil"
                            end
                        end

                        return true, nil
                        "#,
                    )
                    .expect("AutoComplete color-key shape probe should run");

                assert!(
                    ok,
                    "`AUTOCOMPLETE_COLOR_KEYS` shape mismatch: {}",
                    message.unwrap_or_else(|| "unknown mismatch".to_string())
                );
            });
        });
    });
}
