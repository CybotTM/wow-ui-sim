use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

#[test]
fn get_lfg_roles_defaults_to_damage() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local leader, tank, healer, dps = GetLFGRoles()
            if type(leader) ~= "boolean" then return "leader_type=" .. type(leader) end
            if type(tank) ~= "boolean" then return "tank_type=" .. type(tank) end
            if type(healer) ~= "boolean" then return "healer_type=" .. type(healer) end
            if type(dps) ~= "boolean" then return "dps_type=" .. type(dps) end
            if leader ~= false then return "leader=" .. tostring(leader) end
            if tank ~= false then return "tank=" .. tostring(tank) end
            if healer ~= false then return "healer=" .. tostring(healer) end
            if dps ~= true then return "dps=" .. tostring(dps) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFGRoles default tuple: {result}");
}

#[test]
fn set_lfg_roles_persists_all_flags() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            SetLFGRoles(true, true, false, false)
            local leader, tank, healer, dps = GetLFGRoles()
            if leader ~= true then return "leader=" .. tostring(leader) end
            if tank ~= true then return "tank=" .. tostring(tank) end
            if healer ~= false then return "healer=" .. tostring(healer) end
            if dps ~= false then return "dps=" .. tostring(dps) end

            SetLFGRoles(false, false, true, true)
            leader, tank, healer, dps = GetLFGRoles()
            if leader ~= false then return "leader2=" .. tostring(leader) end
            if tank ~= false then return "tank2=" .. tostring(tank) end
            if healer ~= true then return "healer2=" .. tostring(healer) end
            if dps ~= true then return "dps2=" .. tostring(dps) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "SetLFGRoles persistence: {result}");
}

#[test]
fn get_lfd_choice_enabled_state_defaults_specific_joinable_dungeons() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local enabled = GetLFDChoiceEnabledState()
            if type(enabled) ~= "table" then return "type=" .. type(enabled) end
            for _, id in ipairs({1201, 1203, 1205, 1206, 1208}) do
                if enabled[id] ~= true then
                    return "unchecked=" .. id .. ":" .. tostring(enabled[id])
                end
            end
            if enabled[1202] then return "follower=" .. tostring(enabled[1202]) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "GetLFDChoiceEnabledState default selections: {result}"
    );
}

#[test]
fn set_lfg_dungeon_enabled_persists_specific_choice_state() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local enabled = GetLFDChoiceEnabledState()
            if enabled[1203] ~= true then return "initial=" .. tostring(enabled[1203]) end

            SetLFGDungeonEnabled(1203, false)
            enabled = GetLFDChoiceEnabledState()
            if enabled[1203] then return "disabled=" .. tostring(enabled[1203]) end

            SetLFGDungeonEnabled(1203, true)
            enabled = GetLFDChoiceEnabledState()
            if enabled[1203] ~= true then return "restored=" .. tostring(enabled[1203]) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "SetLFGDungeonEnabled persistence: {result}");
}

#[test]
fn get_lfd_choice_order_returns_ids() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local order = GetLFDChoiceOrder()
            if type(order) ~= "table" then return "type=" .. type(order) end
            if #order < 8 then return "count=" .. #order end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFDChoiceOrder: {result}");
}

#[test]
fn get_lfg_dungeon_info_seeded_id() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local name = GetLFGDungeonInfo(1203)
            if name == nil then return "nil" end
            if type(name) ~= "string" then return "type=" .. type(name) end
            if name == "" then return "empty" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFGDungeonInfo(1203): {result}");
}

#[test]
fn get_lfg_dungeon_info_returns_21_values() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local t = {GetLFGDungeonInfo(1203)}
            if #t ~= 21 then return "count=" .. #t end
            -- t[1] = name, t[2] = typeID, t[13] = maxPlayers
            if type(t[1]) ~= "string" then return "name_type=" .. type(t[1]) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFGDungeonInfo 21 values: {result}");
}

