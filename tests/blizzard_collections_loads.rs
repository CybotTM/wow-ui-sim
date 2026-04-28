use std::path::PathBuf;

use wow_ui_sim::loader::discover_blizzard_addons_for_screen;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn collections_toc() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_Collections/Blizzard_Collections_Mainline.toc")
}

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
fn blizzard_collections_is_load_on_demand_not_in_discovery() {
    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);

    let auto_loaded = addons
        .iter()
        .any(|(name, _)| name == "Blizzard_Collections");
    assert!(
        !auto_loaded,
        "Blizzard_Collections is `## LoadOnDemand: 1` and must NOT appear in Game-screen \
         auto-discovery (it is loaded explicitly when the player opens the journal)"
    );
}

const KNOWN_GET_NUM_EXPANSIONS_GAP: &str =
    "attempt to call global 'GetNumExpansions' (a nil value)";

fn unexpected_collections_errors(env: &WowLuaEnv) -> Vec<String> {
    env.state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| {
            message.contains("Blizzard_Collections")
                && !message.contains(KNOWN_GET_NUM_EXPANSIONS_GAP)
        })
        .cloned()
        .collect()
}

#[test]
fn blizzard_collections_loads_via_explicit_load() {
    let env = load_full_game_ui();

    {
        let mut state = env.state().borrow_mut();
        state.lua_errors.clear();
        state.lua_error_records.clear();
        state.lua_error_counts.clear();
    }

    load_addon(&env.loader_env(), &collections_toc())
        .expect("Blizzard_Collections should load via Rust loader");

    let collections_errors = unexpected_collections_errors(&env);
    assert!(
        collections_errors.is_empty(),
        "Blizzard_Collections emitted unexpected Lua errors during load:\n  {}",
        collections_errors.join("\n  ")
    );
}

#[test]
fn blizzard_collections_top_level_frames_are_defined() {
    let env = load_full_game_ui();

    load_addon(&env.loader_env(), &collections_toc()).expect("Blizzard_Collections should load");

    let frames_present: bool = env
        .eval(
            "return CollectionsJournal ~= nil \
                and MountJournal ~= nil \
                and PetJournal ~= nil \
                and ToyBox ~= nil \
                and HeirloomsJournal ~= nil \
                and WardrobeCollectionFrame ~= nil \
                and WarbandSceneJournal ~= nil",
        )
        .expect("frame query should succeed");
    assert!(
        frames_present,
        "All six Collections journal frames (CollectionsJournal, MountJournal, PetJournal, \
         ToyBox, HeirloomsJournal, WardrobeCollectionFrame, WarbandSceneJournal) should be \
         defined after load"
    );

    let tabs_present: bool = env
        .eval(
            "return CollectionsJournal.MountsTab ~= nil \
                and CollectionsJournal.PetsTab ~= nil \
                and CollectionsJournal.ToysTab ~= nil \
                and CollectionsJournal.HeirloomsTab ~= nil \
                and CollectionsJournal.WardrobeTab ~= nil \
                and CollectionsJournal.WarbandScenesTab ~= nil",
        )
        .expect("tab query should succeed");
    assert!(
        tabs_present,
        "CollectionsJournal should expose its six tab parentKeys after XML load"
    );
}

#[test]
fn blizzard_collections_mixins_are_defined() {
    let env = load_full_game_ui();

    load_addon(&env.loader_env(), &collections_toc()).expect("Blizzard_Collections should load");

    let mixins_present: bool = env
        .eval(
            "return type(HeirloomsMixin) == 'table' \
                and type(WarbandSceneJounalMixin) == 'table' \
                and type(MountEquipmentButtonMixin) == 'table' \
                and type(SuppressedMountEquipmentButtonMixin) == 'table' \
                and type(MountJournalSummonRandomFavoriteSpellFrameMixin) == 'table'",
        )
        .expect("mixin query should succeed");
    assert!(
        mixins_present,
        "Top-level Collections mixins should be populated after load"
    );
}

#[test]
fn blizzard_collections_journal_helpers_are_defined() {
    let env = load_full_game_ui();

    load_addon(&env.loader_env(), &collections_toc()).expect("Blizzard_Collections should load");

    let helpers_present: bool = env
        .eval(
            "return type(CollectionsJournal_SetTab) == 'function' \
                and type(CollectionsJournal_GetTab) == 'function' \
                and type(CollectionsJournal_ValidateTab) == 'function' \
                and type(CollectionsJournal_UpdateSelectedTab) == 'function' \
                and type(CollectionsJournal_OnShow) == 'function' \
                and type(CollectionsJournal_OnHide) == 'function'",
        )
        .expect("helper query should succeed");
    assert!(
        helpers_present,
        "Six top-level CollectionsJournal helper functions should be defined after load"
    );
}

/// Regression: opening the Wardrobe (Appearances) tab must populate the
/// items collection with at least one appearance for the active slot.
/// Earlier this returned 0 because `IsUnitModelReadyForUI`,
/// `SetUseTransmogSkin`, `IsSlotAllowed`, and friends were missing — the
/// `ChangeModelsSlot`/`SetActiveCategory` chain bailed out before
/// `RefreshVisualsList` ran.
#[test]
fn wardrobe_appearances_panel_populates_for_head_slot() {
    let env = load_full_game_ui();
    load_addon(&env.loader_env(), &collections_toc()).expect("Blizzard_Collections should load");

    env.eval::<()>("CollectionsJournal:Show(); CollectionsJournal_SetTab(CollectionsJournal, 5)")
        .expect("opening the Appearances tab should not error");

    let active_category: f64 = env
        .eval("return WardrobeCollectionFrame.ItemsCollectionFrame.activeCategory or -1")
        .expect("activeCategory query should succeed");
    assert!(
        active_category > 0.0,
        "ItemsCollectionFrame.activeCategory should be set (>0) after opening the wardrobe, got {active_category}"
    );

    let filtered_count: f64 = env
        .eval("return #(WardrobeCollectionFrame.ItemsCollectionFrame.filteredVisualsList or {})")
        .expect("filteredVisualsList length query should succeed");
    assert!(
        filtered_count > 0.0,
        "filteredVisualsList should contain at least one appearance for the default head slot, got {filtered_count}"
    );

    let first_visible: bool = env
        .eval(
            "local m = WardrobeCollectionFrame.ItemsCollectionFrame.Models \
             return m and m[1] and m[1]:IsShown() or false",
        )
        .expect("first model query should succeed");
    assert!(
        first_visible,
        "First appearance tile (Models[1]) should be visible after the wardrobe populates"
    );
}
