//! Behavior pin: main bar button 1 reflects `state.action_bars[1]` after
//! `ACTIONBAR_SLOT_CHANGED`.
//!
//! Source contract (`Interface/BlizzardUI/Blizzard_ActionBar/`):
//!
//! 1. `MainActionBarMixin:OnLoad` (`Shared/MainActionBar.lua:3`) calls
//!    `ActionBarMixin:ActionBar_OnLoad` (`Shared/ActionBar.lua:3`), which at
//!    `ActionBar.lua:31` runs
//!    `CreateFrame("CheckButton", "ActionButton"..i, buttonContainer,
//!     self.buttonTemplate, i)` for `i = 1..numButtons`. The `i` argument is
//!    the button's `user_id` — `calculate_action()` at
//!    `src/lua_api/frame/methods/button_anchor_hierarchy/buttons.rs:257`
//!    returns `widget.user_id` when non-zero, so `ActionButton1.action == 1`
//!    after `ActionBarActionButtonMixin:OnLoad` (`Shared/ActionButton.lua:444`)
//!    invokes `:UpdateAction()` (lua:529) which writes `self.action = action`.
//!
//! 2. The button registers itself with the per-button event router via
//!    `ActionBarButtonEventsFrame:RegisterFrame(self)` at lua:454. The router's
//!    `OnEvent` (`ActionBarButtonEventsFrameMixin:OnEvent`, lua:220) fans
//!    `ACTIONBAR_SLOT_CHANGED` to every registered button's own `OnEvent`.
//!
//! 3. The button's `OnEvent` (lua:966-976) gates on
//!    `arg1 == 0 or arg1 == tonumber(self.action)` and calls
//!    `:UpdateAction(true)` on match. `:UpdateAction(force)` (lua:529) →
//!    `:Update()` (lua:555) reads
//!    `texture = C_ActionBar.GetActionTexture(action)` and at lua:617-619 calls
//!    `icon:SetTexture(texture)` if non-nil.
//!
//! 4. `:HasAction()` (lua:669-671) returns
//!    `C_ActionBar.HasAction(self.action)`.
//!
//! Simulator wiring (`src/lua_api/globals/action_bar_api.rs`):
//! - `C_ActionBar.HasAction(slot)` returns
//!   `state.action_bars.contains_key(&slot)` (line 544).
//! - `C_ActionBar.GetActionTexture(slot)` resolves the spell via SPELL_DB and
//!   returns the texture path string (line 554).
//!
//! Pre-seeded state caveat: `SimState::default()` calls
//! `seed_default_game_state()` (`src/lua_api/state.rs:3215-3216`), which in
//! turn invokes `default_action_bars()` (`src/lua_api/game_data.rs:1111-1127`)
//! and stamps slots 1-12 with the Protection Paladin rotation BEFORE any
//! addon loads. To pin the empty→populated transition that the PLAN line
//! names, the test clears `state.action_bars` post-startup and fires
//! `ACTIONBAR_SLOT_CHANGED` arg=0 (the broadcast-refresh form — every
//! registered button's OnEvent gate at lua:973 takes the `arg1 == 0` short-
//! circuit and re-runs `:UpdateAction(true)` against the now-empty map).
//!
//! Spell 853 (Hammer of Justice — paladin) is the standard test spell with a
//! valid icon, matching the reference pattern in `tests/action_bar_drag.rs:188`.
//!
//! The test pins five observations across the cleared / post-event boundary:
//!   1. `ActionButton1.action == 1` after OnLoad (proves the user_id binding).
//!   2. After clearing action_bars + arg=0 broadcast,
//!      `ActionButton1:HasAction()` is `false` (proves the broadcast path
//!      reaches button 1 and that `:HasAction` re-reads live state).
//!   3. After seeding `state.action_bars[1] = 853` and firing
//!      `ACTIONBAR_SLOT_CHANGED` arg=1, `:HasAction()` is `true` (proves the
//!      slot-targeted gate at lua:973 also reaches button 1).
//!   4. The icon texture resolves to a non-empty string (proves
//!      `C_ActionBar.GetActionTexture` round-tripped through SPELL_DB).
//!
//! Regression candidates:
//!   - `ActionBarButtonEventsFrame:RegisterFrame` not called → event never
//!     reaches the button → `:HasAction()` stays at its prior reading after
//!     either firing.
//!   - The `arg1 == 0 or arg1 == tonumber(self.action)` gate inverted →
//!     button ignores its own slot or the broadcast.
//!   - `:UpdateAction(true)` not called or `force` semantics dropped →
//!     `self.action` not refreshed, icon stale.
//!   - `C_ActionBar.HasAction` reading the wrong field → `:HasAction()`
//!     mismatches the seeded slot.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use rilua::Val;

