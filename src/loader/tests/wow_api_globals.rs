//! Tests for global WoW API functions and pre-created global frames.

use super::*;

// ---------------------------------------------------------------------------
// Global functions
// ---------------------------------------------------------------------------

#[test]
fn test_get_build_info() {
    let env = WowLuaEnv::new().unwrap();
    let (version, toc): (String, i32) = env
        .eval("local v,_,_,t = GetBuildInfo(); return v, t")
        .unwrap();
    assert!(!version.is_empty());
    assert!(toc > 0);
}

#[test]
fn test_get_locale() {
    let env = WowLuaEnv::new().unwrap();
    let locale: String = env.eval("return GetLocale()").unwrap();
    assert!(!locale.is_empty());
}

#[test]
fn test_unit_name_player() {
    let env = WowLuaEnv::new().unwrap();
    let name: String = env.eval("return UnitName('player')").unwrap();
    assert!(!name.is_empty());
}

#[test]
fn test_get_money() {
    let env = WowLuaEnv::new().unwrap();
    let money: i64 = env.eval("return GetMoney()").unwrap();
    assert!(money >= 0);
}

#[test]
fn test_in_combat_lockdown_false() {
    let env = WowLuaEnv::new().unwrap();
    let in_combat: bool = env.eval("return InCombatLockdown()").unwrap();
    assert!(!in_combat);
}

#[test]
fn test_wipe_function() {
    let (t, _) = load_test_lua(
        "test-wipe",
        r#"
        local t = {1, 2, 3, a = "b"}
        wipe(t)
        WIPE_LEN = #t
        WIPE_A_NIL = (t.a == nil)
    "#,
    );
    let len: i32 = t.env.eval("return WIPE_LEN").unwrap();
    assert_eq!(len, 0);
    t.assert_lua_true("return WIPE_A_NIL", "wipe should clear named keys");
}

#[test]
fn test_copy_table_deep() {
    let (t, _) = load_test_lua(
        "test-copytable",
        r#"
        local orig = {a = 1, b = {c = 2}}
        local copy = CopyTable(orig)
        COPY_A = copy.a
        COPY_BC = copy.b.c
        copy.a = 99
        ORIG_A = orig.a
    "#,
    );
    let copy_a: i32 = t.env.eval("return COPY_A").unwrap();
    assert_eq!(copy_a, 1);
    let copy_bc: i32 = t.env.eval("return COPY_BC").unwrap();
    assert_eq!(copy_bc, 2);
    let orig_a: i32 = t.env.eval("return ORIG_A").unwrap();
    assert_eq!(orig_a, 1, "original should be unmodified");
}

#[test]
fn test_strsplit() {
    let (t, _) = load_test_lua(
        "test-strsplit",
        r#"
        local a, b, c = strsplit(",", "one,two,three")
        SS_A, SS_B, SS_C = a, b, c
    "#,
    );
    t.assert_lua_str("return SS_A", "one");
    t.assert_lua_str("return SS_B", "two");
    t.assert_lua_str("return SS_C", "three");
}

#[test]
fn test_strtrim() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env.eval(r#"return strtrim("  hello  ")"#).unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_geterrorhandler() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return type(geterrorhandler())").unwrap();
    assert_eq!(ty, "function");
}

#[test]
fn test_hooksecurefunc() {
    let (t, _) = load_test_lua(
        "test-hooksecure",
        r#"
        local obj = { MyMethod = function() end }
        HOOK_CALLED = false
        hooksecurefunc(obj, "MyMethod", function() HOOK_CALLED = true end)
        obj:MyMethod()
    "#,
    );
    t.assert_lua_true("return HOOK_CALLED", "hook should fire");
}

#[test]
fn test_hooksecurefunc_on_frame_userdata() {
    let (t, _) = load_test_lua(
        "test-hooksecure-ud",
        r#"
        local f = CreateFrame("Frame", "HookSecureUDTest", UIParent)
        HOOK_CALLED = false
        hooksecurefunc(f, "SetAlpha", function() HOOK_CALLED = true end)
        f:SetAlpha(0.5)
    "#,
    );
    t.assert_lua_true("return HOOK_CALLED", "hook should fire on userdata frame");
}

