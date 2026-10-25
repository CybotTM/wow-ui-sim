# PTR input interface style

Static investigation of the PTR 12.1.7.70493 startup failure at original `Blizzard_SharedXML/Mainline/InputUtil.lua:12`. At the base revision, the namespace has a real existing simulator model, but registration is Forever-only. The [contract](../../specs/ptr-input-interface-style.md) stages PTR publication. The slot-2 results below supersede the original source-only stage; the PTR selector is blocked by a native environment-count question.

## Source declarations

Authenticated private CF-2 cache provenance selects version `12.1.7.70493`, Gethe revision `2cdbc7ff511ec6a8c2f4f8341accd1da44a3c9af`, manifest SHA-256 `ba4414999237ad7b50de73a2a61d7dd18fee31b2f2c3b70d5a21ad3cdc34d328`. Files read in this investigation were hash-checked against that run's inventory, not copied into the repository.

- `InputInterfaceStyleDocumentation.lua`: one function, `GetCurrentStyle() -> style: InputDeviceInterfaceType`, non-nil, Environment All. Its Events and Tables sections are empty.
- `InputConstantsDocumentation.lua`: `InputDeviceInterfaceType.Mkb = 0`, `.Gamepad = 1`.
- `InputDocumentation.lua`: `INPUT_DEVICE_INTERFACE_TRANSITION(newMode, oldMode)`, both non-nil input-style enums, marked synchronous. It does not document initial style or same-style behavior.
- Checked Retail manifest lacks `InputInterfaceStyleDocumentation.lua`; checked Retail generated `InputConstantsDocumentation.lua` lacks this enum, and its shared `InputUtil.lua` lacks the new style helpers. Do not publish this PTR API on Retail on this evidence.

## Original consumer selection

PTR TOC lookup prefers `_Mainline`/`-Mainline`, then generic, then `_Standard`/`-Standard`. Its game-type set is `mainline, standard`; `[Family]` resolves to `Mainline`, `[Game]` to `Standard`.

`Blizzard_SharedXML/Blizzard_SharedXML.toc` is generic, non-LoD, `AllowLoad: Both`, without an addon-level game exclusion. It selects `[Family]\\InputUtil.lua` followed by `InputUtil.lua`, without per-entry filters. Runtime closure loading, not forced evaluation of a cached Mainline file, supplies both original sources to the staged test.

`Blizzard_CompactRaidFrames/Blizzard_CompactRaidFrames.toc` is generic, non-LoD, game-screen eligible and depends on `Blizzard_RaidFrame`. It selects the Mainline manager Lua/XML pair. The XML installs `CompactRaidFrameManager_OnLoad`; that function calls `CompactRaidFrameManager_Collapse`, which calls `InputUtil.IsGamepadUIEnabled()` at line 352. The predicate calls the missing namespace at InputUtil line 12. OnLoad later registers gamepad setup/init handlers. Collapse needs the boolean branch to leave the MKB forward toggle visible without running the gamepad icon path; it does not need a physical controller.

## Consumer needs

The complete static candidate inventory is retained in the lane handoff's `call-sites.tsv` and `source-audit.json`. It separates selected startup files from glue, LoD and unselected variants. It records source line, function-context candidate, operation and need for every lexical reference; selected handlers are not all claimed executed during startup.

| Original source | Need |
| --- | --- |
| Mainline InputUtil lines 4, 12, 16 | Current enum, equality to Gamepad and Mkb. These are the only direct namespace calls in the scanned PTR UI. |
| Shared InputUtil lines 106, 121 | Immediate Init/Setup only when the subscriber's style equals current style; the manager uses this path. Uninit is deferred until an event. |
| InputDeviceIconSetManager Init | Current enum selects Generic versus active gamepad label set; the file calls Init at top level. Device-map queries are separate APIs. |
| CompactRaidFrameManager lines 337, 352, 360, 379, 398 | Expand/collapse controls, binding/ground-targeting/smart-nav guards. Setup/Init registrations at 515–517 use the same state. |
| StatusTrackingManager GetExpectedWidth; Mainline overrides | MKB versus gamepad width/layout. Other selected UI references use the same predicate or subscriber registration, not additional namespace members. |

