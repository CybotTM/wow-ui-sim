//! Behavior pin: seeding `state.equipped_artifact` and
//! `state.artifact_point_costs`, calling `bar:Show()` to satisfy
//! `ArtifactBarMixin:OnEvent`'s `if self:IsVisible()` gate, and firing
//! `ARTIFACT_XP_UPDATE` makes `Update` re-read
//! `C_ArtifactUI.GetEquippedArtifactItemID` →
//! `Item:CreateFromItemID` → `ContinueOnItemLoad` (synchronous via the
//! `ItemEventListener` accessor shim) → `GetEquippedArtifactInfo` →
//! `ArtifactBarGetNumArtifactTraitsPurchasableFromXP` → `SetBarValues`,
//! propagating the XP / point counts to `bar.{xp, totalXP, xpForNextPoint,
//! numPointsAvailableToSpend}` and the underlying StatusBar's
//! `GetValue()` / `GetMinMaxValues()`.
//!
//! ## Source contract (`Mainline/ArtifactBar.lua`)
//!
//! `ArtifactBarMixin:Update` (lua:11-34) is the only Update path; it does
//! NOT run synchronously — instead it queues a callback through
//! `ContinueOnItemLoad`:
//!
//! ```lua
//! function ArtifactBarMixin:Update()
//!     local artifactItemID = C_ArtifactUI.GetEquippedArtifactItemID();
//!     if artifactItemID then
//!         local item = Item:CreateFromItemID(artifactItemID);
//!         item:ContinueOnItemLoad(function()
//!             local artifactItemID, _, _, _, artifactTotalXP,
//!                 artifactPointsSpent, _, _, _, _, _, _, artifactTier =
//!                 C_ArtifactUI.GetEquippedArtifactInfo();
//!             local numPointsAvailableToSpend, xp, xpForNextPoint =
//!                 ArtifactBarGetNumArtifactTraitsPurchasableFromXP(
//!                     artifactPointsSpent, artifactTotalXP, artifactTier);
//!             self:SetBarValues(xp, 0, xpForNextPoint,
//!                 numPointsAvailableToSpend + artifactPointsSpent);
//!             self.StatusBar.artifactItemID = artifactItemID;
//!             self.xp = xp;
//!             self.totalXP = artifactTotalXP;
//!             self.xpForNextPoint = xpForNextPoint;
//!             self.numPointsAvailableToSpend = numPointsAvailableToSpend;
//!             self:Show();
//!             self.Tick:SetShown(numPointsAvailableToSpend > 0);
//!             ...
//!         end);
//!     end
//! end
//! ```
//!
//! `ArtifactBarMixin:OnEvent` (lua:64-75) gates dispatch on
//! `self:IsVisible()` — unlike Honor/XP/Reputation/HouseFavor which run
//! their handlers regardless of visibility. The bar starts hidden via
//! `StatusTrackingBarContainer_OnLoad`'s `bar:Hide()` loop
//! (`Shared/StatusTrackingManager.lua:170-179`), so the test must
//! `bar:Show()` before firing the event.
//!
//! `ArtifactBarMixin:OnLoad` (lua:51-62) registers `ARTIFACT_XP_UPDATE`,
//! `UNIT_INVENTORY_CHANGED`, `PLAYER_ENTERING_WORLD`,
//! `UPDATE_EXTRA_ACTIONBAR`, `CVAR_UPDATE` directly on the frame —
//! registration happens at OnLoad, not OnShow.
//!
//! ## Async callback resolution
//!
//! `Item:CreateFromItemID` returns a stub `ItemMixin`; `ContinueOnItemLoad`
//! routes through `ItemEventListener:AddCallback` (`Blizzard_ObjectAPI/
//! Mainline/AsyncCallbackSystem.lua:47-56`), which calls
//! `self.api.accessor(id) = C_Item.RequestLoadItemDataByID(itemID)` when
//! the callback list goes from 0 to 1. `runtime_surface_bootstrap.lua:10255-10262`
//! shims `C_Item.RequestLoadItemDataByID(itemID)` to call
//! `ItemEventListener:FireCallbacks(itemID)` immediately, which fires the
//! pending callback synchronously and clears the list. So inside the test,
//! `Update`'s `ContinueOnItemLoad` callback runs before `Update` returns.
//!
//! ## SetBarValues else-branch
//!
//! `StatusTrackingBarMixin:SetBarValues`
//! (`Shared/StatusTrackingBar.lua:32-39`) takes the else-branch:
//! `supportsAnimation=true` is on the inner
//! `<StatusBar parentKey="StatusBar">` (`StatusTrackingBarTemplate.xml:11`),
//! NOT on the outer Frame. The `ArtifactStatusBarTemplate`
//! (`ArtifactBar.xml:3-26`) declares only `fadeOutEntireBarAtMaxLevel`
//! at outer-Frame scope, so `self.supportsAnimation` (outer) is nil and
//! the call resolves to a direct
//! `self.StatusBar:SetMinMaxValues(0, xpForNextPoint)` +
//! `self.StatusBar:SetValue(xp)`.
//!
//! ## Bar registration
//!
//! `Mainline/StatusTrackingManagerOverrides.lua:62` adds the bar via
//! `AddBar(StatusTrackingBarInfo.BarsEnum.Artifact, "ArtifactStatusBarTemplate")`,
//! storing it at `MainStatusTrackingBarContainer.bars[BarsEnum.Artifact]`
//! (BarsEnum.Artifact=3 per `Shared/StatusTrackingManager.lua:5-13`). The
//! `ArtifactStatusBarTemplate` inherits `StatusTrackingBarTemplate` and
//! mixes in `ArtifactBarMixin`.
//!
//! ## Why exercise the cost loop with three iterations
//!
//! `ArtifactBarGetNumArtifactTraitsPurchasableFromXP` (lua:96-108) is a
//! tight while-loop that calls `C_ArtifactUI.GetCostForPointAtRank` for
//! the next-rank cost on each iteration. With a single-rank seed the loop
//! never exercises the cost-replacement branch. The test seeds
//! `total_xp = 350`, `points_spent = 2`, `tier = 1` and a four-entry cost
//! map: `(2,1)=100`, `(3,1)=100`, `(4,1)=100`, `(5,1)=200`. The loop runs:
//! 350 ≥ 100 → numPoints=1, xp=250; 250 ≥ 100 → numPoints=2, xp=150;
//! 150 ≥ 100 → numPoints=3, xp=50; 50 < 200 → stop. Final return:
//! `(numPoints=3, xp=50, xpForNextPoint=200)`. This catches a regression
//! in the loop's `xpForNextPoint > 0` continue condition (would
//! over-iterate past the `(5,1)` entry into a missing
//! `(6,1)` → 0 entry), and pins the cost-replacement read at lua:105.
//!
//! ## Seeded artifact item / cost map
//!
//! - `state.equipped_artifact = Some(ArtifactInfo { item_id: 211_993,
//!   total_xp: 350, points_spent: 2, tier: 1, ... })`. `item_id = 211_993`
//!   ("Entombed Seraph's Casque") is one of the equipment IDs already
//!   present in `data/items.rs`. The actual identity does not matter —
//!   the test pins XP-bar behavior, not artifact metadata — but the ID
//!   MUST be in the items DB so that `Item:CreateFromItemID(itemID)` →
//!   `ContinueOnItemLoad` clears `IsItemEmpty()` (which calls
//!   `C_Item.DoesItemExistByID(itemID)`). Using Ashbringer's retail ID
//!   `128_910` would make `DoesItemExistByID` return false, tripping the
//!   `NonEmptyItem:ContinueOnItemLoad invalid itemID` guard at
//!   `Item.lua:317-318` and aborting `Update` before the bar values
//!   are written.
//! - `state.artifact_point_costs = HashMap::from([((2, 1), 100), ...])`.
//!   Tier `1` because the test does not exercise the panel-side
//!   tier-grey-out paths.
//!
//! ## Why fire `ARTIFACT_XP_UPDATE` instead of calling `Update` directly
//!
//! Calling `Update` directly would silently mask three regressions:
//! - `OnLoad` drops `RegisterEvent("ARTIFACT_XP_UPDATE")` (lua:54) →
//!   the bar never wires up.
//! - `OnEvent` drops the `ARTIFACT_XP_UPDATE` arm (lua:66) → no dispatch.
//! - `OnEvent` inverts the `if self:IsVisible()` guard (lua:65) → events
//!   leak to a hidden bar or get blocked when visible.
//! Firing the event also exercises `ContinueOnItemLoad` end-to-end
//! through the `ItemEventListener` synchronous-accessor shim.
//!
//! ## Observations
//!
//! 1. After `with_blizzard_addon_startup_shape(&[Blizzard_ActionBar])`,
//!    `StatusTrackingBarManager.MainStatusTrackingBarContainer.bars[BarsEnum.Artifact]`
//!    resolves with `.StatusBar` and `.Tick`, BarsEnum.Artifact == 3.
//! 2. Cold state: `bar.xp == nil` and `bar.numPointsAvailableToSpend == nil`
//!    (Update never ran — OnLoad does not pre-populate).
//! 3. After seeding `equipped_artifact` and `artifact_point_costs`,
//!    `bar:Show()`, and firing `ARTIFACT_XP_UPDATE`:
//!    - `bar.xp == 50`, `bar.totalXP == 350`, `bar.xpForNextPoint == 200`,
//!      `bar.numPointsAvailableToSpend == 3`.
//!    - `bar.StatusBar:GetMinMaxValues() == (0, 200)`,
//!      `bar.StatusBar:GetValue() == 50`.
//!    - `bar.Tick:IsShown() == true` (numPointsAvailableToSpend > 0).
//!    - `bar.StatusBar.artifactItemID == 211_993`.
//!
//! ## Regression candidates the assertions catch
//!
//! - `OnLoad` drops `RegisterEvent("ARTIFACT_XP_UPDATE")` (lua:54) →
//!   post-event `bar.xp` stays nil.
//! - `OnEvent` drops the `ARTIFACT_XP_UPDATE` arm (lua:66) → same symptom.
//! - `OnEvent`'s `if self:IsVisible()` guard inverts (lua:65) → the test
//!   shows the bar before firing, so the inverted guard would block
//!   dispatch; `bar.xp` stays nil.
//! - `ContinueOnItemLoad` regresses to defer the callback indefinitely
//!   → `bar.xp` stays nil because the callback body never runs.
//! - `ArtifactBarGetNumArtifactTraitsPurchasableFromXP`'s loop terminates
//!   on the wrong condition → wrong `numPointsAvailableToSpend` /
//!   `xpForNextPoint` values, asserted explicitly.
//! - `SetBarValues` else-branch regresses (no `SetMinMaxValues` /
//!   `SetValue`) → `StatusBar:GetValue` / `GetMinMaxValues` stay at
//!   widget defaults instead of the loop's outputs.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use std::collections::HashMap;
use wow_ui_sim::lua_api::{ArtifactInfo, WowLuaEnv};

