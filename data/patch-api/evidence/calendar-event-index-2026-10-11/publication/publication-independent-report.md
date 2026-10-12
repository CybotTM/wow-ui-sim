# Independent bounded Calendar publication / retention verification

**SCOPED PASS; final documentation reconciliation/full handoff not claimed.** Owning epoch: `publication-prefork-compile-20261012T020631000464Z`. Snapshot HEAD: `1931c737b0e4694aea52ec81288c0a5d14446220`. Private machine-readable proof: `publication-independent-proof.json`, also owning epoch `proof.json`. Existing root `proof.json` remains the reused GREEN proof, untouched.

## Publication evidence

Saved compile **exit0**, `source_unchanged:true`, Cargo `build-finished success:true`. Complete compile stdout JSON records parsed; selected `prefork_full_ui` record matches artifact metadata and is **fresh=false**. Six inherited iced_wgpu manifest deprecation warnings remain. Complete runtime stdout/stderr read; saved runtime **exit0**.

Exact custom stdout:

```text
running 1 tests
test patch_8_1_0_publication_sweep::patch_8_1_0_publication_sweep ... ok

test result: ok. 1 passed; 0 failed; 1 total
running 0 tests

test result: ok. 0 passed; 0 failed; 0 total
running 0 tests

test result: ok. 0 passed; 0 failed; 0 total
running 0 tests

test result: ok. 0 passed; 0 failed; 0 total
```

**One exact publication case PASS / zero FAIL, followed by three zero-case groups—not four cases.** Saved `runtime-outcome.json` has `summary:null`; generic libtest parsing does not recognize this custom output. This null is not zero-selection evidence. Earlier wrong integration selector remains separately retained and is not acceptance proof.

`publication.json` contains **145 identity-classification rows, not 145 behavioral cases**: **89 OK / 56 current known gaps**. Every false row corresponds exactly to the current known-gap fixture. Calendar identity `wt-global-api-C_Calendar.GetEventIndexInfo-20`: expected `published`, observed `raw=function; lookup=function`, `ok:true`. This establishes publication, not getter semantics/native parity.

## First NEW artifact authentication

Only the newly sealed prefork artifact was read, once: **509,644,120 bytes**, SHA256 `7edf35884969683b6b061030188095a07414b5f4b516d43bdfe55f2b1a29bc7f`; exact metadata match. Compile record names `prefork_full_ui`; saved runtime submission selects the exact case through this sealed path. Receipt: `publication-independent-auth.json`. No old wrong-target/GREEN/RED executable or library authentication.

## Preservation / reused proof

- `296b9ac77516b36761b8492e6b1a8354d3643917` removes exactly the Calendar event-index row from the current gap fixture; remaining ordered entries unchanged. Current fixture matches that committed result. Historical source registers and frozen 8.1.0 evidence paths have no post-producer changes. **Historical87OK/58gaps is distinct from current89OK/56gaps.**
- Eight recorded source hashes match current files, including producer, registration/state/default path, fixture, behavioral tests and Lua-error sink; later commits do not change Rust/test source. Producer `fa66dd878420fec0f67aab473cc91eebadf1c090` evidence remains applicable.
- Reused `green-independent-report.md`, existing root `proof.json`, and committed retained copies: **11 PASS / 0 FAIL**, normal loaded startup **exit0/[]**, saved fmt/default-check **exit0**. No repeats or reauthentication. Independently proven RED remains **0 PASS / 9 FAIL, exit101**; unreached RED security branches are not promoted to evidence.

## Retention / documentation

Manifest actually observed: **48 copies / 172,495 bytes**, superseding requested earlier38/90,965 snapshot. All **48 copy hashes and 48 source hashes**, with byte sizes, match the manifest. Includes correct owning-target publication receipts; old zero-selection remains labeled separately. Manifest and scoped file hashes retained in proof. No binary/library/cache payload copied.

**26 scoped relative links checked: all targets and referenced heading anchors exist.** Scope: Calendar/evidence README/spec and added/changed wiki lines, not entire historical wiki. Link receipts: `publication-link-audit.json`.

Phase boundary: evidence README records executed one-case publication and independent gate pending; inspected spec/wiki text still describes publication execution pending. Newly arriving retention/docs changes are outside this captured proof. This report proves the captured publication/retention snapshot, not final documentation convergence or full parent acceptance.

## Artifact checks

[EXIST] PASS — owning compile/runtime/publication receipts, retained manifest/copies, reused GREEN/RED reports/proofs and scoped docs exist.

[SUBSTANTIVE] PASS — exact custom status output, 145 concrete identity observations, source-bound receipts and matching retained hashes; no placeholder acceptance inferred from null parser output.

[WIRED] PASS — Cargo artifact target and saved exact runtime submission identify the owning prefork case; `tests/patch_8_1_0_publication_sweep.rs:10–18` connects the case to committed register/current gap fixture/output environment. Calendar raw/lookup function publication observed.

[ANTI-PATTERN] N/A for immutable execution payloads; no implementation edits. Reused independent GREEN scan records TODO0/FIXME0/HACK0/XXX0 in added Rust/test lines. Pending documentation is explicitly a phase boundary, not hidden completion.

No builds/tests/check reruns, operations, polling, delegation or repo changes. Private proof/report writes only. No native parity, full Calendar, all-profile/broad-suite or full-handoff claim.