const ROOT: &str = "Blizzard_ActionBar";
const SLOT: u32 = 1;
const SPELL_ID: u32 = 853;

#[test]
fn main_bar_button_1_reflects_action_bars_slot_one_after_actionbar_slot_changed() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let action_field: i32 = env
            .eval("return ActionButton1.action")
            .expect("ActionButton1.action probe must run cleanly");
        assert_eq!(
            action_field, 1,
            "ActionButton1.action must be 1 after MainActionBar:OnLoad. \
             `MainActionBarMixin:OnLoad` (Shared/MainActionBar.lua:3) calls \
             `ActionBar_OnLoad` (Shared/ActionBar.lua:3), which creates each \
             button at lua:31 via `CreateFrame(..., i)` with i=1..12 — `i` is \
             the user_id. `:UpdateAction()` (Shared/ActionButton.lua:529) \
             reads the user_id via `calculate_action()` and writes it into \
             `self.action`. A regression that dropped the user_id arg from \
             CreateFrame, or that changed `calculate_action()` to fall through \
             to the `action` attribute (default 1) before checking user_id, \
             would land here."
        );

        env.state().borrow_mut().action_bars.clear();
        env.fire_event_with_args("ACTIONBAR_SLOT_CHANGED", &[Val::Num(0.0)])
            .expect("broadcast ACTIONBAR_SLOT_CHANGED arg=0 must dispatch cleanly");

        let pre_has_action: bool = env
            .eval("return ActionButton1:HasAction()")
            .expect("ActionButton1:HasAction() pre-probe must run cleanly");
        assert!(
            !pre_has_action,
            "After clearing state.action_bars and firing arg=0 broadcast, \
             ActionButton1:HasAction() must be false. `:HasAction()` \
             (Shared/ActionButton.lua:669-671) returns \
             `C_ActionBar.HasAction(self.action)`, which the simulator backs \
             with `state.action_bars.contains_key(&slot)` at \
             src/lua_api/globals/action_bar_api.rs:544. SimState::default() \
             pre-seeds slots 1-12 with the Prot Paladin rotation \
             (game_data.rs:1111-1127) — clearing the map before the seeded \
             slots are read should drop slot 1. A `true` reading here means \
             either the clear didn't take, the contains_key sense flipped, or \
             :HasAction() cached `self.hasAction` on a prior read instead of \
             querying live state."
        );

        env.state().borrow_mut().action_bars.insert(SLOT, SPELL_ID);
        env.fire_event_with_args("ACTIONBAR_SLOT_CHANGED", &[Val::Num(SLOT as f64)])
            .expect("targeted ACTIONBAR_SLOT_CHANGED arg=1 must dispatch cleanly");

        let post_has_action: bool = env
            .eval("return ActionButton1:HasAction()")
            .expect("ActionButton1:HasAction() post-probe must run cleanly");
        assert!(
            post_has_action,
            "After seeding state.action_bars[1] = {SPELL_ID} and firing \
             ACTIONBAR_SLOT_CHANGED arg=1, ActionButton1:HasAction() must be \
             true. The event walks ActionBarButtonEventsFrame's listener list \
             (router OnEvent at Shared/ActionButton.lua:220) and dispatches \
             to each registered button's own OnEvent at lua:966-976, which \
             gates on `arg1 == 0 or arg1 == tonumber(self.action)` (lua:973) \
             and calls `:UpdateAction(true)` on match. A regression that \
             skipped RegisterFrame at lua:454, that inverted the arg1 gate, \
             or that dropped the force-flag plumb-through `:UpdateAction → \
             :Update` would leave HasAction reading the still-empty cleared \
             state."
        );

        let texture: String = env
            .eval("return tostring(ActionButton1.icon:GetTexture() or \"\")")
            .expect("icon:GetTexture() probe must run cleanly");
        assert!(
            !texture.is_empty(),
            "After ACTIONBAR_SLOT_CHANGED, ActionButton1.icon:GetTexture() \
             must return a non-empty string. `:Update()` \
             (Shared/ActionButton.lua:555) reads `texture = \
             C_ActionBar.GetActionTexture(action)` and at lua:617-619 calls \
             `icon:SetTexture(texture)` when non-nil. The simulator's \
             `C_ActionBar.GetActionTexture` (src/lua_api/globals/\
             action_bar_api.rs:554) resolves the spell via SPELL_DB and \
             returns a texture path. Spell 853 (Hammer of Justice) is in \
             SPELL_DB with a valid icon_file_data_id; an empty/nil reading \
             would mean either the SetTexture call was skipped, the texture \
             lookup returned nil, or `:Update()` short-circuited before \
             reaching the icon path. Got: `{texture}`."
        );
    });
}
