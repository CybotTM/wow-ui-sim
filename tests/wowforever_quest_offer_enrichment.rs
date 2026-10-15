//! Authored simulator offer enrichment, not native quest-data parity.
//! Acceptance assertions run in Rust, outside Lua/event callbacks.
#![cfg(feature = "client-wowforever")]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::sim_substates::{GossipQuestRow, QuestLogEntry};

const SUPPLY_RUN_ID: u32 = 80002;
const CUSTOM_OFFER_ID: u32 = 456793;
const CUSTOM_TITLE: &str = "Authored custom supply offer";
const CUSTOM_LEVEL: i32 = 27;

fn create_offer_env() -> WowLuaEnv {
    WowLuaEnv::new().expect("PRECONDITION: create isolated Forever environment")
}

fn accepted_event_count(env: &WowLuaEnv) -> usize {
    env.state()
        .borrow()
        .events
        .pending()
        .iter()
        .filter(|event| event.name == "QUEST_ACCEPTED")
        .count()
}

fn assert_offer_absent(env: &WowLuaEnv, quest_id: u32) {
    let state = env.state().borrow();
    assert!(
        !state.quest_log.contains(&quest_id),
        "unexpected lightweight member"
    );
    let rich_present = state
        .quest_log_entries
        .entries
        .iter()
        .any(|row| row.quest_id == quest_id as i32);
    assert!(!rich_present, "unexpected rich member");
    drop(state);
    let absent: bool = env
        .eval(&format!(
            "return not C_QuestLog.IsOnQuest({quest_id}) \
             and C_QuestLog.GetLogIndexForQuestID({quest_id}) == nil \
             and C_QuestLog.GetQuestWatchType({quest_id}) == nil"
        ))
        .expect("query absent offer membership");
    assert!(absent, "public rich/watch membership must be absent");
}

fn read_public_offer(env: &WowLuaEnv) -> GossipQuestRow {
    let state = env.state().borrow();
    assert_eq!(
        state.gossip.available_quests.len(),
        1,
        "PRECONDITION: one offer"
    );
    let offer = state.gossip.available_quests[0].clone();
    drop(state);
    let public: (i32, String, i32) = env
        .eval(
            "local offers = C_GossipInfo.GetAvailableQuests(); \
             assert(#offers == 1, 'PRECONDITION: one public offer'); \
             local row = offers[1]; return row.questID, row.title, row.questLevel",
        )
        .expect("PRECONDITION: public offer metadata is reachable");
    let public_id = i32::try_from(offer.quest_id).expect("fixture ID fits public quest range");
    assert_eq!(public, (public_id, offer.title.clone(), offer.quest_level));
    offer
}

fn read_selected_quest_id(env: &WowLuaEnv) -> u32 {
    let id: i32 = env
        .eval("return GetQuestID()")
        .expect("read selected offer ID");
    u32::try_from(id).expect("selected offer ID is nonnegative")
}

fn assert_modeled_offer_policy(row: &QuestLogEntry) {
    assert!(
        !row.is_world_quest,
        "authored ordinary offer, not world quest"
    );
    assert!(!row.is_failed);
    assert!(!row.is_replayable);
    assert!(!row.is_flagged_completed, "not previously completed");
    assert_eq!(
        row.map_id, None,
        "unknown map must not borrow seeded defaults"
    );
    assert_eq!(row.waypoint, None);
    assert_eq!(
        row.tag_id, None,
        "unknown tag must not borrow seeded defaults"
    );
    assert_eq!(row.details_theme, None);
    assert_eq!(row.criteria_spell_id, None);
    assert_eq!(row.criteria_spell_name, None);
    assert_eq!(row.criteria_spell_texture, None);
    assert!(!row.criteria_spell_finished);
    assert!(
        row.reward_items.is_empty(),
        "authored scenario has no item rewards"
    );
    assert!(
        row.currency_rewards.is_empty(),
        "authored scenario has no currency rewards"
    );
}

fn assert_rich_offer(env: &WowLuaEnv, offer: &GossipQuestRow) {
    let state = env.state().borrow();
    let rows: Vec<_> = state
        .quest_log_entries
        .entries
        .iter()
        .filter(|row| row.quest_id == offer.quest_id as i32)
        .collect();
    assert_eq!(
        rows.len(),
        1,
        "MISSING RICH ROW: acceptance must enrich the selected offer"
    );
    let row = rows[0];
    assert_eq!(row.quest_id, offer.quest_id as i32);
    assert_eq!(row.title, offer.title);
    assert_eq!(row.level, offer.quest_level);
    assert_eq!(row.is_meta, offer.is_meta);
    assert_eq!(row.is_complete, offer.is_complete.unwrap_or(false));
    assert_modeled_offer_policy(row);
    drop(state);
    assert_accepted_membership(env, offer.quest_id);
    assert_public_rich_offer(env, offer);
}

