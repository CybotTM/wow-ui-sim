//! Authored RED candidates, NOT executed. Native contract: Wrath build 12340.
//! Server offer/acceptance and choice receipts are explicit simulator policy.
#![cfg(feature = "client-wrath")]

#[path = "common/wrath_turn_in_consumer.rs"]
mod consumer;

use consumer::*;

#[test]
fn original_buttons_request_then_server_policy_accepts_once() {
    let env = seed_env();
    env.env
        .state()
        .borrow_mut()
        .quest_log_entries
        .turn_in_choices
        .insert(998888, 1);
    open_rewards(&env);
    claim(&env);
    accept_response(&env);
    dispatch_queued(&env);
    assert_target_accepted(&env);
    assert_receipts(&env, None);
}

#[test]
fn accepted_response_remains_answered_after_close() {
    let env = seed_env();
    open_rewards(&env);
    claim(&env);
    accept_response(&env);
    dispatch_queued(&env);
    assert_target_accepted(&env);
    assert_blocked_after_close(&env);
}

#[test]
fn acceptance_emits_immediate_player_unit_with_one_token() {
    let env = seed_env();
    open_rewards(&env);
    claim(&env);
    accept_response_with_immediate_unit(&env);
    exec(
        &env,
        r#"
        for _, seen in ipairs(TurnInEvents) do
            if seen.name == 'UNIT_QUEST_LOG_CHANGED' then
                assert(seen.argc == 1, 'one unit token per emission')
            end
        end
    "#,
    );
}

#[test]
fn acceptance_log_update_is_deferred_and_original_listener_refreshes_once() {
    let env = seed_env();
    open_rewards(&env);
    claim(&env);
    accept_response(&env);
    assert_log_update_deferred(&env);
    dispatch_queued(&env);
    exec(
        &env,
        r#"
        assert(TurnInCount('QUEST_LOG_UPDATE') == 1, 'one deferred refresh after flush')
        assert(TurnInLogUpdates == 1, 'original log listener refreshed once')
        for _, seen in ipairs(TurnInEvents) do
            if seen.name == 'QUEST_LOG_UPDATE' then
                assert(seen.argc == 0, 'argumentless deferred signal')
            end
        end
    "#,
    );
}

#[test]
fn wrong_dialog_state_ignores_both_verbs_with_zero_results() {
    let env = seed_env();
    observe(&env);
    force_complete_callback(&env);
    force_reward_callback(&env);
    exec(
        &env,
        "assert(select('#', CompleteQuest()) == 0); assert(select('#', GetQuestReward(999)) == 0)",
    );
    assert_no_server_request(&env, true);
    assert_no_server_request(&env, false);

    let env = seed_env();
    open_progress(&env);
    clear_trace(&env);
    force_reward_callback(&env);
    exec(
        &env,
        "assert(select('#', GetQuestReward(999)) == 0, 'state 2 is not reward state')",
    );
    assert_no_server_request(&env, false);

    let env = seed_env();
    open_rewards(&env);
    force_complete_callback(&env);
    exec(
        &env,
        "assert(select('#', CompleteQuest()) == 0, 'state 3 is not progress state')",
    );
    assert_no_server_request(&env, true);
    claim(&env);
    accept_response(&env);
    dispatch_queued(&env);
    assert_target_accepted(&env);
}

#[test]
fn missing_player_ignores_progress_and_reward_without_consuming_response() {
    for rewards in [false, true] {
        let env = seed_env();
        if rewards {
            supply_choices(&env, false);
        }
        if rewards {
            open_rewards(&env);
        } else {
            open_progress(&env);
            clear_trace(&env);
        }
        if rewards {
            exec(&env, "QuestInfoItem2:Click()");
        }
        env.env
            .state()
            .borrow_mut()
            .quest_log_entries
            .readiness
            .active_player_present = false;
        if rewards {
            force_reward_callback(&env);
            exec(&env, "assert(select('#', GetQuestReward(999)) == 0)");
        } else {
            force_complete_callback(&env);
            exec(&env, "assert(select('#', CompleteQuest()) == 0)");
        }
        assert_no_server_request(&env, !rewards);
        env.env
            .state()
            .borrow_mut()
            .quest_log_entries
            .readiness
            .active_player_present = true;
        if rewards {
            force_reward_callback(&env);
            assert_request_silent(&env);
        } else {
            force_complete_callback(&env);
            assert_request_silent(&env);
            offer_response(&env);
            clear_trace(&env);
            supply_status(&env, true, false);
            claim(&env);
        }
        accept_response(&env);
        dispatch_queued(&env);
        assert_target_accepted(&env);
    }
}

