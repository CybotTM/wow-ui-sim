# PTR compile — independent proof

**PASS: compile receipts and both sealed artifacts authenticated. Runtime GREEN not established.**

Epoch `ptr-green-20261012T123249639935Z`. Audit snapshot `2026-10-12T12:37:35.244097+00:00`. No build, executable/runtime launch, delegation, repo edit, synchronous wait or polling. All earlier proof preserved. One failed evaluation referenced unavailable `root`; it produced no evidence or writes.

## Authentication

Each seal read and SHA-256 hashed **once**, compared with its corresponding artifact manifest, and its independent receipt immediately created exclusively, flushed and fsynced before the next seal. Byte counts, digest, manifest path and pre/post inode/size/mtime/ctime matched. Receipts are reusable; do not rehash on future verification.

| Seal | Bytes | SHA-256 | Receipt |
|---|---:|---|---|
| integration | 467661672 | `21a2432227267c3705b9ee0bbf7de6df574829d115db55c646ccf2bdafa0f51b` | `independent-auth-integration.json` |
| normal | 414744392 | `600266f11167f436a3d19531c2a91c41417a191f96b3bc12fb1e3ccbc6c81810` | `independent-auth-normal.json` |

## Compiler/toolchain audit

Saved command: `/usr/bin/cargo test --test integration --no-run --message-format=json --offline --locked -j 12 --no-default-features --features sound,gui,casc,client-ptr`, cwd `/home/osso/Projects/wow/wow-ui-sim`, empty explicit env overrides. Result exit **0**. Full JSON stream parsed: **741 records**, comprising **665 compiler-artifact**, **75 build-script-executed**, **1 build-finished success=true**; zero compiler-message records. Each manifest's complete Cargo record occurs exactly once in that stream. Integration target is a test executable, `profile.test=true`; normal target is `wow-sim` bin, `profile.test=false`. Both fresh=false and client-ptr present/client-retail absent. No test execution inferred from this `--no-run` compile.

Saved version commands each exit0, stderr empty: Cargo **1.99.0** (`5f94df478`, 2026-08-27, Arch Linux rust 1:1.99.0-1); rustc **1.99.0** (`b940084d7eb6a299eb4bfeb8e34901bc051e7ac4`, 2026-09-28), host `x86_64-unknown-linux-gnu`, LLVM **23.1.1**. These are independently audited saved version receipts, not newly executed or executable-authenticated toolchain evidence.

Full compiler stderr records completion in **1m47s** and **six manifest lint-key deprecation warnings** in `iced-wgpu-patched/Cargo.toml`: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity. Compile PASS is **not warning-free**, fmt/check PASS, or runtime PASS. No warning remediation performed.

## Source applicability and reuse

Recorded before/after hashes match **9/9**; independent current hashes also match **9/9**: four producer/module files, three tests, Cargo.toml and Cargo.lock. Saved applicability selects `6ec0c1cb8` and records empty src/tests/Cargo diff. Current read-only Git confirms no src/tests/Cargo drift from fixture revision, and no production/dependency delta between `be9c734a4` and `6ec0c1cb8`. HEAD is now `05fddbf399640fdeac345e25a1dcae9a0541cacd`, **not** either earlier target. Spec differs only in introductory progress narrative. Existing untracked `.code-index.db` and `verification/` preserved.

Read `source-proof.json`, source readability report, secret-fixture delta proof/report and saved delta Git proof. Reuse **source-only** be9 producer conformance/readability and **source-only** 6ec fixture contract-retention/readability. The fixture removed two `canaccessvalue` helper predicates, not exact-getter assertions; genuine secret identity, exact fields/type/arity, wrapped nonstring rejection, tainted-secret rejection, ordinary tainted acceptance and taint restoration remain source obligations. Helper/native parity stays separate and open. Earlier test/HEAD identity and first-GREEN 4PASS/1FAIL are historical, not current PTR execution. Nine-path before/after coverage does not attest all dependency/build inputs or every transient state.

## Admission and capability coverage

Compile admitted at **12:32:49.646813 UTC**: available33.5128GiB, load1=3.37, no relevant cargo. Three saved runtime observations deny admission at12:34:40,12:35:13,12:36:15 UTC (load1=24.54,27.37,29.09). Independent single current snapshot at **12:37:35.537098 UTC**: available**19.9034GiB**, load1=**31.25**, no relevant cargo or unreadable cargo cwd. Worker predicate requires available>=6GiB, load1<24, no relevant cargo: **not admitted due to load**. Snapshot observes capacity only; does not launch/authorize execution or declare terminal resource blocking.

| Capability | Evidence / status | Proof level |
|---|---|---|
| PTR integration/normal compilation | exit0, successful build-finished, matching Cargo records | Saved compile receipt + independent seal authentication |
| Source/fixture preservation | 9/9 before/after/current; empty source/test/Cargo diff | Source only; earlier reports reused within unchanged scope |
| Exact getter, security, live state, isolation, misses, popup, alias | Planned exact group5 cases; no runtime result files at snapshot | **0 cases credited; pending/unreached** |
| Legacy/state compatibility | Planned legacy2 + seeded1 cases; no result files | **0 cases credited; pending/unreached** |
| Normal loaded startup/publication | No normal execution result | **Not established** |

No `runtime-admission.json`, runtime results, outcome, failure or per-group result files observed in this epoch at audit snapshot. Runtime may proceed later; this report does not wait for it or credit it. No full acceptance, native parity, new fmt/check, security or runtime GREEN claim.

## Proof ledger

`ptr-compile-independent-audit.json` retains exact five Git argv/stdout/stderr/exits, all0; complete compiler diagnostics/stderr, record counts and matching target metadata; all source hashes; saved toolchain records; admission snapshot/history. `ptr-compile-independent-proof.json` records scoped verdict and explicit unproven gates. Authentication receipts retain exact digests and pre/post stats. This proof applies only to authenticated seals and observed source/admission scope; later source/admission changes require scope reassessment, **not seal rehashing when receipts exist**.
