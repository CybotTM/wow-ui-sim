use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn diff_c_functions_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("docs")
        .join("wow-client-diff")
        .join("diff_c_functions_missing.txt")
}

fn read_missing_c_functions() -> BTreeSet<String> {
    fs::read_to_string(diff_c_functions_path())
        .expect("failed to read diff_c_functions_missing.txt")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

#[test]
fn diff_c_functions_missing_excludes_reconciled_high_impact_functions() {
    let missing = read_missing_c_functions();
    let expected_absent = [
        "C_AccountServices.SaveAccountData",
        "C_EncounterEvents.GetEventList",
        "C_PrototypeDialog.SelectOption",
        "C_PetBattles.GetAbilityInfoByID",
        "C_PetBattles.GetHealth",
        "C_WoWLabsMatchmaking.SetPlayerReady",
        "C_WoWLabsMatchmaking.AcceptPartyInvite",
        "C_WowLabsDataManager.GetWoWLabsAreaInfo",
        "C_WowLabsDataManager.SelectWoWLabsArea",
        "C_TooltipInfo.GetAction",
        "C_TooltipInfo.GetInventoryItem",
        "C_TooltipInfo.GetSocketedItem",
        "C_TooltipInfo.GetSocketGem",
        "C_TooltipInfo.GetExistingSocketGem",
        "C_TooltipInfo.GetWorldCursor",
    ];

    for function_name in expected_absent {
        assert!(
            !missing.contains(function_name),
            "{function_name} should not remain in diff_c_functions_missing.txt"
        );
    }
}

#[test]
fn reconciled_high_impact_c_functions_exist_at_runtime() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local expectations = {
                { "C_AccountServices", "SaveAccountData" },
                { "C_EncounterEvents", "GetEventList" },
                { "C_PrototypeDialog", "SelectOption" },
                { "C_PetBattles", "GetAbilityInfoByID" },
                { "C_PetBattles", "GetHealth" },
                { "C_WoWLabsMatchmaking", "SetPlayerReady" },
                { "C_WoWLabsMatchmaking", "AcceptPartyInvite" },
                { "C_WowLabsDataManager", "GetWoWLabsAreaInfo" },
                { "C_WowLabsDataManager", "SelectWoWLabsArea" },
                { "C_TooltipInfo", "GetSocketedItem" },
                { "C_TooltipInfo", "GetSocketGem" },
                { "C_TooltipInfo", "GetExistingSocketGem" },
                { "C_TooltipInfo", "GetWorldCursor" },
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
        "High-impact C_* functions should exist at runtime"
    );
}
