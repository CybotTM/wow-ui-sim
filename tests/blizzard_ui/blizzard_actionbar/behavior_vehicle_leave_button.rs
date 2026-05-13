//! Behavior pin: clicking `MainMenuBarVehicleLeaveButton` routes through
//! the `OnClicked` mixin method, which reads `UnitOnTaxi("player")` to
//! decide between two distinct exit paths:
//!
//! - **Taxi branch** (`UnitOnTaxi == true`): calls
//!   `TaxiRequestEarlyLanding()`, then `:Disable()`,
//!   `:SetHighlightTexture(...)`, `:LockHighlight()`.
//! - **Vehicle branch** (`UnitOnTaxi == false`): calls `VehicleExit()`.
//!
//! Each branch is observable through a different state seam, and the
//! test fires both within a single harness run to pin both contracts:
//!
//! 1. **Taxi seam**: `state.player.taxi_early_landing_requested` flips
//!    `false → true` after the click. Backed by
//!    `vehicle_possession.rs:116-119` (`TaxiRequestEarlyLanding` is the
//!    only writer of the field).
//! 2. **Disable seam**: `MainMenuBarVehicleLeaveButton:IsEnabled()` flips
//!    `true → false` after the taxi click. Pins the `self:Disable()`
//!    call at lua:59.
//! 3. **Vehicle seam**: `state.player.controlling_vehicle` flips
//!    `true → false` after the vehicle click. Backed by
//!    `vehicle_possession.rs:103` (`VehicleExit` clears the field).
//!    The test deliberately *avoids* setting `in_vehicle` /
//!    `has_vehicle_ui` because either of those would make `VehicleExit`
//!    fire `UNIT_EXITED_VEHICLE` (`vehicle_possession.rs:99-110`),
//!    whose `VehicleSeatIndicatorMixin:UnloadTextures` listener
//!    (`Blizzard_UIPanels_Game/Shared/VehicleSeatIndicator.lua:117-125`)
//!    crashes on `DurabilityFrame:SetAlerts()` because Blizzard_UnitFrame
//!    is not in the harness load set. Checking only
//!    `controlling_vehicle` is a sufficient proof that `VehicleExit`
//!    was called: it is cleared unconditionally inside the function
//!    (lua line `sim.player.controlling_vehicle = false`) and there is
//!    no other Lua-facing writer in the simulator.
//!
//! ## Source contract
//!
//! `MainMenuBarVehicleLeaveButtonMixin:OnClicked`
//! (`Blizzard_ActionBar/Shared/VehicleLeaveButton.lua:54-65`):
//!
//! ```lua
//! function MainMenuBarVehicleLeaveButtonMixin:OnClicked()
//!     if UnitOnTaxi("player") then
//!         TaxiRequestEarlyLanding();
//!         self:Disable();
//!         self:SetHighlightTexture([[Interface\Buttons\CheckButtonHilight]], "ADD");
//!         self:LockHighlight();
//!     else
//!         VehicleExit();
//!     end
//! end
//! ```
//!
//! XML at `Shared/VehicleLeaveButton.xml:16` wires
//! `<OnClick method="OnClicked"/>`, so `:Click()` (the standard
//! `Frame:Click` API) resolves to `MainMenuBarVehicleLeaveButtonMixin:
//! OnClicked` after the mixin's `OnLoad` ran.
//!
//! ## Why two clicks in one test rather than two tests
//!
//! Both branches share the same harness load (which is the slow part —
//! `Blizzard_ActionBarController` pulls in the full action bar tree).
//! Splitting would double the harness cost without meaningful isolation:
//! the two seams are independent (different state fields, different
//! observable methods), and the test order is `taxi-then-vehicle`
//! because the taxi click latches a state field that the vehicle click
//! must not depend on. Order independence is asserted by the second
//! click reading fresh state fields rather than the disable flag.
//!
//! ## Why the test seeds state.player flags rather than firing an event
//!
//! `UNIT_ENTERED_VEHICLE` and `VEHICLE_UPDATE` are listened-for events
//! but the simulator has no Lua-facing mutator that flips
//! `state.player.has_vehicle_ui` true. Real WoW writes these flags
//! server-side and pushes deltas via the events. The canonical write
//! seam is the simulator's player state — which the test fixture stands
//! in for via direct state mutation. Same pattern as
//! `behavior_possess_bar_show.rs:166-184` for
//! `state.action_bar_state.possess_bar_visible`.
//!
//! ## Why the test reads `state.player` directly rather than
//! re-querying via `UnitOnTaxi("player")`
//!
//! Reading the Lua global would round-trip through
//! `vehicle_possession.rs:59-62` which reads the same backing field.
//! That would test the round trip but not specifically pin
//! `VehicleExit`'s effect (the global could return false simply because
//! the read path was untouched). Reading the field directly proves
//! `VehicleExit` *wrote* false, which is the contract under test.
//!
//! ## Observations pinned
//!
//! 1. **`MainMenuBarVehicleLeaveButton` exists after harness settle.**
//!    XML chunk at `Shared/VehicleLeaveButton.xml:4` declares the
//!    `<Button name="MainMenuBarVehicleLeaveButton" parent="MainActionBar">`.
//!    A nil reading means the XML didn't load or the parent
//!    `MainActionBar` is missing.
//!
//! 2. **Cold-state `state.player.taxi_early_landing_requested == false`,
//!    button `:IsEnabled() == true`.** Default `PlayerState` (state.rs)
//!    seeds the latch false; XML declares no `disabled="true"` so the
//!    button is enabled at construction. A truthy / disabled cold
//!    reading means another test left state dirty or the harness fired
//!    a click before the test got control.
//!
//! 3. **After seeding `state.player.on_taxi = true` and clicking, the
//!    latch flips true and the button disables.** Pins the full chain:
//!    `Click` resolves to `:OnClicked` via the XML
//!    `<OnClick method="OnClicked"/>` wire (xml:16), `OnClicked` reads
//!    `UnitOnTaxi("player")` (lua:55) — true via the seeded flag — and
//!    runs the taxi branch (lua:56-61). The latch write is at
//!    `vehicle_possession.rs:117`; the disable call is at lua:59.
//!
//! 4. **After seeding `state.player.controlling_vehicle = true` (and
//!    clearing `on_taxi`) and clicking, `controlling_vehicle` flips
//!    false.** Pins the vehicle branch: `OnClicked` reads `UnitOnTaxi
//!    == false` so the `else` arm at lua:62-64 calls `VehicleExit()`.
//!    The Rust impl at `vehicle_possession.rs:98-112` clears
//!    `controlling_vehicle` (lua:103). The event-firing path is gated
//!    on `was_in_vehicle = in_vehicle || has_vehicle_ui`
//!    (vehicle_possession.rs:101) — which stays false because the
//!    test deliberately seeds only `controlling_vehicle` — so the
//!    `UNIT_EXITED_VEHICLE` cascade into `VehicleSeatIndicator` is
//!    avoided.
//!
//! ## Regression candidates the assertions catch
//!
//! - XML `<OnClick method="OnClicked"/>` wire breaks: observation 3
//!   fails (latch stays false, button stays enabled).
//! - `OnClicked`'s `if UnitOnTaxi("player")` branch inverts: observation
//!   3 fails (taxi click hits VehicleExit instead) AND observation 4
//!   fails (vehicle click hits TaxiRequestEarlyLanding which doesn't
//!   touch the vehicle flags).
//! - `TaxiRequestEarlyLanding` regresses to a no-op: observation 3
//!   asserts the latch field directly, so the failure is precise.
//! - `self:Disable()` call dropped from the taxi branch: observation
//!   3's enabled-flip assertion fails; latch still flips so the failure
//!   is localized to the disable line, not the taxi entry.
//! - `VehicleExit` regresses to drop the `controlling_vehicle = false`
//!   write at vehicle_possession.rs:103: observation 4 fails. (The
//!   sibling clears at lua:102 / lua:104 are not pinned by this test
//!   because exercising them would require seeding `in_vehicle` or
//!   `has_vehicle_ui`, which trips the `UNIT_EXITED_VEHICLE` cascade
//!   above. Surface tests for those clears belong in a separate test
//!   that loads Blizzard_UnitFrame.)
//!
//! Note: there is no observation pinning `UNIT_EXITED_VEHICLE` event
//! emission. The leave button is itself a listener for that event, but
//! its `OnEvent` body is a catch-all `self:Update()` call (lua:25-27)
//! whose effect (`SetShown(self.isInEditMode or self:CanExitVehicle())`)
//! is identical with or without the event after VehicleExit clears the
//! flags — so the event firing is not independently observable from the
//! button. Pinning event emission belongs in a separate test that
//! parks an `RegisterEvent` listener on a fresh frame.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";
const FRAME_NAME: &str = "MainMenuBarVehicleLeaveButton";

