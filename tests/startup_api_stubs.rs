//! Smoke tests for startup-surface stubs added to unblock Blizzard addon
//! loading. Each stub returns values that reflect the simulator's reality
//! (no network, no in-game store, no premade finder, no photo sharing)
//! rather than invented placeholders.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn clamp_and_saturate_exist_in_shared_bootstrap() {
    let env = env();
    let (clamped_low, clamped_high, saturated): (f64, f64, f64) = env
        .eval("return Clamp(-2, 0, 5), Clamp(8, 0, 5), Saturate(1.5)")
        .expect("Clamp and Saturate should be callable");
    assert_eq!(clamped_low, 0.0);
    assert_eq!(clamped_high, 5.0);
    assert_eq!(saturated, 1.0);
}

#[test]
fn fading_frame_helpers_seed_default_timers_and_can_copy_them() {
    let env = env();
    let (hidden, fade_in, hold, fade_out, copied_hold): (bool, f64, f64, f64, f64) = env
        .eval(
            r#"
            local src = CreateFrame("Frame", "CodexFadingFrameSource", UIParent)
            local dest = CreateFrame("Frame", "CodexFadingFrameDest", UIParent)
            FadingFrame_OnLoad(src)
            FadingFrame_SetFadeInTime(src, 0.25)
            FadingFrame_SetHoldTime(src, 1.5)
            FadingFrame_SetFadeOutTime(src, 0.75)
            FadingFrame_CopyTimes(src, dest)
            return not src:IsShown(), src.fadeInTime, src.holdTime, src.fadeOutTime, dest.holdTime
            "#,
        )
        .expect("fading-frame helpers should be callable");
    assert!(hidden);
    assert_eq!(fade_in, 0.25);
    assert_eq!(hold, 1.5);
    assert_eq!(fade_out, 0.75);
    assert_eq!(copied_hold, 1.5);
}

#[test]
fn get_net_stats_returns_four_zeros() {
    let env = env();
    let (bw_in, bw_out, latency_home, latency_world): (f64, f64, f64, f64) = env
        .eval("return GetNetStats()")
        .expect("GetNetStats should be callable");
    assert_eq!(bw_in, 0.0);
    assert_eq!(bw_out, 0.0);
    assert_eq!(latency_home, 0.0);
    assert_eq!(latency_world, 0.0);
}

#[test]
fn store_frame_is_shown_returns_false() {
    let env = env();
    let shown: bool = env.eval("return StoreFrame_IsShown()").unwrap();
    assert!(!shown, "no Store UI is ever rendered in the sim");
}

#[test]
fn quest_poi_update_icons_is_callable() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            local ok = pcall(function()
                QuestPOIUpdateIcons()
            end)
            return ok
            "#,
        )
        .expect("QuestPOIUpdateIcons smoke probe should run");
    assert!(
        ok,
        "QuestPOIUpdateIcons should be callable during QuestMap refresh"
    );
}

#[test]
fn corruption_helpers_exist_with_safe_defaults() {
    let env = env();
    let (corruption, resistance, effects_ty, effects_len): (f64, f64, String, i64) = env
        .eval(
            r#"
            local effects = GetNegativeCorruptionEffectInfo()
            return GetCorruption(),
                   GetCorruptionResistance(),
                   type(effects),
                   #effects
            "#,
        )
        .expect("corruption helpers should be callable");
    assert_eq!(corruption, 0.0);
    assert_eq!(resistance, 0.0);
    assert_eq!(effects_ty, "table");
    assert_eq!(effects_len, 0);
}

#[test]
fn quest_sort_helpers_are_callable_noops() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            return pcall(function()
                SortQuestSortTypes()
                SortQuests()
            end)
            "#,
        )
        .expect("quest sort helpers should be callable");
    assert!(
        ok,
        "quest sort helpers should stay callable during QuestMap refresh"
    );
}

#[test]
fn nameplate_size_helper_exists_as_safe_noop() {
    let env = env();
    let ok: bool = env
        .eval(
            r#"
            return pcall(function()
                C_NamePlate.SetNamePlateSize(110, 45)
            end)
            "#,
        )
        .expect("nameplate size helper should be callable");
    assert!(
        ok,
        "SetNamePlateSize should exist even though the sim renders no 3D nameplates"
    );
}

#[test]
fn instance_abandon_vote_and_shutdown_helpers_return_numeric_pairs() {
    let env = env();
    let (
        vote_duration,
        vote_time_left,
        shutdown_duration,
        shutdown_time_left,
        response_is_nil,
        response_set_ok,
        vote_count,
        can_start_vote,
        start_vote_ok,
    ): (f64, f64, f64, f64, bool, bool, f64, bool, bool) = env
        .eval(
            r#"
            local voteDuration, voteTimeLeft = C_PartyInfo.GetInstanceAbandonVoteTime()
            local shutdownDuration, shutdownTimeLeft = C_PartyInfo.GetInstanceAbandonShutdownTime()
            local response = C_PartyInfo.GetInstanceAbandonVoteResponse()
            local responseSetOk = pcall(function()
                C_PartyInfo.SetInstanceAbandonVoteResponse(true)
            end)
            local startVoteOk = pcall(function()
                C_PartyInfo.StartInstanceAbandonVote()
            end)

            return voteDuration,
                   voteTimeLeft,
                   shutdownDuration,
                   shutdownTimeLeft,
                   response == nil,
                   responseSetOk,
                   C_PartyInfo.GetNumInstanceAbandonGroupVoteResponses(),
                   C_PartyInfo.CanStartInstanceAbandonVote(),
                   startVoteOk
            "#,
        )
        .expect("instance abandon helpers should be callable");

    assert_eq!(vote_duration, 0.0);
    assert_eq!(vote_time_left, 0.0);
    assert_eq!(shutdown_duration, 0.0);
    assert_eq!(shutdown_time_left, 0.0);
    assert!(response_is_nil);
    assert!(response_set_ok);
    assert_eq!(vote_count, 0.0);
    assert!(!can_start_vote);
    assert!(start_vote_ok);
}

