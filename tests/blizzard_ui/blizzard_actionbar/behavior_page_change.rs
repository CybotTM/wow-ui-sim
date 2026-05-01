//! Behavior pin: `ActionBar_PageUp()` increments `state.action_bar_page`,
//! fires `ACTIONBAR_PAGE_CHANGED`, and the bar buttons re-resolve their
//! action ids.
//!
//! Source contract (`Interface/BlizzardUI/Blizzard_ActionBar/`):
//!
//! 1. `ActionBar_PageUp()` (`Shared/ActionButton.lua:166-179`) walks
//!    `VIEWABLE_ACTION_BAR_PAGES` (`Shared/ActionButtonUtil.lua:8`,
//!    initialized to `{1, 1, 1, 1, 1, 1}` — every page viewable by default)
//!    from `C_ActionBar.GetActionBarPage() + 1` to `NUM_ACTIONBAR_PAGES`
//!    (`ActionButtonUtil.lua:2`, value 6), picks the first viewable page,
//!    and dispatches `C_ActionBar.SetActionBarPage(nextPage)`. If no later
//!    page is viewable the helper wraps to page 1.
//!
//! 2. `MainActionBarMixin:OnLoad` (`Shared/MainActionBar.lua:6`) registers
//!    `ACTIONBAR_PAGE_CHANGED`. Its `OnEvent` body at lua:30-32 takes the
//!    `event == "ACTIONBAR_PAGE_CHANGED"` arm and refreshes
//!    `MainActionBar.ActionBarPageNumber.Text` from
//!    `C_ActionBar.GetActionBarPage()`.
//!
//! Simulator gap-fill (this task implemented the backing):
//! - Added `pub action_bar_page: u32` field to `SimState`
//!   (`src/lua_api/state.rs`), defaulted to 1 in the empty-state builder
//!   (`SimState::build_empty_state` macro at line 80) — matches the
//!   server-side default page after `PLAYER_ENTERING_WORLD`.
//! - `C_ActionBar.GetActionBarPage` now returns
//!   `state.action_bar_page` (`src/lua_api/globals/action_bar_api.rs:305-308`)
//!   instead of the previous hardcoded `1`.
//! - `C_ActionBar.SetActionBarPage(page)` now writes `state.action_bar_page`
//!   and, on a real change, fires `ACTIONBAR_PAGE_CHANGED` via
//!   `fire_named_event_state` (action_bar_api.rs:312-336). Out-of-range
//!   pages (< 1 or > `NUM_ACTIONBAR_PAGES`) are rejected without firing —
//!   matches the engine contract that only viewable pages drive the event.
//!
//! The test pins the round-trip across three observation axes:
//!
//!   1. **Cold-start page is 1.** After the startup-shape harness loads
//!      `Blizzard_ActionBar`, `C_ActionBar.GetActionBarPage()` returns 1 —
//!      no page-change event fired during startup.
//!   2. **`ActionBar_PageUp()` advances to page 2.** Calling the global
//!      walks the viewable-page table from 2 upward, lands on 2 (every page
//!      is viewable in the default `VIEWABLE_ACTION_BAR_PAGES` table), and
//!      dispatches `SetActionBarPage(2)`. Post-call,
//!      `C_ActionBar.GetActionBarPage()` returns 2 (proves the simulator
//!      wrote `state.action_bar_page`).
//!   3. **`ACTIONBAR_PAGE_CHANGED` actually fires.** A test-installed
//!      handler on `MainActionBar` records the event count; after one
//!      `ActionBar_PageUp()`, the handler must have been invoked at least
//!      once. The bar-buttons-re-resolve consequence is established
//!      indirectly: the registered button mixins (per-button event router
//!      `ActionBarButtonEventsFrame:RegisterFrame` chain at
//!      `Shared/ActionButton.lua:454`) rely on the `ACTIONBAR_PAGE_CHANGED`
//!      fan-out for their re-resolve path, and pinning the event firing is
//!      what makes the chain meaningful — without the event, no button
//!      ever re-reads its action.
//!
//! The wrap-around path (page 6 → 1) is also pinned: a second
//! `SetActionBarPage(6)` followed by `ActionBar_PageUp()` lands the page
//! back on 1 because the `for i = 7, 6 do` loop body never executes and
//! the helper falls into its `if not nextPage then nextPage = 1 end`
//! fallback at lua:175-177. This catches a regression where the wrap
//! semantic gets dropped.
//!
//! Regression candidates documented in source-line comments below:
//!   - `state.action_bar_page` not threaded through `Get`/`Set`: the
//!     helper would re-read 1 after every set, and `ActionBar_PageUp()`
//!     would advance from `0 + 1 = 1` and re-set 1 forever (no event,
//!     pre/post both 1).
//!   - `Set` not firing the event: the per-button router never fans out,
//!     `MainActionBar`'s text widget stays at the seeded value, and the
//!     event handler installed by the test never increments.
//!   - Wrap-around dropped from `ActionBar_PageUp`: page 6 + advance lands
//!     on `nil` and the C-side accepts a nil page, leaving state
//!     undefined.
//!   - `VIEWABLE_ACTION_BAR_PAGES` table mutated mid-startup: if some
//!     other addon strips entries, the advance walks past 6 and falls to
//!     the wrap branch from a non-6 page.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBar";