#[test]
fn get_lfg_dungeon_info_min_players_slot_is_nil_for_normal_dungeons() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local LFG_RETURN_VALUES = {
                bonusRepAmount = 16,
                minPlayers = 17,
                isTimewalker = 18,
                mapName = 19,
                minGear = 20,
            }
            local t = {GetLFGDungeonInfo(1203)}
            if type(t[LFG_RETURN_VALUES.bonusRepAmount]) ~= "number" then
                return "bonusRepAmount_type=" .. type(t[LFG_RETURN_VALUES.bonusRepAmount])
            end
            if t[LFG_RETURN_VALUES.minPlayers] ~= nil then
                return "minPlayers_type=" .. type(t[LFG_RETURN_VALUES.minPlayers])
            end
            if type(t[LFG_RETURN_VALUES.isTimewalker]) ~= "boolean" then
                return "isTimewalker_type=" .. type(t[LFG_RETURN_VALUES.isTimewalker])
            end
            if type(t[LFG_RETURN_VALUES.mapName]) ~= "string" then
                return "mapName_type=" .. type(t[LFG_RETURN_VALUES.mapName])
            end
            if type(t[LFG_RETURN_VALUES.minGear]) ~= "number" then
                return "minGear_type=" .. type(t[LFG_RETURN_VALUES.minGear])
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFGDungeonInfo return slots: {result}");
}

#[test]
fn lfg_required_group_size_allows_full_party_for_normal_specific_dungeon() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.SetPartySize(4)
            local LFG_RETURN_VALUES = { minPlayers = 17 }
            LFGEnabledList = { [1203] = true }
            LFGLockList = {}

            function LFGIsIDHeader(id)
                return id < 0
            end

            function LFG_HasRequiredGroupSize(category, joinType, dungeonList, hiddenByCollapseList)
                local numGroupMembers, numRequiredPlayers
                if IsInGroup() then
                    numGroupMembers = GetNumGroupMembers()
                else
                    numGroupMembers = 1
                end
                if joinType == "specific" or joinType == "follower" then
                    for _, queueID in pairs(dungeonList) do
                        if not LFGIsIDHeader(queueID) and LFGEnabledList[queueID] and not LFGLockList[queueID] then
                            numRequiredPlayers = select(LFG_RETURN_VALUES.minPlayers, GetLFGDungeonInfo(queueID))
                            if numRequiredPlayers and numRequiredPlayers ~= numGroupMembers then
                                return false, numRequiredPlayers
                            end
                        end
                    end
                end
                return true
            end

            local ok, required = LFG_HasRequiredGroupSize(1, "specific", { 1203 }, {})
            if not ok then return "required=" .. tostring(required) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "normal specific LFD should not require an exact party size: {result}"
    );
}

#[test]
fn lfg_join_verbs_complete_specific_dungeon_queue_path() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if type(ClearAllLFGDungeons) ~= "function" then
                return "missing_clear=" .. type(ClearAllLFGDungeons)
            end
            if type(SetLFGDungeon) ~= "function" then
                return "missing_set=" .. type(SetLFGDungeon)
            end
            if type(JoinLFG) ~= "function" then
                return "missing_join=" .. type(JoinLFG)
            end

            ClearAllLFGDungeons(LE_LFG_CATEGORY_LFD)
            SetLFGDungeon(LE_LFG_CATEGORY_LFD, 1203)
            JoinLFG(LE_LFG_CATEGORY_LFD)

            local mode, submode = GetLFGMode(LE_LFG_CATEGORY_LFD)
            if mode ~= "queued" then return "mode=" .. tostring(mode) end
            if submode ~= nil then return "submode=" .. tostring(submode) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "LFG join verbs: {result}");
}

#[test]
fn get_lfg_info_server_reports_queued_after_join_lfg() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if type(GetLFGInfoServer) ~= "function" then
                return "missing=" .. type(GetLFGInfoServer)
            end
            ClearAllLFGDungeons(LE_LFG_CATEGORY_LFD)
            local _, _, queued = GetLFGInfoServer(LE_LFG_CATEGORY_LFD)
            if queued then return "queued_before" end

            JoinLFG(LE_LFG_CATEGORY_LFD)
            local inParty, joined, queued, noPartialClear, achievements, lfgComment, slotCount =
                GetLFGInfoServer(LE_LFG_CATEGORY_LFD)
            if inParty ~= false then return "inParty=" .. tostring(inParty) end
            if joined ~= false then return "joined=" .. tostring(joined) end
            if queued ~= true then return "queued=" .. tostring(queued) end
            if noPartialClear ~= false then return "noPartialClear=" .. tostring(noPartialClear) end
            if achievements ~= nil then return "achievements=" .. tostring(achievements) end
            if lfgComment ~= "" then return "comment=" .. tostring(lfgComment) end
            if slotCount ~= 0 then return "slotCount=" .. tostring(slotCount) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFGInfoServer queued state: {result}");
}

