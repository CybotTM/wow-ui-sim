use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, find_toc_file};
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}

fn level_up_display_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_LevelUpDisplay")
}

fn level_up_display_mists_toc() -> PathBuf {
    level_up_display_dir().join("Blizzard_LevelUpDisplay_Mists.toc")
}

const LEVEL_UP_DISPLAY_TOC_FILES: &[&str] = &[
    "Mists/LevelUpDisplayConstants.lua",
    "Cata/LevelUpDisplay.lua",
    "Mists/LevelUpDisplay.xml",
    "Localization.lua",
];

#[test]
fn blizzard_level_up_display_find_toc_returns_none_on_mainline_target() {
    let resolved = find_toc_file(&level_up_display_dir());
    assert!(
        resolved.is_none(),
        "Blizzard_LevelUpDisplay must resolve to None on the simulator's Mainline target. \
         The directory ships only `Blizzard_LevelUpDisplay_Mists.toc` (no bare-name TOC, \
         no `_Mainline.toc`); `find_toc_file` (src/loader/mod.rs:65-94) walks \
         `_Mainline.toc` then bare name then a fallback that explicitly EXCLUDES `_Cata` \
         / `_Wrath` / `_TBC` / `_Vanilla` / `_Mists` suffixes — so a Mists-only addon \
         returns None on every variant. The retail level-up surface is delivered by \
         Blizzard_ScenarioPlayerJoinFrame / Blizzard_PlayerChoice / the expansion-feature \
         toaster instead. Got: {resolved:?}"
    );
}

#[test]
fn blizzard_level_up_display_toc_declares_default_state_enabled_and_game_screen_only() {
    let toc =
        TocFile::from_file(&level_up_display_mists_toc()).expect("Blizzard_LevelUpDisplay parses");
    assert!(
        !toc.is_load_on_demand(),
        "Blizzard_LevelUpDisplay omits `## LoadOnDemand:` — `## DefaultState: enabled` makes \
         it an eager-load addon (it must be ready before the player levels up at all, \
         which can fire seconds after entering the world). The level-up splash banner \
         must be live for the very first PLAYER_LEVEL_UP event of the session, so \
         deferred-load is not viable"
    );
    assert!(!toc.is_load_first());
    assert!(!toc.is_secure_env());
    assert!(
        toc.dependencies().is_empty(),
        "Zero `## Dependencies:` — the level-up overlay relies only on the global runtime \
         surface: PLAYER_LEVEL_UP / PLAYER_LEVEL_CHANGED events, the LevelUp* texture \
         atlas under Interface/LevelUp/, GameFontNormalLarge / GameFontNormal / \
         GameFont_Gigantic / SystemFont_Shadow_Large fonts, and the LevelUpDisplay_AnimStep \
         / LevelUpDisplaySide_AnimStep / LevelUpDisplay_OnLoad / LevelUpDisplay_OnEvent / \
         LevelUpDisplaySide_OnHide / OnShow / Remove free-helper globals — these globals \
         are defined in `Cata/LevelUpDisplay.lua` (the shared Cata/Mists source pulled \
         from the upstream Blizzard tree) which is referenced by the TOC body but \
         intentionally NOT shipped in the simulator's Mists-only sparse checkout"
    );
    assert!(toc.optional_deps().is_empty());
    assert!(
        toc.saved_variables().is_empty(),
        "Zero saved variables — the level-up overlay is purely transient (the splash \
         banner shows for ~3 seconds after each level-up and dies). No state worth \
         persisting across sessions"
    );
    assert!(
        toc.allows_screen(ScreenKind::Game),
        "`## AllowLoad: game` makes the addon eligible for the in-game screen — \
         allows_screen returns true for Game when the metadata value is exactly \
         `game` (case-insensitive match at src/toc.rs:308)"
    );
    for screen in [
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        assert!(
            !toc.allows_screen(screen),
            "Blizzard_LevelUpDisplay must NOT publish on glue screens. `## AllowLoad: game` \
             rejects every non-Game ScreenKind. The level-up splash is in-game UI only — \
             you cannot level up at the login or character-select screens. (Screen tested: \
             {screen:?})"
        );
    }
}