fn assert_accepted_membership(env: &WowLuaEnv, quest_id: u32) {
    let state = env.state().borrow();
    assert!(
        !state
            .quest_log_entries
            .completed_quest_ids
            .contains(&(quest_id as i32))
    );
    let occurrences = state.quest_log.iter().filter(|id| **id == quest_id).count();
    assert_eq!(occurrences, 1, "exactly one lightweight membership");
    assert!(state.pending_quest_offer.is_none());
}

fn assert_public_rich_offer(env: &WowLuaEnv, offer: &GossipQuestRow) {
    let id = offer.quest_id;
    let public: (Option<i32>, Option<String>, Option<i32>, Option<i32>) = env
        .eval(&format!(
            "local index = C_QuestLog.GetLogIndexForQuestID({id}); \
             local info = index and C_QuestLog.GetInfo(index); \
             return info and info.questID, info and info.title, \
             info and info.level, info and info.difficultyLevel"
        ))
        .expect("read accepted public rich row");
    let expected = (
        Some(id as i32),
        Some(offer.title.clone()),
        Some(offer.quest_level),
        Some(offer.quest_level),
    );
    assert_eq!(public, expected);
    let difficulty: i32 = env
        .eval(&format!("return C_QuestLog.GetQuestDifficultyLevel({id})"))
        .expect("read accepted Forever difficulty endpoint");
    assert_eq!(difficulty, offer.quest_level);
    assert_public_offer_flags(env, offer);
}

fn assert_public_offer_flags(env: &WowLuaEnv, offer: &GossipQuestRow) {
    let id = offer.quest_id;
    let flags: (bool, bool) = env
        .eval(&format!(
            "return C_QuestLog.IsMetaQuest({id}), C_QuestLog.IsComplete({id})"
        ))
        .expect("read supplied offer flags through public APIs");
    assert_eq!(flags, (offer.is_meta, offer.is_complete.unwrap_or(false)));
    let ordinary: bool = env
        .eval(&format!(
            "return C_QuestLog.IsOnQuest({id}) and not C_QuestLog.IsWorldQuest({id}) \
             and not C_QuestLog.IsFailed({id}) and not C_QuestLog.IsQuestReplayable({id}) \
             and not C_QuestLog.IsQuestFlaggedCompleted({id})"
        ))
        .expect("read accepted public ordinary-offer policy");
    assert!(
        ordinary,
        "public flags must agree with the authored scenario"
    );
}

fn assert_explicit_watch_coherent(env: &WowLuaEnv, quest_id: u32) {
    let before: (Option<i32>, i32) = env
        .eval(&format!(
            "return C_QuestLog.GetQuestWatchType({quest_id}), C_QuestLog.GetNumQuestWatches()"
        ))
        .expect("read watch state before explicit AddQuestWatch");
    let needs_watch = before.0.is_none();
    let added: bool = env
        .eval(&format!("return C_QuestLog.AddQuestWatch({quest_id})"))
        .expect("explicitly watch the accepted offer");
    assert_eq!(added, needs_watch, "add only when membership is absent");
    let expected_count = before.1 + i32::from(needs_watch);
    let expected_type = before.0.unwrap_or(1); // Explicit Add creates Manual membership.
    assert_watch_projection(env, quest_id, expected_count, expected_type);
}

fn assert_watch_projection(
    env: &WowLuaEnv,
    quest_id: u32,
    expected_count: i32,
    expected_type: i32,
) {
    let observed: (i32, i32, bool) = env
        .eval(&format!(
            "local occurrences, matching = 0, false; \
             local index = C_QuestLog.GetLogIndexForQuestID({quest_id}); \
             for i = 1, C_QuestLog.GetNumQuestWatches() do \
                 if C_QuestLog.GetQuestIDForQuestWatchIndex(i) == {quest_id} then \
                     occurrences = occurrences + 1; matching = GetQuestIndexForWatch(i) == index; \
                 end; \
             end; \
             return C_QuestLog.GetNumQuestWatches(), occurrences, \
                 matching and C_QuestLog.GetQuestWatchType({quest_id}) == {expected_type}"
        ))
        .expect("read coherent watch type/ID/index projection");
    assert_eq!(observed, (expected_count, 1, true));
}