#[test]
fn test_issecurevariable_on_frame_userdata() {
    let (t, _) = load_test_lua(
        "test-issecurevar-ud",
        r#"
        local f = CreateFrame("Frame", "IssecureVarUDTest", UIParent)
        -- issecurevariable(frame, "method") should not error on userdata
        local secure, taint = issecurevariable(f, "Show")
        SECURE_RESULT = secure
    "#,
    );
    t.assert_lua_true("return SECURE_RESULT", "native method should be secure");
}

#[test]
fn test_mixin() {
    let (t, _) = load_test_lua(
        "test-mixin",
        r#"
        local target = {}
        Mixin(target, {foo = 1, bar = "hello"})
        MIX_FOO = target.foo
        MIX_BAR = target.bar
    "#,
    );
    let foo: i32 = t.env.eval("return MIX_FOO").unwrap();
    assert_eq!(foo, 1);
    t.assert_lua_str("return MIX_BAR", "hello");
}

#[test]
fn test_global_functions_callable() {
    let env = WowLuaEnv::new().unwrap();
    for f in &[
        "BreakUpLargeNumbers",
        "PlaySound",
        "ReloadUI",
        "GetBindingKey",
        "SetOverrideBinding",
        "ClearOverrideBindings",
        "GetInventoryItemLink",
        "GetInventoryItemTexture",
        "GetInventorySlotInfo",
        "GetFramerate",
        "format",
        "strjoin",
    ] {
        let expr = format!("return type({})", f);
        let ty: String = env.eval(&expr).unwrap();
        assert_eq!(ty, "function", "{} should be function", f);
    }
}

// ---------------------------------------------------------------------------
// Global frames and tables
// ---------------------------------------------------------------------------

#[test]
fn test_uiparent_exists() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return UIParent:GetObjectType()").unwrap();
    assert_eq!(ty, "Frame");
}

#[test]
fn test_create_frame_exposes_core_event_methods() {
    let env = WowLuaEnv::new().unwrap();
    let registry_mt_type: String = {
        let mut lua = env.lua.borrow_mut();
        let state = lua.state_mut();
        match crate::lua_api::rilua_methods::registry_get(state, "__rilua_frame_mt") {
            rilua::Val::Table(_) => "table".to_string(),
            other => other.type_name().to_string(),
        }
    };
    let (set_forbidden, set_script, register_event, get_object_type, mt_type, mt_index_type, mt_set_forbidden, mt_get_object_type): (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ) = env
        .eval(
            r#"
            local f = CreateFrame("Frame")
            local mt = getmetatable(f)
            return type(f.SetForbidden), type(f.SetScript), type(f.RegisterEvent), type(f.GetObjectType),
                type(mt), type(mt and mt.__index), type(mt and mt.SetForbidden), type(mt and mt.GetObjectType)
            "#,
        )
        .unwrap();
    assert_eq!(
        (set_forbidden, set_script, register_event, get_object_type),
        (
            "function".to_string(),
            "function".to_string(),
            "function".to_string(),
            "function".to_string(),
        ),
        "frame surface mismatch: registry={registry_mt_type}, mt={mt_type}, mt.__index={mt_index_type}, mt.SetForbidden={mt_set_forbidden}, mt.GetObjectType={mt_get_object_type}",
    );
}

