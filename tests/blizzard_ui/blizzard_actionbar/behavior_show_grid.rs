//! Behavior pin: `MultiActionBar_ShowAllGrids(reason)` fans the `showgrid`
//! attribute (bit-flag) to every action button on every multi-bar; the
//! symmetric `MultiActionBar_HideAllGrids(reason)` clears the same bit. With
//! the grid attribute set, empty action slots draw the hover-target
//! placeholder texture instead of being invisible — that is what makes spell
//! drag-and-drop and SpellBook drag interactions land somewhere useful when
//! a slot is empty.
//!
//! Source contract (`Interface/BlizzardUI/Blizzard_ActionBar/`):
//!
//! 1. `MultiActionBar_ShowAllGrids(reason)`
//!    (`Shared/MultiActionBars.lua:104-111`) calls `GetMultiActionBars()`
//!    (lua:58-78). The lazy-initialized table maps page → `{bar, getIsVisible}`
//!    for the seven multi-bars: `MultiBarBottomLeft`, `MultiBarBottomRight`,
//!    `MultiBarRight`, `MultiBarLeft`, `MultiBar5`, `MultiBar6`, `MultiBar7`.
//!    The early-out at lua:61-63 returns nil if any of the seven globals is
//!    missing — so the helper is a no-op until every bar's `MultiActionBars.xml`
//!    `<Frame>` has loaded and run `ActionBar_OnLoad`. The loop at lua:107-109
//!    dispatches `barEntry.bar:SetShowGrid(true, reason)` on each entry.
//!
//! 2. `ActionBarMixin:SetShowGrid(showGrid, reason)`
//!    (`Shared/ActionBar.lua:144-171`). Returns early at lua:145-147 if
//!    `not showGrid and KeybindFrames_InQuickKeybindMode()` —
//!    `KeybindFrames_InQuickKeybindMode` is defined at
//!    `Blizzard_SharedXML/BindingUtil.lua:166-168` as
//!    `return QuickKeybindFrame and QuickKeybindFrame:IsShown()`. The
//!    `QuickKeybindFrame` global is created by `Blizzard_QuickKeybind`, which
//!    the test harness does NOT load, so `KeybindFrames_InQuickKeybindMode()`
//!    returns nil (falsy) and the hide path is never short-circuited here.
//!    At lua:151-159, when `reason == ACTION_BUTTON_SHOW_GRID_REASON_EVENT`
//!    (=2, `Shared/ActionButton.lua:13`) or
//!    `ACTION_BUTTON_SHOW_GRID_REASON_SPELLCOLLECTION` (=4,
//!    `ActionButton.lua:14`), the bar's own `showAllButtons` bit-flag field is
//!    OR'd in (show) or AND-NOT'd out (hide) — this branch has NO `issecure`
//!    or "already shown" gate, so it always reflects the most recent
//!    Show/Hide call for those reasons. REASON_CVAR (=1) does NOT touch
//!    `showAllButtons` — only the per-button attribute. Then lua:161-163
//!    fans the call out to every entry in `self.actionButtons` (populated by
//!    `ActionBar_OnLoad` lua:13-44 — twelve buttons per multi-bar because the
//!    `<KeyValue key="numButtons" value="12"/>` at MultiActionBars.xml:52 etc.).
//!
//! 3. `BaseActionButtonMixin:SetShowGrid(showGrid, reason)`
//!    (`Shared/ActionButton.lua:1533-1544`). Asserts `reason` is non-nil
//!    (so the test must pass a numeric reason — `bit.bor`/`bit.band` further
//!    require numeric input). Gated on `issecure() and GetShowGrid() ~=
//!    showGrid` at lua:1536, then either `bit.bor(showGridAttribute or 0,
//!    reason)` (show) or `bit.band(showGridAttribute or 0, bit.bnot(reason))`
//!    (hide) writes the new value to the `showgrid` attribute via
//!    `SetAttribute`. The matching reader `GetShowGrid()` (lua:1528-1531)
//!    returns `(showGridAttribute > 0)`. The boolean `~=` gate is critical
//!    to test setup: if any reason bit is already set, `GetShowGrid()`
//!    returns true, and a `SetShowGrid(true, anyOtherReason)` call becomes a
//!    no-op (it does NOT OR the new bit in). Production semantics treat this
//!    as "grid already shown, nothing to do" — but for a clean round-trip
//!    test, the cold-state bit set by EditMode's
//!    `UpdateSystemSettingAlwaysShowButtons`
//!    (`Blizzard_EditMode/Shared/EditModeSystemTemplates.lua:1141-1147`,
//!    sets REASON_CVAR=1 during system load) must be cleared on the sample
//!    buttons before the round trip can be observed.
//!
//! 4. `MultiActionBar_HideAllGrids(reason)` (lua:113-120) is the symmetric
//!    inverse of ShowAllGrids: same iteration,
//!    `barEntry.bar:SetShowGrid(false, reason)`. With matching reason, the
//!    per-button `bit.band(... bit.bnot(reason))` clears the bit set by
//!    Show. Bits set under a *different* reason are preserved — that is
//!    intentional: drag-cursor (REASON_EVENT) and spellbook-open
//!    (REASON_SPELLCOLLECTION) can co-exist without one closing erasing the
//!    other's "stay shown" intent.
//!
//! Why the test pre-clears the sample buttons' `showgrid` attribute via raw
//! `SetAttribute("showgrid", 0)` before the round trip: the per-button
//! `SetShowGrid` gate at `ActionButton.lua:1536` short-circuits when
//! `GetShowGrid()` already matches the requested boolean. EditMode sets
//! REASON_CVAR=1 during system load, so cold-state `GetShowGrid()` returns
//! true on every multi-bar button. Calling `MultiActionBar_ShowAllGrids(2)`
//! then sees `true ~= true == false` and never executes the
//! `bit.bor(... or 0, 2)` write. The pre-clear is a test fixture (resetting
//! the attribute is permitted from secure context) — it is NOT part of the
//! production code path being pinned. The pinned path remains
//! `MultiActionBar_ShowAllGrids` → `ActionBarMixin:SetShowGrid` → fanout →
//! `BaseActionButtonMixin:SetShowGrid` → bit math.
//!
//! Why the test passes a numeric reason and not the placeholder string in
//! the PLAN.md task title (`"test_reason"`): the per-button impl uses
//! `bit.bor`/`bit.band` which only accepts numeric input; a string would
//! raise inside `SetShowGrid`. The test uses
//! `ACTION_BUTTON_SHOW_GRID_REASON_EVENT` (=2) — the same reason
//! `MultiActionBar_ShowAllGrids` takes when invoked from the live
//! `ACTIONBAR_SHOWGRID` event handler (`Shared/ActionButtonUtil.lua:43-44`),
//! so the test pins the production code path.
//!
//! The test pins the following observations across the round trip:
//!   1. **All seven multi-bar globals exist after harness settle.** A nil
//!      reading on any of them means `MultiActionBars.xml` did not finish
//!      loading or one of the seven `<Frame>` declarations regressed —
//!      `GetMultiActionBars()` would return nil at lua:62 and the entire
//!      Show/Hide cycle would silently no-op.
//!   2. **`MultiBarBottomLeftButton1`, `MultiBarBottomLeftButton12`, and
//!      `MultiBarRightButton1` exist as globals.** These are created by
//!      `ActionBar_OnLoad` lua:31 via `CreateFrame("CheckButton",
//!      buttonName, ..., self.buttonTemplate, i)` with `buttonName =
//!      actionBarName.."Button"..i` for non-Main/Stance/Pet/Possess bars
//!      (lua:27-28). A nil reading on any of them means OnLoad did not run
//!      or the global naming branch regressed. The samples span both
//!      intra-bar fanout (BottomLeft #1 vs. BottomLeft #12, proves the
//!      `actionButtons` loop at lua:161 hits the last button) and cross-bar
//!      fanout (BottomLeft vs. Right, proves the `GetMultiActionBars` table
//!      iteration at lua:107 reaches every bar).
//!   3. **After test fixture pre-clears `showgrid=0`, `:GetShowGrid()`
//!      reads false on every sample.** This is a sanity check on the test
//!      fixture itself — confirms `SetAttribute` writes are observed by the
//!      `GetShowGrid` reader. A true reading means the fixture write didn't
//!      stick or the GetShowGrid reader regressed.
//!   4. **After `MultiActionBar_ShowAllGrids(REASON_EVENT=2)`,
//!      `:GetShowGrid()` returns true on every sample button** AND the
//!      `showgrid` attribute equals exactly `REASON_EVENT` (=2). The chain
//!      fans through `GetMultiActionBars` (proves cross-bar iteration via
//!      Right vs. BottomLeft) and `self.actionButtons` (proves intra-bar
//!      fanout via #1 vs. #12). Per-button `SetShowGrid(true, 2)` writes
//!      `bit.bor(0, 2) = 2`. A false `GetShowGrid` on `BottomLeft #12`
//!      means the bar's `actionButtons` loop short-circuited mid-fan; a
//!      false on `Right #1` means `GetMultiActionBars` returned a partial
//!      subset; false on all three means the per-button `issecure()` gate
//!      at lua:1536 was inverted, or `MultiActionBar_ShowAllGrids` didn't
//!      iterate at all. An attribute value other than 2 means the
//!      `bit.bor(... or 0, reason)` math at lua:1539 regressed.
//!   5. **Show sets the bar's `showAllButtons` field to exactly
//!      REASON_EVENT (=2).** This branch at ActionBar.lua:151-155 has no
//!      gate, so it pins the iteration and bit math reliably. A different
//!      value means the branch evaluated the wrong reason or the
//!      `bit.bor(... or 0, reason)` math regressed.
//!   6. **After `MultiActionBar_HideAllGrids(REASON_EVENT=2)`,
//!      `:GetShowGrid()` returns false on every sample button** AND the
//!      `showgrid` attribute equals exactly 0. The per-button impl writes
//!      `bit.band(2, bit.bnot(2)) = 0`. A true `GetShowGrid` reading means
//!      the Hide path didn't fan, the per-button gate dropped the call, or
//!      the AND-NOT bit math regressed. A non-zero attribute value means
//!      `bit.bnot` math is wrong.
//!   7. **Hide clears the bar's `showAllButtons` field back to 0.** The
//!      symmetric AND-NOT path at ActionBar.lua:156-158. A non-zero reading
//!      means the symmetric clear regressed.
//!
//! Regression candidates the assertions catch:
//!   - `GetMultiActionBars()` mutated to reference a missing global: the
//!     early-out returns nil and the entire Show/Hide cycle no-ops; sample
//!     buttons stay at their pre-Show value. Observation 4 fails on every
//!     sample.
//!   - `actionBar:SetShowGrid` not iterating `self.actionButtons`: only the
//!     bar's `showAllButtons` flag flips; per-button attribute stays unset.
//!     Observation 4 fails on every button; observation 5 still passes.
//!   - Per-button `SetShowGrid` `issecure()` gate inverted: secure callers
//!     skip the write entirely; per-button attribute stays unset.
//!     Observations 4 and 6 both fail.
//!   - Bit-flag inversion (`bit.bor`/`bit.band` swapped at lua:1539-1541):
//!     Show clears bit 2, Hide sets it. Observations 4 and 6 fail with
//!     reversed values.
//!   - Reason-branch `EVENT or SPELLCOLLECTION` widened to include CVAR:
//!     wouldn't fail this test directly but would break drag-cursor and
//!     spellbook-open coexistence. Out of scope.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::lua_api::WowLuaEnv;

