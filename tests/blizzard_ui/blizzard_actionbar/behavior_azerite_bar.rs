//! Behavior pin: seeding `state.azerite_item` with a non-bank-tab
//! `ItemLocation`, calling `bar:Show()` to drive the lazy `OnShow`
//! `FrameUtil.RegisterFrameForEvents` chain, and firing
//! `AZERITE_ITEM_EXPERIENCE_CHANGED` makes `AzeriteBarMixin:OnEvent` →
//! `:Update` re-read `C_AzeriteItem.FindActiveAzeriteItem` →
//! `GetAzeriteItemXPInfo` → `IsUnlimitedLevelingUnlocked` →
//! `GetPowerLevel`, populate `bar.level` / `bar.xpToNextLevel`, and
//! propagate xp / totalLevelXp to `bar.StatusBar` via the
//! `SetBarValues` else-branch.
//!
//! ## Source contract (`Mainline/AzeriteBar.lua`)
//!
//! `AzeriteBarMixin:Update` (lua:27-41):
//!
//! ```lua
//! function AzeriteBarMixin:Update()
//!     local azeriteItemLocation = C_AzeriteItem.FindActiveAzeriteItem();
//!     local xp, totalLevelXp;
//!     if not azeriteItemLocation
//!         or AzeriteUtil.IsAzeriteItemLocationBankTab(azeriteItemLocation) then
//!         xp, totalLevelXp = 0, 1;
//!         self.level = -1;
//!     else
//!         xp, totalLevelXp = C_AzeriteItem.GetAzeriteItemXPInfo(azeriteItemLocation);
//!         self.level = self:GetLevel();
//!     end
//!     self.xpToNextLevel = totalLevelXp - xp;
//!     self:SetBarValues(xp, 0, totalLevelXp, self.level);
//!     self:UpdatePointsTooltip();
//! end
//! ```
//!
//! `AzeriteBarMixin:GetLevel` (lua:14-25) re-resolves the level: nil when
//! no item, `GetUnlimitedPowerLevel` when `IsUnlimitedLevelingUnlocked()`
//! is true, else `GetPowerLevel`. The test seeds
//! `unlimited_unlocked = false` to pin the `GetPowerLevel` arm.
//!
//! `AzeriteBarMixin:OnEvent` (lua:63-79) gates dispatch on
//! `self:IsVisible()` — like Artifact and HouseFavor, but unlike Honor
//! and XP. The `AZERITE_ITEM_EXPERIENCE_CHANGED` arm at lua:65-66 routes
//! to `Update`. The bar starts hidden via
//! `StatusTrackingBarContainer_OnLoad`'s `bar:Hide()` loop
//! (`Shared/StatusTrackingManager.lua`), so the test must `bar:Show()`
//! before firing.
//!
//! `AzeriteBarMixin:OnShow` (lua:81-84) is the registration site:
//! `FrameUtil.RegisterFrameForEvents(self, AZERITE_XP_BAR_EVENTS)`
//! registers `PLAYER_ENTERING_WORLD`, `AZERITE_ITEM_EXPERIENCE_CHANGED`,
//! `CVAR_UPDATE`, `BAG_UPDATE`. Unlike Honor / XP / Artifact (which
//! register in `OnLoad`), Azerite leaves event registration to OnShow
//! because the bar is opt-in based on `CanShowBar` checking
//! `FindActiveAzeriteItem` /
//! `IsAzeriteItemAtMaxLevel` / `IsAzeriteItemEnabled`
//! (`StatusTrackingManagerOverrides.lua:32-34`).
//!
//! ## SetBarValues else-branch
//!
//! `StatusTrackingBarMixin:SetBarValues` (`Shared/StatusTrackingBar.lua:32-39`)
//! takes the else-branch: `supportsAnimation=true` is on the inner
//! `<StatusBar parentKey="StatusBar">` (`StatusTrackingBarTemplate.xml:11`),
//! NOT the outer Frame. The `AzeriteBarTemplate` (`AzeriteBar.xml:3-16`)
//! declares only `fadeOutEntireBarAtMaxLevel` at outer-Frame scope, so
//! `self.supportsAnimation` (outer) is nil and the call resolves to a
//! direct `self.StatusBar:SetMinMaxValues(0, totalLevelXp)` +
//! `self.StatusBar:SetValue(xp)`. The `level` argument is computed but
//! discarded by this dispatch path.
//!
//! ## Bar registration
//!
//! `Mainline/StatusTrackingManagerOverrides.lua:64` adds the bar via
//! `AddBar(StatusTrackingBarInfo.BarsEnum.Azerite, "AzeriteBarTemplate")`,
//! storing it at `MainStatusTrackingBarContainer.bars[BarsEnum.Azerite]`
//! (BarsEnum.Azerite=5 per `Shared/StatusTrackingManager.lua:5-13`).
//!
//! ## ItemLocation shape used to dodge the bank-tab branch
//!
//! `AzeriteUtil.IsAzeriteItemLocationBankTab` (`AzeriteUtil.lua:135-137`)
//! short-circuits to false unless `bagID` is non-nil AND
//! `>= NUM_TOTAL_EQUIPPED_BAG_SLOTS`. Seeding
//! `ItemLocationData { bag_id: None, slot_index: None,
//! equipment_slot_index: Some(2) }` (the Neck slot in WoW) makes the
//! short-circuit return false, so `Update` takes the equipped-item arm
//! at lua:33-36 instead of the cold-fallback arm.
//!
//! ## Why fire `AZERITE_ITEM_EXPERIENCE_CHANGED` instead of calling `Update`
//!
//! Calling `Update` directly would silently mask three regressions: (1)
//! `OnShow` dropping `FrameUtil.RegisterFrameForEvents` (lua:82) so the
//! bar never wires up; (2) `OnEvent` dropping the
//! `AZERITE_ITEM_EXPERIENCE_CHANGED` arm (lua:65-66) so no dispatch
//! reaches `Update`; (3) `OnEvent`'s `if self:IsVisible()` guard at
//! lua:64 inverting so events leak to a hidden bar or get blocked when
//! visible — the test shows the bar before firing, so an inverted guard
//! would silently drop dispatch.
//!
//! ## Observations
//!
//! 1. After `with_blizzard_addon_startup_shape(&[Blizzard_ActionBar])`,
//!    `StatusTrackingBarManager.MainStatusTrackingBarContainer.bars[BarsEnum.Azerite]`
//!    resolves with `.StatusBar`, BarsEnum.Azerite == 5.
//! 2. Cold state: `bar.level == nil`, `bar.xpToNextLevel == nil` —
//!    `OnShow` never fired (bar starts hidden), so registration never
//!    ran, so OnEvent's `PLAYER_ENTERING_WORLD` arm never ran Update.
//! 3. After seeding `azerite_item`, `bar:Show()`, and firing
//!    `AZERITE_ITEM_EXPERIENCE_CHANGED`:
//!    - `bar.level == 25` (`GetPowerLevel` arm — `unlimited_unlocked = false`).
//!    - `bar.xpToNextLevel == 250` (max_xp 1000 - current_xp 750).
//!    - `bar.StatusBar:GetMinMaxValues() == (0, 1000)`.
//!    - `bar.StatusBar:GetValue() == 750`.
//!
//! ## Regression candidates the assertions catch
//!
//! - `OnShow` drops `FrameUtil.RegisterFrameForEvents` (lua:82) → the
//!   bar never registers, the post-event read keeps `bar.level == nil`.
//! - `OnEvent` drops the `AZERITE_ITEM_EXPERIENCE_CHANGED` arm (lua:65-66)
//!   → same symptom.
//! - `OnEvent`'s `if self:IsVisible()` guard at lua:64 inverts → the
//!   test shows the bar before firing, so the inverted guard would
//!   block dispatch; `bar.level` stays nil.
//! - `GetLevel` regresses to `GetUnlimitedPowerLevel` (forgets the
//!   `IsUnlimitedLevelingUnlocked` gate) → `bar.level` reads
//!   `unlimited_power_level` (29) instead of `power_level` (25).
//! - `Update` regresses the bank-tab guard → `bar.level == -1` and
//!   `xpToNextLevel == 1` instead of the seeded values.
//! - `SetBarValues` else-branch regresses → `StatusBar:GetValue` /
//!   `GetMinMaxValues` stay at widget defaults instead of (750, (0, 1000)).

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::{AzeriteItemState, ItemLocationData, WowLuaEnv};

