# Independent bounded copy/doc endpoint verification

**PASS — endpoint `547131525`, bounded retained-copy/README/latest-input scope only.**

## Exact counts

- **51/51 text copies; 231,950 bytes.** All private originals/copies match byte-for-byte, SHA-256 and manifest byte counts; both sides decode strict UTF-8 and have equal text. **0 mismatches.** Directory contains 53 files: 51 retained copies plus README and manifest.
- **5/5 README links** resolve to files. Facts, result distinctions and caveats agree with saved receipts, prior independent report and inspected wiring.
- **9/9 input seals** match before/after hashes in `snapshot-final.json` and current source.
- Latest saved fixture at **`fda902ac2c5a0f83faafec8ec86e334d4484a663`**, `2026-10-11T06:25:32.563179+00:00`: **1/1 PASS, exit 0**, `Ran 1 test in 0.006s`, `OK`; stdout empty. No test rerun.

## Evidence admission

Native/Docker saved metadata each transitions **101→0**; complete green stdout documents parse with five workspace members, helper dependency `kind=dev`, `version=1`, `resolve=null`. Native/Docker root target records are 43/39, not test counts. Prior accepted native comparison remains 4,047 equal common files plus three helper additions; not recaptured.

Snapshot RED remains **one failing test with three subtest failures**, exit 1. Earlier GREEN and latest final are separate saved single-test passes.

**`lint-green.json` remains FAIL: exit 1**, revision `4053713c8`, I001 at `tools/test_build_host.py:3:1`, `Found 1 error.` The filename is not a pass. `lint-final.json` at `fda902ac2` is separate saved exit 0, `All checks passed!`. Formatter availability failure and all failed receipts remain retained unchanged.

## README and current wiring

Dockerfile:35 copies the helper before dependency fetching at :38; prefetch layer does not copy tests. `scripts/build-host.py:24` selects `crates`; :190–219 copies selected bytes; :251 invokes snapshot before `native.execute` at :259. Fixture defines seven concrete file outputs including three helper inputs. README correctly separates manifest availability from production compilation and limits fixture coverage, metadata, full-suite attribution and native/container acceptance. Linked docs retain the same distinction.

## Drift and custody

No tracked source/data drift relative to endpoint; nine current seals and all 51 private-original/copy seals agree. `fc49b4490→fda902ac2` changes only the Python fixture; `fda902ac2→547131525` adds only this 53-file evidence package. No follow-up Rust source change.

Concurrent documentation activity was observed: two initially dirty docs were subsequently committed in `da1220d383223d86a0810f1c47a5baba9a6a4a5c`; endpoint-to-current differences contain only four docs. Final tracked working diff is empty. Existing untracked `.code-index.db`, Python cache directories and `verification/` remain untouched. Whole-checkout immutability is not claimed.

**Historical custody remains PARTIAL.** Current readonly seals are not retroactive capture seals. Earlier snapshot/metadata receipts lack capture-time source/revision seals; Docker RED/GREEN used one mutable staging directory. Latest final fixture receipt does not repair earlier provenance. README full-suite reference is caller/prior context, not a rechecked suite result.

Actual Docker/native-host builds, dependency fetch, transport, runtime/profile/full-handoff acceptance: **NOT EXECUTED or proved**. No commands for build/test/lint/pycompile/metadata/full-suite, no binary hashes, operations or delegation. No new gates or parent-goal closure. Only the two requested report files were written. Readonly Git/file inspection only; initial Git cwd failures provided no proof.

Per-file hashes, exact saved small streams, latest receipt, current seals and Git observations: `independent-report.json`.