#[test]
fn forever_legacy_selected_offer_survives_close_gossip_with_rich_metadata() {
    let env = create_offer_env();
    assert_offer_absent(&env, SUPPLY_RUN_ID);
    let accepted_before = accepted_event_count(&env);
    env.exec("A_Admin.OpenMultiQuestNpc()")
        .expect("PRECONDITION: public multi-quest NPC opens");
    let offer = read_public_offer(&env);
    assert_eq!(
        offer.quest_id, SUPPLY_RUN_ID,
        "PRECONDITION: authored Supply Run offer"
    );
    env.exec("SelectAvailableQuest(1)")
        .expect("PRECONDITION: legacy selector accepts offer index 1");
    let selected = read_selected_quest_id(&env);
    assert_eq!(
        selected, offer.quest_id,
        "PRECONDITION: legacy selection reached offer"
    );
    env.exec("C_GossipInfo.CloseGossip(); ConfirmAcceptQuest()")
        .expect("close public gossip then confirm the retained selection");
    assert_rich_offer(&env, &offer);
    assert_eq!(accepted_event_count(&env), accepted_before + 1);
    assert_explicit_watch_coherent(&env, offer.quest_id);
    assert_rich_offer(&env, &offer);
}

#[test]
fn forever_id_selected_custom_offer_preserves_snapshot_and_confirm_is_idempotent() {
    let env = create_offer_env();
    assert_offer_absent(&env, CUSTOM_OFFER_ID);
    env.exec(&format!(
        "A_Admin.OpenQuestNpc({CUSTOM_OFFER_ID}, '{CUSTOM_TITLE}')"
    ))
    .expect("PRECONDITION: existing public custom-ID/title NPC opens");
    // Authored host fixture input, not claimed native metadata or a new API.
    {
        let mut state = env.state().borrow_mut();
        assert_eq!(
            state.gossip.available_quests.len(),
            1,
            "PRECONDITION: custom row"
        );
        let row = &mut state.gossip.available_quests[0];
        row.quest_level = CUSTOM_LEVEL;
        row.is_meta = true;
        row.is_complete = Some(true);
    }
    let offer = read_public_offer(&env);
    assert_eq!(
        offer.title, CUSTOM_TITLE,
        "PRECONDITION: supplied custom title"
    );
    let accepted_before = accepted_event_count(&env);
    env.exec(&format!(
        "C_GossipInfo.SelectAvailableQuest({CUSTOM_OFFER_ID})"
    ))
    .expect("PRECONDITION: public namespace selector takes quest ID");
    let selected = read_selected_quest_id(&env);
    assert_eq!(
        selected, CUSTOM_OFFER_ID,
        "PRECONDITION: namespace selection reached offer"
    );
    // Replace gossip for the SAME ID after selection, then clear it entirely.
    env.exec(&format!(
        "A_Admin.OpenQuestNpc({CUSTOM_OFFER_ID}, 'Later gossip, not selected metadata'); \
         C_GossipInfo.CloseGossip(); ConfirmAcceptQuest()"
    ))
    .expect("accept snapshot, not later gossip data or other-quest defaults");
    assert_rich_offer(&env, &offer);
    assert_eq!(accepted_event_count(&env), accepted_before + 1);
    env.exec("ConfirmAcceptQuest()")
        .expect("second confirmation without an offer is a no-op");
    assert_rich_offer(&env, &offer);
    assert_eq!(
        accepted_event_count(&env),
        accepted_before + 1,
        "no second acceptance event"
    );
    assert_explicit_watch_coherent(&env, CUSTOM_OFFER_ID);
}

#[test]
fn forever_fresh_environment_and_cancelled_offer_do_not_gain_rich_membership() {
    let accepted = create_offer_env();
    accepted
        .exec("A_Admin.OpenMultiQuestNpc(); SelectAvailableQuest(1); ConfirmAcceptQuest()")
        .expect("PRECONDITION: accept an offer in a separate environment");
    assert!(accepted.state().borrow().quest_log.contains(&SUPPLY_RUN_ID));
    let accepted_events = accepted_event_count(&accepted);
    let fresh = create_offer_env();
    assert_offer_absent(&fresh, SUPPLY_RUN_ID);
    let fresh_events = accepted_event_count(&fresh);
    fresh
        .exec("A_Admin.OpenMultiQuestNpc(); SelectAvailableQuest(1)")
        .expect("PRECONDITION: open and select cancellable offer");
    let selected = read_selected_quest_id(&fresh);
    assert_eq!(
        selected, SUPPLY_RUN_ID,
        "PRECONDITION: cancellable offer selected"
    );
    fresh
        .exec("CloseQuestFrame(); ConfirmAcceptQuest()")
        .expect("cancel quest frame before confirmation");
    assert_offer_absent(&fresh, SUPPLY_RUN_ID);
    assert!(fresh.state().borrow().pending_quest_offer.is_none());
    assert_eq!(
        accepted_event_count(&fresh),
        fresh_events,
        "cancelled offer emits no acceptance"
    );
    assert_eq!(
        accepted_event_count(&accepted),
        accepted_events,
        "no cross-environment event leak"
    );
    assert!(accepted.state().borrow().quest_log.contains(&SUPPLY_RUN_ID));
}
