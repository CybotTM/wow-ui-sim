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

#[test]
fn get_num_random_dungeons() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local n = GetNumRandomDungeons()
            if n < 1 then return "count=" .. n end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetNumRandomDungeons: {result}");
}

#[test]
fn get_lfg_random_dungeon_info() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local id, name = GetLFGRandomDungeonInfo(1)
            if type(id) ~= "number" then return "id_type=" .. type(id) end
            if type(name) ~= "string" then return "name_type=" .. type(name) end
            if name == "" then return "empty_name" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFGRandomDungeonInfo(1): {result}");
}

#[test]
fn get_random_dungeon_best_choice() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local id = GetRandomDungeonBestChoice()
            if type(id) ~= "number" then return "type=" .. type(id) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetRandomDungeonBestChoice: {result}");
}

#[test]
fn dungeon_appears_in_random_lfd_reports_lfd_category_for_seeded_dungeons() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if type(DungeonAppearsInRandomLFD) ~= "function" then
                return "type=" .. type(DungeonAppearsInRandomLFD)
            end
            if DungeonAppearsInRandomLFD(1201) ~= LE_LFG_CATEGORY_LFD then
                return "known=" .. tostring(DungeonAppearsInRandomLFD(1201))
            end
            if DungeonAppearsInRandomLFD(-1) ~= nil then
                return "header=" .. tostring(DungeonAppearsInRandomLFD(-1))
            end
            if DungeonAppearsInRandomLFD(1271) ~= nil then
                return "journal_id=" .. tostring(DungeonAppearsInRandomLFD(1271))
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "DungeonAppearsInRandomLFD: {result}");
}

#[test]
fn unit_has_lfg_random_cooldown_is_registered_and_defaults_false() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if type(UnitHasLFGRandomCooldown) ~= "function" then
                return "type=" .. type(UnitHasLFGRandomCooldown)
            end
            if UnitHasLFGRandomCooldown("player") ~= false then
                return "player=" .. tostring(UnitHasLFGRandomCooldown("player"))
            end
            if UnitHasLFGRandomCooldown("party1") ~= false then
                return "party1=" .. tostring(UnitHasLFGRandomCooldown("party1"))
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "UnitHasLFGRandomCooldown: {result}");
}

#[test]
fn c_lfg_info_is_follower_dungeon() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- 1202 = City of Threads (is_follower_dungeon=true)
            if not C_LFGInfo.IsLFGFollowerDungeon(1202) then return "1202_not_follower" end
            -- 1203 = Mists of Tirna Scithe (is_follower_dungeon=false)
            if C_LFGInfo.IsLFGFollowerDungeon(1203) then return "1203_is_follower" end
            -- unknown id
            if C_LFGInfo.IsLFGFollowerDungeon(9999) then return "9999_is_follower" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "IsLFGFollowerDungeon: {result}");
}

#[test]
fn get_lfd_lock_player_count_returns_zero() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local n = GetLFDLockPlayerCount()
            if type(n) ~= "number" then return "type=" .. type(n) end
            if n ~= 0 then return "count=" .. n end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFDLockPlayerCount: {result}");
}

#[test]
fn get_lfd_lock_info_returns_six_nils() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local a, b, c, d, e, f = GetLFDLockInfo(1203, 1)
            if a ~= nil or b ~= nil or c ~= nil or d ~= nil or e ~= nil or f ~= nil then
                return "non_nil"
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFDLockInfo: {result}");
}

#[test]
fn get_lfd_role_lock_info_returns_empty_table() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local t = GetLFDRoleLockInfo(1203, 1)
            if type(t) ~= "table" then return "type=" .. type(t) end
            if #t ~= 0 then return "count=" .. #t end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFDRoleLockInfo: {result}");
}

#[test]
fn lfg_construct_declined_message_does_not_error() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- Mirrors LFGConstructDeclinedMessage's loop; the call must not
            -- raise even when nothing is locked.
            local ok = pcall(function()
                for i = 1, GetLFDLockPlayerCount() do
                    GetLFDLockInfo(1203, i)
                end
            end)
            return ok and "ok" or "error"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn is_lfg_dungeon_joinable_in_range() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- 1205 = Grim Batol, min_level=15, max_level=80; player default level 70
            local all, player, hide, size = IsLFGDungeonJoinable(1205)
            if all ~= true then return "all=" .. tostring(all) end
            if player ~= true then return "player=" .. tostring(player) end
            if hide ~= false then return "hide=" .. tostring(hide) end
            if size ~= 5 then return "size=" .. tostring(size) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "IsLFGDungeonJoinable(1205): {result}");
}

#[test]
fn is_lfg_dungeon_joinable_out_of_range() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- -1 = Random Heroic header, min/max=80; player default level 70
            local all, player, hide, size = IsLFGDungeonJoinable(-1)
            if all ~= true then return "all=" .. tostring(all) end
            if player ~= false then return "player=" .. tostring(player) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "IsLFGDungeonJoinable(-1): {result}");
}

#[test]
fn is_lfg_dungeon_joinable_unknown_id() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local all, player, hide, size = IsLFGDungeonJoinable(9999)
            if all ~= false then return "all=" .. tostring(all) end
            if player ~= false then return "player=" .. tostring(player) end
            if hide ~= true then return "hide=" .. tostring(hide) end
            if size ~= 0 then return "size=" .. tostring(size) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "IsLFGDungeonJoinable(9999): {result}");
}

#[test]
fn get_lfg_dungeon_num_encounters() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local n, c = GetLFGDungeonNumEncounters(1203)
            if type(n) ~= "number" then return "num_type=" .. type(n) end
            if n < 1 then return "count=" .. n end
            if c ~= 0 then return "completed=" .. c end
            local n2, c2 = GetLFGDungeonNumEncounters(9999)
            if n2 ~= 0 then return "unknown_count=" .. n2 end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetLFGDungeonNumEncounters: {result}");
}