#[test]
fn leave_lfg_clears_queued_lfd_state() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if type(LeaveLFG) ~= "function" then
                return "missing_leave=" .. type(LeaveLFG)
            end

            ClearAllLFGDungeons(LE_LFG_CATEGORY_LFD)
            SetLFGDungeon(LE_LFG_CATEGORY_LFD, 1203)
            JoinLFG(LE_LFG_CATEGORY_LFD)
            LeaveLFG(LE_LFG_CATEGORY_LFD)

            local mode = GetLFGMode(LE_LFG_CATEGORY_LFD)
            if mode ~= nil then return "mode=" .. tostring(mode) end

            local _, _, queued = GetLFGInfoServer(LE_LFG_CATEGORY_LFD)
            if queued ~= false then return "queued=" .. tostring(queued) end

            local queuedList = GetLFGQueuedList(LE_LFG_CATEGORY_LFD)
            if next(queuedList) ~= nil then return "queued_list_not_empty" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "LeaveLFG lifecycle: {result}");
}

#[test]
fn get_lfg_queued_list_reuses_and_clears_passed_table() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            ClearAllLFGDungeons(LE_LFG_CATEGORY_LFD)
            SetLFGDungeon(LE_LFG_CATEGORY_LFD, 1203)

            local queuedList = { stale = true }
            local returned = GetLFGQueuedList(LE_LFG_CATEGORY_LFD, queuedList)
            if returned ~= queuedList then return "different_table" end
            if queuedList.stale ~= nil then return "stale=" .. tostring(queuedList.stale) end
            if queuedList[1203] ~= true then return "queued=" .. tostring(queuedList[1203]) end

            ClearAllLFGDungeons(LE_LFG_CATEGORY_LFD)
            GetLFGQueuedList(LE_LFG_CATEGORY_LFD, queuedList)
            if next(queuedList) ~= nil then return "not_cleared" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFGQueuedList table reuse: {result}");
}

#[test]
fn leave_lfg_without_category_clears_all_lfg_state() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            ClearAllLFGDungeons(LE_LFG_CATEGORY_LFD)
            SetLFGDungeon(LE_LFG_CATEGORY_LFD, 1203)
            JoinLFG(LE_LFG_CATEGORY_LFD)
            LeaveLFG()

            local mode = GetLFGMode(LE_LFG_CATEGORY_LFD)
            if mode ~= nil then return "mode=" .. tostring(mode) end

            local _, _, queued = GetLFGInfoServer(LE_LFG_CATEGORY_LFD)
            if queued ~= false then return "queued=" .. tostring(queued) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "LeaveLFG without category: {result}");
}

#[test]
fn admin_lfg_queue_pop_delay_defaults_persists_and_clamps_invalid_values() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if type(A_Admin.GetLfgQueuePopDelay) ~= "function" then
                return "missing_get=" .. type(A_Admin.GetLfgQueuePopDelay)
            end
            if type(A_Admin.SetLfgQueuePopDelay) ~= "function" then
                return "missing_set=" .. type(A_Admin.SetLfgQueuePopDelay)
            end

            local defaultDelay = A_Admin.GetLfgQueuePopDelay()
            if defaultDelay ~= 5 then return "default=" .. tostring(defaultDelay) end

            A_Admin.SetLfgQueuePopDelay(1.25)
            local positiveDelay = A_Admin.GetLfgQueuePopDelay()
            if positiveDelay ~= 1.25 then return "positive=" .. tostring(positiveDelay) end

            A_Admin.SetLfgQueuePopDelay(-8)
            local clampedDelay = A_Admin.GetLfgQueuePopDelay()
            if clampedDelay ~= 0 then return "clamped=" .. tostring(clampedDelay) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "A_Admin LFG delay config: {result}");
}

