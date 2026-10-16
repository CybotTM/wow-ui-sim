//! Authored simulator turn-in model, not native quest/reward parity.
use wow_ui_sim::lua_api::WowLuaEnv;

// Missing publication leaves state untouched so baseline reaches a real Rust
// membership assertion, not a compilation error or missing-function Lua error.
fn attempt_turn_in(env: &WowLuaEnv) -> Option<bool> {
    env.eval(
        "local turnIn = rawget(A_Admin, 'TurnInSelectedQuest'); \
         if type(turnIn) == 'function' then return turnIn() end",
    )
    .expect("attempt simulator-only selected quest turn-in")
}

fn supply_completion(env: &WowLuaEnv, quest_id: i32) {
    let mut state = env.state().borrow_mut();
    let entry = state
        .quest_log_entries
        .entries
        .iter_mut()
        .find(|entry| entry.quest_id == quest_id)
        .expect("fixture requires an already represented rich quest");
    entry.is_complete = true;
}

fn assert_not_active(env: &WowLuaEnv, quest_id: i32) {
    let state = env.state().borrow();
    assert!(
        !state
            .quest_log_entries
            .entries
            .iter()
            .any(|entry| entry.quest_id == quest_id),
        "turned-in target must leave rich active log"
    );
    assert!(!state.quest_log.contains(&(quest_id as u32)));
    drop(state);
    let projection: (bool, Option<i32>, bool, bool) = env
        .eval(&format!(
            "return C_QuestLog.IsOnQuest({quest_id}), \
             C_QuestLog.GetLogIndexForQuestID({quest_id}), \
             C_QuestLog.IsComplete({quest_id}), \
             C_QuestLog.IsQuestFlaggedCompleted({quest_id})"
        ))
        .expect("read active/completion projections");
    assert_eq!(projection, (false, None, false, true));
}

#[cfg(feature = "client-wowforever")]
#[test]
fn selected_completed_turn_in_commits_before_original_listener_and_cannot_repeat() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r##"
        A_Admin.OpenMultiQuestNpc()
        SelectAvailableQuest(1)
        ConfirmAcceptQuest()
        C_QuestLog.AddQuestWatch(80002)
        A_Admin.OpenQuestNpc(80000, "Unrelated incomplete control")
        SelectAvailableQuest(1)
        ConfirmAcceptQuest()
        C_QuestLog.AddQuestWatch(80000)
        QuestMapLogTitleButton_OnClick(80002)
        turnInCount = 0
        turnInObserver = CreateFrame("Frame")
        turnInObserver:RegisterEvent("QUEST_TURNED_IN")
        turnInObserver:SetScript("OnEvent", function(_, event, ...)
            turnInCount = turnInCount + 1
            local questID, xp, money = ...
            turnInPayload = event .. ":" .. select("#", ...) .. ":" .. questID .. ":" .. xp .. ":" .. money
            local completed = false
            for _, id in ipairs(C_QuestLog.GetAllCompletedQuestIDs()) do
                if id == questID then completed = true end
            end
            turnInCommittedAtEvent = not C_QuestLog.IsOnQuest(questID)
                and C_QuestLog.GetLogIndexForQuestID(questID) == nil
                and C_QuestLog.GetQuestWatchType(questID) == nil
                and C_QuestLog.IsQuestFlaggedCompleted(questID)
                and completed and GetQuestID() == 0
            -- Reentry at the original callback boundary sees committed state.
            turnInReentry = A_Admin.TurnInSelectedQuest()
        end)
        "##,
    )
    .unwrap();
    supply_completion(&env, 80002);
    let (history_before, light_before, watch_before, money_before, xp_before) = {
        let state = env.state().borrow();
        assert!(state.quest_log.contains(&80002));
        assert!(state.quest_watch_types.contains_key(&80002));
        assert!(!state.quest_log_entries.completed_quest_ids.contains(&80002));
        (
            state.quest_log_entries.completed_quest_ids.clone(),
            state.quest_log.clone(),
            state.quest_watch_types.clone(),
            state.player.money,
            state.player.xp,
        )
    };
    let outcome = attempt_turn_in(&env);
    assert_not_active(&env, 80002);
    assert_eq!(outcome, Some(true));
    let observed: (i32, String, bool, bool) = env
        .eval("return turnInCount, turnInPayload, turnInCommittedAtEvent, turnInReentry")
        .unwrap();
    assert_eq!(
        observed,
        (1, "QUEST_TURNED_IN:3:80002:0:0".into(), true, false)
    );
    let mut expected_history = history_before;
    expected_history.insert(80002);
    let state = env.state().borrow();
    assert_eq!(
        state.quest_log_entries.completed_quest_ids,
        expected_history
    );
    let mut expected_light = light_before;
    expected_light.retain(|id| *id != 80002);
    assert_eq!(state.quest_log, expected_light);
    let mut expected_watch = watch_before;
    expected_watch.remove(&80002);
    assert_eq!(state.quest_watch_types, expected_watch);
    assert!(
        state
            .quest_log_entries
            .entries
            .iter()
            .any(|entry| { entry.quest_id == 80000 && !entry.is_complete })
    );
    assert_eq!(
        (state.player.money, state.player.xp),
        (money_before, xp_before)
    );
    drop(state);
    env.exec("QuestMapLogTitleButton_OnClick(80002)").unwrap();
    assert_eq!(
        attempt_turn_in(&env),
        Some(false),
        "removed quest cannot repeat turn-in"
    );
    let count: i32 = env.eval("return turnInCount").unwrap();
    assert_eq!(count, 1);
    assert_eq!(
        env.state().borrow().quest_log_entries.completed_quest_ids,
        expected_history
    );
}