const ROOT: &str = "Blizzard_ActionBar";
const SHOW_GRID_REASON_EVENT: i32 = 2;

struct SampleGridState {
    bottomleft_first: bool,
    bottomleft_last: bool,
    right_first: bool,
}

fn assert_multi_bars_exist(env: &WowLuaEnv) {
    let all_bars_exist: bool = env
        .eval(
            r#"
            return MultiBarBottomLeft ~= nil
                and MultiBarBottomRight ~= nil
                and MultiBarRight ~= nil
                and MultiBarLeft ~= nil
                and MultiBar5 ~= nil
                and MultiBar6 ~= nil
                and MultiBar7 ~= nil
            "#,
        )
        .expect("multi-bar global existence probe must run cleanly");
    assert!(
        all_bars_exist,
        "After the startup-shape harness loads `{ROOT}`, all seven \
         multi-bar globals must exist (MultiBarBottomLeft, MultiBarBottomRight, \
         MultiBarRight, MultiBarLeft, MultiBar5, MultiBar6, MultiBar7). \
         They are declared as `<Frame>` elements in \
         Shared/MultiActionBars.xml:45-220, parented to UIParent. \
         `GetMultiActionBars` (Shared/MultiActionBars.lua:61-63) returns \
         nil if any one is missing, which silently disables \
         `MultiActionBar_ShowAllGrids`. A false reading here means the \
         XML didn't load or a `<Frame name=...>` regressed."
    );
}

