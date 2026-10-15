//! Ordinary-watch projection regressions.
//! Simulator policy: ordinary metadata filters rich non-world rows, in row order.
//! This is not a claim about the native client's watch subset.
#![cfg(feature = "client-wowforever")]

use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_watch_projection(env: &WowLuaEnv, expected: &[i32]) {
    let ids = expected
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join(",");
    let source = format!("local expected = {{{ids}}}\n")
        + r##"
        local count = C_QuestLog.GetNumQuestWatches()
        assert(count == #expected, "ordinary watch count")
        assert(GetNumQuestWatches() == count, "global count alias")
        for index, expectedID in ipairs(expected) do
            local questID = C_QuestLog.GetQuestIDForQuestWatchIndex(index)
            assert(questID == expectedID, "ordered watched identity at " .. index)
            assert(questID ~= 80002, "no static-only quest in rich projection")
            local logIndex = C_QuestLog.GetLogIndexForQuestID(questID)
            assert(type(logIndex) == "number" and logIndex > 1, "rich log lookup")
            local info = C_QuestLog.GetInfo(logIndex)
            assert(type(info) == "table" and not info.isHeader, "rich quest row")
            assert(info.questID == questID and info.questLogIndex == logIndex)
            assert(not info.isWorldQuest, "world rows excluded")
            assert(C_QuestLog.IsOnQuest(questID), "watch resolves to current rich state")
            assert(C_QuestLog.GetQuestDifficultyLevel(questID) == info.level)
            assert(info.difficultyLevel == info.level)
        end
        assert(select('#', C_QuestLog.GetQuestIDForQuestWatchIndex(count + 1)) == 0)
        assert(not C_QuestLog.IsOnQuest(80002), "do not fabricate rich quest80002")
        assert(not pcall(C_QuestLog.GetQuestDifficultyLevel, 80002))
        "##;
    env.exec(&source)
        .expect("public watch projection and rich metadata agree");
}

#[test]
fn forever_quest_watch_defaults_exclude_static_only_80002() {
    let env = WowLuaEnv::new().unwrap();
    assert_watch_projection(&env, &[80000, 80001]);
    env.exec(
        r##"
        assert(C_QuestLog.GetLogIndexForQuestID(80002) == nil)
        assert(C_QuestLog.GetQuestDifficultyLevel(80000) == 80)
        assert(C_QuestLog.GetQuestDifficultyLevel(80001) == 80)
        assert(C_QuestLog.IsOnQuest(90001) and C_QuestLog.IsOnQuest(90101))
        "##,
    )
    .unwrap();
}

#[test]
fn forever_quest_watch_raw_callability_arity_and_index_conversion_preserved() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r##"
        local count = rawget(C_QuestLog, "GetNumQuestWatches")
        local get = rawget(C_QuestLog, "GetQuestIDForQuestWatchIndex")
        local globalCount = rawget(_G, "GetNumQuestWatches")
        assert(type(count) == "function")
        assert(type(get) == "function")
        assert(type(globalCount) == "function")
        assert(select('#', count()) == 1 and count() == 2)
        assert(select('#', globalCount()) == 1 and globalCount() == count())
        assert(count(false, {}, "ignored") == 2)
        assert(globalCount(false, {}, "ignored") == 2)
        assert(select('#', get(1)) == 1 and get(1) == 80000)
        assert(select('#', get(2)) == 1 and get(2) == 80001)
        assert(get(1, false, {}) == 80000, "extra arguments ignored")
        assert(select('#', get()) == 0, "absent defaults to index zero")
        assert(select('#', get(nil)) == 0, "nil defaults to index zero")
        -- Existing Option<f64> followed by `as i32`: truncate/saturate, not validate.
        assert(get(1.9) == 80000 and get(2.9) == 80001)
        for _, index in ipairs({-1, -1.9, 0, 0.9, 3, 3.9, 2147483648,
                                -2147483649, 0/0, math.huge, -math.huge}) do
            assert(select('#', get(index)) == 0, "out-of-range has zero results")
        end
        for _, value in ipairs({"1", false, {}, function() end}) do
            local ok, err = pcall(get, value)
            assert(not ok and type(err) == "string", "nonnumbers remain errors")
        end
        "##,
    )
    .unwrap();
}

