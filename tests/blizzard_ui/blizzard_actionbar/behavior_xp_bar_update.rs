//! Behavior pin: mutating `state.player.xp` / `state.player.xp_max` /
//! `state.player_xp.exhaustion` and firing `PLAYER_XP_UPDATE` makes
//! `ExpBarMixin:Update` re-read `UnitXP("player")` / `UnitXPMax("player")`,
//! repopulate `bar.currXP` / `bar.maxBar`, and propagate the values to the
//! underlying `StatusBar`'s `GetValue()` / `GetMinMaxValues()`. The
//! co-located `ExhaustionTickMixin:UpdateTickPosition` picks up the new
//! `GetXPExhaustion()` reading on the same event and shows the rest-XP
//! pip when the computed `widthRatio` is inside the [0.01, 0.99]
//! `hideAtBarEdge` window.
//!
//! ## Source contract (`Interface/BlizzardUI/Blizzard_ActionBar/`)
//!
//! 1. `StatusTrackingBarManager` is the
//!    `<Frame name="StatusTrackingBarManager" parent="UIParent" ... mixin="StatusTrackingManagerMixin">`
//!    declared at `Mainline/StatusTrackingBar.xml:35`. Its child
//!    `<Frame name="MainStatusTrackingBarContainer" parentKey="MainStatusTrackingBarContainer" ...>`
//!    at xml:41 inherits `StatusTrackingBarContainerTemplate` (xml:3) which
//!    runs `StatusTrackingBarContainer_OnLoad` at xml:31. That OnLoad
//!    (`Shared/StatusTrackingManager.lua:170-179`) sets `self.bars = {}`
//!    then calls `self:InitializeBars()` and `bar:Hide()` on every entry.
//!
//! 2. `StatusTrackingBarContainerMixin:InitializeBars` is overridden in
//!    `Mainline/StatusTrackingManagerOverrides.lua:42-66`. The Mainline
//!    impl creates one bar per entry in `StatusTrackingBarInfo.BarsEnum`
//!    (`Shared/StatusTrackingManager.lua:5-13`) — Reputation=1, Honor=2,
//!    Artifact=3, Experience=4, Azerite=5, HouseFavor=6 — via
//!    `CreateFrame("FRAME", nil, self, template)` and stores the
//!    instance at `self.bars[barIndex] = bar` (lua:57). The Experience
//!    entry (lua:63) uses the `ExpStatusBarTemplate` declared at
//!    `Mainline/ExpBar.xml:3-46`, which inherits `StatusTrackingBarTemplate`
//!    (`Mainline/StatusTrackingBarTemplate.xml:3`) and mixes in
//!    `ExpBarMixin`. The bar instance is anonymous (the second arg to
//!    `CreateFrame` is `nil`); the only handle is
//!    `MainStatusTrackingBarContainer.bars[StatusTrackingBarInfo.BarsEnum.Experience]`.
//!
//! 3. `ExpBarMixin:OnLoad` (`Shared/ExpBar.lua:47-55`) registers
//!    `PLAYER_XP_UPDATE` (lua:53), `PLAYER_ENTERING_WORLD` (lua:52), and
//!    `CVAR_UPDATE` (lua:54), then calls `self:Update()` once at lua:50.
//!    The OnLoad path runs as soon as the bar widget is constructed
//!    inside `InitializeBars` (the XML `<OnLoad method="OnLoad"/>` at
//!    `Mainline/ExpBar.xml:40` fires synchronously during `CreateFrame`),
//!    BEFORE the container's `bar:Hide()` loop in
//!    `StatusTrackingBarContainer_OnLoad` lua:175-177. So the cold-state
//!    `bar.currXP` / `bar.maxBar` reflect the seeded `state.player.xp = 0`
//!    and `state.player.xp_max = 180_000` from `PlayerState::seeded`
//!    (`character_world.rs:235`).
//!
//! 4. `ExpBarMixin:OnEvent` (`Shared/ExpBar.lua:57-66`) routes both
//!    `PLAYER_XP_UPDATE` and `PLAYER_ENTERING_WORLD` to `self:Update()`
//!    (lua:63-65). The harness's `settle_headless_startup`
//!    (`src/startup.rs:412-414`) fires `PLAYER_ENTERING_WORLD`, so by the
//!    time the test runs Update has already been called twice — once on
//!    OnLoad, once on PLAYER_ENTERING_WORLD. Both passes read the seeded
//!    state, so the cold-state observation is stable.
//!
//! 5. `ExpBarMixin:Update` (`Shared/ExpBar.lua:27-41`):
//!    ```lua
//!    function ExpBarMixin:Update()
//!        local level;
//!        self.currXP, self.maxBar, level = self:GetLevelData();
//!        if self:IsCapped() then
//!            ...
//!            self:SetBarValues(1, 0, 1, level, self:GetMaxLevel());
//!        else
//!            local minBar = 0;
//!            self:SetBarValues(self.currXP, minBar, self.maxBar, level, self:GetMaxLevel());
//!        end
//!        self:UpdateCurrentText();
//!    end
//!    ```
//!    `GetLevelData` (lua:18-25) returns
//!    `(UnitXP("player"), UnitXPMax("player"), UnitLevel("player"), UnitTrialBankedLevels("player"))`
//!    for the non-capped case. `IsCapped()` (lua:9-16) returns true only
//!    when `GameLimitedMode_IsBankedXPActive()` is true AND
//!    `UnitLevel >= GameLimitedMode_GetLevelLimit()` — the seeded
//!    `PlayerXpState::default` (`character_world.rs:289-308`) sets
//!    `banked_xp_active = false`, so the test exercises the non-capped
//!    branch unconditionally.
//!
//! 6. `StatusTrackingBarMixin:SetBarValues`
//!    (`Shared/StatusTrackingBar.lua:32-39`) routes through
//!    `self.StatusBar:SetAnimatedValues(currentValue, minBar, maxBar, level, maxLevel)`
//!    when `self.supportsAnimation` is true — which the
//!    `StatusTrackingBarTemplate` sets at xml:11. `SetAnimatedValues`
//!    (`Blizzard_FrameXMLBase/GradualAnimatedStatusBar.lua:69-98`) marks
//!    `pendingValue` / `pendingMin` / `pendingMax` / `pendingLevel` and,
//!    crucially for the test, calls `ProcessChangesInstantly` (lua:95-97)
//!    when the StatusBar is not visible. The bar is hidden by default
//!    (`StatusTrackingBarContainer_OnLoad` runs `bar:Hide()` at lua:176)
//!    so the test sees instant value propagation rather than animation.
//!    `ProcessChangesInstantly` (lua:204-234) calls
//!    `SetMinMaxValues(pendingMin, pendingMax)` (lua:217) and
//!    `SetValue(pendingValue)` (lua:230) — those are the GetValue /
//!    GetMinMaxValues readings the test pins.
//!
//! 7. `ExhaustionTick` is the
//!    `<Button parentKey="ExhaustionTick" mixin="ExhaustionTickMixin">`
//!    declared at `Mainline/ExpBar.xml:20-37`. Its OnEvent
//!    (`Shared/ExpBar.lua:172-196`) routes `PLAYER_XP_UPDATE` (registered
//!    at lua:119) to `UpdateTickPosition` (lua:125-159).
//!    `UpdateTickPosition` reads `GetXPExhaustion()` (lua:128) and:
//!    - if `not exhaustionThreshold or exhaustionThreshold <= 0 or
//!      IsPlayerAtEffectiveMaxLevel() or IsXPUserDisabled()` →
//!      `self:Hide()` (lua:135-138).
//!    - else with `hideAtBarEdge=true` (KeyValue at xml:22) →
//!      `self:SetShown(widthRatio >= 0.01 and widthRatio <= 0.99)`
//!      where `widthRatio = max((playerCurrXP + exhaustionThreshold) /
//!      playerMaxXP, 0)`.
//!
//! ## Why the test mutates `state.player.xp` / `state.player_xp.exhaustion`
//! directly rather than calling a Lua mutator
//!
//! There is no Lua-facing setter for player XP totals or rest XP — real
//! WoW receives both fields via server-pushed `PLAYER_XP_UPDATE` /
//! `UPDATE_EXHAUSTION` deltas. The simulator's combat/quest model would
//! synthesise these deltas, but it has no Lua mutator for "you just got
//! 12345 XP". Direct state mutation is the canonical write seam, mirroring
//! the pattern used by `behavior_extra_action_bar.rs:291-295` for
//! `state.extra_action_button.spell_id` and `behavior_pet_bar_update.rs:225-237`
//! for `state.pet_actions[0]`.
//!
//! ## Why the test fires `PLAYER_XP_UPDATE` instead of calling
//! `ExpBar:Update()` or `ExhaustionTick:UpdateTickPosition()` directly
//!
//! Direct method calls would prove the methods work in isolation but
//! would not catch a regression where `ExpBarMixin:OnLoad` stops
//! registering `PLAYER_XP_UPDATE` (lua:53) or `OnEvent` drops the
//! `PLAYER_XP_UPDATE` arm (lua:63). Likewise for the ExhaustionTick:
//! its OnLoad registers `PLAYER_XP_UPDATE` separately (lua:119), so a
//! regression that breaks ExpBar's registration but leaves
//! ExhaustionTick's registration intact (or vice versa) would be
//! invisible to a direct-call test. Firing the event proves both
//! registration → dispatch → handler chains are still wired. Same
//! pattern as `behavior_pet_bar_update.rs:240-241` and
//! `behavior_extra_action_bar.rs:297-298`.
//!
//! ## Why the test pins `bar.currXP` AND `StatusBar:GetValue()` /
//! `GetMinMaxValues()`
//!
//! These pin two distinct contracts with different failure modes:
//!
//! - `bar.currXP` / `bar.maxBar` (pinned by `ExpBar.lua:29`) prove the
//!   `Update` body ran and read fresh `UnitXP` / `UnitXPMax`. A
//!   regression that broke `GetLevelData` (e.g., reading
//!   `UnitTrialXP("player")` unconditionally instead of gating on
//!   `IsCapped()`) would surface here as a mismatch on `currXP`.
//! - `StatusBar:GetValue()` / `GetMinMaxValues()` (pinned by
//!   `SetBarValues` → `SetAnimatedValues` → `ProcessChangesInstantly`)
//!   prove the value-propagation path through the animated-status-bar
//!   indirection still works. A regression that dropped the
//!   `not self:IsVisible() then self:ProcessChangesInstantly()` branch
//!   (`GradualAnimatedStatusBar.lua:95-97`) would leave `pendingValue`
//!   stuck without ever calling `SetValue`, so `GetValue()` would still
//!   read the pre-mutation value while `bar.currXP` would track.
//!
//! ## Why the test seeds exhaustion such that `widthRatio` falls in the
//! middle of the visible range
//!
//! With `xp = 12_345`, `exhaustion = 5_000`, and `xp_max = 50_000`,
//! `widthRatio = (12_345 + 5_000) / 50_000 = 0.347`. That lands cleanly
//! between the `hideAtBarEdge` cutoffs at 0.01 and 0.99
//! (`ExpBar.lua:142`), so `ExhaustionTick:SetShown(true)` is the
//! deterministic outcome — a 0.005 or 0.999 reading would flip the
//! observation depending on which side of the cutoff it landed and make
//! the test brittle. The maxXP value `50_000` is also distinct from the
//! seeded `180_000` so the post-mutation reading on `bar.maxBar` cannot
//! be confused with a stale cold-state read.
//!
//! ## Observations
//!
//! 1. **`StatusTrackingBarManager`, `MainStatusTrackingBarContainer`,
//!    `bar = bars[Experience]`, `bar.StatusBar`, and `bar.ExhaustionTick`
//!    all exist after harness settle.** A nil reading means the XML
//!    didn't load (TOC walk regressed), the parentKey attachment failed,
//!    or `InitializeBars` (Mainline/StatusTrackingManagerOverrides.lua:42-66)
//!    stopped storing the bar at the Experience index.
//!
//! 2. **Cold-state `bar.currXP == 0` and `bar.maxBar == 180_000`.**
//!    Pinned by the seeded `PlayerState::seeded` defaults at
//!    `character_world.rs:223-238` (xp=0 from Default, xp_max=180_000
//!    explicit). The cold reading also confirms `Update` ran during
//!    OnLoad / PLAYER_ENTERING_WORLD and successfully wrote
//!    `self.currXP, self.maxBar = self:GetLevelData()` (lua:29).
//!
//! 3. **Cold-state `GetXPExhaustion() == nil` and
//!    `bar.ExhaustionTick:IsShown() == false`.** Pinned by
//!    `PlayerXpState::default` (`character_world.rs:289-308`) setting
//!    `exhaustion = None` → `xp_honor_rest.rs:38-44` pushes nil → the
//!    `not exhaustionThreshold` branch at ExpBar.lua:135 fires →
//!    `self:Hide()`.
//!
//! 4. **After mutating `state.player.xp = 12_345`,
//!    `state.player.xp_max = 50_000`, and `state.player_xp.exhaustion =
//!    Some(5_000)`, then firing `PLAYER_XP_UPDATE`:**
//!    - `bar.currXP == 12_345` and `bar.maxBar == 50_000` (lua:29 reads
//!      `UnitXP` / `UnitXPMax`, which read `state.player.xp` / `xp_max`
//!      via `unit_stats.rs:434-444`).
//!    - `bar.StatusBar:GetValue() == 12_345` and
//!      `(min, max) = bar.StatusBar:GetMinMaxValues() == (0, 50_000)`
//!      (`SetBarValues` → `SetAnimatedValues` → `ProcessChangesInstantly`).
//!    - `GetXPExhaustion() == 5_000` (state-field read at
//!      `xp_honor_rest.rs:38-44`).
//!    - `bar.ExhaustionTick:IsShown() == true` (widthRatio = 0.347 lands
//!      in the [0.01, 0.99] hideAtBarEdge range so SetShown(true) fires).
//!
//! ## Regression candidates the assertions catch
//!
//! - `ExpBarMixin:OnLoad` stops registering `PLAYER_XP_UPDATE` (lua:53):
//!   observation 4's `currXP` / `maxBar` checks fail (no listener picks
//!   up the fire), 1, 2, 3 still pass.
//! - `ExpBarMixin:OnEvent` drops the `PLAYER_XP_UPDATE` arm (lua:63):
//!   same as above.
//! - `GetLevelData` regresses to reading `UnitTrialXP` unconditionally:
//!   observation 4's `currXP` reads 0 (the default `trial_xp`) instead
//!   of 12_345; `maxBar` still reads 50_000 because `nextXP` doesn't go
//!   through `IsCapped`.
//! - `SetAnimatedValues`'s `not IsVisible → ProcessChangesInstantly`
//!   branch regresses: observation 4's `bar.currXP` still tracks (Update
//!   wrote it directly) but `StatusBar:GetValue()` stays at the
//!   cold-state `0` because the pending value never got flushed.
//! - `ExhaustionTickMixin:OnLoad` stops registering `PLAYER_XP_UPDATE`
//!   (lua:119): observation 4's `IsShown()` check fails (the tick stays
//!   on its cold-state Hide).
//! - `xp_honor_rest.rs:38-44` `GetXPExhaustion` regresses to a hardcoded
//!   value: observation 3 fails (cold non-nil) or observation 4's
//!   `IsShown()` fails (post-seed reads stale value, widthRatio
//!   computes wrong).
//! - The `hideAtBarEdge` clamp at `ExpBar.lua:142` regresses to
//!   `widthRatio < 0.01 or widthRatio > 0.99`: observation 4's
//!   `IsShown()` flips false because the inverted check hides the tick
//!   when widthRatio is in range.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBar";

