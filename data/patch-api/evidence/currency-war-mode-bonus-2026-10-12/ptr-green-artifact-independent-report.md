# PTR GREEN independent artifact proof

Epoch: `ptr-green-compile-20261012T080604474078Z`  
Producer: `ce2c37ad5623642bec418733ac363600ea3e7b63`

## Authentication — PASS

Each new seal was SHA-256 authenticated exactly once. Digest/size comparisons were persisted immediately per seal, before reporting:

| Seal | Bytes | SHA-256 | Match |
|---|---:|---|---|
| `integration-sealed` | 467474048 | `9ba40034c051ad4a29cfc45937271e83c39148bf746fcba26aadcb26ad4148a6` | PASS |
| `wow-sim-sealed` | 414537496 | `fd8c3c13321f6c76a98b770fb07e261d216881aa58c7473d7643866587b263c2` | PASS |

Authentication files: `ptr-green-independent-auth-integration.json`, `ptr-green-independent-auth-normal.json`.

## Compile/profile — recorded PASS, not execution

`compile-result.json`: exit 0, source unchanged. Producer revision matches `ce2c37ad5`. Both manifests' full Cargo records match exactly one corresponding record in `compile.stdout`. Both have `client-ptr` (no other client feature) and `retail-12-1-5`; no profile inferred from filenames. Build-finished success recorded. Six deprecated iced_wgpu manifest lint-name warnings remain in the complete compile stderr.

Normal seal timestamp: `2026-10-12T08:10:12.874169+00:00`. Recorded boundary: **before default publication compile submission, while original PTR runtime remained admission-blocked**. This is recorded producer provenance, not a new observation of the publication process.

## Runtime/normal — PENDING

Single observed inventory contained no runtime submission/outcome/stdout/stderr and no normal submission/outcome/stdout/stderr. Original runtime admission was denied at `2026-10-12T08:08:44.210607+00:00` (load 49.98). Deferred attempts 1–3 also denied; latest observed attempt at `2026-10-12T08:12:20.725313+00:00` (load 31.59). Recorded outcome: `COMPILED_NOT_EXECUTED`, 0 executed cases.

No actual case counts beyond recorded zero, normal `[]`, completed script assertions, or error-route outputs exist in that snapshot. Queued controller/request text is not execution proof. No wait, replay, later discovery, or polling performed.

## Coverage boundary

| Claim | Status | Proof level |
|---|---|---|
| Getter | PENDING | Artifact/compile only; no runtime assertions |
| Native | PENDING | No native-client behavioral proof |
| QuestUtils | PENDING | No positive normal reward-path output; supplied-state scope cannot establish membership |
| Full goal | PENDING | No runtime/end-to-end completion proof |

Verifier executed no tests, builds, sealed binaries, operations, delegation, polling, or network access. Retail/RED artifacts were not rehashed. Only the requested authentication/report/proof files were written. **Zero execution claim.**