#[test]
fn answered_dialog_ignores_both_verbs_and_close_does_not_reset_latch() {
    for rewards in [false, true] {
        let env = seed_env();
        if rewards {
            supply_choices(&env, false);
        }
        if rewards {
            open_rewards(&env);
        } else {
            open_progress(&env);
            clear_trace(&env);
        }
        // Supplied prior response context, not a request implementation assertion.
        if rewards {
            exec(&env, "QuestInfoItem2:Click()");
        }
        env.env
            .state()
            .borrow_mut()
            .quest_log_entries
            .readiness
            .response_sent = true;
        if rewards {
            force_reward_callback(&env);
        } else {
            force_complete_callback(&env);
        }
        exec(
            &env,
            "assert(select('#', CompleteQuest()) == 0); assert(select('#', GetQuestReward(999)) == 0)",
        );
        assert_no_server_request(&env, !rewards);
        assert_blocked_after_close(&env);
        assert_active(&env);
    }
}

#[test]
fn original_new_dialog_resets_response_latch() {
    let env = seed_env();
    open_progress(&env);
    env.env
        .state()
        .borrow_mut()
        .quest_log_entries
        .readiness
        .response_sent = true;
    exec(&env, "CloseQuest()");
    dispatch_queued(&env);
    // A new real active-title selection performs native SetState, unlike CloseQuest.
    env.env.fire_event("QUEST_GREETING").unwrap();
    exec(&env, "QuestTitleButton2:Click()");
    dispatch_queued(&env);
    clear_trace(&env);
    force_complete_callback(&env);
    assert_request_silent(&env);
    offer_response(&env);
}

#[test]
fn incomplete_and_failed_rows_are_not_native_send_guards() {
    for (complete, failed) in [(false, false), (true, true)] {
        let env = seed_env();
        open_progress(&env);
        supply_status(&env, complete, failed);
        clear_trace(&env);
        force_complete_callback(&env);
        assert_request_silent(&env);
        offer_response(&env);
        clear_trace(&env);
        force_reward_callback(&env);
        assert_request_silent(&env);
        // Only server acceptance uses supplied completed/nonfailed policy.
        // Restore AFTER the native send: a client readiness guard would lose the request.
        supply_status(&env, true, false);
        accept_response(&env);
        dispatch_queued(&env);
        assert_target_accepted(&env);
    }
}

#[test]
fn dialog_current_quest_survives_unrelated_complete_log_selection() {
    let env = seed_env();
    open_progress(&env);
    supply_status(&env, true, false);
    exec(&env, "SelectQuestLogEntry(1)");
    dispatch_queued(&env);
    clear_trace(&env);
    force_complete_callback(&env);
    assert_request_silent(&env);
    offer_response(&env);
    exec(&env, "QuestLogFrame:Show(); SelectQuestLogEntry(1)");
    dispatch_queued(&env);
    // Log OnShow uses the shared QuestInfoFrame. Restore original reward presentation.
    exec(
        &env,
        "QuestFrameRewardPanel:Hide(); QuestFrameRewardPanel:Show()",
    );
    clear_trace(&env);
    force_reward_callback(&env);
    assert_request_silent(&env);
    accept_response(&env);
    dispatch_queued(&env);
    assert_target_accepted(&env);
}

#[test]
fn absent_npc_does_not_guard_native_requests() {
    let env = seed_env();
    open_progress(&env);
    supply_status(&env, true, false);
    {
        let mut sim = env.env.state().borrow_mut();
        sim.gossip.active = false;
        sim.gossip.giver_guid = 0;
    }
    clear_trace(&env);
    force_complete_callback(&env);
    assert_request_silent(&env);
    offer_response(&env);
    clear_trace(&env);
    force_reward_callback(&env);
    assert_request_silent(&env);
    accept_response(&env);
    dispatch_queued(&env);
    assert_target_accepted(&env);
}

