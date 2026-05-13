use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";

#[test]
fn blizzard_auto_complete_list_templates_expose_documented_key_shapes() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before its list templates can be checked. \
                     Loaded set: {loaded:?}"
                );

                let (ok, message): (bool, Option<String>) = env
                    .eval(
                        r#"
                        local expected = {
                            "ALL",
                            "ALL_OTHERS",
                            "ALL_OTHER_CHARS",
                            "ALL_CHARS",
                            "FRIENDLY_CHARS",
                            "ONLINE",
                            "ONLINE_NOT_BNET",
                            "ONLINE_NOT_IN_GROUP",
                            "ONLINE_NOT_IN_GUILD",
                            "NOT_FRIEND",
                            "IN_GROUP",
                            "IN_GUILD",
                            "FRIEND",
                            "FRIEND_NOT_GUILD",
                            "FRIEND_AND_GUILD",
                            "KNOWN",
                            "KNOWN_NOT_GUILD",
                            "BNET_NOT_IN_PARTY",
                            "ALL_BNET",
                        }

                        if type(AUTOCOMPLETE_LIST_TEMPLATES) ~= "table" then
                            return false, "AUTOCOMPLETE_LIST_TEMPLATES is not a table"
                        end

                        for _, key in ipairs(expected) do
                            local entry = AUTOCOMPLETE_LIST_TEMPLATES[key]
                            if type(entry) ~= "table" then
                                return false, key.." entry is not a table"
                            end
                            if type(entry.include) ~= "number" then
                                return false, key..".include is not numeric"
                            end
                            if type(entry.exclude) ~= "number" then
                                return false, key..".exclude is not numeric"
                            end
                        end

                        return true, nil
                        "#,
                    )
                    .expect("AutoComplete list-template shape probe should run");

                assert!(
                    ok,
                    "`AUTOCOMPLETE_LIST_TEMPLATES` shape mismatch: {}",
                    message.unwrap_or_else(|| "unknown mismatch".to_string())
                );
            });
        });
    });
}

#[test]
fn blizzard_auto_complete_list_aliases_reference_template_entries() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before its use-case aliases can be checked. \
                     Loaded set: {loaded:?}"
                );

                let (ok, message): (bool, Option<String>) = env
                    .eval(
                        r#"
                        local aliases = {
                            ALL = "ALL",
                            WHISPER = "ONLINE_NOT_BNET",
                            SMART_WHISPER = "ONLINE",
                            WHISPER_EXTRACT = "ALL_OTHER_CHARS",
                            SMART_WHISPER_EXTRACT = "ALL_OTHERS",
                            INVITE = "ONLINE_NOT_IN_GROUP",
                            UNINVITE = "IN_GROUP",
                            PROMOTE = "IN_GROUP",
                            TEAM_INVITE = "ONLINE_NOT_BNET",
                            GUILD_INVITE = "ONLINE_NOT_IN_GUILD",
                            GUILD_UNINVITE = "IN_GUILD",
                            GUILD_PROMOTE = "IN_GUILD",
                            GUILD_DEMOTE = "IN_GUILD",
                            GUILD_LEADER = "IN_GUILD",
                            ADDFRIEND = "NOT_FRIEND",
                            FRIENDS = "NOT_FRIEND",
                            REMOVEFRIEND = "FRIEND",
                            CHANINVITE = "ONLINE_NOT_BNET",
                            MAIL = "ALL_CHARS",
                            CALENDARGUILDEVENT = "KNOWN_NOT_GUILD",
                            CALENDAREVENT = "KNOWN",
                            IGNORE = "NOT_FRIEND",
                            LOOT_MASTER = "IN_GROUP",
                            WARGAME = "BNET_NOT_IN_PARTY",
                            COMMUNITY = "ALL_OTHER_CHARS",
                            CRAFTINGORDER = "ALL_CHARS",
                            HOUSE_FINDER = "ALL_BNET",
                        }

                        if type(AUTOCOMPLETE_LIST) ~= "table" then
                            return false, "AUTOCOMPLETE_LIST is not a table"
                        end

                        for alias, templateKey in pairs(aliases) do
                            if AUTOCOMPLETE_LIST[alias] ~= AUTOCOMPLETE_LIST_TEMPLATES[templateKey] then
                                return false, alias.." does not reference "..templateKey
                            end
                        end

                        return true, nil
                        "#,
                    )
                    .expect("AutoComplete alias mapping probe should run");

                assert!(
                    ok,
                    "`AUTOCOMPLETE_LIST` alias mismatch: {}",
                    message.unwrap_or_else(|| "unknown mismatch".to_string())
                );
            });
        });
    });
}