#[test]
fn vehicle_leave_button_routes_taxi_and_vehicle_branches_through_onclicked() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let cold_button_exists: bool = env
            .eval(&format!("return _G[{FRAME_NAME:?}] ~= nil"))
            .expect("MainMenuBarVehicleLeaveButton existence probe must run cleanly");
        assert!(
            cold_button_exists,
            "After the startup-shape harness loads `{ROOT}`, \
             `_G[{FRAME_NAME:?}]` must not be nil. The button is declared \
             at `Shared/VehicleLeaveButton.xml:4` as \
             `<Button name=\"MainMenuBarVehicleLeaveButton\" \
             parent=\"MainActionBar\" parentKey=\"VehicleLeaveButton\" \
             mixin=\"MainMenuBarVehicleLeaveButtonMixin\">`. A nil reading \
             means the XML chunk didn't load (TOC walk regressed) or the \
             parent `MainActionBar` is missing so the parented child \
             couldn't attach."
        );

        let cold_taxi_latch = env.state().borrow().player.taxi_early_landing_requested;
        assert!(
            !cold_taxi_latch,
            "Cold-state `state.player.taxi_early_landing_requested` must \
             be false. The default `PlayerState` (state_types/character_\
             world.rs:209) seeds it false. `TaxiRequestEarlyLanding` \
             (vehicle_possession.rs:116-119) is the only writer in the \
             simulator. A truthy cold reading means another test left \
             state dirty or the harness fired a click before this test \
             got control."
        );

        let cold_enabled: bool = env
            .eval(&format!("return _G[{FRAME_NAME:?}]:IsEnabled()"))
            .expect("cold IsEnabled probe must run cleanly");
        assert!(
            cold_enabled,
            "Cold-state `{FRAME_NAME}:IsEnabled()` must be true. The XML \
             at `Shared/VehicleLeaveButton.xml:4` declares no \
             `disabled=\"true\"` attribute, so the Button is enabled at \
             construction. The `MainMenuBarVehicleLeaveButtonMixin:Update` \
             (lua:37-52) runs `:Enable()` when `:CanExitVehicle()` is \
             true and is silent (no explicit `:Disable()`) when false — \
             so cold state preserves the construction-time enabled flag. \
             A false reading means the harness fired a Hide/Disable path \
             before this test got control, or the XML default flipped."
        );

        env.state().borrow_mut().player.on_taxi = true;

        env.exec(&format!("_G[{FRAME_NAME:?}]:Click()"))
            .expect("first :Click() must dispatch through OnClicked cleanly");

        let post_taxi_latch = env.state().borrow().player.taxi_early_landing_requested;
        assert!(
            post_taxi_latch,
            "After seeding `state.player.on_taxi = true` and clicking, \
             `state.player.taxi_early_landing_requested` must be true. \
             Pins the full chain: `:Click()` resolves to the OnClick \
             script wired by `Shared/VehicleLeaveButton.xml:16` \
             (`<OnClick method=\"OnClicked\"/>`), which calls \
             `MainMenuBarVehicleLeaveButtonMixin:OnClicked` (lua:54). \
             OnClicked reads `UnitOnTaxi(\"player\")` (lua:55) — true via \
             the seeded flag — and runs the taxi branch lua:56 \
             `TaxiRequestEarlyLanding()`. The Rust impl at \
             vehicle_possession.rs:117 sets the latch field. A false \
             reading means `:Click` didn't reach OnClicked, OnClicked's \
             condition inverted, or `TaxiRequestEarlyLanding` regressed \
             to a no-op."
        );

        let post_taxi_enabled: bool = env
            .eval(&format!("return _G[{FRAME_NAME:?}]:IsEnabled()"))
            .expect("post-taxi IsEnabled probe must run cleanly");
        assert!(
            !post_taxi_enabled,
            "After the taxi-branch click, `{FRAME_NAME}:IsEnabled()` must \
             be false. Pinned by `self:Disable()` at lua:59 inside \
             OnClicked's taxi arm. A truthy reading means the disable \
             line was dropped — the latch flipped (so the taxi entry \
             worked) but the button stayed enabled. This catches a \
             surgical regression on the disable line specifically, \
             distinct from the entry-condition regression caught by the \
             latch assertion."
        );

        // Reset for the vehicle-branch click. Clear on_taxi (so OnClicked
        // takes the else arm) and seed ONLY `controlling_vehicle = true`
        // — `VehicleExit`'s event-firing gate at vehicle_possession.rs:101
        // checks `in_vehicle || has_vehicle_ui`, both of which stay
        // false here, so `UNIT_EXITED_VEHICLE` is NOT fired. This avoids
        // a downstream cascade where the event listener
        // `VehicleSeatIndicatorMixin:UnloadTextures` at
        // `Blizzard_UIPanels_Game/Shared/VehicleSeatIndicator.lua:122`
        // calls `DurabilityFrame:SetAlerts()`, hits a nil global because
        // Blizzard_UnitFrame isn't in the harness load set, and recurses
        // through Blizzard_ScriptErrors's OnUpdate handler. Re-enable the
        // button so post-click readers can't be confused by leftover
        // disabled state.
        {
            let mut state = env.state().borrow_mut();
            state.player.on_taxi = false;
            state.player.controlling_vehicle = true;
        }
        env.exec(&format!("_G[{FRAME_NAME:?}]:Enable()"))
            .expect("Re-enable for vehicle branch must run cleanly");

        env.exec(&format!("_G[{FRAME_NAME:?}]:Click()"))
            .expect("second :Click() must dispatch through OnClicked cleanly");

        let post_vehicle_controlling = env.state().borrow().player.controlling_vehicle;
        assert!(
            !post_vehicle_controlling,
            "After seeding `state.player.controlling_vehicle = true` \
             (with `on_taxi = false`) and clicking, \
             `state.player.controlling_vehicle` must be false. Pinned by \
             `VehicleExit` at vehicle_possession.rs:103 \
             (`sim.player.controlling_vehicle = false`). The OnClicked \
             else arm at lua:62-64 reaches VehicleExit because \
             `UnitOnTaxi(\"player\")` returns false (cleared above). A \
             truthy reading means VehicleExit dropped the \
             `controlling_vehicle` clear specifically, OnClicked's \
             condition inverted (taking the taxi branch instead), or \
             the XML `<OnClick method=\"OnClicked\"/>` wire stopped \
             dispatching."
        );
    });
    }
}