#[test]
fn zero_delay_lfg_queue_pop_transitions_to_active_proposal() {
    let env = env();
    env.exec(
        r#"
        A_Admin.SetLfgQueuePopDelay(0)
        ClearAllLFGDungeons(LE_LFG_CATEGORY_LFD)
        SetLFGDungeon(LE_LFG_CATEGORY_LFD, 1203)
        JoinLFG(LE_LFG_CATEGORY_LFD)
        "#,
    )
    .unwrap();

    let fired = env.process_timers().unwrap();
    assert_eq!(fired, 1, "zero-delay LFG pop should fire exactly one timer");

    let result: String = env
        .eval(
            r#"
            local mode, submode = GetLFGMode(LE_LFG_CATEGORY_LFD)
            if mode ~= "proposal" then return "mode=" .. tostring(mode) end
            if submode ~= "unaccepted" then return "submode=" .. tostring(submode) end

            local proposalExists, id, typeID, subtypeID, name, backgroundTexture, role,
                hasResponded, totalEncounters, completedEncounters, numMembers, isLeader,
                isHoliday, proposalCategory, isSilent = GetLFGProposal()
            if proposalExists ~= true then return "proposalExists=" .. tostring(proposalExists) end
            if id ~= 1203 then return "id=" .. tostring(id) end
            if typeID ~= 2 then return "typeID=" .. tostring(typeID) end
            if subtypeID ~= 2 then return "subtypeID=" .. tostring(subtypeID) end
            if type(name) ~= "string" or name == "" then return "name=" .. tostring(name) end
            if type(backgroundTexture) ~= "string" or backgroundTexture == "" then return "backgroundTexture=" .. tostring(backgroundTexture) end
            if role ~= "DAMAGER" then return "role=" .. tostring(role) end
            if hasResponded ~= false then return "hasResponded=" .. tostring(hasResponded) end
            if totalEncounters ~= 3 then return "totalEncounters=" .. tostring(totalEncounters) end
            if completedEncounters ~= 0 then return "completedEncounters=" .. tostring(completedEncounters) end
            if numMembers ~= 5 then return "numMembers=" .. tostring(numMembers) end
            if isLeader ~= true then return "isLeader=" .. tostring(isLeader) end
            if isHoliday ~= false then return "isHoliday=" .. tostring(isHoliday) end
            if proposalCategory ~= LE_LFG_CATEGORY_LFD then return "proposalCategory=" .. tostring(proposalCategory) end
            if isSilent ~= false then return "isSilent=" .. tostring(isSilent) end

            local memberLeader, memberRole, memberLevel, responded, accepted, memberName, memberClass =
                GetLFGProposalMember(1)
            if memberLeader ~= true then return "memberLeader=" .. tostring(memberLeader) end
            if memberRole ~= "DAMAGER" then return "memberRole=" .. tostring(memberRole) end
            if memberLevel ~= UnitLevel("player") then return "memberLevel=" .. tostring(memberLevel) end
            if responded ~= false then return "responded=" .. tostring(responded) end
            if accepted ~= false then return "accepted=" .. tostring(accepted) end
            if type(memberName) ~= "string" or memberName == "" then return "memberName=" .. tostring(memberName) end
            if type(memberClass) ~= "string" or memberClass == "" then return "memberClass=" .. tostring(memberClass) end

            local bossName, texture, isKilled = GetLFGProposalEncounter(1)
            if type(bossName) ~= "string" or bossName == "" then return "bossName=" .. tostring(bossName) end
            if texture ~= "" then return "texture=" .. tostring(texture) end
            if isKilled ~= false then return "isKilled=" .. tostring(isKilled) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "LFG proposal transition: {result}");
}

#[test]
fn leave_lfg_other_category_does_not_hide_active_proposal() {
    let env = env();
    env.exec(
        r#"
        A_Admin.SetLfgQueuePopDelay(0)
        ClearAllLFGDungeons(LE_LFG_CATEGORY_LFD)
        SetLFGDungeon(LE_LFG_CATEGORY_LFD, 1203)
        JoinLFG(LE_LFG_CATEGORY_LFD)
        "#,
    )
    .unwrap();
    assert_eq!(env.process_timers().unwrap(), 1);

    let result: String = env
        .eval(
            r#"
            LeaveLFG(LE_LFG_CATEGORY_RF)
            local proposalExists, id = GetLFGProposal()
            if proposalExists ~= true then return "proposalExists=" .. tostring(proposalExists) end
            if id ~= 1203 then return "id=" .. tostring(id) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "cross-category LeaveLFG: {result}");
}

