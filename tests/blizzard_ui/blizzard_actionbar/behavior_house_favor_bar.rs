//! Behavior pin: seeding `state.housing.tracked_house_guid` plus
//! `state.housing.level_thresholds`, calling `bar:Show()` to drive the
//! lazy `OnShow` event-registration, and firing `HOUSE_LEVEL_FAVOR_UPDATED`
//! with a matching `houseGUID` payload makes `HouseFavorBarMixin:OnEvent`
//! store the payload on `self.houseLevelFavor`, then `:Update` re-reads
//! `C_Housing.GetHouseLevelFavorForLevel(level)` /
//! `(level + 1)` and propagates the values to
//! `bar.StatusBar:SetMinMaxValues` / `:SetValue` via the
//! `SetBarValues` else-branch.
//!
//! ## Source contract
//!
//! `HouseFavorBarMixin:Update` (`Mainline/HouseFavorBar.lua:13-24`):
//!
//! ```lua
//! function HouseFavorBarMixin:Update()
//!     local current, minBar, maxBar, level = 0, 0, 1, 1;
//!     if self.houseLevelFavor then
//!         current = self.houseLevelFavor.houseFavor;
//!         level = self.houseLevelFavor.houseLevel;
//!         minBar = C_Housing.GetHouseLevelFavorForLevel(level);
//!         maxBar = C_Housing.GetHouseLevelFavorForLevel(level + 1);
//!     end
//!     if maxBar ~= 0 then
//!         self:SetBarValues(current, minBar, maxBar, level);
//!     end
//! end
//! ```
//!
//! `HouseFavorBarMixin:OnEvent` (lua:51-66) routes `HOUSE_LEVEL_FAVOR_UPDATED`:
//!
//! ```lua
//! if event == "HOUSE_LEVEL_FAVOR_UPDATED" then
//!     local houseLevelFavor = ...;
//!     if houseLevelFavor.houseGUID == C_Housing.GetTrackedHouseGuid() then
//!         self.houseLevelFavor = houseLevelFavor;
//!         self:Update();
//!     end
//! end
//! ```
//!
//! `HouseFavorBarMixin:OnShow` (lua:68-72) is the registration site:
//! `FrameUtil.RegisterFrameForEvents(self, HouseFavorBarEvents)` registers
//! `PLAYER_ENTERING_WORLD`, `HOUSE_LEVEL_FAVOR_UPDATED`, `CVAR_UPDATE`. Unlike
//! the Honor / XP bars (which register in `OnLoad`), the HouseFavor bar leaves
//! event registration to OnShow because the bar is opt-in based on
//! `C_Housing.GetTrackedHouseGuid()` returning a non-nil GUID
//! (`Mainline/StatusTrackingManagerOverrides.lua:35-37`'s `CanShowBar` arm).
//!
//! `StatusTrackingBarContainer_OnLoad`
//! (`Shared/StatusTrackingManager.lua:170-179`) calls `bar:Hide()` on every
//! bar after `InitializeBars`. So at harness-settle the HouseFavor bar is
//! hidden and has zero registrations — calling `bar:Show()` from the test is
//! what triggers OnShow → RegisterFrameForEvents.
//!
//! `SetBarValues` (`Shared/StatusTrackingBar.lua:32-39`) routes through the
//! else-branch: `supportsAnimation=true` is on the inner
//! `<StatusBar parentKey="StatusBar">` of `StatusTrackingBarTemplate.xml:11`,
//! NOT on the outer bar — so `self.supportsAnimation` (outer) is nil, and
//! the call resolves to a direct `self.StatusBar:SetMinMaxValues(minBar, maxBar)`
//! + `self.StatusBar:SetValue(currentValue)`. The `level` argument is computed
//! but discarded by this dispatch path. Same reasoning as
//! `behavior_honor_bar_update.rs` and `behavior_reputation_bar_update.rs`.
//!
//! ## Bar registration
//!
//! `Mainline/StatusTrackingManagerOverrides.lua:65` adds the HouseFavor bar
//! via `AddBar(StatusTrackingBarInfo.BarsEnum.HouseFavor, "HouseFavorBarTemplate")`,
//! storing it at `MainStatusTrackingBarContainer.bars[BarsEnum.HouseFavor]`
//! (BarsEnum.HouseFavor=6 per `Shared/StatusTrackingManager.lua:5-13`). The
//! `HouseFavorBarTemplate` (`HouseFavorBar.xml:3-12`) inherits
//! `StatusTrackingBarTemplate` and mixes in `HouseFavorBarMixin`.
//!
//! ## C_Housing sources in the simulator
//!
//! - `C_Housing.GetTrackedHouseGuid` (`globals/housing.rs:73-83`) reads
//!   `state.housing.tracked_house_guid` (`Option<String>`); `None` returns nil.
//! - `C_Housing.GetHouseLevelFavorForLevel(level)` (`globals/housing.rs:117-121`)
//!   indexes `state.housing.level_thresholds[level - 1]`; out-of-range returns
//!   `0` — the sentinel `Update` checks at `lua:21` to skip `SetBarValues`.
//! - `state.housing.tracked_house_guid` defaults to `None`
//!   (`HousingState::Default` at `state.rs:533-541`), so without seeding,
//!   `OnShow`'s call to `C_Housing.GetCurrentHouseLevelFavor(GetTrackedHouseGuid())`
//!   passes nil and returns `(0, 0, 0)` — a no-op rather than an error.
//!
//! ## Why fire `HOUSE_LEVEL_FAVOR_UPDATED` instead of calling `Update()` directly
//!
//! Calling `Update()` would silently mask three regressions the test needs
//! to catch:
//! - `OnShow` drops the `FrameUtil.RegisterFrameForEvents` call (lua:69) →
//!   the bar never wires up, so `HOUSE_LEVEL_FAVOR_UPDATED` never reaches
//!   `OnEvent`.
//! - `OnEvent` drops the `HOUSE_LEVEL_FAVOR_UPDATED` arm (lua:52-57) →
//!   `self.houseLevelFavor` never gets set.
//! - The GUID match guard at lua:54 inverts → mismatched GUIDs would be
//!   silently accepted, or matching ones rejected.
//! Firing the event also exercises `lua_api::env::fire_event` →
//! `dispatch_event_now` → frame's `OnEvent` dispatch with a Lua table arg.
//!
//! ## Observations
//!
//! 1. After `with_blizzard_addon_startup_shape(&[Blizzard_ActionBar])`,
//!    `StatusTrackingBarManager.MainStatusTrackingBarContainer.bars[BarsEnum.HouseFavor]`
//!    resolves with `.StatusBar`, BarsEnum.HouseFavor == 6.
//! 2. Cold state: `bar.houseLevelFavor == nil` (no event has fired yet,
//!    OnLoad does not pre-seed it).
//! 3. After seeding `state.housing.tracked_house_guid = Some("test-house-guid")`,
//!    `state.housing.level_thresholds = [..., 1500@idx1, 2500@idx2, ...]`,
//!    calling `bar:Show()` to register events, and firing
//!    `HOUSE_LEVEL_FAVOR_UPDATED` with payload
//!    `{ houseGUID = "test-house-guid", houseFavor = 1800, houseLevel = 2 }`:
//!    - `bar.houseLevelFavor.houseFavor == 1800`,
//!      `bar.houseLevelFavor.houseLevel == 2`.
//!    - `bar.StatusBar:GetMinMaxValues() == (1500, 2500)` (from
//!      `level_thresholds[1]` and `level_thresholds[2]`).
//!    - `bar.StatusBar:GetValue() == 1800` (the seeded `houseFavor`).
//!
//! ## Regression candidates the assertions catch
//!
//! - `HouseFavorBarMixin:OnShow` drops `FrameUtil.RegisterFrameForEvents`
//!   (lua:69) → no listener for `HOUSE_LEVEL_FAVOR_UPDATED`, post-event
//!   `bar.houseLevelFavor` stays nil and StatusBar values stay at the
//!   pre-event defaults.
//! - `OnEvent` arm at lua:52-57 stops routing
//!   `HOUSE_LEVEL_FAVOR_UPDATED` to `Update()` → same symptom.
//! - GUID match guard at lua:54 inverts → matching payload is rejected,
//!   `bar.houseLevelFavor` stays nil.
//! - `Update` regresses to read `level + 1` as `level` (or vice versa)
//!   → `StatusBar:GetMinMaxValues()` collapses to a single threshold.
//! - `C_Housing.GetHouseLevelFavorForLevel` regresses to a hardcoded
//!   value or wrong indexing → StatusBar min/max mismatch.
//! - `SetBarValues` else-branch regresses (no `SetMinMaxValues` /
//!   `SetValue`) → `StatusBar:GetValue` / `GetMinMaxValues` stay at
//!   widget defaults instead of the seeded thresholds.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBar";

