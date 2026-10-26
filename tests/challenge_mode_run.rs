//! Supplied challenge state through the selected original tracker consumers.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever"
))]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_tracker() -> WowLuaEnv {
    let ui = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui);
    let loaded =
        load_blizzard_addon_closure_into_env(&env, &ui, &["Blizzard_ObjectiveTracker"], &[]);
    assert!(
        loaded
            .iter()
            .any(|name| name == "Blizzard_ObjectiveTracker")
    );
    env.exec(r#"
        assert(ScenarioTimerFrame and ScenarioObjectiveTracker.ChallengeModeBlock)
        assert(ScenarioTimerFrame.CheckTimers == ScenarioTimerMixin.CheckTimers)
        assert(ScenarioObjectiveTracker.ChallengeModeBlock.Activate == ScenarioObjectiveTrackerChallengeModeMixin.Activate)
        local source = debug.getinfo(ScenarioTimerFrame.CheckTimers, 'S').source:gsub('\\', '/')
        assert(source:find('Blizzard_ObjectiveTracker/Blizzard_ScenarioObjectiveTracker.lua', 1, true), source)
    "#).unwrap();
    env
}

const START: &str = r#"
    A_Admin.SetChallengeModeMapInfo(9001, {
        name='Supplied forecast dungeon', id=9001, timeLimit=1500,
        texture=123, backgroundTexture=456, mapID=789,
    })
    A_Admin.StartChallengeRun({mapChallengeModeID=9001, level=12, affixIDs={},
        wasActiveKeystoneCharged=true, numDeaths=2, timeLost=30})
    A_Admin.StartWorldElapsedTimer(1, 'Supplied timer', 1, 120)
"#;

#[test]
fn original_challenge_timer_reads_supplied_run_and_reset() {
    let env = load_tracker();
    env.exec(START).unwrap();
    env.exec(
        r#"
        local block = ScenarioObjectiveTracker.ChallengeModeBlock
        assert(block:IsActive() and block.timerID == 1)
        assert(block.timeLimit == 1500 and block.Level:GetText():find('12', 1, true))
        assert(block.StatusBar:GetValue() == 1380)
        assert(block.deathCount == 2 and block.timeLost == 30)
        assert(not block.wasDepleted)
        A_Admin.SetWorldElapsedTime(1, 345)
        ScenarioTimerFrame:StartTimer(block)
        ScenarioTimerFrame:OnUpdate(0)
        assert(block.StatusBar:GetValue() == 1155)
        A_Admin.ClearChallengeRun()
        assert(not block:IsActive() and not ScenarioTimerFrame:IsShown())
        assert(C_ChallengeMode.GetActiveChallengeMapID() == nil)
        assert(not C_ChallengeMode.IsChallengeModeActive())
        assert(select('#', GetWorldElapsedTimers()) == 0)
    "#,
    )
    .unwrap();
    env.exec(START).unwrap();
    env.exec(
        r#"
        assert(ScenarioObjectiveTracker.ChallengeModeBlock.StatusBar:GetValue() == 1380)
        A_Admin.CompleteChallengeRun({mapChallengeModeID=9001, level=12,
            time=1234000, onTime=true, keystoneUpgradeLevels=2})
        assert(C_ChallengeMode.GetChallengeCompletionInfo().time == 1234000)
        assert(not C_ChallengeMode.IsChallengeModeActive())
        assert(not ScenarioObjectiveTracker.ChallengeModeBlock:IsActive())
    "#,
    )
    .unwrap();
}

