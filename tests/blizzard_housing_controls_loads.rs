use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, find_toc_file, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}

fn housing_controls_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_HousingControls")
}

fn housing_controls_toc() -> PathBuf {
    housing_controls_dir().join("Blizzard_HousingControls.toc")
}

fn load_full_game_ui_with_housing_controls_lod() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    wow_ui_sim::xml::register_intrinsic_templates();

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("[load {name}] FAILED: {err}"));
    }

    env.apply_post_load_workarounds();
    fire_startup_events_for_screen(&env, ScreenKind::Game);

    load_addon(&env.loader_env(), &housing_controls_toc())
        .expect("Blizzard_HousingControls should load via explicit Rust loader call");

    env
}

#[test]
fn blizzard_housing_controls_find_toc_resolves_bare_variant() {
    let resolved = find_toc_file(&housing_controls_dir())
        .expect("Blizzard_HousingControls TOC should resolve");
    assert_eq!(
        resolved,
        housing_controls_toc(),
        "Blizzard_HousingControls ships exactly one bare TOC \
         (`Blizzard_HousingControls.toc`) — no flavor variants. The decor controls top-strip \
         only ships on retail (`## AllowLoadGameType: standard`) and uses the bare TOC suffix \
         that `find_toc_file` (src/loader/mod.rs:65) falls through to"
    );
}

#[test]
fn blizzard_housing_controls_toc_declares_lod_with_single_dependency() {
    let toc =
        TocFile::from_file(&housing_controls_toc()).expect("HousingControls TOC should parse");
    assert!(
        toc.is_load_on_demand(),
        "Blizzard_HousingControls declares `## LoadOnDemand: 1` — the decor-controls top-strip \
         only loads when the player enters a housing plot or house. Three independent triggers \
         pull it: `Blizzard_UIParent/Mainline/UIParent.lua:1622` calls \
         `C_AddOns.LoadAddOn(\"Blizzard_HousingControls\")` from a global event guarded by \
         `C_Housing.IsInsideHouseOrPlot()`; \
         `Blizzard_HousingEventHandler/Blizzard_HousingEventHandler.lua:285` calls it from \
         `HousingEventHandlerMixin:OnPlotEntered` after a HousingControlsFrame nil-check; \
         `Blizzard_HouseEditor/Blizzard_HouseEditor.toc:4` declares it as a regular \
         `## Dependencies` entry, so any explicit LoD load of HouseEditor pulls HousingControls \
         transitively"
    );
    assert!(
        !toc.is_load_first(),
        "Blizzard_HousingControls does not declare `## LoadFirst: 1` — LoadOnDemand precludes \
         any load-order priority"
    );
    assert!(
        !toc.is_secure_env(),
        "Blizzard_HousingControls does not declare `## UseSecureEnvironment` — runs in the \
         standard Lua environment"
    );
    assert_eq!(
        toc.dependencies(),
        vec!["Blizzard_HousingTemplates".to_string()],
        "Blizzard_HousingControls declares exactly one `## Dependencies:` entry — \
         Blizzard_HousingTemplates provides the `controls-frame` + `controls-frame-guest` + \
         `decor-controls-decoratemode-{{inactive,active,pressed}}` + \
         `decor-controls-settings-{{default,active,pressed}}` + \
         `decor-controls-exit-{{default,active,pressed}}` + \
         `decor-controls-houseinfo-{{default,active,pressed}}` + \
         `decor-controls-inspect-{{default,active,pressed}}` + `keybind-bg` + `keybind-bg_active` \
         atlas references plus the BaseHousingActionButtonTemplate / \
         BaseHousingModeButtonTemplate base button templates plus the \
         BaseHousingActionButtonMixin / BaseHousingModeButtonMixin base mixins, the \
         HousingFramesUtil module, and the HOUSING_CONTROL_PANEL_TITLE / WHITE_FONT_COLOR / \
         DARKGRAY_COLOR color references"
    );
}

