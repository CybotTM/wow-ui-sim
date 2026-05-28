use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, find_toc_file, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}

fn login_warning_dialogs_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_LoginWarningDialogs")
}

fn login_warning_dialogs_toc() -> PathBuf {
    login_warning_dialogs_dir().join("Blizzard_LoginWarningDialogs.toc")
}

const LOGIN_WARNING_DIALOGS_TOC_FILES: &[&str] = &[
    "Localization.lua",
    "Blizzard_LoginWarningDialogs.lua",
    "Blizzard_LoginWarningDialogs.xml",
];

const LOGIN_WARNING_DIALOGS_REQUIRED_DEPS: &[&str] = &[
    "Blizzard_SharedXML",
    "Blizzard_GlueXMLBase",
    "Blizzard_GlueParent",
];

const LOGIN_WARNING_FRAMES: &[&str] = &[
    "ChinaAgeAppropriatenessWarning",
    "KoreanRatings",
    "TaiwanFraudWarning",
];

fn load_character_select_screen() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::CharacterSelect);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    wow_ui_sim::xml::register_intrinsic_templates();

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::CharacterSelect);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("[load {name}] FAILED: {err}"));
    }

    env.apply_post_load_workarounds();
    settle_headless_startup(&env);
    env
}

#[test]
fn blizzard_login_warning_dialogs_find_toc_resolves_bare_variant() {
    let resolved = find_toc_file(&login_warning_dialogs_dir())
        .expect("Blizzard_LoginWarningDialogs TOC should resolve");
    assert_eq!(
        resolved,
        login_warning_dialogs_toc(),
        "Blizzard_LoginWarningDialogs ships a single bare TOC. Glue-screen warning dialogs \
         are flavor-agnostic (the same legal-compliance copy applies on retail + classic), so \
         there is no `_Mainline.toc` variant — `find_toc_file` (src/loader/mod.rs:65) walks \
         the bare-TOC fallback after the Mainline lookup misses"
    );
}

#[test]
fn blizzard_login_warning_dialogs_toc_declares_eager_glue_with_three_required_deps() {
    let toc = TocFile::from_file(&login_warning_dialogs_toc())
        .expect("Blizzard_LoginWarningDialogs TOC parses");
    assert!(
        !toc.is_load_on_demand(),
        "Blizzard_LoginWarningDialogs omits `## LoadOnDemand:` — `## DefaultState: enabled` \
         makes the addon eager-load on the glue screen so the warning frames exist before \
         GlueParent's screen-management code probes their `localeMatches` flag during the \
         legal-compliance show/hide pass"
    );
    assert!(!toc.is_load_first());
    assert!(!toc.is_secure_env());
    assert_eq!(
        toc.dependencies(),
        LOGIN_WARNING_DIALOGS_REQUIRED_DEPS
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
        "## RequiredDep declares 3 deps (parsed by `dependencies()` at src/toc.rs:210 — \
         RequiredDep is the canonical key, with RequiredDeps / Dependencies as fallbacks): \
         Blizzard_SharedXML supplies the DialogBorderTemplate / ScrollFrameTemplate / \
         ResizeCheckButtonTemplate XML inheritance the warning frames pull from, \
         Blizzard_GlueXMLBase supplies the GlueButtonTemplate / GlueFontNormalLarge / \
         GlueFontNormalGigantor font + button base templates, and Blizzard_GlueParent supplies \
         the `GlueParent` parent the 3 frames attach to (and the GlueParent_AddModalFrame / \
         GlueParent_RemoveModalFrame functions used by TaiwanFraudWarningMixin OnShow/OnHide)"
    );
    assert!(toc.optional_deps().is_empty());
    assert!(toc.saved_variables().is_empty());
    assert!(
        !toc.is_game_type_restricted(),
        "TOC omits `## AllowLoadGameType:` — the legal-compliance dialogs ship across every \
         flavor (the locale gate inside `localizeFrames` is the only filter)"
    );
    assert!(
        toc.is_glue_only(),
        "TOC declares `## AllowLoad: Glue` — `is_glue_only` (src/toc.rs:276) returns true. \
         The warnings only display before world-enter, never in-game"
    );
}

