# Bounded sealed runtime verification

Scope: epoch 20261013T070941017540Z; sealed integration compiled_revision 11131c724, production_revision 0c947a298; staged test revision fcb0b7823. No Cargo, build, check, rustfmt, tracked edits, delegation, startup execution, or production/service changes. Only requested transient resource-limited test units were created.

## Artifact authentication
SHA-256 authenticated exactly once: 11299965398d177f1a9e5dc7d88cfc7426f4de0bde8a16f8b81d9bc2b56d8641. Size 469870624, inode 139891302, mtime_ns 1791875514246048231 all match NEW seal. Receipt persisted immediately as runtime-authentication-receipt.json; subsequent runs reused receipt and stat metadata, not hashing. Compile producer record exactly equals seal producer. git diff 0c947a298 11131c724 -- src tests is empty, exit0; fcb0b7823 ancestor proof exit0.

## Runtime proof
| Filter | Result | Exit | Harness time | Unit memory peak |
|---|---|---|---|---|
| path_control_point:: | 6 PASS, 0 FAIL, 0 ignored, 0 measured, 10943 filtered | 0 | 0.78s | 914M |
| animation_query_lifecycle:: | 6 PASS, 0 FAIL, 0 ignored, 0 measured, 10943 filtered | 0 | 0.42s | 65.8M |

Separate fresh admissions retained in path-runtime-admission.json and lifecycle-runtime-admission.json. Conservative credit only half inactive file cache after subtracting dirty/writeback/shmem; no slab credit. Projected memory 31843725312 and 34398879744 bytes, below slice high 42949672960 and max 45097156608. avg10 PSI below 1% on both admissions. Each fresh unit: agents.slice, MemoryMax=4G, MemoryHigh=3G, CPUQuota=200%, timeout 88s plus 2s kill grace, root read-only bwrap, private <TMP>, preserved HOME, private network. Exact argv, full stdout/stderr, exit codes retained separately. No test reruns.

## Requirements/source audit at 0c947a298
| Coverage | Evidence/proof level |
|---|---|
| Supplied point orders, offsets, mutation and ascending multi-return; NONE round-trip, named XML loading, equal-time segments | Four existing Path behavioral cases PASS; source-path.rs contains public methods, source-path_runtime.rs uses ordered supplied points and normalized equal-time segments. |
| SMOOTH round-trip, unchanged cached Path XML reaches both points; child-only Catmull-Rom displacement, endpoints duplicated | Two added fcb0b7823 cases PASS; setter permits NONE/SMOOTH; runtime neighbors use saturating/min endpoint selection. Samples (4.375,5.625), (10,10), (15.625,5.625) and cached XML (-27.1875,36.09375) are asserted actual layout outputs, not source-shape tests. |
| Saved baseline, Stop/finish restoration, replay with changed baseline; owner/sibling/dimensions isolation | Behavioral PASS. SMOOTH stop baseline is zero; changed nonzero baseline/replay tested on NONE, shared restore code source-inspected. |
| Unknown curve rejection; hidden-subtree restoration | Unknown setter token explicit error at path.rs:155-156; SOURCE ONLY, no negative-token test in these six. Hidden-subtree hook SOURCE ONLY; no behavioral acceptance claimed. |
| Animation query regression | Six lifecycle cases PASS: pause/resume/restart/stop, pending finish callbacks, reverse/repeat, bounce, delayed elapsed and parallel ordered timing. These are Alpha query tests, not Path reverse/loop parity. |

Wiring: creation.rs:105 attaches Path methods; play.rs:55 and runtime.rs:37/:72 apply motion/restoration; animations.rs:225 invokes shared motion. Exact revision grep retained in path-wiring.txt.

Motion is simulator policy, NOT native/historical parity. Cached XML fixture is external runtime cache read by sealed executable, not itself sealed by executable hash. Test extracts actual Path element unchanged into minimal normal TOC/XML fixture; does not load complete normal Blizzard startup.

## Manual changed-Rust readability audit
Read skill completely; inspected production path.rs/path_runtime.rs changed lines and all changed tests/path_control_point.rs lines from fcb0b7823. No automated lint/check/formatter invoked.

[LENGTH] src/lua_api/frame/methods/button_anchor_hierarchy/animations/path_runtime.rs:71 — interpolate_path()
Problem: 33 body lines exceeds skill non-test threshold of 30; function combines segment selection and curve sampling.
Fix suggestion only: extract curve sampling into a named helper. No edits authorized or made.

No further changed-line violations identified. Catmull-Rom coefficient/neighbor notation is conventional mathematical notation; concrete geometry literals in tests are behavioral expected values. Test bodies remain below 200-line threshold. No new warning suppressions or TODO/FIXME/HACK/XXX placeholders in audited Rust additions.

## Retained producer diagnostics
Read all 741 compile stdout JSON records; 0 compiler-message records; build-finished success=true; exactly one matching integration producer. Existing terminal status ExecMainStatus=0. Existing compile.stderr has six inherited iced-wgpu-patched manifest deprecation warnings: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity. Then aggregate generated-6-warnings line and successful test profile completion (2m13s). Not warning-clean. Producer diagnostics retained, not rerun.

## Verification setup failures (not behavioral test failures)
Four initial git inspection commands (status --short; show --stat 0c947a298; show --stat 11131c724; log -5 --oneline) omitted explicit Pyrun cwd and exited128: fatal not a git repository. Reissued with required repo cwd, exit0. First follow-up evaluation failed NameError Path; next failed missing ctx.repo after failed evaluation; corrected with explicit local variables. First runtime preparation failed invalid st_inode attribute; corrected to st_ino before any test executable invocation. Initial source git show omitted frame/ in both path.rs/path_runtime.rs, exit128; corrected revision paths succeeded. These caused no extra authentication or test executions.

## Scope boundaries
No fresh normal startup binary supplied or run. NO startup-clean claim; retained old 12 SMOOTH errors remain historical pre-fix evidence until separately verified. No native curve parity, full-suite/profile, GPU, hidden-subtree, unsupported-order, nonunit-scale, competing-animation or broad acceptance claims.
Full production, staged-test and cumulative source diffs saved; exact proof commands/outputs/exits retained. Concurrent worktree changed from initial widget scaffolding dirt to only untracked .code-index.db and verification/ at final observation; verifier made no tracked writes and did not commit others' work. final-head.txt records observed final HEAD independently of sealed scope.