const ROOT: &str = "Blizzard_ActionBar";

const ARTIFACT_BAR_LUA: &str = "StatusTrackingBarManager.MainStatusTrackingBarContainer\
    .bars[StatusTrackingBarInfo.BarsEnum.Artifact]";

const ITEM_ID: i32 = 211_993;
const TIER: i32 = 1;
const POINTS_SPENT: i32 = 2;
const TOTAL_XP: i64 = 350;
/// Cost-per-rank table seeded into `state.artifact_point_costs`. The
/// loop runs three iterations and exits when `total_xp - 300 = 50` is
/// less than the next cost `200`.
const POINT_COSTS: [((i32, i32), i64); 4] =
    [((2, 1), 100), ((3, 1), 100), ((4, 1), 100), ((5, 1), 200)];
const EXPECTED_NUM_POINTS: i32 = 3;
const EXPECTED_REMAINING_XP: i64 = 50;
const EXPECTED_NEXT_COST: i64 = 200;

#[test]
fn artifact_bar_round_trips_through_artifact_xp_update_event() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        assert_cold_globals_resolve(env);
        assert_cold_artifact_bar_unset(env);
        seed_equipped_artifact(env);
        seed_artifact_point_costs(env);
        show_artifact_bar(env);
        env.fire_event("ARTIFACT_XP_UPDATE")
            .expect("ARTIFACT_XP_UPDATE fire must dispatch cleanly");
        assert_post_event_cost_loop_fields(env);
        assert_post_event_status_bar_item_id(env);
        assert_post_event_status_bar_values(env);
        assert_post_event_tick_shown(env);
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
                and StatusTrackingBarInfo.BarsEnum.Artifact
            local bar = container and idx and container.bars and container.bars[idx]
            return manager ~= nil and container ~= nil and idx == 3
                and bar ~= nil and bar.StatusBar ~= nil and bar.Tick ~= nil
            "#,
        )
        .expect("artifact bar global existence probe must run cleanly");
    assert!(
        cold_globals_exist,
        "Chain StatusTrackingBarManager.MainStatusTrackingBarContainer.\
         bars[BarsEnum.Artifact].{{StatusBar,Tick}} must resolve after \
         `{ROOT}` load; nil reading means TOC/InitializeBars/parentKey \
         regression — see header observation 1."
    );
}

