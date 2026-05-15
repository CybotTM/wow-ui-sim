//! Startup-time global functions (UnitPower, GetTime, ACTIONBAR_HOTKEY_FONT_COLOR,
//! social/LFG, presence and return-shape checks).

use super::super::*;

#[test]
fn test_startup_utility_globals_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        get_text_ty,
        get_text_value,
        game_time_ty,
        hour,
        minute,
        trial_ty,
        trial_value,
        restricted_ty,
        restricted_value,
    ): (String, String, String, i64, i64, String, bool, String, bool) = env
        .eval(
            r#"
            local hour, minute = GetGameTime()
            return type(GetText),
                GetText("ERR_FAKE_TOKEN"),
                type(GetGameTime),
                hour,
                minute,
                type(IsTrialAccount),
                IsTrialAccount(),
                type(IsRestrictedAccount),
                IsRestrictedAccount()
            "#,
        )
        .unwrap();

    assert_eq!(get_text_ty, "function");
    assert_eq!(get_text_value, "ERR_FAKE_TOKEN");
    assert_eq!(game_time_ty, "function");
    assert!((0..=23).contains(&hour));
    assert!((0..=59).contains(&minute));
    assert_eq!(trial_ty, "function");
    assert!(!trial_value);
    assert_eq!(restricted_ty, "function");
    assert!(!restricted_value);

    let (
        stream_ty,
        stream_value,
        callstack_ty,
        callstack_height,
        arena_ty,
        arena_specs,
        kiosk_ty,
        kiosk_enabled_ty,
        kiosk_enabled,
    ): (String, i64, String, i64, String, i64, String, String, bool) = env
        .eval(
            r#"
            return type(GetFileStreamingStatus),
                GetFileStreamingStatus(),
                type(GetErrorCallstackHeight),
                GetErrorCallstackHeight(),
                type(GetNumArenaOpponentSpecs),
                GetNumArenaOpponentSpecs(),
                type(Kiosk),
                type(Kiosk and Kiosk.IsEnabled),
                Kiosk and Kiosk.IsEnabled and Kiosk.IsEnabled() or false
            "#,
        )
        .unwrap();

    assert_eq!(stream_ty, "function");
    assert_eq!(stream_value, 0);
    assert_eq!(callstack_ty, "function");
    assert_eq!(callstack_height, 0);
    assert_eq!(arena_ty, "function");
    assert_eq!(arena_specs, 0);
    assert_eq!(kiosk_ty, "table");
    assert_eq!(kiosk_enabled_ty, "function");
    assert!(!kiosk_enabled);
}
#[test]
fn test_startup_service_globals_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (background_ty, background_status, debugstack_ty, debugstack_value, issecure_ty, secure): (
        String,
        i64,
        String,
        String,
        String,
        bool,
    ) = env
        .eval(
            r#"
            return type(GetBackgroundLoadingStatus),
                GetBackgroundLoadingStatus(),
                type(debugstack),
                debugstack(1),
                type(issecure),
                issecure()
            "#,
        )
        .unwrap();

    assert_eq!(background_ty, "function");
    assert_eq!(background_status, 0);
    assert_eq!(debugstack_ty, "function");
    assert!(
        debugstack_value.contains("in main chunk") || debugstack_value.contains("in function"),
        "expected debugstack output to contain stack frames, got: {debugstack_value:?}"
    );
    assert_eq!(issecure_ty, "function");
    assert!(secure);
}