#[test]
fn selected_world_turn_in_removes_task_map_time_and_world_log_projections() {
    let env = WowLuaEnv::new().unwrap();
    supply_completion(&env, 90001);
    env.exec("QuestMapLogTitleButton_OnClick(90001)").unwrap();
    let before: (bool, bool, bool) = env
        .eval("return C_TaskQuest.IsActive(90001), C_QuestLog.GetWorldQuestInfo(90001) ~= nil, C_TaskQuest.IsActive(90002)")
        .unwrap();
    assert_eq!(before, (true, true, true));
    let outcome = attempt_turn_in(&env);
    assert_not_active(&env, 90001);
    assert_eq!(
        outcome,
        Some(true),
        "rich-only seed needs no lightweight ID"
    );
    let after: (bool, bool, bool, bool) = env
        .eval(
            r#"
            local targetOnTaskMap, unrelatedOnTaskMap = false, false
            for _, quest in ipairs(C_TaskQuest.GetQuestsOnMap(2025)) do
                if quest.questID == 90001 then targetOnTaskMap = true end
                if quest.questID == 90002 then unrelatedOnTaskMap = true end
            end
            local targetOnLogMap = false
            for _, quest in ipairs(C_QuestLog.GetQuestsOnMap(2025)) do
                if quest.questID == 90001 then targetOnLogMap = true end
            end
            return C_TaskQuest.IsActive(90001), targetOnTaskMap, targetOnLogMap, unrelatedOnTaskMap
            "#,
        )
        .unwrap();
    assert_eq!(after, (false, false, false, true));
    let timed: (bool, bool, bool, bool) = env
        .eval(
            "return C_TaskQuest.GetQuestTimeLeftMinutes(90001) == nil, \
             C_TaskQuest.GetQuestTimeLeftSeconds(90001) == nil, \
             C_QuestLog.GetWorldQuestInfo(90001) == nil, \
             C_TaskQuest.DoesMapShowTaskQuestObjectives(2025)",
        )
        .unwrap();
    assert_eq!(timed, (true, true, true, true));
    env.exec("QuestMapLogTitleButton_OnClick(90001)").unwrap();
    let legacy_time_gone: bool = env.eval("return GetQuestLogTimeLeft() == nil").unwrap();
    assert!(
        legacy_time_gone,
        "reselection cannot restore a consumed quest timer"
    );
    let catalog_title: String = env
        .eval("return C_TaskQuest.GetQuestInfoByQuestID(90001)")
        .unwrap();
    assert_eq!(
        catalog_title, "Glittering Geodes",
        "catalog metadata is not active membership"
    );
    // Supply the existing rich model for the remaining static task catalog row.
    {
        let mut state = env.state().borrow_mut();
        let mut second = state
            .quest_log_entries
            .entries
            .iter()
            .find(|entry| entry.quest_id == 90101)
            .unwrap()
            .clone();
        second.quest_id = 90002;
        second.map_id = Some(2025);
        second.is_complete = true;
        state.quest_log_entries.entries.push(second);
    }
    env.exec("QuestMapLogTitleButton_OnClick(90002)").unwrap();
    assert_eq!(attempt_turn_in(&env), Some(true));
    let map_after: (i32, bool) = env
        .eval("return #C_TaskQuest.GetQuestsOnMap(2025), C_TaskQuest.DoesMapShowTaskQuestObjectives(2025)")
        .unwrap();
    assert_eq!(map_after, (0, false));
    #[cfg(not(feature = "retail-12-0-0"))]
    {
        let alias_count: i32 = env
            .eval("return #C_TaskQuest.GetQuestsForPlayerByMapID(2025)")
            .unwrap();
        assert_eq!(alias_count, 0);
    }
    let independent = WowLuaEnv::new().unwrap();
    let available: bool = independent
        .eval("return C_TaskQuest.IsActive(90001)")
        .unwrap();
    assert!(available, "completion history remains environment-local");
}