`IsGamepadUISupported` is different: it checks whether CVar `InputDeviceInterfaceStyle` exists. It does not query current style and is not changed here. Alias/reference rows and conditional handlers are candidates, not runtime invocation evidence. Full original CompactRaidFrameManager startup remains a future stock-run check, not a claim made by the SharedXML selector.

## Existing state and minimal repair

`SimState.input_interface_style` already stores `InputInterfaceStyle::{Mkb, Gamepad}`. The existing C API getter reads that field; `set_input_interface_style` updates it and queues the documented new/old payload. The legacy Forever `IsUsingGamepad` reads the same field. Mapped sticks, capability probes, frame gamepad-enable flags and hover policy are separate state, not current interface style.

GREEN registers the existing namespace and enum when `client_profile::ACTIVE` is PTR or Forever. It preserves the existing model, getter, default and transition policy; Retail and Classic do not enter this registration path.

Slot compilation exposed three stale Forever-only compile gates: the module, state field and initializer. The backing model now compiles on every profile, as required by the runtime `ACTIVE` branch. Namespace publication remains PTR/Forever-only. This removes the repeated compile gates without adding a new feature or changing modeled values. The original PTR compile failure is recorded in the slot-2 handoff.

The staged test uses the existing setter before observing each of MKB → Gamepad → MKB. It asserts the original InputUtil query, both booleans and all four immediate setup/init counters. It does not replace or wrap any original helper. No new passive production scaffold is needed: the setter already exists.

## Static contradiction table

One independently claimed RED selector: `ptr_input_util_observes_supplied_styles_and_initializes_matching_subscribers`. These are static contradictions, not executed mutants.

| Candidate producer | Contradiction within that selector |
| --- | --- |
| Namespace/member missing | Original InputUtil.GetCurrentInterfaceStyle cannot return the supplied MKB value. |
| Constant Mkb / 0 | Gamepad iteration must return 1 and run only gamepad setup/init. |
| Constant Gamepad / 1 | First and final MKB iterations must return 0 and run only MKB setup/init. |
| Constant nil / false / true / other value | Exact numeric return assertion rejects it. |
| No-op style input / getter ignores supplied state | Both different values are required in the same environment. |
| Incorrect enum values | Explicit documented numeric assertions reject them. |

The selector tests immediate registration behavior, not event-delivery timing, physical devices or every startup widget. The original source stage did not execute these assertions. Historical slot-2 execution reached an earlier listener-count blocker; those style assertions were unreached then. The approved-twice GREEN below now reaches and passes them.

## PTR event-validity extension (2026-10-25)

The earlier namespace GREEN `9ea7c4c52` does not make the transition event registrable. PTR uses the strict generated event table; the existing Forever-only entry cannot authorize PTR. The narrow repair adds this event to the existing epoch additions under `all(client-ptr, retail-12-1-5)` in `src/event/valid_events.rs`, without changing Retail, Forever or dispatch policy.

Declaration: `/home/osso/.local/state/wow-ui-sim/handoff/cf2-aura-cleanup/ptr-cache-12.1.7.70493/wow-ui-sim/blizzard-ui/ptr/AddOns/Blizzard_APIDocumentationGenerated/InputDocumentation.lua:341-348` declares the literal name, `SynchronousEvent = true`, and non-nil `newMode`/`oldMode` enum payloads. The same cache's `Blizzard_SharedXML/Blizzard_SharedXML.toc:34-35` selects Mainline then shared InputUtil; `Blizzard_SharedXML/InputUtil.lua:28-30` creates the forbidden listener and registers the event at top level.

