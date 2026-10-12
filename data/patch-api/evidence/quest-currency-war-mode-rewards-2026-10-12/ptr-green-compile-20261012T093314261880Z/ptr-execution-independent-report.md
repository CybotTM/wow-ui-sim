# PTR precursor execution follow-up — BLOCKED / NOT EXECUTED

Scope: `ptr-green-compile-20261012T093314261880Z`; pinned test source `4e19a8bad30ffe039a7604f3dc0cff45334e46ee`, superseded by `a5eaf4fc6` after compilation (caller-supplied chronology). Precursor only, never current final acceptance.

## Reused compile and integration proof

Compile proof reused from `proof.json`: exit 0, source unchanged during compile, 741 stdout records (665 compiler artifacts, 75 build-script records, one successful build-finished); zero compiler messages; nine stderr lines with six manifest deprecation warnings. Integration authentication reused from `ptr-independent-auth-integration.json`: PASS, 467185280 bytes, SHA-256 `3fec92817fc9032a28fe8c2ae130a914534eba2c335f7e41f457544ddadf4ab1`. Integration binary not opened or hashed again.

## Earlier snapshot versus later terminal evidence

Earlier independent snapshot at 2026-10-12T09:43:32.322860+00:00 recorded admission pending, five rejected admissions, zero executed cases, and unavailable normal artifact (binary_stream_count 0). Preserved original receipts and reports unchanged.

Later root `precursor-controller-terminal-observation.json` records `ActiveState=active`, `SubState=exited`, `MainPID=0`, `Result=success`, `ExecMainStatus=0`, empty `ControlGroup=`. This proves controller termination only, not test execution. Complete controller stdout contains a serialized worker result with `type=error` and exact error `Error: runtime admission denied`. Controller stderr is empty. `runtime-blocked.json` records `RESOURCE_BLOCKED`, executed_cases 0. Runtime admission history now contains six rejected attempts, last at 2026-10-12T09:45:56.826374+00:00 (load1 47.88).

## Coverage and missing proof

| Claim | Evidence / verdict |
|---|---|
| PTR precursor compile | PASS; reused authenticated compile proof |
| PTR exact 13 tests including secret/vendor controls | NOT EXECUTED: no runtime submission, stdout, stderr, or outcome |
| Normal sealed artifact authentication | UNAVAILABLE: normal-artifact.json and wow-sim-sealed absent; zero streams |
| Normal `[]`, four raw getters, Default/type assertions | NOT EXECUTED: no normal submission, stdout, stderr, or outcome |
| Current final revision | NOT COVERED; precursor source superseded |

No runtime/normal streams exist to read. No 13/13 PASS or normal `[]` claim is admissible. New `ptr-independent-auth-normal-executed.json` records UNAVAILABLE, authenticated=false, executed=false; its requested filename is not an execution claim. Old `ptr-independent-auth-normal.json` remains unchanged. Full machine evidence: `ptr-execution-independent-proof.json`.

No builds, tests, replays, operational commands, questions, delegation, or binary reads/hashes performed.
