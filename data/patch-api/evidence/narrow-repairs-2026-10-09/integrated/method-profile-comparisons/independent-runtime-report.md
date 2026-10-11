# Independent saved six-profile bare-method comparison audit

**Capture integrity: PASS. Actual comparator outcomes: 1 PASS, 11 FAIL; 12 commands select 12 actual cases; zero resource blocks.**

Epoch: `20261011T031711Z` (UTC). Saved artifacts only; no runtime, tests, builds, replay, operations, delegation, or canonical/tracked mutations. Owned outputs are this report and `independent-report.json` only. Main's retained packet has not been provided or inspected.

## Exact matrix

Full rows mean complete extra/missing difference rows relative to the frozen discovery oracle, not total methods or behavioral coverage. Shared default-Retail fixtures contain 12,380 extra rows and 33 missing rows.

| Profile | Extra outcome / exit | Extra removed / added | Extra full rows | Missing outcome / exit | Missing removed / added | Missing full rows | Actual selected |
|---|---|---|---:|---|---|---:|---:|
| PTR | FAIL / 101 | 0 / 32 | 12,412 | PASS / 0 | 0 / 0 | 33 | 2 |
| Wrath | FAIL / 101 | 939 / 192 | 11,633 | FAIL / 101 | 16 / 21 | 38 | 2 |
| Mists | FAIL / 101 | 939 / 192 | 11,633 | FAIL / 101 | 16 / 21 | 38 | 2 |
| Era | FAIL / 101 | 939 / 192 | 11,633 | FAIL / 101 | 16 / 21 | 38 | 2 |
| Anniversary | FAIL / 101 | 939 / 192 | 11,633 | FAIL / 101 | 16 / 21 | 38 | 2 |
| WoWForever | FAIL / 101 | 683 / 128 | 11,825 | FAIL / 101 | 16 / 21 | 38 | 2 |
| **Total** | **6 FAIL** | | | **1 PASS / 5 FAIL** | | | **12** |

Each command contains `--exact`, the matching extra or missing selector, `--nocapture`, `--test-threads=1`, and `timeout 90`, using its accepted Unit2 sealed integration binary. Every complete stdout contains `running 1 test`, exactly one matching named result, and a matching one-selected-case summary (zero ignored/measured). Filtered-out counts per command: PTR 9,895; Wrath 8,034; Mists 8,198; Era 8,022; Anniversary 8,022; WoWForever 8,338. No zero-selection pass, timeout, or noncomparison failure.

## Complete diagnostic and derived-file audit

All 24 stdout/stderr files (183,941 bytes, including 5,552 diagnostic rows) were read in full, without fixed-length slicing. Every diagnostic section was parsed to its actual end; all removed/added rows were consumed and independently compared with the saved execution receipt. Unexpected diagnostic lines: zero. Non-row stderr lines and complete stdout, along with all diagnostic rows and stream hashes, are preserved in the JSON report.

All 11 failures reach the explicit `diff_methods_<kind>.txt is out of sync with the current metatable surface.` assertion at `tests/method_diff_coverage.rs:264:5`. No environment creation, snapshot acquisition, or missing-fixture panic is misclassified as a comparison.

For each of the 12 complete files, bytes equal exactly:

`sorted(default fixture minus complete removed rows plus complete added rows)` joined with LF and one terminal LF.

All fixture/diagnostic/complete rows are unique and sorted; no CRs or blank rows in complete files. Every removal belongs to its fixture and every addition is absent from its fixture. Full-row counts match saved receipts. PTR missing PASS independently proves fixture equality through the named test receipt; its complete file equals the 33-row missing fixture.

Worker `reconstruct` returns `None` with `NON_COMPARISON_FAILURE` when a failed command lacks the assertion marker, and its caller writes no complete file for `None`. That guard is source-inspected, not an executed negative test. This epoch contains no such failures; no rows were derived from one.

## Worker, request, controller, admission

