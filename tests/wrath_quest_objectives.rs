//! Supplied objective rendering through the original Wrath consumer.
//! Supplied objective rendering only; no acceptance/progress dialog/turn-in producer.
#![cfg(feature = "client-wrath")]

use sha2::{Digest, Sha256};
use wow_ui_sim::c_api::quest_objectives::{
    QuestObjective, QuestObjectiveProgress, commit_objective_progress,
};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::GossipQuestRow;

#[path = "common/wrath_full_ui.rs"]
mod wrath_full_ui;

fn assert_errors(env: &WowLuaEnv) {
    let errors: String = env.eval("return table.concat(N2Errors, '\\n')").unwrap();
    assert!(errors.is_empty(), "handler errors: {errors}");
    let sim = env.state().borrow();
    assert!(sim.lua_errors.is_empty(), "{:?}", sim.lua_errors);
    assert!(
        sim.lua_error_counts.is_empty(),
        "{:?}",
        sim.lua_error_counts
    );
}

fn exec(env: &WowLuaEnv, source: &str) {
    let result = env.exec(source);
    assert_errors(env);
    result.expect("original consumer and objective assertions");
}

fn assert_source_seals() {
    for (path, expected) in [
        (
            "/home/osso/.cache/wow-ui-sim/blizzard-ui/wrath/FrameXML/QuestInfo.lua",
            "3bfe691acdb2a095a6633c3ce7bef4a13f53f464d20781b2e850e442f9b715cd",
        ),
        (
            "/home/osso/.cache/wow-ui-sim/blizzard-ui/wrath/FrameXML/QuestLogFrame.lua",
            "6ac6ca92b3036417082b88cc2f8b5c9fc8889f51f82aa726fe0bcd44990bc5e2",
        ),
        (
            "/home/osso/.cache/wow-ui-sim/blizzard-ui/wrath/FrameXML/QuestInfo.xml",
            "290ed9e781c98734b69829184b171b99348c8d34ff3972292a01b1db7736bccc",
        ),
    ] {
        let bytes = std::fs::read(path).expect("original source, no alternate source");
        assert_eq!(format!("{:x}", Sha256::digest(bytes)), expected, "{path}");
    }
}

fn objective(text: &str, kind: &str, finished: bool, counts: (i32, i32)) -> QuestObjective {
    QuestObjective {
        text: text.into(),
        objective_type: kind.into(),
        finished,
        num_fulfilled: counts.0,
        num_required: counts.1,
        criteria_id: None,
    }
}

fn snapshot_a(done: bool) -> QuestObjectiveProgress {
    QuestObjectiveProgress {
        objectives: vec![
            objective(
                if done {
                    "Wolves slain: 2/2"
                } else {
                    "Wolves slain: 1/2"
                },
                "monster",
                done,
                (if done { 2 } else { 1 }, 2),
            ),
            objective("Sealed relic recovered: 1/1", "item", true, (1, 1)),
        ],
        ..Default::default()
    }
}

fn seed(env: &WowLuaEnv) {
    let mut sim = env.state().borrow_mut();
    let rows = [
        (990101, "Wrath supplied wolves", 17),
        (990202, "Wrath supplied delivery", 23),
    ]
    .into_iter()
    .map(|(quest_id, title, quest_level)| {
        sim.quest_log_entries
            .snapshot_gossip_offer(&GossipQuestRow {
                quest_id,
                title: title.into(),
                quest_level,
                is_complete: Some(false),
                ..Default::default()
            })
            .unwrap()
    })
    .collect();
    sim.quest_log_entries.entries = rows;
    sim.quest_log = vec![990101, 990202];
    sim.quest_log_entries.objective_progress.clear();
    sim.quest_log_entries.completed_quest_ids.clear();
    sim.quest_log_entries.processed_reward_factions.clear();
    sim.selected_quest_log_id = None;
    sim.pending_quest_offer = None;
    assert!(commit_objective_progress(
        &mut sim.quest_log_entries,
        990101,
        snapshot_a(false)
    ));
    assert!(commit_objective_progress(
        &mut sim.quest_log_entries,
        990202,
        QuestObjectiveProgress {
            objectives: vec![objective("Crates gathered: 0/3", "item", false, (0, 3))],
            ..Default::default()
        },
    ));
}

fn load_supplied_objective_env() -> WowLuaEnv {
    assert_source_seals();
    let env = wrath_full_ui::load_original_wrath_startup();
    // Do not clear startup errors to manufacture an objective RED.
    {
        let sim = env.state().borrow();
        assert!(
            sim.lua_errors.is_empty(),
            "startup precondition: {:?}",
            sim.lua_errors
        );
        assert!(
            sim.lua_error_counts.is_empty(),
            "startup precondition: {:?}",
            sim.lua_error_counts
        );
    }
    env.exec("N2Errors = {}; seterrorhandler(function(e) N2Errors[#N2Errors+1] = tostring(e) end)")
        .unwrap();
    seed(&env);
    exec(&env, include_str!("wrath_quest_objectives_fixture.lua"));
    exec(
        &env,
        "SelectQuestLogEntry(1); assert(GetQuestLogSelection() == 1)",
    );
    env
}

