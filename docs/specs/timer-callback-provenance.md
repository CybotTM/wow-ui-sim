# Timer callback timing provenance

Opt-in simulator diagnostics identify original timer callbacks rather than only the simulator's container invoker. Source: `src/lua_api/handler_timing.rs`, `timer_layout.rs`, `env_runtime.rs` and `workarounds/temporary/proxy_object_factories.rs`. [Timer system](../wiki/systems/lua-api.md#timer-system) describes capture and lifetime management.

## What it must do

- [x] With `WOW_SIM_LOG_HANDLER_TIMINGS` enabled before process initialization, capture the original callback's `source:linedefined` once per registration, including container-wrapped After/NewTimer/NewTicker callbacks; repeating invocations reuse that identity.
- [x] Emit `[handler-entry]` on stderr before each callback, regardless of duration threshold. Include `addon`, `handler=C_Timer`, `frame=#0`, existing invoker `source` when known, `timer_id` and original `callback_source`. Missing Lua source is `<unknown>`.
- [x] Preserve existing thresholded `[handler]` return records and field meanings (`addon`, `handler`, `frame`, `duration_ms`, optional invoker `source`); append matching `timer_id`, `callback_source` and `outcome=ok|error`. Callback failures retain the existing error-handler path. A missing return identifies an entered callback, not its failure cause.
- [x] With the variable unset, emit no new records and bypass provenance capture, metadata lookup and formatting. Preserve callback arguments, container/proxy identity, taint, cancellation, errors and scheduling in both modes. Ownership/source-capture frequency and unchanged invocation blocks are source-audited; bounded behavioral coverage is listed below. Diagnostic tracking must not extend callback lifetimes.

## How it works

- [Timer system and diagnostic storage](../wiki/systems/lua-api.md#timer-system)
- [Related file-budget diagnostics](startup-file-instruction-diagnostics.md) — same opt-in, separate records and attribution scope.

## Implementation inventory

- `src/lua_api/workarounds/temporary/proxy_object_factories.rs`: opt-in weak wrapper-to-original association at invoker creation; callback bodies unchanged.
- `src/lua_api/timer_layout.rs`: capture identities at native registration; retire metadata with the stored callback.
- `src/lua_api/state{,/sim_state}.rs`: per-environment timer-ID-to-source strings; no callback references.
- `src/lua_api/handler_timing.rs`: source capture and additive entry/return formatting; existing threshold parser unchanged.
- `src/lua_api/env_runtime.rs`: emit entry before invoking and return after existing error handling; shared TimedSignalMap firing retains its direct callback source.

## Tests asserting this spec

- `tests/timer_callback_provenance.rs` and `tests/fixtures/TimerProvenance/`: subprocess-captured stderr for concrete After success/error, entry-before-body ordering, matching return ID/source, disabled gate, high threshold and repeated container-backed ticker. Fixture asserts zero After arguments, ticker/container identity, addon stack taint and unchanged error delivery.
- `tests/timer_callback_container.rs`, `tests/timed_signal_map.rs` and `src/lua_api/handler_timing.rs::tests`: existing invocation/lifecycle/security and timing-format preservation controls.
- RED at `245f55610`: compile exit0; 1 PASS / 3 FAIL, missing original entry records, existing invoker-only sources `=(string):273/284`. Evidence: private `handoff/timer-provenance/red-retail-20261018T075507Z`.

## Bounded proof — 2026-10-18

Private evidence root: `/home/osso/.local/state/wow-ui-sim/handoff/timer-provenance/`.

| Scope | Exact source and receipt | Result |
|---|---|---|
| Retail runtime | `52d074234`, `green-retail-20261018T081650Z` | 4 provenance + 5 container + 16 TimedSignalMap + 7 existing timing cases = **32 PASS**, exit0 |
| Forever runtime | `30b1c32c6`, `green-wowforever-20261018T084501Z` | Same **32 PASS**, exit0 |
| Retail check | `30b1c32c6`, `check-retail-20261018T084242Z` | exit0, no compiler messages |
| Forever check | `30b1c32c6`, `check-wowforever-20261018T090954Z` | exit0; existing `UnitRole` dead-code warning |
| Formatting | `30b1c32c6`, `fmt-check-30b.txt` | `cargo fmt --check`, exit0 |

All compilation uses `--no-default-features --features sound,gui,casc,client-<profile> -j1`, async user units with MemoryMax9G and slot0 after capacity ADD. Executables stay in `deps` (normal test CVar isolation); no standalone-copy runtime was launched. `30b1c32c6` removes only an import made unused by this feature; Retail behavior is unchanged from `52d074234`. Its old lib/lib-test warnings are fixed, not suppressed. Forever's unrelated `UnitRole` warning remains.

Final rebase onto `35822d5c7` completed without conflict. All eleven owning diagnostic/fixture/container/TimedSignalMap source hashes match the pre-rebase source (`pre-rebase-source-hashes.json`, `post-rebase-source-hashes.json`). Rebased feature commits are `aca5c8be6`, `705d5051a`, `e8ebcf711`; upstream Exec-capture/UnitExists changes are outside these fixtures' timer paths. Saved runtime receipts are pre-rebase evidence, not a newly executed whole-HEAD gate. No broad suites were repeated.

## Known gaps (current cycle)

Callback lifetime preservation and once-per-registration capture are source-audited (weak wrapper keys, string-only metadata and no per-tick closure lookup), not dedicated GC/source-lookup-counter regressions. No full startup, GUI, native parity or full-suite acceptance is claimed.

## Out of scope

Registration caller stacks, callback arguments/payloads/SavedVariables, per-line cost, vendor/addon edits, timer policy changes and diagnosis of the earlier 95-second callback. Source identities can contain private paths. No additional protected call, retry or error suppression is introduced.
