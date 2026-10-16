//! Unapproved profiles retain their pre-QL2 surface and cached gossip policy.
#![cfg(not(any(feature = "client-retail", feature = "client-wowforever")))]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn quest_readiness_is_not_published_outside_retail_and_forever() {
    let env = WowLuaEnv::new().unwrap();
    let id = {
        let mut sim = env.state().borrow_mut();
        let entry = &mut sim.quest_log_entries.entries[0];
        entry.is_complete = true;
        entry.is_failed = false;
        let id = entry.quest_id;
        sim.gossip.active = true;
        let mut row = sim
            .gossip
            .active_quests
            .first()
            .cloned()
            .unwrap_or_default();
        row.quest_id = id as u32;
        row.is_complete = Some(false);
        sim.gossip.active_quests = vec![row];
        id
    };
    env.exec(&format!(
        r#"
        assert(rawget(A_Admin, 'SetQuestObjectives') == nil)
        assert(rawget(A_Admin, 'SetQuestReadiness') == nil)
        assert(C_QuestLog.ReadyForTurnIn({id}) == false, 'retain existing unmodeled readiness')
        assert(C_GossipInfo.GetActiveQuests()[1].isComplete == false, 'retain cached row flag')
        local _, complete = GetActiveTitle(1)
        assert(complete == false)
    "#
    ))
    .unwrap();
}
