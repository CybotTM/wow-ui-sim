# Timer callback timing provenance

Opt-in simulator diagnostics identify original timer callbacks rather than only the simulator's container invoker. Source: `src/lua_api/handler_timing.rs`, `timer_layout.rs`, `env_runtime.rs` and `workarounds/temporary/proxy_object_factories.rs`. [Timer system](../wiki/systems/lua-api.md#timer-system) describes capture and lifetime management.

## What it must do

- [ ] With `WOW_SIM_LOG_HANDLER_TIMINGS` enabled before process initialization, capture the original callback's `source:linedefined` once per registration, including container-wrapped After/NewTimer/NewTicker callbacks; repeating invocations reuse that identity.
- [ ] Emit `[handler-entry]` on stderr before each callback, regardless of duration threshold. Include `addon`, `handler=C_Timer`, `frame=#0`, existing invoker `source` when known, `timer_id` and original `callback_source`. Missing Lua source is `<unknown>`.
- [ ] Preserve existing thresholded `[handler]` return records and field meanings (`addon`, `handler`, `frame`, `duration_ms`, optional invoker `source`); append matching `timer_id`, `callback_source` and `outcome=ok|error`. Callback failures retain the existing error-handler path. A missing return identifies an entered callback, not its failure cause.
- [ ] With the variable unset, emit no new records and bypass provenance capture, metadata lookup and formatting. Preserve callback arguments, container/proxy identity, taint, cancellation, errors and scheduling in both modes. Diagnostic tracking must not extend callback lifetimes.

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

## Known gaps (current cycle)

- [ ] Execute bounded Retail and Forever GREEN/preservation checks and record exact scope.
- [ ] Verify formatting and profile compilation after implementation.
- Callback lifetime preservation is currently source-audited (weak wrapper keys and string-only metadata), not a dedicated GC regression.

## Out of scope

Registration caller stacks, callback arguments/payloads/SavedVariables, per-line cost, vendor/addon edits, timer policy changes and diagnosis of the earlier 95-second callback. Source identities can contain private paths. No additional protected call, retry or error suppression is introduced.
