//! Temporary `C_CampaignInfo` and `C_CovenantSanctumUI` placeholder defaults.
//!
//! Campaign and covenant progression are not modeled yet. These defaults keep
//! legacy panels on stable no-progress table shapes until real state exists.

const CAMPAIGN_COVENANT_DEFAULTS_LUA: &str = r#"
C_CampaignInfo = C_CampaignInfo or __wow_namespace()
C_CovenantSanctumUI = C_CovenantSanctumUI or __wow_namespace()

local function campaignIDArg(campaignID)
    return tonumber(campaignID) or 0
end

local function campaignName(campaignID)
    if campaignID == 290 then
        return "Broken Shore"
    end

    return "Campaign " .. campaignID
end

local function invalidCampaignState()
    if Enum and Enum.CampaignState and Enum.CampaignState.Invalid ~= nil then
        return Enum.CampaignState.Invalid
    end

    return 0
end

if rawget(C_CampaignInfo, "GetCampaignID") == nil then
    function C_CampaignInfo.GetCampaignID(campaignID)
        return campaignIDArg(campaignID)
    end
end

if rawget(C_CampaignInfo, "GetCampaignInfo") == nil then
    function C_CampaignInfo.GetCampaignInfo(campaignID)
        local normalizedID = campaignIDArg(campaignID)
        return {
            campaignID = normalizedID,
            id = normalizedID,
            name = campaignName(normalizedID),
        }
    end
end

if rawget(C_CampaignInfo, "GetState") == nil then
    function C_CampaignInfo.GetState(_campaignID)
        return invalidCampaignState()
    end
end

if rawget(C_CovenantSanctumUI, "GetRenownRewardsForLevel") == nil then
    function C_CovenantSanctumUI.GetRenownRewardsForLevel(factionID, level)
        if tonumber(factionID) == 1 and tonumber(level) == 5 then
            return {
                {
                    name = "Path of Ascension",
                    description = "Unlocks a new covenant activity.",
                    toastDescription = "Path of Ascension unlocked",
                    icon = 4089529,
                },
            }
        end

        return {}
    end
end

if rawget(C_CovenantSanctumUI, "GetSoulCurrencies") == nil then
    function C_CovenantSanctumUI.GetSoulCurrencies()
        return {}
    end
end

if rawget(C_CovenantSanctumUI, "GetAnimaInfo") == nil then
    function C_CovenantSanctumUI.GetAnimaInfo()
        return 0, 0
    end
end

if rawget(C_CovenantSanctumUI, "CanDepositAnima") == nil then
    function C_CovenantSanctumUI.CanDepositAnima()
        return false
    end
end

if rawget(C_CovenantSanctumUI, "DepositAnima") == nil then
    function C_CovenantSanctumUI.DepositAnima()
    end
end

if rawget(C_CovenantSanctumUI, "EndInteraction") == nil then
    function C_CovenantSanctumUI.EndInteraction()
    end
end

if rawget(C_CovenantSanctumUI, "GetFeatures") == nil then
    function C_CovenantSanctumUI.GetFeatures()
        return {}
    end
end

if rawget(C_CovenantSanctumUI, "GetCurrentTalentTreeID") == nil then
    function C_CovenantSanctumUI.GetCurrentTalentTreeID()
        return 0
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(CAMPAIGN_COVENANT_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_campaign_and_covenant_placeholder_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (i32, i32, String, bool, i32, i32, i32, bool, i32) = env
            .eval(
                r#"
                local campaign = C_CampaignInfo.GetCampaignInfo(290)
                local fallbackCampaign = C_CampaignInfo.GetCampaignInfo("12")
                local anima, maxAnima = C_CovenantSanctumUI.GetAnimaInfo()
                return C_CampaignInfo.GetCampaignID("12"),
                       campaign.id,
                       campaign.name,
                       C_CampaignInfo.GetState(99999) == Enum.CampaignState.Invalid,
                       fallbackCampaign.campaignID,
                       anima,
                       maxAnima,
                       C_CovenantSanctumUI.CanDepositAnima(),
                       C_CovenantSanctumUI.GetCurrentTalentTreeID()
                "#,
            )
            .expect("campaign/covenant defaults should be callable");

        assert_eq!(
            result,
            (
                12,
                290,
                "Broken Shore".to_string(),
                true,
                12,
                0,
                0,
                false,
                0
            )
        );
    }

    #[test]
    fn installs_path_of_ascension_reward_shape() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (i32, String, String, String, i32, i32) = env
            .eval(
                r#"
                local rewards = C_CovenantSanctumUI.GetRenownRewardsForLevel(1, 5)
                local emptyRewards = C_CovenantSanctumUI.GetRenownRewardsForLevel(2, 5)
                local reward = rewards[1]
                return #rewards,
                       reward.name,
                       reward.description,
                       reward.toastDescription,
                       reward.icon,
                       #emptyRewards
                "#,
            )
            .expect("covenant reward defaults should be callable");

        assert_eq!(
            result,
            (
                1,
                "Path of Ascension".to_string(),
                "Unlocks a new covenant activity.".to_string(),
                "Path of Ascension unlocked".to_string(),
                4_089_529,
                0,
            )
        );
    }

    #[test]
    fn preserves_existing_campaign_and_covenant_providers() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_CampaignInfo = C_CampaignInfo or __wow_namespace()
            C_CovenantSanctumUI = C_CovenantSanctumUI or __wow_namespace()

            function C_CampaignInfo.GetCampaignInfo(_campaignID)
                return { id = 77, name = "Existing Campaign" }
            end

            function C_CovenantSanctumUI.GetAnimaInfo()
                return 5, 10
            end
            "#,
        )
        .expect("fixture should install existing campaign/covenant providers");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: (i32, String, i32, i32) = env
            .eval(
                r#"
                local campaign = C_CampaignInfo.GetCampaignInfo(290)
                local anima, maxAnima = C_CovenantSanctumUI.GetAnimaInfo()
                return campaign.id, campaign.name, anima, maxAnima
                "#,
            )
            .expect("existing campaign/covenant providers should remain callable");

        assert_eq!(result, (77, "Existing Campaign".to_string(), 5, 10));
    }
}