#[test]
fn test_create_texture_exposes_core_visual_methods() {
    let env = WowLuaEnv::new().unwrap();
    let (
        set_texture,
        set_color_texture,
        set_vertex_color,
        set_blend_mode,
        set_tex_coord,
        set_horiz_tile,
        set_vert_tile,
        set_texel_snapping_bias,
        get_texel_snapping_bias,
        set_snap_to_pixel_grid,
        set_desaturated_ty,
        is_desaturated_ty,
        get_desaturation_ty,
        bias,
        is_desaturated,
        desaturation,
    ): (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        f64,
        bool,
        f64,
    ) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            local tex = frame:CreateTexture()
            tex:SetTexture("Interface\\Buttons\\WHITE8X8")
            tex:SetVertexColor(0.1, 0.2, 0.3, 0.4)
            tex:SetBlendMode("ADD")
            tex:SetColorTexture(0.5, 0.6, 0.7, 0.8)
            tex:SetTexCoord(0, 1, 0, 1)
            tex:SetHorizTile(true)
            tex:SetVertTile(true)
            tex:SetTexelSnappingBias(0.25)
            tex:SetSnapToPixelGrid(true)
            tex:SetDesaturated(true)
            return type(tex.SetTexture), type(tex.SetColorTexture), type(tex.SetVertexColor),
                type(tex.SetBlendMode), type(tex.SetTexCoord), type(tex.SetHorizTile),
                type(tex.SetVertTile), type(tex.SetTexelSnappingBias),
                type(tex.GetTexelSnappingBias), type(tex.SetSnapToPixelGrid),
                type(tex.SetDesaturated), type(tex.IsDesaturated), type(tex.GetDesaturation),
                tex:GetTexelSnappingBias(), tex:IsDesaturated(), tex:GetDesaturation()
            "#,
        )
        .unwrap();
    for ty in [
        set_texture,
        set_color_texture,
        set_vertex_color,
        set_blend_mode,
        set_tex_coord,
        set_horiz_tile,
        set_vert_tile,
        set_texel_snapping_bias,
        get_texel_snapping_bias,
        set_snap_to_pixel_grid,
        set_desaturated_ty,
        is_desaturated_ty,
        get_desaturation_ty,
    ] {
        assert_eq!(ty, "function");
    }
    assert_eq!(bias, 0.25);
    assert!(is_desaturated);
    assert_eq!(desaturation, 1.0);
}

#[test]
fn test_set_attribute_fires_on_attribute_changed() {
    let env = WowLuaEnv::new().unwrap();
    let (name_ty, seen_name, value_ty, seen_value, stored_ty, stored_value): (
        String,
        String,
        String,
        String,
        String,
        String,
    ) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            local seenName, seenValue

            frame:SetScript("OnAttributeChanged", function(_, name, value)
                seenName = name
                seenValue = value
            end)

            frame:SetAttribute("count", 7)

            return type(seenName), tostring(seenName), type(seenValue), tostring(seenValue),
                type(frame:GetAttribute("count")), tostring(frame:GetAttribute("count"))
            "#,
        )
        .unwrap();

    assert_eq!(name_ty, "string");
    assert_eq!(seen_name, "count");
    assert_eq!(
        value_ty, "number",
        "seen_name={seen_name} seen_value={seen_value} stored_ty={stored_ty} stored_value={stored_value}"
    );
    assert_eq!(seen_value, "7");
    assert_eq!(stored_ty, "number");
    assert_eq!(stored_value, "7");
}

#[test]
fn test_global_unpack_exists() {
    let env = WowLuaEnv::new().unwrap();
    let (global_ty, table_ty, first, second, third): (String, String, i64, i64, i64) = env
        .eval(
            r#"
            local values = {11, 22, 33}
            return type(unpack), type(table.unpack), table.unpack(values)
            "#,
        )
        .unwrap();

    assert_eq!(global_ty, "function");
    assert_eq!(table_ty, "function");
    assert_eq!((first, second, third), (11, 22, 33));
}

