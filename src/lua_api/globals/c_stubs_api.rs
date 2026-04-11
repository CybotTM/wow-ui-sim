//! C_* namespace stubs and global function stubs for Blizzard UI code.
//! See also: c_stubs_api_missing.rs, c_stubs_api_namespaces.rs, c_stubs_api_extra.rs,
//! c_stubs_api_combat.rs, c_stubs_api_glue.rs, c_stubs_api_professions.rs,
//! c_stubs_api_lfg.rs, c_stubs_api_unit_frame.rs, c_stubs_api_chat_quest.rs.

use mlua::{Lua, Result, Value};

/// Register all additional C_* namespace stubs.
pub fn register_c_stubs_api(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    register_core_namespaces(lua, std::rc::Rc::clone(&state))?;
    register_ui_and_chat_stubs(lua, state.clone())?;
    super::c_stubs_api_missing::register_missing_globals(lua, state.clone())?;
    super::c_stubs_api_namespaces::register_missing_namespaces(lua, state.clone())?;
    super::c_stubs_api_namespaces::register_c_perks_activities(lua)?;
    super::c_stubs_api_namespaces::register_game_state_stubs(lua)?;
    super::c_stubs_api_namespaces::register_c_incoming_summon(lua)?;
    super::c_stubs_api_extra::register_extra_stubs(lua, state.clone())?;
    super::c_stubs_api_combat::register_combat_stubs(lua)?;
    super::c_stubs_api_professions::register_profession_stubs(lua, state)?;
    Ok(())
}

fn register_core_namespaces(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    register_c_achievement_info(lua)?;
    register_c_battle_net(lua, std::rc::Rc::clone(&state))?;
    register_c_debug(lua, std::rc::Rc::clone(&state))?;
    super::hero_talents::register_c_class_talents(lua, std::rc::Rc::clone(&state))?;
    super::c_stubs_api_lfg::register_c_guild(lua, std::rc::Rc::clone(&state))?;
    super::c_stubs_api_lfg::register_c_guild_info(lua)?;
    super::c_stubs_api_lfg::register_c_lfg_list(lua, std::rc::Rc::clone(&state))?;
    super::c_stubs_api_lfg::register_c_loss_of_control(lua, std::rc::Rc::clone(&state))?;
    register_c_mail(lua, std::rc::Rc::clone(&state))?;
    register_c_stable_info(lua)?;
    register_c_tutorial(lua)?;
    super::action_bar_api::register_c_action_bar_namespace(lua, state.clone())?;
    super::c_stubs_api_unit_frame::register_unit_frame_global_stubs(
        lua,
        std::rc::Rc::clone(&state),
    )?;
    super::c_stubs_api_unit_frame::register_powerbar_prediction_colors(lua)?;
    super::c_stubs_achievement::register_achievement_stubs(lua)?;
    super::c_stubs_achievement::register_tracking_stubs(lua)?;
    Ok(())
}

fn register_ui_and_chat_stubs(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    register_c_log(lua)?;
    register_c_campaign_info(lua)?;
    super::c_stubs_api_chat_quest::register_quest_global_functions(lua, state)?;
    super::c_stubs_api_chat_quest::register_chat_stubs(lua)?;
    super::c_stubs_api_chat_quest::register_chat_window_stubs(lua)?;
    super::c_stubs_api_chat_quest::register_c_macro(lua)?;
    register_c_wowlabs_matchmaking(lua)?;
    super::fading_frame_api::register_fading_frame_stubs(lua)?;
    Ok(())
}

fn register_c_achievement_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetRewardItemID",
        lua.create_function(|_, _achievement_id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetAchievementInfo",
        lua.create_function(|_, _achievement_id: i32| Ok(Value::Nil))?,
    )?;
    lua.globals().set("C_AchievementInfo", t)?;
    Ok(())
}

fn register_c_battle_net(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let globals = lua.globals();
    let t: mlua::Table = match globals.get::<Value>("C_BattleNet")? {
        Value::Table(table) => table,
        _ => lua.create_table()?,
    };

    let installed_state = std::rc::Rc::clone(&state);
    t.set(
        "AreHighResTexturesInstalled",
        lua.create_function(move |_, ()| {
            Ok(installed_state
                .borrow()
                .cvars
                .get_bool("useHighResTextures"))
        })?,
    )?;

    let install_state = std::rc::Rc::clone(&state);
    t.set(
        "InstallHighResTextures",
        lua.create_function(move |_, ()| {
            install_state.borrow().cvars.set("useHighResTextures", "1");
            Ok(())
        })?,
    )?;

    globals.set("C_BattleNet", t)?;
    Ok(())
}

