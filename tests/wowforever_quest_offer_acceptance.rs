//! Public seeded offer acceptance must agree with rich-log and watch queries.
#![cfg(feature = "client-wowforever")]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn seeded_public_offer_acceptance_preserves_rich_and_watch_coherence() {
    let env = WowLuaEnv::new().expect("fresh isolated quest environment");
    {
        let state = env.state().borrow();
        assert!(
            !state.quest_log.contains(&80002),
            "BLOCKER: offered quest is already in the lightweight log"
        );
        assert!(state.pending_quest_offer.is_none(), "BLOCKER: preexisting offer");
    }
    let (quest_id, coherent, observations): (i32, bool, String) = env
        .eval(include_str!("wowforever_quest_offer_acceptance.lua"))
        .expect("public seeded offer path must be reachable; no injected preconditions");
    let quest_id = u32::try_from(quest_id).expect("public offered ID must be nonnegative");
    let state = env.state().borrow();
    let accepted_occurrences = state.quest_log.iter().filter(|id| **id == quest_id).count();
    let pending_offer = state.pending_quest_offer;
    let selected_id = state.selected_quest_log_id;
    let queued_accepted = state
        .events
        .pending()
        .iter()
        .filter(|event| event.name == "QUEST_ACCEPTED")
        .count();
    eprintln!(
        "{observations}\nlightweightOccurrences={accepted_occurrences} pendingOffer={pending_offer:?} selectedID={selected_id:?} queuedAccepted={queued_accepted}"
    );
    // Final assertion is outside Lua/event handlers; no callback assertion can be swallowed.
    assert!(
        accepted_occurrences == 1 && pending_offer.is_none() && coherent,
        "public acceptance must publish matching rich metadata and allow explicit ordinary watch; {observations}; lightweightOccurrences={accepted_occurrences}; pendingOffer={pending_offer:?}"
    );
}
