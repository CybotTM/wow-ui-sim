//! QL-2 authored readiness/dialog policy; no reward or native-parity claim.
#![cfg(any(feature = "client-retail", feature = "client-wowforever"))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn seed(env: &WowLuaEnv) {
    {
        let mut sim = env.state().borrow_mut();
        let mut entry = sim.quest_log_entries.entries[0].clone();
        entry.quest_id = 990002;
        entry.title = "Readiness fixture".into();
        entry.is_complete = false;
        entry.is_failed = false;
        entry.is_flagged_completed = false;
        sim.quest_log_entries.entries.push(entry);
        sim.quest_log.push(990002);
        sim.gossip.active = true;
        let mut row = sim
            .gossip
            .active_quests
            .first()
            .cloned()
            .unwrap_or_default();
        row.quest_id = 990002;
        row.title = "Readiness fixture".into();
        row.is_complete = Some(false);
        sim.gossip.active_quests = vec![row];
    }
    env.exec(include_str!("quest_readiness.lua")).unwrap();
}

fn consumer_env() -> WowLuaEnv {
    let cache = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path()
        .expect("current-profile cache required, no source replacement");
    let (env, loaded) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &cache,
        &[
            "Blizzard_AccessibilityTemplates",
            "Blizzard_UIPanels_Game",
            "Blizzard_ObjectiveTracker",
            "Blizzard_FrameXML",
            "Blizzard_SplashFrame",
        ],
        &[],
    );
    assert!(loaded.iter().any(|name| name == "Blizzard_UIPanels_Game"));
    assert!(
        loaded
            .iter()
            .any(|name| name == "Blizzard_ObjectiveTracker")
    );
    seed(&env);
    env
}

#[test]
fn ql2_last_objective_commits_readiness_not_history_before_log_callback() {
    let env = WowLuaEnv::new().unwrap();
    seed(&env);
    env.exec("QL2.objectives(false); QL2.observe(); QL2.objectives(true)")
        .unwrap();
    let ready: bool = env
        .eval("return C_QuestLog.ReadyForTurnIn(990002)")
        .unwrap();
    assert!(
        ready,
        "QL-2 behavioral RED: finishing final objective must publish readiness"
    );
    env.exec("QL2.assertReady(); assert(QL2.committed == true); assert(QL2.autoCount == 0)")
        .unwrap();
    let other = WowLuaEnv::new().unwrap();
    seed(&other);
    other
        .exec("assert(not C_QuestLog.ReadyForTurnIn(990002))")
        .unwrap();
    env.exec("QL2.objectives(false); assert(not C_QuestLog.ReadyForTurnIn(990002))")
        .unwrap();
}

#[test]
fn ql2_explicit_readiness_rejects_unknown_failed_and_keeps_history_independent() {
    let env = WowLuaEnv::new().unwrap();
    seed(&env);
    env.exec("QL2.configure(true, false)").unwrap();
    let ready: bool = env
        .eval("return C_QuestLog.ReadyForTurnIn(990002)")
        .unwrap();
    assert!(ready, "explicit non-objective readiness must be supplied");
    env.exec("QL2.assertReady(); assert(not C_QuestLog.ReadyForTurnIn(123456))")
        .unwrap();
    {
        let mut sim = env.state().borrow_mut();
        let entry = sim
            .quest_log_entries
            .entries
            .iter_mut()
            .find(|e| e.quest_id == 990002)
            .unwrap();
        entry.is_failed = true;
    }
    env.exec("assert(not C_QuestLog.ReadyForTurnIn(990002)); QL2.configure(true, false); assert(not C_QuestLog.ReadyForTurnIn(990002))")
        .unwrap();
    // Historical flag alone must not make an active incomplete quest ready/complete.
    {
        let mut sim = env.state().borrow_mut();
        let entry = sim
            .quest_log_entries
            .entries
            .iter_mut()
            .find(|e| e.quest_id == 990002)
            .unwrap();
        entry.is_failed = false;
        entry.is_complete = false;
        entry.is_flagged_completed = true;
        sim.quest_log_entries.completed_quest_ids.insert(990002);
    }
    env.exec("QL2.configure(false, false); assert(not C_QuestLog.ReadyForTurnIn(990002)); assert(not IsQuestComplete(990002)); assert(C_QuestLog.IsQuestFlaggedCompleted(990002))")
        .unwrap();
}