After merging master with both wiki sides retained, event RED `16c75aeda` extends only the existing test file. Event GREEN `0bd666459` changes only the validator; test bytes are identical. Both already include the earlier C API publication. The normal closure loader must report no Lua errors and leave exactly one registered listener. The test retrieves its existing frame reference because forbidden frames are hidden from enumeration, checks `IsEventRegistered` true for the transition and false for `PLAYER_LOGIN`, and checks that its OnEvent handler comes from the original shared InputUtil. No vendor code or API is replaced or wrapped.

An unconditional RegisterEvent success without recording fails the listener-count assertion. Constant-true event validation fails the unknown-event registration rejection; constant-false validation fails the original top-level load. A constant true/false IsEventRegistered fails one of the positive/negative queries. Existing MKB → Gamepad → MKB assertions still reject constant style getters. These are static contradictions, not executed mutation receipts. These contradictions were source-only at staging. Slot-2 results follow; no mutation runs are claimed.

## User decision: two environments (2026-10-25)

Decision `sharedxml_twice_default_20261025` corrects the PTR test expectation from one listener to two, one per environment. Every other assertion remains unchanged. Normal loading and secure replay stand; this correction does not establish native parity. The one-listener requirement in the event-validity stage is historical.

REVISIT: Assumed twice (normal + secure replay) per user decision 2026-10-25; native count unproven (benlla-re 5875, Clank 5.0.3 audit game-engine acd869efa). Re-check once Retail is decompiled.

Final merge `e98809b2a` includes master `f1f3c930d` and its phase-2 crate split. The narrow PTR/epoch event exception stays in `src/event/valid_events.rs`, above the feature-neutral `sim-event` policy call; the other event rules are inherited unchanged. Wiki index/log conflicts retain both sides through line-based union.

Forever case 1 is a missing documented API, not a fixture issue. The 70532 `GamePadDocumentation.lua` declares `C_GamePad.IsEnabled()` returning one non-nil boolean (source SHA256 `b42cbb4b4a10a2106c116126f7fe93acb652020b1affafcbec47533957519c35`). The raw C API registration omits it; existing state models mapped sticks and hover policy, not global capability. `GamePadEnable` exists as a CVar, but its relation to this query is unproven. No constant, new capability state or assumed CVar link is added. This case remains report-only under the user's state-backed-fix condition.

## Approved-twice final proof (2026-10-25)

Merged source `8798e6012` passes the corrected PTR selector: exactly one `--exact --list`, then one `--exact` run, one test passed. Every other test byte is unchanged from `e71d70b91`. This reaches no-load-error, two-listener, original handler-source, positive/negative event-registration and explicit MKB → Gamepad → MKB query/setup/init assertions. Two listeners remain an approved assumption, not native proof.

Producer-only detached RED `c779d1882` removes only the eight-line PTR/epoch acceptance block from the root event facade. Its test blob matches GREEN. Exactly one list and one run produce exit 101 at the original shared InputUtil rejected event; callback-helper and icon-texture failures follow. The runtime error names the original chunk without a line number; the unchanged pinned source registers the event at line 30. No substituted consumer or diagnostic seam is used.

PTR and Retail each receive one `check --all-targets`, both exit 0. PTR has only `find_original_frame`, `original_on_click` and `original_script` dead-code warnings in `tests/fast_handler_late_bind.rs`; Retail has zero warnings. Workspace formatting passes. There is no new stock, full-suite, Forever runtime, host-device or native-count claim.

All builds use `slot-build.py`, slot 2, `CAPACITY_ID=lead_ptr_input_twice`, literal ADD admission and `-j1`. All four build units are `exited` with captured CPU counters before mode0600 ownership is released. The private PTR70493 cache matches all 4,578 authenticated source files; all four sealed test invocations use separate fresh CVar copies with unchanged original O_NOATIME stat/hash guards.

