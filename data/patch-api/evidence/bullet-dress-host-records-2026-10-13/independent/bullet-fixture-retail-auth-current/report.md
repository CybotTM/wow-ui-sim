# PASS — Retail compile and sealed executable authentication

Runtime GREEN **UNPROVED**. Compile-only evidence does not establish corrected test outcomes.

## Exact paths

- Epoch: `<epochs>/bullet-fixture-retail-green-retry1-20261014T005621584814Z`
- Seal: `<epochs>/bullet-fixture-retail-green-retry1-20261014T005621584814Z/seal-integration.json`
- New authenticated copy: `<epochs>/bullet-fixture-retail-green-retry1-20261014T005621584814Z/artifacts/deps/integration-9f07e4a55f2bbbf7`
- Receipt: `<epochs>/bullet-fixture-retail-green-retry1-20261014T005621584814Z/receipt-integration.json`
- Report: `<verification>/bullet-fixture-retail-auth-current/report.md`
- Proof: `<verification>/bullet-fixture-retail-auth-current/proof.json`

## Compile evidence

Complete `<epochs>/bullet-fixture-retail-green-retry1-20261014T005621584814Z/compile.stdout` parsed: **741 JSON records**, 665 compiler artifacts, 75 build-script records, one `build-finished` with `success: true`. **Zero compiler-message diagnostics**. Integration record is newly built (`fresh: false`) with Retail features matching seal.

Complete `<epochs>/bullet-fixture-retail-green-retry1-20261014T005621584814Z/compile.stderr` read: **six manifest deprecations**, all from `iced-wgpu-patched/Cargo.toml`; no source compiler diagnostics. Cargo reports **1m 13s**. Warnings are not hidden or counted as source diagnostics.

Complete `<epochs>/bullet-fixture-retail-green-retry1-20261014T005621584814Z/compile-terminal.txt` read: `Result=success`, `ExecMainStatus=0`. Start **October 13, 2026 19:56:22 CDT**, exit **October 13, 2026 19:57:36 CDT** (October 14 UTC). Peak **4,835,811,328 bytes**. These are recorded terminal facts; no service query or mutation performed.

## Exactly-once authentication

Receipt absent before authentication. One binary open/read and one SHA-256 computation on **only the new sealed copy**. Digest `ecb77c89c2f7b4f8538bede216d82651a5f1382517f333f71f0d9c02b446c151` matches seal. Path and open-file pre/post stats all equal seal: device 66306, inode 148117929, size 471369688, mtime_ns 1791939455793969784.

Receipt records `status: PASS`, `sha256_matches_seal: true`, `hash_reads: 1`, path, pre/post stats and digest. Exclusive creation; mode **0600**; file and directory fsync; JSON readback equality confirmed. No second binary hash performed.

## One-line source contract and scoped readability

Parent `69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c` → `bde7ea277e86f9ad17f7135b3cd676f80e73057b`: one file, one insertion/deletion, `tests/widget_bullet/fixtures.rs:78`: `env.eval::<()>(code)` → `env.exec(code)`.

Inspected selected source copy `<epochs>/bullet-fixture-retail-green-retry1-20261014T005621584814Z/source/tests/widget_bullet/fixtures.rs`. `WowLuaEnv::exec` (`source/src/lua_api/env.rs:123-125`) calls `exec_rilua`; that loads code unchanged and calls it (`source/src/lua_api/env_rilua.rs:23-26,37-38`). `eval` normalizes apparent single-line expressions and prefixes `return` (`source/src/lua_api/env.rs:140-184`). Actual helper input `bulletRejectedEvents = 0` (`source/tests/widget_bullet/feed.rs:119-123`) is a statement, not an expression. Corrected helper matches statement execution contract. Existing phase-context errors and Lua-error assertion retained.

Manual review limited to changed line and immediate helper: no readability violations found, no numerical metrics claimed, no broad audit or runtime inference.

## Proof boundary

No build/test/check/format reruns; no services, delegation, Bash or cwd-tool calls. No normal target executable, previous binary, library, source archive or source-tree authentication. Source inspection is not cryptographic source provenance. Historical producer **6 PASS / 1 fixture FAIL** remains historical; corrected runtime acceptance remains **UNPROVED**.
