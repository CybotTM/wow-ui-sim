use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, find_toc_file, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}

fn load_locale_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_LoadLocale")
}

fn load_locale_toc() -> PathBuf {
    load_locale_dir().join("Blizzard_LoadLocale.toc")
}

const LOAD_LOCALE_TOC_FILES: &[&str] = &["LoadLocale.lua"];

fn load_full_game_ui() -> WowLuaEnv {
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

    env
}

#[test]
fn blizzard_load_locale_find_toc_resolves_bare_variant() {
    let resolved =
        find_toc_file(&load_locale_dir()).expect("Blizzard_LoadLocale TOC should resolve");
    assert_eq!(
        resolved,
        load_locale_toc(),
        "Blizzard_LoadLocale ships exactly one bare TOC. Locale-marker is a cross-flavor \
         concept (every WoW client / every screen needs to know which UI locale was \
         negotiated at startup), so there are no flavor-suffixed variants — the bare TOC \
         resolves via `find_toc_file` after the `_Mainline.toc` lookup misses"
    );
}

#[test]
fn blizzard_load_locale_toc_declares_default_state_enabled_with_allow_load_both() {
    let toc = TocFile::from_file(&load_locale_toc()).expect("Blizzard_LoadLocale TOC parses");
    assert!(
        !toc.is_load_on_demand(),
        "Blizzard_LoadLocale omits `## LoadOnDemand:` — `## DefaultState: enabled` makes \
         it an eager-load addon. The locale-marker globals MUST be live before any addon \
         that probes them runs (the marker is read by every L10n table consumer in the \
         Blizzard UI, so deferred-load is not viable)"
    );
    assert!(!toc.is_load_first());
    assert!(!toc.is_secure_env());
    assert!(
        toc.dependencies().is_empty(),
        "Zero `## Dependencies:` — the locale-marker addon is dependency-free. It only \
         publishes 2 globals (LOCALE_enUS = true, UI_LOCALE = \"enUS\") with no calls into \
         any external API surface"
    );
    assert!(toc.optional_deps().is_empty());
    assert!(
        toc.saved_variables().is_empty(),
        "Zero saved variables — the locale-marker is sourced from the build-time TOC \
         filename convention (Blizzard ships per-locale .toc copies of this addon — \
         enUS / deDE / esES / esMX / frFR / itIT / koKR / ptBR / ruRU / zhCN / zhTW — \
         each carrying the matching LOCALE_<code> = true assignment)"
    );
    assert!(
        !toc.is_game_type_restricted(),
        "TOC omits `## AllowLoadGameType:` — locale-marker is unrestricted across every \
         game flavor. The locale convention is a universal contract that every classic \
         and retail flavor honors"
    );

    for screen in [
        ScreenKind::Game,
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        assert!(
            toc.allows_screen(screen),
            "Blizzard_LoadLocale declares `## AllowLoad: Both` — `allows_screen` \
             (src/toc.rs:307) returns true for every ScreenKind when the value matches \
             `both` case-insensitively. The locale-marker MUST publish on glue screens \
             (the login + character-select code reads UI_LOCALE for date / number / \
             ordering formatting) AND on the game screen (every in-game L10n consumer \
             also reads it). The capitalized `Both` literal in the TOC normalizes through \
             `eq_ignore_ascii_case`. (Screen tested: {screen:?})"
        );
    }
}

#[test]
fn blizzard_load_locale_toc_declares_allow_load_both_with_capital_b_in_raw_bytes() {
    let raw = std::fs::read_to_string(load_locale_toc()).expect("Blizzard_LoadLocale TOC reads");
    assert!(
        raw.contains("## AllowLoad: Both"),
        "TOC must declare `## AllowLoad: Both` exactly with capitalized `Both`. This is one \
         of the few addons in the Blizzard tree that uses the capitalized variant — most \
         others ship lowercase `both`. The case-insensitive matcher at src/toc.rs:307 \
         normalizes through `eq_ignore_ascii_case`, so behavior is identical, but the raw \
         spelling is a visible reminder that the simulator's parser must tolerate either \
         capitalization"
    );
    assert!(
        raw.contains("## DefaultState: enabled"),
        "TOC must declare `## DefaultState: enabled` exactly — eager-load by default"
    );
    assert!(
        !raw.contains("## Dependencies"),
        "TOC must NOT declare `## Dependencies:` — the locale-marker is self-contained"
    );
    assert!(
        !raw.contains("## LoadOnDemand"),
        "TOC must NOT declare `## LoadOnDemand:` — DefaultState: enabled overrides any \
         deferred-load contract; the locale globals must be live before any L10n consumer \
         runs"
    );
}

#[test]
fn blizzard_load_locale_toc_lists_single_lua_file() {
    let toc = TocFile::from_file(&load_locale_toc()).expect("Blizzard_LoadLocale TOC parses");
    assert_eq!(
        toc.files
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<_>>(),
        LOAD_LOCALE_TOC_FILES,
        "TOC body must list exactly 1 file — LoadLocale.lua. The 3-line addon is one of \
         the smallest in the Blizzard tree: `LOCALE_enUS = true; UI_LOCALE = \"enUS\";` \
         with a single comment line above. No XML, no other Lua files, no Localization.lua"
    );
}

