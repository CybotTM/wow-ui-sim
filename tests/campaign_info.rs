use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn campaign_info_get_state_defaults_to_invalid_for_unknown_campaigns() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_CampaignInfo.GetState(0) ~= Enum.CampaignState.Invalid then
                return "campaign_zero_should_be_invalid"
            end

            if C_CampaignInfo.GetState(99999) ~= Enum.CampaignState.Invalid then
                return "unknown_campaign_should_be_invalid"
            end

            local campaignID = C_CampaignInfo.GetCampaignID(42)
            if C_CampaignInfo.GetState(campaignID) ~= Enum.CampaignState.Invalid then
                return "derived_campaign_id_should_be_invalid"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "C_CampaignInfo.GetState should default missing campaigns to Invalid"
    );
}