#[test]
fn blizzard_housing_controls_toc_is_retail_only_and_omits_allow_load() {
    let toc =
        TocFile::from_file(&housing_controls_toc()).expect("HousingControls TOC should parse");
    let toc_text = std::fs::read_to_string(housing_controls_toc()).expect("TOC should read");
    assert!(
        toc_text.contains("## AllowLoadGameType: standard"),
        "Blizzard_HousingControls declares `## AllowLoadGameType: standard` — the decor \
         controls top-strip is a Midnight expansion feature that only ships on retail. \
         `is_game_type_restricted()` (src/toc.rs:294) treats `standard` and `mainline` as the \
         unrestricted retail flavor"
    );
    assert!(
        !toc.is_game_type_restricted(),
        "Blizzard_HousingControls must NOT be game-type restricted — `## AllowLoadGameType: \
         standard` matches the retail flavor that the simulator runs as"
    );
    assert!(
        !toc_text.contains("## AllowLoad:"),
        "Blizzard_HousingControls omits `## AllowLoad:` — LoadOnDemand precludes auto-discovery \
         gating, so the AllowLoad value would be inert"
    );
    assert!(
        !toc_text.contains("## DefaultState:"),
        "Blizzard_HousingControls omits `## DefaultState:` — relies on the loader's \
         implicit-enabled default for Blizzard prefix LoD addons"
    );
    assert!(
        toc.saved_variables().is_empty(),
        "Blizzard_HousingControls declares NO `## SavedVariables*` — control-strip visibility \
         is server-driven via HOUSE_PLOT_ENTERED / HOUSE_PLOT_EXITED / \
         HOUSE_EDITOR_AVAILABILITY_CHANGED / CURRENT_HOUSE_INFO_RECIEVED / \
         HOUSE_EDITOR_MODE_CHANGED / UPDATE_BINDINGS / HOUSE_INFO_UPDATED events plus \
         C_Housing.IsInsideHouseOrPlot() polling, so no per-installation persistence is needed"
    );
}

#[test]
fn blizzard_housing_controls_toc_lists_five_files() {
    let toc =
        TocFile::from_file(&housing_controls_toc()).expect("HousingControls TOC should parse");
    let files: Vec<String> = toc
        .files
        .iter()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .collect();
    assert_eq!(
        files,
        vec![
            "Blizzard_HousingControlsUtil.lua".to_string(),
            "Blizzard_HousingControlButton.lua".to_string(),
            "Blizzard_HousingControlButton.xml".to_string(),
            "Blizzard_HousingControls.lua".to_string(),
            "Blizzard_HousingControls.xml".to_string(),
        ],
        "Blizzard_HousingControls TOC body lists exactly 5 source files in this exact order: \
         Blizzard_HousingControlsUtil.lua (publishes the HousingControlsUtil module table with \
         the single CanActivateHousingControls(availabilityResult) helper that maps \
         Enum.HousingResult.Success to true and otherwise looks up the error text from \
         HousingResultToErrorText — must load FIRST because the ControlButton mixins reference \
         HousingControlsUtil.CanActivateHousingControls), Blizzard_HousingControlButton.lua \
         (publishes BaseHousingControlButtonMixin + the 5 button-specific mixins — \
         HouseEditorButtonMixin, HouseExitButtonMixin, HouseInfoButtonMixin, \
         HouseInspectorButtonMixin, HouseSettingsButtonMixin — must load BEFORE the matching \
         XML templates can reference them via `mixin=`), Blizzard_HousingControlButton.xml \
         (publishes 8 virtual Button templates — the 3 base/intermediate templates \
         BaseHousingControlButtonTemplate / HousingControlActionButtonTemplate / \
         HousingControlModeButtonTemplate plus the 5 specific implementations \
         HouseEditorButtonTemplate / HouseSettingsButtonTemplate / HousingExitButtonTemplate / \
         HouseInfoButtonTemplate / HouseInspectorButtonTemplate), Blizzard_HousingControls.lua \
         (publishes HousingControlsMixin + VisitorControlFrameMixin + the 2 local event tables \
         HousingControlsEvents (4 events — HOUSE_PLOT_ENTERED, HOUSE_PLOT_EXITED, \
         HOUSE_EDITOR_AVAILABILITY_CHANGED, CURRENT_HOUSE_INFO_RECIEVED) and \
         HousingControlsShownEvents (3 events — HOUSE_EDITOR_MODE_CHANGED, UPDATE_BINDINGS, \
         HOUSE_INFO_UPDATED)), Blizzard_HousingControls.xml (publishes the named non-virtual \
         HousingControlsFrame inheriting HousingControlsMixin + the named non-virtual \
         HousingVisitorControlsFrame placeholder)"
    );
}