#[test]
fn blizzard_login_warning_dialogs_allow_load_glue_routes_to_glue_screens_only() {
    let toc = TocFile::from_file(&login_warning_dialogs_toc())
        .expect("Blizzard_LoginWarningDialogs TOC parses");
    assert!(
        !toc.allows_screen(ScreenKind::Game),
        "`## AllowLoad: Glue` routes through src/toc.rs:309 to `screen.is_glue()` — Game is \
         NOT a glue screen, so the addon must be excluded. The in-game UI does not show \
         legal-compliance dialogs (the player has already accepted them on the glue screen)"
    );
    for screen in [
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        assert!(
            toc.allows_screen(screen),
            "`## AllowLoad: Glue` allows every glue ScreenKind — Login (initial connection), \
             CharacterSelect (post-login dialogs may still be queued), and CharacterCreate \
             (legal-compliance dialogs persist across the glue surface). (Screen tested: \
             {screen:?})"
        );
    }
}

#[test]
fn blizzard_login_warning_dialogs_toc_raw_bytes_declare_glue_eager_with_three_required_deps() {
    let raw = std::fs::read_to_string(login_warning_dialogs_toc())
        .expect("Blizzard_LoginWarningDialogs TOC reads");
    assert!(
        raw.contains("## DefaultState: enabled"),
        "TOC must declare `## DefaultState: enabled` — eager-load by default on the glue surface"
    );
    assert!(
        raw.contains("## AllowLoad: Glue"),
        "TOC must declare `## AllowLoad: Glue` (capital G) — the case-insensitive matcher at \
         src/toc.rs:309 normalizes through `eq_ignore_ascii_case`, but the raw spelling is the \
         convention upstream Blizzard uses"
    );
    assert!(
        raw.contains(
            "## RequiredDep: Blizzard_SharedXML, Blizzard_GlueXMLBase, Blizzard_GlueParent"
        ),
        "TOC must declare the 3-dep `## RequiredDep:` line exactly — `RequiredDep` (singular) \
         is the canonical key Blizzard uses on this addon, NOT `RequiredDeps` / `Dependencies`"
    );
    assert!(
        !raw.contains("## LoadOnDemand"),
        "TOC must NOT declare `## LoadOnDemand:` — the addon is eager-load on glue screens"
    );
    assert!(
        !raw.contains("## SavedVariables"),
        "TOC must NOT declare `## SavedVariables:` — the wasAccepted / wasShown state lives on \
         the frame for the duration of the glue surface, not persisted to disk. Persistence is \
         handled per-warning via CVar (TaiwanFraudWarning's `doNotShowTWFraudWarning`)"
    );
}

#[test]
fn blizzard_login_warning_dialogs_toc_lists_three_files_in_load_order() {
    let toc = TocFile::from_file(&login_warning_dialogs_toc())
        .expect("Blizzard_LoginWarningDialogs TOC parses");
    assert_eq!(
        toc.files
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<_>>(),
        LOGIN_WARNING_DIALOGS_TOC_FILES,
        "TOC body must list exactly 3 files in this order: Localization.lua first (publishes \
         the `localizeFrames` per-locale callback table to SetupLocalization, but the \
         callbacks reference frames that don't exist yet — they fire later when the frames \
         have been instantiated), then Blizzard_LoginWarningDialogs.lua (declares the 4 mixin \
         tables: LoginWarningDialogBaseMixin + the 3 CreateFromMixins-derived per-locale \
         mixins ChinaAgeAppropriatenessWarningMixin / KoreanRatingsMixin / \
         TaiwanFraudWarningMixin), then Blizzard_LoginWarningDialogs.xml (instantiates the 3 \
         named frames, which must come AFTER the .lua so the `mixin=` attributes resolve)"
    );
}

