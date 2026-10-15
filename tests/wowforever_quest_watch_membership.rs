//! Ordinary-watch membership regressions; simulator policies, not native parity.
#![cfg(feature = "client-wowforever")]

use wow_ui_sim::lua_api::WowLuaEnv;

fn create_membership_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("create separate watch environment");
    env.exec(include_str!("wowforever_quest_watch_membership.lua"))
        .expect("define membership cases without replacing public APIs");
    env
}

#[test]
fn forever_watch_membership_seeds_manual_and_preserves_rich_projection() {
    let env = create_membership_env();
    env.exec("WatchMembershipTests.defaults()")
        .expect("seeded watch metadata and rich queries agree");
}

#[test]
fn forever_watch_membership_transitions_publish_post_state_synchronously() {
    let env = create_membership_env();
    env.exec(include_str!("fixtures/quest_watch_membership_original.lua"))
        .expect("unchanged original Remove/Add rich-row regression");
    env.exec("WatchMembershipTests.transitions()")
        .expect("real Add/Remove transitions, exact payloads and idempotence");
}

#[test]
fn forever_watch_membership_rejects_invalid_ids_and_excludes_world_rows() {
    let env = create_membership_env();
    env.exec("WatchMembershipTests.invalidAndWorld()")
        .expect("strict ID validation and ordinary/world separation");
}

#[test]
fn forever_watch_membership_duplicate_add_preserves_supplied_automatic_type() {
    let env = create_membership_env();
    env.state()
        .borrow_mut()
        .quest_watch_types
        .insert(80000, wow_ui_sim::c_api::QuestWatchType::Automatic);
    env.exec("WatchMembershipTests.automaticDuplicate()")
        .expect("duplicate leaves Automatic intact; removal and re-add use Manual");
}

fn register_watch_counter(env: &WowLuaEnv) {
    env.exec(
        r#"
        WatchMembershipEventCount = 0
        WatchMembershipListener = CreateFrame('Frame')
        WatchMembershipListener:RegisterEvent('QUEST_WATCH_LIST_CHANGED')
        WatchMembershipListener:SetScript('OnEvent', function()
            WatchMembershipEventCount = WatchMembershipEventCount + 1
        end)
        "#,
    )
    .expect("observe real mutations independently in each environment");
}

#[test]
fn forever_watch_membership_live_rows_and_events_are_environment_local() {
    let mutated = create_membership_env();
    let fresh = create_membership_env();
    register_watch_counter(&mutated);
    register_watch_counter(&fresh);
    {
        let mut state = mutated.state().borrow_mut();
        let mut only = state.quest_log_entries.entries[0].clone();
        only.quest_id = 456791;
        only.title = "Supplied unwatched ordinary quest".into();
        only.level = 19;
        state.quest_log_entries.entries = vec![only];
    }
    mutated
        .exec(
            r#"
            WatchMembershipTests.assertProjection({})
            WatchMembershipTests.assertUnwatched(456791)
            WatchMembershipTests.assertUnwatched(80000)
            WatchMembershipTests.assertUnwatched(80001)
            local index = C_QuestLog.GetLogIndexForQuestID(456791)
            local info = C_QuestLog.GetInfo(index)
            assert(info.title == 'Supplied unwatched ordinary quest')
            assert(info.level == 19 and info.difficultyLevel == 19)
            assert(C_QuestLog.GetQuestDifficultyLevel(456791) == 19)
            assert(C_QuestLog.AddQuestWatch(456791) == true)
            assert(WatchMembershipEventCount == 1)
            WatchMembershipTests.assertProjection({456791})
            -- Removal tests actual metadata, even when its rich row disappeared.
            assert(C_QuestLog.RemoveQuestWatch(80000) == true)
            assert(WatchMembershipEventCount == 2)
            assert(C_QuestLog.RemoveQuestWatch(80000) == false)
            assert(WatchMembershipEventCount == 2)
            WatchMembershipTests.assertProjection({456791})
            "#,
        )
        .expect("host ID replacement does not silently transfer watch membership");
    fresh
        .exec(
            r#"
            WatchMembershipTests.defaults()
            assert(WatchMembershipEventCount == 0, 'other environment leaked an event')
            assert(C_QuestLog.GetLogIndexForQuestID(456791) == nil)
            assert(C_QuestLog.RemoveQuestWatch(80001) == true)
            assert(WatchMembershipEventCount == 1)
            WatchMembershipTests.assertProjection({80000})
            "#,
        )
        .expect("fresh metadata and event listeners are separate");
    mutated
        .exec(
            r#"
            assert(WatchMembershipEventCount == 2, 'fresh Remove leaked an event')
            WatchMembershipTests.assertProjection({456791})
            "#,
        )
        .expect("fresh mutation cannot alter supplied-row membership");
    mutated.state().borrow_mut().quest_log_entries.entries[0].is_world_quest = true;
    mutated
        .exec(
            r#"
            WatchMembershipTests.assertProjection({})
            WatchMembershipTests.assertUnwatched(456791)
            assert(C_QuestLog.AddQuestWatch(456791) == false, 'world row cannot be added')
            assert(WatchMembershipEventCount == 2)
            assert(C_QuestLog.RemoveQuestWatch(456791) == true, 'remove actual metadata')
            assert(WatchMembershipEventCount == 3)
            "#,
        )
        .expect("rich world classification filters retained ordinary-watch metadata");
    fresh
        .exec(
            r#"
            assert(WatchMembershipEventCount == 1)
            WatchMembershipTests.assertProjection({80000})
            "#,
        )
        .expect("host row classification also remains environment-local");
}

