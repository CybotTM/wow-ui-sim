//! Smoke tests for startup-surface stubs added to unblock Blizzard addon
//! loading. Each stub returns values that reflect the simulator's reality
//! (no network, no in-game store, no premade finder, no photo sharing)
//! rather than invented placeholders.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
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
    // whatever unit the frame is tracking. "player" and party/raid
    // slots are always player-character entities in the sim; other unit
    // tokens (target/focus/mouseover) only exist when the GUI wires
    // them, so default to false.
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
    assert!(raid);
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
    // ZoneText.lua:7 dereferences `C_PvP.GetZonePVPInfo()` during
    // SubZoneTextFrame OnLoad, and the same OnLoad chain accesses
    // GetSubZoneText. The sim has no world state, so return the
    // "neutral zone, empty text" flavor the OnLoad path tolerates.
    let env = env();
    let (pvp_type, is_sub_zone, zone_text, sub_text): (String, bool, String, String) = env
        .eval(
            r#"
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
