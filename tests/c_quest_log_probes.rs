//! Integration tests for `src/lua_api/globals/missing_surface/quest_log.rs`.
//!
//! Covers all 17 ported C_QuestLog methods against the seeded
//! `SimState.quest_log_entries` store.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── GetNumQuestLogEntries ─────────────────────────────────────────────────────

#[test]
fn get_num_quest_log_entries_returns_seeded_count() {
    let env = env();
    let (shown, total): (i32, i32) = env
        .eval("return C_QuestLog.GetNumQuestLogEntries()")
        .unwrap();
    assert_eq!(shown, 5);
    assert_eq!(total, 4);
}

#[test]
fn get_num_quest_log_entries_reflects_state_mutation() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.quest_log_entries.entries.clear();
    }
    let (shown, total): (i32, i32) = env
        .eval("return C_QuestLog.GetNumQuestLogEntries()")
        .unwrap();
    assert_eq!(shown, 0);
    assert_eq!(total, 0);
}

#[test]
fn get_max_num_quests_can_accept_returns_classic_limit() {
    let env = env();
    let max_quests: i32 = env.eval("return C_QuestLog.GetMaxNumQuestsCanAccept()").unwrap();
    assert_eq!(max_quests, 25);
}

// ── GetInfo ───────────────────────────────────────────────────────────────────

#[test]
fn get_info_returns_table_for_valid_index() {
    let env = env();
    let quest_id: i32 = env
        .eval("local info = C_QuestLog.GetInfo(2); return info and info.questID or -1")
        .unwrap();
    assert_eq!(quest_id, 80000);
}

#[test]
fn get_info_exposes_header_at_first_index() {
    let env = env();
    let (title, quest_id, is_header): (String, i32, bool) = env
        .eval(
            r#"
            local info = C_QuestLog.GetInfo(1)
            return info.title, info.questID, info.isHeader
            "#,
        )
        .unwrap();
    assert_eq!(title, "Khaz Algar");
    assert_eq!(quest_id, 0);
    assert!(is_header);
}

#[test]
fn get_info_returns_nil_for_out_of_range() {
    let env = env();
    let result: Option<bool> = env
        .eval("local info = C_QuestLog.GetInfo(999); return info ~= nil or nil")
        .unwrap();
    assert!(result.is_none());
}

#[test]
fn get_info_fields_match_entry() {
    let env = env();
    let (title, level, is_complete): (String, i32, bool) = env
        .eval(
            r#"
            local info = C_QuestLog.GetInfo(3)
            return info.title, info.level, info.isComplete
            "#,
        )
        .unwrap();
    assert_eq!(title, "Defending the Gates");
    assert_eq!(level, 80);
    assert!(is_complete);
}

// ── GetLogIndexForQuestID ─────────────────────────────────────────────────────

#[test]
fn get_log_index_for_quest_id_returns_correct_index() {
    let env = env();
    let idx: i32 = env
        .eval("return C_QuestLog.GetLogIndexForQuestID(80001) or -1")
        .unwrap();
    assert_eq!(idx, 3);
}

#[test]
fn get_log_index_for_unknown_quest_returns_nil() {
    let env = env();
    let result: Option<i32> = env
        .eval("return C_QuestLog.GetLogIndexForQuestID(99999)")
        .unwrap();
    assert!(result.is_none());
}

// ── IsOnQuest ─────────────────────────────────────────────────────────────────

#[test]
fn is_on_quest_true_for_seeded_quest() {
    let env = env();
    let result: bool = env.eval("return C_QuestLog.IsOnQuest(80000)").unwrap();
    assert!(result);
}

#[test]
fn is_on_quest_false_for_unknown_quest() {
    let env = env();
    let result: bool = env.eval("return C_QuestLog.IsOnQuest(12345)").unwrap();
    assert!(!result);
}

// ── IsComplete ────────────────────────────────────────────────────────────────

#[test]
fn is_complete_true_for_completed_quest() {
    let env = env();
    let result: bool = env.eval("return C_QuestLog.IsComplete(80001)").unwrap();
    assert!(result);
}