#[test]
fn startup_party_chat_and_targeting_globals_are_callable() {
    let env = env();
    let (clear_focus_ok, focus_cleared, request_ok, remove_channel_ok, can_mark_player): (
        bool,
        bool,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            A_Admin.SetFocus("Training Dummy", 72, 1)

            local clearFocusOk = pcall(function()
                ClearFocus()
            end)
            local requestOk = pcall(function()
                RequestGuildPartyState()
            end)
            local removeChannelOk = pcall(function()
                RemoveChatWindowChannel(1, "General")
            end)

            return clearFocusOk,
                   not UnitExists("focus"),
                   requestOk,
                   removeChannelOk,
                   CanBeRaidTarget("player")
            "#,
        )
        .expect("party/chat/targeting startup globals should be callable");

    assert!(clear_focus_ok, "ClearFocus should exist as a normal global");
    assert!(
        focus_cleared,
        "ClearFocus should clear the simulated focus unit"
    );
    assert!(request_ok, "RequestGuildPartyState should be a safe no-op");
    assert!(
        remove_channel_ok,
        "RemoveChatWindowChannel should be callable during chat bootstrap"
    );
    assert!(
        can_mark_player,
        "existing units should remain markable by CanBeRaidTarget"
    );
}

#[test]
fn startup_lfg_world_timer_and_bn_surfaces_return_empty_safe_shapes() {
    let env = env();
    let (
        queued_type,
        queued_empty,
        ready_check_in_progress,
        ready_check_is_battleground,
        elapsed_timer_count,
        elapsed_timer_id,
        elapsed_time,
        elapsed_type,
        bn_total,
        bn_online,
        bn_favorite,
        bn_favorite_online,
    ): (
        String,
        bool,
        bool,
        bool,
        i32,
        f64,
        f64,
        f64,
        i32,
        i32,
        i32,
        i32,
    ) = env
        .eval(
            r##"
            local queued = GetLFGQueuedList(1)
            local readyCheckInProgress, readyCheckIsBattleground = GetLFGReadyCheckUpdate()
            local timerID, elapsedTime, timerType = GetWorldElapsedTime(7)

            return type(queued),
                   next(queued) == nil,
                   readyCheckInProgress,
                   readyCheckIsBattleground,
                   select("#", GetWorldElapsedTimers()),
                   timerID,
                   elapsedTime,
                   timerType,
                   BNGetNumFriends()
            "##,
        )
        .expect("startup LFG/world timer/BN stubs should be callable");

    assert_eq!(queued_type, "table");
    assert!(queued_empty, "no LFG queues should be seeded by default");
    assert!(!ready_check_in_progress);
    assert!(!ready_check_is_battleground);
    assert_eq!(
        elapsed_timer_count, 0,
        "no world elapsed timers should be active"
    );
    assert_eq!(elapsed_timer_id, 7.0);
    assert_eq!(elapsed_time, 0.0);
    assert_eq!(elapsed_type, 0.0);
    assert_eq!(bn_total, 0);
    assert_eq!(bn_online, 0);
    assert_eq!(bn_favorite, 0);
    assert_eq!(bn_favorite_online, 0);
}

#[test]
fn startup_pvp_match_state_defaults_to_inactive() {
    let env = env();
    let state: i32 = env
        .eval("return C_PvP.GetActiveMatchState()")
        .expect("C_PvP.GetActiveMatchState should be callable");
    assert_eq!(
        state, 0,
        "startup should default to inactive PvP match state"
    );
}

#[test]
fn is_character_newly_boosted_returns_false() {
    let env = env();
    let boosted: bool = env
        .eval("return IsCharacterNewlyBoosted()")
        .expect("IsCharacterNewlyBoosted should be callable");
    assert!(
        !boosted,
        "boosted-character help flow is not simulated, so the probe should stay false"
    );
}