#[test]
fn lfg_proposal_response_globals_clear_active_proposal() {
    let env = env();
    env.exec(
        r#"
        A_Admin.SetLfgQueuePopDelay(0)
        ClearAllLFGDungeons(LE_LFG_CATEGORY_LFD)
        SetLFGDungeon(LE_LFG_CATEGORY_LFD, 1203)
        JoinLFG(LE_LFG_CATEGORY_LFD)
        "#,
    )
    .unwrap();
    assert_eq!(env.process_timers().unwrap(), 1);

    let result: String = env
        .eval(
            r#"
            if type(AcceptProposal) ~= "function" then
                return "missing_accept=" .. type(AcceptProposal)
            end
            if type(RejectProposal) ~= "function" then
                return "missing_reject=" .. type(RejectProposal)
            end

            AcceptProposal()
            local proposalExists = GetLFGProposal()
            if proposalExists ~= false then return "proposal_after_accept=" .. tostring(proposalExists) end

            SetLFGDungeon(LE_LFG_CATEGORY_LFD, 1203)
            JoinLFG(LE_LFG_CATEGORY_LFD)
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "AcceptProposal clears proposal: {result}");

    assert_eq!(env.process_timers().unwrap(), 1);
    let result: String = env
        .eval(
            r#"
            RejectProposal()
            local proposalExists = GetLFGProposal()
            if proposalExists ~= false then return "proposal_after_reject=" .. tostring(proposalExists) end
            local mode = GetLFGMode(LE_LFG_CATEGORY_LFD)
            if mode ~= nil then return "mode_after_reject=" .. tostring(mode) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "RejectProposal clears proposal: {result}");
}

#[test]
fn lfg_queue_stats_and_queued_list_track_selected_dungeons() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            ClearAllLFGDungeons(LE_LFG_CATEGORY_LFD)
            SetLFGDungeon(LE_LFG_CATEGORY_LFD, 1203)
            JoinLFG(LE_LFG_CATEGORY_LFD)

            local queued = GetLFGQueuedList(LE_LFG_CATEGORY_LFD)
            if queued[1203] ~= true then return "missing_queued_id" end

            local activeID = select(18, GetLFGQueueStats(LE_LFG_CATEGORY_LFD))
            if activeID ~= 1203 then return "activeID=" .. tostring(activeID) end

            local hasData, leaderNeeds, tankNeeds, healerNeeds, dpsNeeds, totalTanks, totalHealers,
                totalDPS, instanceType, instanceSubType, instanceName, averageWait, tankWait,
                healerWait, damageWait, myWait, queuedTime = GetLFGQueueStats(LE_LFG_CATEGORY_LFD, 1203)
            if hasData ~= true then return "hasData=" .. tostring(hasData) end
            if leaderNeeds ~= 0 then return "leaderNeeds=" .. tostring(leaderNeeds) end
            if tankNeeds ~= 0 then return "tankNeeds=" .. tostring(tankNeeds) end
            if healerNeeds ~= 0 then return "healerNeeds=" .. tostring(healerNeeds) end
            if dpsNeeds ~= 0 then return "dpsNeeds=" .. tostring(dpsNeeds) end
            if totalTanks ~= 1 then return "totalTanks=" .. tostring(totalTanks) end
            if totalHealers ~= 1 then return "totalHealers=" .. tostring(totalHealers) end
            if totalDPS ~= 3 then return "totalDPS=" .. tostring(totalDPS) end
            if instanceType ~= 2 then return "instanceType=" .. tostring(instanceType) end
            if instanceSubType ~= 2 then return "instanceSubType=" .. tostring(instanceSubType) end
            if type(instanceName) ~= "string" or instanceName == "" then return "instanceName=" .. tostring(instanceName) end
            if averageWait ~= 0 or tankWait ~= 0 or healerWait ~= 0 or damageWait ~= 0 or myWait ~= 0 or queuedTime ~= 0 then
                return "waits"
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "LFG queued list/stats: {result}");
}