const ROOT: &str = "Blizzard_ActionBar";

const AZERITE_BAR_LUA: &str = "StatusTrackingBarManager.MainStatusTrackingBarContainer\
    .bars[StatusTrackingBarInfo.BarsEnum.Azerite]";

const NECK_EQUIPMENT_SLOT: i32 = 2;
const SEEDED_CURRENT_XP: i64 = 750;
const SEEDED_MAX_XP: i64 = 1_000;
const SEEDED_POWER_LEVEL: i32 = 25;
const SEEDED_UNLIMITED_LEVEL: i32 = 29;
const EXPECTED_XP_TO_NEXT_LEVEL: i64 = SEEDED_MAX_XP - SEEDED_CURRENT_XP;

#[test]
fn azerite_bar_round_trips_through_azerite_item_experience_changed_event() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        assert_cold_globals_resolve(env);
        assert_cold_azerite_bar_unset(env);
        seed_azerite_item(env);
        show_azerite_bar(env);
        assert_event_registered_after_show(env);
        env.fire_event("AZERITE_ITEM_EXPERIENCE_CHANGED")
            .expect("AZERITE_ITEM_EXPERIENCE_CHANGED fire must dispatch cleanly");
        assert_post_event_azerite_bar_fields(env);
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
                and StatusTrackingBarInfo.BarsEnum.Azerite
            local bar = container and idx and container.bars and container.bars[idx]
            return manager ~= nil and container ~= nil and idx == 5
                and bar ~= nil and bar.StatusBar ~= nil
            "#,
        )
        .expect("azerite bar global existence probe must run cleanly");
    assert!(
        cold_globals_exist,
        "Chain StatusTrackingBarManager.MainStatusTrackingBarContainer.\
         bars[BarsEnum.Azerite].StatusBar must resolve after `{ROOT}` \
         load; nil reading means TOC/InitializeBars/parentKey regression \
         — see header observation 1."
    );
}