#[test]
fn is_complete_false_for_incomplete_quest() {
    let env = env();
    let result: bool = env.eval("return C_QuestLog.IsComplete(80000)").unwrap();
    assert!(!result);
}

// ── IsFailed ──────────────────────────────────────────────────────────────────

#[test]
fn is_failed_false_by_default() {
    let env = env();
    let result: bool = env.eval("return C_QuestLog.IsFailed(80000)").unwrap();
    assert!(!result);
}

#[test]
fn is_failed_true_when_mutated() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.quest_log_entries.entries[0].is_failed = true;
    }
    let result: bool = env.eval("return C_QuestLog.IsFailed(80000)").unwrap();
    assert!(result);
}

// ── IsMetaQuest ───────────────────────────────────────────────────────────────

#[test]
fn is_meta_quest_false_for_normal_quest() {
    let env = env();
    let result: bool = env.eval("return C_QuestLog.IsMetaQuest(80000)").unwrap();
    assert!(!result);
}

// ── IsWorldQuest ──────────────────────────────────────────────────────────────

#[test]
fn is_world_quest_true_for_world_quest() {
    let env = env();
    let result: bool = env.eval("return C_QuestLog.IsWorldQuest(90101)").unwrap();
    assert!(result);
}

#[test]
fn is_world_quest_false_for_normal_quest() {
    let env = env();
    let result: bool = env.eval("return C_QuestLog.IsWorldQuest(80000)").unwrap();
    assert!(!result);
}

// ── IsQuestReplayable ─────────────────────────────────────────────────────────

#[test]
fn is_quest_replayable_true_for_world_quest() {
    let env = env();
    let result: bool = env
        .eval("return C_QuestLog.IsQuestReplayable(90101)")
        .unwrap();
    assert!(result);
}

// ── IsOnMap ───────────────────────────────────────────────────────────────────

#[test]
fn is_on_map_true_when_map_id_set() {
    let env = env();
    let result: bool = env.eval("return C_QuestLog.IsOnMap(80000)").unwrap();
    assert!(result);
}

// ── IsQuestFlaggedCompleted ───────────────────────────────────────────────────

#[test]
fn is_quest_flagged_completed_true_for_seeded_id() {
    let env = env();
    let result: bool = env
        .eval("return C_QuestLog.IsQuestFlaggedCompleted(79999)")
        .unwrap();
    assert!(result);
}

#[test]
fn is_quest_flagged_completed_false_for_unknown() {
    let env = env();
    let result: bool = env
        .eval("return C_QuestLog.IsQuestFlaggedCompleted(12345)")
        .unwrap();
    assert!(!result);
}

// ── GetAllCompletedQuestIDs ───────────────────────────────────────────────────

#[test]
fn get_all_completed_quest_ids_contains_seeded_ids() {
    let env = env();
    let (has79999, has80001): (bool, bool) = env
        .eval(
            r#"
            local ids = C_QuestLog.GetAllCompletedQuestIDs()
            local has79999, has80001 = false, false
            for _, id in ipairs(ids) do
                if id == 79999 then has79999 = true end
                if id == 80001 then has80001 = true end
            end
            return has79999, has80001
            "#,
        )
        .unwrap();
    assert!(has79999);
    assert!(has80001);
}

// ── GetNextWaypoint ───────────────────────────────────────────────────────────

#[test]
fn get_next_waypoint_returns_xy_for_quest_with_waypoint() {
    let env = env();
    let (x, y): (f64, f64) = env
        .eval("return C_QuestLog.GetNextWaypoint(80000)")
        .unwrap();
    assert!((x - 0.45).abs() < 0.001);
    assert!((y - 0.35).abs() < 0.001);
}

#[test]
fn get_next_waypoint_returns_nothing_for_no_waypoint() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local t = {C_QuestLog.GetNextWaypoint(80001)}
            return #t
            "#,
        )
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn get_next_waypoint_for_map_returns_xy_for_matching_map() {
    let env = env();
    let (x, y): (f64, f64) = env
        .eval("return C_QuestLog.GetNextWaypointForMap(80000, 2248)")
        .unwrap();
    assert!((x - 0.45).abs() < 0.001);
    assert!((y - 0.35).abs() < 0.001);
}

