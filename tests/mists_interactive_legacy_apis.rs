#![cfg(feature = "client-mists")]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn mists_bootstrap_supplies_interactive_legacy_panel_apis() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r##"
            local currencyReturns = select("#", GetCurrencyListInfo(1))
            local setActionOk = pcall(function() GameTooltip:SetAction(1) end)
            local setCurrencyOk = pcall(function() GameTooltip:SetCurrencyToken(1) end)
            return type(GetUnitSpeed) == "function" and GetUnitSpeed("player") == 0,
                type(HasPetUI) == "function" and HasPetUI() == false,
                type(GetNumFactions) == "function" and GetNumFactions() == C_Reputation.GetNumFactions(),
                type(GetCurrencyListInfo) == "function" and currencyReturns >= 7,
                type(SelectQuestLogEntry) == "function" and SelectQuestLogEntry(1) == nil,
                type(GetGuildInfoText) == "function" and GetGuildInfoText() == "",
                type(GameTooltip.SetAction) == "function",
                setActionOk,
                type(GameTooltip.SetCurrencyToken) == "function",
                setCurrencyOk,
                type(GetExpansionForLevel) == "function" and GetExpansionForLevel(90) == LE_EXPANSION_MISTS_OF_PANDARIA
            "##,
        )
        .expect("Mists interactive legacy APIs should be callable");

    assert_eq!(
        result,
        (
            true, true, true, true, true, true, true, true, true, true, true,
        ),
        "Mists lazy-loaded panels and tooltips should see concrete legacy APIs"
    );
}