fn assert_sample_buttons_exist(env: &WowLuaEnv) {
    let sample_buttons_exist: bool = env
        .eval(
            r#"
            return MultiBarBottomLeftButton1 ~= nil
                and MultiBarBottomLeftButton12 ~= nil
                and MultiBarRightButton1 ~= nil
            "#,
        )
        .expect("sample button global existence probe must run cleanly");
    assert!(
        sample_buttons_exist,
        "After OnLoad, `MultiBarBottomLeftButton1`, \
         `MultiBarBottomLeftButton12`, and `MultiBarRightButton1` must \
         exist as globals. `ActionBar_OnLoad` (Shared/ActionBar.lua:13-44) \
         creates each button via `CreateFrame(\"CheckButton\", buttonName, \
         ..., self.buttonTemplate, i)` with `buttonName = \
         actionBarName..\"Button\"..i` for non-Main/Stance/Pet/Possess \
         bars (lua:27-28). A nil reading means OnLoad did not run for \
         one of the bars, or the global naming branch regressed. The \
         samples span both intra-bar fanout (BottomLeft #1 vs. #12) and \
         cross-bar fanout (BottomLeft vs. Right)."
    );
}

fn clear_multibar_showgrid_attributes(env: &WowLuaEnv) {
    env.exec(
        r#"
        -- Test fixture: clear the `showgrid` attribute on every multi-bar
        -- button. EditMode's UpdateSystemSettingAlwaysShowButtons sets
        -- bit 1 (REASON_CVAR) during system load, which makes the
        -- per-button SetShowGrid gate at ActionButton.lua:1536 short-
        -- circuit. Pre-clearing gives a clean cold state of 0 so the
        -- Show/Hide round trip is observable.
        local bars = {MultiBarBottomLeft, MultiBarBottomRight, MultiBarRight,
                      MultiBarLeft, MultiBar5, MultiBar6, MultiBar7}
        for _, bar in ipairs(bars) do
            if bar.actionButtons then
                for _, button in pairs(bar.actionButtons) do
                    button:SetAttribute("showgrid", 0)
                end
            end
        end
        "#,
    )
    .expect("test fixture pre-clear of showgrid attributes must run cleanly");
}