#[test]
fn debugstack_uses_wow_bracketed_file_locations() {
    let env = WowLuaEnv::new().unwrap();

    let result = env.exec_named(
        r#"
        local function current_folder()
            local stack = debugstack()
            __debugstack_stack = stack
            local _, _, luafilepath = string.find(stack, "[%[](.-)[%]]")
            local i = 1
            local lastPart
            while string.find(luafilepath, "([/].+)", i) do
                local startPoint
                startPoint, _, lastPart = string.find(luafilepath, "([/].+)", i)
                i = startPoint + 1
            end
            return string.gsub(luafilepath, lastPart, "")
        end
        __debugstack_folder = current_folder()
        "#,
        "@Interface/AddOns/Angleur/angTemplates/LegolandoTemplates/Legolando_MouseScanAnim/Legolando_MouseScanAnim.lua",
    );
    if let Err(error) = result {
        let stack: String = env.eval("__debugstack_stack").unwrap_or_default();
        panic!("debugstack path extraction failed: {error}; stack={stack:?}");
    }

    let folder: String = env.eval("__debugstack_folder").unwrap();
    assert_eq!(
        folder,
        "Interface/AddOns/Angleur/angTemplates/LegolandoTemplates/Legolando_MouseScanAnim"
    );
}
#[test]
fn test_old_stack_startup_globals_exist_on_rilua_path() {
    let env = WowLuaEnv::new().unwrap();
    let (
        actionbar_color_method_ty,
        actionbar_r,
        actionbar_g,
        actionbar_b,
        red_rgba_ty,
        raid_class_rgb_ty,
        class_color_ty,
        class_color_rgb_ty,
        spec_get_ty,
        spec_info_ty,
        spec_class_ty,
        model_info_ty,
        timerunning_ty,
        timerunning_season,
        unit_casting_ty,
        unit_casting_nil,
        active_spec_id,
        active_spec_name,
    ): (
        String,
        f64,
        f64,
        f64,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        i64,
        String,
        bool,
        i64,
        String,
    ) = env
        .eval(
            r#"
            local hotkeyR, hotkeyG, hotkeyB = ACTIONBAR_HOTKEY_FONT_COLOR:GetRGB()
            local classColor = C_ClassColor.GetClassColor("PALADIN")
            local activeSpecIndex = C_SpecializationInfo.GetSpecialization()
            local activeSpecID, activeSpecName = C_SpecializationInfo.GetSpecializationInfo(activeSpecIndex)
            return type(ACTIONBAR_HOTKEY_FONT_COLOR.GetRGB),
                hotkeyR, hotkeyG, hotkeyB,
                type(RED_FONT_COLOR.GetRGBA),
                type(RAID_CLASS_COLORS.WARRIOR.GetRGB),
                type(C_ClassColor.GetClassColor),
                type(classColor and classColor.GetRGB),
                type(C_SpecializationInfo.GetSpecialization),
                type(C_SpecializationInfo.GetSpecializationInfo),
                type(C_SpecializationInfo.GetClassIDFromSpecID),
                type(C_ModelInfo.GetModelSceneInfoByID),
                type(PlayerGetTimerunningSeasonID),
                PlayerGetTimerunningSeasonID(),
                type(UnitCastingInfo),
                UnitCastingInfo("player") == nil,
                activeSpecID,
                activeSpecName
            "#,
        )
        .unwrap();

    assert_eq!(actionbar_color_method_ty, "function");
    assert_eq!((actionbar_r, actionbar_g, actionbar_b), (0.6, 0.6, 0.6));
    assert_eq!(red_rgba_ty, "function");
    assert_eq!(raid_class_rgb_ty, "function");
    assert_eq!(class_color_ty, "function");
    assert_eq!(class_color_rgb_ty, "function");
    assert_eq!(spec_get_ty, "function");
    assert_eq!(spec_info_ty, "function");
    assert_eq!(spec_class_ty, "function");
    assert_eq!(model_info_ty, "function");
    assert_eq!(timerunning_ty, "function");
    assert_eq!(timerunning_season, 0);
    assert_eq!(unit_casting_ty, "function");
    assert!(unit_casting_nil);
    assert!(active_spec_id > 0);
    assert!(!active_spec_name.is_empty());
}
#[test]
fn test_old_stack_power_lfg_and_cleanup_globals_exist_on_rilua_path() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("C_WowTokenSecure = nil").unwrap();
    env.exec("LE_GAME_ERR_SPELL_COOLDOWN = nil").unwrap();
    env.restore_post_cleanup_globals();

    let (
        unit_power,
        unit_power_with_type,
        unit_power_max,
        power_type,
        power_token,
        lfg_lfd_ty,
        lfg_lfr_ty,
        can_use_lfd,
        can_use_lfr,
        faction_rgba_ty,
        token_secure_ty,
        token_count,
        price_lock_duration,
        spell_cooldown_err,
    ): (
        i64,
        i64,
        i64,
        i64,
        String,
        String,
        String,
        bool,
        bool,
        String,
        String,
        i64,
        i64,
        i64,
    ) = env
        .eval(
            r#"
            local ptype, ptoken = UnitPowerType("player")
            local canUseLFD = C_LFGInfo.CanPlayerUseLFD()
            local canUseLFR = C_LFGInfo.CanPlayerUseLFR()
            return UnitPower("player"),
                UnitPower("player", 0),
                UnitPowerMax("player"),
                ptype,
                ptoken,
                type(C_LFGInfo.CanPlayerUseLFD),
                type(C_LFGInfo.CanPlayerUseLFR),
                canUseLFD,
                canUseLFR,
                type(FACTION_RED_COLOR.GetRGBA),
                type(C_WowTokenSecure.GetTokenCount),
                C_WowTokenSecure.GetTokenCount(),
                C_WowTokenSecure.GetPriceLockDuration(),
                LE_GAME_ERR_SPELL_COOLDOWN
            "#,
        )
        .unwrap();

    assert_eq!(unit_power, 50_000);
    assert_eq!(unit_power_with_type, 50_000);
    assert_eq!(unit_power_max, 100_000);
    assert_eq!(power_type, 0);
    assert_eq!(power_token, "MANA");
    assert_eq!(lfg_lfd_ty, "function");
    assert_eq!(lfg_lfr_ty, "function");
    assert!(can_use_lfd);
    assert!(can_use_lfr);
    assert_eq!(faction_rgba_ty, "function");
    assert_eq!(token_secure_ty, "function");
    assert_eq!(token_count, 2);
    assert_eq!(price_lock_duration, 900);
    assert_eq!(spell_cooldown_err, 61);
}
#[test]
fn test_startup_social_and_lfg_globals_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        tutorial_ty,
        tutorial_flagged,
        lfg_ty,
        has_active_entry,
        premade_style,
        search_result_ty,
        queue_ty,
        group_count,
        queue_config_ty,
    ): (String, bool, String, bool, i64, String, String, i64, String) = env
        .eval(
            r#"
            local groups = C_SocialQueue.GetAllGroups()
            local config = C_SocialQueue.GetConfig()
            return type(IsTutorialFlagged),
                IsTutorialFlagged(42),
                type(C_LFGList.GetApplications),
                C_LFGList.HasActiveEntryInfo(),
                C_LFGList.GetPremadeGroupFinderStyle(),
                type(C_LFGList.GetSearchResultInfo(7)),
                type(C_SocialQueue.GetAllGroups),
                #groups,
                type(config)
            "#,
        )
        .unwrap();

    assert_eq!(tutorial_ty, "function");
    assert!(!tutorial_flagged);
    assert_eq!(lfg_ty, "function");
    assert!(!has_active_entry);
    assert_eq!(premade_style, 0);
    assert_eq!(search_result_ty, "table");
    assert_eq!(queue_ty, "function");
    assert_eq!(group_count, 0);
    assert_eq!(queue_config_ty, "table");
}
#[test]
fn test_startup_time_and_service_globals_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        get_time_ty,
        now,
        action_info_ty,
        action_type_value,
        action_id,
        web_ticket_ty,
        web_ticket_value_ty,
        dungeon_ty,
        dungeon_id,
        unit_in_vehicle_ty,
        in_vehicle,
        roles_ty,
        can_tank,
        can_heal,
        can_dps,
    ): (
        String,
        f64,
        String,
        String,
        i64,
        String,
        String,
        String,
        i64,
        String,
        bool,
        String,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            local actionType, actionId = GetActionInfo(1)
            local canTank, canHeal, canDps = UnitGetAvailableRoles("player")
            return type(GetTime),
                GetTime(),
                type(GetActionInfo),
                actionType,
                actionId,
                type(GetWebTicket),
                type(GetWebTicket()),
                type(GetDungeonDifficultyID),
                GetDungeonDifficultyID(),
                type(UnitInVehicle),
                UnitInVehicle("player"),
                type(UnitGetAvailableRoles),
                canTank,
                canHeal,
                canDps
            "#,
        )
        .unwrap();

    assert_eq!(get_time_ty, "function");
    assert!(now >= 0.0);
    assert_eq!(action_info_ty, "function");
    assert_eq!(action_type_value, "spell");
    assert!(action_id > 0);
    assert_eq!(web_ticket_ty, "function");
    assert_eq!(web_ticket_value_ty, "nil");
    assert_eq!(dungeon_ty, "function");
    assert_eq!(dungeon_id, 1);
    assert_eq!(unit_in_vehicle_ty, "function");
    assert!(!in_vehicle);
    assert_eq!(roles_ty, "function");
    assert!(can_tank);
    assert!(can_heal);
    assert!(can_dps);

    let (
        auth_ty,
        auth_succeeded,
        class_trial_ty,
        is_class_trial,
        logout_seconds,
        character_services_ty,
        has_trial_boost,
    ): (String, bool, String, bool, i64, String, bool) = env
        .eval(
            r#"
            return type(C_AuthChallenge.SetFrame),
                C_AuthChallenge.DidChallengeSucceed(),
                type(C_ClassTrial.IsClassTrialCharacter),
                C_ClassTrial.IsClassTrialCharacter(),
                C_ClassTrial.GetClassTrialLogoutTimeSeconds(),
                type(C_CharacterServices.HasRequiredBoostForClassTrial),
                C_CharacterServices.HasRequiredBoostForClassTrial()
            "#,
        )
        .unwrap();

    assert_eq!(auth_ty, "function");
    assert!(!auth_succeeded);
    assert_eq!(class_trial_ty, "function");
    assert!(!is_class_trial);
    assert_eq!(logout_seconds, 0);
    assert_eq!(character_services_ty, "function");
    assert!(!has_trial_boost);
}