/// Distinct from the seeded `xp_max = 180_000` (`character_world.rs:235`)
/// and from any plausible defaulted-to-zero state, so a stale cold-state
/// reading on `bar.maxBar` cannot be confused with a successful refresh.
const NEW_XP: i64 = 12_345;
const NEW_XP_MAX: i64 = 50_000;
/// Chosen so `widthRatio = (NEW_XP + NEW_EXHAUSTION) / NEW_XP_MAX = 0.347`
/// lands cleanly inside the `hideAtBarEdge` [0.01, 0.99] range
/// (`ExpBar.lua:142`); see header comment for rationale.
const NEW_EXHAUSTION: i64 = 5_000;
const SEEDED_XP: f64 = 0.0;
const SEEDED_XP_MAX: f64 = 180_000.0;

/// Lua expression that resolves to the Experience-bar instance via the
/// `bars[Experience]` index defined at
/// `Shared/StatusTrackingManager.lua:5-13` and populated by
/// `Mainline/StatusTrackingManagerOverrides.lua:63`. Inlined into Lua
/// snippets so each probe re-resolves the path; if any link in the chain
/// regresses to nil, the eval errors out at the precise probe whose
/// assertion will surface the failure.
const EXP_BAR_LUA: &str = "StatusTrackingBarManager.MainStatusTrackingBarContainer\
    .bars[StatusTrackingBarInfo.BarsEnum.Experience]";