fn sample_button_grid_state(env: &WowLuaEnv, stage: &str) -> SampleGridState {
    SampleGridState {
        bottomleft_first: env
            .eval("return MultiBarBottomLeftButton1:GetShowGrid()")
            .unwrap_or_else(|_| panic!("BottomLeft #1 {stage} GetShowGrid probe must run cleanly")),
        bottomleft_last: env
            .eval("return MultiBarBottomLeftButton12:GetShowGrid()")
            .unwrap_or_else(|_| {
                panic!("BottomLeft #12 {stage} GetShowGrid probe must run cleanly")
            }),
        right_first: env
            .eval("return MultiBarRightButton1:GetShowGrid()")
            .unwrap_or_else(|_| panic!("Right #1 {stage} GetShowGrid probe must run cleanly")),
    }
}

fn bottomleft_first_showgrid_attr(env: &WowLuaEnv, stage: &str) -> i32 {
    env.eval("return MultiBarBottomLeftButton1:GetAttribute('showgrid') or -1")
        .unwrap_or_else(|_| {
            panic!("BottomLeft #1 {stage} showgrid attribute probe must run cleanly")
        })
}

fn bottomleft_show_all_buttons(env: &WowLuaEnv, stage: &str) -> i32 {
    env.eval("return MultiBarBottomLeft.showAllButtons or -1")
        .unwrap_or_else(|_| {
            panic!("MultiBarBottomLeft.showAllButtons {stage} probe must run cleanly")
        })
}