#[test]
fn blizzard_login_warning_dialogs_directory_holds_four_entries() {
    let entries = std::fs::read_dir(login_warning_dialogs_dir())
        .expect("Blizzard_LoginWarningDialogs directory reads")
        .count();
    assert_eq!(
        entries, 4,
        "Directory must hold exactly 4 entries — 1 TOC + Localization.lua + the 2 main \
         addon files (Blizzard_LoginWarningDialogs.lua + Blizzard_LoginWarningDialogs.xml). \
         No subdirectories, no per-flavor variants — this addon is flavor-agnostic"
    );
}

#[test]
fn blizzard_login_warning_dialogs_auto_discovered_on_every_glue_screen() {
    for screen in [
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), screen);
        let found = addons
            .iter()
            .any(|(name, _)| name == "Blizzard_LoginWarningDialogs");
        assert!(
            found,
            "Blizzard_LoginWarningDialogs (## AllowLoad: Glue + ## DefaultState: enabled) must \
             be auto-discovered on every glue ScreenKind — the eager-load contract requires \
             the warning frames exist before GlueParent's per-screen show/hide pass. (Screen \
             tested: {screen:?})"
        );
    }
}

#[test]
fn blizzard_login_warning_dialogs_excluded_from_game_screen_discovery() {
    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    let in_game = addons
        .iter()
        .any(|(name, _)| name == "Blizzard_LoginWarningDialogs");
    assert!(
        !in_game,
        "Blizzard_LoginWarningDialogs must NOT appear in Game-screen auto-discovery — \
         `## AllowLoad: Glue` routes through `allows_screen` (src/toc.rs:309) to \
         `screen.is_glue()` which returns false for ScreenKind::Game. The legal-compliance \
         dialogs target the pre-world-enter glue surface only"
    );
}

#[test]
fn blizzard_login_warning_dialogs_loads_without_addon_specific_lua_errors() {
    let env = load_character_select_screen();

    let load_errors: Vec<String> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| {
            message.contains("Blizzard_LoginWarningDialogs")
                || message.contains("LoginWarningDialog")
                || message.contains("ChinaAgeAppropriatenessWarning")
                || message.contains("KoreanRatings")
                || message.contains("TaiwanFraudWarning")
        })
        .cloned()
        .collect();
    assert!(
        load_errors.is_empty(),
        "Blizzard_LoginWarningDialogs emitted addon-specific Lua errors during \
         CharacterSelect-screen load:\n  {}",
        load_errors.join("\n  ")
    );
}

#[test]
fn blizzard_login_warning_dialogs_is_addon_loaded_after_glue_discovery() {
    let env = load_character_select_screen();

    let loaded: bool = env
        .eval("return C_AddOns.IsAddOnLoaded('Blizzard_LoginWarningDialogs')")
        .expect("IsAddOnLoaded probe succeeds");
    assert!(
        loaded,
        "C_AddOns.IsAddOnLoaded('Blizzard_LoginWarningDialogs') must return true after the \
         CharacterSelect glue-screen boot — proves the eager `## AllowLoad: Glue` discovery \
         path registered the addon with the loaded-set without an explicit load_addon call"
    );
}

#[test]
fn blizzard_login_warning_dialogs_publishes_base_mixin_with_should_show_and_try_show() {
    let env = load_character_select_screen();

    let kind: String = env
        .eval("return type(LoginWarningDialogBaseMixin)")
        .expect("LoginWarningDialogBaseMixin probe succeeds");
    assert_eq!(
        kind, "table",
        "Blizzard_LoginWarningDialogs.lua line 1 publishes `LoginWarningDialogBaseMixin = {{}}` \
         — the abstract base mixin that the 3 per-locale mixins extend via CreateFromMixins"
    );

    for method in ["ShouldShow", "TryShow"] {
        let has_method: bool = env
            .eval(&format!(
                "return type(LoginWarningDialogBaseMixin.{method}) == 'function'"
            ))
            .expect("base mixin method probe succeeds");
        assert!(
            has_method,
            "LoginWarningDialogBaseMixin.{method} must be a function. The base mixin owns \
             exactly 2 methods: ShouldShow (returns false — overridden by every concrete \
             mixin) and TryShow (calls ShouldShow → Show + return true / Hide + return \
             false). The base ShouldShow is the explicit override-target documented inline"
        );
    }
}

