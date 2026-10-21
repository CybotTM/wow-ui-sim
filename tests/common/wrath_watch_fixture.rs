use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::QuestLogEntry;

/// Supply the same three authored quests to the legacy selected-index reader.
pub fn seed_watch_quests(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    let template = state
        .quest_log_entries
        .entries
        .iter()
        .find(|entry| entry.quest_id == 80000)
        .expect("Lost Expedition seed")
        .clone();
    let supply_run = QuestLogEntry {
        quest_id: 80002,
        title: "Supply Run".into(),
        ..template
    };
    let existing = state
        .quest_log_entries
        .entries
        .iter()
        .filter(|entry| matches!(entry.quest_id, 80000 | 80001))
        .cloned();
    state.quest_log_entries.entries = existing.chain([supply_run]).collect();
    assert_eq!(state.quest_log_entries.entries.len(), 3);
    state.quest_log = state
        .quest_log_entries
        .entries
        .iter()
        .map(|entry| entry.quest_id as u32)
        .collect();
}
