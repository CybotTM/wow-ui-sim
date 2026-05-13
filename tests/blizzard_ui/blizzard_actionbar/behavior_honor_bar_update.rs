//! Behavior pin: mutating `state.player_xp.honor` / `honor_max` and
//! `state.player.honor_level` then firing `HONOR_XP_UPDATE` makes
//! `HonorBarMixin:Update` re-read `UnitHonor("player")` /
//! `UnitHonorMax("player")` / `UnitHonorLevel("player")` and propagate
//! the values to the underlying `StatusBar`'s
//! `GetValue()` / `GetMinMaxValues()`.
//!
//! ## Source contract
//!
//! `HonorBarMixin:Update` (`Mainline/HonorBar.lua:7-12`) is straight-line:
//!
//! ```lua
//! function HonorBarMixin:Update()
//!     local current = UnitHonor("player");
//!     local maxHonor = UnitHonorMax("player");
//!     local level = UnitHonorLevel("player");
//!     self:SetBarValues(current, 0, maxHonor, level);
//! end
//! ```
//!
//! `HonorBarMixin:OnEvent` (lua:40-50) dispatches to `Update()` on
//! `PLAYER_ENTERING_WORLD`, `HONOR_XP_UPDATE`, `ZONE_CHANGED`, and
//! `ZONE_CHANGED_NEW_AREA`. `OnLoad` (lua:25-38) registers those events
//! plus `CVAR_UPDATE`, and the headless-startup harness fires
//! `PLAYER_ENTERING_WORLD` (`src/startup.rs::settle_headless_startup`),
//! so by the time the test runs `Update` has already been invoked once
//! with the seeded cold state (honor=0, honor_max=0, honor_level=0).
//!
//! `SetBarValues` (`Shared/StatusTrackingBar.lua:32-39`) takes the else
//! arm because `supportsAnimation=true` is declared on the inner
//! `<StatusBar parentKey="StatusBar">` of `StatusTrackingBarTemplate.xml:11`,
//! NOT on the outer bar — so `self.supportsAnimation` (outer) is nil
//! and the call resolves to a direct
//! `self.StatusBar:SetMinMaxValues(0, maxHonor)` +
//! `self.StatusBar:SetValue(current)`. The `level` argument is
//! computed but discarded by this dispatch path.
//!
//! ## Bar registration
//!
//! `Blizzard_ActionBar/Mainline/StatusTrackingManagerOverrides.lua:61`
//! adds the Honor bar via
//! `AddBar(StatusTrackingBarInfo.BarsEnum.Honor, "HonorStatusBarTemplate")`,
//! storing it at
//! `MainStatusTrackingBarContainer.bars[BarsEnum.Honor]` (BarsEnum.Honor=2
//! per `Shared/StatusTrackingManager.lua:5-13`). The
//! `HonorStatusBarTemplate` (`HonorBar.xml:3-12`) inherits
//! `StatusTrackingBarTemplate` and mixes in `HonorBarMixin`.
//!
//! ## Honor sources in the simulator
//!
//! - `UnitHonor` / `UnitHonorMax`
//!   (`globals/xp_honor_rest.rs:94-100, 143-144`) read
//!   `state.player_xp.honor` / `state.player_xp.honor_max` (i32).
//! - `UnitHonorLevel` (`globals/state_backed_queries.rs:177-180`) reads
//!   `state.player.honor_level` (i32).
//! - All three default to `0` per `PlayerXpState::default` and
//!   `PlayerState::seeded`, so cold-state pin is the all-zero baseline.
//!
//! ## Why fire `HONOR_XP_UPDATE` instead of calling `Update()` directly
//!
//! The dependency this task pins is "registration → dispatch", not just
//! "Update body": a regression in
//! `HonorBarMixin:OnLoad`-time `RegisterEvent("HONOR_XP_UPDATE")` (lua:27)
//! or the `OnEvent` arm at lua:46-49 would silently drop the refresh.
//! Calling `Update()` directly would mask both regressions. Firing the
//! event also exercises `lua_api::env::fire_event` → frame's `OnEvent`
//! dispatch, mirroring how the live game pushes `HONOR_XP_UPDATE` after
//! a kill in PvP.
//!
//! ## Observations
//!
//! 1. After `with_blizzard_addon_startup_shape(&[Blizzard_ActionBar])`,
//!    the chain
//!    `StatusTrackingBarManager.MainStatusTrackingBarContainer.bars[BarsEnum.Honor]`
//!    resolves with `.StatusBar`, BarsEnum.Honor == 2.
//! 2. Cold state (honor=0, honor_max=0, honor_level=0):
//!    `StatusBar:GetMinMaxValues() == (0, 0)`, `StatusBar:GetValue() == 0`.
//! 3. Seeding `state.player_xp.honor = 4500`, `honor_max = 10000`,
//!    `state.player.honor_level = 42` then firing `HONOR_XP_UPDATE`:
//!    `StatusBar:GetMinMaxValues() == (0, 10000)`,
//!    `StatusBar:GetValue() == 4500`.
//!
//! ## Regression candidates the assertions catch
//!
//! - `HonorBarMixin:OnLoad` drops the `HONOR_XP_UPDATE` registration →
//!   StatusBar values stay at the cold-state zeros after firing.
//! - `OnEvent` arm at lua:46-49 stops routing `HONOR_XP_UPDATE` to
//!   `Update()` → same symptom.
//! - `UnitHonor` / `UnitHonorMax` regress to read the wrong PlayerState
//!   field → StatusBar values mismatch the seeded numbers.
//! - `SetBarValues` else-branch regresses (`SetMinMaxValues`+`SetValue`)
//!   → `StatusBar:GetValue()`/`GetMinMaxValues()` stay at zero.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBar";