#[test]
fn test_addon_startup_time_and_difficulty_globals_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        precise_ty,
        precise_now,
        server_time_ty,
        server_time,
        raid_difficulty_ty,
        raid_difficulty_id,
        legacy_raid_difficulty_ty,
        legacy_raid_difficulty_id,
    ): (String, f64, String, f64, String, i64, String, i64) = env
        .eval(
            r#"
            return type(GetTimePreciseSec),
                GetTimePreciseSec(),
                type(GetServerTime),
                GetServerTime(),
                type(GetRaidDifficultyID),
                GetRaidDifficultyID(),
                type(GetLegacyRaidDifficultyID),
                GetLegacyRaidDifficultyID()
            "#,
        )
        .unwrap();

    assert_eq!(precise_ty, "function");
    assert!(precise_now >= 0.0);
    assert_eq!(server_time_ty, "function");
    assert!(server_time > 1_700_000_000.0);
    assert_eq!(raid_difficulty_ty, "function");
    assert_eq!(raid_difficulty_id, 14);
    assert_eq!(legacy_raid_difficulty_ty, "function");
    assert_eq!(legacy_raid_difficulty_id, 3);
}

#[test]
fn test_addon_startup_frame_scale_and_merchant_namespaces_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (dpi_ty, dpi_scale, merchant_ty, merchant_item_ty, raid_locks_ty, encounter_complete): (
        String,
        f64,
        String,
        String,
        String,
        bool,
    ) = env
        .eval(
            r#"
            local itemInfo = C_MerchantFrame.GetItemInfo(1)
            return type(GetScreenDPIScale),
                GetScreenDPIScale(),
                type(C_MerchantFrame),
                type(itemInfo),
                type(C_RaidLocks),
                C_RaidLocks.IsEncounterComplete(1)
            "#,
        )
        .unwrap();

    assert_eq!(dpi_ty, "function");
    assert_eq!(dpi_scale, 1.0);
    assert_eq!(merchant_ty, "table");
    assert_eq!(merchant_item_ty, "table");
    assert_eq!(raid_locks_ty, "table");
    assert!(!encounter_complete);
}