fn assert_cold_azerite_bar_unset(env: &WowLuaEnv) {
    let cold_level_nil: bool = env
        .eval(&format!("return {AZERITE_BAR_LUA}.level == nil"))
        .expect("cold bar.level probe must run cleanly");
    assert!(
        cold_level_nil,
        "Cold `bar.level` must be nil — Update never ran because OnShow \
         registers events lazily and the bar starts hidden; non-nil \
         means an unexpected dispatch reached OnEvent — see header \
         observation 2."
    );

    let cold_xp_nil: bool = env
        .eval(&format!("return {AZERITE_BAR_LUA}.xpToNextLevel == nil"))
        .expect("cold bar.xpToNextLevel probe must run cleanly");
    assert!(
        cold_xp_nil,
        "Cold `bar.xpToNextLevel` must be nil (Update never ran) — see \
         header observation 2."
    );
}

fn seed_azerite_item(env: &WowLuaEnv) {
    env.state().borrow_mut().azerite_item = Some(AzeriteItemState {
        item_location: ItemLocationData {
            bag_id: None,
            slot_index: None,
            equipment_slot_index: Some(NECK_EQUIPMENT_SLOT),
        },
        current_xp: SEEDED_CURRENT_XP,
        max_xp: SEEDED_MAX_XP,
        power_level: SEEDED_POWER_LEVEL,
        unlimited_power_level: SEEDED_UNLIMITED_LEVEL,
        unlimited_unlocked: false,
        at_max_level: false,
        enabled: true,
    });
}

