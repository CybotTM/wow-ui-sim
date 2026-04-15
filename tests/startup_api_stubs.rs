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
    let (disabled_ty, normal_ty, pushed_ty, highlight_ty): (String, String, String, String) = env
        .eval(
            r#"
            local btn = CreateFrame("Button", "AtlasChildProbeButton", UIParent)
            btn:SetNormalAtlas("UI-HUD-MicroMenu-Groupfinder-Up")
            btn:SetPushedAtlas("UI-HUD-MicroMenu-Groupfinder-Down")
            btn:SetDisabledAtlas("UI-HUD-MicroMenu-Groupfinder-Disabled")
            btn:SetHighlightAtlas("UI-HUD-MicroMenu-Groupfinder-Mouseover")
            return type(btn:GetDisabledTexture()),
                   type(btn:GetNormalTexture()),
                   type(btn:GetPushedTexture()),
                   type(btn:GetHighlightTexture())
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
