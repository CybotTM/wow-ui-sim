//! Behavior pin: `ActionButtonDown(1)` followed by `ActionButtonUp(1)` routes
//! through `TryUseActionButton` and lands a `UseAction(1)` semantic — i.e. the
//! cast pipeline reaches `state.casting` after the round trip.
//!
//! Source contract (`Interface/BlizzardUI/Blizzard_ActionBar/`):
//!
//! 1. `ActionButtonDown(id)` (`Shared/ActionButton.lua:136-149`) gates on
//!    `CheckPetActionButtonEvent(id, true)`, looks up the button via
//!    `GetActionButtonForID(id)` (lua:97-107, falls back to
//!    `_G["ActionButton"..id]` when `OverrideActionBar` is not shown),
//!    transitions the button state from `NORMAL` to `PUSHED`, and dispatches
//!    `TryUseActionButton(button, true)` — the trailing `true` is
//!    `checkingFromDown`.
//!
//! 2. `ActionButtonUp(id)` (`Shared/ActionButton.lua:151-164`) is the
//!    symmetric release path: pet-battle gate, button lookup, `PUSHED→NORMAL`
//!    transition, then `TryUseActionButton(button, false)`.
//!
//! 3. `TryUseActionButton(self, checkingFromDown)` (lua:109-120) calls
//!    `SecureActionButton_OnClick(self, "LeftButton", checkingFromDown,
//!    isKeyPress=true, isSecureAction=true)`.
//!
//! 4. `SecureActionButton_OnClick` (`Blizzard_FrameXML/SecureTemplates.lua:797`)
//!    computes `useOnKeyDown` from `SecureActionButton_ShouldUseOnKeyDown(self)`
//!    (which reads the button's `useOnKeyDown` attribute, falling back to
//!    `GetCVarBool("ActionButtonUseKeyDown")` — default `'1'` per
//!    `src/cvars.yaml:3`), then takes the
//!    `clickAction = (down and useOnKeyDown) or (not down and not useOnKeyDown)`
//!    branch. With `useOnKeyDown=true` and the press path (`down=true`),
//!    `clickAction=true` and `OnActionButtonClick` fires; the release path
//!    (`down=false`) lands `clickAction=false` and the function returns false
//!    (`releasePressAndHoldAction` is also false because
//!    `ActionButtonUseKeyHeldSpell` defaults to `'0'` per `cvars.yaml:4`).
//!
//! 5. `OnActionButtonClick` (`SecureTemplates.lua:754`) → `PerformAction`
//!    (lua:725-752) → `SECURE_ACTIONS.action` (lua:334-352) reads
//!    `action = self:CalculateAction(button)` then dispatches
//!    `UseAction(action, unit, button, isKeyPress)`.
//!
//! 6. `ActionBarActionButtonMixin:OnLoad` (`Shared/ActionButton.lua:444-468`)
//!    sets `self:SetAttribute("type", "action")` at lua:450 (which selects the
//!    `SECURE_ACTIONS.action` handler in `PerformAction`), and binds the
//!    button's `:CalculateAction()` method via the mixin's `OnLoad` chain. The
//!    main bar's slot/page resolver returns the button's `user_id` (i.e. the
//!    `i` argument from `CreateFrame(..., i)` at `Shared/ActionBar.lua:31`),
//!    so `ActionButton1:CalculateAction("LeftButton")` returns 1.
//!
//! Simulator wiring (`src/lua_api/globals/combat_verbs.rs`):
//! - `UseAction(slot)` (`use_action`, line 404) reads
//!   `state.action_bars[slot]` and calls `execute_spell_by_id`.
//! - `execute_spell_by_id` (line 183) for a cast-time spell calls `start_cast`
//!   which populates `state.casting: Option<CastingState>` with `spell_id`,
//!   `spell_name`, `icon_path`, `start_time`, `end_time`, `cast_id` — the
//!   field defined at `src/lua_api/state.rs:1459` and modeled by
//!   `CastingState` at `src/lua_api/game_data.rs:80`.
//! - `UnitCastingInfo("player")` reads `state.casting` and returns the cast
//!   shape; nil when `state.casting` is `None`.
//!
//! Spell choice: Flash of Light (spell id 19750). `spell_cast_time(19750)`
//! returns 1500ms (`src/lua_api/globals/spell_api.rs:14`), so the
//! `cast_time_ms > 0` branch in `execute_spell_by_id` runs `start_cast`
//! (populating `state.casting`) instead of the instant
//! `start_instant_spell_cooldowns + apply_spell_to_target` branch which would
//! leave `state.casting` untouched. Flash of Light is `Helpful`
//! (`spell_target_type` returns Helpful), so `spell_can_execute_now` returns
//! true regardless of `state.current_target` — no target seeding required.
//! Default seeded `state.action_bars[1]` is already Flash of Light per
//! `default_action_bars` (`src/lua_api/game_data.rs:1113`); the test re-seeds
//! it explicitly to defend against future changes to the seeded rotation.
//!
//! The test pins five observations across the press → release boundary:
//!   1. **Cold-start: not casting, button NORMAL.** After the harness loads
//!      `Blizzard_ActionBar`, `UnitCastingInfo("player")` is nil and
//!      `ActionButton1:GetButtonState()` is `"NORMAL"`. The harness does not
//!      drive a cast during settle.
//!   2. **`ActionButtonDown(1)` transitions the button to `PUSHED`.** Proves
//!      the `:SetButtonState("PUSHED")` at `Shared/ActionButton.lua:144` was
//!      reached — the Blizzard Lua redefinition at lua:136 took precedence
//!      over the simulator's Rust-registered `ActionButtonDown` global at
//!      `combat_verbs.rs:522`.
//!   3. **`ActionButtonDown(1)` populates `state.casting` with spell 19750.**
//!      Proves the chain reached `UseAction(1)` and that the cast-time
//!      branch of `execute_spell_by_id` ran. `UnitCastingInfo("player")`
//!      returns `select(1, ...) == "Flash of Light"` — pins both the
//!      end-to-end routing AND the spell-id round-trip through SPELL_DB.
//!   4. **`ActionButtonUp(1)` transitions the button back to `NORMAL`.**
//!      Proves the symmetric release path at lua:158-160 ran the
//!      `PUSHED→NORMAL` transition.
//!   5. **`ActionButtonUp(1)` does NOT cancel the cast.** With
//!      `ActionButtonUseKeyDown=1` and `ActionButtonUseKeyHeldSpell=0`, the
//!      `Up` call's `SecureActionButton_OnClick` lands
//!      `clickAction=false` and `releasePressAndHoldAction=false` and returns
//!      without firing — so the cast started by `Down` continues. A regression
//!      that fired `ActionRelease` on every Up, or that wiped `state.casting`
//!      on button-state transition, would land here.
//!
//! Regression candidates documented in source-line comments below:
//!   - Blizzard's `function ActionButtonDown(id)` redefinition not running:
//!     the Rust-registered `action_button_down` (combat_verbs.rs:457) would
//!     still fire — it also calls `dispatch_action_for_button` →
//!     `execute_spell_by_id`, so the `state.casting` observation would still
//!     pass, but the `:GetButtonState()` transition flow proves the Blizzard
//!     Lua chain ran (the Rust path uses `set_button_state` directly without
//!     calling the Lua `:SetButtonState` mixin override).
//!   - `clickAction` branch inverted in `SecureActionButton_OnClick`: both
//!     Down and Up would either fire or not fire, breaking observation 5.
//!   - `SECURE_ACTIONS.action` not selected: `type` attribute defaulting to
//!     something other than `"action"` would skip `UseAction` entirely.
//!   - `:CalculateAction()` returning `nil`: the `if action then` gate at
//!     `SecureTemplates.lua:337` would short-circuit, leaving `state.casting`
//!     None.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use rilua::Val;

