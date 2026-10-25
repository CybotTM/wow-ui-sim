# PTR input interface style

PTR 12.1.7.70493 declares `C_InputInterfaceStyle.GetCurrentStyle()` in generated `InputInterfaceStyleDocumentation.lua`. The original TOC-selected `Blizzard_SharedXML/Mainline/InputUtil.lua` uses this query. This contract covers publication of the existing input-style state, not native device selection. See the [investigation](../wiki/investigations/ptr-input-interface-style.md).

## What it must do

- [x] PTR publishes `GetCurrentStyle()` with a non-nil `InputDeviceInterfaceType` result from the environment's existing supplied input style.
- [x] PTR publishes the documented enum members `Mkb = 0` and `Gamepad = 1`.
- [x] Original, runtime-selected `InputUtil` predicates and immediate setup/init callbacks distinguish explicit MKB, Gamepad, then MKB inputs in one selector.
- [x] Preserve existing Forever publication and leave Retail and Classic publication unchanged. The checked Retail manifest and generated input docs do not declare this namespace or enum.

- [x] PTR accepts `INPUT_DEVICE_INTERFACE_TRANSITION` through the strict `all(client-ptr, retail-12-1-5)` epoch exception. Loading the original SharedXML closure must produce no Lua errors; its listener must report registration true and an unrelated event false. Retail and Forever event validity remain unchanged.

## SharedXML listener assumption

The PTR selector expects two listeners, one per environment, without changing normal loading or secure replay. This is an approved expectation correction, not native-count proof.

REVISIT: Assumed twice (normal + secure replay) per user decision 2026-10-25; native count unproven (benlla-re 5875, Clank 5.0.3 audit game-engine acd869efa). Re-check once Retail is decompiled.

Decision key: `sharedxml_twice_default_20261025`.

## How it works

- [Selection, backing state and static contradiction table](../wiki/investigations/ptr-input-interface-style.md).
- [Consumer selection and RED proof](../wiki/investigations/consumer-selection-and-red-proof.md).

## Implementation inventory

- `src/c_api/c_input_interface_style.rs` — existing state-backed getter, enum publication and supplied-style transition input.
- `src/c_api/registration.rs` — profile publication boundary; GREEN selects PTR and existing Forever through `client_profile::ACTIVE`.
- `src/lua_api/globals/real/input_interface.rs` — existing Forever legacy query over the same style field; unchanged.

- `src/event/valid_events.rs` — narrow `all(client-ptr, retail-12-1-5)` event exception. Declaration: `/home/osso/.local/state/wow-ui-sim/handoff/cf2-aura-cleanup/ptr-cache-12.1.7.70493/wow-ui-sim/blizzard-ui/ptr/AddOns/Blizzard_APIDocumentationGenerated/InputDocumentation.lua:341-348` names `INPUT_DEVICE_INTERFACE_TRANSITION`, marks it synchronous, and declares non-nil `newMode`/`oldMode` input-style enums. This does not change queued delivery policy.

## Tests asserting this spec

- `tests/ptr_input_interface_style.rs::ptr_input_util_observes_supplied_styles_and_initializes_matching_subscribers` — original SharedXML dependency closure through runtime TOC selection; both styles and the shared InputUtil top-level event registration in one selector. The final corrected selector passes on merged source `8798e6012`; detached producer-only RED `c779d1882` fails at the original rejected event with byte-identical tests. Each phase has one exact list and one exact run.
- `tests/wowforever_input_style.rs` — prior slot-2 preservation: three pass; case 1 fails on missing `C_GamePad.IsEnabled`. The 70532 API declaration exists, but no backing capability state is modeled. This is a report-only API gap, not a fixture repair; no Forever runtime rerun is claimed here.

## Known gaps (current cycle)

- [x] Current merged-source PTR GREEN and producer-only RED executed; PTR all-target check passes with only the three allowed `fast_handler_late_bind.rs` dead-code warnings; Retail all-target check passes with zero warnings. Exact source/cache/CVar/CPU receipts are linked from the [investigation](../wiki/investigations/ptr-input-interface-style.md#approved-twice-final-proof-2026-10-25). Historical stock/Forever evidence is not replaced by a new runtime claim.
- [ ] Laptop questions: native initial style, relation to the style CVar and physical device state, same-style event behavior, and synchronous transition delivery. Existing simulator policy is not native evidence.

## Out of scope

Host gamepad delivery, CVar/device reconciliation, extra APIs, new default values, Retail publication without declaring docs, vendor edits and general PTR startup cleanup. Generated `InputDocumentation.lua` documents the transition event; this repair does not change its existing simulator queue policy.