#[test]
fn xp_bar_round_trip_through_player_xp_update_event() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        assert_cold_globals_resolve(env);
        assert_cold_xp_bar_state(env);
        assert_cold_exhaustion_state(env);
        seed_player_xp(env);
        env.fire_event("PLAYER_XP_UPDATE")
            .expect("PLAYER_XP_UPDATE fire must dispatch cleanly");
        assert_post_event_xp_bar_state(env);
        assert_post_event_status_bar_values(env);
        assert_post_event_exhaustion_state(env);
    });
    }
}

fn assert_cold_globals_resolve(env: &WowLuaEnv) {
    let cold_globals_exist: bool = env
        .eval(
            r#"
            local manager = StatusTrackingBarManager
            local container = manager and manager.MainStatusTrackingBarContainer
            local exp_index = StatusTrackingBarInfo and StatusTrackingBarInfo.BarsEnum
                and StatusTrackingBarInfo.BarsEnum.Experience
            local bar = container and exp_index and container.bars
                and container.bars[exp_index]
            return manager ~= nil
                and container ~= nil
                and exp_index == 4
                and bar ~= nil
                and bar.StatusBar ~= nil
                and bar.ExhaustionTick ~= nil
            "#,
        )
        .expect("XP bar global existence probe must run cleanly");
    assert!(
        cold_globals_exist,
        "Chain StatusTrackingBarManager.MainStatusTrackingBarContainer.\
         bars[BarsEnum.Experience].{{StatusBar,ExhaustionTick}} must \
         resolve after `{ROOT}` load; nil reading means TOC/InitializeBars/\
         parentKey regression — see header observation 1."
    );
}

