# Achievement superceding relations — initial RED, expanded execution blocked

User selected explicit relation map with one empty table for absent IDs. [Contract](../../../../docs/specs/achievement-superceding-relations.md) marks this simulator policy, not native catalog, ordering or unknown-ID semantics.

[Independent initial RED gate](gate/report.md) at `37d17371b`, epoch `20261011T141132Z`: compile0; **64 existing PASS/1 new FAIL/65 executed**, execution101. Failure is `expected a relation table`; [sealed initial test source](initial-test-source.rs) first asserts exactly one return. Recorded evidence establishes a non-table result, not its exact value. No new namespace query preceded that capture. [Snapshot provenance](test-source-provenance.json) matches its Git source and initial input seal; current expanded tests/state are later revisions, not initial-source parity.

At `c9f3043bf`, explicit shared map/empty initializer and eight behavior cases were committed, but query remains unimplemented. [Expanded attempt](expanded-blocked/outcome.json), epoch `20261011T141534Z`, reports **RESOURCE_BLOCKED/zero cases** after five [admission failures](expanded-blocked/admission.json): load1 71.36–95.35 exceeds24 despite sufficient memory and no relevant Cargo. This is neither behavioral RED nor PASS. Retry requires actual guard admission; no producer edits during an active proof or guard bypass.

[Manifest](retention-manifest.json):10 exact UTF-8 copies,59,858 bytes, plus601-byte Git test snapshot. Binaries/full compiler-controller streams/session-provider payloads/native data excluded. Initial gate's private originals remain available; copying is not new runtime evidence. No GREEN, current state compilation, native/profile/catalog or parent-handoff acceptance claimed.