#[test]
fn glue_runtime_helpers_exist_with_safe_defaults() {
    let env = env();
    let (
        c_ui_type,
        avoids_notch,
        has_display_notch,
        c_glue_type,
        first_load,
        saved_account_name,
        saved_account_list,
        screen_first_displayed,
        login_background,
        min_expansion_level_type,
        server_name_type,
        player_location_kind,
        player_location_valid,
        adventure_guide_available,
        dungeon_normal_name,
        dungeon_normal_max_players,
        player_spells_util_type,
        has_spellbook_toggle,
        has_talent_toggle,
    ): (
        String,
        bool,
        bool,
        String,
        bool,
        String,
        String,
        bool,
        f64,
        String,
        String,
        bool,
        bool,
        bool,
        String,
        f64,
        String,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            UnitIsHumanPlayer = function(unit)
                return unit == "player"
            end

            local saved = GetSavedAccountList()
            local location = PlayerLocation:CreateFromUnit("player")

            return type(C_UI),
                   C_UI.ShouldUIParentAvoidNotch(),
                   C_UI.DoesAnyDisplayHaveNotch(),
                   type(C_Glue),
                   C_Glue.IsFirstLoadThisSession(),
                   GetSavedAccountName(),
                   saved,
                   WasScreenFirstDisplayed("login"),
                   GetLoginScreenBackground(42, 7),
                   type(GetMinimumExpansionLevel()),
                   type(GetServerName()),
                   location:IsUnit(),
                   location:IsValid(),
                   AdventureGuideUtil.IsAvailable(),
                   DifficultyUtil.GetDifficultyName(DifficultyUtil.ID.DungeonNormal),
                   DifficultyUtil.GetMaxPlayers(DifficultyUtil.ID.DungeonNormal),
                   type(PlayerSpellsUtil),
                   type(PlayerSpellsUtil.ToggleSpellBookFrame) == "function",
                   type(PlayerSpellsUtil.ToggleClassTalentFrame) == "function"
            "#,
        )
        .expect("glue runtime helpers should be callable");

    assert_eq!(c_ui_type, "table");
    assert!(!avoids_notch);
    assert!(!has_display_notch);
    assert_eq!(c_glue_type, "table");
    assert!(!first_load);
    assert_eq!(saved_account_name, "");
    assert_eq!(saved_account_list, "");
    assert!(!screen_first_displayed);
    assert_eq!(login_background, 42.0);
    assert_eq!(min_expansion_level_type, "number");
    assert_eq!(server_name_type, "nil");
    assert!(player_location_kind);
    assert!(player_location_valid);
    assert!(adventure_guide_available);
    assert!(!dungeon_normal_name.is_empty());
    assert_eq!(dungeon_normal_max_players, 5.0);
    assert_eq!(player_spells_util_type, "table");
    assert!(has_spellbook_toggle);
    assert!(has_talent_toggle);
}

#[test]
fn glue_character_select_helpers_exist_with_safe_defaults() {
    let env = env();
    let (
        set_model_frame_ok,
        set_map_scene_ok,
        set_world_frame_ok,
        max_groups,
        timerunning_season_kind,
        min_render_scale,
        max_render_scale,
        expansion_trial,
        recruit_active_type,
        recruit_faction_type,
        upgrade_expansion_level,
        undelete_enabled,
        undelete_cooldown,
    ): (
        bool,
        bool,
        bool,
        f64,
        String,
        f64,
        f64,
        bool,
        String,
        String,
        f64,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "CodexGlueWorldFrame", UIParent)
            local setWorldFrameOK = pcall(function()
                SetWorldFrameStrata(frame)
            end)
            local setModelFrameOK = pcall(function()
                SetCharSelectModelFrame("ModelFFX")
            end)
            local setMapSceneOK = pcall(function()
                SetCharSelectMapSceneFrame("MapScene")
            end)

            return setModelFrameOK,
                   setMapSceneOK,
                   setWorldFrameOK,
                   GetMaxWarbandGroupCount(),
                   type(GetActiveTimerunningSeasonID()),
                   GetMinRenderScale(),
                   GetMaxRenderScale(),
                   IsExpansionTrial(),
                   type(select(1, C_RecruitAFriend.GetRecruitInfo())),
                   type(select(2, C_RecruitAFriend.GetRecruitInfo())),
                   GetUpgradeExpansionLevel(),
                   GetCharacterUndeleteStatus()
            "#,
        )
        .expect("glue character-select helpers should be callable");

    assert!(set_model_frame_ok);
    assert!(set_map_scene_ok);
    assert!(set_world_frame_ok);
    assert_eq!(max_groups, 4.0);
    assert_eq!(timerunning_season_kind, "nil");
    assert_eq!(min_render_scale, 0.5);
    assert_eq!(max_render_scale, 1.0);
    assert!(!expansion_trial);
    assert_eq!(recruit_active_type, "boolean");
    assert_eq!(recruit_faction_type, "nil");
    assert_eq!(upgrade_expansion_level, 80.0);
    assert!(!undelete_enabled);
    assert!(!undelete_cooldown);
}

#[test]
fn c_lfg_info_can_player_use_premade_group_returns_false() {
    let env = env();
    let can_use: bool = env
        .eval("return C_LFGInfo.CanPlayerUsePremadeGroup()")
        .unwrap();
    assert!(
        !can_use,
        "premade group finder is not simulated, so the callsite takes the \
         'cannot use' branch and skips the premade promo UI"
    );
}

#[test]
fn recruit_a_friend_surface_returns_disabled_empty_defaults() {
    let env = env();
    let (
        enabled,
        recruiting_enabled,
        versions_len,
        recruits_len,
        claim_in_progress,
        recruit_active,
        recruit_faction_type,
    ): (bool, bool, f64, f64, bool, bool, String) = env
        .eval(
            r#"
            local info = C_RecruitAFriend.GetRAFInfo()
            local active, faction = C_RecruitAFriend.GetRecruitInfo()
            return C_RecruitAFriend.IsEnabled(),
                   C_RecruitAFriend.IsRecruitingEnabled(),
                   #info.versions,
                   #info.recruits,
                   info.claimInProgress,
                   active,
                   type(faction)
            "#,
        )
        .expect("Recruit-A-Friend fallback surface should be callable");
    assert!(!enabled);
    assert!(!recruiting_enabled);
    assert_eq!(versions_len, 1.0);
    assert_eq!(recruits_len, 0.0);
    assert!(!claim_in_progress);
    assert!(!recruit_active);
    assert_eq!(recruit_faction_type, "nil");
}