const HONOR_BAR_LUA: &str = "StatusTrackingBarManager.MainStatusTrackingBarContainer\
    .bars[StatusTrackingBarInfo.BarsEnum.Honor]";

const NEW_HONOR: i32 = 4_500;
const NEW_HONOR_MAX: i32 = 10_000;
const NEW_HONOR_LEVEL: i32 = 42;

#[test]
fn honor_bar_round_trips_through_honor_xp_update_event() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        assert_cold_globals_resolve(env);
        assert_cold_status_bar_zero(env);
        seed_player_honor(env);
        env.fire_event("HONOR_XP_UPDATE")
            .expect("HONOR_XP_UPDATE fire must dispatch cleanly");
        assert_post_event_status_bar_values(env);
    });
    }
}

fn assert_cold_globals_resolve(env: &WowLuaEnv) {
    let cold_globals_exist: bool = env
        .eval(
            r#"
            local manager = StatusTrackingBarManager
            local container = manager and manager.MainStatusTrackingBarContainer
            local idx = StatusTrackingBarInfo and StatusTrackingBarInfo.BarsEnum
                and StatusTrackingBarInfo.BarsEnum.Honor
            local bar = container and idx and container.bars and container.bars[idx]
            return manager ~= nil and container ~= nil and idx == 2
                and bar ~= nil and bar.StatusBar ~= nil
            "#,
        )
        .expect("honor bar global existence probe must run cleanly");
    assert!(
        cold_globals_exist,
        "Chain StatusTrackingBarManager.MainStatusTrackingBarContainer.\
         bars[BarsEnum.Honor].StatusBar must resolve after `{ROOT}` load; \
         nil reading means TOC/InitializeBars/parentKey regression — see \
         header observation 1."
    );
}

fn assert_cold_status_bar_zero(env: &WowLuaEnv) {
    let (min, max) = read_status_bar_min_max(env);
    assert_eq!(
        (min, max),
        (0.0, 0.0),
        "Cold `StatusBar:GetMinMaxValues()` must equal (0, 0) (seeded \
         honor_max=0); other reading means the cold PLAYER_ENTERING_WORLD \
         pass diverged — see header observation 2. Got ({min}, {max}).",
    );

    let value: f64 = env
        .eval(&format!("return {HONOR_BAR_LUA}.StatusBar:GetValue()"))
        .expect("cold StatusBar:GetValue probe must run cleanly");
    assert_eq!(
        value, 0.0,
        "Cold `StatusBar:GetValue()` must equal 0 (seeded honor=0); \
         non-zero means a stale value bled through — see header \
         observation 2. Got {value}.",
    );
}

fn seed_player_honor(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.player_xp.honor = NEW_HONOR;
    state.player_xp.honor_max = NEW_HONOR_MAX;
    state.player.honor_level = NEW_HONOR_LEVEL;
}

fn assert_post_event_status_bar_values(env: &WowLuaEnv) {
    let (min, max) = read_status_bar_min_max(env);
    assert_eq!(
        (min, max),
        (0.0, NEW_HONOR_MAX as f64),
        "Post-HONOR_XP_UPDATE `StatusBar:GetMinMaxValues()` must equal \
         (0, {NEW_HONOR_MAX}) (registration → OnEvent → Update → \
         UnitHonorMax → SetBarValues else-arm SetMinMaxValues chain) — \
         see header observation 3. Got ({min}, {max}).",
    );

    let value: f64 = env
        .eval(&format!("return {HONOR_BAR_LUA}.StatusBar:GetValue()"))
        .expect("post-event StatusBar:GetValue probe must run cleanly");
    assert_eq!(
        value, NEW_HONOR as f64,
        "Post-HONOR_XP_UPDATE `StatusBar:GetValue()` must equal \
         {NEW_HONOR} (Update → UnitHonor → SetBarValues else-arm \
         SetValue chain); reading 0 means the registration → dispatch \
         path dropped the event — see header observation 3. Got {value}.",
    );
}

fn read_status_bar_min_max(env: &WowLuaEnv) -> (f64, f64) {
    let min: f64 = env
        .eval(&format!(
            "local mn, _ = {HONOR_BAR_LUA}.StatusBar:GetMinMaxValues() return mn"
        ))
        .expect("StatusBar min probe must run cleanly");
    let max: f64 = env
        .eval(&format!(
            "local _, mx = {HONOR_BAR_LUA}.StatusBar:GetMinMaxValues() return mx"
        ))
        .expect("StatusBar max probe must run cleanly");
    (min, max)
}
