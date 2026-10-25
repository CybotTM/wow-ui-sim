# Cargo dep-info fixtures

Sanitized input subsets copied on 2026-10-25 from slot-0's real
`debug/wow-sim.d`, `debug/wow-cli.d`, and
`debug/deps/integration-7df7921f32c569e7.d`.

These are the three files probed in freshness verification report5. Output
rules, fixed source-mount paths, relative integration entry point, and the
`tests/../Cargo.lock` spelling are retained. Unrelated inputs and empty rules
are omitted. Tests replace slot/output roots without building or modifying the
shared slot.