const HOUSE_FAVOR_BAR_LUA: &str = "StatusTrackingBarManager.MainStatusTrackingBarContainer\
    .bars[StatusTrackingBarInfo.BarsEnum.HouseFavor]";

const TRACKED_GUID: &str = "test-house-guid";
/// `level_thresholds[1]` (0-indexed) — `Update` reads this as
/// `GetHouseLevelFavorForLevel(level)` when `level == 2`.
const LEVEL_2_THRESHOLD: i64 = 1_500;
/// `level_thresholds[2]` — `Update` reads this as
/// `GetHouseLevelFavorForLevel(level + 1)` when `level == 2`.
const LEVEL_3_THRESHOLD: i64 = 2_500;
const PAYLOAD_HOUSE_FAVOR: i64 = 1_800;
const PAYLOAD_HOUSE_LEVEL: i64 = 2;

#[test]
fn house_favor_bar_round_trips_through_house_level_favor_updated_event() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        assert_cold_globals_resolve(env);
        assert_cold_house_level_favor_nil(env);
        seed_housing_state(env);
        register_house_favor_bar_events(env);
        fire_house_level_favor_updated(env);
        assert_post_event_house_level_favor(env);
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
                and StatusTrackingBarInfo.BarsEnum.HouseFavor
            local bar = container and idx and container.bars and container.bars[idx]
            return manager ~= nil and container ~= nil and idx == 6
                and bar ~= nil and bar.StatusBar ~= nil
            "#,
        )
        .expect("house favor bar global existence probe must run cleanly");
    assert!(
        cold_globals_exist,
        "Chain StatusTrackingBarManager.MainStatusTrackingBarContainer.\
         bars[BarsEnum.HouseFavor].StatusBar must resolve after `{ROOT}` load; \
         nil reading means TOC/InitializeBars/parentKey regression — see \
         header observation 1."
    );
}

