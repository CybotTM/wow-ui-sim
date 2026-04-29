#![cfg(feature = "client-retail")]
use std::path::PathBuf;

use wow_ui_sim::loader::discover_blizzard_addons_for_screen;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
}

fn cooldown_broadcaster_toc() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_CooldownBroadcaster/Blizzard_CooldownBroadcaster.toc")
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
fn blizzard_cooldown_broadcaster_toc_is_lod_and_mainline_only() {
    let toc = TocFile::from_file(&cooldown_broadcaster_toc())
        .expect("Blizzard_CooldownBroadcaster TOC should parse");
    assert!(
        toc.is_load_on_demand(),
        "Blizzard_CooldownBroadcaster declares `## LoadOnDemand: 1` (the MDI commentator-relay \
         addon is only loaded on demand by the esports observer client, not at startup)"
    );
    assert!(
        !toc.is_game_type_restricted(),
        "Blizzard_CooldownBroadcaster declares `## AllowLoadGameType: standard`; the simulator \
         treats both `mainline` and `standard` as the live retail game type, so this should NOT \
         be flagged as game-type-restricted"
    );
    assert!(
        !toc.is_secure_env(),
        "Blizzard_CooldownBroadcaster does not declare UseSecureEnvironment"
    );
}

#[test]
fn blizzard_cooldown_broadcaster_is_absent_from_auto_discovery() {
    let game_addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    let in_game = game_addons
        .iter()
        .any(|(name, _)| name == "Blizzard_CooldownBroadcaster");
    assert!(
        !in_game,
        "Blizzard_CooldownBroadcaster is `## LoadOnDemand: 1`, so it must NOT appear in \
         Game-screen auto-discovery — UIParentLoadAddOn loads it explicitly when MDI sync is \
         requested"
    );
}

#[test]
fn blizzard_cooldown_broadcaster_loads_via_load_addon_without_errors() {
    let env = load_full_game_ui();

    {
        let mut state = env.state().borrow_mut();
        state.lua_errors.clear();
        state.lua_error_records.clear();
        state.lua_error_counts.clear();
    }

    load_addon(&env.loader_env(), &cooldown_broadcaster_toc())
        .expect("Blizzard_CooldownBroadcaster should load via Rust loader");

    let load_errors: Vec<String> = env.state().borrow().lua_errors.clone();
    assert!(
        load_errors.is_empty(),
        "Blizzard_CooldownBroadcaster emitted Lua errors during explicit load:\n  {}",
        load_errors.join("\n  ")
    );
}

#[test]
fn blizzard_cooldown_broadcaster_frame_is_created_and_carries_mixin_methods() {
    let env = load_full_game_ui();
    load_addon(&env.loader_env(), &cooldown_broadcaster_toc())
        .expect("Blizzard_CooldownBroadcaster should load via Rust loader");

    let frame_present: bool = env
        .eval(
            "local f = _G.CooldownBroadcasterFrame; \
             return type(f) == 'table' \
                and type(f.GetSupportedTrackedSpells) == 'function' \
                and type(f.GetChannel) == 'function' \
                and type(f.SendComm) == 'function' \
                and type(f.SendINF) == 'function' \
                and type(f.RefreshCooldownSyncIDs) == 'function' \
                and type(f.GetSpellCooldown) == 'function' \
                and type(f.EnableSync) == 'function' \
                and type(f.DisableSync) == 'function' \
                and type(f.ShouldSyncBeEnabled) == 'function' \
                and type(f.UpdateSyncState) == 'function' \
                and type(f.BuildCooldownPayload) == 'function' \
                and type(f.FlushCooldownsIfChanged) == 'function' \
                and type(f.ADDON_LOADED) == 'function' \
                and type(f.PLAYER_ENTERING_WORLD) == 'function' \
                and type(f.GROUP_ROSTER_UPDATE) == 'function' \
                and type(f.SPELLS_CHANGED) == 'function' \
                and type(f.SPELL_UPDATE_COOLDOWN) == 'function' \
                and type(f.OnLoad) == 'function'",
        )
        .expect("CooldownBroadcasterFrame method query should succeed");
    assert!(
        frame_present,
        "Blizzard_CooldownBroadcaster.lua creates an anonymous `CreateFrame(\"Frame\")`, mixes \
         in the local `CooldownSyncRelayMixin`, and exports it as `CooldownBroadcasterFrame`. \
         After load the frame should expose all 18 mixin methods (12 helpers — \
         GetSupportedTrackedSpells / GetChannel / SendComm / SendINF / RefreshCooldownSyncIDs / \
         GetSpellCooldown / EnableSync / DisableSync / ShouldSyncBeEnabled / UpdateSyncState / \
         BuildCooldownPayload / FlushCooldownsIfChanged — plus 5 event handlers \
         ADDON_LOADED / PLAYER_ENTERING_WORLD / GROUP_ROSTER_UPDATE / SPELLS_CHANGED / \
         SPELL_UPDATE_COOLDOWN, plus OnLoad)"
    );
}