#[test]
fn test_bootstrap_fills_existing_namespace_defaults() {
    let env = WowLuaEnv::new().unwrap();
    let (trade_ty, trade_value, quest_ty, quest_value, color_ty, hex_ty): (
        String,
        i64,
        String,
        bool,
        String,
        String,
    ) = env
        .eval(
            r#"
            local color = C_ColorOverrides.GetColorForQuality(0)
            return type(C_TradeSkillUI.GetProfessionSkillLineID),
                C_TradeSkillUI.GetProfessionSkillLineID(7),
                type(C_QuestLog.ReadyForTurnIn),
                C_QuestLog.ReadyForTurnIn(42),
                type(C_ColorOverrides.GetColorForQuality),
                type(color and color.GenerateHexColorMarkup)
            "#,
        )
        .unwrap();

    assert_eq!(trade_ty, "function");
    assert_eq!(trade_value, 7);
    assert_eq!(quest_ty, "function");
    assert!(!quest_value);
    assert_eq!(color_ty, "function");
    assert_eq!(hex_ty, "function");
}

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
fn test_startup_bootstrap_namespaces_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        chat_ty,
        replaced_message,
        chat_restricted,
        nav_ty,
        nav_distance,
        nav_frame_ty,
        token_ty,
        commerce_enabled,
        commerce_poll_seconds,
        commerce_balance_enabled,
    ): (
        String,
        String,
        bool,
        String,
        i64,
        String,
        String,
        bool,
        i64,
        bool,
    ) = env
        .eval(
            r#"
            local enabled, pollSeconds, balanceEnabled = C_WowTokenPublic.GetCommerceSystemStatus()
            return type(C_ChatInfo.PerformEmote),
                C_ChatInfo.ReplaceIconAndGroupExpressions("{rt1} hello"),
                C_ChatInfo.AreOutgoingAddonChatMessagesRestricted(),
                type(C_Navigation.GetDistance),
                C_Navigation.GetDistance(),
                type(C_Navigation.GetFrame()),
                type(C_WowTokenPublic.GetCommerceSystemStatus),
                enabled,
                pollSeconds,
                balanceEnabled
            "#,
        )
        .unwrap();

    assert_eq!(chat_ty, "function");
    assert_eq!(replaced_message, "{rt1} hello");
    assert!(!chat_restricted);
    assert_eq!(nav_ty, "function");
    assert_eq!(nav_distance, 0);
    assert_eq!(nav_frame_ty, "table");
    assert_eq!(token_ty, "function");
    assert!(!commerce_enabled);
    assert_eq!(commerce_poll_seconds, 0);
    assert!(!commerce_balance_enabled);

    let (market_price_ty, market_price, guaranteed_price_ty, guaranteed_price): (
        String,
        i64,
        String,
        i64,
    ) = env
        .eval(
            r#"
            return type(C_WowTokenPublic.GetCurrentMarketPrice),
                C_WowTokenPublic.GetCurrentMarketPrice(),
                type(C_WowTokenPublic.GetGuaranteedPrice),
                C_WowTokenPublic.GetGuaranteedPrice()
            "#,
        )
        .unwrap();

    assert_eq!(market_price_ty, "function");
    assert_eq!(market_price, 0);
    assert_eq!(guaranteed_price_ty, "function");
    assert_eq!(guaranteed_price, 0);
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
    assert_eq!(debugstack_value, "");
    assert_eq!(issecure_ty, "function");
    assert!(secure);
}