#[test]
fn forever_watch_membership_legacy_indices_follow_current_membership() {
    let env = create_membership_env();
    env.exec(
        r##"
        assert(GetQuestIndexForWatch(1) == 2 and GetQuestIndexForWatch(2) == 3)
        assert(GetQuestWatchIndex(2) == 1 and GetQuestSortIndex(3) == 2)
        assert(C_QuestLog.RemoveQuestWatch(80000) == true)
        assert(GetNumQuestWatches() == 1)
        assert(IsQuestWatched(2) == false and IsQuestWatched(3) == true)
        assert(GetQuestIndexForWatch(1) == 3 and GetQuestIndexForWatch(2) == 0)
        assert(GetQuestWatchIndex(2) == 0 and GetQuestWatchIndex(3) == 1)
        assert(GetQuestSortIndex(2) == 0 and GetQuestSortIndex(3) == 1)
        assert(GetQuestIndexForWatch(1.9) == 3)
        assert(GetQuestWatchIndex(3.9) == 1 and IsQuestWatched(3.9) == true)
        for _, get in ipairs({GetQuestIndexForWatch, GetQuestWatchIndex, GetQuestSortIndex}) do
            assert(select('#', get()) == 1 and get() == 0)
            assert(get(nil) == 0 and get(0) == 0 and get(-1) == 0)
            assert(get(math.huge) == 0 and get(0/0) == 0)
            assert(not pcall(get, '3'))
        end
        assert(IsQuestWatched() == false and IsQuestWatched(1) == false)
        assert(IsQuestWatched(4) == false, 'rich world row is not an ordinary watch')
        assert(not pcall(IsQuestWatched, '3'))
        assert(C_QuestLog.AddQuestWatch(80000) == true)
        assert(GetQuestIndexForWatch(1) == 2 and GetQuestIndexForWatch(2) == 3)
        assert(GetQuestWatchIndex(2) == 1 and GetQuestSortIndex(3) == 2)
        "##,
    )
    .expect("legacy watch indexes use current rich rows and explicit membership");
}

#[test]
fn forever_watch_membership_survives_post_cleanup_restoration() {
    let env = create_membership_env();
    register_watch_counter(&env);
    env.exec(
        r#"
        assert(C_QuestLog.RemoveQuestWatch(80000) == true)
        assert(WatchMembershipEventCount == 1)
        WatchMembershipTests.assertProjection({80001})
        WatchMembershipTests.assertUnwatched(80000)
        "#,
    )
    .expect("remove seeded membership before real cleanup restoration");

    env.restore_post_cleanup_globals();

    env.exec(
        r#"
        for _, name in ipairs({'AddQuestWatch', 'RemoveQuestWatch', 'GetQuestWatchType',
            'GetNumQuestWatches', 'GetQuestIDForQuestWatchIndex'}) do
            assert(type(rawget(C_QuestLog, name)) == 'function', name .. ' after restore')
        end
        for _, name in ipairs({'GetNumQuestWatches', 'IsQuestWatched',
            'GetQuestIndexForWatch', 'GetQuestWatchIndex', 'GetQuestSortIndex'}) do
            assert(type(rawget(_G, name)) == 'function', name .. ' after restore')
        end
        assert(WatchMembershipEventCount == 1, 'restoration must not emit watch events')
        WatchMembershipTests.assertProjection({80001})
        WatchMembershipTests.assertUnwatched(80000)
        assert(C_QuestLog.GetQuestWatchType(80001) == Enum.QuestWatchType.Manual)
        assert(GetQuestIndexForWatch(1) == 3 and GetQuestIndexForWatch(2) == 0)
        assert(GetQuestWatchIndex(2) == 0 and GetQuestWatchIndex(3) == 1)
        assert(GetQuestSortIndex(2) == 0 and GetQuestSortIndex(3) == 1)
        assert(C_QuestLog.RemoveQuestWatch(80000) == false, 'restore must not reseed')
        assert(WatchMembershipEventCount == 1)
        assert(C_QuestLog.AddQuestWatch(80000) == true)
        assert(WatchMembershipEventCount == 2, 'restored mutator and listener remain live')
        WatchMembershipTests.assertProjection({80000, 80001})
        assert(C_QuestLog.GetQuestWatchType(80000) == Enum.QuestWatchType.Manual)
        assert(GetQuestIndexForWatch(1) == 2 and GetQuestIndexForWatch(2) == 3)
        assert(GetQuestWatchIndex(2) == 1 and GetQuestSortIndex(3) == 2)
        assert(C_QuestLog.AddQuestWatch(80000) == false)
        assert(WatchMembershipEventCount == 2)
        "#,
    )
    .expect("restoration preserves mutated membership without reseeding or notifications");
}