fn show_all_grids(env: &WowLuaEnv) {
    env.exec(&format!(
        "MultiActionBar_ShowAllGrids({SHOW_GRID_REASON_EVENT})"
    ))
    .expect("MultiActionBar_ShowAllGrids must be callable as a global function");
}

fn hide_all_grids(env: &WowLuaEnv) {
    env.exec(&format!(
        "MultiActionBar_HideAllGrids({SHOW_GRID_REASON_EVENT})"
    ))
    .expect("MultiActionBar_HideAllGrids must be callable as a global function");
}

fn assert_post_fixture_grid_clear(state: &SampleGridState) {
    assert!(
        !state.bottomleft_first && !state.bottomleft_last && !state.right_first,
        "After the test fixture writes `showgrid=0` to every multi-bar \
         button, `:GetShowGrid()` must return false on every sample. \
         `GetShowGrid` (Shared/ActionButton.lua:1528-1531) returns \
         `(showGridAttribute > 0)`. A true reading here means the \
         SetAttribute write didn't stick (broken attribute storage) or \
         the GetShowGrid reader regressed and is reading a different \
         field."
    );
}

fn assert_post_show_grid_state(state: &SampleGridState) {
    assert!(
        state.bottomleft_first && state.bottomleft_last && state.right_first,
        "After `MultiActionBar_ShowAllGrids({SHOW_GRID_REASON_EVENT})`, \
         `:GetShowGrid()` must return true on every sampled button. \
         The chain: lua:107 iterates GetMultiActionBars's table (proves \
         multi-bar table iteration via Right-vs-BottomLeft samples), \
         and lua:161 iterates `self.actionButtons` (proves intra-bar \
         fanout via #1-vs-#12 samples). Per-button \
         `SetShowGrid(true, 2)` writes `bit.bor(0, 2) = 2` into the \
         `showgrid` attribute (Shared/ActionButton.lua:1539). \
         `GetShowGrid` then returns `(2 > 0) = true`. A false reading \
         on BottomLeft #12 means the bar's `actionButtons` loop \
         short-circuited mid-fan; a false reading on Right #1 means \
         `GetMultiActionBars` returned a partial subset; a false \
         reading on all three means the per-button `issecure()` gate \
         at lua:1536 was inverted, or `MultiActionBar_ShowAllGrids` \
         didn't iterate at all. Got: BottomLeft #1={}, \
         BottomLeft #12={}, Right #1={}.",
        state.bottomleft_first,
        state.bottomleft_last,
        state.right_first
    );
}

fn assert_post_hide_grid_state(state: &SampleGridState) {
    assert!(
        !state.bottomleft_first && !state.bottomleft_last && !state.right_first,
        "After `MultiActionBar_HideAllGrids({SHOW_GRID_REASON_EVENT})`, \
         `:GetShowGrid()` must return false on every sampled button. \
         The per-button impl writes `bit.band(2, bit.bnot(2)) = 0` \
         into the `showgrid` attribute (Shared/ActionButton.lua:1541). \
         `GetShowGrid` then returns `(0 > 0) = false`. A true reading \
         means either the Hide path didn't fan to actionButtons \
         (Shared/ActionBar.lua:161-163), or the AND-NOT bit math \
         regressed and the bit set by Show is still present. Got: \
         BottomLeft #1={}, BottomLeft #12={}, Right #1={}.",
        state.bottomleft_first,
        state.bottomleft_last,
        state.right_first
    );
}

