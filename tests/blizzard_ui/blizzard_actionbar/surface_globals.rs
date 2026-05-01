//! Surface-level globals pinned by the `Blizzard_ActionBar` lane —
//! action-button keybind input dispatch.
//!
//! PLAN.md task: `ActionButtonDown`, `ActionButtonUp`,
//! `MultiActionButtonDown`, `MultiActionButtonUp`, `ExtraActionButtonKey`,
//! `TryUseActionButton`, `ActionBar_PageUp`, `ActionBar_PageDown` are
//! functions (depends-on: action-button input globals gap).
//!
//! All eight PLAN-named globals match the actual Blizzard source verbatim —
//! no spec/source mismatches in this batch. Each is declared via the
//! `function <name>(args)` syntax (not `<name> = function(args)`), which
//! under Lua 5.1 semantics binds to the global table at file-chunk
//! execution time:
//!
//! - `TryUseActionButton(self, checkingFromDown)` at
//!   `Shared/ActionButton.lua:109` — the shared "fire the secure click"
//!   helper. Calls `SecureActionButton_OnClick(self, "LeftButton",
//!   checkingFromDown, isKeyPress=true, isSecureAction=true)`, then if the
//!   click resolved to a real action, clears any "new action highlight"
//!   mark on `self.action` and refreshes the highlight visual via
//!   `self:UpdateHighlightMark()`. Always finishes with
//!   `self:UpdateState()` so the button's checked/visual state stays in
//!   sync. Both `ActionButtonDown`/`Up`, `MultiActionButtonDown`/`Up`, and
//!   `ExtraActionButtonKey` route through this single helper.
//!
//! - `ActionButtonDown(id)` at `Shared/ActionButton.lua:136` — the
//!   `BINDING_HEADER_ACTIONBAR` keybind down-handler. Early-returns when
//!   `CheckPetActionButtonEvent(id, true)` reports the player is in pet
//!   battle (the helper at line 123 routes to `PetBattleFrame_ButtonDown`
//!   instead). Otherwise resolves the bound button via
//!   `GetActionButtonForID(id)`, transitions `NORMAL`→`PUSHED` via
//!   `SetButtonState`, and dispatches `TryUseActionButton(button, true)`
//!   with `checkingFromDown=true`.
//!
//! - `ActionButtonUp(id)` at `Shared/ActionButton.lua:151` — the
//!   key-release counterpart. Same pet-battle short-circuit, then
//!   `PUSHED`→`NORMAL` transition + `TryUseActionButton(button, false)`.
//!   The down/up split exists because keybindings dispatch separately on
//!   key-press vs key-release; the secure click semantics differ between
//!   the two phases (only one of them actually fires the spell, depending
//!   on the `ActionButtonUseKeyDown` CVar).
//!
//! - `MultiActionButtonDown(barName, id)` at `Shared/MultiActionBars.lua:35`
//!   — keybind down-handler for the multi-bars (Bottom Right / Bottom
//!   Left / Right / Right2 / and the additional 3 bars added in newer
//!   patches). Reads `_G[barName].actionButtons[id]` to resolve the
//!   button and dispatches the same `NORMAL`→`PUSHED` + `TryUseActionButton`
//!   pair as `ActionButtonDown`. Two-arg surface (the bar name disambiguates
//!   which multi-bar the keypress belongs to; `id` is the button index 1-12
//!   within that bar).
//!
//! - `MultiActionButtonUp(barName, id)` at `Shared/MultiActionBars.lua:44`
//!   — key-release counterpart for the multi-bars. **Asymmetric with
//!   `ActionButtonUp`**: only fires `TryUseActionButton(button, false)` if
//!   the button was already in `PUSHED` state (i.e. had received a matching
//!   `MultiActionButtonDown`). The single-bar `ActionButtonUp` ALWAYS fires
//!   `TryUseActionButton` even from `NORMAL`, but the multi-bar variant
//!   gates on the down-state. This asymmetry exists because the multi-bars
//!   can be hidden mid-keypress via `MultiActionBar_HideAllGrids`, leaving
//!   a stranded button in `NORMAL` whose Up handler should not re-fire
//!   the action.
//!
//! - `ExtraActionButtonKey(id, isDown)` at `Shared/ExtraActionBar.lua:63`
//!   — keybind handler for the contextual extra-action bar (vehicle
//!   abilities, encounter-specific extra buttons). **Single function
//!   handles both down AND up phases** via the `isDown` parameter — the
//!   only entry point in the lane that fuses both phases into one global.
//!   Early-returns when `C_ActionBar.HasExtraActionBar()` is false (the
//!   bar isn't currently active). Resolves `_G["ExtraActionButton"..id]`
//!   and dispatches the same state-transition + TryUseActionButton pair
//!   as the regular Up handler, with the same Multi-bar-style PUSHED-gate
//!   on the up phase.
//!
//! - `ActionBar_PageUp()` at `Shared/ActionButton.lua:166` — the
//!   `BINDING_NAME_ACTIONPAGE2` (and similar) page-cycle handler. Walks
//!   the `VIEWABLE_ACTION_BAR_PAGES` table from `current_page + 1` to
//!   `NUM_ACTIONBAR_PAGES`, picks the first viewable page, and dispatches
//!   `C_ActionBar.SetActionBarPage(nextPage)`. Wraps to page 1 when no
//!   later page is viewable. Zero-arg surface.
//!
//! - `ActionBar_PageDown()` at `Shared/ActionButton.lua:181` — the
//!   page-cycle handler in the opposite direction. Walks
//!   `current_page - 1` down to 1, then wraps to the highest viewable
//!   page. Two-pass loop (descend, then wrap) — slightly larger control
//!   flow than `_PageUp` because the wrap target has to be found
//!   iteratively rather than always 1. Zero-arg surface.
//!
//! Why a single test with a table-driven loop is sufficient: every PLAN
//! global has the same contract — `type(name) == "function"` after the
//! startup-shape harness loads `Blizzard_ActionBar`. Splitting them into
//! per-global tests would buy nothing (all eight failure modes have the
//! same root cause — file chunk failed before reaching the declaration,
//! OR Blizzard re-shaped the surface).
//!
//! **Note on the gap dependency.** PLAN.md line 93 ("Implement action-
//! button input globals ...") is already checked off — the simulator's
//! `runtime_surface_bootstrap.lua` / Rust env-init publishes
//! `ActionButtonDown`, `ActionButtonUp`, `MultiActionButtonDown`,
//! `MultiActionButtonUp`, `ExtraActionButtonKey`, and `TryUseActionButton`
//! BEFORE any addon loads. The Blizzard source then re-declares these at
//! file scope (lines 109/136/151 in ActionButton.lua, 35/44 in
//! MultiActionBars.lua, 63 in ExtraActionBar.lua), overwriting the
//! simulator's pre-published versions. The post-load contract is therefore
//! a two-way pin: either the simulator's pre-publish OR the addon's
//! source declaration must result in a function. `ActionBar_PageUp` and
//! `ActionBar_PageDown` are the only two not on the gap line — they're
//! published exclusively by the addon's source (lines 166/181). A nil
//! reading on those two would prove the addon's `Shared/ActionButton.lua`
//! file chunk failed before reaching the declarations.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBar";
const PLAN_NAMED_INPUT_GLOBALS: &[&str] = &[
    "ActionButtonDown",
    "ActionButtonUp",
    "MultiActionButtonDown",
    "MultiActionButtonUp",
    "ExtraActionButtonKey",
    "TryUseActionButton",
    "ActionBar_PageUp",
    "ActionBar_PageDown",
];
const PLAN_NAMED_HIGHLIGHT_GLOBALS: &[&str] = &[
    "MarkNewActionHighlight",
    "ClearNewActionHighlight",
    "GetNewActionHighlightMark",
    "ClearOnBarHighlightMarks",
    "GetOnBarHighlightMark",
    "UpdateOnBarHighlightMarksBySpell",
    "UpdateOnBarHighlightMarksByFlyout",
    "UpdateOnBarHighlightMarksByPetAction",
    "GetActionButtonForID",
];