#[test]
fn blizzard_cooldown_broadcaster_state_is_initialized_by_on_load() {
    let env = load_full_game_ui();
    load_addon(&env.loader_env(), &cooldown_broadcaster_toc())
        .expect("Blizzard_CooldownBroadcaster should load via Rust loader");

    let state_initialized: bool = env
        .eval(
            "local f = CooldownBroadcasterFrame; \
             return type(f.cooldownSyncIDs) == 'table' \
                and type(f.cooldownSyncOrder) == 'table' \
                and f.syncEnabled == false",
        )
        .expect("state init query should succeed");
    assert!(
        state_initialized,
        "OnLoad runs at end-of-file via `CooldownBroadcasterFrame:OnLoad()` and should \
         initialize `cooldownSyncIDs={{}}` (per-spell prev-state cache), \
         `cooldownSyncOrder={{}}` (ordered list capped at MAX_COOLDOWNS=5), and \
         `syncEnabled=false`"
    );
}

#[test]
fn blizzard_cooldown_broadcaster_get_channel_returns_nil_when_solo() {
    let env = load_full_game_ui();
    load_addon(&env.loader_env(), &cooldown_broadcaster_toc())
        .expect("Blizzard_CooldownBroadcaster should load via Rust loader");

    // Force the global group-state queries into the solo state so the test is
    // deterministic regardless of whether other addons populated a fake party
    // for their own startup pipelines.
    let solo_channel_is_nil: bool = env
        .eval(
            "IsInRaid = function() return false end; \
             IsInGroup = function() return false end; \
             return CooldownBroadcasterFrame:GetChannel() == nil \
                and CooldownBroadcasterFrame:ShouldSyncBeEnabled() == false",
        )
        .expect("GetChannel solo query should succeed");
    assert!(
        solo_channel_is_nil,
        "When IsInRaid() and IsInGroup() both report false, `GetChannel()` must return nil \
         (the `IsInRaid() and 'RAID' or (IsInGroup() and 'PARTY' or nil)` chain bottoms out) \
         and `ShouldSyncBeEnabled()` must return false (the sync is solo-disabled, so the \
         relay never spams chat with cooldown payloads outside groups)"
    );

    // Conversely, in a party we should pick `PARTY` and ShouldSyncBeEnabled
    // should still gate on whether we actually have any tracked spells for the
    // current spec.
    let party_channel_is_party: bool = env
        .eval(
            "IsInRaid = function() return false end; \
             IsInGroup = function(_) return true end; \
             return CooldownBroadcasterFrame:GetChannel() == 'PARTY'",
        )
        .expect("GetChannel party query should succeed");
    assert!(
        party_channel_is_party,
        "With IsInGroup() reporting true and IsInRaid() reporting false, `GetChannel()` should \
         pick 'PARTY' (the `IsInRaid() and 'RAID' or (IsInGroup() and 'PARTY' or nil)` chain \
         resolves to the second branch when not in a raid)"
    );

    // And in a raid, the channel should be `RAID`.
    let raid_channel_is_raid: bool = env
        .eval(
            "IsInRaid = function() return true end; \
             IsInGroup = function(_) return true end; \
             return CooldownBroadcasterFrame:GetChannel() == 'RAID'",
        )
        .expect("GetChannel raid query should succeed");
    assert!(
        raid_channel_is_raid,
        "With IsInRaid() reporting true, `GetChannel()` should pick 'RAID' as the highest \
         priority — even though IsInGroup() also returns true, the `RAID` branch wins"
    );
}

