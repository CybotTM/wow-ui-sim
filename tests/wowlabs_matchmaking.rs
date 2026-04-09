use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn wowlabs_matchmaking_party_and_queue_state_are_mutable() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local members = C_WoWLabsMatchmaking.GetCurrentParty()
            if #members ~= 2 then
                return "expected_two_members"
            end
            if not C_WoWLabsMatchmaking.IsPartyLeader() then
                return "expected_local_leader"
            end
            if C_WoWLabsMatchmaking.IsAloneInWoWLabsParty() then
                return "expected_non_solo_party"
            end
            if not C_WoWLabsMatchmaking.CanEnterMatchmaking() then
                return "expected_queueable_party"
            end
            if C_WoWLabsMatchmaking.SetPartyPlaylistEntry(Enum.PartyPlaylistEntry.SoloGameMode) then
                return "solo_playlist_should_fail_for_party_of_two"
            end
            if not C_WoWLabsMatchmaking.SetPartyPlaylistEntry(Enum.PartyPlaylistEntry.TrioGameMode) then
                return "trio_playlist_should_succeed"
            end

            C_WoWLabsMatchmaking.SetPlayerReady(true)
            local autoQueueBefore, queueTypeBefore = C_WoWLabsMatchmaking.GetAutoQueueOnLogout()
            C_WoWLabsMatchmaking.SetAutoQueueOnLogout(true, Enum.PartyPlaylistEntry.TrainingGameMode)
            local autoQueueAfter, queueTypeAfter = C_WoWLabsMatchmaking.GetAutoQueueOnLogout()

            if autoQueueBefore then
                return "auto_queue_should_start_disabled"
            end
            if not autoQueueAfter then
                return "auto_queue_should_enable"
            end
            if queueTypeAfter ~= Enum.PartyPlaylistEntry.TrainingGameMode then
                return "auto_queue_should_store_queue_type"
            end
            if not C_WoWLabsMatchmaking.IsPlayerReady() then
                return "player_ready_should_round_trip"
            end
            if not C_WoWLabsMatchmaking.IsFindingMatch() then
                return "leader_ready_should_start_queue"
            end
            if C_WoWLabsMatchmaking.GetInQueueTimeStart() <= 0 then
                return "queue_start_time_should_be_seeded"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok", "WoWLabs matchmaking state should round-trip");
}

#[test]
fn wowlabs_matchmaking_invites_and_members_are_state_backed() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_WoWLabsMatchmaking.GetNumPartyInvites() ~= 1 then
                return "expected_one_invite"
            end

            local inviterName, inviteID = C_WoWLabsMatchmaking.GetPartyInviteByIndex(0)
            if inviterName ~= "PartyPal" then
                return "invite_name_mismatch"
            end
            if inviteID ~= "WoWLabsInvite-1" then
                return "invite_id_mismatch"
            end
            if C_WoWLabsMatchmaking.AcceptPartyInvite("missing") then
                return "missing_invite_should_not_accept"
            end
            if not C_WoWLabsMatchmaking.AcceptPartyInvite(inviteID) then
                return "accept_invite_should_succeed"
            end
            if C_WoWLabsMatchmaking.GetNumPartyInvites() ~= 0 then
                return "accepted_invite_should_be_removed"
            end
            if C_WoWLabsMatchmaking.IsPartyLeader() then
                return "accepted_invite_should_make_local_player_non_leader"
            end

            local party = C_WoWLabsMatchmaking.GetCurrentParty()
            if #party ~= 2 then
                return "accepted_invite_should_rebuild_party"
            end

            local foundLocalPlayer = false
            local foundLeader = false
            for _, member in ipairs(party) do
                foundLocalPlayer = foundLocalPlayer or member.isLocalPlayer == true
                foundLeader = foundLeader or member.isPartyLeader == true
            end
            if not foundLocalPlayer or not foundLeader then
                return "accepted_party_should_include_local_player_and_leader"
            end

            if not C_WoWLabsMatchmaking.LeaveParty() then
                return "leave_party_should_succeed"
            end
            if C_WoWLabsMatchmaking.GetPartySize() ~= 1 then
                return "leave_party_should_reset_to_solo"
            end
            if not C_WoWLabsMatchmaking.IsAloneInWoWLabsParty() then
                return "leave_party_should_make_party_solo"
            end
            if not C_WoWLabsMatchmaking.IsPartyLeader() then
                return "solo_party_should_restore_local_leadership"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok", "WoWLabs invites should mutate party state");
}

#[test]
fn wowlabs_area_data_manager_selection_and_circle_queries_are_state_backed() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if not C_WowLabsDataManager.IsInPrematch() then
                return "prematch_should_start_enabled"
            end

            local areas = C_WowLabsDataManager.GetWoWLabsAreaInfo()
            local queried = C_WowLabsDataManager.QueryWoWLabsAreaInfo()
            if #areas ~= 3 or #queried ~= 3 then
                return "expected_three_seeded_areas"
            end

            local targetAreaID = areas[1].wowLabsAreaID
            if not C_WowLabsDataManager.SelectWoWLabsArea(targetAreaID) then
                return "area_selection_should_succeed"
            end
            if C_WowLabsDataManager.QuerySelectedWoWLabsArea() ~= targetAreaID then
                return "selected_area_should_round_trip"
            end
            if C_WowLabsDataManager.GetConfirmedWoWLabsArea() ~= targetAreaID then
                return "confirmed_area_should_track_selection"
            end

            local startLerpTime, timeToLerp, outerPosition, innerPosition, baseRadius, outerScale, innerScale, predictionPosition, predictionScale, initialBaseSize = C_WowLabsDataManager.PushCircleInfoToLua()
            if type(outerPosition) ~= "table" or outerPosition.x == nil or outerPosition.y == nil then
                return "outer_circle_position_should_be_a_point"
            end
            if type(predictionPosition) ~= "table" or predictionPosition.x == nil or predictionPosition.y == nil then
                return "prediction_circle_position_should_be_a_point"
            end
            if timeToLerp <= 0 or baseRadius <= 0 or outerScale <= 0 or innerScale <= 0 or predictionScale <= 0 or initialBaseSize <= 0 then
                return "circle_values_should_be_positive"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok", "WoWLabs area state should round-trip");
}