#[test]
fn test_addon_startup_table_time_and_string_helpers_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        find_ty,
        found_index,
        found_value,
        secure_found_index,
        secure_found_value,
        secure_pair_count,
        difftime_ty,
        time_delta,
        parsed_time,
        colored,
        replaced,
        unmute_result,
    ): (
        String,
        i64,
        String,
        i64,
        String,
        i64,
        String,
        i64,
        i64,
        String,
        String,
        bool,
    ) = env
        .eval(
            r#"
            local index, value = FindInTableIf({"a", "b"}, function(v) return v == "b" end)
            local secureArray = SecureTypes.CreateSecureArray()
            secureArray:Insert("a")
            secureArray:Insert("b")
            local secureIndex, secureValue = secureArray:FindInTableIf(function(v) return v == "b" end)
            local securePairCount = 0
            for _ in pairs(secureArray) do
                securePairCount = securePairCount + 1
            end
            return type(FindInTableIf),
                index,
                value,
                secureIndex,
                secureValue,
                securePairCount,
                type(difftime),
                difftime(10, 3),
                time({ year = "2024", month = "6", day = "3", hour = "0", min = "41", sec = "7" }),
                ("Requires a reload"):SetColorOrange(),
                ("Hello {name}"):K_ReplaceVars({ name = "Palaky" }),
                UnmuteSoundFile(123)
            "#,
        )
        .unwrap();

    assert_eq!(find_ty, "function");
    assert_eq!(found_index, 2);
    assert_eq!(found_value, "b");
    assert_eq!(secure_found_index, 2);
    assert_eq!(secure_found_value, "b");
    assert_eq!(secure_pair_count, 2);
    assert_eq!(difftime_ty, "function");
    assert_eq!(time_delta, 7);
    assert!(parsed_time > 0);
    assert!(colored.starts_with("|c"));
    assert!(colored.ends_with("|r"));
    assert_eq!(replaced, "Hello Palaky");
    assert!(unmute_result);
}
