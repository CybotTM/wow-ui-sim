#![cfg(feature = "client-mists")]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn mists_bootstrap_supplies_interactive_legacy_panel_apis() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: Vec<bool> = env
        .eval(
            r##"
            local currencyReturns = select("#", GetCurrencyListInfo(1))
            local baseSpeed, runSpeed, flightSpeed, swimSpeed = GetUnitSpeed("player")
            local factionName, factionDescription, standingID, barMin, barMax, barValue, atWarWith, canToggleAtWar, isHeader, isCollapsed, hasRep, isWatched, isChild, factionID = GetFactionInfo(1)
            local setActionOk = pcall(function() GameTooltip:SetAction(1) end)
            local setCurrencyOk = pcall(function() GameTooltip:SetCurrencyToken(1) end)
            return {
                type(GetUnitSpeed) == "function" and baseSpeed == 0 and runSpeed > 0 and flightSpeed > 0 and swimSpeed > 0,
                BASE_MOVEMENT_SPEED == runSpeed,
                type(HasPetUI) == "function" and HasPetUI() == false,
                type(GetNumFactions) == "function" and GetNumFactions() == C_Reputation.GetNumFactions(),
                type(GetFactionInfo) == "function" and type(factionName) == "string" and factionName ~= "" and type(factionDescription) == "string" and type(standingID) == "number" and type(barMin) == "number" and type(barMax) == "number" and type(barValue) == "number" and type(atWarWith) == "boolean" and type(canToggleAtWar) == "boolean" and type(isHeader) == "boolean" and type(isCollapsed) == "boolean" and type(hasRep) == "boolean" and type(isChild) == "boolean" and type(factionID) == "number",
                type(GetCurrencyListInfo) == "function" and currencyReturns >= 7,
                type(SelectQuestLogEntry) == "function" and SelectQuestLogEntry(1) == nil,
                type(GetGuildInfoText) == "function" and GetGuildInfoText() == "",
                type(GameTooltip.SetAction) == "function",
                setActionOk,
                type(GameTooltip.SetCurrencyToken) == "function",
                setCurrencyOk,
                type(GetExpansionForLevel) == "function" and GetExpansionForLevel(90) == LE_EXPANSION_MISTS_OF_PANDARIA,
                type(GetOverrideSpellPowerByAP) == "function" and GetOverrideSpellPowerByAP() == 0,
                type(GetOverrideAPBySpellPower) == "function" and GetOverrideAPBySpellPower() == 0,
                LE_UNIT_STAT_SPIRIT == 5,
                type(IsRangedWeapon) == "function" and IsRangedWeapon() == false
            }
            "##,
        )
        .expect("Mists interactive legacy APIs should be callable");

    assert!(
        result.iter().all(|value| *value),
        "Mists lazy-loaded panels and tooltips should see concrete legacy APIs: {result:?}"
    );
}