#[test]
fn blizzard_housing_controls_directory_holds_six_entries() {
    let dir = housing_controls_dir();
    let entries: Vec<String> = std::fs::read_dir(&dir)
        .expect("Blizzard_HousingControls directory should exist")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        entries.len(),
        6,
        "Blizzard_HousingControls directory ships exactly 6 entries: 5 source files referenced \
         by the TOC + 1 TOC file. No flavor subdirectory and no Localization.lua — the \
         strings (HOUSING_CONTROLS_EDITOR_UNAVAILABLE / HOUSING_CONTROLS_EDITOR_UNAVAILABLE_FMT \
         / HOUSING_CONTROLS_EDITOR_BUTTON_ENTER / HOUSING_CONTROLS_EDITOR_BUTTON_ENTER_FMT / \
         HOUSING_CONTROLS_EDITOR_BUTTON_EXIT / HOUSING_CONTROLS_EDITOR_BUTTON_EXIT_FMT / \
         HOUSING_CONTROLS_SETTINGS_TOOLTIP / HOUSING_CONTROLS_SETTINGS_UNAVAILABLE / \
         HOUSING_CONTROLS_SETTINGS_UNAVAILABLE_FMT / HOUSING_CONTROLS_EXIT_BUTTON / \
         HOUSING_CONTROLS_INSPECT_TOOLTIP / HOUSING_CONTROLS_INSPECT_UNAVAILABLE_EDITOR_ACTIVE \
         / HOUSING_DASHBOARD_HOUSEINFO_TOOLTIP / HOUSING_DASHBOARD_OWNERS_HOUSE) are pulled \
         from the global locale table maintained by the housing dependency chain. Got: \
         {entries:?}"
    );
    assert!(
        entries.contains(&"Blizzard_HousingControls.toc".to_string()),
        "Blizzard_HousingControls directory must contain the bare TOC file"
    );
    assert!(
        entries.contains(&"Blizzard_HousingControlsUtil.lua".to_string()),
        "Blizzard_HousingControls directory must contain the Util tail file (10 lines, the \
         CanActivateHousingControls helper that ControlButton mixins consume)"
    );
}

#[test]
fn blizzard_housing_controls_excluded_from_all_screen_auto_discovery_passes() {
    let ui = blizzard_ui_dir();
    for screen in [
        ScreenKind::Game,
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        let addons = discover_blizzard_addons_for_screen(&ui, screen);
        let discovered = addons
            .iter()
            .any(|(name, _)| name == "Blizzard_HousingControls");
        assert!(
            !discovered,
            "Blizzard_HousingControls MUST NOT appear in {screen:?} auto-discovery — \
             `## LoadOnDemand: 1` keeps it out of every screen pass. Three consumers pull it \
             via explicit `C_AddOns.LoadAddOn(\"Blizzard_HousingControls\")` calls \
             (UIParent.lua:1622, HousingEventHandler.lua:285) or via the LoD-loaded \
             Blizzard_HouseEditor's `## Dependencies` chain — none uses `## RequiredDep:`, so \
             the LoD-pull promotion path in `pull_required_lod_addons` (src/loader/mod.rs:357) \
             does not escalate HousingControls onto any auto-discovery pass"
        );
    }
}

