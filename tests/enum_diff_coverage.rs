use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use wow_ui_sim::lua_api::WowLuaEnv;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn parse_enum_names(file_name: &str) -> BTreeSet<String> {
    let path = manifest_dir().join("docs/wow-client-diff").join(file_name);
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

#[test]
fn diff_enums_missing_matches_live_runtime_gaps() {
    let env = WowLuaEnv::new().unwrap();
    let missing = parse_enum_names("diff_enums_missing.txt");

    let lua_list = missing
        .iter()
        .map(|name| format!("\"{}\"", name.replace('\\', "\\\\").replace('"', "\\\"")))
        .collect::<Vec<_>>()
        .join(", ");

    let script = format!(
        r#"
        local names = {{ {lua_list} }}
        local missing = {{}}
        for _, name in ipairs(names) do
            if type(Enum[name]) ~= "table" then
                table.insert(missing, name)
            end
        end
        return table.concat(missing, "\n")
        "#
    );

    let runtime_missing = env.eval::<String>(&script).unwrap();
    let runtime_missing: BTreeSet<String> = runtime_missing
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect();

    assert_eq!(missing, runtime_missing);
}

#[test]
fn representative_missing_enums_are_available_with_expected_values() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            local checks = {
                { "AbbreviationDataError", "InvalidBreakpoint", 0 },
                { "AccountData", "Config", 0 },
                { "AccountStoreItemStatus", "Owned", 3 },
                { "ClientDebugAISpellReadyStatus", "Ready", 0 },
                { "ClientDebugAISpellReadyStatusMeta", "NumValues", 32 },
            }

            for _, check in ipairs(checks) do
                local enumTable = Enum[check[1]]
                if type(enumTable) ~= "table" then
                    return "missing_enum:" .. check[1]
                end

                if enumTable[check[2]] ~= check[3] then
                    return "wrong_value:" .. check[1] .. "." .. check[2] .. "=" .. tostring(enumTable[check[2]])
                end
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}

#[test]
fn cooldown_layout_enums_are_available_with_expected_values() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            local checks = {
                { "CooldownLayoutStatus", "Success", 0 },
                { "CooldownLayoutStatus", "NoValidAlerts", 6 },
                { "CDMLayoutMode", "AccessOnly", false },
                { "CDMLayoutMode", "AllowCreate", true },
                { "CooldownLayoutAction", "ChangeOrder", 0 },
                { "CooldownLayoutAction", "AddAlert", 3 },
                { "CooldownLayoutType", "Character", 1 },
                { "CooldownLayoutType", "Account", 2 },
            }

            for _, check in ipairs(checks) do
                local enumTable = Enum[check[1]]
                if type(enumTable) ~= "table" then
                    return "missing_enum:" .. check[1]
                end

                if enumTable[check[2]] ~= check[3] then
                    return "wrong_value:" .. check[1] .. "." .. check[2] .. "=" .. tostring(enumTable[check[2]])
                end
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}

#[test]
fn edit_mode_chat_frame_display_only_setting_is_available_with_expected_values() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            local enumTable = Enum.EditModeChatFrameDisplayOnlySetting
            if type(enumTable) ~= "table" then
                return "missing_enum"
            end

            if enumTable.Width ~= 4 then
                return "wrong_width:" .. tostring(enumTable.Width)
            end

            if enumTable.Height ~= 5 then
                return "wrong_height:" .. tostring(enumTable.Height)
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}

#[test]
fn diff_enums_extra_is_empty_and_removed_runtime_enums_stay_absent() {
    let extra = parse_enum_names("diff_enums_extra.txt");
    assert!(
        extra.is_empty(),
        "expected no extra enums, found: {extra:?}"
    );

    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            local removed = {
                "ExpansionLandingPageType",
                "TransmogOutfitFlags",
            }

            for _, name in ipairs(removed) do
                if type(Enum[name]) == "table" then
                    return "unexpected_enum:" .. name
                end
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}