fn assert_post_show_flags(env: &WowLuaEnv) {
    let show_attr_bottomleft_first = bottomleft_first_showgrid_attr(env, "post-Show");
    assert_eq!(
        show_attr_bottomleft_first, SHOW_GRID_REASON_EVENT,
        "After `MultiActionBar_ShowAllGrids({SHOW_GRID_REASON_EVENT})` \
         on a freshly-cleared (showgrid=0) cold state, the `showgrid` \
         attribute must equal exactly REASON_EVENT (=2) on \
         MultiBarBottomLeftButton1. The per-button impl computes \
         `bit.bor(0, 2) = 2` (Shared/ActionButton.lua:1539). A value \
         different from 2 means the bit math regressed (e.g. \
         `bit.bor(0, 4)` — wrong reason constant) or the SetAttribute \
         call wrote something else. Got: {show_attr_bottomleft_first}."
    );

    let bar_show_all_buttons_after_show = bottomleft_show_all_buttons(env, "post-Show");
    assert_eq!(
        bar_show_all_buttons_after_show, SHOW_GRID_REASON_EVENT,
        "After `MultiActionBar_ShowAllGrids({SHOW_GRID_REASON_EVENT})`, \
         `MultiBarBottomLeft.showAllButtons` must equal exactly \
         REASON_EVENT (=2). The `reason == \
         ACTION_BUTTON_SHOW_GRID_REASON_EVENT` branch at \
         Shared/ActionBar.lua:151-155 writes \
         `bit.bor(self.showAllButtons or 0, reason)` so subsequent \
         `UpdateVisibility` calls keep the bar shown while a drag is \
         active. Cold-state `showAllButtons` is nil/0 (only set by \
         EVENT/SPELLCOLLECTION reasons, not by EditMode's CVAR), so the \
         OR yields exactly 2. A different value means the branch \
         evaluated the wrong reason or the bit math regressed. \
         Got: {bar_show_all_buttons_after_show}."
    );
}

fn assert_post_hide_flags(env: &WowLuaEnv) {
    let hide_attr_bottomleft_first = bottomleft_first_showgrid_attr(env, "post-Hide");
    assert_eq!(
        hide_attr_bottomleft_first, 0,
        "After `MultiActionBar_HideAllGrids({SHOW_GRID_REASON_EVENT})` \
         on a state where Show set the attribute to exactly 2, the \
         `showgrid` attribute must equal exactly 0 on \
         MultiBarBottomLeftButton1. The per-button impl computes \
         `bit.band(2, bit.bnot(2)) = 0` (Shared/ActionButton.lua:1541). \
         A value different from 0 means the bit math regressed (e.g. \
         missing `bit.bnot`). Got: {hide_attr_bottomleft_first}."
    );

    let bar_show_all_buttons_after_hide = bottomleft_show_all_buttons(env, "post-Hide");
    assert_eq!(
        bar_show_all_buttons_after_hide, 0,
        "After `MultiActionBar_HideAllGrids({SHOW_GRID_REASON_EVENT})`, \
         `MultiBarBottomLeft.showAllButtons` must equal exactly 0. The \
         same `reason == EVENT` branch at Shared/ActionBar.lua:151-158 \
         takes the AND-NOT path on the hide direction (lua:157), \
         writing `bit.band(2, bit.bnot(2)) = 0`. A non-zero value \
         means the symmetric clear regressed. \
         Got: {bar_show_all_buttons_after_hide}."
    );
}

#[test]
fn multi_action_bar_show_all_grids_fans_showgrid_attribute_to_every_multibar_button() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        assert_multi_bars_exist(env);
        assert_sample_buttons_exist(env);
        clear_multibar_showgrid_attributes(env);
        assert_post_fixture_grid_clear(&sample_button_grid_state(env, "post-fixture"));

        show_all_grids(env);
        assert_post_show_grid_state(&sample_button_grid_state(env, "post-Show"));
        assert_post_show_flags(env);

        hide_all_grids(env);
        assert_post_hide_grid_state(&sample_button_grid_state(env, "post-Hide"));
        assert_post_hide_flags(env);
    });
    }
}
