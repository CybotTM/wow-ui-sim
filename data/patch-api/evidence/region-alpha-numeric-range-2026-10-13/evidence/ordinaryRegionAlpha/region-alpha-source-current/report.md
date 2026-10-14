# Region alpha: bounded SOURCE/SPEC/readability audit

## Verdict and identity
PASS SOURCE ONLY. No in-scope implementation or changed-code readability violations found. Audit covers 135b03c13fc093d3c4211e403207ccca3419f62d and 71127a8e164c9ebeeee3439251984d08402483dc in <REPO>. All five changed files initially matched producer Git blobs exactly; shared Lua fixture also matched test commit. HEAD initially was producer commit. During retention, concurrent documentation commit 51508fde9aa3657b09ea806d5c0411a28dc0e07b updated spec execution-status text only; original audited spec SHA256 was 9d866fb845cd1aa2e39e82d711a1623805c4df34168f2c50b57276a06a064603. Current spec SHA256 retained separately. Other four audited files unchanged. Existing untracked .code-index.db and verification/ were untouched.

Not runtime GREEN. Saved RED metadata arrived during audit and was inspected at metadata level; not rerun. No implementation edits, build, Cargo invocation, behavioral test, compiler check, fmt, workloads, authentication, delegation, or public-payload reads. Scope deviation: inadvertent `git diff --check` whitespace-only inspection returned exit 0 despite check prohibition; not compiler/format/runtime proof. Initial Git calls failed outside repo, then corrected with explicit cwd.

## Concrete coverage matrix
References use repo-relative paths; alpha.rs/helpers.rs abbreviate src/lua_api/frame/methods/core_state/.

| Contract | Evidence | Proof |
|---|---|---|
| Retail/PTR ordinary numeric rejection before narrowing | alpha.rs:9-12 validates before opt_f32; :44-58 profile guard and raw Val::Num range check; helpers.rs:17-21 casts afterward | SOURCE PASS |
| NaN, infinities, f64 near-boundary outside [0,1] | Inclusive range rejects unordered/unbounded/out-of-range original numbers; dependency Val::Num is f64, not f32 | SOURCE PASS, not executed |
| No stored/effective/dirty mutation on rejection | Validator returns error before simulator borrows; setter writes at alpha.rs:36-40 only after validation | SOURCE PASS |
| Shared Frame/Texture/FontString setter | src/lua_api/frame/methods/core_state/mod.rs:39,81-89 registers SetAlpha; src/lua_api/env_init/frames.rs:56 wires registration; src/lua_api/methods.rs:296-320 attaches per-widget clone; src/lua_api/methods/frame_metatable.rs:83-111 does not filter SetAlpha | WIRED SOURCE PASS |
| Valid 0,.5,1 and no return values | Existing Ok(0)/no-op exits unchanged; tests/region_alpha_validation/ordinary_range.lua:6-12 authors 9 combinations/profile | SOURCE + authored fixture |
| Invalid values and preserved stored/effective alpha | Fixture :15-24 authors 7 values x 3 types, with .625 and pre-call effective alpha assertions | Authored, not runtime GREEN |
| Valid-value dirty/no-op behavior | Old setter no-op exits and propagation unchanged; tests/noop_hot_setters.rs:42-62 and :96 onward unchanged valid-value cases | SOURCE preservation |
| Invalid hot setter | tests/noop_hot_setters.rs:65-93 now distinguishes modern errors from earlier clamped success, retains zero visual/rect dirty assertions for SetAlpha(2) on alpha=1 | SOURCE + authored fixture |
| Earlier compatibility | alpha.rs:47-48 returns before validation outside Retail/PTR; original opt_f32/clamp/set path preserved | SOURCE only, not native history |
| Exclusions | Exact diff adds ordinary validator/call only; SetAlphaFromBoolean, secret/coercion behavior, vendor files unchanged | SOURCE PASS |

Dependency precision evidence: Cargo.toml:26 pins rilua 842e4d3ff8592af45a5a93058eddf93419d29012; cached source <HOME>/.cargo/git/checkouts/rilua-fd5a0715e46b5888/842e4d3/src/vm/value.rs:198 declares Num(f64). src/lua_bridge/from_stack.rs:42-49 reads the original stack value without narrowing.

