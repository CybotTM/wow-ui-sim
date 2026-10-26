//! Declaration-backed per-member publication, including unsupported profiles.
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn challenge_run_members_follow_pinned_profile_declarations() {
    let env = WowLuaEnv::new().unwrap();
    if ACTIVE == ClientProfile::Wrath {
        env.exec(
            r#"
            assert(rawget(C_ChallengeMode, 'GetActiveChallengeMapID') == nil)
            assert(rawget(C_ChallengeMode, 'GetActiveKeystoneInfo') == nil)
            assert(rawget(C_ChallengeMode, 'GetDeathCount') == nil)
            assert(rawget(C_ScenarioInfo, 'GetCriteriaInfo') == nil)
            assert(rawget(C_ScenarioInfo, 'GetCriteriaInfoByStep') == nil)
        "#,
        )
        .unwrap();
        return;
    }
    env.exec(
        r#"
        assert(C_ChallengeMode.GetActiveChallengeMapID() == nil)
        assert(not C_ChallengeMode.IsChallengeModeActive())
        A_Admin.StartChallengeRun({mapChallengeModeID=9011, eventMapID=7011, level=3,
            affixIDs={9}, wasActiveKeystoneCharged=false, numDeaths=1, timeLost=8})
        assert(C_ChallengeMode.GetActiveChallengeMapID() == 9011)
        assert(C_ChallengeMode.IsChallengeModeActive())
        A_Admin.SetScenarioCriteria({stepID=4, title='Fourth', description='Supplied'}, {
            {criteriaID=11, description='Forces', quantityString='41%', quantity=41,
                totalQuantity=100, isWeightedProgress=true},
        })
        assert(C_ScenarioInfo.GetScenarioStepInfo().stepID == 4)
        assert(C_ScenarioInfo.GetScenarioStepInfo().numCriteria == 1)
        assert(C_ScenarioInfo.GetCriteriaInfo(1).quantity == 41)
        assert(C_ScenarioInfo.GetCriteriaInfoByStep(4, 1).quantityString == '41%')
        assert(C_ScenarioInfo.GetCriteriaInfoByStep(5, 1) == nil)
        A_Admin.ClearChallengeRun()
        assert(C_ChallengeMode.GetActiveChallengeMapID() == nil)
        assert(C_ScenarioInfo.GetCriteriaInfo(1) == nil)
    "#,
    )
    .unwrap();
    if matches!(
        ACTIVE,
        ClientProfile::Retail | ClientProfile::Ptr | ClientProfile::WowForever
    ) {
        env.exec(
            r#"
            assert(select('#', C_ChallengeMode.GetDeathCount()) == 0)
        "#,
        )
        .unwrap();
    } else {
        env.exec(
            r#"
            assert(rawget(C_ChallengeMode, 'GetActiveKeystoneInfo') == nil)
            assert(rawget(C_ChallengeMode, 'GetDeathCount') == nil)
        "#,
        )
        .unwrap();
    }
}
