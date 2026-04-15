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
        bias,
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
            return type(tex.SetTexture), type(tex.SetColorTexture), type(tex.SetVertexColor),
                type(tex.SetBlendMode), type(tex.SetTexCoord), type(tex.SetHorizTile),
                type(tex.SetVertTile), type(tex.SetTexelSnappingBias),
                type(tex.GetTexelSnappingBias), type(tex.SetSnapToPixelGrid),
                tex:GetTexelSnappingBias()
            "#,
        )
        .unwrap();
    assert_eq!(
        (
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
        ),
        (
            "function".to_string(),
            "function".to_string(),
            "function".to_string(),
            "function".to_string(),
            "function".to_string(),
            "function".to_string(),
            "function".to_string(),
            "function".to_string(),
            "function".to_string(),
            "function".to_string(),
        )
    );
    assert_eq!(bias, 0.25);
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