#[test]
fn action_bar_plan_named_input_globals_are_functions() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for name in PLAN_NAMED_INPUT_GLOBALS {
            let is_function = env
                .eval::<bool>(&format!(r#"return type({name}) == "function""#))
                .expect("file-scope global type probe must run cleanly");

            assert!(
                is_function,
                "Global `{name}` MUST be a function after the startup-shape harness loads \
                 `Blizzard_ActionBar`. Each entry in `PLAN_NAMED_INPUT_GLOBALS` is declared via \
                 `function <name>(...)` syntax in the lane's Lua files \
                 (Shared/ActionButton.lua:109/136/151/166/181, Shared/MultiActionBars.lua:35/44, \
                 Shared/ExtraActionBar.lua:63). A non-function reading proves either (a) the \
                 file chunk failed before reaching the declaration (load.rs's \
                 `action_bar_load_emits_no_lane_specific_lua_errors` would also fail in that case \
                 — multi-test failure mode), OR (b) Blizzard re-shaped the keybind dispatch \
                 surface (e.g. moved `ActionButtonDown` onto a SecureHandler-style frame method \
                 instead of a top-level global, which would break every existing `Bindings.xml` \
                 entry that names `ActionButtonDown` directly), OR (c) — for `ActionButtonDown`, \
                 `_Up`, `MultiActionButtonDown`, `_Up`, `ExtraActionButtonKey`, \
                 `TryUseActionButton` — the simulator's gap-fill pre-publish at PLAN.md:93 \
                 regressed AND the addon's source-side declaration didn't run; for \
                 `ActionBar_PageUp`/`_PageDown` (the only two NOT on the gap-fill line) a nil \
                 reading proves the addon's file chunk failed before the declaration line. \
                 Got `type({name}) == \"function\"` returned false."
            );
        }
    });
}