#[test]
fn get_quests_on_map_returns_seeded_quest_pois() {
    let env = env();
    let (count, first_id, first_objectives): (i32, i32, i32) = env
        .eval(
            r#"
            local quests = C_QuestLog.GetQuestsOnMap(2248)
            return #quests, quests[1].questID, quests[1].numObjectives
            "#,
        )
        .unwrap();
    assert_eq!(count, 2);
    assert_eq!(first_id, 80000);
    assert_eq!(first_objectives, 2);
}

#[test]
fn get_max_num_quests_returns_numeric_capacity() {
    let env = env();
    let max_num_quests: i32 = env.eval("return C_QuestLog.GetMaxNumQuests()").unwrap();
    assert!(
        max_num_quests > 0,
        "QuestDataProvider needs a numeric quest-log capacity"
    );
}

#[test]
fn get_num_quest_objectives_uses_seeded_objectives() {
    let env = env();
    let count: i32 = env
        .eval("return C_QuestLog.GetNumQuestObjectives(80000)")
        .unwrap();
    assert_eq!(count, 2);
}

#[test]
fn is_threat_quest_defaults_to_false() {
    let env = env();
    let is_threat: bool = env.eval("return C_QuestLog.IsThreatQuest(80000)").unwrap();
    assert!(!is_threat);
}

#[test]
fn quest_poi_map_id_round_trips() {
    let env = env();
    let map_id: i32 = env
        .eval(
            r#"
            C_QuestLog.SetMapForQuestPOIs(2248)
            return C_QuestLog.GetMapForQuestPOIs()
            "#,
        )
        .unwrap();
    assert_eq!(map_id, 2248);
}

#[test]
fn quest_map_frame_get_focused_quest_id_returns_nothing_when_unfocused() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local values = {QuestMapFrame_GetFocusedQuestID()}
            return #values
            "#,
        )
        .unwrap();
    assert_eq!(count, 0);
}

// ── GetQuestTagInfo ───────────────────────────────────────────────────────────

#[test]
fn get_quest_tag_info_for_world_quest() {
    let env = env();
    let (tag_id, is_world): (i32, bool) = env
        .eval(
            r#"
            local info = C_QuestLog.GetQuestTagInfo(90101)
            return info.tagID, (info.tagName == "World Quest")
            "#,
        )
        .unwrap();
    assert_eq!(tag_id, 2);
    assert!(is_world);
}

#[test]
fn get_quest_tag_info_returns_nil_for_unknown_quest() {
    let env = env();
    let result: Option<bool> = env
        .eval("return C_QuestLog.GetQuestTagInfo(99999) ~= nil or nil")
        .unwrap();
    assert!(result.is_none());
}

// ── GetWorldQuestInfo ─────────────────────────────────────────────────────────

#[test]
fn get_world_quest_info_returns_table_for_world_quest() {
    let env = env();
    let (quest_id, map_id): (i32, i32) = env
        .eval(
            r#"
            local info = C_QuestLog.GetWorldQuestInfo(90101)
            return info.questID, info.mapID
            "#,
        )
        .unwrap();
    assert_eq!(quest_id, 90101);
    assert_eq!(map_id, 2248);
}

#[test]
fn get_world_quest_info_returns_nothing_for_normal_quest() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local t = {C_QuestLog.GetWorldQuestInfo(80000)}
            return #t
            "#,
        )
        .unwrap();
    assert_eq!(count, 0);
}

// ── GetQuestDetailsTheme ──────────────────────────────────────────────────────

#[test]
fn get_quest_details_theme_returns_nil_by_default() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local t = {C_QuestLog.GetQuestDetailsTheme(80000)}
            return #t
            "#,
        )
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn get_quest_details_theme_returns_value_when_set() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.quest_log_entries.entries[0].details_theme = Some("dragonisles".into());
    }
    let theme: String = env
        .eval("return C_QuestLog.GetQuestDetailsTheme(80000)")
        .unwrap();
    assert_eq!(theme, "dragonisles");
}

