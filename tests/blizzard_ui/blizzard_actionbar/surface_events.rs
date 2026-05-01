//! Event registration surface for the `Blizzard_ActionBar` lane.
//!
//! PLAN.md task: pin that `ActionBarButtonEventsFrame` registers
//! `PLAYER_ENTERING_WORLD`, `ACTIONBAR_SLOT_CHANGED`, `UPDATE_BINDINGS`,
//! `GAME_PAD_ACTIVE_CHANGED`, `UPDATE_SHAPESHIFT_FORM`,
//! `ACTIONBAR_UPDATE_COOLDOWN`, `PET_BAR_UPDATE` after OnLoad.
//!
//! **Spec/source mismatch — PLAN under-counts the registrations.** The
//! mixin OnLoad at `Shared/ActionButton.lua:203-218` registers TEN
//! events, not seven:
//!
//! ```lua
//! function ActionBarButtonEventsFrameMixin:OnLoad()
//!     self.frames = {};
//!     self:RegisterEvent("PLAYER_ENTERING_WORLD");        -- PLAN
//!     self:RegisterEvent("ACTIONBAR_SLOT_CHANGED");       -- PLAN
//!     self:RegisterEvent("UPDATE_BINDINGS");              -- PLAN
//!     self:RegisterEvent("GAME_PAD_ACTIVE_CHANGED");      -- PLAN
//!     self:RegisterEvent("UPDATE_SHAPESHIFT_FORM");       -- PLAN
//!     self:RegisterEvent("ACTIONBAR_UPDATE_COOLDOWN");    -- PLAN
//!     self:RegisterEvent("PET_BAR_UPDATE");               -- PLAN
//!     self:RegisterUnitEvent("UNIT_FLAGS", "pet");        -- ★ source-only
//!     self:RegisterUnitEvent("UNIT_AURA", "pet");         -- ★ source-only
//!     self:RegisterEvent("PLAYER_MOUNT_DISPLAY_CHANGED"); -- ★ source-only
//!     ...
//! end
//! ```
//!
//! The XML at `Shared/ActionButtonComponentTemplate.xml:89-94` declares
//! the frame as `<Frame name="ActionBarButtonEventsFrame"
//! mixin="ActionBarButtonEventsDerivedFrameMixin">` with
//! `<OnLoad method="OnLoad"/>` and `<OnEvent method="OnEvent"/>`. The
//! derived mixin at `Shared/ActionButton.lua:1443`
//! (`ActionBarButtonEventsDerivedFrameMixin = CreateFromMixins(
//! ActionBarButtonEventsFrameMixin)`) shallow-copies `OnLoad` from the
//! base, so the derived OnLoad is the same body — same 10
//! registrations.
//!
//! Note: `RegisterUnitEvent(event, ...)` and `RegisterEvent(event)`
//! both add the event to the frame's `registered_events` set in the
//! simulator (`src/lua_api/frame/methods/text_attribute_event/events.rs`
//! line 51-70 vs 41-49 — same `registered_events.insert(event)`). So
//! `IsEventRegistered("UNIT_FLAGS")` returns true after the
//! `RegisterUnitEvent` call, the same way it would for a plain
//! `RegisterEvent`. The unit-filter ("pet") is intentionally ignored
//! for the registration set and only matters at dispatch-filter time.
//!
//! Test pins all ten registrations + OnEvent script + handler global,
//! split as:
//!
//! - **PLAN-named events (7)** in `PLAN_NAMED_EVENTS` — the contract the
//!   PLAN line names verbatim.
//!
//! - **Source-additional events (3)** in `SOURCE_ADDITIONAL_EVENTS` —
//!   `UNIT_FLAGS`, `UNIT_AURA` (registered via RegisterUnitEvent with
//!   "pet" filter), and `PLAYER_MOUNT_DISPLAY_CHANGED` (the trailing
//!   plain RegisterEvent the PLAN list omits). A `false` reading on any
//!   would prove either OnLoad's RegisterUnitEvent path no longer
//!   marks events as registered (regression in
//!   `src/lua_api/frame/methods/text_attribute_event/events.rs:62`'s
//!   `registered_events.insert(event)`) or Blizzard removed those three
//!   registrations from the mixin OnLoad. Either way, the spec needs
//!   updating because dispatch arms in `OnEvent` (lua:220-225 — the
//!   event-fanout to per-button frames) would lose those events.
//!
//! - **OnEvent script + handler-global presence** so a regression that
//!   drops the script wiring or removes the mixin's `OnEvent` method
//!   surfaces with a clear cause.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBar";
const FRAME_NAME: &str = "ActionBarButtonEventsFrame";
const FRAME_XML_SITE: &str = "Shared/ActionButtonComponentTemplate.xml:89";
const ONLOAD_LUA_SITE: &str = "Shared/ActionButton.lua:203";
const DERIVED_MIXIN_LUA_SITE: &str = "Shared/ActionButton.lua:1443";

