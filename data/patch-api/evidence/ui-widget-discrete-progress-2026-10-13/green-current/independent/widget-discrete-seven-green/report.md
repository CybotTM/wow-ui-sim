# Discrete Progress Steps — saved-evidence verification

**PASS: 7 unique tests, 0 failures** at `50f6f0ea894d0bbe2f281887ef36d25e0bbbf313`. **Parent OPEN.**

Repository: `<REPO>`  
Evidence: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z`

| Scope | Runtime result |
|---|---|
| Queries | 3 PASS |
| Security | 1 PASS |
| Typed host feed | 2 PASS |
| Actual Retail cached consumer | 1 PASS |

Child consumer `1 passed` is a duplicate, excluded. Parent output: `7 passed; 0 failed; 0 ignored; 0 measured; 10967 filtered out; finished in 6.21s`. Saved wait launcher: `Finished with result: success`, `status=0/SUCCESS`; service runtime 7.346s. Full runtime argv/admission/stdout/stderr/launcher/result read; admission allowed. Stderr names actual Retail cache `<HOME>/.cache/wow-ui-sim/blizzard-ui/retail/AddOns`. Runtime uses read-only snapshot/cache, isolated networking and typed feed before cached registration.

## Assertion reachability

Saved `source/tests/widget_discrete/consumer.rs:265-279` runs initial supplied query, cached loading, registration/geometry, update/hide/restore, record removal/membership removal/unregister in sequence. Successful child and parent consumer results prove the non-conditional assertions completed: pool/layout counts; dimensions and rotated anchor; marker/completion visibility; catalog lookup and applied atlas names; paused cooldown and gap-adjusted timing; hidden release/animation stop; restored geometry; record and membership removal; subscription removal and no post-unregister reacquisition. Consumer path introduces no vendor patches/overrides; saved runtime cache is read-only. Not a general vendor-integrity audit.

**Actual atlas application and paused cooldown state proven. Render/native/tutorial acceptance not claimed.** HelpTip presence is only a prerequisite.

## Compile/provenance

Full compile JSON read: 741 records, 665 compiler artifacts, 75 build-script records, 1 successful build-finished, 0 compiler-message records. Full terminal/stderr read; saved exit status 0. Stderr contains **6 manifest deprecation warnings** in `iced-wgpu-patched/Cargo.toml`: compile passed, not warning-free. Selected integration and wow-sim compiler records each match seal metadata exactly once. Saved authentication receipts PASS; current stat-only device/inode/size/mtime match seal and both receipt stats. No binary/library contents read or new digests computed. Selected wow-sim metadata does not prove startup.

## Separate pending gates

Normal startup, preservation, PTR and check remain pending. Parent goal remains OPEN. No reruns/builds/checks/delegation/runtime operations/tracked edits. Machine-readable proof: `<REPO>/verification/widget-discrete-seven-green/proof.json`.