Full report and exact argv/artifact/CPU/guard receipts: `/home/osso/.local/state/wow-ui-sim/handoff/ptr-input-interface-style/twice-run/REPORT.md`. The following slot-2 results remain historical evidence.

## Slot-2 execution (2026-10-25)

Code GREEN `a3987cfbc` includes one `--no-ff` merge of master `8784a49cd` (`7f463e73f`) and the separate compile-gate correction. Producer-only RED `63b91c8f4` removes only the four validator lines. Both retain test blob `5abd258ee6a8dee0538a0cf6ce384022de21fda6`.

| Boundary | Observed result |
| --- | --- |
| PTR RED, one exact case | Exit 101: original `InputUtil.lua:30` rejects the event; original `InputDeviceIconSetManager.lua:14` then lacks the callback helper. |
| PTR GREEN, one exact case | Exit 101: no closure Lua errors, then two registered listeners instead of the asserted one. Later registration/style assertions are unreached. **BLOCKED-BY-NATIVE-ANSWER**. |
| Two-listener diagnosis | Unnamed frame 386 registers during `load_addon_internal` normal pass (`addon.rs:230`); unnamed frame 673 registers during `maybe_replay_blizzard_lua_in_secure_env` (`addon.rs:585`, caller line238). Both Lua stacks identify the same original `InputUtil.lua:30`. There is one vendor registration line; the simulator deliberately replays SharedXML in the secure environment. |
| Forever preservation | Three exact cases pass: transition-time query, immediate vendor callbacks, transition/environment isolation. Capability/frame-flag independence fails before style assertions because `C_GamePad.IsEnabled` is nil. Its producer and test are unchanged from the merged master. |
| All-target compilation | PTR and Forever each pass once on code GREEN; only the three allowed inherited `fast_handler_late_bind.rs` dead-code warnings occur. |

No loader semantics or assertions were changed. Main asked the laptop whether native PTR executes this listener once globally or once per public/secure environment. Diagnostic instrumentation was committed only on a detached proof chain and is absent from the lane branch.

Source-bound private PTR70493 sync authenticated all 4,578 files. Raw stock master/GREEN error maps contain 430/300 unique messages and 610/414 occurrences: 230 keys disappear, 100 appear, and six shared-key counts change. Event-registration, callback-helper and missing-input-namespace errors disappear, but stock startup is not clean. The full per-message successor classification is evidence, not a claim that every raw delta is a new regression.

Exact commands, artifact hashes, CVar/O_NOATIME guards, full stacks, raw maps and CPU counters are retained in `/home/osso/.local/state/wow-ui-sim/handoff/ptr-input-interface-style/slot2-run/REPORT.md`. No native default, same-style, hardware or synchronous-delivery parity is established.

## Laptop questions

1. What is the native initial style with no connected controller and with a controller? Existing simulator MKB default is policy only.
2. How does the style CVar interact with active device, capability and device hotplug state? This patch does not infer those links.
3. Does reselecting the same style emit an event, and exactly when is the documented synchronous event delivered? The existing queued/no-transition policy is not established as native parity.

No values are invented to answer these questions. No default, event policy, CVar or host-device behavior changes are part of this repair.

## Sources

- [C API model](../../../src/c_api/c_input_interface_style.rs), [registration](../../../src/c_api/registration.rs) and [legacy query](../../../src/lua_api/globals/real/input_interface.rs).
- [TOC selection](../../../src/loader/mod.rs) and [file filters/substitution](../../../src/toc/mod.rs).
- [Staged original-consumer selector](../../../tests/ptr_input_interface_style.rs).
- [Committed PTR manifest](../../../data/blizzard-ui-files/ptr.txt) and [Retail manifest](../../../data/blizzard-ui-files/retail.txt).

## See Also

- [[consumer-selection-and-red-proof]] — applicability and standalone contradictions.
- [[client-profiles]] — profile publication and cache provenance.
