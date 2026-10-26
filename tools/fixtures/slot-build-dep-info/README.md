# Cargo dep-info fixtures

Sanitized input subsets copied on 2026-10-25 from slot-0's real
`debug/wow-sim.d`, `debug/wow-cli.d`, and
`debug/deps/integration-7df7921f32c569e7.d`.

These are the three files probed in freshness verification report5. Output
rules, fixed source-mount paths, relative integration entry point, and the
`tests/../Cargo.lock` spelling are retained. Unrelated inputs and empty rules
are omitted. Tests replace slot/output roots without building or modifying the
shared slot.

`ct-bins-utility.d` is a byte-exact copy of slot-2's `debug/bench_spellbook.d`,
read on 2026-10-25 (SHA256
`0ef167dd61700c87a97f9a4e50f80f44b54a53399369d0bfa59f10be8b81c45d`).
The other three utility `.d` files have the same 1,247 inputs, all under
`/home/osso/.cache/wow-ui-sim-srcmount/ct-bins-slot-2`. The test substitutes
only target names and temporary roots. This reproduces the zero-input errors in
`tts-integrate2-wowforever-integration-1792966038272658748.json` without touching
or building in the shared slot.