fn assert_cold_xp_bar_state(env: &WowLuaEnv) {
    let cold_curr_xp: f64 = env
        .eval(&format!("return {EXP_BAR_LUA}.currXP"))
        .expect("cold bar.currXP probe must run cleanly");
    assert_eq!(
        cold_curr_xp, SEEDED_XP,
        "Cold `bar.currXP` must equal seeded xp=0 (PlayerState default); \
         non-zero means seeded preset, `Update`/`GetLevelData`, or \
         `unit_xp` regressed — see header observation 2. Got {cold_curr_xp}.",
    );

    let cold_max_bar: f64 = env
        .eval(&format!("return {EXP_BAR_LUA}.maxBar"))
        .expect("cold bar.maxBar probe must run cleanly");
    assert_eq!(
        cold_max_bar, SEEDED_XP_MAX,
        "Cold `bar.maxBar` must equal seeded xp_max=180_000 \
         (PlayerState::seeded); wrong reading means seed regressed or \
         cold Update pass never ran — see header observation 2. Got \
         {cold_max_bar}.",
    );
}

fn assert_cold_exhaustion_state(env: &WowLuaEnv) {
    let cold_exhaustion_nil: bool = env
        .eval("return GetXPExhaustion() == nil")
        .expect("cold GetXPExhaustion probe must run cleanly");
    assert!(
        cold_exhaustion_nil,
        "Cold `GetXPExhaustion()` must be nil (PlayerXpState default \
         exhaustion=None) — see header observation 3."
    );

    let cold_tick_hidden: bool = env
        .eval(&format!(
            "return not {EXP_BAR_LUA}.ExhaustionTick:IsShown()"
        ))
        .expect("cold ExhaustionTick:IsShown probe must run cleanly");
    assert!(
        cold_tick_hidden,
        "Cold `ExhaustionTick:IsShown()` must be false (cold exhaustion=nil \
         drives the `not exhaustionThreshold → Hide` branch at \
         ExpBar.lua:135) — see header observation 3."
    );
}

