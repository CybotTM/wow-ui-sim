//! Simulator abandonment preserves unrelated active quests and completion history.
#![cfg(feature = "client-wowforever")]

use std::collections::HashSet;
use wow_ui_sim::c_api::QuestWatchType;
use wow_ui_sim::lua_api::WowLuaEnv;

const SUPPLY_RUN: u32 = 80002;
const UNRELATED_QUEST: u32 = 80000;
const HISTORICAL_QUEST_IDS: [i32; 2] = [79999, 80001];

fn read_public_watch_ids(env: &WowLuaEnv) -> Vec<i32> {
    let count: i32 = env
        .eval("return C_QuestLog.GetNumQuestWatches()")
        .expect("read public ordinary watch count");
    (1..=count)
        .map(|index| {
            env.eval(&format!(
                "return C_QuestLog.GetQuestIDForQuestWatchIndex({index})"
            ))
            .expect("read public watched quest ID")
        })
        .collect()
}

fn read_completion_history(env: &WowLuaEnv) -> (HashSet<i32>, String) {
    let completed = env
        .state()
        .borrow()
        .quest_log_entries
        .completed_quest_ids
        .clone();
    let sorted_ids = env
        .eval(
            "local sortedIDs = C_QuestLog.GetAllCompletedQuestIDs(); \
             table.sort(sortedIDs); return table.concat(sortedIDs, ',')",
        )
        .expect("read sorted public completed quest IDs");
    (completed, sorted_ids)
}

fn assert_active_and_watched(env: &WowLuaEnv, quest_id: u32) {
    {
        let state = env.state().borrow();
        let lightweight_count = state.quest_log.iter().filter(|id| **id == quest_id).count();
        let rich_count = state
            .quest_log_entries
            .entries
            .iter()
            .filter(|entry| entry.quest_id == quest_id as i32)
            .count();
        assert_eq!(
            lightweight_count, 1,
            "quest {quest_id}: lightweight membership"
        );
        assert_eq!(rich_count, 1, "quest {quest_id}: rich membership");
        assert_eq!(
            state.quest_watch_types.get(&(quest_id as i32)),
            Some(&QuestWatchType::Manual),
            "quest {quest_id}: explicit watch metadata"
        );
    }
    let (on_quest, log_index, watch_type): (bool, Option<i32>, Option<i32>) = env
        .eval(&format!(
            "return C_QuestLog.IsOnQuest({quest_id}), \
             C_QuestLog.GetLogIndexForQuestID({quest_id}), \
             C_QuestLog.GetQuestWatchType({quest_id})"
        ))
        .expect("read public active membership and watch type");
    assert!(on_quest, "quest {quest_id}: public active membership");
    assert!(log_index.is_some_and(|index| index > 0));
    assert_eq!(watch_type, Some(QuestWatchType::Manual as i32));
    let watch_count = read_public_watch_ids(env)
        .iter()
        .filter(|id| **id == quest_id as i32)
        .count();
    assert_eq!(watch_count, 1, "quest {quest_id}: public watch membership");
}

#[test]
fn abandon_removes_active_quest_and_watch_preserving_unrelated_membership_and_history() {
    let env = WowLuaEnv::new().expect("create isolated quest environment");
    {
        let state = env.state().borrow();
        assert!(!state.quest_log.contains(&SUPPLY_RUN));
        assert!(!state.quest_log.contains(&UNRELATED_QUEST));
        assert!(
            !state
                .quest_log_entries
                .entries
                .iter()
                .any(|entry| entry.quest_id == SUPPLY_RUN as i32)
        );
        assert!(!state.quest_watch_types.contains_key(&(SUPPLY_RUN as i32)));
    }
    assert!(!read_public_watch_ids(&env).contains(&(SUPPLY_RUN as i32)));

    let (offered, selected, watched): (i32, i32, bool) = env
        .eval(
            r#"
            A_Admin.OpenMultiQuestNpc()
            local offered = C_GossipInfo.GetAvailableQuests()[1].questID
            SelectAvailableQuest(1)
            local selected = GetQuestID()
            ConfirmAcceptQuest()
            return offered, selected, C_QuestLog.AddQuestWatch(80002)
            "#,
        )
        .expect("publicly offer, select, accept and explicitly watch supply run");
    assert_eq!(offered, SUPPLY_RUN as i32);
    assert_eq!(selected, SUPPLY_RUN as i32);
    assert!(
        watched,
        "supply run must newly acquire explicit watch membership"
    );

    env.exec(
        r#"
        A_Admin.OpenQuestNpc(80000, "The Lost Expedition")
        SelectAvailableQuest(1)
        ConfirmAcceptQuest()
        C_QuestLog.AddQuestWatch(80000)
        "#,
    )
    .expect("publicly accept and explicitly watch unrelated control quest");
    assert_active_and_watched(&env, SUPPLY_RUN);
    assert_active_and_watched(&env, UNRELATED_QUEST);

    let history_before = read_completion_history(&env);
    for historical_id in HISTORICAL_QUEST_IDS {
        assert!(history_before.0.contains(&historical_id));
    }
    assert!(!history_before.0.contains(&(SUPPLY_RUN as i32)));
    let mut completed_ids: Vec<i32> = history_before.0.iter().copied().collect();
    completed_ids.sort_unstable();
    let expected_public_ids = completed_ids
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join(",");
    assert_eq!(history_before.1, expected_public_ids);

    env.exec("QuestMapLogTitleButton_OnClick(80002); SetAbandonQuest()")
        .expect("publicly select and abandon target quest");

    {
        let state = env.state().borrow();
        assert!(
            !state.quest_log.contains(&SUPPLY_RUN),
            "target must leave lightweight log"
        );
        assert!(
            !state
                .quest_log_entries
                .entries
                .iter()
                .any(|entry| entry.quest_id == SUPPLY_RUN as i32),
            "target must leave rich log"
        );
        assert!(
            !state.quest_watch_types.contains_key(&(SUPPLY_RUN as i32)),
            "target watch metadata must be removed"
        );
    }
    let target_membership: (bool, Option<i32>, Option<i32>) = env
        .eval(
            "return C_QuestLog.IsOnQuest(80002), \
             C_QuestLog.GetLogIndexForQuestID(80002), \
             C_QuestLog.GetQuestWatchType(80002)",
        )
        .expect("read public target membership after abandonment");
    assert_eq!(target_membership, (false, None, None));
    assert!(!read_public_watch_ids(&env).contains(&(SUPPLY_RUN as i32)));
    assert_active_and_watched(&env, UNRELATED_QUEST);
    assert_eq!(
        read_completion_history(&env),
        history_before,
        "completion set and sorted public IDs must remain exactly unchanged"
    );
}