#[test]
fn original_scenario_criteria_render_order_quantity_and_completion() {
    let env = load_tracker();
    env.exec(r#"
        A_Admin.SetScenarioCriteria({stepID=1, title='Fixture stage', description='Fixture objectives'}, {
            {criteriaID=901, description='First boss', criteriaType=165, quantity=0, totalQuantity=1, quantityString='0'},
            {criteriaID=902, description='Enemy forces', quantity=25, totalQuantity=100,
                quantityString='25%', isWeightedProgress=true},
        })
        ScenarioObjectiveTracker:SetShouldShowCriteria(true)
        ScenarioObjectiveTracker:UpdateCriteria(C_ScenarioInfo.GetScenarioStepInfo().numCriteria)
        local block = ScenarioObjectiveTracker.ObjectivesBlock
        assert(block:GetExistingLine(1).Text:GetText() == '0/1 First boss')
        assert(block:GetExistingLine(2).Text:GetText() == 'Enemy forces')
        assert(C_ScenarioInfo.GetCriteriaInfo(2).quantityString == '25%')
        assert(C_ScenarioInfo.GetCriteriaInfoByStep(1, 1).criteriaID == 901)
        A_Admin.SetScenarioCriteria({stepID=1, title='Fixture stage', description='Fixture objectives'}, {
            {criteriaID=901, description='First boss', criteriaType=165, quantity=1, totalQuantity=1, quantityString='1', completed=true},
            {criteriaID=902, description='Enemy forces', quantity=70, totalQuantity=100,
                quantityString='70%', isWeightedProgress=true},
        })
        ScenarioObjectiveTracker:UpdateCriteria(2)
        assert(block:GetExistingLine(1).Text:GetText() == '1/1 First boss')
        assert(C_ScenarioInfo.GetCriteriaInfo(1).completed)
        assert(C_ScenarioInfo.GetCriteriaInfo(2).quantity == 70)
        assert(C_ScenarioInfo.GetCriteriaInfo(3) == nil)
        local row = C_ScenarioInfo.GetCriteriaInfo(2)
        row.quantity = 999
        assert(C_ScenarioInfo.GetCriteriaInfo(2).quantity == 70)
        assert(C_Scenario.GetCriteriaInfo == nil and C_Scenario.GetCriteriaInfoByStep == nil)
    "#).unwrap();
    let independent = WowLuaEnv::new().unwrap();
    independent.exec("assert(C_ScenarioInfo.GetCriteriaInfo(1) == nil); assert(C_ChallengeMode.GetActiveChallengeMapID() == nil)").unwrap();
}

#[test]
fn challenge_admin_events_observe_updated_state_and_reject_invalid_input() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        local events = {}
        local f = CreateFrame('Frame')
        for _, event in ipairs({'CHALLENGE_MODE_START', 'WORLD_STATE_TIMER_START', 'SCENARIO_CRITERIA_UPDATE', 'CHALLENGE_MODE_RESET', 'CHALLENGE_MODE_COMPLETED'}) do f:RegisterEvent(event) end
        f:SetScript('OnEvent', function(_, event, id)
            events[#events+1] = event
            if event == 'CHALLENGE_MODE_START' then assert(C_ChallengeMode.GetActiveChallengeMapID() == 9001)
            elseif event == 'WORLD_STATE_TIMER_START' then
                assert(id == 1 and GetWorldElapsedTimers() == 1)
                local name, elapsed, kind = GetWorldElapsedTime(id)
                assert(name == 'Supplied timer' and elapsed == 120 and kind == 1)
            elseif event == 'SCENARIO_CRITERIA_UPDATE' then assert(id == 901 and C_ScenarioInfo.GetCriteriaInfo(1).quantity == 7)
            else assert(not C_ChallengeMode.IsChallengeModeActive()) end
        end)
        A_Admin.StartChallengeRun({mapChallengeModeID=9001, level=2, affixIDs={9,10}, wasActiveKeystoneCharged=false, numDeaths=0, timeLost=0})
        local level, affixes, charged = C_ChallengeMode.GetActiveKeystoneInfo()
        assert(level == 2 and #affixes == 2 and affixes[2] == 10 and charged == false)
        affixes[1] = 999
        assert(select(2, C_ChallengeMode.GetActiveKeystoneInfo())[1] == 9)
        A_Admin.StartWorldElapsedTimer(1, 'Supplied timer', 1, 120)
        A_Admin.AdvanceWorldElapsedTime(1, 10)
        assert(select(2, GetWorldElapsedTime(1)) == 130)
        A_Admin.SetScenarioCriteria({stepID=1, title='Supplied', description='Supplied'}, {{criteriaID=901, description='Supplied', quantity=7, quantityString='7'}})
        A_Admin.ClearChallengeRun()
        assert(table.concat(events, ',') == 'CHALLENGE_MODE_START,WORLD_STATE_TIMER_START,SCENARIO_CRITERIA_UPDATE,CHALLENGE_MODE_RESET')
        assert(not pcall(A_Admin.AdvanceWorldElapsedTime, 1, -1))
        assert(not pcall(A_Admin.StartWorldElapsedTimer, 1, 'bad', 1, 0))
    "#).unwrap();
}