fn assert_cold_house_level_favor_nil(env: &WowLuaEnv) {
    let cold_payload_nil: bool = env
        .eval(&format!(
            "return {HOUSE_FAVOR_BAR_LUA}.houseLevelFavor == nil"
        ))
        .expect("cold bar.houseLevelFavor probe must run cleanly");
    assert!(
        cold_payload_nil,
        "Cold `bar.houseLevelFavor` must be nil (no \
         HOUSE_LEVEL_FAVOR_UPDATED has fired yet, OnLoad does not pre-seed); \
         non-nil means an unexpected dispatch reached OnEvent — see header \
         observation 2."
    );
}

fn seed_housing_state(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.housing.tracked_house_guid = Some(TRACKED_GUID.to_string());
    state.housing.level_thresholds = vec![0, LEVEL_2_THRESHOLD, LEVEL_3_THRESHOLD, 4_000];
}

fn register_house_favor_bar_events(env: &WowLuaEnv) {
    let registered: bool = env
        .eval(&format!(
            "{HOUSE_FAVOR_BAR_LUA}:Show(); \
             return {HOUSE_FAVOR_BAR_LUA}:IsEventRegistered(\"HOUSE_LEVEL_FAVOR_UPDATED\")"
        ))
        .expect("bar:Show + IsEventRegistered probe must run cleanly");
    assert!(
        registered,
        "After `bar:Show()`, the HouseFavor bar must have \
         `HOUSE_LEVEL_FAVOR_UPDATED` registered (OnShow → \
         FrameUtil.RegisterFrameForEvents at lua:69); false means OnShow \
         dropped the registration call."
    );
}