#[test]
fn closed_unanswered_progress_still_sends_for_retained_quest() {
    let env = seed_env();
    open_progress(&env);
    supply_status(&env, true, false);
    clear_trace(&env);
    exec(&env, "CloseQuest()");
    dispatch_queued(&env);
    exec(
        &env,
        r#"
        assert(TurnInCount('QUEST_FINISHED') == 1 and not QuestFrame:IsShown())
        for _, seen in ipairs(TurnInEvents) do assert(seen.name == 'QUEST_FINISHED' and seen.argc == 0) end
        SelectQuestLogEntry(1)
    "#,
    );
    clear_trace(&env);
    force_complete_callback(&env);
    assert_request_silent(&env);
    offer_response(&env);
    clear_trace(&env);
    force_reward_callback(&env);
    assert_request_silent(&env);
    accept_response(&env);
    dispatch_queued(&env);
    assert_target_accepted(&env);
    // CloseQuest cleared the concrete nonzero giver; offer/claim cannot resurrect it.
    assert_finish_count(&env, 0);
}

#[test]
fn closed_unanswered_reward_can_claim_but_sent_response_stays_blocked() {
    let env = seed_env();
    supply_choices(&env, false);
    open_rewards(&env);
    exec(&env, "QuestInfoItem2:Click(); CloseQuest()");
    dispatch_queued(&env);
    exec(
        &env,
        r#"
        assert(TurnInCount('QUEST_FINISHED') == 1 and not QuestFrame:IsShown())
        for _, seen in ipairs(TurnInEvents) do assert(seen.name == 'QUEST_FINISHED' and seen.argc == 0) end
        SelectQuestLogEntry(1)
    "#,
    );
    clear_trace(&env);
    force_reward_callback(&env);
    assert_request_silent(&env);
    // While awaiting acceptance, CloseQuest must not clear the sent latch or request.
    exec(&env, "CloseQuest()");
    dispatch_queued(&env);
    clear_trace(&env);
    exec(
        &env,
        "assert(select('#', GetQuestReward(999)) == 0, 'sent guard precedes invalid choice')",
    );
    force_reward_callback(&env);
    assert_request_silent(&env);
    accept_response(&env);
    dispatch_queued(&env);
    assert_target_accepted(&env);
    assert_receipts(&env, Some(2));
    assert_finish_count(&env, 0);
    assert_blocked_after_close(&env);
}