## Fixture/wiring and limits
Rust tests/region_alpha_validation.rs:2,5-21 scopes Retail/PTR and executes the shared complete Lua fixture with valid/invalid mode. Actual CreateFrame/CreateTexture/CreateFontString and setters/getters are used; no monkey patches, replacing getters, or simulated rejection. Despite Cargo autotests=false, tests/integration.rs:1 invokes integration_tests!; crates/test-harness/build.rs:441-477 discovers top-level .rs modules and excludes neither affected module.

Upper `1 + 2^-30` exposes f64-to-f32 rounding to one. Lower `-2^-30` is near zero but not a negative-to-zero underflow fixture. Source covers ordinary f64 before casting, so this is a test-coverage limit, not observed code failure. Fixture checks target effective alpha, not every descendant immediately following failed parent mutation. No runtime result inferred from assertions.

## Spec grounding / RED evidence
Local data/patch-api/sources/10.0.0-api-changes.txt:14: “Region:SetAlpha() errors if the argument is not between 0.0 and 1.0.” Spec :10 explicitly calls earlier clamping simulator compatibility, not independently verified native history. No missing-catalog inference used.

region-alpha-numeric-red-current was initially absent, then became available during audit. Read its proof metadata and report summary; no originating runtime artifacts or binaries were opened, hashed, statted, or executed. Its report embeds retained diagnostic/launcher text; no external/public payload store read. Metadata records 24 original full artifact reads; this audit does not independently repeat those reads.

| Recorded run | Source revision | Exit | Boundary |
|---|---|---|---|
| Retail valid | cd9f42756472b8f8d706fd3e592f1977b3d89868 | 0 | completion marker; 9 combinations / 18 assertions by fixture flow; zero errors |
| Retail invalid | same | 1 | FIRST Frame -0.25 rejection assertion failed; 1 reached / 20 unreached; preservation assertions unreached |
| PTR valid | a5397a2d60ebdd2dc813309671bfb26534e3621c | 0 | completion marker; 9 combinations / 18 assertions by fixture flow; zero errors |
| PTR invalid | same | 1 | FIRST Frame -0.25 rejection assertion failed; 1 reached / 20 unreached; preservation assertions unreached |

Invalid metadata records pcall accepted and two diagnostics of the same failing case, not two independently tested inputs. Texture/FontString construction occurred; their invalid setup/calls/assertions were unreached. No post-invalid stored/effective value or legacy/native clamp semantics established.

Retained complete shared script body equals the actual Git-matching Lua fixture byte-for-byte, bridging the RED verifier's stated fixture-to-source gap. Metadata reports both source-to-test-commit alpha.rs diff-tree comparisons exit 0 with no changed path. Binary identity remains receipt metadata only; this is a METADATA audit of existing pre-producer full-Lua RED/valid baseline, not new runtime authentication or producer GREEN. Rust cases remain authored, uncompiled/unexecuted by this audit. Recorded artifact dates October 14, 2026 UTC are consistent with authored October 13, 2026 22:16:52 -0500; instruction date is October 13, 2026. No newly executed RED/GREEN, native parity, compiler/format completion claimed.

## Manual readability / artifact verification
rust-code-analysis-cli unavailable on PATH. Read every changed Rust line and affected setter/test context against rust-readability skill. No new readability violations: validator is a short profile guard then numeric/range guard; immutable validation precedes visible mutation. Existing setter length/duplicate no-op paths inherited, not newly introduced findings. No new warning suppressions, opaque chains, excessive nesting, overloaded primitive signatures, hidden mutation, TODO/FIXME/HACK/XXX, placeholder bodies, or commented-out implementation.

[EXIST] PASS: five changed files exist; digests/sizes/line counts retained.
[SUBSTANTIVE] PASS: concrete validator, real authored behavior assertions, scoped spec.
[WIRED] PASS: public registration, widget method retention, integration discovery inspected.
[ANTI-PATTERN] PASS: changed additions manually inspected; no listed new markers/stubs.

Findings are hypotheses, not adjacent tasks. No in-scope defects accepted; evidence/coverage limitations only. No implementation changes proposed or made. Reports alone written outside repo, mode 0600, file and directory fsync, complete byte-for-byte readback.
