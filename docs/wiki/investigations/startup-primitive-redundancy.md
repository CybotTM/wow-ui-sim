# Startup Primitive Redundancy

Two simulator-side startup optimizations remove duplicate bookkeeping without changing Blizzard Lua. SetText optimization remains deferred; its new tests preserve current behavior only.

## Changes

`LayoutCache.resolved` already receives a zero-layout sentinel before parent/anchor recursion. Recursive lookups return that sentinel before reaching the separate private `visiting` set. Removing that set and its begin/finish probes preserves the sentinel, final overwrite, independent `recomputed` set and invalidation. Cyclic anchor results remain entry-order-dependent; this is intentionally preserved, not corrected.

UI-string replacement on Retail discarded an unconditional raw lookup. Restoration performed that lookup twice with no intervening write. Registration now makes one policy decision: Preserve reads once and retains every non-nil value; Replace reads only for Forever's function-preservation rule. Integer/float/string constants, raw lookup helpers, interning and security behavior are unchanged.

## Bounded preservation

Tests-first `85e8f8eda` precedes layout `f74ecfc64` and registration `ee9b563d8`. Retail/default dev opt1: **42/42 selected tests PASS both before and after**, including seven new cases. Coverage includes cyclic geometry/effective scale/cache removal/missing frames; raw registration values, function/table identity and absence of `__index` callbacks; text nil/empty/Unicode/NUL, repeated public SetText after style changes, and existing layout/cursor/callback tests.

This does not establish native parity, all-profile acceptance, rendered-cycle equivalence or a performance gain. Forever's branch is source-inspected, not executed in this Retail-only gate.

## Measurement

Matched sealed before/after startup measurement is pending. Private receipts and exact argv: `/home/osso/.local/state/wow-ui-sim/handoff/perf-build/report.md`. No gain claimed from source deletion or historical profile percentages.

## Sources

- [Layout implementation](../../../src/layout.rs) and [cycle preservation](../../../tests/layout_cycle.rs).
- [String registration and tests](../../../src/lua_api/globals/strings/mod.rs).
- [Text preservation tests](../../../src/lua_api/frame/methods/text_attribute_event/text/tests.rs).
- [Profiling protocol](../../../.claude/skills/flamegraph/SKILL.md) — acquisition guidance; private report records the matched cold-start protocol and recovery limitations.

## See Also

- [[layout-profile]] — historical release layout profile; not the current A/B comparison.
- [[intern-string-ranking]] — related interning investigation; no interner redesign here.
