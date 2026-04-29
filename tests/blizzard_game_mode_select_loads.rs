#![cfg(feature = "client-retail")]
use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, find_toc_file, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
}

fn game_mode_select_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_GameModeSelect")
}

fn game_mode_select_mainline_toc() -> PathBuf {
    game_mode_select_dir().join("Blizzard_GameModeSelect_Mainline.toc")
}

fn load_full_login_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Login);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    wow_ui_sim::xml::register_intrinsic_templates();

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::Login);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("[load {name}] FAILED: {err}"));
    }

    env.apply_post_load_workarounds();
    fire_startup_events_for_screen(&env, ScreenKind::Login);
    env
}

#[test]
fn blizzard_game_mode_select_picks_mainline_toc_variant() {
    let resolved = find_toc_file(&game_mode_select_dir())
        .expect("Blizzard_GameModeSelect directory must contain a discoverable TOC");
    let resolved_name = resolved
        .file_name()
        .expect("resolved TOC must have a filename")
        .to_str()
        .expect("resolved TOC filename must be utf-8");

    assert_eq!(
        resolved_name, "Blizzard_GameModeSelect_Mainline.toc",
        "Blizzard_GameModeSelect ships TWO TOC files (Blizzard_GameModeSelect_Mainline.toc \
         and Blizzard_GameModeSelect_Classic.toc). `find_toc_file` (src/loader/mod.rs:65) \
         prefers the `_Mainline.toc` variant — the Classic variant is reachable only by \
         direct path and gates the Classic\\GameModeSelectConstants.lua subdir"
    );
}

#[test]
fn blizzard_game_mode_select_mainline_toc_declares_glue_only_load() {
    let toc = TocFile::from_file(&game_mode_select_mainline_toc())
        .expect("Blizzard_GameModeSelect Mainline TOC parse");

    assert!(
        !toc.is_load_on_demand(),
        "Blizzard_GameModeSelect has no `## LoadOnDemand` line — the game-mode picker \
         must be eagerly loaded on the glue screens so the realm-list / character-select \
         UI can host the GameModeFrameTemplate without an LOD round-trip"
    );
    assert!(
        !toc.is_secure_env(),
        "Blizzard_GameModeSelect does not declare `## UseSecureEnvironment` — it runs \
         in the standard taint environment (the glue screens have no combat-protected \
         frames)"
    );
    assert!(
        !toc.is_game_type_restricted(),
        "Blizzard_GameModeSelect_Mainline.toc declares `## AllowLoadGameType: mainline`, \
         but `is_game_type_restricted()` returns false because src/toc.rs:299 treats \
         `mainline` as the unrestricted retail game type. The Classic variant exists in \
         a sibling TOC for non-mainline flavors"
    );
    assert!(
        toc.saved_variables().is_empty(),
        "Blizzard_GameModeSelect declares no `## SavedVariables` — the only \
         persisted-ish state is `g_newGameModeAvailableAcknowledged` which lives in \
         globals (lua line 3) and resets per-session"
    );

    let deps = toc.dependencies();
    assert_eq!(
        deps,
        vec!["Blizzard_SharedXML".to_string()],
        "`## Dependencies: Blizzard_SharedXML` declares the only required dependency \
         — needed for the SelectableButtonMixin / CallbackRegistrantMixin / \
         ResizeLayoutMixin / DefaultScaleFrame / ButtonGroupBaseMixin / \
         CreateRadioButtonGroup / CreateFramePoolCollection plumbing the templates and \
         frame mixin all consume. Got: {:?}",
        deps
    );

    let toc_text = std::fs::read_to_string(game_mode_select_mainline_toc())
        .expect("Blizzard_GameModeSelect Mainline TOC should read");
    assert!(
        toc_text.contains("## AllowLoad: Glue"),
        "Blizzard_GameModeSelect_Mainline.toc declares `## AllowLoad: Glue` — the \
         game-mode picker is exclusively a glue-screen widget (lives on the Login / \
         CharacterSelect realm-list panel), it has no in-world surface"
    );
    assert!(
        toc_text.contains("## AllowLoadGameType: mainline"),
        "Blizzard_GameModeSelect_Mainline.toc declares `## AllowLoadGameType: mainline` \
         — explicitly retail-only, with the Classic flavor served by the sibling TOC"
    );
}