#[test]
fn blizzard_load_locale_directory_holds_two_entries_one_toc_one_lua() {
    let entries = std::fs::read_dir(load_locale_dir())
        .expect("Blizzard_LoadLocale directory reads")
        .count();
    assert_eq!(
        entries, 2,
        "Directory must hold exactly 2 entries — Blizzard_LoadLocale.toc and \
         LoadLocale.lua. The simulator ships only the enUS variant; on the real Blizzard \
         tree, parallel directories Blizzard_LoadLocale_deDE / esES / frFR / etc carry \
         per-locale copies of this addon, but only the locale matching the client's \
         negotiated UI language ships in the live install"
    );
}

#[test]
fn blizzard_load_locale_auto_discovered_on_every_screen() {
    for screen in [
        ScreenKind::Game,
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), screen);
        let found = addons.iter().any(|(name, _)| name == "Blizzard_LoadLocale");
        assert!(
            found,
            "Blizzard_LoadLocale must be auto-discovered on every ScreenKind. The \
             `## DefaultState: enabled` + `## AllowLoad: Both` combo + zero \
             game-type-restriction means every screen's discovery sweep picks it up into \
             the eager `addons` set (NOT the lod_pool — no LoadOnDemand). The locale \
             marker has to be live on every screen because the L10n surface is a \
             foundational global contract. (Screen tested: {screen:?})"
        );
    }
}

#[test]
fn blizzard_load_locale_loads_without_addon_specific_lua_errors() {
    let env = load_full_game_ui();

    let load_errors: Vec<String> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| {
            message.contains("Blizzard_LoadLocale")
                || message.contains("LoadLocale.lua")
                || message.contains("LOCALE_enUS")
                || message.contains("UI_LOCALE")
        })
        .cloned()
        .collect();
    assert!(
        load_errors.is_empty(),
        "Blizzard_LoadLocale emitted addon-specific Lua errors during load:\n  {}",
        load_errors.join("\n  ")
    );
}

#[test]
fn blizzard_load_locale_is_addon_loaded_after_auto_discovery() {
    let env = load_full_game_ui();

    let loaded: bool = env
        .eval("return C_AddOns.IsAddOnLoaded('Blizzard_LoadLocale')")
        .expect("IsAddOnLoaded probe succeeds");
    assert!(
        loaded,
        "C_AddOns.IsAddOnLoaded('Blizzard_LoadLocale') must return true after the eager \
         auto-discovery sweep — proves the locale-marker addon registers with the \
         loaded-set during the standard Game-screen boot pipeline, no explicit \
         load_addon call required"
    );
}

#[test]
fn blizzard_load_locale_publishes_locale_enus_global_as_true() {
    let env = load_full_game_ui();

    let kind: String = env
        .eval("return type(LOCALE_enUS)")
        .expect("LOCALE_enUS probe succeeds");
    assert_eq!(
        kind, "boolean",
        "LOCALE_enUS must publish at `_G` as a boolean — LoadLocale.lua line 2 sets the \
         locale-marker global to `true`. This is the canonical convention every Blizzard \
         L10n table consumer probes (e.g. `if LOCALE_enUS then ... end` to gate \
         locale-specific string overrides). NOT pre-stubbed by the simulator's runtime \
         bootstrap — the addon load is the only path that publishes it"
    );

    let value: bool = env
        .eval("return LOCALE_enUS")
        .expect("LOCALE_enUS value probe succeeds");
    assert!(
        value,
        "LOCALE_enUS must equal true exactly — the marker convention is `<locale> == true` \
         only for the active locale, all other locale globals stay nil. A consumer can \
         therefore probe `if LOCALE_enUS then` as a presence-check"
    );
}

#[test]
fn blizzard_load_locale_publishes_ui_locale_global_as_enus_string() {
    let env = load_full_game_ui();

    let kind: String = env
        .eval("return type(UI_LOCALE)")
        .expect("UI_LOCALE probe succeeds");
    assert_eq!(
        kind, "string",
        "UI_LOCALE must publish at `_G` as a string. The simulator's shared_bootstrap.lua \
         (lines 219-225) pre-seeds UI_LOCALE to `GetLocale()` or `\"enUS\"` if \
         GetLocale is unavailable; LoadLocale.lua line 3 then re-assigns \
         `UI_LOCALE = \"enUS\"` unconditionally, which idempotently confirms the value \
         post-load"
    );

    let value: String = env
        .eval("return UI_LOCALE")
        .expect("UI_LOCALE value probe succeeds");
    assert_eq!(
        value, "enUS",
        "UI_LOCALE must equal exactly the string `enUS`. The 4-character locale code is \
         the canonical Blizzard format (lowercase region prefix + 2-letter uppercase \
         country suffix — enUS / deDE / esES / esMX / frFR / itIT / koKR / ptBR / ruRU / \
         zhCN / zhTW). UI_LOCALE feeds the `localeTable = l10nTable[UI_LOCALE]` lookup \
         pattern at shared_bootstrap.lua line 238 inside SetupLocalization"
    );
}