#[test]
fn blizzard_level_up_display_toc_declares_mists_only_game_type_restriction() {
    let toc =
        TocFile::from_file(&level_up_display_mists_toc()).expect("Blizzard_LevelUpDisplay parses");
    assert!(
        toc.is_game_type_restricted(),
        "Blizzard_LevelUpDisplay declares `## AllowLoadGameType: mists`. \
         `is_game_type_restricted` (src/toc.rs:294-302) returns true when the value is \
         not in the {{mainline, standard}} set — `mists` falls outside that set, so the \
         addon is build-time gated to the Mists-of-Pandaria classic flavor. On a Mainline \
         simulator target this addon must be filtered out of every auto-discovery pass, \
         and the simulator never even attempts to load the missing Cata/LevelUpDisplay.lua \
         file referenced in the TOC body"
    );

    let raw = std::fs::read_to_string(level_up_display_mists_toc())
        .expect("Blizzard_LevelUpDisplay TOC reads");
    assert!(
        raw.contains("## AllowLoadGameType: mists"),
        "TOC must declare `## AllowLoadGameType: mists` exactly — the metadata value drives \
         the is_game_type_restricted gate. Distinct from the `mainline` / `standard` \
         retail values that the discovery pass treats as unrestricted"
    );
    assert!(
        raw.contains("## AllowLoad: game"),
        "TOC must declare `## AllowLoad: game` exactly — game-screen-only, the level-up \
         overlay never runs on glue screens"
    );
    assert!(
        raw.contains("## DefaultState: enabled"),
        "TOC must declare `## DefaultState: enabled` exactly — eager-load by default, \
         not LoadOnDemand"
    );
    assert!(
        !raw.contains("## Dependencies"),
        "TOC must NOT declare `## Dependencies:` — the level-up overlay is dependency-free"
    );
    assert!(
        !raw.contains("## LoadOnDemand"),
        "TOC must NOT declare `## LoadOnDemand:` — DefaultState: enabled overrides any \
         deferred-load contract; the addon needs to be live before the first PLAYER_LEVEL_UP"
    );
}

#[test]
fn blizzard_level_up_display_toc_lists_four_files_with_cross_flavor_cata_reference() {
    let toc =
        TocFile::from_file(&level_up_display_mists_toc()).expect("Blizzard_LevelUpDisplay parses");
    assert_eq!(
        toc.files
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<_>>(),
        LEVEL_UP_DISPLAY_TOC_FILES,
        "TOC body must list exactly 4 files in this order: \
         Mists/LevelUpDisplayConstants.lua (the LEVEL_UP_EVENTS / LEVEL_UP_CLASS_HACKS \
         table — per-level unlock map keyed by character level + class+race composite \
         key for race/class-specific unlock surfaces like Plate-at-40 for warriors / \
         paladin mounts at 20/40), Cata/LevelUpDisplay.lua (the shared Cata/Mists \
         implementation that defines the LevelUpDisplay_OnLoad / OnEvent dispatch and \
         the LevelUpDisplaySide animation pipeline — referenced from the cross-flavor \
         source tree where Cata and Mists share a single Lua file), \
         Mists/LevelUpDisplay.xml (the 549-line frame XML defining the LevelUpSkillTemplate \
         virtual template plus the LevelUpDisplay + LevelUpDisplaySide named non-virtual \
         frames with the showAnim / sideAnimIn AnimationGroups + per-FontString slot \
         hierarchy for the level-banner / scenario-frame / challenge-mode-frame / \
         spell-frame / pet-battle-rarity surfaces), Localization.lua (empty string-table \
         placeholder — kept around for the localization toolchain even though the actual \
         strings ship via the GlobalStrings registry). Backslashes in the source TOC body \
         (`Mists\\LevelUpDisplayConstants.lua`) are normalized to forward slashes by \
         `push_file_entry` (src/toc.rs:147 — `replace('\\\\', \"/\")`), so the resolved \
         file paths are platform-portable"
    );
}

