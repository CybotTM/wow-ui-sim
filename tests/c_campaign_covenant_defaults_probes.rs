use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn campaign_defaults_return_stable_placeholder_info() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_CampaignInfo.GetCampaignID(290) ~= 290 then return "id" end
            local known = C_CampaignInfo.GetCampaignInfo(290)
            if known.campaignID ~= 290 or known.id ~= 290 or known.name ~= "Broken Shore" then return "known" end
            local unknown = C_CampaignInfo.GetCampaignInfo(12345)
            if unknown.name ~= "Campaign 12345" then return "unknown" end
            if C_CampaignInfo.GetState(290) ~= Enum.CampaignState.Invalid then return "state" end
            return "ok"
            "#,
        )
        .expect("campaign defaults should be callable");

    assert_eq!(result, "ok");
}

#[test]
fn covenant_sanctum_defaults_return_empty_state_and_one_known_reward() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local rewards = C_CovenantSanctumUI.GetRenownRewardsForLevel(1, 5)
            if #rewards ~= 1 or rewards[1].name ~= "Path of Ascension" then return "reward" end
            if #C_CovenantSanctumUI.GetRenownRewardsForLevel(1, 6) ~= 0 then return "other-rewards" end
            if #C_CovenantSanctumUI.GetSoulCurrencies() ~= 0 then return "currencies" end
            local anima, maxAnima = C_CovenantSanctumUI.GetAnimaInfo()
            if anima ~= 0 or maxAnima ~= 0 then return "anima" end
            if C_CovenantSanctumUI.CanDepositAnima() ~= false then return "deposit" end
            if C_CovenantSanctumUI.DepositAnima() ~= nil then return "deposit-call" end
            if C_CovenantSanctumUI.EndInteraction() ~= nil then return "end" end
            if #C_CovenantSanctumUI.GetFeatures() ~= 0 then return "features" end
            if C_CovenantSanctumUI.GetCurrentTalentTreeID() ~= 0 then return "tree" end
            return "ok"
            "#,
        )
        .expect("covenant sanctum defaults should be callable");

    assert_eq!(result, "ok");
}
