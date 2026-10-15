//! Existing same-quest data is not replaced by ordinary-offer defaults.
#![cfg(feature = "client-wowforever")]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn accepting_represented_quest_preserves_existing_metadata_and_rewards() {
    let env = WowLuaEnv::new().expect("create isolated existing-quest environment");
    env.exec("A_Admin.OpenMultiQuestNpc()")
        .expect("seed the existing completed-quest reward");
    let (original, rich_count) = {
        let state = env.state().borrow();
        let original = state
            .quest_log_entries
            .entries
            .iter()
            .find(|entry| entry.quest_id == 80001)
            .expect("PRECONDITION: existing rich completed quest")
            .clone();
        (original, state.quest_log_entries.entries.len())
    };
    assert_eq!(
        original.reward_items.len(),
        1,
        "PRECONDITION: supplied reward"
    );
    let watch_count: i32 = env.eval("return C_QuestLog.GetNumQuestWatches()").unwrap();
    env.exec(
        "A_Admin.OpenQuestNpc(80001, 'Offer label must not replace existing log data'); \
         C_GossipInfo.SelectAvailableQuest(80001); C_GossipInfo.CloseGossip(); ConfirmAcceptQuest()",
    )
    .expect("accept the selected snapshot without duplicating an existing rich row");
    {
        let state = env.state().borrow();
        let rows: Vec<_> = state
            .quest_log_entries
            .entries
            .iter()
            .filter(|entry| entry.quest_id == original.quest_id)
            .collect();
        assert_eq!(rows.len(), 1);
        assert_eq!(state.quest_log_entries.entries.len(), rich_count);
        let row = rows[0];
        assert_eq!(row.title, original.title);
        assert_eq!(row.level, original.level);
        assert_eq!(row.is_complete, original.is_complete);
        assert_eq!(row.map_id, original.map_id);
        assert_eq!(row.tag_id, original.tag_id);
        assert_eq!(row.waypoint, original.waypoint);
        assert_eq!(row.reward_items.len(), original.reward_items.len());
        assert_eq!(row.reward_items[0].name, original.reward_items[0].name);
        assert_eq!(row.reward_items[0].count, original.reward_items[0].count);
        assert_eq!(
            row.reward_items[0].quality,
            original.reward_items[0].quality
        );
        assert_eq!(state.quest_log.iter().filter(|id| **id == 80001).count(), 1);
        assert!(state.pending_quest_offer.is_none());
    }
    let (title, level, reward_name, after_watches): (String, i32, String, i32) = env
        .eval(
            "local index = C_QuestLog.GetLogIndexForQuestID(80001); \
             local info = C_QuestLog.GetInfo(index); \
             local name = GetQuestLogRewardInfo(1, 80001); \
             return info.title, info.level, name, C_QuestLog.GetNumQuestWatches()",
        )
        .expect("project retained log metadata and the existing quest reward");
    assert_eq!(title, original.title);
    assert_eq!(level, original.level);
    assert_eq!(reward_name, original.reward_items[0].name);
    assert_eq!(after_watches, watch_count);
}