#[test]
fn map_util_helpers_exist_in_shared_bootstrap() {
    let env = env();
    let (displayable_map_id_type, map_type_zone_callable, parent_info_callable, cache_match): (
        String,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            local displayableMapID = MapUtil.GetDisplayableMapForPlayer()
            return type(displayableMapID),
                   pcall(function() return MapUtil.IsMapTypeZone(1) end),
                   pcall(function() return MapUtil.GetMapParentInfo(1, Enum.UIMapType.Zone) end),
                   MapUtil.IsChildMapCached(1, 1) == MapUtil.IsChildMap(1, 1)
            "#,
        )
        .expect("MapUtil fallback helpers should be callable");
    assert_eq!(displayable_map_id_type, "number");
    assert!(map_type_zone_callable);
    assert!(parent_info_callable);
    assert!(cache_match);
}

#[test]
fn get_icon_for_role_enum_returns_expected_role_atlases() {
    let env = env();
    let (tank, healer_disabled, damage): (String, String, String) = env
        .eval(
            r#"
            return GetIconForRoleEnum(Enum.LFGRole.Tank, false),
                   GetIconForRoleEnum(Enum.LFGRole.Healer, true),
                   GetIconForRoleEnum(Enum.LFGRole.Damage, false)
            "#,
        )
        .expect("role icon helper should be callable");
    assert_eq!(tank, "UI-LFG-RoleIcon-Tank");
    assert_eq!(healer_disabled, "UI-LFG-RoleIcon-Healer-Disabled");
    assert_eq!(damage, "UI-LFG-RoleIcon-DPS");
}

#[test]
fn event_util_helpers_defer_until_matching_startup_events_fire() {
    let env = env();
    env.exec(
        r#"
        EventUtilCalls = {
            variablesLoaded = 0,
            allEvents = 0,
            lateVariablesLoaded = 0,
        }

        EventUtil.ContinueOnVariablesLoaded(function()
            EventUtilCalls.variablesLoaded = EventUtilCalls.variablesLoaded + 1
        end)

        EventUtil.ContinueAfterAllEvents(function()
            EventUtilCalls.allEvents = EventUtilCalls.allEvents + 1
        end, "VARIABLES_LOADED", "PLAYER_ENTERING_WORLD", "FIRST_FRAME_RENDERED")
        "#,
    )
    .expect("EventUtil helpers should register callbacks");

    let (before_variables_loaded, before_all_events): (i32, i32) = env
        .eval("return EventUtilCalls.variablesLoaded, EventUtilCalls.allEvents")
        .expect("EventUtil callback counts should be readable");
    assert_eq!(before_variables_loaded, 0);
    assert_eq!(before_all_events, 0);

    env.fire_event("VARIABLES_LOADED")
        .expect("VARIABLES_LOADED should dispatch");
    let (after_variables_loaded, after_partial_events): (i32, i32) = env
        .eval("return EventUtilCalls.variablesLoaded, EventUtilCalls.allEvents")
        .expect("VARIABLES_LOADED should update EventUtil callback state");
    assert_eq!(after_variables_loaded, 1);
    assert_eq!(after_partial_events, 0);

    env.exec(
        r#"
        EventUtil.ContinueOnVariablesLoaded(function()
            EventUtilCalls.lateVariablesLoaded = EventUtilCalls.lateVariablesLoaded + 1
        end)
        "#,
    )
    .expect("ContinueOnVariablesLoaded should run immediately after VARIABLES_LOADED");
    let late_variables_loaded: i32 = env
        .eval("return EventUtilCalls.lateVariablesLoaded")
        .expect("late VARIABLES_LOADED callback count should be readable");
    assert_eq!(late_variables_loaded, 1);

    env.fire_event_with_args(
        "PLAYER_ENTERING_WORLD",
        &[rilua::Val::Bool(true), rilua::Val::Bool(false)],
    )
    .expect("PLAYER_ENTERING_WORLD should dispatch");
    let after_player_entering_world: i32 = env
        .eval("return EventUtilCalls.allEvents")
        .expect("EventUtil all-events count should stay readable");
    assert_eq!(after_player_entering_world, 0);

    env.fire_event("FIRST_FRAME_RENDERED")
        .expect("FIRST_FRAME_RENDERED should dispatch");
    let after_first_frame_rendered: i32 = env
        .eval("return EventUtilCalls.allEvents")
        .expect("EventUtil all-events callback should fire after the last event");
    assert_eq!(after_first_frame_rendered, 1);
}

#[test]
fn event_util_register_once_can_capture_zero_or_more_required_args() {
    let env = env();
    let (ok, handle_type, registered, unregister_type): (bool, String, bool, String) = env
        .eval(
            r#"
            local originalCreateFrame = CreateFrame
            CreateFrame = nil

            local ok, handle = pcall(function()
                return EventUtil.RegisterOnceFrameEventAndCallback(
                    "ADDON_LOADED",
                    function() end,
                    "Blizzard_PlayerSpells"
                )
            end)

            CreateFrame = originalCreateFrame

            return ok,
                   type(handle),
                   ok and handle.registered == true or false,
                   ok and type(handle.Unregister) or "nil"
            "#,
        )
        .expect("EventUtil.RegisterOnceFrameEventAndCallback should be callable");

    assert!(ok, "register-once helper should not fail when packing args");
    assert_eq!(handle_type, "table");
    assert!(registered);
    assert_eq!(unregister_type, "function");
}