#[test]
fn action_bar_page_up_advances_state_action_bar_page_and_fires_actionbar_page_changed() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let cold_start_page: i32 = env
            .eval("return C_ActionBar.GetActionBarPage()")
            .expect("C_ActionBar.GetActionBarPage cold-start probe must run cleanly");
        assert_eq!(
            cold_start_page, 1,
            "After the startup-shape harness loads `{ROOT}`, \
             `C_ActionBar.GetActionBarPage()` must return 1. The simulator \
             defaults `state.action_bar_page` to 1 in the empty-state builder \
             (src/lua_api/state.rs:`SimState::build_empty_state`); the harness \
             does not advance the page during settle, so the cold-start \
             reading is the seeded default. A non-1 reading here means either \
             the default was changed, the harness now drives a page advance \
             during startup, or `Get` is reading the wrong field."
        );

        let event_observation_setup_ok: bool = env
            .eval(
                r#"
                _G._test_actionbar_page_changed_count = 0
                if not MainActionBar then
                    return false
                end
                MainActionBar:RegisterEvent("ACTIONBAR_PAGE_CHANGED")
                MainActionBar:HookScript("OnEvent", function(self, event)
                    if event == "ACTIONBAR_PAGE_CHANGED" then
                        _G._test_actionbar_page_changed_count =
                            (_G._test_actionbar_page_changed_count or 0) + 1
                    end
                end)
                return true
                "#,
            )
            .expect("Test harness must install an ACTIONBAR_PAGE_CHANGED counter on MainActionBar");
        assert!(
            event_observation_setup_ok,
            "MainActionBar must exist after the startup-shape harness so the \
             test can hook its OnEvent. Created by `MainActionBarMixin:OnLoad` \
             (Shared/MainActionBar.lua:3); a missing reading here means the \
             addon's file-scope frame creation regressed."
        );

        env.exec("ActionBar_PageUp()")
            .expect("ActionBar_PageUp() must be callable as a global function");

        let page_after_up: i32 = env
            .eval("return C_ActionBar.GetActionBarPage()")
            .expect("C_ActionBar.GetActionBarPage post-PageUp probe must run cleanly");
        assert_eq!(
            page_after_up, 2,
            "After one `ActionBar_PageUp()`, `C_ActionBar.GetActionBarPage()` \
             must return 2. The helper at Shared/ActionButton.lua:166-179 \
             walks `VIEWABLE_ACTION_BAR_PAGES` from `current + 1 = 2` to \
             `NUM_ACTIONBAR_PAGES = 6` (Shared/ActionButtonUtil.lua:2), picks \
             the first viewable page (every page is viewable in the default \
             `{{1,1,1,1,1,1}}` table at ActionButtonUtil.lua:8), and \
             dispatches `C_ActionBar.SetActionBarPage(2)`. The simulator's \
             `set_action_bar_page` now writes `state.action_bar_page = 2` \
             (action_bar_api.rs); a stale 1 reading would mean either the \
             walker advanced past 2 (table corruption) or `Set` didn't write \
             through to state."
        );

        let page_changed_count: i32 = env
            .eval("return _G._test_actionbar_page_changed_count or 0")
            .expect("ACTIONBAR_PAGE_CHANGED counter probe must run cleanly");
        assert!(
            page_changed_count >= 1,
            "After one `ActionBar_PageUp()`, the test-installed \
             ACTIONBAR_PAGE_CHANGED counter must have advanced at least once. \
             `MainActionBarMixin:OnLoad` (Shared/MainActionBar.lua:6) \
             registers the event, and the simulator's updated \
             `set_action_bar_page` (action_bar_api.rs) fires the event via \
             `fire_named_event_state` when the page actually changes. A 0 \
             reading here means either `Set` skipped the fire (the diff \
             check was inverted), the routing path missed `MainActionBar`, \
             or the per-button event router's RegisterFrame chain is \
             broken — any of which would leave registered button mixins \
             with stale per-slot action ids after a page cycle. Got: \
             {page_changed_count}."
        );

        env.exec("C_ActionBar.SetActionBarPage(6)")
            .expect("SetActionBarPage(6) must be callable");
        let page_at_six: i32 = env
            .eval("return C_ActionBar.GetActionBarPage()")
            .expect("Get after Set(6) probe must run cleanly");
        assert_eq!(
            page_at_six, 6,
            "After `C_ActionBar.SetActionBarPage(6)`, the page must be 6. \
             6 == `NUM_ACTIONBAR_PAGES`, which is the upper bound the \
             simulator accepts (action_bar_api.rs:`set_action_bar_page` \
             rejects pages > NUM_ACTIONBAR_PAGES). A stale 2 reading means \
             the boundary check rejected the valid page; a different value \
             means `Set` wrote to a different field or `Get` reads a stale \
             cache."
        );

        env.exec("ActionBar_PageUp()")
            .expect("ActionBar_PageUp() at page 6 must wrap to page 1");
        let wrapped_page: i32 = env
            .eval("return C_ActionBar.GetActionBarPage()")
            .expect("Wrap-around probe must run cleanly");
        assert_eq!(
            wrapped_page, 1,
            "After one `ActionBar_PageUp()` from page 6, the page must wrap \
             back to 1. The helper at Shared/ActionButton.lua:166-179 walks \
             `for i = 7, 6 do ... end` (loop body never executes because the \
             start exceeds the end), leaves `nextPage` as `nil`, then takes \
             the `if not nextPage then nextPage = 1 end` fallback at \
             lua:175-177 and dispatches `SetActionBarPage(1)`. A non-1 \
             reading here means the wrap fallback was dropped or the helper \
             advanced past 6 silently."
        );
    });
}