const ROOT: &str = "Blizzard_ActionBar";
const SLOT: u32 = 1;
const FLASH_OF_LIGHT_SPELL_ID: u32 = 19750;
const FLASH_OF_LIGHT_NAME: &str = "Flash of Light";

#[test]
fn action_button_down_routes_through_try_use_action_button_and_populates_state_casting() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.state()
            .borrow_mut()
            .action_bars
            .insert(SLOT, FLASH_OF_LIGHT_SPELL_ID);
        env.fire_event_with_args("ACTIONBAR_SLOT_CHANGED", &[Val::Num(SLOT as f64)])
            .expect("ACTIONBAR_SLOT_CHANGED arg=1 must dispatch cleanly");

        let cold_casting: bool = env
            .eval("return UnitCastingInfo('player') ~= nil")
            .expect("UnitCastingInfo cold-start probe must run cleanly");
        assert!(
            !cold_casting,
            "After the startup-shape harness loads `{ROOT}`, \
             `UnitCastingInfo('player')` must be nil. The harness drives \
             startup events but does not initiate a cast — `state.casting` \
             starts as `None` per `SimState::build_empty_state`. A non-nil \
             reading here means either the harness now triggers a cast \
             during settle, or `UnitCastingInfo` is reading the wrong field."
        );

        let cold_button_state: String = env
            .eval("return ActionButton1:GetButtonState()")
            .expect("ActionButton1:GetButtonState cold-start probe must run cleanly");
        assert_eq!(
            cold_button_state, "NORMAL",
            "After OnLoad, ActionButton1's button state must be NORMAL. \
             `ActionBarActionButtonMixin:OnLoad` (Shared/ActionButton.lua:444) \
             does not press the button; it just registers the slot binding. \
             A `PUSHED` reading here would mean OnLoad accidentally invoked \
             a press path, or that a prior test left the button stuck."
        );

        env.exec("ActionButtonDown(1)")
            .expect("ActionButtonDown(1) must be callable as a global function");

        let pressed_button_state: String = env
            .eval("return ActionButton1:GetButtonState()")
            .expect("ActionButton1:GetButtonState post-Down probe must run cleanly");
        assert_eq!(
            pressed_button_state, "PUSHED",
            "After `ActionButtonDown(1)`, the button state must be PUSHED. \
             The Blizzard global at Shared/ActionButton.lua:136 looks up \
             `ActionButton1` via `GetActionButtonForID(1)` and at lua:144 \
             dispatches `button:SetButtonState(\"PUSHED\")` when the button \
             is currently NORMAL. A NORMAL reading means either the global \
             redefinition didn't replace the simulator's hot-Rust \
             `action_button_down` (combat_verbs.rs:457), the lua:143 NORMAL \
             gate failed, or `:SetButtonState` mutated something other than \
             the queryable button state."
        );

        let casting_after_down: bool = env
            .eval("return UnitCastingInfo('player') ~= nil")
            .expect("UnitCastingInfo post-Down probe must run cleanly");
        assert!(
            casting_after_down,
            "After `ActionButtonDown(1)`, `UnitCastingInfo('player')` must \
             return non-nil. The chain is `ActionButtonDown(1)` → \
             `TryUseActionButton(button, true)` (Shared/ActionButton.lua:147) \
             → `SecureActionButton_OnClick(button, \"LeftButton\", true, \
             true, true)` (SecureTemplates.lua:797) → `OnActionButtonClick` \
             (lua:754) → `PerformAction` → `SECURE_ACTIONS.action` (lua:334) \
             → `UseAction(action, ...)`. The simulator's `UseAction` \
             (combat_verbs.rs:404) reads `state.action_bars[1] = 19750` and \
             calls `execute_spell_by_id(state, 19750)`; with \
             `spell_cast_time(19750) = 1500ms` (spell_api.rs:14), the \
             `cast_time_ms > 0` branch at combat_verbs.rs:204 calls \
             `start_cast` which populates `state.casting`. A nil reading \
             means the chain broke somewhere between `ActionButtonDown` and \
             `start_cast` — likely `clickAction` evaluating false on Down \
             (would mean `ActionButtonUseKeyDown` cvar is no longer `'1'`), \
             `:CalculateAction` returning nil, or the `type` attribute not \
             selecting `SECURE_ACTIONS.action`."
        );

        let cast_spell_name: String = env
            .eval("return select(1, UnitCastingInfo('player'))")
            .expect("UnitCastingInfo spell-name probe must run cleanly");
        assert_eq!(
            cast_spell_name, FLASH_OF_LIGHT_NAME,
            "The cast spell name must be `{FLASH_OF_LIGHT_NAME}` — the \
             round-trip from `state.action_bars[{SLOT}] = {FLASH_OF_LIGHT_SPELL_ID}` \
             through `UseAction({SLOT})` must resolve to spell id \
             {FLASH_OF_LIGHT_SPELL_ID} via SPELL_DB and store the spell name \
             in `state.casting.spell_name` (CastingState at game_data.rs:80). \
             A different name means either `UseAction` resolved the wrong \
             slot, or `start_cast` wrote a stale name from an earlier cast."
        );

        env.exec("ActionButtonUp(1)")
            .expect("ActionButtonUp(1) must be callable as a global function");

        let released_button_state: String = env
            .eval("return ActionButton1:GetButtonState()")
            .expect("ActionButton1:GetButtonState post-Up probe must run cleanly");
        assert_eq!(
            released_button_state, "NORMAL",
            "After `ActionButtonUp(1)`, the button state must be NORMAL. \
             `ActionButtonUp` (Shared/ActionButton.lua:151) at lua:158-160 \
             checks `:GetButtonState() == \"PUSHED\"` and dispatches \
             `:SetButtonState(\"NORMAL\")`. A PUSHED reading means the Up \
             redefinition didn't run, the gate at lua:158 was inverted, or \
             the prior Down didn't actually transition to PUSHED."
        );

        let still_casting: bool = env
            .eval("return UnitCastingInfo('player') ~= nil")
            .expect("UnitCastingInfo post-Up probe must run cleanly");
        assert!(
            still_casting,
            "After `ActionButtonUp(1)`, the cast started by Down must still \
             be in flight. With `ActionButtonUseKeyDown='1'` (cvars.yaml:3) \
             and `ActionButtonUseKeyHeldSpell='0'` (cvars.yaml:4), the Up \
             call lands `clickAction = (false and true) or (true and false) = \
             false` and `releasePressAndHoldAction = true and (nil or false) \
             = false` in `SecureActionButton_OnClick` \
             (SecureTemplates.lua:806-807) and returns without firing. So \
             `state.casting` remains populated. A nil reading here means \
             either the Up path fired `actionrelease` and wiped \
             `state.casting`, or the button-state transition wiped the cast \
             as a side effect — both regressions where the Up handler over- \
             reaches into the cast pipeline."
        );
    });
    }
}