#[test]
fn supplied_objectives_through_original_objective_body() {
    let env = load_supplied_objective_env();
    // Expected first RED is original consumer frame return, not missing producer.
    exec(&env, "N2.directA(false)");
}

#[test]
fn supplied_objectives_original_detail_reorder_and_finished_transition() {
    let env = load_supplied_objective_env();
    exec(&env, "N2.detailA(false)");
    {
        let mut sim = env.state().borrow_mut();
        sim.quest_log_entries.entries.reverse();
        sim.quest_log.reverse();
    }
    exec(
        &env,
        "assert(GetQuestLogSelection() == 2); N2.detailA(false); N2.assertExplicitRows()",
    );
    exec(
        &env,
        "SelectQuestLogEntry(1); N2.detailB(); SelectQuestLogEntry(2)",
    );
    let (history, membership, money, xp, other) = {
        let sim = env.state().borrow();
        (
            sim.quest_log_entries.completed_quest_ids.clone(),
            sim.quest_log.clone(),
            sim.player.money,
            sim.player.xp,
            sim.quest_log_entries
                .objective_progress
                .get(&990202)
                .cloned(),
        )
    };
    {
        let mut sim = env.state().borrow_mut();
        assert!(commit_objective_progress(
            &mut sim.quest_log_entries,
            990101,
            snapshot_a(true)
        ));
    }
    exec(&env, "N2.detailA(true)");
    let sim = env.state().borrow();
    assert_eq!(sim.selected_quest_log_id, Some(990101));
    assert_eq!(sim.quest_log, membership);
    assert_eq!(
        sim.quest_log_entries
            .entries
            .iter()
            .map(|q| q.quest_id)
            .collect::<Vec<_>>(),
        vec![990202, 990101]
    );
    assert!(
        sim.quest_log_entries
            .entries
            .iter()
            .all(|q| !q.is_complete && !q.is_failed)
    );
    assert_eq!(sim.quest_log_entries.completed_quest_ids, history);
    assert_eq!(sim.player.money, money);
    assert_eq!(sim.player.xp, xp);
    assert_eq!(
        sim.quest_log_entries.objective_progress.get(&990202),
        other.as_ref()
    );
    // Native exact return arity/types, kinds, finished representation and invalid
    // indices remain PENDING authority; no event scheduling claim.
}

#[test]
fn supplied_objectives_empty_unknown_and_independent_environments() {
    let env = load_supplied_objective_env();
    let independent = load_supplied_objective_env();
    exec(&env, "N2.directA(false)");
    {
        let mut sim = env.state().borrow_mut();
        assert!(commit_objective_progress(
            &mut sim.quest_log_entries,
            990101,
            QuestObjectiveProgress::default(),
        ));
    }
    exec(&env, "N2.empty()");
    exec(&independent, "N2.directA(false); N2.detailA(false)");
    {
        let sim = env.state().borrow();
        assert!(
            sim.quest_log_entries
                .objective_progress
                .contains_key(&990101)
        );
        assert!(
            sim.quest_log_entries.objective_progress[&990101]
                .objectives
                .is_empty()
        );
    }
    env.state()
        .borrow_mut()
        .quest_log_entries
        .objective_progress
        .remove(&990101);
    exec(&env, "N2.empty()");
}

#[test]
fn supplied_objective_type_substitution_and_queries_are_read_only() {
    let env = load_supplied_objective_env();
    let before = {
        let mut sim = env.state().borrow_mut();
        let progress = QuestObjectiveProgress {
            objectives: vec![objective("", "authored fallback text", false, (0, 1))],
            ..Default::default()
        };
        assert!(commit_objective_progress(
            &mut sim.quest_log_entries,
            990101,
            progress
        ));
        format!("{:?}", sim.quest_log_entries)
    };
    exec(
        &env,
        r#"
        local frame, last = QuestInfo_ShowObjectives()
        assert(frame == QuestInfoObjectivesFrame and last == QuestInfoObjective1)
        assert(QuestInfoObjective1:GetText() == "authored fallback text")
        assert(QuestInfoObjective1:IsShown() and not QuestInfoObjective2:IsShown())
        for i = 1, 3 do
            assert(GetNumQuestLeaderBoards() == 1)
            assert(GetQuestLogLeaderBoard(1) == "")
            GetQuestLogQuestText()
        end
    "#,
    );
    let sim = env.state().borrow();
    assert_eq!(format!("{:?}", sim.quest_log_entries), before);
    assert_eq!(sim.selected_quest_log_id, Some(990101));
}