#[test]
fn blizzard_level_up_display_cata_subdir_is_intentionally_missing_in_sparse_checkout() {
    assert!(
        !level_up_display_dir().join("Cata").exists(),
        "Cata/ subdir must NOT exist in the simulator's sparse checkout. The TOC body \
         references `Cata/LevelUpDisplay.lua` (the shared Cata/Mists implementation) but \
         the upstream Blizzard tree forks the `Cata` and `Mists` source flavors at the \
         subdir level — the simulator's Mists-only sparse checkout intentionally omits \
         the Cata/ subtree to keep the working copy size minimal. The is_game_type_restricted \
         gate ensures the simulator never attempts to load the missing file: on a Mainline \
         target the addon is filtered out of auto-discovery before load_addon would walk \
         the file list, so the `Cata/LevelUpDisplay.lua` reference stays a TOC declaration \
         that no code path ever resolves"
    );
}

#[test]
fn blizzard_level_up_display_directory_holds_three_entries_one_toc_one_lua_one_subdir() {
    let entries: Vec<String> = std::fs::read_dir(level_up_display_dir())
        .expect("Blizzard_LevelUpDisplay directory reads")
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        entries.len(),
        3,
        "Directory must hold exactly 3 entries — Blizzard_LevelUpDisplay_Mists.toc + \
         Localization.lua + Mists/ subdir. The Cata/ subdir is intentionally absent (the \
         simulator's sparse checkout drops the Cata flavor branch). Actual entries: {:?}",
        entries
    );
}

#[test]
fn blizzard_level_up_display_mists_subdir_holds_constants_and_xml_only() {
    let mists_dir = level_up_display_dir().join("Mists");
    let mut entries: Vec<String> = std::fs::read_dir(&mists_dir)
        .expect("Mists/ subdir reads")
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    entries.sort();
    assert_eq!(
        entries,
        vec![
            "LevelUpDisplay.xml".to_string(),
            "LevelUpDisplayConstants.lua".to_string(),
        ],
        "Mists/ subdir must hold exactly 2 files — LevelUpDisplay.xml (the 549-line frame \
         definition) and LevelUpDisplayConstants.lua (the 62-line LEVEL_UP_EVENTS + \
         LEVEL_UP_CLASS_HACKS table). The shared Cata/Mists Lua implementation lives in \
         the missing Cata/ subdir, intentionally absent. Actual entries: {:?}",
        entries
    );
}

#[test]
fn blizzard_level_up_display_localization_lua_is_comment_only_placeholder() {
    let localization = level_up_display_dir().join("Localization.lua");
    let contents =
        std::fs::read_to_string(&localization).expect("Localization.lua should be readable");
    assert_eq!(
        contents.trim(),
        "-- This file is executed at the end of addon load",
        "Localization.lua must be a single-comment placeholder. Blizzard's localization \
         toolchain expects every addon to ship a Localization.lua file referenced in the \
         TOC body — when the level-up overlay needs no per-addon strings (it sources \
         everything from the global GlobalStrings registry: LEVEL_UP / SPELL_LEVEL_UP / \
         etc), the file ships as a single comment-line marker. The placeholder still \
         executes at addon-load time (it lands LAST in the TOC body, after the XML), \
         which is the canonical signal-of-completion for the localization tool's \
         end-of-load hook"
    );
}

#[test]
fn blizzard_level_up_display_excluded_from_every_screen_due_to_mists_game_type() {
    for screen in [
        ScreenKind::Game,
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), screen);
        let found = addons
            .iter()
            .any(|(name, _)| name == "Blizzard_LevelUpDisplay");
        assert!(
            !found,
            "Blizzard_LevelUpDisplay must be filtered out of auto-discovery on every \
             ScreenKind. The `## AllowLoadGameType: mists` declaration trips the \
             is_game_type_restricted gate at src/loader/mod.rs:527 — the discovery pass \
             skips the addon entirely (NOT routed to the LoD pool either, because the \
             gate is checked before the LoadOnDemand split). On a Mainline simulator \
             target the Mists-only addon stays out of every eager + every LoD pull. \
             (Screen tested: {screen:?})"
        );
    }
}
