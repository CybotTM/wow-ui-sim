//! Supplied quest-dialog reputation; no inferred major-faction classification.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever"
))]

use wow_ui_sim::c_api::quest_readiness::QuestDialogPhase;
use wow_ui_sim::lua_api::WowLuaEnv;

fn dialog_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    {
        let mut sim = env.state().borrow_mut();
        let id = sim.quest_log_entries.entries[0].quest_id;
        sim.gossip.active = true;
        let mut row = sim
            .gossip
            .active_quests
            .first()
            .cloned()
            .unwrap_or_default();
        row.quest_id = id as u32;
        sim.gossip.active_quests = vec![row];
        sim.selected_quest_log_id = Some(id as u32);
        // PTR has no QL-2 producer: supply the existing context, not a new gate.
        sim.quest_log_entries.readiness.dialog = Some((id, QuestDialogPhase::Rewards));
    }
    env
}

fn selected_id(env: &WowLuaEnv) -> i32 {
    env.state().borrow().selected_quest_log_id.unwrap() as i32
}

fn assert_no_rewards(env: &WowLuaEnv) {
    let count: i32 = env
        .eval("return select('#', C_QuestOffer.GetQuestOfferMajorFactionReputationRewards())")
        .unwrap();
    assert_eq!(count, 0, "absent dialog/rewards produce no Lua values");
}

#[test]
fn absent_supplied_rewards_do_not_reinterpret_legacy_factions() {
    let env = dialog_env();
    env.state().borrow_mut().quest_log_entries.entries[0].reward_factions = vec![(2507, 300)];
    env.exec("assert(type(rawget(C_QuestOffer, 'GetQuestOfferMajorFactionReputationRewards')) == 'function')")
        .unwrap();
    assert_no_rewards(&env);
}

#[test]
fn supplied_rewards_preserve_order_values_and_live_updates() {
    let env = dialog_env();
    let id = selected_id(&env);
    env.state()
        .borrow_mut()
        .quest_log_entries
        .offer
        .major_faction_rewards
        .insert(id, vec![(2507, 300), (2510, 125)]);
    env.exec(
        r#"
        local rows = C_QuestOffer.GetQuestOfferMajorFactionReputationRewards()
        assert(select('#', C_QuestOffer.GetQuestOfferMajorFactionReputationRewards()) == 1)
        assert(#rows == 2)
        assert(rows[1].factionID == 2507 and rows[1].rewardAmount == 300)
        assert(rows[2].factionID == 2510 and rows[2].rewardAmount == 125)
        rows[1].rewardAmount = 999
        assert(C_QuestOffer.GetQuestOfferMajorFactionReputationRewards()[1].rewardAmount == 300)
        "#,
    )
    .unwrap();
    env.state()
        .borrow_mut()
        .quest_log_entries
        .offer
        .major_faction_rewards
        .insert(id, vec![(2510, 75)]);
    env.exec("local rows = C_QuestOffer.GetQuestOfferMajorFactionReputationRewards(); assert(#rows == 1 and rows[1].factionID == 2510 and rows[1].rewardAmount == 75)").unwrap();
    env.state()
        .borrow_mut()
        .quest_log_entries
        .offer
        .major_faction_rewards
        .insert(id, vec![]);
    env.exec("local rows = C_QuestOffer.GetQuestOfferMajorFactionReputationRewards(); assert(type(rows) == 'table' and #rows == 0)").unwrap();
}

#[test]
fn rewards_require_matching_dialog_not_log_selection_or_other_quest() {
    let env = dialog_env();
    let id = selected_id(&env);
    let other_id = env.state().borrow().quest_log_entries.entries[1].quest_id;
    env.state()
        .borrow_mut()
        .quest_log_entries
        .offer
        .major_faction_rewards
        .insert(other_id, vec![(2510, 125)]);
    assert_no_rewards(&env);
    {
        let mut sim = env.state().borrow_mut();
        sim.quest_log_entries
            .offer
            .major_faction_rewards
            .insert(id, vec![(2507, 300)]);
        sim.selected_quest_log_id = Some(other_id as u32);
    }
    assert_no_rewards(&env);
    {
        let mut sim = env.state().borrow_mut();
        sim.gossip.active_quests[0].quest_id = other_id as u32;
        sim.quest_log_entries.readiness.dialog = Some((other_id, QuestDialogPhase::Progress));
    }
    env.exec("local rows = C_QuestOffer.GetQuestOfferMajorFactionReputationRewards(); assert(#rows == 1 and rows[1].factionID == 2510 and rows[1].rewardAmount == 125)").unwrap();
    env.state().borrow_mut().quest_log_entries.readiness.dialog = None;
    assert_no_rewards(&env);
}

#[test]
fn close_invalidates_rewards_even_while_log_selection_remains() {
    let env = dialog_env();
    let id = selected_id(&env);
    env.state()
        .borrow_mut()
        .quest_log_entries
        .offer
        .major_faction_rewards
        .insert(id, vec![(2507, 300)]);
    env.exec("assert(#C_QuestOffer.GetQuestOfferMajorFactionReputationRewards() == 1); A_Admin.CloseQuestNpc()")
        .unwrap();
    assert_eq!(selected_id(&env), id);
    assert_no_rewards(&env);
}

#[test]
fn supplied_rewards_are_environment_local() {
    let env = dialog_env();
    let other = dialog_env();
    let id = selected_id(&env);
    env.state()
        .borrow_mut()
        .quest_log_entries
        .offer
        .major_faction_rewards
        .insert(id, vec![(2507, 300)]);
    assert_no_rewards(&other);
    env.exec(
        "assert(C_QuestOffer.GetQuestOfferMajorFactionReputationRewards()[1].rewardAmount == 300)",
    )
    .unwrap();
}

#[test]
fn removing_quest_clears_supplied_rewards_for_its_next_lifetime() {
    for turn_in in [false, true] {
        let env = dialog_env();
        let id = selected_id(&env);
        let saved_entry = {
            let mut sim = env.state().borrow_mut();
            sim.quest_log_entries.entries[0].is_complete = true;
            sim.quest_log_entries
                .offer
                .major_faction_rewards
                .insert(id, vec![(2507, 300)]);
            sim.quest_log_entries.entries[0].clone()
        };
        env.exec("assert(C_QuestOffer.GetQuestOfferMajorFactionReputationRewards()[1].rewardAmount == 300)")
            .unwrap();
        {
            let mut sim = env.state().borrow_mut();
            let removed = if turn_in {
                sim.quest_log_entries.turn_in(id)
            } else {
                sim.quest_log_entries.abandon(id)
            };
            assert!(removed);
            sim.quest_log_entries.entries.push(saved_entry);
            sim.quest_log_entries.readiness.dialog = Some((id, QuestDialogPhase::Rewards));
        }
        assert_no_rewards(&env);
    }
}