#[test]
fn blizzard_housing_controls_loads_without_addon_specific_lua_errors() {
    let env = load_full_game_ui_with_housing_controls_lod();

    let lua_errors: Vec<String> = env.state().borrow().lua_errors.clone();
    let related: Vec<&String> = lua_errors
        .iter()
        .filter(|e| {
            e.contains("Blizzard_HousingControls/")
                || e.contains("Blizzard_HousingControls\\")
                || e.contains("HousingControlsMixin")
                || e.contains("VisitorControlFrameMixin")
                || e.contains("BaseHousingControlButtonMixin")
                || e.contains("HouseEditorButtonMixin")
                || e.contains("HouseExitButtonMixin")
                || e.contains("HouseInfoButtonMixin")
                || e.contains("HouseInspectorButtonMixin")
                || e.contains("HouseSettingsButtonMixin")
        })
        .collect();
    assert!(
        related.is_empty(),
        "Blizzard_HousingControls emitted addon-specific Lua errors during explicit LoD load:\n  {}",
        related
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}

#[test]
fn blizzard_housing_controls_is_addon_loaded_returns_true_after_explicit_lod_load() {
    let env = load_full_game_ui_with_housing_controls_lod();

    let loaded: bool = env
        .eval("return C_AddOns.IsAddOnLoaded('Blizzard_HousingControls')")
        .expect("IsAddOnLoaded query should succeed");
    assert!(
        loaded,
        "After explicit `load_addon` of Blizzard_HousingControls.toc following Game-screen \
         auto-discovery (which loads Blizzard_HousingTemplates as a normal Game-screen addon \
         but skips HousingControls itself due to LoadOnDemand), \
         `C_AddOns.IsAddOnLoaded('Blizzard_HousingControls')` should return true"
    );
}

#[test]
fn blizzard_housing_controls_publishes_housing_controls_frame_global() {
    let env = load_full_game_ui_with_housing_controls_lod();

    let exists: bool = env
        .eval(
            "local f = _G['HousingControlsFrame']; return type(f) == 'table' and type(f.GetName) == 'function'",
        )
        .expect("HousingControlsFrame global lookup should succeed");
    assert!(
        exists,
        "After LoD load, `HousingControlsFrame` should publish as a global frame instance — \
         Blizzard_HousingControls.xml line 3 declares `<Frame name=\"HousingControlsFrame\" \
         mixin=\"HousingControlsMixin\" toplevel=\"true\" parent=\"UIParent\">`. The frame \
         self-anchors to TOP of UIParent at offset (0, -30) and sizes itself to 150x50 (the \
         child OwnerControlFrame and VisitorControlFrame swap visibility based on \
         C_HousingNeighborhood.IsPlayerInOtherPlayersPlot or being inside someone else's house)"
    );
}

#[test]
fn blizzard_housing_controls_publishes_housing_visitor_controls_frame_placeholder_global() {
    let env = load_full_game_ui_with_housing_controls_lod();

    let exists: bool = env
        .eval(
            "local f = _G['HousingVisitorControlsFrame']; return type(f) == 'table' and type(f.GetName) == 'function'",
        )
        .expect("HousingVisitorControlsFrame global lookup should succeed");
    assert!(
        exists,
        "After LoD load, `HousingVisitorControlsFrame` should publish as a global frame \
         instance — Blizzard_HousingControls.xml lines 100-102 declare `<Frame \
         name=\"HousingVisitorControlsFrame\">` with empty body. This is a placeholder \
         non-virtual Frame at file scope reserved for future visitor-control wiring; current \
         visitor controls live as a parentKey child of HousingControlsFrame \
         (HousingControlsFrame.VisitorControlFrame, mixin=VisitorControlFrameMixin)"
    );
}

#[test]
fn blizzard_housing_controls_mixin_publishes_eight_methods() {
    let env = load_full_game_ui_with_housing_controls_lod();

    for method in [
        "OnLoad",
        "OnEvent",
        "UpdateControlVisibility",
        "OnShow",
        "OnHide",
        "UpdateActiveFrame",
        "GetActiveFrame",
        "UpdateButtons",
    ] {
        let exists: bool = env
            .eval(&format!(
                "return type(HousingControlsMixin['{method}']) == 'function'"
            ))
            .expect("HousingControlsMixin method existence query should succeed");
        assert!(
            exists,
            "HousingControlsMixin must expose `:{method}()` — the mixin drives the \
             housing-decor controls top-strip: OnLoad calls UpdateControlVisibility with the \
             current C_Housing.IsInsideHouseOrPlot() result, then \
             FrameUtil.RegisterFrameForEvents(self, HousingControlsEvents) for the 4 \
             always-on events (HOUSE_PLOT_ENTERED, HOUSE_PLOT_EXITED, \
             HOUSE_EDITOR_AVAILABILITY_CHANGED, CURRENT_HOUSE_INFO_RECIEVED), and \
             FrameUtil.RegisterForTopLevelParentChanged; OnEvent dispatches the 7 events \
             across the 2 event tables (PLOT_ENTERED → UpdateControlVisibility(true), \
             PLOT_EXITED → UpdateControlVisibility(false), AVAILABILITY_CHANGED / \
             HOUSE_INFO_UPDATED / CURRENT_HOUSE_INFO_RECIEVED → \
             UpdateControlVisibility(IsInsideHouseOrPlot), UPDATE_BINDINGS / \
             HOUSE_EDITOR_MODE_CHANGED → UpdateButtons); UpdateControlVisibility shows the \
             frame only when isInsideHouseOrPlot AND \
             C_HouseEditor.IsHouseEditorStatusAvailable() (avoids partial-state flash before \
             editor is ready), then calls UpdateActiveFrame + UpdateButtons; OnShow calls \
             UpdateButtons + RegisterFrameForEvents for the 3 shown-only events + registers \
             EventRegistry callbacks for HousingInspectMode.{{Activated,Deactivated}}; OnHide \
             unregisters both; UpdateActiveFrame swaps OwnerControlFrame ↔ VisitorControlFrame \
             based on C_HousingNeighborhood.IsPlayerInOtherPlayersPlot OR (IsInsideHouse AND \
             NOT IsInsideOwnHouse), calls VisitorControlFrame:UpdateOwnerInfomation [sic — \
             Blizzard's typo \"Infomation\" preserved verbatim], stores the chosen frame on \
             self.activeFrame; GetActiveFrame returns self.activeFrame; UpdateButtons \
             iterates activeFrame.Buttons (the parentArray collected by \
             BaseHousingControlButtonTemplate's `parentArray=\"Buttons\"`) and calls \
             :UpdateState() on each"
        );
    }
}

#[test]
fn blizzard_visitor_control_frame_mixin_publishes_one_method() {
    let env = load_full_game_ui_with_housing_controls_lod();

    let exists: bool = env
        .eval("return type(VisitorControlFrameMixin['UpdateOwnerInfomation']) == 'function'")
        .expect("VisitorControlFrameMixin method existence query should succeed");
    assert!(
        exists,
        "VisitorControlFrameMixin must expose `:UpdateOwnerInfomation()` (note Blizzard's \
         typo — `Infomation`, not `Information` — preserved verbatim from the source) — calls \
         C_Housing.GetCurrentHouseInfo() and either clears OwnerNameText (when no info) or \
         formats it via `string.format(HOUSING_DASHBOARD_OWNERS_HOUSE, houseInfo.ownerName)`. \
         Stores ownerName on self for downstream use"
    );
}

#[test]
fn blizzard_housing_controls_publishes_six_button_mixins_globally() {
    let env = load_full_game_ui_with_housing_controls_lod();

    for mixin in [
        "BaseHousingControlButtonMixin",
        "HouseEditorButtonMixin",
        "HouseExitButtonMixin",
        "HouseInfoButtonMixin",
        "HouseInspectorButtonMixin",
        "HouseSettingsButtonMixin",
    ] {
        let exists: bool = env
            .eval(&format!("return type(_G['{mixin}']) == 'table'"))
            .expect("Mixin global lookup should succeed");
        assert!(
            exists,
            "{mixin} must publish as a global table after LoD load — the 6 button mixins are \
             declared at module scope in Blizzard_HousingControlButton.lua: \
             BaseHousingControlButtonMixin (the shared base — provides GetDefaultTexture, \
             GetIconForState (3-way enabled/pressed/active branch), GetIconColorForState \
             (WHITE_FONT_COLOR vs DARKGRAY_COLOR), IsActive default-not-implemented assert, \
             CheckEnabled with Kiosk + nyiLabel guards, OnClick with Kiosk skip + sound + \
             BaseHousingModeButtonMixin.OnClick chain), HouseEditorButtonMixin (CheckEnabled \
             via C_HouseEditor.GetHouseEditorAvailability + HousingControlsUtil + \
             HOUSING_CONTROLS_EDITOR_UNAVAILABLE_FMT, IsActive via \
             C_HouseEditor.IsHouseEditorActive, EnterMode via C_HouseEditor.EnterHouseEditor + \
             UIErrorsFrame fallback, LeaveMode via HousingFramesUtil.LeaveHouseEditor), \
             HouseExitButtonMixin (OnClick with Kiosk skip + C_Housing.LeaveHouse, IsActive \
             always false, CheckEnabled via IsInsideHouse AND \
             GetActiveHouseEditorMode==Enum.HouseEditorMode.None), HouseInfoButtonMixin \
             (OnClick LoadAddOn-pulls Blizzard_HousingCornerstone then ToggleUIPanel on \
             HousingCornerstoneHouseInfoFrame, CheckEnabled always true, IsActive checks the \
             info frame's IsShown), HouseInspectorButtonMixin (EnterMode LoadAddOn-pulls \
             Blizzard_HousingInspectModeUI then EnterInspectMode, LeaveMode ExitInspectMode, \
             CheckEnabled blocks while editor is active with \
             HOUSING_CONTROLS_INSPECT_UNAVAILABLE_EDITOR_ACTIVE, IsActive via \
             C_HousingInspectMode.IsInInspectMode), HouseSettingsButtonMixin (EnterMode \
             LoadAddOn-pulls Blizzard_HousingHouseSettings then ShowUIPanel, LeaveMode \
             HideUIPanel, IsActive via the settings frame's IsShown, CheckEnabled mirrors \
             HouseEditorButtonMixin's availability logic with \
             HOUSING_CONTROLS_SETTINGS_UNAVAILABLE_FMT)"
        );
    }
}

#[test]
fn blizzard_housing_controls_util_publishes_can_activate_helper() {
    let env = load_full_game_ui_with_housing_controls_lod();

    let exists: bool = env
        .eval("return type(HousingControlsUtil) == 'table' and type(HousingControlsUtil.CanActivateHousingControls) == 'function'")
        .expect("HousingControlsUtil global lookup should succeed");
    assert!(
        exists,
        "HousingControlsUtil global table must publish with the CanActivateHousingControls \
         helper — Blizzard_HousingControlsUtil.lua line 4 defines \
         `function HousingControlsUtil.CanActivateHousingControls(availabilityResult)` which \
         maps `Enum.HousingResult.Success` to `(true, nil)` and otherwise returns \
         `(false, HousingResultToErrorText[availabilityResult])`. This helper is consumed by \
         HouseEditorButtonMixin:CheckEnabled and HouseSettingsButtonMixin:CheckEnabled to \
         decide whether the editor / settings buttons are clickable"
    );
}

#[test]
fn blizzard_housing_controls_does_not_publish_virtual_button_templates() {
    let env = load_full_game_ui_with_housing_controls_lod();

    for template in [
        "BaseHousingControlButtonTemplate",
        "HousingControlActionButtonTemplate",
        "HousingControlModeButtonTemplate",
        "HouseEditorButtonTemplate",
        "HouseSettingsButtonTemplate",
        "HousingExitButtonTemplate",
        "HouseInfoButtonTemplate",
        "HouseInspectorButtonTemplate",
    ] {
        let published: bool = env
            .eval(&format!("return _G['{template}'] ~= nil"))
            .expect("Template global lookup should succeed");
        assert!(
            !published,
            "{template} is declared `virtual=\"true\"` in \
             Blizzard_HousingControlButton.xml — virtual XML templates are NOT instantiated as \
             global frames at load time. They only materialize when a parent frame inherits \
             them. The HousingControlsFrame's OwnerControlFrame instantiates 5 of these (one \
             of each specific-implementation template) per the inherits= attribute in its \
             child Buttons, but the template names themselves stay out of `_G`"
        );
    }
}

#[test]
fn blizzard_housing_controls_owner_control_frame_publishes_five_buttons() {
    let env = load_full_game_ui_with_housing_controls_lod();

    let owner_frame_exists: bool = env
        .eval(
            "local f = HousingControlsFrame.OwnerControlFrame; return type(f) == 'table' and type(f.GetName) == 'function'",
        )
        .expect("OwnerControlFrame parentKey lookup should succeed");
    assert!(
        owner_frame_exists,
        "HousingControlsFrame.OwnerControlFrame must publish via parentKey — \
         Blizzard_HousingControls.xml line 10 declares `<Frame parentKey=\"OwnerControlFrame\" \
         setAllPoints=\"true\" hidden=\"true\">` as the container for the 5 owner-mode buttons"
    );

    for parent_key in [
        "HouseEditorButton",
        "SettingsButton",
        "ExitButton",
        "HouseInfoButton",
        "InspectorButton",
    ] {
        let exists: bool = env
            .eval(&format!(
                "return type(HousingControlsFrame.OwnerControlFrame['{parent_key}']) ~= 'nil'"
            ))
            .expect("OwnerControlFrame parentKey child lookup should succeed");
        assert!(
            exists,
            "HousingControlsFrame.OwnerControlFrame.{parent_key} must publish via parentKey — \
             the XML wires 5 button children with `parentKey=` so that the HousingControlsMixin \
             can address them without touching `_G`: HouseEditorButton (68x68 \
             HouseEditorButtonTemplate centered with -12 y-offset), SettingsButton \
             (HouseSettingsButtonTemplate left of editor with 10,3 offset), ExitButton \
             (HousingExitButtonTemplate left of settings), HouseInfoButton \
             (HouseInfoButtonTemplate right of editor with -10,3 offset), InspectorButton \
             (HouseInspectorButtonTemplate right of HouseInfoButton). All 5 inherit \
             BaseHousingControlButtonTemplate's `parentArray=\"Buttons\"`, so they collect \
             into HousingControlsFrame.OwnerControlFrame.Buttons for the UpdateButtons loop"
        );
    }
}

#[test]
fn blizzard_housing_controls_visitor_control_frame_publishes_three_buttons() {
    let env = load_full_game_ui_with_housing_controls_lod();

    let visitor_frame_exists: bool = env
        .eval(
            "local f = HousingControlsFrame.VisitorControlFrame; return type(f) == 'table' and type(f.GetName) == 'function'",
        )
        .expect("VisitorControlFrame parentKey lookup should succeed");
    assert!(
        visitor_frame_exists,
        "HousingControlsFrame.VisitorControlFrame must publish via parentKey — \
         Blizzard_HousingControls.xml line 52 declares `<Frame parentKey=\"VisitorControlFrame\" \
         mixin=\"VisitorControlFrameMixin\" setAllPoints=\"true\" hidden=\"true\">` as the \
         container for the 3 visitor-mode buttons + the OwnerNameText FontString + Divider \
         texture (atlas=`controls-frame-guest`)"
    );

    for parent_key in [
        "VisitorHouseInfoButton",
        "VisitorExitButton",
        "VisitorInspectorButton",
        "OwnerNameText",
        "Divider",
    ] {
        let exists: bool = env
            .eval(&format!(
                "return type(HousingControlsFrame.VisitorControlFrame['{parent_key}']) ~= 'nil'"
            ))
            .expect("VisitorControlFrame parentKey child lookup should succeed");
        assert!(
            exists,
            "HousingControlsFrame.VisitorControlFrame.{parent_key} must publish via parentKey — \
             the XML wires the visitor-mode children: OwnerNameText (GameFontNormalHuge \
             FontString anchored TOP, color=HOUSING_CONTROL_PANEL_TITLE), Divider \
             (controls-frame-guest atlas anchored 8 below OwnerNameText), \
             VisitorHouseInfoButton (HouseInfoButtonTemplate anchored TOP of Divider with \
             0,-5), VisitorExitButton (HousingExitButtonTemplate left-of VisitorHouseInfoButton), \
             VisitorInspectorButton (HouseInspectorButtonTemplate right-of \
             VisitorHouseInfoButton). The 3 buttons collect into the VisitorControlFrame's \
             Buttons parentArray for the UpdateButtons loop when the visitor frame is active"
        );
    }
}

#[test]
fn blizzard_housing_controls_dependency_loads_via_game_screen_pass() {
    let env = load_full_game_ui_with_housing_controls_lod();

    let templates_loaded: bool = env
        .eval("return C_AddOns.IsAddOnLoaded('Blizzard_HousingTemplates')")
        .expect("IsAddOnLoaded query should succeed");
    assert!(
        templates_loaded,
        "Blizzard_HousingTemplates must be loaded by the Game-screen auto-discovery pass \
         before the explicit HousingControls LoD load runs. HousingTemplates is the only \
         `## Dependencies` entry on HousingControls's TOC, and the test harness's full \
         Game-screen pass hits it via the normal discovery flow because HousingTemplates is \
         itself non-LoD with `## AllowLoad: Both` semantics"
    );
}