#[test]
fn test_startup_service_namespaces_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        voices_ty,
        voices_len,
        transcription_ty,
        transcription_allowed,
        calendar_ty,
        month_day,
        adjusted_day,
        reset_ty,
        reset_start,
    ): (String, i64, String, bool, String, i64, i64, String, i64) = env
        .eval(
            r#"
            local now = C_DateAndTime.GetCurrentCalendarTime()
            local tomorrow = C_DateAndTime.AdjustTimeByDays(now, 1)
            return type(C_VoiceChat.GetTtsVoices),
                #C_VoiceChat.GetTtsVoices(),
                type(C_VoiceChat.IsTranscriptionAllowed),
                C_VoiceChat.IsTranscriptionAllowed(),
                type(now),
                now.monthDay or 0,
                tomorrow.monthDay or 0,
                type(C_DateAndTime.GetWeeklyResetStartTime),
                C_DateAndTime.GetWeeklyResetStartTime()
            "#,
        )
        .unwrap();

    assert_eq!(voices_ty, "function");
    assert_eq!(voices_len, 0);
    assert_eq!(transcription_ty, "function");
    assert!(!transcription_allowed);
    assert_eq!(calendar_ty, "table");
    assert!(month_day > 0);
    assert_eq!(adjusted_day, month_day + 1);
    assert_eq!(reset_ty, "function");
    assert_eq!(reset_start, 0);

    let (
        shop_ty,
        shop_enabled,
        has_new_products_ty,
        has_new_products,
        token_secure_ty,
        price_lock_duration,
        token_count,
    ): (String, bool, String, bool, String, i64, i64) = env
        .eval(
            r#"
            return type(C_CatalogShop.IsShop2Enabled),
                C_CatalogShop.IsShop2Enabled(),
                type(C_CatalogShop.HasNewProducts),
                C_CatalogShop.HasNewProducts(),
                type(C_WowTokenSecure.GetPriceLockDuration),
                C_WowTokenSecure.GetPriceLockDuration(),
                C_WowTokenSecure.GetTokenCount()
            "#,
        )
        .unwrap();

    assert_eq!(shop_ty, "function");
    assert!(!shop_enabled);
    assert_eq!(has_new_products_ty, "function");
    assert!(!has_new_products);
    assert_eq!(token_secure_ty, "function");
    assert_eq!(price_lock_duration, 900);
    assert_eq!(token_count, 2);
}

#[test]
fn test_startup_runtime_method_and_namespace_gaps_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        same_group,
        group_text,
        group_order,
        group_categories_ty,
        groups_ty,
        item_button_scale_ty,
        count_scale,
        calculate_action_ty,
        action_slot,
        force_update_calls,
        private_warning_ty,
        pet_effects_count,
    ): (
        bool,
        String,
        i64,
        String,
        String,
        String,
        f64,
        String,
        i64,
        i64,
        String,
        i64,
    ) = env
        .eval(
            r#"
            local panel = CreateFrame("Frame")
            local firstGroup = panel:GetOrCreateGroup("advanced", 7)
            local secondGroup = panel:GetOrCreateGroup("advanced", 3)

            local itemButton = CreateFrame("Button")
            local count = itemButton:CreateFontString(nil, "OVERLAY")
            itemButton.Count = count
            itemButton:SetItemButtonScale(1.25)

            local actionButton = CreateFrame("Button")
            actionButton:SetID(3)

            local timerContainer = CreateFrame("Frame")
            local calls = 0
            timerContainer.activeTimers = {
                one = { OnUpdate = function() calls = calls + 1 end },
                two = { OnUpdate = function() calls = calls + 1 end },
            }
            timerContainer:ForceUpdateTimers()

            return firstGroup == secondGroup,
                firstGroup.groupText,
                firstGroup.order,
                type(firstGroup.categories),
                type(panel.groups),
                type(itemButton.SetItemButtonScale),
                count:GetScale(),
                type(actionButton.CalculateAction),
                actionButton:CalculateAction(),
                calls,
                type(C_UnitAuras.SetPrivateWarningTextAnchor),
                #{C_PetBattles.GetAllEffectNames()}
            "#,
        )
        .unwrap();

    assert!(same_group);
    assert_eq!(group_text, "advanced");
    assert_eq!(group_order, 7);
    assert_eq!(group_categories_ty, "table");
    assert_eq!(groups_ty, "table");
    assert_eq!(item_button_scale_ty, "function");
    assert_eq!(count_scale, 1.25);
    assert_eq!(calculate_action_ty, "function");
    assert_eq!(action_slot, 3);
    assert_eq!(force_update_calls, 2);
    assert_eq!(private_warning_ty, "function");
    assert_eq!(pet_effects_count, 0);
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
                C_WowTokenSecure.GetPriceLockDuration()
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
    assert_eq!(search_result_ty, "nil");
    assert_eq!(queue_ty, "function");
    assert_eq!(group_count, 0);
    assert_eq!(queue_config_ty, "table");
}