fn format_debug_window_args(args: &[Value]) -> String {
    let mut output = String::new();
    for (index, arg) in args.iter().enumerate() {
        if index > 0 {
            output.push('\t');
        }
        match arg {
            Value::Nil => output.push_str("nil"),
            Value::Boolean(value) => output.push_str(if *value { "true" } else { "false" }),
            Value::Integer(value) => output.push_str(&value.to_string()),
            Value::Number(value) => output.push_str(&value.to_string()),
            Value::String(value) => output.push_str(&value.to_string_lossy()),
            Value::Table(_) => output.push_str("table"),
            Value::Function(_) => output.push_str("function"),
            Value::UserData(_) => output.push_str("userdata"),
            _ => output.push_str(&format!("{arg:?}")),
        }
    }
    output
}

fn register_c_debug(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let globals = lua.globals();
    let t: mlua::Table = match globals.get::<Value>("C_Debug")? {
        Value::Table(table) => table,
        _ => lua.create_table()?,
    };

    let print_state = std::rc::Rc::clone(&state);
    t.set(
        "PrintToDebugWindow",
        lua.create_function(move |_, message: String| {
            print_state.borrow_mut().console_output.push(message);
            Ok(())
        })?,
    )?;

    let view_state = std::rc::Rc::clone(&state);
    t.set(
        "ViewInDebugWindow",
        lua.create_function(move |_, args: mlua::Variadic<Value>| {
            let line = format_debug_window_args(args.as_slice());
            view_state.borrow_mut().console_output.push(line);
            Ok(())
        })?,
    )?;

    globals.set("C_Debug", t)?;
    Ok(())
}

fn register_c_mail(
    lua: &Lua,
    state: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let globals = lua.globals();
    let t: mlua::Table = match globals.get::<Value>("C_Mail")? {
        Value::Table(existing) => existing,
        _ => {
            let created = lua.create_table()?;
            globals.set("C_Mail", created.clone())?;
            created
        }
    };
    t.set(
        "GetNumItems",
        lua.create_function({
            let state = std::rc::Rc::clone(&state);
            move |_, ()| Ok(state.borrow().player.inbox.len() as i32)
        })?,
    )?;
    t.set(
        "HasNewMail",
        lua.create_function(move |_, ()| {
            let has_unread = state
                .borrow()
                .player
                .inbox
                .iter()
                .any(|mail| !mail.was_read);
            Ok(has_unread)
        })?,
    )?;
    t.set("IsCommandPending", lua.create_function(|_, ()| Ok(false))?)?;
    Ok(())
}

fn register_c_stable_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("GetNumStablePets", lua.create_function(|_, ()| Ok(0i32))?)?;
    lua.globals().set("C_StableInfo", t)?;
    Ok(())
}

fn register_c_tutorial(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetTutorialStatus",
        lua.create_function(|_, _tutorial_id: Option<i32>| Ok(false))?,
    )?;
    t.set(
        "SetTutorialFlag",
        lua.create_function(|_, _tutorial_id: i32| Ok(()))?,
    )?;
    lua.globals().set("C_Tutorial", t)?;
    Ok(())
}

fn register_c_log(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("LogMessage", lua.create_function(|_, _msg: Value| Ok(()))?)?;
    t.set(
        "LogErrorMessage",
        lua.create_function(|_, _msg: Value| Ok(()))?,
    )?;
    lua.globals().set("C_Log", t)?;
    Ok(())
}