#[test]
fn forever_quest_watch_observes_live_rows_order_removal_and_world_classification() {
    let env = WowLuaEnv::new().unwrap();
    assert_watch_projection(&env, &[80000, 80001]);
    {
        let mut state = env.state().borrow_mut();
        let entry = &mut state.quest_log_entries.entries[0];
        entry.quest_id = 456789;
        entry.title = "Live ordinary quest".into();
        entry.level = 23;
    }
    assert_watch_projection(&env, &[80001]);
    env.exec(
        r##"
        local logIndex = C_QuestLog.GetLogIndexForQuestID(456789)
        assert(IsQuestWatched(logIndex) == false, "changed ID is not auto-watched")
        assert(select('#', C_QuestLog.GetQuestWatchType(456789)) == 1)
        assert(C_QuestLog.GetQuestWatchType(456789) == nil)
        assert(C_QuestLog.AddQuestWatch(456789) == true)
        "##,
    )
    .unwrap();
    assert_watch_projection(&env, &[456789, 80001]);
    env.exec(
        r##"
        assert(not C_QuestLog.IsOnQuest(80000))
        assert(not pcall(C_QuestLog.GetQuestDifficultyLevel, 80000))
        local info = C_QuestLog.GetInfo(C_QuestLog.GetLogIndexForQuestID(456789))
        assert(info.title == "Live ordinary quest" and info.level == 23)
        assert(C_QuestLog.GetQuestDifficultyLevel(456789) == 23)
        "##,
    )
    .unwrap();
    {
        let mut state = env.state().borrow_mut();
        let mut inserted = state.quest_log_entries.entries[0].clone();
        inserted.quest_id = 456790;
        inserted.title = "Inserted ordinary quest".into();
        inserted.level = 37;
        state.quest_log_entries.entries.insert(1, inserted);
    }
    assert_watch_projection(&env, &[456789, 80001]);
    env.exec(
        r##"
        local logIndex = C_QuestLog.GetLogIndexForQuestID(456790)
        assert(IsQuestWatched(logIndex) == false, "inserted row is not auto-watched")
        assert(select('#', C_QuestLog.GetQuestWatchType(456790)) == 1)
        assert(C_QuestLog.GetQuestWatchType(456790) == nil)
        assert(C_QuestLog.AddQuestWatch(456790) == true)
        "##,
    )
    .unwrap();
    assert_watch_projection(&env, &[456789, 456790, 80001]);
    env.state().borrow_mut().quest_log_entries.entries.reverse();
    assert_watch_projection(&env, &[80001, 456790, 456789]);
    {
        let mut state = env.state().borrow_mut();
        for entry in &mut state.quest_log_entries.entries {
            if entry.quest_id == 80001 {
                entry.is_world_quest = true;
            }
            if entry.quest_id == 90001 {
                entry.is_world_quest = false;
            }
        }
    }
    assert_watch_projection(&env, &[456790, 456789]);
    env.exec(
        r##"
        local logIndex = C_QuestLog.GetLogIndexForQuestID(90001)
        assert(IsQuestWatched(logIndex) == false, "newly ordinary row needs membership")
        assert(select('#', C_QuestLog.GetQuestWatchType(90001)) == 1)
        assert(C_QuestLog.GetQuestWatchType(90001) == nil)
        assert(C_QuestLog.AddQuestWatch(90001) == true)
        "##,
    )
    .unwrap();
    assert_watch_projection(&env, &[90001, 456790, 456789]);
    env.state()
        .borrow_mut()
        .quest_log_entries
        .entries
        .retain(|entry| entry.quest_id != 456789);
    assert_watch_projection(&env, &[90001, 456790]);
    env.exec(
        r##"
        assert(not C_QuestLog.IsOnQuest(456789))
        assert(C_QuestLog.GetLogIndexForQuestID(456789) == nil)
        assert(not pcall(C_QuestLog.GetQuestDifficultyLevel, 456789))
        assert(C_QuestLog.GetQuestDifficultyLevel(456790) == 37)
        "##,
    )
    .unwrap();
    {
        let mut state = env.state().borrow_mut();
        for entry in &mut state.quest_log_entries.entries {
            entry.is_world_quest = true;
        }
    }
    assert_watch_projection(&env, &[]);
    env.state().borrow_mut().quest_log_entries.entries.clear();
    assert_watch_projection(&env, &[]);
}

#[test]
fn forever_quest_watch_fresh_environments_do_not_share_projection_state() {
    let mutated = WowLuaEnv::new().unwrap();
    {
        let mut state = mutated.state().borrow_mut();
        let mut only = state.quest_log_entries.entries[0].clone();
        only.quest_id = 456791;
        only.level = 19;
        state.quest_log_entries.entries = vec![only];
    }
    assert_watch_projection(&mutated, &[]);
    mutated
        .exec(
            r##"
            local logIndex = C_QuestLog.GetLogIndexForQuestID(456791)
            assert(IsQuestWatched(logIndex) == false, "replacement ID needs membership")
            assert(select('#', C_QuestLog.GetQuestWatchType(456791)) == 1)
            assert(C_QuestLog.GetQuestWatchType(456791) == nil)
            assert(C_QuestLog.AddQuestWatch(456791) == true)
            "##,
        )
        .unwrap();
    assert_watch_projection(&mutated, &[456791]);
    let fresh = WowLuaEnv::new().unwrap();
    assert_watch_projection(&fresh, &[80000, 80001]);
    fresh
        .exec(
            r##"
            assert(not C_QuestLog.IsOnQuest(456791))
            assert(C_QuestLog.GetLogIndexForQuestID(456791) == nil)
            assert(not pcall(C_QuestLog.GetQuestDifficultyLevel, 456791))
            assert(C_QuestLog.GetQuestDifficultyLevel(80000) == 80)
            "##,
        )
        .unwrap();
    assert_watch_projection(&mutated, &[456791]);
    mutated
        .state()
        .borrow_mut()
        .quest_log_entries
        .entries
        .clear();
    assert_watch_projection(&mutated, &[]);
    assert_watch_projection(&fresh, &[80000, 80001]);
}