#[test]
fn test_text_runtime_helpers_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        font_object_ty,
        width_ty,
        text_to_fit_ty,
        hyperlink_ty,
        scroll_script_ty,
        frame_script_ty,
        width,
        hyperlinks_enabled,
    ): (String, String, String, String, String, String, f64, bool) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            local fontString = frame:CreateFontString()
            fontString:SetText("Runtime helper text")
            fontString:SetFontObjectsToTry("GameFontHighlightLarge", "GameFontHighlightSmall")
            fontString:SetTextToFit("Runtime helper text")

            local messageFrame = CreateFrame("MessageFrame")
            messageFrame:SetHyperlinksEnabled(true)

            local scrollFrame = CreateFrame("ScrollFrame")
            scrollFrame:SetScript("OnVerticalScroll", function() end)
            frame:SetScript("OnKeyDown", function() end)

            return type(fontString:GetFontObject()),
                type(fontString.GetUnboundedStringWidth),
                type(fontString.SetTextToFit),
                type(messageFrame.SetHyperlinksEnabled),
                type(scrollFrame:GetScript("OnVerticalScroll")),
                type(frame:GetScript("OnKeyDown")),
                fontString:GetUnboundedStringWidth(),
                messageFrame:GetHyperlinksEnabled()
            "#,
        )
        .unwrap();

    assert_eq!(font_object_ty, "string");
    assert_eq!(width_ty, "function");
    assert_eq!(text_to_fit_ty, "function");
    assert_eq!(hyperlink_ty, "function");
    assert_eq!(scroll_script_ty, "function");
    assert_eq!(frame_script_ty, "function");
    assert!(width > 0.0);
    assert!(hyperlinks_enabled);
}

#[test]
fn test_startup_time_and_service_globals_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (
        get_time_ty,
        now,
        action_info_ty,
        action_type_ty,
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
            local actionType = GetActionInfo(1)
            local canTank, canHeal, canDps = UnitGetAvailableRoles("player")
            return type(GetTime),
                GetTime(),
                type(GetActionInfo),
                type(actionType),
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
    assert_eq!(action_type_ty, "nil");
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
fn test_c_texture_exposes_atlas_lookup() {
    let env = WowLuaEnv::new().unwrap();
    let (exists_ty, exists, info_ty, width, height, tile_h, tile_v): (
        String,
        bool,
        String,
        i64,
        i64,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            local info = C_Texture.GetAtlasInfo("UI-Frame-InnerTopLeft")
            return type(C_Texture.GetAtlasExists),
                C_Texture.GetAtlasExists("UI-Frame-InnerTopLeft"),
                type(info),
                info and info.width or 0,
                info and info.height or 0,
                info and info.tilesHorizontally or false,
                info and info.tilesVertically or false
            "#,
        )
        .unwrap();

    assert_eq!(exists_ty, "function");
    assert!(exists);
    assert_eq!(info_ty, "table");
    assert!(width > 0);
    assert!(height > 0);
    assert!(!tile_h);
    assert!(!tile_v);
}

#[test]
fn test_animation_runtime_exposes_core_configuration_methods() {
    let env = WowLuaEnv::new().unwrap();
    let (group_method_ty, animation_duration_ty, animation_order_ty, finished_script_ty): (String, String, String, String) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            local group = frame:CreateAnimationGroup()
            local animation = group:CreateAnimation("Alpha")
            group:SetToFinalAlpha(true)
            animation:SetDuration(0.5)
            animation:SetOrder(2)
            group:SetScript("OnFinished", function() end)
            return type(group.SetToFinalAlpha), type(animation.SetDuration), type(animation.SetOrder), type(group:GetScript("OnFinished"))
            "#,
        )
        .unwrap();

    assert_eq!(group_method_ty, "function");
    assert_eq!(animation_duration_ty, "function");
    assert_eq!(animation_order_ty, "function");
    assert_eq!(finished_script_ty, "function");
}

