//! Supplied flat Wrath log reads through unchanged original subconsumers.
#![cfg(feature = "client-wrath")]

use sha2::{Digest, Sha256};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::GossipQuestRow;

fn assert_errors(env: &WowLuaEnv) {
    let handler: String = env.eval("return table.concat(LogErrors, '\\n')").unwrap();
    let sim = env.state().borrow();
    eprintln!(
        "handler={handler:?}; lua_errors={:?}; counts={:?}",
        sim.lua_errors, sim.lua_error_counts
    );
    assert!(handler.is_empty(), "{handler}");
    assert!(sim.lua_errors.is_empty(), "{:?}", sim.lua_errors);
    assert!(
        sim.lua_error_counts.is_empty(),
        "{:?}",
        sim.lua_error_counts
    );
}

fn exec(env: &WowLuaEnv, source: &str) {
    let result = env.exec(source);
    eprintln!("direct eval: {result:?}");
    assert_errors(env);
    result.expect("original consumer and contract assertions");
}

fn load_originals(env: &WowLuaEnv) {
    for (file, expected) in [
        (
            "QuestLogFrame.lua",
            "6ac6ca92b3036417082b88cc2f8b5c9fc8889f51f82aa726fe0bcd44990bc5e2",
        ),
        (
            "QuestInfo.lua",
            "3bfe691acdb2a095a6633c3ce7bef4a13f53f464d20781b2e850e442f9b715cd",
        ),
    ] {
        let path = dirs::cache_dir()
            .unwrap()
            .join("wow-ui-sim/blizzard-ui/wrath/FrameXML")
            .join(file);
        let bytes = std::fs::read(&path).expect("unchanged Wrath cache required");
        let hash = format!("{:x}", Sha256::digest(&bytes));
        assert_eq!(hash, expected, "source drift: {}", path.display());
        eprintln!("original {} sha256={hash}", path.display());
        let result = env.exec_named(
            std::str::from_utf8(&bytes).unwrap(),
            &format!("@{}", path.display()),
        );
        assert_errors(env);
        result.expect("whole original source loads");
    }
}

