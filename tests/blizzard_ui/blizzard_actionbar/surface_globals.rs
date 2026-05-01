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