#[test]
fn forever_watch_membership_event_can_reenter_mutator_synchronously() {
    let env = create_membership_env();
    env.exec(
        r#"
        local records, outcomes = {}, {}
        local reentered = false
        local nestedResult
        local listener = CreateFrame('Frame')
        listener:RegisterEvent('QUEST_WATCH_LIST_CHANGED')
        local function record(phase, self, event, ...)
            local ids = {}
            for index = 1, C_QuestLog.GetNumQuestWatches() do
                ids[index] = C_QuestLog.GetQuestIDForQuestWatchIndex(index)
            end
            records[#records + 1] = {
                phase = phase, sameFrame = self == listener, event = event,
                payload = {n = select('#', ...), ...}, ids = ids,
                count = C_QuestLog.GetNumQuestWatches(), globalCount = GetNumQuestWatches(),
                watchType = C_QuestLog.GetQuestWatchType(80000),
                watched = IsQuestWatched(C_QuestLog.GetLogIndexForQuestID(80000)),
            }
        end
        listener:SetScript('OnEvent', function(self, event, ...)
            local payload = {n = select('#', ...), ...}
            -- Save callback errors; every assertion runs after protected dispatch returns.
            local ok, message = pcall(function()
                record('enter', self, event, unpack(payload, 1, payload.n))
                if not reentered then
                    reentered = true
                    nestedResult = C_QuestLog.AddQuestWatch(80000)
                end
                record('exit', self, event, unpack(payload, 1, payload.n))
            end)
            outcomes[#outcomes + 1] = {ok = ok, message = message}
        end)

        assert(C_QuestLog.RemoveQuestWatch(80000) == true)
        assert(reentered and nestedResult == true, 'nested Add must finish before Remove returns')
        assert(#outcomes == 2, 'both real event callbacks must finish')
        for _, outcome in ipairs(outcomes) do
            assert(outcome.ok, 'callback failed: ' .. tostring(outcome.message))
        end
        assert(#records == 4, 'exactly two nested event deliveries, each with enter/exit')
        local expected = {
            {phase = 'enter', added = false, ids = {80001}, watched = false},
            {phase = 'enter', added = true, ids = {80000, 80001}, watched = true},
            {phase = 'exit', added = true, ids = {80000, 80001}, watched = true},
            {phase = 'exit', added = false, ids = {80000, 80001}, watched = true},
        }
        for index, want in ipairs(expected) do
            local got = records[index]
            assert(got.phase == want.phase, 'nested event order at ' .. index)
            assert(got.sameFrame and got.event == 'QUEST_WATCH_LIST_CHANGED')
            assert(got.payload.n == 2)
            assert(got.payload[1] == 80000 and got.payload[2] == want.added)
            assert(got.count == #want.ids and got.globalCount == #want.ids)
            assert(#got.ids == #want.ids)
            for watchIndex, questID in ipairs(want.ids) do
                assert(got.ids[watchIndex] == questID, 'nested snapshot at ' .. index)
            end
            assert(got.watched == want.watched)
            local wantType
            if want.watched then wantType = Enum.QuestWatchType.Manual end
            assert(got.watchType == wantType, 'nested watch type at ' .. index)
        end
        WatchMembershipTests.assertProjection({80000, 80001})
        assert(C_QuestLog.GetQuestWatchType(80000) == Enum.QuestWatchType.Manual)
        assert(C_QuestLog.AddQuestWatch(80000) == false)
        assert(#records == 4 and #outcomes == 2, 'duplicate after nested Add emits nothing')
        listener:UnregisterEvent('QUEST_WATCH_LIST_CHANGED')
        "#,
    )
    .expect("real OnEvent reenters Add without a borrow failure and preserves nested ordering");
}
