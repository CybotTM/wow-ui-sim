//! Source-derived 12340 manual watch contracts; no automatic/timed watches.
#![cfg(feature = "client-wrath")]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::GossipQuestRow;

fn supplied(count: u32) -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    let mut sim = env.state().borrow_mut();
    sim.quest_log_entries.entries = (1..=count)
        .map(|row| {
            sim.quest_log_entries
                .snapshot_gossip_offer(&GossipQuestRow {
                    quest_id: 992000 + row,
                    title: format!("N3 contract {row}"),
                    quest_level: 23,
                    ..Default::default()
                })
                .unwrap()
        })
        .collect();
    sim.quest_log = (1..=count).map(|row| 992000 + row).collect();
    sim.selected_quest_log_id = None;
    sim.abandon_quest_id = None;
    sim.events.drain();
    drop(sim);
    env
}

#[test]
fn n3_native_empty_cap_duplicates_order_and_zero_mutator_returns() {
    let env = supplied(26);
    env.exec(
        r#"
        assert(select('#', GetNumQuestWatches()) == 1)
        assert(GetNumQuestWatches() == 0, 'no synthetic seeded watches')
        assert(select('#', AddQuestWatch(3)) == 0)
        AddQuestWatch(1); AddQuestWatch(3)
        assert(GetNumQuestWatches() == 2, 'duplicate add is inert')
        assert(GetQuestIndexForWatch(1) == 3 and GetQuestIndexForWatch(2) == 1)
        assert(select('#', RemoveQuestWatch(2)) == 0)
        assert(GetNumQuestWatches() == 2, 'absent removal is inert')
        RemoveQuestWatch(3)
        assert(GetQuestIndexForWatch(1) == 1, 'survivors compact in order')
        for row=2,26 do AddQuestWatch(row) end
        assert(GetNumQuestWatches() == 25, '26th add silently ignored')
        assert(IsQuestWatched(26) == nil)
        for watch=1,25 do assert(GetQuestIndexForWatch(watch) == watch) end
        assert(GetQuestIndexForWatch(26) == nil)
        "#,
    )
    .unwrap();
    assert!(env.state().borrow().events.pending().is_empty());
}

#[test]
fn n3_native_selected_fallback_numeric_strings_query_usage_and_arities() {
    let env = supplied(4);
    env.exec(
        r#"
        SelectQuestLogEntry(2)
        for _, invalid in ipairs({false, {}, 'not numeric'}) do
            assert(select('#', AddQuestWatch(invalid)) == 0)
            assert(GetNumQuestWatches() == 1 and IsQuestWatched(2) == 1)
            assert(select('#', RemoveQuestWatch(invalid)) == 0)
            assert(GetNumQuestWatches() == 0)
        end
        AddQuestWatch(); assert(IsQuestWatched('2') == 1)
        RemoveQuestWatch(); assert(IsQuestWatched(2) == nil)
        AddQuestWatch('3')
        for _, row in ipairs({0, -1, 5}) do AddQuestWatch(row); RemoveQuestWatch(row) end
        assert(IsQuestWatched(2) == nil, 'explicit invalid never uses selection')
        assert(select('#', IsQuestWatched(2)) == 1)
        assert(select('#', IsQuestWatched(3)) == 1 and IsQuestWatched(3) == 1)
        assert(select('#', GetQuestIndexForWatch(0)) == 1 and GetQuestIndexForWatch(0) == nil)
        assert(GetQuestIndexForWatch('1') == 3)
        for name, usage in pairs({IsQuestWatched='Usage: IsQuestWatched(index)',
                                  GetQuestIndexForWatch='Usage: GetQuestIndexForWatch(index)'}) do
            local ok, err = pcall(_G[name])
            assert(not ok and string.find(err, usage, 1, true), tostring(err))
            ok, err = pcall(_G[name], {})
            assert(not ok and string.find(err, usage, 1, true), tostring(err))
        end
        "#,
    )
    .unwrap();
    assert!(env.state().borrow().events.pending().is_empty());
}

#[test]
fn n3_native_retained_identity_and_append_order_across_log_reorder() {
    let env = supplied(4);
    env.exec("AddQuestWatch(4); AddQuestWatch(2); AddQuestWatch(1)")
        .unwrap();
    {
        let mut sim = env.state().borrow_mut();
        sim.quest_log_entries.entries.swap(0, 3);
        sim.quest_log.swap(0, 3);
    }
    env.exec(
        r#"
        assert(GetQuestIndexForWatch(1) == 1, 'same D ID now at row 1')
        assert(GetQuestIndexForWatch(2) == 2)
        assert(GetQuestIndexForWatch(3) == 4, 'same A ID now at row 4')
        assert(IsQuestWatched(1) == 1 and IsQuestWatched(4) == 1)
        RemoveQuestWatch(2); AddQuestWatch(3)
        assert(GetQuestIndexForWatch(1) == 1 and GetQuestIndexForWatch(2) == 4)
        assert(GetQuestIndexForWatch(3) == 3, 'new watch appends without sorting')
        "#,
    )
    .unwrap();
}

#[test]
fn n3_native_manual_mutators_emit_nothing_and_abandon_prunes_on_deferred_refresh() {
    let env = supplied(4);
    env.exec(
        r#"
        N3Events = {}
        local observer = CreateFrame('Frame')
        for _, name in ipairs({'QUEST_LOG_UPDATE','UNIT_QUEST_LOG_CHANGED','QUEST_WATCH_UPDATE','QUEST_WATCH_LIST_CHANGED'}) do
            observer:RegisterEvent(name)
        end
        observer:SetScript('OnEvent', function(_, name, ...)
            N3Events[#N3Events+1] = {name, select('#', ...), ...}
            if name == 'QUEST_LOG_UPDATE' then
                N3Refresh = {GetNumQuestWatches(), GetQuestIndexForWatch(1), GetQuestIndexForWatch(2)}
            end
        end)
        AddQuestWatch(4); AddQuestWatch(2); AddQuestWatch(1)
        RemoveQuestWatch(3); AddQuestWatch(2)
        assert(#N3Events == 0, 'manual mutations have no synchronous events')
        SelectQuestLogEntry(2); SetAbandonQuest(); AbandonQuest()
        assert(GetNumQuestWatches() == 3, 'no immediate abandon prune')
        assert(GetQuestIndexForWatch(2) == nil, 'removed ID has no current row')
        assert(#N3Events == 1 and N3Events[1][1] == 'UNIT_QUEST_LOG_CHANGED')
        assert(N3Events[1][2] == 1 and N3Events[1][3] == 'player')
        AbandonQuest()
        "#,
    )
    .unwrap();
    let queued = env.state().borrow_mut().events.drain();
    assert_eq!(queued.len(), 1, "coalesced deferred refresh only");
    assert_eq!(queued[0].name, "QUEST_LOG_UPDATE");
    assert!(queued[0].args.is_empty());
    env.fire_event(&queued[0].name).unwrap();
    env.exec(
        r#"
        assert(#N3Events == 2 and N3Events[2][1] == 'QUEST_LOG_UPDATE' and N3Events[2][2] == 0)
        assert(N3Refresh[1] == 2 and N3Refresh[2] == 3 and N3Refresh[3] == 1, 'pruned before listeners, survivors retain order')
        "#,
    )
    .unwrap();
    assert!(env.state().borrow().events.pending().is_empty());
}
