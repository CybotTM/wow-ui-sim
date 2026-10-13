# Saved actual PTR audit — PASS within scope

Evidence: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/ptr-20261013T123925051908Z`

## Identity and compile

Recorded source `7fbcd5e475b3c16ae24a5a247ac3949bc4d48b27` (docs-only over producer `50f6f0ea8`). Saved revision/runtime-input records agree. Exact read-only filesystem snapshot mount is `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/ptr-20261013T123925051908Z/source` → `<REPO>`. No source/archive hashing or independent ancestry authentication; this is recorded provenance, not a new content seal.

Entire compile JSON parsed: 741 records (665 compiler-artifact, 75 build-script-executed, one build-finished success=true); zero compiler-message records. Full terminal: Result=success, SubState=exited, ExecMainStatus=0. Full compile stderr inspected: six inherited iced-wgpu-patched manifest deprecations plus one summary line, package-cache lock waits, and successful finish; not warning-free. Deprecated names: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity. Saved toolchain: Arch Cargo/rustc 1.99.0.

Both selected artifact records match the compile JSON exactly once and contain `client-ptr`, NOT `client-retail`; retail-12-* entries are API-epoch features, not the retail client profile. Saved seals/receipts agree on paths, digests and before/after stats; receipts PASS. Current device/inode/size/mtime stats still match both seals. No new hash or binary/library content read. Digest authenticity credited only to saved receipts.

Compile argv argc=33; four integration launcher argv argc=55; normal startup argc=57. All saved argv and six admission JSON records inspected; each admission allowed=true. Runtime argv selects sealed integration or wow-sim, exact snapshot read-only bind, isolated network, 86s timeout, 4G/3G memory and 200% CPU. These are saved execution conditions, not operations performed during this audit.

Actual cache evidence: `[discrete-cache] profile=ptr path=<HOME>/.cache/wow-ui-sim/blizzard-ui/ptr/AddOns`; bars/HUD also print that PTR AddOns path.

## Exact counts

| Parent group | Passed | Failed | Child summaries excluded | Filtered out |
|---|---:|---:|---:|---:|
| discrete | 7 | 0 | 1 | 10144 |
| bars-consumer | 1 | 0 | 1 | 10150 |
| hud-consumers | 2 | 0 | 2 | 10149 |
| core-lifecycle | 2 | 0 | 2 | 10149 |

**12 unique parent tests PASS = 7 new discrete + 5 preservation.** Six child summaries repeat parent cases and add zero test credit. Each parent has zero ignored/measured; all launchers exit 0. Previous PTR26 and consumer3 add zero credit. Normal startup is one separate process, not a thirteenth test. Exact names and final parent summaries in proof.json.

## Bounded coverage matrix

| Coverage | Saved proof | Result |
|---|---|---|
| Query/full19 | Three query cases + complete 19-field typed fixture, exact-key checks, snapshot freshness, mutation isolation, replace/remove, numeric/nonnumeric and environment ownership | PASS |
| Secrets | Actual host-secret NUMBER inputs; untainted/tainted supplied and absent lookups; public snapshots and caller recovery | PASS |
| Feed | Two cases: type19 record/membership before synchronous events, replace, retain record after membership removal, reject unknown set/type mismatch without side effects | PASS |
| Cached lifecycle | One actual PTR-cache consumer: acquire/update/hide/restore, record/membership removal, unregister and no reacquisition, OnUpdate pool/layout counts | PASS |
| Atlas application + paused cooldown | Actual cached consumer asserts catalog/applied names, dimensions, marker positions/shown states, paused duration/remaining values | PASS, state only |
| Preservation | Bars1 + HUD2 + core lifecycle2 | PASS, five cases |
| Fresh normal empty state | One wow-sim process, publication function and exactly one nil result; stdout [] and entire stderr CLEAN | PASS |

Saved-source comments saying later assertions are unreached describe the previous RED boundary; current saved parent and child PASS outcomes cover these cases. No claim of the entire integration parent suite (10151 available cases) follows from the selected filters.

## Entire runtime diagnostics

All five stderr streams fully read: discrete 5833 bytes/100 lines; bars 728/12; HUD 1455/24; core 1280/22; normal 14699/219. No actual runtime error, warning, panic, or AnimSetup diagnostic observed. Normal keyword matches are cache counters replay_fail=0/store_fail=0, seterrorhandler-restored messages and the final zero-error summary, not failures.

Fresh normal stdout, verbatim:

```text
DISCRETE_EMPTY_STATE	function	1	nil
[]
```

Entire normal stderr ends:

```text
=== Final observed Lua error summary ===
Status: CLEAN
Lua errors: 0 unique, 0 occurrence(s)
Attributed owners: 0
Unattributed Lua errors: 0 occurrence(s)
```

Normal argv uses --no-addons, --no-saved-vars, CASC=0 and sound disabled. Stderr records 283 Blizzard addons and read-only WTF EditMode cache. This is the saved normal loader path under these conditions, not all-addon/native/rendered acceptance.

## Exclusions and audit boundaries

No rendered/native/tutorial/other-profile/whole-parent claim. HelpTip function presence is only a cached-consumer prerequisite, not tutorial behavior. Atlas names and cooldown state do not prove pixels or native parity.

No runtime reruns, builds, checks, new hashes, binary/library reads, delegation, operational actions or tracked edits performed. Only requested untracked report artifacts written. Entire compile JSON/terminal/stderr, runtime stdout/stderr, argv, admissions, selected seals/receipts and relevant saved test source inspected.

Outputs:

- `<REPO>/verification/widget-discrete-ptr/report.md`
- `<REPO>/verification/widget-discrete-ptr/proof.json`