fn assert_cold_artifact_bar_unset(env: &WowLuaEnv) {
    let cold_xp_nil: bool = env
        .eval(&format!("return {ARTIFACT_BAR_LUA}.xp == nil"))
        .expect("cold bar.xp probe must run cleanly");
    assert!(
        cold_xp_nil,
        "Cold `bar.xp` must be nil (Update never ran — OnLoad does not \
         pre-populate); non-nil means an unexpected dispatch reached \
         OnEvent — see header observation 2."
    );

    let cold_points_nil: bool = env
        .eval(&format!(
            "return {ARTIFACT_BAR_LUA}.numPointsAvailableToSpend == nil"
        ))
        .expect("cold bar.numPointsAvailableToSpend probe must run cleanly");
    assert!(
        cold_points_nil,
        "Cold `bar.numPointsAvailableToSpend` must be nil (Update never \
         ran) — see header observation 2."
    );
}

fn seed_equipped_artifact(env: &WowLuaEnv) {
    env.state().borrow_mut().equipped_artifact = Some(ArtifactInfo {
        item_id: ITEM_ID,
        alt_item_id: 0,
        name: "Test Artifact".to_string(),
        icon: "Interface/Icons/inv_sword_2h_artifactashbringer_d_01".to_string(),
        total_xp: TOTAL_XP,
        points_spent: POINTS_SPENT,
        quality: 6,
        artifact_appearance_id: 0,
        appearance_mod_id: 0,
        item_appearance_id: 0,
        alt_item_appearance_id: 0,
        alt_on_top: false,
        tier: TIER,
        maxed: false,
        disabled: false,
        category: 1,
    });
}