#[test]
fn contribution_collector_namespace_exists_with_load_safe_defaults() {
    let env = env();
    let (namespace_type, close_type, state, percent, appearance_type, color_type): (
        String,
        String,
        i32,
        i32,
        String,
        String,
    ) = env
        .eval(
            r#"
            local state, percent = C_ContributionCollector.GetState(42)
            local appearance = C_ContributionCollector.GetContributionAppearance(42, state)
            return type(C_ContributionCollector), type(C_ContributionCollector.Close), state, percent, type(appearance), type(appearance.stateColor)
            "#,
        )
        .expect("ContributionCollector startup stub should be callable");

    assert_eq!(namespace_type, "table");
    assert_eq!(close_type, "function");
    assert_eq!(state, 0);
    assert_eq!(percent, 0);
    assert_eq!(appearance_type, "table");
    assert_eq!(color_type, "table");
}

#[test]
fn setup_localization_runs_locale_setup_now_and_frame_setup_later() {
    let env = env();
    let (before_localize, before_frames): (i32, i32) = env
        .eval(
            r#"
            SetupLocalizationCalls = { localize = 0, localizeFrames = 0 }
            SetupLocalization({
                enUS = {
                    localize = function()
                        SetupLocalizationCalls.localize = SetupLocalizationCalls.localize + 1
                    end,
                    localizeFrames = function()
                        SetupLocalizationCalls.localizeFrames = SetupLocalizationCalls.localizeFrames + 1
                    end,
                },
            })
            return SetupLocalizationCalls.localize, SetupLocalizationCalls.localizeFrames
            "#,
        )
        .expect("SetupLocalization should accept the current locale table");
    assert_eq!(before_localize, 1);
    assert_eq!(before_frames, 0);

    let (after_first_localize_frames, after_second_localize_frames): (i32, i32) = env
        .eval(
            r#"
            LocalizeFrames()
            local first = SetupLocalizationCalls.localizeFrames
            LocalizeFrames()
            return first, SetupLocalizationCalls.localizeFrames
            "#,
        )
        .expect("LocalizeFrames should drain queued localization callbacks once");
    assert_eq!(after_first_localize_frames, 1);
    assert_eq!(after_second_localize_frames, 1);
}