/// Events the PLAN line names verbatim. All seven are registered via
/// `RegisterEvent` at `Shared/ActionButton.lua:205-211`, in this exact
/// order, as the first seven statements after `self.frames = {}`.
const PLAN_NAMED_EVENTS: &[&str] = &[
    "PLAYER_ENTERING_WORLD",
    "ACTIONBAR_SLOT_CHANGED",
    "UPDATE_BINDINGS",
    "GAME_PAD_ACTIVE_CHANGED",
    "UPDATE_SHAPESHIFT_FORM",
    "ACTIONBAR_UPDATE_COOLDOWN",
    "PET_BAR_UPDATE",
];

/// Events the source registers but PLAN omits. `UNIT_FLAGS` and
/// `UNIT_AURA` are registered via `RegisterUnitEvent(event, "pet")` at
/// `Shared/ActionButton.lua:212-213` — the unit filter is ignored for
/// registration-set membership, so `IsEventRegistered` returns true.
/// `PLAYER_MOUNT_DISPLAY_CHANGED` is a plain `RegisterEvent` at line
/// 214, after the unit-event pair. Pinned here so a regression that
/// drops any of them surfaces with a meaningful failure rather than
/// silently breaking the mixin's `OnEvent` fan-out at lua:220-225.
const SOURCE_ADDITIONAL_EVENTS: &[&str] =
    &["UNIT_FLAGS", "UNIT_AURA", "PLAYER_MOUNT_DISPLAY_CHANGED"];