fn seed_player_xp(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.player.xp = NEW_XP;
    state.player.xp_max = NEW_XP_MAX;
    state.player_xp.exhaustion = Some(NEW_EXHAUSTION);
}

fn assert_post_event_xp_bar_state(env: &WowLuaEnv) {
    let post_curr_xp: f64 = env
        .eval(&format!("return {EXP_BAR_LUA}.currXP"))
        .expect("post-event bar.currXP probe must run cleanly");
    assert_eq!(
        post_curr_xp, NEW_XP as f64,
        "Post-PLAYER_XP_UPDATE `bar.currXP` must equal {NEW_XP} \
         (registration → OnEvent → Update → UnitXP chain) — see header \
         observation 4. Got {post_curr_xp}.",
    );

    let post_max_bar: f64 = env
        .eval(&format!("return {EXP_BAR_LUA}.maxBar"))
        .expect("post-event bar.maxBar probe must run cleanly");
    assert_eq!(
        post_max_bar, NEW_XP_MAX as f64,
        "Post-PLAYER_XP_UPDATE `bar.maxBar` must equal {NEW_XP_MAX} \
         (Update writes `self.maxBar = UnitXPMax(\"player\")`) — see \
         header observation 4. Got {post_max_bar}.",
    );
}

fn assert_post_event_status_bar_values(env: &WowLuaEnv) {
    let post_status_bar_value: f64 = env
        .eval(&format!("return {EXP_BAR_LUA}.StatusBar:GetValue()"))
        .expect("post-event StatusBar:GetValue probe must run cleanly");
    assert_eq!(
        post_status_bar_value, NEW_XP as f64,
        "Post-PLAYER_XP_UPDATE `StatusBar:GetValue()` must equal {NEW_XP} \
         (SetBarValues → SetAnimatedValues → ProcessChangesInstantly → \
         SetValue); reading {SEEDED_XP} means the hidden-bar instant-flush \
         branch regressed — see header observation 4. Got \
         {post_status_bar_value}.",
    );

    let (post_min, post_max) = read_status_bar_min_max(env);
    assert_eq!(
        (post_min, post_max),
        (0.0, NEW_XP_MAX as f64),
        "Post-PLAYER_XP_UPDATE `StatusBar:GetMinMaxValues()` must return \
         (0, {NEW_XP_MAX}); wrong min means `minBar = 0` literal at \
         ExpBar.lua:36 regressed, wrong max means flush path broke — see \
         header observation 4. Got ({post_min}, {post_max}).",
    );
}