fn fire_house_level_favor_updated(env: &WowLuaEnv) {
    let dispatched: bool = env
        .eval(&format!(
            "local payload = {{ \
                houseGUID = \"{TRACKED_GUID}\", \
                houseFavor = {PAYLOAD_HOUSE_FAVOR}, \
                houseLevel = {PAYLOAD_HOUSE_LEVEL}, \
            }}; \
            FireEvent(\"HOUSE_LEVEL_FAVOR_UPDATED\", payload); \
            return true"
        ))
        .expect("HOUSE_LEVEL_FAVOR_UPDATED FireEvent must dispatch cleanly");
    assert!(
        dispatched,
        "FireEvent(\"HOUSE_LEVEL_FAVOR_UPDATED\", payload) must complete \
         cleanly; failure means the dispatch path errored on the table arg."
    );
}

fn assert_post_event_house_level_favor(env: &WowLuaEnv) {
    let payload_house_favor: f64 = env
        .eval(&format!(
            "return {HOUSE_FAVOR_BAR_LUA}.houseLevelFavor.houseFavor"
        ))
        .expect("post-event bar.houseLevelFavor.houseFavor probe must run cleanly");
    assert_eq!(
        payload_house_favor, PAYLOAD_HOUSE_FAVOR as f64,
        "Post-event `bar.houseLevelFavor.houseFavor` must equal \
         {PAYLOAD_HOUSE_FAVOR} (OnEvent stored the payload at lua:55); \
         mismatch means OnEvent's GUID-match guard or assignment regressed \
         — see header observation 3. Got {payload_house_favor}.",
    );

    let payload_house_level: f64 = env
        .eval(&format!(
            "return {HOUSE_FAVOR_BAR_LUA}.houseLevelFavor.houseLevel"
        ))
        .expect("post-event bar.houseLevelFavor.houseLevel probe must run cleanly");
    assert_eq!(
        payload_house_level, PAYLOAD_HOUSE_LEVEL as f64,
        "Post-event `bar.houseLevelFavor.houseLevel` must equal \
         {PAYLOAD_HOUSE_LEVEL} (OnEvent stored the payload at lua:55) — see \
         header observation 3. Got {payload_house_level}.",
    );
}

fn assert_post_event_status_bar_values(env: &WowLuaEnv) {
    let (min, max) = read_status_bar_min_max(env);
    assert_eq!(
        (min, max),
        (LEVEL_2_THRESHOLD as f64, LEVEL_3_THRESHOLD as f64),
        "Post-event `StatusBar:GetMinMaxValues()` must equal \
         ({LEVEL_2_THRESHOLD}, {LEVEL_3_THRESHOLD}) — Update reads \
         GetHouseLevelFavorForLevel(level) and (level + 1), then \
         SetBarValues else-arm calls SetMinMaxValues — see header \
         observation 3. Got ({min}, {max}).",
    );

    let value: f64 = env
        .eval(&format!(
            "return {HOUSE_FAVOR_BAR_LUA}.StatusBar:GetValue()"
        ))
        .expect("post-event StatusBar:GetValue probe must run cleanly");
    assert_eq!(
        value, PAYLOAD_HOUSE_FAVOR as f64,
        "Post-event `StatusBar:GetValue()` must equal \
         {PAYLOAD_HOUSE_FAVOR} (Update → SetBarValues else-arm SetValue \
         chain); reading 0 means the registration → dispatch path dropped \
         the event or SetBarValues regressed — see header observation 3. \
         Got {value}.",
    );
}

fn read_status_bar_min_max(env: &WowLuaEnv) -> (f64, f64) {
    let min: f64 = env
        .eval(&format!(
            "local mn, _ = {HOUSE_FAVOR_BAR_LUA}.StatusBar:GetMinMaxValues() return mn"
        ))
        .expect("StatusBar min probe must run cleanly");
    let max: f64 = env
        .eval(&format!(
            "local _, mx = {HOUSE_FAVOR_BAR_LUA}.StatusBar:GetMinMaxValues() return mx"
        ))
        .expect("StatusBar max probe must run cleanly");
    (min, max)
}