fn seed_artifact_point_costs(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.artifact_point_costs = HashMap::from(POINT_COSTS);
}

fn show_artifact_bar(env: &WowLuaEnv) {
    let visible: bool = env
        .eval(&format!(
            "{ARTIFACT_BAR_LUA}:Show(); return {ARTIFACT_BAR_LUA}:IsVisible()"
        ))
        .expect("bar:Show + IsVisible probe must run cleanly");
    assert!(
        visible,
        "After `bar:Show()`, the Artifact bar must report \
         `IsVisible() == true` so OnEvent's `if self:IsVisible()` guard \
         (lua:65) lets ARTIFACT_XP_UPDATE through; false means parent \
         visibility chain regressed."
    );
}

fn assert_post_event_cost_loop_fields(env: &WowLuaEnv) {
    let xp = read_bar_number_field(env, "xp");
    assert_eq!(
        xp, EXPECTED_REMAINING_XP as f64,
        "Post-event `bar.xp` must equal {EXPECTED_REMAINING_XP} (cost \
         loop: 350 - 3*100 = 50); mismatch means \
         ArtifactBarGetNumArtifactTraitsPurchasableFromXP regressed — \
         see header observation 3. Got {xp}.",
    );

    let total_xp = read_bar_number_field(env, "totalXP");
    assert_eq!(
        total_xp, TOTAL_XP as f64,
        "Post-event `bar.totalXP` must equal {TOTAL_XP} \
         (artifactTotalXP from GetEquippedArtifactInfo position 5) — see \
         header observation 3. Got {total_xp}.",
    );

    let next_cost = read_bar_number_field(env, "xpForNextPoint");
    assert_eq!(
        next_cost, EXPECTED_NEXT_COST as f64,
        "Post-event `bar.xpForNextPoint` must equal {EXPECTED_NEXT_COST} \
         (the (5,1) cost the loop terminated on); mismatch means the \
         cost-replacement read at lua:105 regressed — see header \
         observation 3. Got {next_cost}.",
    );

    let num_points = read_bar_number_field(env, "numPointsAvailableToSpend");
    assert_eq!(
        num_points, EXPECTED_NUM_POINTS as f64,
        "Post-event `bar.numPointsAvailableToSpend` must equal \
         {EXPECTED_NUM_POINTS} (3 loop iterations) — see header \
         observation 3. Got {num_points}.",
    );
}