/// C_CampaignInfo namespace - campaign/war campaign data.
fn register_c_campaign_info(lua: &Lua) -> Result<()> {
    const CAMPAIGN_STATE_INVALID: i32 = 0;

    let t = lua.create_table()?;
    t.set(
        "GetCampaignID",
        lua.create_function(|_, _quest_id: i32| Ok(0i32))?,
    )?;
    t.set(
        "GetCampaignInfo",
        lua.create_function(|_, _campaign_id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetState",
        lua.create_function(|_, _campaign_id: i32| Ok(CAMPAIGN_STATE_INVALID))?,
    )?;
    lua.globals().set("C_CampaignInfo", t)?;
    Ok(())
}

fn register_c_wowlabs_matchmaking(lua: &Lua) -> Result<()> {
    lua.load(WOWLABS_MATCHMAKING_LUA).exec()?;
    Ok(())
}

const WOWLABS_MATCHMAKING_LUA: &str = r#"
    C_WoWLabsMatchmaking = C_WoWLabsMatchmaking or {}
    local matchmaking = C_WoWLabsMatchmaking

    local PARTY_PLAYLIST = Enum and Enum.PartyPlaylistEntry or {
        SoloGameMode = 0,
        DuoGameMode = 1,
        TrioGameMode = 2,
        TrainingGameMode = 3,
    }

    local AREA_TYPE = Enum and Enum.WoWLabsAreaType or {
        PlunderstormDropSparse = 0,
        PlunderstormDropMedium = 1,
        PlunderstormDropDense = 2,
    }

    matchmaking._state = matchmaking._state or {
        playerGUID = "Player-0000-0000A11E5510",
        playerName = "Alessio",
        playerBNetID = 42,
        partyPlaylistEntry = PARTY_PLAYLIST.DuoGameMode,
        playerReady = false,
        findingMatch = false,
        fastLogin = false,
        autoQueueOnLogout = false,
        autoQueuePartyPlaylistEntry = PARTY_PLAYLIST.DuoGameMode,
        inQueueTimeStart = 0,
        partyInvites = {
            {
                inviteID = "WoWLabsInvite-1",
                playerName = "PartyPal",
                inviterGUID = "Player-0000-INVITE0001",
                bnetIDAccount = 84,
            },
        },
        currentParty = {
            {
                partyMemberGUID = "Player-0000-0000A11E5510",
                playerName = "Alessio",
                isPartyLeader = true,
                isReady = false,
                isLocalPlayer = true,
                raceFilename = "Human",
                gender = Enum and Enum.UnitSex and Enum.UnitSex.Male or 0,
                classFilename = "PALADIN",
                bnetIDAccount = 42,
            },
            {
                partyMemberGUID = "Player-0000-0000BEEFBEEF",
                playerName = "Riona",
                isPartyLeader = false,
                isReady = false,
                isLocalPlayer = false,
                raceFilename = "NightElf",
                gender = Enum and Enum.UnitSex and Enum.UnitSex.Female or 1,
                classFilename = "DRUID",
                bnetIDAccount = 84,
            },
        },
    }

    C_WowLabsDataManager = C_WowLabsDataManager or {}
    local dataManager = C_WowLabsDataManager
    dataManager._state = dataManager._state or {
        isInPrematch = true,
        selectedAreaID = nil,
        confirmedAreaID = nil,
        areas = {
            { wowLabsAreaID = 101, x = 0.48, y = 0.42, areaType = AREA_TYPE.PlunderstormDropDense },
            { wowLabsAreaID = 102, x = 0.56, y = 0.57, areaType = AREA_TYPE.PlunderstormDropMedium },
            { wowLabsAreaID = 103, x = 0.37, y = 0.66, areaType = AREA_TYPE.PlunderstormDropSparse },
        },
        circleInfo = {
            startLerpTime = 0,
            timeToLerp = 30000,
            outerPosition = { x = 0.5, y = 0.5 },
            innerPosition = { x = 0.5, y = 0.5 },
            baseRadius = 0.38,
            outerScale = 1.0,
            innerScale = 0.8,
            predictionPosition = { x = 0.58, y = 0.47 },
            predictionScale = 0.6,
            initialBaseSize = 0.32,
        },
    }

    local function deepcopy(value)
        if type(value) ~= "table" then
            return value
        end

        local copy = {}
        for key, innerValue in pairs(value) do
            copy[key] = deepcopy(innerValue)
        end
        return copy
    end

    local function normalize_playlist_entry(value)
        local numericValue = tonumber(value)
        if numericValue == nil then
            return nil
        end

        if numericValue < PARTY_PLAYLIST.SoloGameMode or numericValue > PARTY_PLAYLIST.TrainingGameMode then
            return nil
        end

        return numericValue
    end

    local function max_party_size_for_playlist(playlistEntry)
        if playlistEntry == PARTY_PLAYLIST.SoloGameMode then
            return 1
        elseif playlistEntry == PARTY_PLAYLIST.DuoGameMode then
            return 2
        elseif playlistEntry == PARTY_PLAYLIST.TrioGameMode then
            return 3
        elseif playlistEntry == PARTY_PLAYLIST.TrainingGameMode then
            return 3
        end

        return 0
    end

    local function get_local_player_entry()
        for _, member in ipairs(matchmaking._state.currentParty) do
            if member.isLocalPlayer then
                return member
            end
        end
        return nil
    end

    local function is_party_leader()
        local localPlayer = get_local_player_entry()
        return localPlayer ~= nil and localPlayer.isPartyLeader == true
    end

    local function get_party_size()
        return #matchmaking._state.currentParty
    end

    local function can_enter_matchmaking()
        local maxSize = max_party_size_for_playlist(matchmaking._state.partyPlaylistEntry)
        return maxSize > 0 and get_party_size() > 0 and get_party_size() <= maxSize
    end

    local function refresh_player_ready_flags()
        local localPlayer = get_local_player_entry()
        if localPlayer then
            localPlayer.isReady = matchmaking._state.playerReady == true
        end
    end

    local function refresh_queue_state()
        local shouldFindMatch = matchmaking._state.playerReady == true and is_party_leader() and can_enter_matchmaking()
        local wasFindingMatch = matchmaking._state.findingMatch == true
        matchmaking._state.findingMatch = shouldFindMatch
        if shouldFindMatch then
            if not wasFindingMatch or tonumber(matchmaking._state.inQueueTimeStart) == 0 then
                matchmaking._state.inQueueTimeStart = math.floor(GetTime() * 1000)
            end
        else
            matchmaking._state.inQueueTimeStart = 0
        end
    end

    function matchmaking.GetCurrentParty()
        return deepcopy(matchmaking._state.currentParty)
    end

    function matchmaking.GetPartyPlaylistEntry()
        return matchmaking._state.partyPlaylistEntry
    end

    function matchmaking.SetPartyPlaylistEntry(playlistEntry)
        local normalized = normalize_playlist_entry(playlistEntry)
        if normalized == nil then
            return false
        end

        local maxPartySize = max_party_size_for_playlist(normalized)
        if get_party_size() > maxPartySize then
            return false
        end

        matchmaking._state.partyPlaylistEntry = normalized
        refresh_queue_state()
        return true
    end

    function matchmaking.GetPartySize()
        return get_party_size()
    end

    function matchmaking.IsPartyFull()
        return get_party_size() >= 3
    end

    function matchmaking.IsAloneInWoWLabsParty()
        return get_party_size() <= 1
    end

    function matchmaking.IsPartyLeader()
        return is_party_leader()
    end

    function matchmaking.IsPlayer(guid)
        local localPlayer = get_local_player_entry()
        return localPlayer ~= nil and localPlayer.partyMemberGUID == guid
    end

    function matchmaking.IsWowLabsMatchmakingMember(guid)
        for _, member in ipairs(matchmaking._state.currentParty) do
            if member.partyMemberGUID == guid then
                return true
            end
        end
        return false
    end

    function matchmaking.IsPlayerReady()
        return matchmaking._state.playerReady == true
    end

    function matchmaking.SetPlayerReady(isReady)
        matchmaking._state.playerReady = isReady == true
        refresh_player_ready_flags()
        refresh_queue_state()
    end

    function matchmaking.CanEnterMatchmaking()
        return can_enter_matchmaking()
    end

    function matchmaking.IsFindingMatch()
        return matchmaking._state.findingMatch == true
    end

    function matchmaking.GetInQueueTimeStart()
        return tonumber(matchmaking._state.inQueueTimeStart) or 0
    end

    function matchmaking.IsFastLogin()
        return matchmaking._state.fastLogin == true
    end

    function matchmaking.ClearFastLogin()
        matchmaking._state.fastLogin = false
    end

    function matchmaking.GetAutoQueueOnLogout()
        return matchmaking._state.autoQueueOnLogout == true, matchmaking._state.autoQueuePartyPlaylistEntry
    end

    function matchmaking.SetAutoQueueOnLogout(flag, queueType)
        matchmaking._state.autoQueueOnLogout = flag == true
        matchmaking._state.autoQueuePartyPlaylistEntry = normalize_playlist_entry(queueType) or matchmaking._state.partyPlaylistEntry
    end

    function matchmaking.GetNumPartyInvites()
        return #matchmaking._state.partyInvites
    end

    function matchmaking.GetPartyInviteByIndex(index)
        local invite = matchmaking._state.partyInvites[(tonumber(index) or -1) + 1]
        if invite == nil then
            return nil
        end
        return invite.playerName, invite.inviteID
    end

    local function clear_invite(inviteIndex)
        table.remove(matchmaking._state.partyInvites, inviteIndex)
    end

    function matchmaking.AcceptPartyInvite(inviteID)
        for index, invite in ipairs(matchmaking._state.partyInvites) do
            if invite.inviteID == inviteID then
                local localPlayer = get_local_player_entry()
                matchmaking._state.currentParty = {
                    {
                        partyMemberGUID = invite.inviterGUID,
                        playerName = invite.playerName,
                        isPartyLeader = true,
                        isReady = false,
                        isLocalPlayer = false,
                        raceFilename = "Orc",
                        gender = Enum and Enum.UnitSex and Enum.UnitSex.Male or 0,
                        classFilename = "WARRIOR",
                        bnetIDAccount = invite.bnetIDAccount,
                    },
                    {
                        partyMemberGUID = localPlayer and localPlayer.partyMemberGUID or matchmaking._state.playerGUID,
                        playerName = localPlayer and localPlayer.playerName or matchmaking._state.playerName,
                        isPartyLeader = false,
                        isReady = false,
                        isLocalPlayer = true,
                        raceFilename = localPlayer and localPlayer.raceFilename or "Human",
                        gender = localPlayer and localPlayer.gender or (Enum and Enum.UnitSex and Enum.UnitSex.Male or 0),
                        classFilename = localPlayer and localPlayer.classFilename or "PALADIN",
                        bnetIDAccount = localPlayer and localPlayer.bnetIDAccount or matchmaking._state.playerBNetID,
                    },
                }
                clear_invite(index)
                matchmaking._state.playerReady = false
                refresh_player_ready_flags()
                refresh_queue_state()
                return true
            end
        end
        return false
    end

    function matchmaking.DeclinePartyInvite(inviteID)
        for index, invite in ipairs(matchmaking._state.partyInvites) do
            if invite.inviteID == inviteID then
                clear_invite(index)
                return true
            end
        end
        return false
    end

    function matchmaking.LeaveParty()
        local localPlayer = get_local_player_entry()
        matchmaking._state.currentParty = {
            {
                partyMemberGUID = localPlayer and localPlayer.partyMemberGUID or matchmaking._state.playerGUID,
                playerName = localPlayer and localPlayer.playerName or matchmaking._state.playerName,
                isPartyLeader = true,
                isReady = false,
                isLocalPlayer = true,
                raceFilename = localPlayer and localPlayer.raceFilename or "Human",
                gender = localPlayer and localPlayer.gender or (Enum and Enum.UnitSex and Enum.UnitSex.Male or 0),
                classFilename = localPlayer and localPlayer.classFilename or "PALADIN",
                bnetIDAccount = localPlayer and localPlayer.bnetIDAccount or matchmaking._state.playerBNetID,
            },
        }
        matchmaking._state.playerReady = false
        refresh_player_ready_flags()
        refresh_queue_state()
        return true
    end

    function matchmaking.RemovePlayerFromParty(guid)
        for index, member in ipairs(matchmaking._state.currentParty) do
            if member.partyMemberGUID == guid and not member.isLocalPlayer then
                table.remove(matchmaking._state.currentParty, index)
                refresh_queue_state()
                return true
            end
        end
        return false
    end

    function matchmaking.SendPartyInvite(bnetIDAccount)
        if matchmaking.IsPartyFull() or tonumber(bnetIDAccount) == nil then
            return false
        end
        return true
    end

    function dataManager.IsInPrematch()
        return dataManager._state.isInPrematch == true
    end

    function dataManager.GetWoWLabsAreaInfo()
        return deepcopy(dataManager._state.areas)
    end

    function dataManager.GetConfirmedWoWLabsArea()
        return dataManager._state.confirmedAreaID
    end

    function dataManager.SelectWoWLabsArea(areaID)
        local normalized = tonumber(areaID)
        if normalized == nil then
            return false
        end

        for _, area in ipairs(dataManager._state.areas) do
            if area.wowLabsAreaID == normalized then
                dataManager._state.selectedAreaID = normalized
                dataManager._state.confirmedAreaID = normalized
                return true
            end
        end

        return false
    end

    function dataManager.QuerySelectedWoWLabsArea()
        return dataManager._state.selectedAreaID
    end

    function dataManager.QueryWoWLabsAreaInfo()
        return deepcopy(dataManager._state.areas)
    end

    function dataManager.PushCircleInfoToLua()
        local info = dataManager._state.circleInfo
        return info.startLerpTime, info.timeToLerp, deepcopy(info.outerPosition), deepcopy(info.innerPosition), info.baseRadius, info.outerScale, info.innerScale, deepcopy(info.predictionPosition), info.predictionScale, info.initialBaseSize
    end
"#;
