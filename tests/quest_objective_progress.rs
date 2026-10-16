//! Approval-pending QL-1: supplied objectives, not native quest lifecycle parity.
#![cfg(any(feature = "client-retail", feature = "client-wowforever"))]

use wow_ui_sim::lua_api::WowLuaEnv;

const QUEST_ID: i32 = 990001;

fn supply_active_quest(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    let mut quest = state.quest_log_entries.entries[0].clone();
    quest.quest_id = QUEST_ID;
    quest.title = "Authored wolves and relic".into();
    quest.is_complete = false;
    quest.is_failed = false;
    quest.is_flagged_completed = false;
    state.quest_log_entries.entries.push(quest);
    state.quest_log.push(QUEST_ID as u32);
}

fn objective_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("isolated objective environment");
    supply_active_quest(&env);
    env.exec(include_str!("quest_objective_progress.lua"))
        .expect("define test inputs; do not substitute quest APIs");
    env
}

fn tracker_env() -> WowLuaEnv {
    let cache = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path()
        .expect("current-profile cache must exist; no source fallback");
    let (env, loaded) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &cache,
        &["Blizzard_ObjectiveTracker"],
        &[],
    );
    assert!(
        loaded
            .iter()
            .any(|name| name == "Blizzard_ObjectiveTracker")
    );
    supply_active_quest(&env);
    env.exec(include_str!("quest_objective_progress.lua"))
        .expect("define objective fixtures after original addon closure");
    env.exec("QL1.installTracker()")
        .expect("original cached function and real block/line/progress templates");
    env
}

fn assert_objective_count(env: &WowLuaEnv, count: i32) {
    let actual: i32 = env
        .eval("return C_QuestLog.GetNumQuestObjectives(990001)")
        .expect("query objective count for represented custom quest");
    assert_eq!(actual, count, "QL-1: supplied objectives must be queryable");
}

#[test]
fn ql1_mutable_counts_share_rich_and_legacy_projections_without_history_or_cross_env_leaks() {
    let env = objective_env();
    let other = objective_env();
    let history = env
        .state()
        .borrow()
        .quest_log_entries
        .completed_quest_ids
        .clone();
    env.exec("QL1.supply(0)").unwrap();
    assert_objective_count(&env, 2);
    env.exec("QL1.assertRows(0)").unwrap();
    other.exec("QL1.supply(0)").unwrap();
    for fulfilled in [1, 2] {
        env.exec(&format!("QL1.supply({fulfilled})")).unwrap();
        env.exec(&format!("QL1.assertRows({fulfilled})")).unwrap();
    }
    // ID-based objectives must follow the live rich index, never catalog index 2.
    env.state().borrow_mut().quest_log_entries.entries.reverse();
    env.exec("QL1.assertRows(2)").unwrap();
    assert_objective_count(&other, 2);
    other.exec("QL1.assertRows(0)").unwrap();
    assert_eq!(
        env.state().borrow().quest_log_entries.completed_quest_ids,
        history
    );
    env.exec("assert(C_QuestLog.IsOnQuest(990001)); assert(not C_QuestLog.IsQuestFlaggedCompleted(990001))")
        .unwrap();
}

#[test]
fn ql1_notifications_observe_committed_queries_and_original_tracker_text_and_finished_transition() {
    let env = tracker_env();
    env.exec("QL1.supply(0)").unwrap();
    assert_objective_count(&env, 2);
    env.exec("QL1.render(); QL1.observeTracker()").unwrap();
    let before: String = env
        .eval("return QL1.block:GetExistingLine(1).Text:GetText()")
        .unwrap();
    assert_eq!(before, "Wolves: 0/2");
    for fulfilled in [1, 2] {
        env.exec(&format!("QL1.supply({fulfilled})")).unwrap();
        // Outside callbacks: swallowed event-handler errors cannot pass this gate.
        let observed: (i32, String, String, bool) = env
            .eval(
                "return #QL1.updates, QL1.updates[#QL1.updates].text, \
                   QL1.updates[#QL1.updates].relic, QL1.updates[#QL1.updates].committed",
            )
            .expect("each real notification must finish original tracker consumption");
        assert_eq!(
            observed,
            (
                fulfilled,
                format!("Wolves: {fulfilled}/2"),
                "Relic: 0/1".into(),
                true
            )
        );
        env.exec(&format!("QL1.assertRows({fulfilled})")).unwrap();
    }
    let completing: bool = env
        .eval(
            "return QL1.block:GetExistingLine(1).state == ObjectiveTrackerAnimLineState.Completing",
        )
        .unwrap();
    assert!(
        completing,
        "original existing-line finished branch must begin completion"
    );
    env.exec("QL1.supply(2)").unwrap();
    let counts: (i32, i32, i32) = env
        .eval("return #QL1.updates, #QL1.watches, QL1.criteriaCount")
        .unwrap();
    assert_eq!(
        counts,
        (2, 2, 0),
        "unchanged snapshot is inert; ordinary objectives invent no criteria ID"
    );
    env.exec("QL1.assertWatchPayloads()").unwrap();
}