/// Pin `ActionBarButtonEventsFrame`'s post-OnLoad event-registration
/// surface together with the OnEvent script wiring and handler-global
/// presence.
///
/// Twelve assertions split into:
///
/// **Frame existence (1):** `_G.ActionBarButtonEventsFrame` is a
/// table — the XML at `Shared/ActionButtonComponentTemplate.xml:89`
/// resolved its `name=` token and registered the frame in `_G`.
///
/// **PLAN-named registrations (7):** loop `PLAN_NAMED_EVENTS` and pin
/// `IsEventRegistered(event) == true` for each.
///
/// **Source-additional registrations (3):** loop
/// `SOURCE_ADDITIONAL_EVENTS` and pin the same — these would slip past
/// a PLAN-only test even though they're part of the OnLoad contract.
///
/// **OnEvent script + handler global (1 each):** the XML wires
/// `<OnEvent method="OnEvent"/>` against the mixin, and the derived
/// mixin's `OnEvent` is the inherited base body at lua:220.
#[test]
fn action_bar_button_events_frame_registers_plan_and_source_additional_events_after_onload() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let frame_type: String = env
            .eval(&format!("return type(_G[{FRAME_NAME:?}])"))
            .expect("ActionBarButtonEventsFrame global probe must run cleanly");

        assert_eq!(
            frame_type, "table",
            "Expected `_G[{FRAME_NAME:?}]` to be a table after `{ROOT}` loads, got \
             `{frame_type}`. The XML at `{FRAME_XML_SITE}` declares this frame as \
             `<Frame name=\"ActionBarButtonEventsFrame\" \
             mixin=\"ActionBarButtonEventsDerivedFrameMixin\">`, which resolves the name \
             at XML-parse time and registers the frame in `_G`. A nil reading means \
             either the XML chunk failed to execute, the name= attribute changed, or the \
             frame was removed. Every event-registration assertion below depends on this \
             frame existing."
        );

        for event in PLAN_NAMED_EVENTS {
            let registered: bool = env
                .eval(&format!(
                    "return _G[{FRAME_NAME:?}]:IsEventRegistered({event:?})"
                ))
                .unwrap_or_else(|err| {
                    panic!("`{FRAME_NAME}:IsEventRegistered({event:?})` raised: {err}")
                });

            assert!(
                registered,
                "Expected `{FRAME_NAME}:IsEventRegistered({event:?})` to be true after \
                 `{ROOT}` loads. The base mixin OnLoad at `{ONLOAD_LUA_SITE}` declares \
                 `function ActionBarButtonEventsFrameMixin:OnLoad()` and registers \
                 `{event}` via `self:RegisterEvent({event:?})` (one of the first seven \
                 statements after `self.frames = {{}}`). The XML at `{FRAME_XML_SITE}` \
                 wires `<OnLoad method=\"OnLoad\"/>`, and the derived mixin at \
                 `{DERIVED_MIXIN_LUA_SITE}` (`CreateFromMixins(ActionBarButtonEventsFrameMixin)`) \
                 shallow-copies `OnLoad` so the derived body is the same. A false \
                 reading means either OnLoad did not run (regression in the loader's \
                 method-style OnLoad dispatch — `<OnLoad method=\"OnLoad\"/>` resolves \
                 against the mixin's `OnLoad` field copied onto the frame at \
                 `src/lua_api/env_init/shared_bootstrap.lua`'s Mixin impl) or the \
                 RegisterEvent call was removed from the mixin OnLoad. Without `{event}` \
                 registered, the OnEvent fan-out at lua:220-225 — \
                 `for k, frame in pairs(self.frames) do frame:OnEvent(event, ...) end` \
                 — would never fire `{event}` to the per-button frames, breaking every \
                 ActionButton's reaction to that event."
            );
        }

        for event in SOURCE_ADDITIONAL_EVENTS {
            let registered: bool = env
                .eval(&format!(
                    "return _G[{FRAME_NAME:?}]:IsEventRegistered({event:?})"
                ))
                .unwrap_or_else(|err| {
                    panic!("`{FRAME_NAME}:IsEventRegistered({event:?})` raised: {err}")
                });

            assert!(
                registered,
                "Expected `{FRAME_NAME}:IsEventRegistered({event:?})` to be true after \
                 `{ROOT}` loads. PLAN omits this event, but the source registers it as \
                 part of the same OnLoad body at `{ONLOAD_LUA_SITE}`. Specifically: \
                 `UNIT_FLAGS` and `UNIT_AURA` are registered at lua:212-213 via \
                 `self:RegisterUnitEvent(event, \"pet\")`, and `PLAYER_MOUNT_DISPLAY_CHANGED` \
                 at lua:214 via plain `self:RegisterEvent(...)`. The simulator's \
                 `register_unit_event` and `register_event` impls in \
                 `src/lua_api/frame/methods/text_attribute_event/events.rs` both insert \
                 into the frame's `registered_events` set, so `IsEventRegistered` \
                 returns true for both modes. A false reading means either OnLoad \
                 regressed (loader did not invoke method-style OnLoad), the \
                 `RegisterUnitEvent` registration path stopped marking events as \
                 registered (regression in events.rs:62), or Blizzard removed those \
                 three registrations from the mixin OnLoad. Spec needs updating in any \
                 case — the OnEvent dispatch at lua:220-225 would no longer fan `{event}` \
                 out to the per-button frames."
            );
        }

        let onevent_script: String = env
            .eval(&format!(
                "return type(_G[{FRAME_NAME:?}]:GetScript(\"OnEvent\"))"
            ))
            .expect("`GetScript(\"OnEvent\")` must run cleanly on ActionBarButtonEventsFrame");

        assert_eq!(
            onevent_script, "function",
            "Expected `{FRAME_NAME}:GetScript(\"OnEvent\")` to be a function after \
             `{ROOT}` loads, got `{onevent_script}`. The XML at `{FRAME_XML_SITE}` line \
             92 wires `<OnEvent method=\"OnEvent\"/>` against the mixin — the loader \
             resolves this by reading the mixin's `OnEvent` field (copied onto the \
             frame via the codegen path at `src/loader/xml_frame_codegen.rs:155-173` \
             plus the shared `Mixin(object, ...)` impl). The base mixin at \
             `Shared/ActionButton.lua:220` declares \
             `function ActionBarButtonEventsFrameMixin:OnEvent(event, ...)` whose body \
             fans the event out to every per-button frame in `self.frames`. Without an \
             OnEvent script wired, every registered event above would fire but no \
             dispatch would run, breaking the entire per-button event-fanout that \
             ActionButton OnLoad relies on (the per-button OnLoad at lua:459 calls \
             `ActionBarButtonEventsFrame:RegisterFrame(self)` to subscribe). A nil \
             reading means either the XML dropped the `<OnEvent>` element, the loader \
             stopped wiring `<OnEvent method=...>` against mixin methods, or the mixin \
             did not provide an `OnEvent` field at script-attach time."
        );
    });
}