// ── GetBountySetInfoForMapID ──────────────────────────────────────────────────

#[test]
fn get_bounty_set_info_for_map_id_returns_nil() {
    let env = env();
    let result: Option<bool> = env
        .eval("return C_QuestLog.GetBountySetInfoForMapID(2248) ~= nil or nil")
        .unwrap();
    assert!(result.is_none());
}

#[test]
fn suggested_group_size_apis_return_number_not_nil() {
    let env = env();
    let (capi_group, global_group, can_compare_capi, can_compare_global): (i32, i32, bool, bool) =
        env.eval(
            r#"
            C_QuestLog.SetSelectedQuest(80000)
            local capi = C_QuestLog.GetSuggestedGroupSize(C_QuestLog.GetSelectedQuest())
            local global = GetSuggestedGroupSize()
            return capi, global, (capi > 0), (global > 0)
            "#,
        )
        .unwrap();
    assert_eq!(capi_group, 0);
    assert_eq!(global_group, 0);
    assert!(!can_compare_capi);
    assert!(!can_compare_global);
}

#[test]
fn quest_reward_globals_are_present_and_numeric_safe() {
    let env = env();
    let (quest_log_ok, offer_ok): (bool, bool) = env
        .eval(
            r#"
            local function quest_log_probe()
                local numQuestRewards = GetNumQuestLogRewards()
                local numQuestChoices = GetNumQuestLogChoices(80000, true)
                local money = GetQuestLogRewardMoney()
                local xp = GetQuestLogRewardXP()
                local artifactXP = GetQuestLogRewardArtifactXP()
                local honor = GetQuestLogRewardHonor()
                local _ = (artifactXP > 0) or (numQuestChoices > 0) or (money > 0) or (xp > 0) or (honor > 0) or (numQuestRewards > 0)
                return true
            end

            local function offer_probe()
                local numQuestRewards = GetNumQuestRewards()
                local numQuestChoices = GetNumQuestChoices()
                local money = GetRewardMoney()
                local xp = GetRewardXP()
                local artifactXP = GetRewardArtifactXP()
                local honor = GetRewardHonor()
                local _ = (artifactXP > 0) or (numQuestChoices > 0) or (money > 0) or (xp > 0) or (honor > 0) or (numQuestRewards > 0)
                return true
            end

            return pcall(quest_log_probe), pcall(offer_probe)
            "#,
        )
        .unwrap();
    assert!(quest_log_ok);
    assert!(offer_ok);
}

#[test]
fn criteria_spell_globals_return_selected_quest_spell_data() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.quest_log_entries.entries[0].criteria_spell_id = Some(19750);
        st.quest_log_entries.entries[0].criteria_spell_name = Some("Flash of Light".into());
        st.quest_log_entries.entries[0].criteria_spell_texture =
            Some("Interface\\Icons\\Spell_Holy_FlashHeal".into());
        st.quest_log_entries.entries[0].criteria_spell_finished = true;
    }

    let quest_log_result: (i32, String, String, bool) = env
        .eval(
            r#"
            C_QuestLog.SetSelectedQuest(80000)
            return GetQuestLogCriteriaSpell()
            "#,
        )
        .unwrap();
    assert_eq!(quest_log_result.0, 19750);
    assert_eq!(quest_log_result.1, "Flash of Light");
    assert_eq!(quest_log_result.2, "Interface\\Icons\\Spell_Holy_FlashHeal");
    assert!(quest_log_result.3);

    let criteria_result: (i32, String, String, bool) = env
        .eval(
            r#"
            C_QuestLog.SetSelectedQuest(80000)
            return GetCriteriaSpell()
            "#,
        )
        .unwrap();
    assert_eq!(criteria_result.0, 19750);
    assert_eq!(criteria_result.1, "Flash of Light");
    assert_eq!(criteria_result.2, "Interface\\Icons\\Spell_Holy_FlashHeal");
    assert!(criteria_result.3);
}