`worker.py`, `request.jsonl` code, and the completed controller's executed code are byte-identical. Controller console names this exact epoch; controller stderr is empty. Queue submission exit is 0 and its oneshot unit, request input, controller output paths and working directory match the worker receipts. Each saved profile cgroup names `wow-method-profiles-20261011t031711z.service` under `agents.slice`. Each final admission receipt admits; PTR admits on attempt 2 and the other five on attempt 1. No profile is resource-blocked. Per-profile outcomes, aggregate results and epoch outcome agree with independently parsed actual-case counts.

## Accepted Unit2 composition and source identity

All six comparison submissions embed the exact accepted Unit2 `artifact.json`. Each saved Cargo integration artifact has source `tests/integration.rs`, target kind `test`, identical feature/build-profile metadata, and exactly its intended `client-*` feature. Saved pre/post seals and accepted independent Unit2 evidence match; current file sizes match. Large binaries were **not rehashed** by this audit. This composes saved attested seals, not a new cryptographic claim about current binary contents.

| Profile | Saved SHA256 | Bytes |
|---|---|---:|
| PTR | `363984787c902985c21d540dc69aa68baee9891ed2c46c892b8344b343af10a9` | 466,938,512 |
| Wrath | `f458b5f445a816cbf61b4a76aade638c18c3660ae6c3a450968a597da0fb1a9f` | 435,601,288 |
| Mists | `0cd805af673f8bbafb9d8af38a3eb9454fd7cb036aa325277e2eeefd2b6ed33c` | 438,312,496 |
| Era | `869a9db256f221337bf7a40df8e4d9d5c5c5e35124e97daa25b11567c5dc2cff` | 433,600,848 |
| Anniversary | `51e77ea6d2c90ef7217bfce41eff4454c52b68035896579f4bf818fdb8aa108c` | 433,309,856 |
| WoWForever | `ca952f984697f20e6b2ac4183b106c1aeb842ade679d8098ad1fc49874586e7b` | 439,867,624 |

All six exact seals, byte sizes and feature lists are also preserved in `independent-report.json`.

The four current comparator inputs exactly match every saved submission's input hashes: `WowDiscovery.lua`, both canonical diff fixtures, and `tests/method_diff_coverage.rs`. All **3,846** accepted Unit2 source/build/test baseline paths also independently match current SHA256; zero missing or changed paths. All six accepted Unit2 before/after-compile/after-execution snapshots equal that baseline. Accepted Unit2's cc4f4c782 → 24c727075 documentation-only revision composition remains the governing prior evidence; no build was repeated.

Canonical fixtures and source inputs were not written by this auditor. Saved worker source writes only its verification epoch, not canonical fixtures. Identity evidence is limited to the four recorded comparator inputs and the 3,846-path accepted source scope; it is not a claim that every repository file stayed unchanged.

## Meaning and proof limits

These failures are **shared default-Retail fixture assertions under other simulator profiles**. They prove exact disagreement of bare metatable enumeration with those shared fixtures, not eleven native/profile implementation defects. Missing and extra mean relative to the frozen `WowDiscovery.lua` oracle. Current per-profile native epoch remains **UNKNOWN**; no native capture or oracle refresh was performed. Enumeration is not method-dispatch or semantic parity proof.

Source lanes **KEEP_BLOCKED**. Tests instantiate bare `WowLuaEnv::new`, enumerate synthetic objects' metatable `__index`, and read the retained discovery/fixture inputs. Receipts set `WOW_SIM_CASC=0`. No Blizzard UI/cache-loading, asset acquisition, source acquisition, GUI, full addon startup, native behavior, dependency closure, or full-suite acceptance is established.

Prior Retail's two covered passes—one fresh and one saved—are distinct and excluded from this 12-case capture. That classification is caller-provided here, not re-counted or replayed. Main's later retained packet remains uninspected until its path is supplied; inspection must not replay anything. Parent remains OPEN.

Retention note: private verification root redacted; original raw report JSON not copied. Prior audit counts describe the earlier65-file packet.