fn seed_env(order: &[usize]) -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        "LogErrors = {}; seterrorhandler(function(e) LogErrors[#LogErrors+1] = tostring(e) end)",
    )
    .unwrap();
    {
        let mut sim = env.state().borrow_mut();
        let fixtures = [
            (990101, "Wrath supplied first", 17),
            (990202, "Wrath supplied second", 23),
        ];
        let entries = order
            .iter()
            .map(|&index| {
                let (quest_id, title, quest_level) = fixtures[index];
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
        sim.quest_log_entries.entries = entries;
        sim.quest_log = sim
            .quest_log_entries
            .entries
            .iter()
            .map(|row| row.quest_id as u32)
            .collect();
        sim.quest_log_entries.completed_quest_ids.clear();
        sim.selected_quest_log_id = None;
        sim.pending_quest_offer = None;
    }
    exec(
        &env,
        r#"
        QuestInfoFrame = CreateFrame("Frame", "QuestInfoFrame")
        QuestInfoFrame.questLog = true
        QuestInfoTitleHeader = QuestInfoFrame:CreateFontString("QuestInfoTitleHeader", "ARTWORK")
        QuestDifficultyColors = {}
        for _, key in ipairs({"impossible", "verydifficult", "difficult", "standard", "trivial", "header"}) do
            QuestDifficultyColors[key] = {}
        end
        QuestDifficulty_Impossible = CreateFont("LogImpossible")
        QuestDifficulty_VeryDifficult = CreateFont("LogVeryDifficult")
        QuestDifficulty_Difficult = CreateFont("LogDifficult")
        QuestDifficulty_Standard = CreateFont("LogStandard")
        QuestDifficulty_Trivial = CreateFont("LogTrivial")
        QuestDifficulty_Header = CreateFont("LogHeader")
    "#,
    );
    load_originals(&env);
    env
}

fn preserved(env: &WowLuaEnv) -> String {
    let sim = env.state().borrow();
    let mut log = sim.quest_log_entries.clone();
    // Selection may invalidate dialog readiness, but must not mutate membership/data.
    log.readiness.dialog = None;
    format!(
        "{:?}|{log:?}|{:?}|{:?}|{}|{}|{:?}",
        sim.quest_log,
        sim.gossip,
        sim.pending_quest_offer,
        sim.player.money,
        sim.player.xp,
        sim.events.pending()
    )
}

fn assert_tuple(env: &WowLuaEnv, index: usize, id: i32, title: &str, level: i32, completion: &str) {
    exec(
        env,
        &format!(
            r#"
        local title, level, tag, group, header, collapsed, complete, daily, id, display = GetQuestLogTitle({index})
        assert(title == "{title}", "supplied title")
        assert(level == {level} and id == {id}, "supplied level and slot-nine identity")
        assert(tag == nil and group == nil, "unknown tag/group absent")
        assert(header == nil and collapsed == nil, "ordinary flags are nil, not boolean/zero")
        assert(complete == {completion}, "period completion flag")
        assert(daily == nil and display == nil, "unrepresented classifications absent")
    "#
        ),
    );
}

fn exercise_selection(order: &[usize], index: usize) {
    let env = seed_env(order);
    let independent = seed_env(&[0, 1]);
    let before = preserved(&env);
    let independent_before = preserved(&independent);
    let (id, title, level) = if order[index - 1] == 0 {
        (990101, "Wrath supplied first", 17)
    } else {
        (990202, "Wrath supplied second", 23)
    };
    exec(
        &env,
        &format!(
            r#"
        local n, quests = GetNumQuestLogEntries()
        assert(n == 2 and quests == 2)
        assert(QuestLog_GetFirstSelectableQuest() == 1)
        SelectQuestLogEntry({index})
        assert(GetQuestLogSelection() == {index}, "legacy index selection")
        local shown = QuestInfo_ShowTitle()
        assert(shown == QuestInfoTitleHeader)
        assert(shown:GetText() == "{title}", "original selected-title consumer")
    "#
        ),
    );
    assert_eq!(env.state().borrow().selected_quest_log_id, Some(id as u32));
    assert_tuple(&env, index, id, title, level, "nil");
    assert_eq!(preserved(&env), before);
    assert_eq!(preserved(&independent), independent_before);
    assert!(independent.state().borrow().selected_quest_log_id.is_none());
    assert_errors(&independent);
}

#[test]
fn original_index_one() {
    exercise_selection(&[0, 1], 1);
}

#[test]
fn original_index_two() {
    exercise_selection(&[0, 1], 2);
}

#[test]
fn original_reordered_index_two() {
    exercise_selection(&[1, 0], 2);
}

#[test]
fn selection_tracks_identity_after_reordering() {
    let env = seed_env(&[0, 1]);
    exec(&env, "SelectQuestLogEntry(1)");
    env.state().borrow_mut().quest_log_entries.entries.reverse();
    exec(
        &env,
        r#"
        assert(GetQuestLogSelection() == 2)
        assert(QuestInfo_ShowTitle():GetText() == "Wrath supplied first")
    "#,
    );
}

#[test]
fn original_empty_log() {
    let env = seed_env(&[]);
    let before = preserved(&env);
    exec(
        &env,
        r#"
        local n, quests = GetNumQuestLogEntries()
        assert(n == 0 and quests == 0)
        assert(QuestLog_GetFirstSelectableQuest() == 0)
        assert(GetQuestLogSelection() == 0)
        assert(GetQuestLogTitle(1) == nil)
        assert(QuestInfo_ShowTitle():GetText() == "")
    "#,
    );
    assert!(env.state().borrow().selected_quest_log_id.is_none());
    assert_eq!(preserved(&env), before);
}

#[test]
fn completion_flags_use_active_rows_not_history() {
    let env = seed_env(&[0, 1]);
    {
        let mut sim = env.state().borrow_mut();
        sim.quest_log_entries.entries[0].is_complete = true;
        sim.quest_log_entries.entries[1].is_failed = true;
    }
    let before = preserved(&env);
    assert_tuple(&env, 1, 990101, "Wrath supplied first", 17, "1");
    assert_tuple(&env, 2, 990202, "Wrath supplied second", 23, "-1");
    assert_eq!(preserved(&env), before);
    {
        let mut sim = env.state().borrow_mut();
        sim.quest_log_entries.entries[0].is_complete = false;
        sim.quest_log_entries.entries[0].is_flagged_completed = true;
        sim.quest_log_entries.completed_quest_ids.insert(990101);
    }
    let before = preserved(&env);
    assert_tuple(&env, 1, 990101, "Wrath supplied first", 17, "nil");
    assert_eq!(preserved(&env), before);
}