#[test]
fn blizzard_game_mode_select_allows_only_glue_screens() {
    let toc = TocFile::from_file(&game_mode_select_mainline_toc())
        .expect("Blizzard_GameModeSelect Mainline TOC parse");

    assert!(
        !toc.allows_screen(ScreenKind::Game),
        "`## AllowLoad: Glue` REJECTS the Game screen — the game-mode picker is glue \
         only, never visible in-world (src/toc.rs:307)"
    );
    assert!(
        toc.allows_screen(ScreenKind::Login),
        "`## AllowLoad: Glue` allows the Login screen — game-mode buttons appear on the \
         realm list at login"
    );
    assert!(
        toc.allows_screen(ScreenKind::CharacterSelect),
        "`## AllowLoad: Glue` allows CharacterSelect — the game-mode picker is also \
         visible on the character-select panel so users can switch flavors without \
         logging out"
    );
}

#[test]
fn blizzard_game_mode_select_auto_loads_on_login_and_skips_game() {
    let game_addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    let in_game = game_addons
        .iter()
        .any(|(name, _)| name == "Blizzard_GameModeSelect");
    assert!(
        !in_game,
        "`## AllowLoad: Glue` excludes Blizzard_GameModeSelect from Game-screen \
         auto-discovery — this addon is glue-only"
    );

    let login_addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Login);
    let in_login = login_addons
        .iter()
        .any(|(name, _)| name == "Blizzard_GameModeSelect");
    assert!(
        in_login,
        "Blizzard_GameModeSelect has no `## LoadOnDemand` line and `## AllowLoad: Glue`, \
         so it MUST appear in Login-screen auto-discovery"
    );

    let char_select_addons =
        discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::CharacterSelect);
    let in_char_select = char_select_addons
        .iter()
        .any(|(name, _)| name == "Blizzard_GameModeSelect");
    assert!(
        in_char_select,
        "`## AllowLoad: Glue` includes CharacterSelect alongside Login — the picker \
         appears on both glue screens"
    );
}

#[test]
fn blizzard_game_mode_select_loads_via_full_login_ui_without_errors() {
    let env = load_full_login_ui();

    let load_errors: Vec<String> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| {
            message.contains("GameModeSelect")
                || message.contains("GameModeButton")
                || message.contains("GameModeFrame")
                || message.contains("GameModePromo")
        })
        .cloned()
        .collect();

    assert!(
        load_errors.is_empty(),
        "Blizzard_GameModeSelect emitted Lua errors during the full Login-screen load:\n  {}",
        load_errors.join("\n  ")
    );
}

#[test]
fn blizzard_game_mode_select_is_addon_loaded_returns_true_after_full_login_ui_load() {
    let env = load_full_login_ui();

    let post_load: bool = env
        .eval("return C_AddOns.IsAddOnLoaded('Blizzard_GameModeSelect') and true or false")
        .expect("IsAddOnLoaded probe should succeed");
    assert!(
        post_load,
        "After full Login-screen load, IsAddOnLoaded('Blizzard_GameModeSelect') must \
         return true — auto-discovery picks up the addon (no LoadOnDemand) and \
         `mark_addon_loaded` registers it"
    );
}

#[test]
fn blizzard_game_mode_select_publishes_three_mixins() {
    let env = load_full_login_ui();

    let mixins: (bool, bool, bool) = env
        .eval(
            "return type(GameModeButtonMixin) == 'table', \
                    type(GameModeButtonPromoMixin) == 'table', \
                    type(GameModeFrameMixin) == 'table'",
        )
        .expect("GameModeSelect mixin probe should succeed");
    assert_eq!(
        mixins,
        (true, true, true),
        "GameModeSelect.lua publishes three mixin globals: GameModeButtonMixin (line \
         14 — base button mixin with OnLoad/OnShow/OnEnter/OnLeave/SetDisabled/\
         SetSelectedState/InitSize/SetGameMode/RefreshStandardLogo/RefreshScale, \
         consumed by GameModeButtonTemplate via `mixin=\"SelectableButtonMixin, \
         GameModeButtonMixin\"`), GameModeButtonPromoMixin (line 102, = \
         `CreateFromMixins(GameModeButtonMixin)` — extends the base with PulseAnim \
         playback, PromoText label management, and InitSize override that shrinks the \
         logo to make room for promo text below; consumed by GameModePromoButtonTemplate \
         via `mixin=\"SelectableButtonMixin, GameModeButtonPromoMixin\"`), \
         GameModeFrameMixin (line 202 — owner frame mixin with the radio-button group / \
         frame-pool collection / event registration / SelectGameMode dispatch / \
         ChangeGameMode → C_GameRules.AutoConnectToGameModeRealm path; consumed by \
         GameModeFrameTemplate)"
    );
}