#[test]
fn ql2_original_gossip_refresh_and_click_use_live_readiness() {
    let env = consumer_env();
    let errors_before = env.state().borrow().lua_error_counts.clone();
    env.exec("QL2.objectives(false); GossipFrame.gossipOptions = {}; GossipFrame:Update(); GossipFrame:RegisterEvent('QUEST_LOG_UPDATE'); QL2.objectives(true)")
        .unwrap();
    let row_ready: bool = env.eval("return QL2.gossipReady()").unwrap();
    assert!(
        row_ready,
        "original GossipFrame data provider must refresh active row readiness"
    );
    env.exec("QL2.clickGossip(); assert(QuestFrameRewardPanel:IsShown()); QL2.assertReady()")
        .unwrap();
    env.exec("QL2.objectives(false); QL2.clickGossip(); assert(QuestFrameProgressPanel:IsShown()); assert(not QuestFrameCompleteButton:IsEnabled())")
        .unwrap();
    // Both public selectors must resolve current rich state, not cached row flags.
    env.exec("QL2.objectives(true); SelectActiveQuest(1); assert(QuestFrameRewardPanel:IsShown()); QL2.assertReady()")
        .unwrap();
    assert_eq!(
        env.state().borrow().lua_error_counts,
        errors_before,
        "no swallowed consumer errors"
    );
}

#[test]
fn ql2_original_progress_button_advances_once_without_grant_or_turn_in() {
    let env = consumer_env();
    let errors_before = env.state().borrow().lua_error_counts.clone();
    env.exec("QL2.objectives(false); QL2.clickGossip(); QL2.observe(); if type(CompleteQuest) == 'function' then QuestProgressCompleteButton_OnClick() end")
        .unwrap();
    env.exec("assert(QL2.completeCount == 0); assert(QuestFrameProgressPanel:IsShown()); QL2.objectives(true)")
        .unwrap();
    let ready: bool = env
        .eval("return type(IsQuestCompletable) == 'function' and IsQuestCompletable()")
        .unwrap();
    assert!(
        ready,
        "selected live progress interaction must become completable"
    );
    let before = {
        let sim = env.state().borrow();
        (
            sim.quest_log.clone(),
            sim.quest_log_entries.completed_quest_ids.clone(),
            sim.player.money,
            sim.player.xp,
        )
    };
    env.exec("assert(QuestFrameCompleteButton:IsEnabled()); QuestProgressCompleteButton_OnClick(); QuestProgressCompleteButton_OnClick(); assert(QL2.completeCount == 1); assert(QuestFrameRewardPanel:IsShown()); QL2.assertReady(); assert(QL2.turnInCount == 0)")
        .unwrap();
    let sim = env.state().borrow();
    assert_eq!(
        (
            sim.quest_log.clone(),
            sim.quest_log_entries.completed_quest_ids.clone(),
            sim.player.money,
            sim.player.xp
        ),
        before
    );
    drop(sim);
    env.exec("A_Admin.CloseQuestNpc(); QuestProgressCompleteButton_OnClick(); assert(QL2.completeCount == 1)")
        .unwrap();
    assert_eq!(
        env.state().borrow().lua_error_counts,
        errors_before,
        "no swallowed consumer errors"
    );
}

#[test]
fn ql2_opt_in_autocomplete_reaches_original_tracker_popup_once_not_turn_in() {
    let env = consumer_env();
    let errors_before = env.state().borrow().lua_error_counts.clone();
    env.exec("QuestObjectiveTracker:RegisterEvent('QUEST_AUTOCOMPLETE'); QL2.configure(false, true); QL2.objectives(false); QL2.observe(); QL2.objectives(true); QL2.objectives(true)")
        .unwrap();
    let count: i32 = env.eval("return QL2.autoCount").unwrap();
    assert_eq!(
        count, 1,
        "opt-in final progress publishes one QUEST_AUTOCOMPLETE"
    );
    env.exec("assert(QL2.autoPayload == '1:990002'); QL2.assertReady(); assert(QL2.turnInCount == 0); QL2.assertPopup(); QL2.objectives(false); QL2.objectives(true); assert(QL2.autoCount == 1)")
        .unwrap();
    assert_eq!(
        env.state().borrow().lua_error_counts,
        errors_before,
        "no swallowed consumer errors"
    );
}