#[test]
fn blizzard_cooldown_broadcaster_tracked_spec_table_carries_outlaw_rogue_cooldowns() {
    let env = load_full_game_ui();
    load_addon(&env.loader_env(), &cooldown_broadcaster_toc())
        .expect("Blizzard_CooldownBroadcaster should load via Rust loader");

    // The TrackedCooldownsBySpec table lives in the addon's private namespace.
    // We can only observe it indirectly: SetSpecialization to 260 (Outlaw Rogue),
    // refresh the sync IDs, and check that the resulting order list contains the
    // configured spell IDs whose `IsSpellKnown` happens to be true. Because the
    // sim's SpellBook stub returns true for every spell ID, this should pull in
    // all 5 Outlaw entries in order, capped at MAX_COOLDOWNS=5.
    let outlaw_order: bool = env
        .eval(
            "C_SpecializationInfo.GetSpecialization = function() return 1 end; \
             C_SpecializationInfo.GetSpecializationInfo = function() return 260 end; \
             C_SpellBook.IsSpellKnown = function() return true end; \
             CooldownBroadcasterFrame:RefreshCooldownSyncIDs(); \
             local o = CooldownBroadcasterFrame.cooldownSyncOrder; \
             return #o == 5 \
                and o[1] == 13750 and o[2] == 2094 and o[3] == 31224 \
                and o[4] == 1966  and o[5] == 20572",
        )
        .expect("RefreshCooldownSyncIDs query should succeed");
    assert!(
        outlaw_order,
        "TrackedCooldowns.lua declares `namespace.TrackedCooldownsBySpec[260] = {{13750, 2094, \
         31224, 1966, 20572}}` (Adrenaline Rush / Blind / Cloak of Shadows / Feint / Blood \
         Fury). Forcing the player into spec 260 (Outlaw Rogue) and stubbing IsSpellKnown to \
         true should leave `cooldownSyncOrder` populated in the same order, capped at \
         MAX_COOLDOWNS=5"
    );
}

#[test]
fn blizzard_cooldown_broadcaster_warrior_specs_share_blood_fury_only() {
    let env = load_full_game_ui();
    load_addon(&env.loader_env(), &cooldown_broadcaster_toc())
        .expect("Blizzard_CooldownBroadcaster should load via Rust loader");

    let warrior_orders: bool = env
        .eval(
            "C_SpellBook.IsSpellKnown = function() return true end; \
             local function order_for(specID) \
               C_SpecializationInfo.GetSpecialization = function() return 1 end; \
               C_SpecializationInfo.GetSpecializationInfo = function() return specID end; \
               CooldownBroadcasterFrame:RefreshCooldownSyncIDs(); \
               return CooldownBroadcasterFrame.cooldownSyncOrder; \
             end; \
             local arms = order_for(71); \
             local fury = order_for(72); \
             local prot = order_for(73); \
             return #arms == 1 and arms[1] == 20572 \
                and #fury == 1 and fury[1] == 20572 \
                and #prot == 1 and prot[1] == 20572",
        )
        .expect("warrior spec order query should succeed");
    assert!(
        warrior_orders,
        "TrackedCooldownsBySpec for Warrior specs 71 (Arms), 72 (Fury), 73 (Protection) each \
         contain only spell 20572 (Blood Fury racial). RefreshCooldownSyncIDs should produce a \
         length-1 order list containing exactly that spell ID for each warrior spec"
    );
}
