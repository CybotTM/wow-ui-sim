# c490 five-producer and receiver runtime verification — PASS

Epoch: `$PRIVATE_VERIFICATION/pass2-five-green-current/20261013T030752959898Z`.
Compiled source: `c490d77f15dce96ba757ff2e1f0a24faeaf2ba9e`.
Runtime checkout: `2691111d5ab85328ebcc5dae70eb38303b0ce2d3`.

## Exact requested results

| Filter | PASS | FAIL | Ignored | Exit |
|---|---:|---:|---:|---:|
| item_stats_supplied:: | 9 | 0 | 0 | 0 |
| item_set_bonuses_supplied:: | 9 | 0 | 0 | 0 |
| ping_cooldown_interval:: | 6 | 0 | 0 | 0 |
| party_ai_membership:: | 9 | 0 | 0 | 0 |
| patch_10_2_5_secure_cvars:: | 4 | 0 | 0 | 0 |
| vertex_color_animation:: | 7 | 0 | 0 | 0 |
| texture_file_dimensions:: | 7 | 0 | 0 | 0 |
| method_diff_coverage::diff_methods_ | 2 | 0 | 0 | 0 |

Requested **53/53 PASS**. Five producers **37/37 PASS**. All four secure-CVar tests pass: three complete 7-name × 2-setter × 2-spelling matrices plus four historical controls = **88 matrix cases**. Their closures explicitly assert entry and genuine caller taint outside pcall; denials require the modeled combat-protection error and preserved reads/no event, accepted writes require committed reads/exactly one event.

Method selection deliberately excludes the ignored baseline-refresh helper; it was never executed. Zero selected-case skips, ignored/measured cases, zero selections, timeout exits, uncaught test-body errors, or fixture failures. Filtered-out tests are unselected scope, not executed-case skips. Individual ok lines reconcile with every summary.

## Bounded regression gap proof

| Bounded control | PASS | Exit |
|---|---:|---:|
| global setter → namespace read | 1 | 0 |
| setter commits before CVAR_UPDATE | 1 | 0 |
| historical protected-name boolean encoding | 1 | 0 |
| role target/focus represented identity | 1 | 0 |
| role player/party environment isolation | 1 | 0 |
| existing scroll/message method exclusions | 1 | 0 |
| non-VertexColor animation factory, template, timeline | 3 | 0 |
| lib shared metatable query without instance | 1 | 0 |

Regressions **10/10 PASS**; combined **63/63 PASS**, sixteen runtime invocations exit 0. Exact argv, environment, durations and summaries: `green-results.json`, `bounded-regression-results.json`, individual result files and `verification-proof-ledger.json`.

Scope chosen before execution in `bounded-regression-scope.json`: changed setter predicate, new party state/query sharing existing identity, changed per-widget method cloning and CreateAnimation receiver attachment. Prior full role13/scheduler18 groups not replayed; only two role identity/isolation cases intersect the shared state/identity boundary. Prior texture-atlas/setter proof not rerun because containment does not change those setters. Historical 080 vertex6 + role13 + scheduler18 proof remains historical; no duplicate full37 run.

Every runtime used `timeout 90`, `--test-threads=1`, `--nocapture`, repo cwd, `WOW_SIM_CASC=0`, `WOW_SIM_NO_SOUND=1`. No broad/all suite, independent startup, build, cargo fmt/check, service operation, baseline refresh, source/vendor edit, or delegation.

## Full output and assertion-boundary inspection

Complete stdout/stderr read for all sixteen invocations, retained individually and in `green-full-read-0..11.txt` / `regression-full-read-0..1.txt`. All stderr consists solely of normal bare-environment initialization timing records: no additional error/probe lines. These are test-environment logs, not a separate startup audit.

Passing bodies reach their nonconditional postconditions and unwrap/expect boundaries; exact individual assertion counts were not instrumented. Item validation distinguishes required string/number errors from permission denials. Genuine-secret denial helpers require the `untainted caller` class and preserve original wrappers/caller taint, rather than counting an arbitrary pcall error as proof. Party host secrets are rooted authentic VM wrappers and recovered after GC. Corrected cache consumers actually load and execute unchanged active-profile PingUI/CompactUnitFrame definitions; no cache lookup fixture failure. Ping timer delivery uses explicit controlled sinks and direct callback delivery, not native timer scheduling proof.

## Compile and source binding

