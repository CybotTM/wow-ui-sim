# Friend-note mutation — bounded evidence

[Contract](../../../../docs/specs/friend-note-mutation.md) · [Retention manifest](retention-manifest.json)

## Executed behavior

- Retail RED at `d61c78a2b`: [eight failures](retail-red.stdout), each at [absent raw setter](retail-red-boundary-excerpt.stderr); downstream assertions unreached. [Proof](retail-red-proof.json). Error excerpt is not the full stderr stream.
- Retail/PTR GREEN at `a883157c49`: [Retail8/8](retail-green.stdout), [PTR8/8](ptr-green.stdout), [report](focused-green-report.md), [proof](focused-green-proof.json). Exact live mutation/hit/miss/clear, projections/isolation, genuine-secret/type/taint ordering and unchanged whole-popup Accept/Enter/reopen completed.
- Both normal startup probes exit0 with marker and literal `[]` ([Retail](retail-normal.stdout), [PTR](ptr-normal.stdout), [retention](normal-retention-manifest.json)); zero recorded Lua errors. Formatter exit0 once. Source stability applies to the recorded verification epoch, not current HEAD.

## Open gates

Requested default cargo check **NEVER INVOKED**, admission blocked, zero credit. Fresh test compilation does not satisfy that gate. Actual setter row60 publication remains pending; current retained publication145/94OK/51gaps unchanged. No native/network/events/persistence, earlier-profile, broad-suite or whole-handoff acceptance.

Six vendor-manifest deprecations per producer and optional guarded nil-probe diagnostics were preserved privately, not suppressed. Packet contains selective behavioral logs/proof, not binaries or private startup diagnostics. Retained hashes cover text only; no old executable/library reauthentication. Independent packet retention/privacy gate pending.