#[test]
fn eligible_choice_omitted_raises_exact_body_without_consuming_response() {
    assert_bad_choice(r#""#);
}

#[test]
fn eligible_choice_nil_raises_exact_body_without_consuming_response() {
    assert_bad_choice(r#"nil"#);
}

#[test]
fn eligible_choice_zero_raises_exact_body_without_consuming_response() {
    assert_bad_choice(r#"0"#);
}

#[test]
fn eligible_choice_negative_raises_exact_body_without_consuming_response() {
    assert_bad_choice(r#"-1"#);
}

#[test]
fn eligible_choice_above_count_raises_exact_body_without_consuming_response() {
    assert_bad_choice(r#"GetNumQuestChoices() + 1"#);
}

#[test]
fn eligible_choice_fraction_truncates_to_first() {
    assert_valid_choice(r#"1.5"#, 1);
    // Fresh eligible call: a first/default-only implementation must fail here.
    assert_valid_choice(r#"2.9"#, 2);
}

#[test]
fn eligible_choice_numeric_string_selects_first() {
    assert_valid_choice(r#""1""#, 1);
    assert_valid_choice(r#""2""#, 2);
}

#[test]
fn eligible_choice_count_plus_half_selects_last() {
    assert_valid_choice(r#"GetNumQuestChoices() + 0.5"#, 2);
}

#[test]
fn answered_guard_precedes_invalid_choice_after_successful_send() {
    let env = seed_env();
    supply_choices(&env, false);
    open_rewards(&env);
    exec(&env, "GetQuestReward(2)");
    assert_request_silent(&env);
    exec(
        &env,
        r#"
        local ok = pcall(function() GetQuestReward(GetNumQuestChoices() + 1) end)
        assert(ok, 'answered guard precedes invalid-choice validation')
    "#,
    );
    assert_request_silent(&env);
    exec(
        &env,
        "assert(A_Admin.TurnInSelectedQuest(), 'answered guard preserves an acceptable first claim after invalid repeat')",
    );
    dispatch_queued(&env);
    assert_target_accepted(&env);
    assert_eq!(
        env.env
            .state()
            .borrow()
            .quest_log_entries
            .turn_in_choices
            .get(&TARGET),
        Some(&2),
        "answered guard preserves the first successful choice despite invalid repeat"
    );
    assert_receipts(&env, Some(2));
}

#[test]
fn original_unselected_choice_guard_is_separate_from_native_validation() {
    let env = seed_env();
    supply_choices(&env, false);
    open_rewards(&env);
    exec(
        &env,
        "assert(GetNumQuestChoices() == 2 and not QuestInfoItemHighlight:IsShown(), 'no displayed choice selected')",
    );
    force_reward_callback(&env);
    dispatch_queued(&env);
    assert_active(&env);
    exec(
        &env,
        r#"
        assert(not A_Admin.TurnInSelectedQuest(), 'original unselected UI never sent a claim')
        assert(TurnInCount('QUEST_FINISHED') == 0 and TurnInCount('QUEST_TURNED_IN') == 0)
        assert(TurnInCount('UNIT_QUEST_LOG_CHANGED') == 0 and TurnInCount('QUEST_LOG_UPDATE') == 0)
        assert(QuestFrameRewardPanel:IsShown(), 'UI error feedback does not close rewards')
    "#,
    );
    assert_receipts(&env, None);
}

#[test]
fn original_choice_click_tracks_supplied_order_and_records_only_on_acceptance() {
    for (reversed, index) in [(false, 2), (true, 1)] {
        let env = seed_env();
        supply_choices(&env, reversed);
        open_rewards(&env);
        exec(
            &env,
            "assert(GetNumQuestChoices() == 2 and GetNumQuestRewards() == 0); assert(QuestInfoRewardsFrame:IsShown())",
        );
        let names = if reversed {
            ["Patrol recovery seal", "Northern recovery satchel"]
        } else {
            ["Northern recovery satchel", "Patrol recovery seal"]
        };
        for (offset, name) in names.iter().enumerate() {
            let item = offset + 1;
            let (count, quality, usable) = if *name == "Patrol recovery seal" {
                (3, 2, false)
            } else {
                (1, 3, true)
            };
            exec(
                &env,
                &format!(
                    r#"
                assert(QuestInfoItem{item}:IsShown() and QuestInfoItem{item}:GetID() == {item})
                assert(QuestInfoItem{item}Name:GetText() == '{name}')
                local name, texture, count, quality, usable = GetQuestItemInfo('choice', {item})
                assert(name == '{name}' and count == {count} and quality == {quality} and usable == {usable}, 'concrete supplied tuple')
                assert(QuestInfoItem{item}Count:GetText() == '{count}' or ({count} == 1 and not QuestInfoItem{item}Count:IsShown()), 'original count display')
            "#
                ),
            );
        }
        exec(
            &env,
            &format!(
                "QuestInfoItem{index}:Click(); assert(QuestInfoItemHighlight:IsShown(), 'original selection highlighted')"
            ),
        );
        assert_receipts(&env, None);
        claim(&env);
        assert_receipts(&env, None);
        accept_response(&env);
        dispatch_queued(&env);
        assert_target_accepted(&env);
        assert_receipts(&env, Some(index));
    }
}

#[test]
fn simulator_no_choice_original_button_accepts_without_choice_receipt() {
    // Simulator policy via the original UI's zero-choice path, not evidence for
    // native omitted/nil/zero normalization or successful-call result arity.
    let env = seed_env();
    open_rewards(&env);
    exec(
        &env,
        "assert(GetNumQuestChoices() == 0 and GetNumQuestRewards() == 0)",
    );
    claim(&env);
    accept_response(&env);
    dispatch_queued(&env);
    assert_target_accepted(&env);
    assert!(
        env.env
            .state()
            .borrow()
            .quest_log_entries
            .turn_in_choices
            .is_empty(),
        "SIMULATOR POLICY: no-choice original button creates no choice receipt"
    );
}

#[test]
fn completion_finish_depends_only_on_next_quest_and_concrete_giver() {
    for (next, giver, gossip_active, finishes) in [
        (0, NPC_GUID, true, 1),
        (990399, NPC_GUID, true, 0),
        (0, 0, true, 0),
        (990399, 0, true, 0),
        (0, 0, false, 0),
    ] {
        let env = seed_env();
        open_rewards(&env);
        {
            let mut sim = env.env.state().borrow_mut();
            sim.quest_log_entries.readiness.next_quest_id = next;
            sim.gossip.giver_guid = giver;
            sim.gossip.active = gossip_active;
        }
        // Test both active-gossip GUID-zero and absent-NPC completion contexts.
        claim(&env);
        accept_response(&env);
        dispatch_queued(&env);
        assert_target_accepted(&env);
        assert_finish_count(&env, finishes);
        exec(
            &env,
            &format!("assert(QuestFrame:IsShown() == {})", finishes == 0),
        );
    }
}

#[test]
fn two_accepted_claims_emit_immediate_units_and_one_deferred_log_refresh() {
    let env = seed_env();
    open_rewards(&env);
    env.env
        .state()
        .borrow_mut()
        .quest_log_entries
        .readiness
        .next_quest_id = 990399;
    claim(&env);
    accept_response_with_immediate_unit(&env);
    assert_log_update_deferred(&env);
    // Supply a greeting, then select the surviving complete control through the
    // original active-title callback. Production SetState must reset the latch.
    // Never drain the first acceptance's queued log refresh during this transition.
    env.env.fire_event("QUEST_GREETING").unwrap();
    assert_errors(&env);
    exec(
        &env,
        r#"
        assert(QuestTitleButton1:IsShown())
        assert(string.find(QuestTitleButton1:GetText(), 'Wrath unrelated patrol', 1, true))
        QuestTitleButton1:Click()
        assert(QuestFrameRewardPanel:IsShown())
        assert(QuestInfoTitleHeader:GetText() == 'Wrath unrelated patrol')
        QuestFrameCompleteQuestButton:GetScript('OnClick')(QuestFrameCompleteQuestButton)
        assert(TurnInCount('QUEST_LOG_UPDATE') == 0, 'no intermediate flush')
    "#,
    );
    accept_response_with_immediate_unit(&env);
    assert_log_update_deferred(&env);
    dispatch_queued(&env);
    assert_trace(&env, 0, 2);
    exec(
        &env,
        "assert(select(2, GetNumQuestLogEntries()) == 0); assert(IsQuestFlaggedCompleted(990301) and IsQuestFlaggedCompleted(991300))",
    );
}

#[test]
fn simulator_completion_without_native_claim_is_inert() {
    let env = seed_env();
    observe(&env);
    supply_status(&env, true, false);
    // No original dialog/readers/native verb/offer prerequisite: select a complete
    // log row without making any claim, then exercise only the acceptance policy.
    exec(&env, "SelectQuestLogEntry(2)");
    dispatch_queued(&env);
    clear_trace(&env);
    assert_no_server_request(&env, false);
}

#[test]
fn no_choice_ordinary_zero_index_inputs_send_without_error() {
    for argument in ["", "nil", "0", "-1", "1.5", "\"1\"", "0.5"] {
        let env = seed_env();
        open_rewards(&env);
        exec(&env, &format!("GetQuestReward({argument})"));
        assert_request_silent(&env);
        accept_response(&env);
        dispatch_queued(&env);
        assert_target_accepted(&env);
        assert!(
            env.env
                .state()
                .borrow()
                .quest_log_entries
                .turn_in_choices
                .is_empty()
        );
    }
}

#[test]
fn no_choice_positive_internal_index_does_not_raise() {
    let env = seed_env();
    open_rewards(&env);
    exec(&env, "GetQuestReward(3)");
    assert_request_silent(&env);
    // Native sends index 2, but no public send-index reader exists here. Do not
    // infer server acceptance or raw wire index from a no-choice receipt map.
    // Exact sent index remains a recorded observability gap.
}