#[test]
fn blizzard_login_warning_dialogs_china_mixin_inherits_base_and_owns_three_methods() {
    let env = load_character_select_screen();

    let kind: String = env
        .eval("return type(ChinaAgeAppropriatenessWarningMixin)")
        .expect("ChinaAgeAppropriatenessWarningMixin probe succeeds");
    assert_eq!(
        kind, "table",
        "Blizzard_LoginWarningDialogs.lua line 18 publishes \
         `ChinaAgeAppropriatenessWarningMixin = CreateFromMixins(LoginWarningDialogBaseMixin)` \
         — the zhCN-locale-gated dialog mixin"
    );

    for method in ["OnLoad", "ShouldShow", "OnAcknowledged", "TryShow"] {
        let has_method: bool = env
            .eval(&format!(
                "return type(ChinaAgeAppropriatenessWarningMixin.{method}) == 'function'"
            ))
            .expect("china mixin method probe succeeds");
        assert!(
            has_method,
            "ChinaAgeAppropriatenessWarningMixin.{method} must be a function — 3 own methods \
             (OnLoad wires OkayButton OnClick to GenerateClosure(self.OnAcknowledged, self), \
             ShouldShow gates on `localeMatches and not wasAccepted and not \
             C_Login.WasEverLauncherLogin()`, OnAcknowledged sets wasAccepted=true + Hide + \
             EventRegistry:TriggerEvent(\"LoginWarningDialogs.DialogClosed\")) plus the \
             TryShow method copied over from LoginWarningDialogBaseMixin by CreateFromMixins"
        );
    }
}

#[test]
fn blizzard_login_warning_dialogs_korean_mixin_inherits_base_and_owns_six_methods() {
    let env = load_character_select_screen();

    let kind: String = env
        .eval("return type(KoreanRatingsMixin)")
        .expect("KoreanRatingsMixin probe succeeds");
    assert_eq!(
        kind, "table",
        "Blizzard_LoginWarningDialogs.lua line 34 publishes \
         `KoreanRatingsMixin = CreateFromMixins(LoginWarningDialogBaseMixin)` — the \
         koKR-locale-gated dialog mixin (4 ratings icons + 3-second auto-close timer)"
    );

    for method in [
        "OnLoad",
        "OnEvent",
        "ScreenDisplayed",
        "ShouldShow",
        "OnShow",
        "OnUpdate",
        "TryShow",
    ] {
        let has_method: bool = env
            .eval(&format!(
                "return type(KoreanRatingsMixin.{method}) == 'function'"
            ))
            .expect("korean mixin method probe succeeds");
        assert!(
            has_method,
            "KoreanRatingsMixin.{method} must be a function — 6 own methods (OnLoad branches \
             on WasScreenFirstDisplayed → ScreenDisplayed or RegisterEvent \
             SCREEN_FIRST_DISPLAYED; OnEvent dispatches SCREEN_FIRST_DISPLAYED → \
             ScreenDisplayed; ScreenDisplayed installs OnUpdate; ShouldShow gates on \
             `localeMatches and (not wasShown or closeTimer)`; OnShow sets wasShown=true + \
             closeTimer=3; OnUpdate decrements closeTimer + auto-hides at 0) plus TryShow \
             from CreateFromMixins"
        );
    }
}

