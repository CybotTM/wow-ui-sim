use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn diff_c_namespaces_missing_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("docs")
        .join("wow-client-diff")
        .join("diff_c_namespaces_missing.txt")
}

fn read_missing_namespaces() -> BTreeSet<String> {
    fs::read_to_string(diff_c_namespaces_missing_path())
        .expect("failed to read diff_c_namespaces_missing.txt")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

#[test]
fn diff_c_namespaces_missing_file_excludes_seeded_namespace_stubs() {
    let missing = read_missing_namespaces();
    let expected_absent = [
        "C_AccountServices",
        "C_ArrowCalloutManager",
        "C_EncounterEvents",
        "C_PrototypeDialog",
        "C_Reincarnation",
        "C_TableUtil",
        "C_TradeSkillUI",
        "C_Tutorial",
    ];

    for namespace in expected_absent {
        assert!(
            !missing.contains(namespace),
            "{namespace} should not remain in diff_c_namespaces_missing.txt"
        );
    }
}

#[test]
fn seeded_c_namespace_tables_exist_with_representative_methods() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local expectations = {
                { "C_AccountServices", "SaveAccountData" },
                { "C_ArrowCalloutManager", "ShowCallout" },
                { "C_EncounterEvents", "GetEventList" },
                { "C_PrototypeDialog", "SelectOption" },
                { "C_Reincarnation", "StartReincarnation" },
                { "C_TableUtil", "FindIndexedMismatch" },
                { "C_TradeSkillUI", "GetTradeSkillLine" },
                { "C_Tutorial", "GetTutorialStatus" },
            }

            for _, expectation in ipairs(expectations) do
                local namespace = _G[expectation[1]]
                if type(namespace) ~= "table" then
                    return expectation[1] .. ":missing_namespace"
                end
                if type(namespace[expectation[2]]) ~= "function" then
                    return expectation[1] .. ":missing_method:" .. expectation[2]
                end
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "seeded namespace tables should exist with representative methods"
    );
}