#[test]
fn turn_in_invalid_incomplete_and_failed_selection_is_inert() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        A_Admin.OpenMultiQuestNpc()
        SelectAvailableQuest(1)
        ConfirmAcceptQuest()
        A_Admin.OpenQuestNpc(81000, "Pending control")
        SelectAvailableQuest(1)
        turnInCount = 0
        turnInObserver = CreateFrame("Frame")
        turnInObserver:RegisterEvent("QUEST_TURNED_IN")
        turnInObserver:SetScript("OnEvent", function() turnInCount = turnInCount + 1 end)
        "#,
    )
    .unwrap();
    for selected in [None, Some(0), Some(u32::MAX), Some(123456), Some(80002)] {
        env.state().borrow_mut().selected_quest_log_id = selected;
        let (light, rich_ids, history, pending) = {
            let state = env.state().borrow();
            (
                state.quest_log.clone(),
                state
                    .quest_log_entries
                    .entries
                    .iter()
                    .map(|entry| entry.quest_id)
                    .collect::<Vec<_>>(),
                state.quest_log_entries.completed_quest_ids.clone(),
                state.pending_quest_offer.as_ref().unwrap().quest_id,
            )
        };
        assert_eq!(attempt_turn_in(&env), Some(false), "selection {selected:?}");
        let state = env.state().borrow();
        assert_eq!(state.quest_log, light);
        assert_eq!(
            state
                .quest_log_entries
                .entries
                .iter()
                .map(|entry| entry.quest_id)
                .collect::<Vec<_>>(),
            rich_ids
        );
        assert_eq!(state.quest_log_entries.completed_quest_ids, history);
        assert_eq!(
            state.pending_quest_offer.as_ref().unwrap().quest_id,
            pending
        );
        assert_eq!(state.selected_quest_log_id, selected);
    }
    supply_completion(&env, 80002);
    env.state()
        .borrow_mut()
        .quest_log_entries
        .entries
        .iter_mut()
        .find(|entry| entry.quest_id == 80002)
        .unwrap()
        .is_failed = true;
    assert_eq!(
        attempt_turn_in(&env),
        Some(false),
        "failed row is ineligible"
    );
    let state = env.state().borrow();
    assert!(state.quest_log.contains(&80002));
    assert!(!state.quest_log_entries.completed_quest_ids.contains(&80002));
    drop(state);
    let count: i32 = env.eval("return turnInCount").unwrap();
    assert_eq!(count, 0);
}

#[test]
fn turn_in_clears_matching_pending_and_gossip_rows_preserving_unrelated_offer() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.OpenMultiQuestNpc(); SelectActiveQuest(1)")
        .unwrap();
    let selected: i32 = env.eval("return GetQuestID()").unwrap();
    assert_eq!(selected, 80001);
    let history_before = env
        .state()
        .borrow()
        .quest_log_entries
        .completed_quest_ids
        .clone();
    // Already-completed history must not prevent removing a represented row.
    assert!(history_before.contains(&80001));
    env.exec("A_Admin.OpenQuestNpc(80001, 'Matching pending'); SelectAvailableQuest(1); A_Admin.OpenMultiQuestNpc(); SelectActiveQuest(1)").unwrap();
    let outcome = attempt_turn_in(&env);
    assert_not_active(&env, 80001);
    assert_eq!(outcome, Some(true));
    let state = env.state().borrow();
    assert!(state.pending_quest_offer.is_none());
    assert_eq!(state.quest_log_entries.completed_quest_ids, history_before);
    assert!(
        !state
            .gossip
            .active_quests
            .iter()
            .any(|quest| quest.quest_id == 80001)
    );
    assert_eq!(
        state.gossip.num_active_quests as usize,
        state.gossip.active_quests.len()
    );
    assert!(
        state
            .gossip
            .available_quests
            .iter()
            .any(|quest| quest.quest_id == 80002)
    );
    drop(state);
    env.exec("A_Admin.OpenQuestNpc(81000, 'Unrelated pending'); SelectAvailableQuest(1); QuestMapLogTitleButton_OnClick(90101)").unwrap();
    supply_completion(&env, 90101);
    assert_eq!(attempt_turn_in(&env), Some(true));
    assert_eq!(
        env.state()
            .borrow()
            .pending_quest_offer
            .as_ref()
            .unwrap()
            .quest_id,
        81000
    );
}