fn read_status_bar_min_max(env: &WowLuaEnv) -> (f64, f64) {
    let min: f64 = env
        .eval(&format!(
            "local minBar, _ = {EXP_BAR_LUA}.StatusBar:GetMinMaxValues() return minBar"
        ))
        .expect("post-event StatusBar:GetMinMaxValues min probe must run cleanly");
    let max: f64 = env
        .eval(&format!(
            "local _, maxBar = {EXP_BAR_LUA}.StatusBar:GetMinMaxValues() return maxBar"
        ))
        .expect("post-event StatusBar:GetMinMaxValues max probe must run cleanly");
    (min, max)
}

fn assert_post_event_exhaustion_state(env: &WowLuaEnv) {
    let post_exhaustion: f64 = env
        .eval("return GetXPExhaustion()")
        .expect("post-event GetXPExhaustion probe must run cleanly");
    assert_eq!(
        post_exhaustion, NEW_EXHAUSTION as f64,
        "Post-mutation `GetXPExhaustion()` must equal {NEW_EXHAUSTION} \
         (xp_honor_rest.rs Some-arm) — see header observation 4. Got \
         {post_exhaustion}.",
    );

    let post_tick_shown: bool = env
        .eval(&format!("return {EXP_BAR_LUA}.ExhaustionTick:IsShown()"))
        .expect("post-event ExhaustionTick:IsShown probe must run cleanly");
    assert!(
        post_tick_shown,
        "Post-PLAYER_XP_UPDATE `ExhaustionTick:IsShown()` must be true \
         (widthRatio = ({NEW_XP} + {NEW_EXHAUSTION}) / {NEW_XP_MAX} = \
         0.347, in [0.01, 0.99]); false means OnLoad/OnEvent registration \
         or `hideAtBarEdge` clamp regressed — see header observation 4."
    );
}