All **742** Cargo stdout JSON records decoded and inspected: 666 compiler-artifact, 75 build-script-executed, one build-finished success:true; zero compiler-message records. Full compile stderr and saved journal/terminal read. Successful optimized test build: 10m10s. Six existing iced-wgpu-patched deprecated lint-name warnings remain; **not warning-free**. Saved systemd result: inactive/dead, Result=success, ExecMainStatus=0; journal wall 10min10.987s, CPU 18min34.322s, memory peak 10G, swap peak 7.5G. No service query/mutation performed.

Fixed c490→269 Rust/Cargo diff empty. Final HEAD remains 269; both c490→HEAD and c490→working-tree Rust/Cargo diffs empty, exit 0. Eight changed fixed-range files are AGENTS/docs only. Pre-existing untracked `.code-index.db` and `verification/` left untouched. Full binding evidence retained in source-binding files and ledger.

## New executable authentication only

Cargo emitted fresh:false integration and libtest executables. Copied each exactly once with exclusive `xb` destination creation into epoch/deps. Source stat stable across each copy; destination mode 0555. SHA-256 computed once during copy, never rehashed:

- integration-sealed: 469434728 bytes; `<DIGEST OMITTED>`.
- libtest-sealed: 391469032 bytes; `<DIGEST OMITTED>`.

Exclusive receipts: `integration-exclusive-receipt.json`, `wow_ui_sim-exclusive-receipt.json`; final destination stat identity matches both receipts. No old/new seal rehash or library/dependency digests. Main's existing wow-sim-sealed preserved without reading/hashing/copying it. Disk admission checked: about 260.6GB available versus 860.9MB new test executable bytes plus reserve.

Inspected src/cvars.rs storage guard: executable parent ends in deps → per-process/per-instance temporary CVar paths. Both test executables retain that placement; secure-CVar fixture enforces guard before constructing state. No user config/WTF mutation requested or performed; no comprehensive filesystem audit claimed.

Preparation-only incidents: initial Git calls lacked explicit cwd and failed; corrected read-only calls use repo cwd. A cross-evaluation Python local was unavailable; corrected self-contained evaluation. Epoch/deps already existed from main's wow-sim seal; initial exclusive directory assertion stopped before copying, then preserved directory and exclusively created only the two authorized new files. Discovery initially included the ignored refresh helper; count guard stopped before any runtime; corrected selection excludes helper. No product failures or test reruns resulted.

## Historical RED and baseline classification

Original 76cf33ef epoch remains untouched: **46 executed, 12 PASS, 33 behavioral FAIL, 1 cache fixture FAIL**, 34 exit101, zero timeouts. It is not overwritten or rewritten as later GREEN. Cache/error-class correction commit **346590852** remains distinct and precedes c490; corrected fixture PASS does not retroactively turn original fixture failure into behavioral RED.

Fixed `7ec9c049a^..2691111d5` docs/wow-client-diff diff contains **exactly two intentional default-fixture additions**, `Texture:GetFileHeight` and `Texture:GetFileWidth`. No other baseline row or native discovery capture changed in that scope; no refresh ran. Both current missing/extra comparison tests pass. Receiver/public-method graph consistency PASS is **not native parity**, and those two retained modeled Texture extras remain intentional native-capture differences.

## Concrete coverage limits

| Capability | Observed proof | Not established |
|---|---|---|
| Item stats/deltas | Exact supplied-link maps, zero/difference rules, freshness, isolation, authentication/validation: 9/9 | Native catalog/inference, cross-profile runtime |
| Item-set lists | Ordered duplicates, absent versus empty, signed32 selector policy, freshness, isolation, authentication: 9/9 | Native item/set data |
| Ping interval | Absolute supplied milliseconds, zero/fresh/isolation, real cached consumer: 6/6 | Native cooldown production/real timer lifecycle |
| Party AI | Explicit GUID supply intersecting active roster, aliases/removal/isolation, secret class, cached name consumer: 9/9 | Native AI population/classification, earlier profiles |
| Secure CVars | Both setters/cases, secure/insecure combat policy and event/storage atomicity: 4/4, 88 cases | Native defaults/error text, full historical policy list |
| Receiver containment | VertexColor endpoints/lifecycle 7/7; Texture metadata/getter lookup/WorldFrame control 7/7; comparison 2/2 | Native parity, GPU/CASC dimensions, opposite fresh WorldFrame cache order |

Runtime gate complete for bounded c490 scope only. Main independently owns current cargo check/startup and any broader acceptance.