#[test]
fn blizzard_game_mode_select_publishes_button_mixin_methods() {
    let env = load_full_login_ui();

    let methods: (bool, bool, bool, bool, bool) = env
        .eval(
            "return type(GameModeButtonMixin.OnLoad) == 'function', \
                    type(GameModeButtonMixin.OnShow) == 'function', \
                    type(GameModeButtonMixin.OnEnter) == 'function', \
                    type(GameModeButtonMixin.OnLeave) == 'function', \
                    type(GameModeButtonMixin.SetDisabled) == 'function'",
        )
        .expect("GameModeButtonMixin lifecycle probe should succeed");
    assert_eq!(
        methods,
        (true, true, true, true, true),
        "GameModeButtonMixin publishes the five XML-bound script methods consumed by \
         GameModeButtonTemplate: OnLoad (line 16 — chains SelectableButtonMixin.OnLoad \
         then InitSize), OnShow (line 22 — sets alpha based on selection state, \
         refreshes the standard expansion logo for non-promo modes), OnEnter (line 30 \
         — shows GlueTooltip with GAME_MODE_DISABLED_TOOLTIP on disabled buttons, \
         brightens unselected enabled buttons), OnLeave (line 42 — hides GlueTooltip, \
         dims unselected buttons), SetDisabled (line 50 — desaturates the NormalTexture \
         and stores the disabled flag for the OnEnter tooltip path)"
    );

    let game_mode_methods: (bool, bool, bool, bool) = env
        .eval(
            "return type(GameModeButtonMixin.SetSelectedState) == 'function', \
                    type(GameModeButtonMixin.SetGameMode) == 'function', \
                    type(GameModeButtonMixin.RefreshStandardLogo) == 'function', \
                    type(GameModeButtonMixin.RefreshScale) == 'function'",
        )
        .expect("GameModeButtonMixin game-mode methods probe should succeed");
    assert_eq!(
        game_mode_methods,
        (true, true, true, true),
        "Game-mode-specific methods on GameModeButtonMixin: SetSelectedState (line 55 \
         — toggles SelectionArrow / BackgroundGlowTop / BackgroundGlowBottom visibility, \
         applies RefreshScale, sets alpha 1.0/0.5; early-returns if disabled), \
         SetGameMode (line 75 — fetches `C_GameRules.GetGameModeDisplayInfoByRecordID` \
         and either applies the gameMode-specific logo or falls back to the standard \
         expansion logo with usingExpansionLogo flag), RefreshStandardLogo (line 86 — \
         calls `AccountUpgradePanel_GetBannerInfo` + `SetExpansionLogo` only when the \
         expansion level changed since last refresh, caching shownExpansionLevel), \
         RefreshScale (line 94 — applies the GameModeSelectNormalTextureScale.\
         {{selected,deselected}} scale to NormalTexture)"
    );
}

#[test]
fn blizzard_game_mode_select_promo_mixin_extends_base_via_create_from_mixins() {
    let env = load_full_login_ui();

    let promo_methods: (bool, bool, bool, bool) = env
        .eval(
            "return type(GameModeButtonPromoMixin.OnLoad) == 'function', \
                    type(GameModeButtonPromoMixin.OnShow) == 'function', \
                    type(GameModeButtonPromoMixin.OnSelected) == 'function', \
                    type(GameModeButtonPromoMixin.SetPulsePlaying) == 'function'",
        )
        .expect("GameModeButtonPromoMixin lifecycle probe should succeed");
    assert_eq!(
        promo_methods,
        (true, true, true, true),
        "GameModeButtonPromoMixin (line 102 — `CreateFromMixins(GameModeButtonMixin)`) \
         overrides OnLoad / OnShow / OnEnter / OnLeave / OnSelected / InitSize / \
         SetGameMode / RefreshScale to manage the PulseAnim animation group on \
         GameModePromoButtonTemplate. SetPulsePlaying (line 179) is the promo-only \
         method that toggles PulseTexture / PulseTextureTwo visibility and starts/stops \
         the PulseAnim AnimationGroup. CreateFromMixins copies the base mixin's methods \
         into the promo table at module-load, so all base methods are also reachable via \
         the promo namespace"
    );

    let inherited_methods: (bool, bool) = env
        .eval(
            "return type(GameModeButtonPromoMixin.SetDisabled) == 'function', \
                    type(GameModeButtonPromoMixin.SetSelectedState) == 'function'",
        )
        .expect("GameModeButtonPromoMixin inherited probe should succeed");
    assert_eq!(
        inherited_methods,
        (true, true),
        "Methods that the promo mixin inherits via CreateFromMixins(GameModeButtonMixin) \
         without overriding: SetDisabled / SetSelectedState. These names resolve via \
         the copied base table, not via runtime metatable lookup, so they MUST be \
         present as functions on GameModeButtonPromoMixin directly"
    );
}

