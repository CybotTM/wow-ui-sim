# AuraContainer EventRegistrations: existing exact proof reuse

Read-only audit recorded 2026-10-11. No source or test execution performed. Repository HEAD: `7e161290d8acd5e473e7d5bba5c0274ec19ad085`; c75 proof commit: `c75ccfe02c52f76a0456ae393581a3b9ca601ce7`. HEAD and c75 have identical bytes for the requested tests, implementation file, and spec (path-limited `git diff` was empty). Relevant files last changed at `4a303f6d036477eeff88e527942270028e12357c` (2026-10-04).

## Existing proof in c75 log

`/home/osso/Projects/wow/full-suite-results/c75ccfe02c52f76a0456ae393581a3b9ca601ce7.log`:

- Line 5837: `PASS ... forbidden_aspect_enforcement::aura_container_intrinsic_event_registrations_block_addons_not_blizzard`.
- Line 5793: `PASS ... forbidden_aspect_creation::event_registration_aspect_rejects_addon_callers_and_permits_secure_callers`.
- Line 5792: `PASS ... forbidden_aspect_creation::event_registration_aspect_preserves_callbacks_when_replacement_is_rejected`.
- Lines 5808 and 5857: PASS for AuraButton initializer aspects and AuraButton handler/input guards.
- Line 10732: main integration suite summary, 10,699 passed, 19 skipped.
- Lines 10734 onward show a separate `prefork_full_ui` invocation; line 13306 reports 2,320 passed, one failed. The same log later records suite summaries of 2,001/2,001 passed, zero skipped (line 15830). Do not misreport the entire multi-command log as uniformly green. These unrelated prefork failure details do not invalidate the exact named AuraContainer PASS, but this is reuse of logged evidence, not a current rerun.

No exact named PASS exists in the inspected relevant lines for every guard; absence of a logged named test is not evidence the guard is missing. The logged test-name selector search found the AuraContainer test and the two event-registration tests above.

## What assertions prove

`tests/forbidden_aspect_enforcement.rs::aura_container_intrinsic_event_registrations_block_addons_not_blizzard` loads `Blizzard_AuraContainer`, creates `AuraContainer` with `CustomAuraContainerTemplate` inside a tainted addon setup, calls `SetUnit('player')`, asserts `GetForbiddenAspects()` includes `EventRegistrations`, and confirms Blizzard `OnLoad_Intrinsic` registered `AURA_DATA_PROVIDER_SWITCH`. It taints addon closures and verifies `RegisterEvent`, `IsEventRegistered`, and `UnregisterAllEvents` calls fail; subsequent assertions prove no `PLAYER_LOGIN` registration was added, the Blizzard event remained registered, and a secure caller can register/query `PLAYER_LOGIN`. A plain Frame is asserted not to carry the aspect. This directly disproves the stale mask-zero claim for the tested current runtime path.

`tests/forbidden_aspect_creation.rs::event_registration_aspect_rejects_addon_callers_and_permits_secure_callers` checks eight methods—RegisterEvent, RegisterUnitEvent, RegisterAllEvents, RegisterEventCallback, RegisterUnitEventCallback, UnregisterEvent, UnregisterAllEvents, IsEventRegistered—against untainted and addon-tainted closures on an aspect-bearing Frame. It compares observed success booleans to the expected secure-success/addon-failure matrix. The companion delivery test proves rejected mutations preserve listeners, unit filters, callbacks, and delivery. These are simulator behavior assertions, not native-client proof.

## Source semantics (current and c75 identical)

The implementation is tracked at `src/lua_api/frame/methods/forbidden_aspects/events.rs` (not `src/frame/forbidden_aspects/events.rs`; requested path is absent). `apply_xml_forbidden_aspects` reads the existing XML frame field `frame.intrinsic`; when it is exactly `Some(true)`, it calls `native_intrinsic_aspects(frame.name.as_deref())`, which maps the literal intrinsic name `AuraContainer` to `EventRegistrations`. The resolved mask is passed to `add_forbidden_aspects`; no new type is involved. The function applies to XML intrinsic objects, including their base/template construction path; evidence specifically proves the `CustomAuraContainerTemplate` instance.

`ensure_forbidden_aspect_absent` returns immediately for mask zero or secure caller (`state_is_secure`). Otherwise it resolves the named aspect and errors if the stored mask includes it. Therefore the event-registration restriction rejects tainted addon callers, while secure/Blizzard callers retain access. Source comment grounds this in the 12.1.0 prose and Blizzard's `AuraContainerPrivateMixin:UpdateEventRegistrations`; current test verifies simulator-side behavior. This does not establish native-client behavior independently.

## Reconciliation

`docs/specs/patch-12-1-0-events.md` is stale in two material assertions: its L88 checkbox and Known gaps say the intrinsic mask is zero / the guard test failed, and its inventory says the cached Blizzard XML lacks the declaration. Current source instead applies `EventRegistrations` for intrinsic `AuraContainer` through `frame.intrinsic`/intrinsic name handling, and the exact regression test passed in c75. The spec also says L83 rejects operations for “secure and tainted callers”; that is opposite to current source and test semantics: **addon-tainted callers are rejected; secure/Blizzard callers are permitted**. Its broader seven-operation/listener-preservation claim has adjacent test evidence, although the current named test covers eight methods and listener/callback preservation is in companion tests.

No docs were edited per read-only request. Do not carry the old failure forward as current. Source/probe describes simulator behavior; native behavior remains unverified here, consistent with the June 23 source note and the spec's no-native-parity disclaimer.

## Scope / constraints honored

No tests, builds, reruns, operations, source edits, delegation, shell, or repo mutation. Read-only git status showed unrelated pre-existing changes (`docs/specs/patch-12-1-0-onupdate.md`, `docs/specs/scenario-unit-criteria.md`, untracked `.code-index.db`, `verification/`); left untouched. This handoff file is the requested output artifact outside the repository.