#[test]
fn ql1_progressbar_query_and_original_statusbar_react_to_supplied_fifty_percent() {
    let env = tracker_env();
    env.exec("QL1.supplyBar(25)").unwrap();
    let initial: f64 = env
        .eval("return GetQuestProgressBarPercent(990001)")
        .unwrap();
    assert_eq!(
        initial, 25.0,
        "QL-1: percentage cannot remain constant zero"
    );
    env.exec("QL1.render(); QL1.observeTracker(); QL1.supplyBar(50)")
        .unwrap();
    let output: (i32, f64, f64, String, bool) = env
        .eval(
            "local line = QL1.block:GetExistingLine(1); \
               return #QL1.updates, GetQuestProgressBarPercent(990001), \
               line.progressBar.Bar:GetValue(), line.progressBar.Bar.Label:GetText(), \
               QL1.updates[1].committed",
        )
        .expect("original progress-bar consumer emits real StatusBar/FontString state");
    assert_eq!(output, (1, 50.0, 50.0, "50%".into(), true));
    env.exec("QL1.assertBar(50); QL1.assertWatchPayloads()")
        .unwrap();
}

#[test]
fn ql1_represented_criteria_payload_is_exact_and_queries_are_committed_before_notification() {
    let env = objective_env();
    env.exec("QL1.supplyCriteria(1)").unwrap();
    assert_objective_count(&env, 1);
    env.exec("QL1.observeCriteria(); QL1.supplyCriteria(2); QL1.supplyCriteria(2)")
        .unwrap();
    let result: (i32, String, bool) = env
        .eval("return #QL1.criteria, QL1.criteria[1].payload, QL1.criteria[1].committed")
        .expect("represented criteria notification must reach callback completion");
    assert_eq!(result, (1, "5:990001:71:Runes:2:3".into(), true));
}

#[test]
fn ql1_bootstrap_preserves_nonzero_catalog_cardinality_and_static_objective_text() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        assert(C_QuestLog.GetNumQuestObjectives(80000) == 2)
        local map = C_QuestLog.GetQuestsOnMap(2248)
        assert(#map == 2 and map[1].questID == 80000 and map[1].numObjectives == 2)
        local declared = {
            [90101]=1, [90102]=2, [90103]=3, [90104]=1,
            [90001]=1, [90002]=1, [90003]=3, [90004]=1,
            [90005]=1, [90006]=2, [90007]=1, [90008]=2,
        }
        for _, mapID in ipairs({2248,2215,2214,2025,2024,2023,2022}) do
            for _, quest in ipairs(C_TaskQuest.GetQuestsOnMap(mapID)) do
                assert(quest.numObjectives == declared[quest.questID], 'preserve declared cardinality')
                assert(C_QuestLog.GetNumQuestObjectives(quest.questID) == declared[quest.questID])
            end
        end
        C_QuestLog.SetSelectedQuest(80000)
        local description, text = GetQuestLogQuestText()
        assert(description:find('Ironforge expedition', 1, true))
        assert(text == 'Ironforge Relics collected: 3/5\nExplore the Old Quarry')
        C_QuestLog.SetSelectedQuest(80002)
        assert(GetObjectiveText() == 'Supplies gathered: 5/5\nDeliver to Quartermaster')
    "#).unwrap();
}

#[test]
fn ql1_world_map_counts_share_supplied_rows_before_notifications() {
    let env = objective_env();
    env.exec(
        r#"
        QL1.id = 90001
        QL1.worldEvents = 0
        QL1.worldObserver = CreateFrame('Frame')
        QL1.worldObserver:RegisterEvent('QUEST_LOG_UPDATE')
        QL1.worldObserver:SetScript('OnEvent', function()
            local count = C_QuestLog.GetNumQuestObjectives(90001)
            assert(count == 2)
            assert(#C_QuestLog.GetQuestObjectives(90001) == count)
            assert(C_QuestLog.GetWorldQuestInfo(90001).numObjectives == count)
            local seenLog, seenTask = false, false
            for _, quest in ipairs(C_QuestLog.GetQuestsOnMap(2025)) do
                if quest.questID == 90001 then seenLog = quest.numObjectives == count end
            end
            for _, quest in ipairs(C_TaskQuest.GetQuestsOnMap(2025)) do
                if quest.questID == 90001 then seenTask = quest.numObjectives == count end
            end
            assert(seenLog and seenTask)
            QL1.worldEvents = QL1.worldEvents + 1
        end)
        QL1.supply(1)
    "#,
    )
    .unwrap();
    let events: i32 = env.eval("return QL1.worldEvents").unwrap();
    assert_eq!(
        events, 1,
        "all world/map count projections must commit before event consumer"
    );
}