#[test]
fn blizzard_game_mode_select_publishes_frame_mixin_methods() {
    let env = load_full_login_ui();

    let lifecycle: (bool, bool, bool, bool, bool) = env
        .eval(
            "return type(GameModeFrameMixin.OnLoad) == 'function', \
                    type(GameModeFrameMixin.OnShow) == 'function', \
                    type(GameModeFrameMixin.OnHide) == 'function', \
                    type(GameModeFrameMixin.OnKeyDown) == 'function', \
                    type(GameModeFrameMixin.OnEvent) == 'function'",
        )
        .expect("GameModeFrameMixin lifecycle probe should succeed");
    assert_eq!(
        lifecycle,
        (true, true, true, true, true),
        "GameModeFrameMixin publishes the five XML-bound script methods consumed by \
         GameModeFrameTemplate: OnLoad (line 213 — creates the radio-button group, \
         allocates the FramePoolCollection for the two button templates, registers \
         AVAILABLE_GAME_MODES_UPDATED / GAME_MODE_DISPLAY_INFO_UPDATED / \
         GAME_MODE_DISPLAY_MODE_TOGGLE_DISABLED, wires GameMode.Selected / \
         RealmList.Cancel EventRegistry hooks, runs initial OnAvailableGameModesUpdated), \
         OnShow (line 233 — chains CallbackRegistrantMixin.OnShow + ResizeLayoutMixin.\
         OnShow, calls SelectRadioButtonForGameMode and TryShowGameModeButtons), OnHide \
         (line 241 — symmetric CallbackRegistrantMixin.OnHide), OnKeyDown (line 245 — \
         on ESCAPE fires the GameModeFrame.Hide EventRegistry trigger), OnEvent (line \
         251 — central event dispatcher routing to OnAvailableGameModesUpdated / \
         GameModeFrame.Hide / SetDisabledForMode based on event name)"
    );

    let dispatch: (bool, bool, bool, bool) = env
        .eval(
            "return type(GameModeFrameMixin.SelectGameMode) == 'function', \
                    type(GameModeFrameMixin.OnGameModeSelected) == 'function', \
                    type(GameModeFrameMixin.ChangeGameMode) == 'function', \
                    type(GameModeFrameMixin.SelectRadioButtonForGameMode) == 'function'",
        )
        .expect("GameModeFrameMixin dispatch probe should succeed");
    assert_eq!(
        dispatch,
        (true, true, true, true),
        "Selection-flow methods: SelectGameMode (line 356 — radio-button-group callback \
         that fires GameMode.Selected EventRegistry trigger with the requested record \
         ID), OnGameModeSelected (line 301 — GameMode.Selected listener that early-\
         returns if disabled, sets g_newGameModeAvailableAcknowledged=1 if the mode has \
         a promo, then calls ChangeGameMode), ChangeGameMode (line 331 — saves \
         character order via CharacterSelectListUtil.SaveCharacterOrder unless \
         IsCharacterlessLoginActive, then C_GameRules.AutoConnectToGameModeRealm to \
         actually switch flavors), SelectRadioButtonForGameMode (line 345 — sweeps the \
         button group setting selected state per-button and triggering \
         GameMode.UpdateNavBar)"
    );

    let helpers: (bool, bool, bool, bool) = env
        .eval(
            "return type(GameModeFrameMixin.OnAvailableGameModesUpdated) == 'function', \
                    type(GameModeFrameMixin.OnRealmListCancel) == 'function', \
                    type(GameModeFrameMixin.SetDisabledForMode) == 'function', \
                    type(GameModeFrameMixin.TryShowGameModeButtons) == 'function'",
        )
        .expect("GameModeFrameMixin helpers probe should succeed");
    assert_eq!(
        helpers,
        (true, true, true, true),
        "Refresh helpers: OnAvailableGameModesUpdated (line 265 — clears existing \
         buttons / acquires from the right pool per mode / lays them out left-to-right \
         with GameModeSelectButtonSpacing, then runs TryShowGameModeButtons), \
         OnRealmListCancel (line 317 — re-selects the current game mode's radio button \
         when the realm-list dialog is canceled), SetDisabledForMode (line 204 — sweeps \
         button group looking for matching gameModeRecordID and applies SetDisabled), \
         TryShowGameModeButtons (line 321 — toggles between the button group and the \
         NoGameModesText fallback based on numDisplayedGameModes > 1, applies width \
         padding for the empty case, runs Layout)"
    );
}

