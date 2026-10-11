# Mists corrected-parent follow-up — two bounded cases

Revision `95e215f780678d0672e0851687b31edf8cf8edbe`, epoch `20261011T171434Z`, includes parent-expectation correction `83acf239e`. [Earlier failure packet](../README.md) remains unchanged; [current contract](../../../../../docs/wiki/investigations/mists-cast-startup-visibility.md#current-fixture-phase--2026-10-11-unverified).

| Behavior | Result | Evidence |
|---|---|---|
| Exact original prefork cast-bar case, settled fade and corrected retained-parent contract | 1 PASS / 0 FAIL; exit0 | [stdout](original.stdout), [execution](execution-results.json) |
| Integration idle source-fade control | 1 unique PASS / 0 FAIL; exit0 | [stdout](fade-control.stdout), [trace](fade-control.stderr) |

Original case reaches hidden state after normal0.55s OnUpdate, requires parent and layoutParent equal to `UIParentBottomManagedFrameContainer` and absent showingFrames membership, then passes strict unlocked player-anchor and locked cast-anchor transitions, ending attached to `PlayerFrame`. Fade control retains shown/playing at0.25s, then hidden/not-playing at0.55s. Its nested timeout summaries count once: combined **two unique executions**, not three.

Independent saved-artifact auditor2262 authenticated Cargo target/features, corrected-parent ancestry, both sealed executable hashes and behavioral source endpoints against saved before/after values. [Artifact identities](artifacts.json) and [compile submission](compile-submission.json) retain the original receipts. Worker source sets `PREFORK_EXACT_FIXTURE_GROUP=cast-bar`; the execution environment was not separately captured. Vendor-cache historical hashes/provenance were not retained, so exact vendor identity at execution is not independently authenticated.

Raw [outcome](outcome.json) incorrectly says original assertions unchanged: one parent expectation was explicitly corrected; later lock/anchor assertions stayed strict. Raw receipt is preserved, not rewritten. Startup stderr still contains self-anchor errors and an early systemInfo READ_ERROR; these case passes do not establish zero startup Lua errors, full Mists phase, native timing parity or full handoff acceptance. No production/vendor reparenting, Hide or timing patch.

[Retention manifest](retention-manifest.json): ten exact private metadata/stream copies,33,908bytes; no executables or vendor Lua copied. Subsequent Retail-gated Garrison import repair `7635fef91` is outside this saved revision and does not add Mists runtime proof.