fn assert_post_event_status_bar_item_id(env: &WowLuaEnv) {
    let status_bar_item_id: f64 = env
        .eval(&format!(
            "return {ARTIFACT_BAR_LUA}.StatusBar.artifactItemID"
        ))
        .expect("post-event StatusBar.artifactItemID probe must run cleanly");
    assert_eq!(
        status_bar_item_id, ITEM_ID as f64,
        "Post-event `bar.StatusBar.artifactItemID` must equal {ITEM_ID} \
         (set at lua:21 from GetEquippedArtifactInfo position 1) — see \
         header observation 3. Got {status_bar_item_id}.",
    );
}

fn assert_post_event_status_bar_values(env: &WowLuaEnv) {
    let (min, max) = read_status_bar_min_max(env);
    assert_eq!(
        (min, max),
        (0.0, EXPECTED_NEXT_COST as f64),
        "Post-event `StatusBar:GetMinMaxValues()` must equal \
         (0, {EXPECTED_NEXT_COST}) — SetBarValues else-arm SetMinMaxValues \
         (StatusTrackingBar.lua:36) — see header observation 3. Got \
         ({min}, {max}).",
    );

    let value: f64 = env
        .eval(&format!("return {ARTIFACT_BAR_LUA}.StatusBar:GetValue()"))
        .expect("post-event StatusBar:GetValue probe must run cleanly");
    assert_eq!(
        value, EXPECTED_REMAINING_XP as f64,
        "Post-event `StatusBar:GetValue()` must equal \
         {EXPECTED_REMAINING_XP} (Update → SetBarValues else-arm SetValue) \
         — see header observation 3. Got {value}.",
    );
}

fn assert_post_event_tick_shown(env: &WowLuaEnv) {
    let tick_shown: bool = env
        .eval(&format!("return {ARTIFACT_BAR_LUA}.Tick:IsShown()"))
        .expect("post-event Tick:IsShown probe must run cleanly");
    assert!(
        tick_shown,
        "Post-event `bar.Tick:IsShown()` must be true \
         (numPointsAvailableToSpend = {EXPECTED_NUM_POINTS} > 0 drives \
         self.Tick:SetShown at lua:27); false means the show branch \
         regressed — see header observation 3."
    );
}

fn read_bar_number_field(env: &WowLuaEnv, field: &str) -> f64 {
    env.eval(&format!("return {ARTIFACT_BAR_LUA}.{field}"))
        .unwrap_or_else(|_| panic!("bar.{field} probe must run cleanly"))
}

fn read_status_bar_min_max(env: &WowLuaEnv) -> (f64, f64) {
    let min: f64 = env
        .eval(&format!(
            "local mn, _ = {ARTIFACT_BAR_LUA}.StatusBar:GetMinMaxValues() return mn"
        ))
        .expect("StatusBar min probe must run cleanly");
    let max: f64 = env
        .eval(&format!(
            "local _, mx = {ARTIFACT_BAR_LUA}.StatusBar:GetMinMaxValues() return mx"
        ))
        .expect("StatusBar max probe must run cleanly");
    (min, max)
}