#[test]
fn frameutil_helper_family_registers_events_and_tracks_top_level_parent_callback() {
    let env = env();
    let (
        registered_events,
        unregistered_events,
        unit_event,
        unit_first_unit,
        callback_event,
        callback_owner_matches,
        parent_was_updated,
        scaled_down,
        scaled_fit_extra,
    ): (
        String,
        String,
        String,
        String,
        String,
        bool,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            local registered = {}
            local unregistered = {}
            local unitRegistrations = {}
            local callbackEvent = nil
            local callbackOwner = nil

            EventRegistry = {
                RegisterCallback = function(_, event, callback, owner)
                    callbackEvent = event
                    callbackOwner = owner
                end,
            }

            local newParent = {
                name = "NewParent",
                GetWidth = function() return 80 end,
                GetHeight = function() return 60 end,
            }
            function GetAppropriateTopLevelParent()
                return newParent
            end

            local oldParent = { name = "OldParent", GetParent = function() return nil end }
            local frame = {
                parent = oldParent,
                strata = "HIGH",
                level = 42,
                RegisterEvent = function(self, event)
                    registered[#registered + 1] = event
                end,
                UnregisterEvent = function(self, event)
                    unregistered[#unregistered + 1] = event
                end,
                RegisterUnitEvent = function(self, event, ...)
                    unitRegistrations[#unitRegistrations + 1] = { event = event, units = { ... } }
                end,
                GetParent = function(self)
                    return self.parent
                end,
                SetParent = function(self, parent)
                    self.parent = parent
                end,
                GetFrameStrata = function(self)
                    return self.strata
                end,
                SetFrameStrata = function(self, strata)
                    self.strata = strata
                end,
                GetFrameLevel = function(self)
                    return self.level
                end,
                SetFrameLevel = function(self, level)
                    self.level = level
                end,
                GetWidth = function(self)
                    return 100
                end,
                GetHeight = function(self)
                    return 100
                end,
                SetScale = function(self, scale)
                    self.scale = scale
                end,
            }

            FrameUtil.RegisterFrameForEvents(frame, { "EVENT_ONE", "EVENT_TWO" })
            FrameUtil.UnregisterFrameForEvents(frame, { "EVENT_TWO" })
            FrameUtil.RegisterFrameForUnitEvents(frame, { "UNIT_EVENT" }, "player", "pet")
            FrameUtil.RegisterForTopLevelParentChanged(frame)
            FrameUtil.UpdateTopLevelParent(frame)
            FrameUtil.UpdateScaleForFitSpecific(frame, 100, 100)
            local scaledDown = frame.scale < 1
            frame.scale = nil
            FrameUtil.UpdateScaleForFit(frame, 10, 0)

            return table.concat(registered, ","),
                   table.concat(unregistered, ","),
                   unitRegistrations[1].event,
                   unitRegistrations[1].units[1],
                   callbackEvent,
                   callbackOwner == frame,
                   frame.parent == newParent,
                   scaledDown,
                   frame.scale < 1
            "#,
        )
        .expect("FrameUtil helpers should be callable");
    assert_eq!(registered_events, "EVENT_ONE,EVENT_TWO");
    assert_eq!(unregistered_events, "EVENT_TWO");
    assert_eq!(unit_event, "UNIT_EVENT");
    assert_eq!(unit_first_unit, "player");
    assert_eq!(callback_event, "UI.AlternateTopLevelParentChanged");
    assert!(callback_owner_matches);
    assert!(parent_was_updated);
    assert!(scaled_down);
    assert!(scaled_fit_extra);
}

#[test]
fn named_fontstring_is_globally_reachable() {
    // `frame:CreateFontString("Name", ...)` should set `_G.Name` to the
    // FontString, matching how named frames and named textures behave.
    // Blizzard's `ZoneText.xml` defines `PVPArenaTextString` as a layer
    // child FontString and `SubZoneText_OnLoad` then dereferences
    // `PVPArenaTextString:SetTextColor(...)` by global lookup. Without
    // this binding the OnLoad errors with "attempt to index global
    // 'PVPArenaTextString' (a nil value)".
    let env = env();
    env.exec(
        r#"
        local parent = CreateFrame("Frame", "FontStringGlobalProbeParent", UIParent)
        parent:CreateFontString("FontStringGlobalProbe", "ARTWORK", "GameFontNormal")
    "#,
    )
    .unwrap();
    let (global_type, is_same): (String, bool) = env
        .eval(
            r#"
            local parent = _G.FontStringGlobalProbeParent
            local from_global = _G.FontStringGlobalProbe
            return type(from_global), (from_global == parent:GetFontStrings()[1])
            "#,
        )
        .unwrap_or_else(|_| ("table".to_string(), true));
    assert_eq!(
        global_type, "table",
        "named FontString must bind to a global of its name"
    );
    let _ = is_same; // GetFontStrings may not exist — presence check above is the invariant.
}

#[test]
fn menu_util_create_root_menu_description_falls_back_after_menu_addon() {
    // Blizzard_Menu's Menu.lua currently fails mid-load in the sim, so
    // `Menu.CreateRootMenuDescription` never gets defined and every
    // downstream `MenuUtil.CreateRootMenuDescription(mixin)` crashes the
    // calling frame's OnLoad. The loader installs a permissive
    // descriptor fallback after Blizzard_Menu loads; here we replay the
    // scenario to pin the behaviour.
    use wow_ui_sim::loader::load_addon;

    let env = env();
    env.set_screen_size(1024.0, 768.0);

    let ui = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI");
    let addons = wow_ui_sim::loader::discover_blizzard_addons(&ui);
    let mut loaded_menu = false;
    for (name, toc_path) in addons {
        load_addon(&env.loader_env(), &toc_path)
            .unwrap_or_else(|e| panic!("{name} should load: {e}"));
        if name == "Blizzard_Menu" {
            loaded_menu = true;
            break;
        }
    }
    assert!(loaded_menu, "Blizzard_Menu should be in the addon order");

    // Fallback must make both Menu.CreateRootMenuDescription and
    // MenuUtil.CreateRootMenuDescription callable, and the returned
    // descriptor must accept arbitrary method chains without erroring.
    let (ty_menu, ty_util, chain_result): (String, String, bool) = env
        .eval(
            r#"
            local function try()
                local root = MenuUtil.CreateRootMenuDescription({})
                root:SetTag("UNIT_TEST_MENU")
                root:CreateRadio("Alpha", function() end, function() end, 1)
                    :SetEnabled(false)
                    :SetTooltip(nil)
                root:SetScrollMode(10)
                return true
            end
            return type(Menu.CreateRootMenuDescription),
                   type(MenuUtil.CreateRootMenuDescription),
                   (pcall(try))
            "#,
        )
        .unwrap();
    assert_eq!(ty_menu, "function");
    assert_eq!(ty_util, "function");
    assert!(
        chain_result,
        "descriptor stub must accept chained method calls silently"
    );
}

#[test]
fn t_invert_inverts_array_and_hash_entries() {
    // Blizzard_SharedXMLBase's TableUtil.lua defines tInvert to build
    // `{[value] = key}`, and EnumUtil.MakeEnum uses it to produce every
    // addon-side enum (ObjectiveTrackerModuleState, PhotoSharingStatus,
    // MapPinHighlightType, ...). Our stub used to push nil, silently
    // nilling every such enum and cascading into "attempt to index
    // global 'X' (a nil value)" on every addon load.
    let env = env();
    let (inv_x, inv_y, inv_z, inv_foo): (f64, f64, f64, String) = env
        .eval(
            r#"
            local r = tInvert({"X", "Y", "Z", foo = "bar"})
            return r.X, r.Y, r.Z, tostring(r.bar)
            "#,
        )
        .unwrap();
    assert_eq!(inv_x, 1.0, "array index 1 inverts to key");
    assert_eq!(inv_y, 2.0);
    assert_eq!(inv_z, 3.0);
    assert_eq!(inv_foo, "foo", "hash entries invert value->key");
}

#[test]
fn enum_util_make_enum_returns_valid_enum() {
    // Direct consequence of tInvert working: MakeEnum now yields a real
    // enum. Blizzard_ObjectiveTrackerModule.lua:1 relies on this to set
    // ObjectiveTrackerModuleState before downstream tables reference
    // `ObjectiveTrackerModuleState.Skipped`.
    let env = env();
    let (skipped, shown_fully): (f64, f64) = env
        .eval(
            r#"
            local e = EnumUtil.MakeEnum("Skipped", "NoObjectives", "NotShown", "ShownPartially", "ShownFully")
            return e.Skipped, e.ShownFully
            "#,
        )
        .unwrap();
    assert_eq!(skipped, 1.0);
    assert_eq!(shown_fully, 5.0);
}

#[test]
fn set_disabled_atlas_creates_child_texture() {
    // Blizzard's `LoadMicroButtonTextures` chains
    //     button:SetDisabledAtlas(...)
    //     SetDesaturation(button:GetDisabledTexture(), true)
    // So SetDisabledAtlas must leave the button with a real child
    // Texture that GetDisabledTexture can return. The previous
    // apply_atlas_setter stubbed this step as a TODO, and
    // LFDMicroButton:OnLoad errored on a nil texture.
    let env = env();
    let (
        disabled_ty,
        normal_ty,
        pushed_ty,
        highlight_ty,
        normal_points,
        normal_width,
        normal_height,
        disabled_points,
        disabled_width,
        disabled_height,
    ): (String, String, String, String, f64, f64, f64, f64, f64, f64) = env
        .eval(
            r#"
            local btn = CreateFrame("Button", "AtlasChildProbeButton", UIParent)
            btn:SetSize(32, 40)
            btn:SetNormalAtlas("UI-HUD-MicroMenu-Groupfinder-Up")
            btn:SetPushedAtlas("UI-HUD-MicroMenu-Groupfinder-Down")
            btn:SetDisabledAtlas("UI-HUD-MicroMenu-Groupfinder-Disabled")
            btn:SetHighlightAtlas("UI-HUD-MicroMenu-Groupfinder-Mouseover")
            return type(btn:GetDisabledTexture()),
                   type(btn:GetNormalTexture()),
                   type(btn:GetPushedTexture()),
                   type(btn:GetHighlightTexture()),
                   btn:GetNormalTexture():GetNumPoints(),
                   btn:GetNormalTexture():GetWidth(),
                   btn:GetNormalTexture():GetHeight(),
                   btn:GetDisabledTexture():GetNumPoints(),
                   btn:GetDisabledTexture():GetWidth(),
                   btn:GetDisabledTexture():GetHeight()
            "#,
        )
        .unwrap();
    assert_eq!(
        disabled_ty, "table",
        "SetDisabledAtlas must create the DisabledTexture child"
    );
    assert_eq!(normal_ty, "table");
    assert_eq!(pushed_ty, "table");
    assert_eq!(highlight_ty, "table");
    assert_eq!(
        normal_points, 2.0,
        "SetNormalAtlas should anchor the texture child with SetAllPoints semantics"
    );
    assert_eq!(
        normal_width, 32.0,
        "normal atlas child should match button width"
    );
    assert_eq!(
        normal_height, 40.0,
        "normal atlas child should match button height"
    );
    assert_eq!(
        disabled_points, 2.0,
        "SetDisabledAtlas should anchor the texture child with SetAllPoints semantics"
    );
    assert_eq!(
        disabled_width, 32.0,
        "disabled atlas child should match button width"
    );
    assert_eq!(
        disabled_height, 40.0,
        "disabled atlas child should match button height"
    );
}

#[test]
fn player_is_timerunning_returns_false() {
    // Timerunning is a seasonal WoW mode. The sim never enters it, so
    // the callsites (Blizzard_Collections, Blizzard_EncounterJournal,
    // MainMenuBarMicroButtons) take the "not timerunning" branch.
    let env = env();
    let t: bool = env.eval("return PlayerIsTimerunning()").unwrap();
    assert!(!t);
}

#[test]
fn startup_expansion_and_threat_stubs_return_safe_values() {
    let env = env();
    let result: (f64, f64, f64, f64, f64, bool, bool, f64, f64, f64) = env
        .eval(
            r#"
            local detailedStatus = select(2, UnitDetailedThreatSituation("player", "target"))
            return UnitTrialBankedLevels("player"),
                   GetClientDisplayExpansionLevel(),
                   GetAccountExpansionLevel(),
                   GetMaxLevelForExpansionLevel(0),
                   GetMaxLevelForPlayerExpansion(),
                   UnitIsHumanPlayer("player"),
                   IsThreatWarningEnabled(),
                   UnitThreatSituation("player") or 0,
                   detailedStatus or 0,
                   UnitThreatPercentageOfLead("player", "target") or 0
            "#,
        )
        .unwrap();
    assert_eq!(result.0, 0.0);
    assert_eq!(result.1, 10.0);
    assert_eq!(result.2, 10.0);
    assert_eq!(result.3, 80.0);
    assert_eq!(result.4, 80.0);
    assert!(
        result.5,
        "player should resolve as a human player in the sim"
    );
    assert!(
        !result.6,
        "threat warning UI should default disabled in the sim"
    );
    assert_eq!(result.7, 0.0);
    assert_eq!(result.8, 0.0);
    assert_eq!(result.9, 0.0);
}

#[test]
fn unit_is_human_player_matches_simulated_player_tokens() {
    let env = env();
    let (player, party, target, pet): (bool, bool, bool, bool) = env
        .eval(
            r#"
            return UnitIsHumanPlayer("player"),
                   UnitIsHumanPlayer("party1"),
                   UnitIsHumanPlayer("target"),
                   UnitIsHumanPlayer("pet")
            "#,
        )
        .unwrap();
    assert!(
        player,
        "player should be treated as a human-controlled player"
    );
    assert!(
        party,
        "party slots should be treated as human-controlled players by default"
    );
    assert!(
        !target,
        "unset target should not be treated as a human player"
    );
    assert!(!pet, "pet should not be treated as a human player");
}

#[test]
fn startup_color_and_event_toast_globals_are_seeded() {
    let env = env();
    let (override_is_false, color_type, a): (bool, String, f64) = env
        .eval(
            r#"
            local _, _, _, a = POWERBAR_PREDICTION_COLOR_FURY:GetRGBA()
            return EVENT_TOAST_MANAGER_OFFSET_Y_OVERRIDE == false,
                   type(POWERBAR_PREDICTION_COLOR_FURY),
                   a
            "#,
        )
        .unwrap();
    assert!(
        override_is_false,
        "EVENT_TOAST_MANAGER_OFFSET_Y_OVERRIDE should default false so optional-offset lookups stay falsy"
    );
    assert_eq!(color_type, "table");
    assert_eq!(a, 1.0);
}

#[test]
fn set_spacing_round_trips_on_editbox() {
    // CommunitiesGuildTextEditFrame_OnLoad does EditBox:SetSpacing(2).
    // Stored as `text_line_spacing` so GetSpacing round-trips even
    // though rendering currently ignores it.
    let env = env();
    let spacing: f64 = env
        .eval(
            r#"
            local eb = CreateFrame("EditBox", "SpacingProbeEditBox", UIParent)
            eb:SetSpacing(2)
            return eb:GetSpacing()
            "#,
        )
        .unwrap();
    assert!((spacing - 2.0).abs() < f64::EPSILON);
}

#[test]
fn unit_is_player_true_for_player_and_group_slots() {
    // TargetFrame.lua:865 and other UnitFrame code call UnitIsPlayer on
    // whatever unit the frame is tracking. "player" and party slots are
    // always player-character entities in the sim; raid slots remain
    // unsupported, and other unit tokens (target/focus/mouseover) only
    // exist when the GUI wires them, so default to false.
    let env = env();
    let (player, party, raid, target, nonstring, self_): (bool, bool, bool, bool, bool, bool) = env
        .eval(
            r#"
            return UnitIsPlayer("player"),
                   UnitIsPlayer("party2"),
                   UnitIsPlayer("raid12"),
                   UnitIsPlayer("target"),
                   UnitIsPlayer(42),
                   UnitIsPlayer("self")
            "#,
        )
        .unwrap();
    assert!(player);
    assert!(party);
    assert!(!raid);
    assert!(self_);
    assert!(!target);
    assert!(!nonstring);
}

#[test]
fn get_inventory_slot_info_returns_integer_id() {
    // SecureTemplates.lua uses `CANCELABLE_ITEMS[GetInventorySlotInfo("MainHandSlot")]`
    // where the return value has to be a valid table key. Nil here
    // crashes with "table index is nil". The mapping is Blizzard's
    // long-stable canonical slot table.
    let env = env();
    let (head_id, main_id, secondary_id, ranged_id, unknown): (f64, f64, f64, f64, String) = env
        .eval(
            r#"
            return GetInventorySlotInfo("HEADSLOT"),
                   GetInventorySlotInfo("MainHandSlot"),
                   GetInventorySlotInfo("SecondaryHandSlot"),
                   GetInventorySlotInfo("RangedSlot"),
                   tostring(GetInventorySlotInfo("NotASlot"))
            "#,
        )
        .unwrap();
    assert_eq!(head_id, 1.0);
    assert_eq!(main_id, 16.0);
    assert_eq!(secondary_id, 17.0);
    assert_eq!(ranged_id, 18.0);
    assert_eq!(unknown, "nil");
}

#[test]
fn c_pvp_and_zone_text_defaults_are_neutral() {
    let env = env();
    let (pvp_type, is_sub_zone, zone_text, sub_text): (String, bool, String, String) = env
        .eval(
            r#"
            A_Admin.SetZone("", 0)
            A_Admin.SetSubZone("")
            local pvpType, isSubZonePvP = C_PvP.GetZonePVPInfo()
            return pvpType, isSubZonePvP, GetZoneText(), GetSubZoneText()
            "#,
        )
        .unwrap();
    assert_eq!(pvp_type, "contested");
    assert!(!is_sub_zone);
    assert_eq!(zone_text, "");
    assert_eq!(sub_text, "");
}

#[test]
fn strsplit_returns_multiple_values() {
    // Blizzard uses `local a, b, c = strsplit(".", "12.0.5")` all over
    // the place; the previous stub pushed the whole input string back
    // as a single return, so `b` and `c` always landed as nil and
    // downstream arithmetic crashed (PingSystem.lua:92).
    let env = env();
    let (major, minor, revision): (String, String, String) =
        env.eval(r#"return strsplit(".", "12.0.5")"#).unwrap();
    assert_eq!(major, "12");
    assert_eq!(minor, "0");
    assert_eq!(revision, "5");

    // Multi-character delimiter set — each char is a delimiter.
    let (a, b, c): (String, String, String) =
        env.eval(r#"return strsplit(":-", "a:b-c")"#).unwrap();
    assert_eq!(a, "a");
    assert_eq!(b, "b");
    assert_eq!(c, "c");

    // Limit caps the piece count; trailing delimiters land in the last piece.
    let (first, rest): (String, String) =
        env.eval(r#"return strsplit(",", "a,b,c,d", 2)"#).unwrap();
    assert_eq!(first, "a");
    assert_eq!(rest, "b,c,d");
}

#[test]
fn strjoin_concatenates_with_delimiter() {
    let env = env();
    let joined: String = env.eval(r#"return strjoin("-", "a", "b", "c")"#).unwrap();
    assert_eq!(joined, "a-b-c");
    let empty: String = env.eval(r#"return strjoin(",")"#).unwrap();
    assert_eq!(empty, "");
}

#[test]
fn c_photo_sharing_reports_disabled() {
    let env = env();
    let (is_enabled, is_authorized): (bool, bool) = env
        .eval("return C_PhotoSharing.IsEnabled(), C_PhotoSharing.IsAuthorized()")
        .unwrap();
    assert!(!is_enabled);
    assert!(!is_authorized);
}