#[test]
fn lfg_join_dungeon_specific_path_reaches_queued_mode() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.SetPartySize(4)
            local LFG_RETURN_VALUES = { minPlayers = 17 }
            LFGEnabledList = { [1203] = true }
            LFGLockList = {}

            function LFGIsIDHeader(id)
                return id < 0
            end

            function LFG_QueueForInstanceIfEnabled(category, queueID)
                if not LFGIsIDHeader(queueID) and LFGEnabledList[queueID] and not LFGLockList[queueID] then
                    SetLFGDungeon(category, queueID)
                    return true
                end
                return false
            end

            function LFG_HasRequiredGroupSize(category, joinType, dungeonList, hiddenByCollapseList)
                local numGroupMembers, numRequiredPlayers
                if IsInGroup() then
                    numGroupMembers = GetNumGroupMembers()
                else
                    numGroupMembers = 1
                end
                for _, queueID in pairs(dungeonList) do
                    if not LFGIsIDHeader(queueID) and LFGEnabledList[queueID] and not LFGLockList[queueID] then
                        numRequiredPlayers = select(LFG_RETURN_VALUES.minPlayers, GetLFGDungeonInfo(queueID))
                        if numRequiredPlayers and numRequiredPlayers ~= numGroupMembers then
                            return false, numRequiredPlayers
                        end
                    end
                end
                for _, queueID in pairs(hiddenByCollapseList) do
                    if not LFGIsIDHeader(queueID) and LFGEnabledList[queueID] and not LFGLockList[queueID] then
                        numRequiredPlayers = select(LFG_RETURN_VALUES.minPlayers, GetLFGDungeonInfo(queueID))
                        if numRequiredPlayers and numRequiredPlayers ~= numGroupMembers then
                            return false, numRequiredPlayers
                        end
                    end
                end
                return true
            end

            function LFG_JoinDungeon(category, joinType, dungeonList, hiddenByCollapseList)
                local hasReqGroupSize, requiredGroupSize = LFG_HasRequiredGroupSize(category, joinType, dungeonList, hiddenByCollapseList)
                if not hasReqGroupSize then
                    return "required=" .. tostring(requiredGroupSize)
                end
                ClearAllLFGDungeons(category)
                for _, queueID in pairs(dungeonList) do
                    LFG_QueueForInstanceIfEnabled(category, queueID)
                end
                for _, queueID in pairs(hiddenByCollapseList) do
                    LFG_QueueForInstanceIfEnabled(category, queueID)
                end
                JoinLFG(category)
                return "joined"
            end

            local joined = LFG_JoinDungeon(LE_LFG_CATEGORY_LFD, "specific", { 1203 }, {})
            if joined ~= "joined" then return joined end
            local mode = GetLFGMode(LE_LFG_CATEGORY_LFD)
            if mode ~= "queued" then return "mode=" .. tostring(mode) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "specific LFG join path: {result}");
}

#[test]
fn get_lfg_dungeon_info_unknown_returns_nil() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local v = GetLFGDungeonInfo(9999)
            return v == nil and "nil" or "not_nil"
            "#,
        )
        .unwrap();
    assert_eq!(result, "nil");
}

#[test]
fn get_lfg_dungeon_reward_cap_info_returns_inert_cap_shape() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if type(GetLFGDungeonRewardCapInfo) ~= "function" then
                return "type=" .. type(GetLFGDungeonRewardCapInfo)
            end
            local values = {GetLFGDungeonRewardCapInfo(1203)}
            if #values ~= 0 then return "count=" .. #values end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFGDungeonRewardCapInfo: {result}");
}

#[test]
fn lfg_rewards_frame_estimate_remaining_completions_handles_no_cap() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            function LFGRewardsFrame_EstimateRemainingCompletions(dungeonID)
                local currencyID, currencyQuantity, specificQuantity, specificLimit,
                    overallQuantity, overallLimit, periodPurseQuantity, periodPurseLimit,
                    purseQuantity, purseLimit, isWeekly = GetLFGDungeonRewardCapInfo(dungeonID)
                if not currencyID then
                    return 0, false
                end
                local remainingAllotment = math.min(specificLimit - specificQuantity, overallLimit - overallQuantity)
                if periodPurseLimit ~= 0 then
                    remainingAllotment = math.min(remainingAllotment, periodPurseLimit - periodPurseQuantity)
                end
                if currencyQuantity == 0 then
                    return 0, isWeekly
                end
                return math.ceil(remainingAllotment / currencyQuantity), isWeekly
            end

            local count, weekly = LFGRewardsFrame_EstimateRemainingCompletions(1203)
            if count ~= 0 then return "count=" .. tostring(count) end
            if weekly ~= false then return "weekly=" .. tostring(weekly) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "LFG reward cap no-cap path: {result}");
}