#[test]
fn test_flipbook_animation_runtime_exposes_configuration_surface() {
    let env = WowLuaEnv::new().unwrap();
    let (
        set_frames_ty,
        get_frames_ty,
        get_columns_ty,
        get_width_ty,
        frames,
        columns,
        width,
        height,
    ): (String, String, String, String, i64, i64, f64, f64) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame")
            local group = frame:CreateAnimationGroup()
            local animation = group:CreateAnimation("FlipBook")
            animation:SetFlipBookRows(3)
            animation:SetFlipBookColumns(4)
            animation:SetFlipBookFrames(12)
            animation:SetFlipBookFrameWidth(64)
            animation:SetFlipBookFrameHeight(32)
            return type(animation.SetFlipBookFrames), type(animation.GetFlipBookFrames),
                type(animation.GetFlipBookColumns), type(animation.GetFlipBookFrameWidth),
                animation:GetFlipBookFrames(), animation:GetFlipBookColumns(),
                animation:GetFlipBookFrameWidth(), animation:GetFlipBookFrameHeight()
            "#,
        )
        .unwrap();

    assert_eq!(set_frames_ty, "function");
    assert_eq!(get_frames_ty, "function");
    assert_eq!(get_columns_ty, "function");
    assert_eq!(get_width_ty, "function");
    assert_eq!(frames, 12);
    assert_eq!(columns, 4);
    assert_eq!(width, 64.0);
    assert_eq!(height, 32.0);
}

#[test]
fn test_gamepad_cursor_bootstrap_functions_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (auto_ty, auto_value, set_ty): (String, bool, String) = env
        .eval(
            r#"
            return type(CanAutoSetGamePadCursorControl),
                CanAutoSetGamePadCursorControl(true),
                type(SetGamePadCursorControl)
            "#,
        )
        .unwrap();

    assert_eq!(auto_ty, "function");
    assert!(!auto_value);
    assert_eq!(set_ty, "function");
}

#[test]
fn test_unit_state_bootstrap_functions_exist() {
    let env = WowLuaEnv::new().unwrap();
    let (ghost_ty, ghost_value, dead_ty, dead_value): (String, bool, String, bool) = env
        .eval(
            r#"
            return type(UnitIsGhost), UnitIsGhost("player"), type(UnitIsDead), UnitIsDead("player")
            "#,
        )
        .unwrap();

    assert_eq!(ghost_ty, "function");
    assert!(!ghost_value);
    assert_eq!(dead_ty, "function");
    assert!(!dead_value);
}

#[test]
fn test_ui_special_frames_table() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return type(UISpecialFrames)").unwrap();
    assert_eq!(ty, "table");
}

// SOUNDKIT: from Blizzard_SharedXML/SoundKitConstants.lua
// Tested via Lua addon tests (run-tests).

#[test]
fn test_game_tooltip_methods() {
    let env = WowLuaEnv::new().unwrap();
    for m in &["SetOwner", "Show", "Hide"] {
        let expr = format!("return type(GameTooltip.{})", m);
        let ty: String = env.eval(&expr).unwrap();
        assert_eq!(ty, "function", "GameTooltip.{} should be function", m);
    }
}

#[test]
fn test_static_popup() {
    let env = WowLuaEnv::new().unwrap();
    let ty: String = env.eval("return type(StaticPopup_Show)").unwrap();
    assert_eq!(ty, "function");
    let ty2: String = env.eval("return type(StaticPopupDialogs)").unwrap();
    assert_eq!(ty2, "table");
}

// ContinuableContainer, ItemButtonUtil, ScrollUtil, CreateScrollBoxLinearView,
// MainMenuBarBackpackButton: all from Blizzard addon Lua/XML.
// Tested via Lua addon tests (run-tests).