/// Pin the action-highlight-tracking globals — the surface that
/// `SpellBookFrame`, `Blizzard_PlayerSpells`, and `Blizzard_TalentUI`
/// consume to mark "new action" indicators on action bar buttons and to
/// flag the "this spell is already on a bar" hint while a player is
/// dragging from spellbook/talent panels.
///
/// PLAN.md task: `MarkNewActionHighlight`, `ClearNewActionHighlight`,
/// `GetNewActionHighlightMark`, `ClearOnBarHighlightMarks`,
/// `GetOnBarHighlightMark`, `UpdateOnBarHighlightMarksBySpell`,
/// `UpdateOnBarHighlightMarksByFlyout`,
/// `UpdateOnBarHighlightMarksByPetAction`, `GetActionButtonForID` are
/// functions (depends-on: action highlight tracking globals gap).
///
/// All 9 PLAN-named globals match the actual Blizzard source verbatim.
/// Each is declared via the `function <name>(...)` syntax (not `<name> =
/// function(...)`), which under Lua 5.1 semantics binds to the global
/// table at file-chunk execution time. Source map (all in
/// `Shared/ActionButton.lua`):
///
/// - `MarkNewActionHighlight(action)` at `lua:27` — single-line setter:
///   `ACTION_HIGHLIGHT_MARKS[action] = true`. The file-scope
///   `ACTION_HIGHLIGHT_MARKS = { }` declaration at lua:9 owns the
///   storage; this function is the public mutator.
///
/// - `ClearNewActionHighlight(action, preventIdenticalActionsFromClearing)`
///   at `lua:31` — clears the mark on `action` and (unless the second
///   arg is truthy) walks `ACTION_HIGHLIGHT_MARKS` to also clear marks
///   on every other action whose `(actionType, actionID)` from
///   `GetActionInfo` matches the cleared one. The cascade exists so
///   that putting the same spell on two bars only requires one click
///   to dismiss both highlights.
///
/// - `GetNewActionHighlightMark(action)` at `lua:61` — read accessor.
///   Trampolines through `securecallfunction(SecureGetNewActionHighlightMark, ...)`
///   (the local at lua:57) so taint from the caller's stack doesn't
///   leak into the table read. The comment at lua:55 flags
///   `ACTION_HIGHLIGHT_MARKS`/`ON_BAR_HIGHLIGHT_MARKS` keys as
///   "vulnerable to taint from talent and spellbook code" —
///   `securecallfunction` is the runtime defence.
///
/// - `ClearOnBarHighlightMarks()` at `lua:65` — re-binds
///   `ON_BAR_HIGHLIGHT_MARKS` to a fresh empty table (NOT
///   `wipe(ON_BAR_HIGHLIGHT_MARKS)` — re-binding is intentional so
///   stale references held by tainted callers don't survive the clear).
///   Zero-arg.
///
/// - `GetOnBarHighlightMark(action)` at `lua:73` — read accessor.
///   Mirror of `GetNewActionHighlightMark` but against
///   `ON_BAR_HIGHLIGHT_MARKS`; same `securecallfunction` taint
///   firewall, same single-arg shape.
///
/// - `UpdateOnBarHighlightMarksBySpell(spellID)` at `lua:85` — calls
///   `C_ActionBar.FindSpellActionButtons(spellID)` to resolve the bar
///   slots that currently hold the spell, then dispatches to the
///   file-local `UpdateOnBarHighlightMarks` (lua:77) which `tInvert`s
///   the result so `ON_BAR_HIGHLIGHT_MARKS[slot] = true` for each
///   matching slot. Falls through to `ClearOnBarHighlightMarks()` when
///   no buttons match.
///
/// - `UpdateOnBarHighlightMarksByFlyout(flyoutID)` at `lua:89` — same
///   shape as the spell variant but probes `FindFlyoutActionButtons`
///   for flyout-button slots (the master flyout button, e.g. for
///   mage portals or hunter pet specials).
///
/// - `UpdateOnBarHighlightMarksByPetAction(petAction)` at `lua:93` —
///   same shape, `FindPetActionButtons`. The three Update*By* variants
///   are deliberately thin trampolines into one shared
///   `UpdateOnBarHighlightMarks` helper — keeping the public surface
///   matched to the three different action-source kinds while letting
///   the storage re-write logic live in one place.
///
/// - `GetActionButtonForID(id)` at `lua:97` — the keybind dispatcher
///   uses this to map an action-button id (1-12) to a frame. Branches
///   on `OverrideActionBar:IsShown()`: when the override bar is up
///   (vehicle / possess / temporary shapeshift), returns
///   `_G["OverrideActionBarButton"..id]` (or nil when `id >
///   NUM_OVERRIDE_BUTTONS`); otherwise returns
///   `_G["ActionButton"..id]`. A regression on this function would
///   silently break every keybind on the override bar.
///
/// **Note on the gap dependency.** PLAN.md line 94 ("Implement action
/// highlight tracking globals ...") is checked off — the simulator's
/// `runtime_surface_bootstrap.lua` / Rust env-init pre-publishes ALL
/// 9 of these globals BEFORE any addon loads. The Blizzard source
/// then re-declares them at file scope (lines 27/31/61/65/73/85/89/93/97
/// in `Shared/ActionButton.lua`), overwriting the simulator's
/// pre-published versions. The post-load contract is therefore a
/// two-way pin: either the simulator's pre-publish OR the addon's
/// source declaration must result in a function. Unlike the input-
/// globals batch above, this batch has NO PLAN-only globals — every
/// entry is on PLAN.md:94, so a nil reading on any of them proves the
/// gap-fill regressed AND the addon's file-scope declaration didn't
/// run.
#[test]
fn action_bar_plan_named_highlight_tracking_globals_are_functions() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for name in PLAN_NAMED_HIGHLIGHT_GLOBALS {
            let is_function = env
                .eval::<bool>(&format!(r#"return type({name}) == "function""#))
                .expect("file-scope global type probe must run cleanly");

            assert!(
                is_function,
                "Global `{name}` MUST be a function after the startup-shape harness loads \
                 `Blizzard_ActionBar`. Each entry in `PLAN_NAMED_HIGHLIGHT_GLOBALS` is declared \
                 via `function <name>(...)` syntax in `Shared/ActionButton.lua` \
                 (lines 27/31/61/65/73/85/89/93/97). A non-function reading proves either (a) the \
                 file chunk failed before reaching the declaration (load.rs's \
                 `action_bar_load_emits_no_lane_specific_lua_errors` would also fail in that case \
                 — multi-test failure mode), OR (b) Blizzard re-shaped the highlight-tracking \
                 surface — e.g. moved the global setters/getters onto an `ActionBarHighlightMixin` \
                 frame mixin instead of top-level globals, which would break every \
                 SpellBookFrame / Blizzard_PlayerSpells / Blizzard_TalentUI consumer that names \
                 these globals directly), OR (c) the simulator's gap-fill pre-publish at \
                 PLAN.md:94 regressed AND the addon's source-side declaration didn't run. Every \
                 one of these 9 globals is on the gap-fill line, so a nil reading is unambiguous \
                 — both publishers failed. Storage globals `ACTION_HIGHLIGHT_MARKS` (lua:9) and \
                 `ON_BAR_HIGHLIGHT_MARKS` (lua:10) are NOT pinned by this test (separate \
                 surface_globals task) but a regression that wiped them would be observable here \
                 indirectly: `ClearOnBarHighlightMarks()` re-binds the global; if the file-scope \
                 declaration didn't run, the re-bind would create an out-of-band global that \
                 readers of the original file-scope name wouldn't see. Got `type({name}) == \
                 \"function\"` returned false."
            );
        }
    });
}