#[test]
fn blizzard_login_warning_dialogs_taiwan_mixin_inherits_base_and_owns_five_methods() {
    let env = load_character_select_screen();

    let kind: String = env
        .eval("return type(TaiwanFraudWarningMixin)")
        .expect("TaiwanFraudWarningMixin probe succeeds");
    assert_eq!(
        kind, "table",
        "Blizzard_LoginWarningDialogs.lua line 74 publishes \
         `TaiwanFraudWarningMixin = CreateFromMixins(LoginWarningDialogBaseMixin)` — the \
         zhTW-locale-gated dialog mixin (CVar-persisted DoNotShowAgainCheckbox + modal frame)"
    );

    for method in [
        "OnLoad",
        "ShouldShow",
        "OnShow",
        "OnHide",
        "OnAcknowledged",
        "TryShow",
    ] {
        let has_method: bool = env
            .eval(&format!(
                "return type(TaiwanFraudWarningMixin.{method}) == 'function'"
            ))
            .expect("taiwan mixin method probe succeeds");
        assert!(
            has_method,
            "TaiwanFraudWarningMixin.{method} must be a function — 5 own methods (OnLoad sets \
             disableHideOnEscape=true + wires OkayButton OnClick to OnAcknowledged via \
             GenerateClosure; ShouldShow gates on `localeMatches and not wasAccepted and not \
             GetCVarBool(self.noShowCvar)`; OnShow calls GlueParent_AddModalFrame; OnHide \
             persists DoNotShowAgainCheckbox state to CVar `doNotShowTWFraudWarning` + \
             GlueParent_RemoveModalFrame; OnAcknowledged sets wasAccepted=true + Hide + \
             EventRegistry trigger) plus TryShow from CreateFromMixins"
        );
    }
}

#[test]
fn blizzard_login_warning_dialogs_publishes_three_named_frames_with_glue_parent() {
    let env = load_character_select_screen();

    for frame_name in LOGIN_WARNING_FRAMES {
        let kind: String = env
            .eval(&format!("return type(_G['{frame_name}'])"))
            .expect("frame global probe succeeds");
        assert_eq!(
            kind, "table",
            "Blizzard_LoginWarningDialogs.xml declares 3 named non-virtual top-level frames \
             that must publish at `_G` after load — `{frame_name}` is one of them. All 3 use \
             `parent=GlueParent` + `frameStrata=DIALOG` + `hidden=true` + a Mixin matching \
             the locale gate applied via Localization.lua's `localizeFrames` callback"
        );

        let frame_get_name: String = env
            .eval(&format!("return _G['{frame_name}']:GetName()"))
            .expect("frame GetName probe succeeds");
        assert_eq!(
            frame_get_name, *frame_name,
            "_G['{frame_name}']:GetName() must return the literal frame name — proves the \
             frame is a real FrameRef userdata with the standard frame-method surface, NOT a \
             plain table assigned to the global"
        );
    }
}

#[test]
fn blizzard_login_warning_dialogs_three_frames_start_hidden_with_glue_parent() {
    let env = load_character_select_screen();

    for frame_name in LOGIN_WARNING_FRAMES {
        let parent_name: String = env
            .eval(&format!("return _G['{frame_name}']:GetParent():GetName()"))
            .expect("frame parent probe succeeds");
        assert_eq!(
            parent_name, "GlueParent",
            "{frame_name}:GetParent() must resolve to GlueParent — the XML attribute \
             `parent=GlueParent` (Blizzard_LoginWarningDialogs.xml lines 3, 78, 127) docks \
             every warning into the glue surface (not UIParent — UIParent is in-game only). \
             GlueParent comes from Blizzard_GlueParent which the dependency chain pulls in"
        );

        let starts_hidden: bool = env
            .eval(&format!("return not _G['{frame_name}']:IsShown()"))
            .expect("frame visibility probe succeeds");
        assert!(
            starts_hidden,
            "{frame_name} must start hidden — every warning frame XML declares `hidden=true` \
             so the frames only show when the per-locale `localeMatches` flag is set + \
             ShouldShow gating passes (TryShow is the entry point that flips the dialog \
             visible)"
        );
    }
}