fn show_azerite_bar(env: &WowLuaEnv) {
    let visible: bool = env
        .eval(&format!(
            "{AZERITE_BAR_LUA}:Show(); return {AZERITE_BAR_LUA}:IsVisible()"
        ))
        .expect("bar:Show + IsVisible probe must run cleanly");
    assert!(
        visible,
        "After `bar:Show()`, the Azerite bar must report \
         `IsVisible() == true` so OnEvent's `if self:IsVisible()` guard \
         (lua:64) lets AZERITE_ITEM_EXPERIENCE_CHANGED through; false \
         means parent visibility chain regressed."
    );
}

fn assert_event_registered_after_show(env: &WowLuaEnv) {
    let registered: bool = env
        .eval(&format!(
            "return {AZERITE_BAR_LUA}:IsEventRegistered(\"AZERITE_ITEM_EXPERIENCE_CHANGED\")"
        ))
        .expect("IsEventRegistered probe must run cleanly");
    assert!(
        registered,
        "After `bar:Show()`, the Azerite bar must be registered for \
         AZERITE_ITEM_EXPERIENCE_CHANGED via OnShow → \
         FrameUtil.RegisterFrameForEvents (lua:82); false means OnShow \
         dropped the registration call."
    );
}

fn assert_post_event_azerite_bar_fields(env: &WowLuaEnv) {
    let level = read_bar_number_field(env, "level");
    assert_eq!(
        level, SEEDED_POWER_LEVEL as f64,
        "Post-event `bar.level` must equal {SEEDED_POWER_LEVEL} \
         (`GetLevel` → `IsUnlimitedLevelingUnlocked() == false` → \
         `GetPowerLevel`); reading {SEEDED_UNLIMITED_LEVEL} means \
         `GetLevel` skipped the unlimited gate (lua:20-22). Got {level}.",
    );

    let xp_to_next = read_bar_number_field(env, "xpToNextLevel");
    assert_eq!(
        xp_to_next, EXPECTED_XP_TO_NEXT_LEVEL as f64,
        "Post-event `bar.xpToNextLevel` must equal \
         {EXPECTED_XP_TO_NEXT_LEVEL} (totalLevelXp {SEEDED_MAX_XP} - xp \
         {SEEDED_CURRENT_XP}); reading 1 means the bank-tab cold-fallback \
         arm (lua:30-32) ran instead of the equipped-item arm. Got \
         {xp_to_next}.",
    );
}

fn assert_post_event_status_bar_values(env: &WowLuaEnv) {
    let (min, max) = read_status_bar_min_max(env);
    assert_eq!(
        (min, max),
        (0.0, SEEDED_MAX_XP as f64),
        "Post-event `StatusBar:GetMinMaxValues()` must equal \
         (0, {SEEDED_MAX_XP}) — SetBarValues else-arm SetMinMaxValues \
         (StatusTrackingBar.lua:36); other reading means the else-arm \
         regressed or the cold-fallback arm ran. Got ({min}, {max}).",
    );

    let value: f64 = env
        .eval(&format!("return {AZERITE_BAR_LUA}.StatusBar:GetValue()"))
        .expect("post-event StatusBar:GetValue probe must run cleanly");
    assert_eq!(
        value, SEEDED_CURRENT_XP as f64,
        "Post-event `StatusBar:GetValue()` must equal \
         {SEEDED_CURRENT_XP} (Update → SetBarValues else-arm SetValue); \
         reading 0 means the registration → dispatch path dropped the \
         event or the cold-fallback arm ran. Got {value}.",
    );
}

fn read_bar_number_field(env: &WowLuaEnv, field: &str) -> f64 {
    env.eval(&format!("return {AZERITE_BAR_LUA}.{field}"))
        .unwrap_or_else(|_| panic!("bar.{field} probe must run cleanly"))
}

fn read_status_bar_min_max(env: &WowLuaEnv) -> (f64, f64) {
    let min: f64 = env
        .eval(&format!(
            "local mn, _ = {AZERITE_BAR_LUA}.StatusBar:GetMinMaxValues() return mn"
        ))
        .expect("StatusBar min probe must run cleanly");
    let max: f64 = env
        .eval(&format!(
            "local _, mx = {AZERITE_BAR_LUA}.StatusBar:GetMinMaxValues() return mx"
        ))
        .expect("StatusBar max probe must run cleanly");
    (min, max)
}