#[test]
fn blizzard_game_mode_select_publishes_layout_constants() {
    let env = load_full_login_ui();

    let scalars: (i64, i64, f64) = env
        .eval(
            "return GameModeSelectFixedHeight, \
                    GameModeSelectButtonSpacing, \
                    GameModeSelectPromoButtonTextureScale",
        )
        .expect("GameModeSelect scalar constant probe should succeed");
    assert_eq!(
        scalars,
        (122, -24, 0.82),
        "Mainline/GameModeSelectConstants.lua publishes the layout scalars: \
         GameModeSelectFixedHeight=122 (the canonical button height — width is double \
         this, see GameModeSelect.lua line 8 `GameModeSelectButtonSize.width = 2 * \
         GameModeSelectFixedHeight`), GameModeSelectButtonSpacing=-24 (negative spacing \
         to make the buttons overlap slightly per Constants.lua comment), \
         GameModeSelectPromoButtonTextureScale=0.82 (promo buttons shrink the logo by \
         18% to make room for the LIMITED_TIME_EVENT promo text below)"
    );

    let normal_scale: (f64, f64) = env
        .eval(
            "return GameModeSelectNormalTextureScale.selected, \
                    GameModeSelectNormalTextureScale.deselected",
        )
        .expect("GameModeSelectNormalTextureScale probe should succeed");
    assert_eq!(
        normal_scale,
        (0.9, 0.81),
        "GameModeSelectNormalTextureScale={{selected=0.9, deselected=0.81}} (Constants.\
         lua lines 8-11) is the texture-scale dictionary applied by RefreshScale — \
         selected buttons render at 90% of native logo size, unselected at 81%"
    );

    let promo_text_scale: (f64, f64) = env
        .eval(
            "return GameModeSelectPromoTextScale.selected, \
                    GameModeSelectPromoTextScale.deselected",
        )
        .expect("GameModeSelectPromoTextScale probe should succeed");
    assert_eq!(
        promo_text_scale,
        (1.0, 0.89),
        "GameModeSelectPromoTextScale={{selected=1, deselected=0.89}} (Constants.lua \
         lines 17-20) — selected promo buttons show full-size text, unselected shrink \
         to 89% to de-emphasize"
    );
}

#[test]
fn blizzard_game_mode_select_publishes_acknowledged_flag_global() {
    let env = load_full_login_ui();

    let initial: bool = env
        .eval("return g_newGameModeAvailableAcknowledged == nil")
        .expect("g_newGameModeAvailableAcknowledged probe should succeed");
    assert!(
        initial,
        "GameModeSelect.lua line 3 declares `g_newGameModeAvailableAcknowledged = \
         g_newGameModeAvailableAcknowledged or nil` — the conditional-or pattern \
         preserves any pre-set value across reloads but defaults to nil at first load. \
         OnGameModeSelected sets this to 1 (line 310) when the user picks a promo \
         game mode, signalling that the new-mode pulse animation should stop"
    );
}

#[test]
fn blizzard_game_mode_select_publishes_only_virtual_templates_no_top_level_frames() {
    let env = load_full_login_ui();

    let no_global_frames: (bool, bool, bool) = env
        .eval(
            "return _G.GameModeButtonTemplate == nil, \
                    _G.GameModePromoButtonTemplate == nil, \
                    _G.GameModeFrameTemplate == nil",
        )
        .expect("Virtual template global-leak probe should succeed");
    assert_eq!(
        no_global_frames,
        (true, true, true),
        "GameModeSelect.xml declares THREE virtual=\"true\" templates \
         (GameModeButtonTemplate / GameModePromoButtonTemplate / GameModeFrameTemplate) \
         and ZERO non-virtual top-level frames. Virtual templates are registered in the \
         template registry but do NOT publish as named globals — they are instantiated \
         only when a parent XML or CreateFrame call references them by name. The glue \
         realm-list panel inherits GameModeFrameTemplate to host the picker; the buttons \
         are created dynamically by GameModeFrameMixin.OnAvailableGameModesUpdated \
         using the FramePoolCollection. So `_G.GameMode<X>Template` must all be nil"
    );
}
