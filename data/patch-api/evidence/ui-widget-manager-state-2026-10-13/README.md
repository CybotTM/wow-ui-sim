# UI widget manager: bounded public evidence

Retained evidence only, 2026-10-13. No new execution, build, check, test, authentication, commit, or independent gate. Source roots are private verification epochs identified in manifest; payloads are complete small text artifacts, not excerpts. No source archives, binaries, full Cargo JSON output, or private identity mapping included.

## Coverage and limits

| Revision / epoch | Observed evidence | Claim limit |
|---|---|---|
| d119 / state-red | Existing state RED: five failures | Retained original failure evidence; not rerun |
| 6a852c322 / lifecycle-red | Actual cached registration RED: one failure; security RED: one failure | Registration fails before later lifecycle assertions; security recovery/isolation unproved |
| 67775404d / state-green | Core 23/24 pass: state 6, security 1, button header 9, path 6, prey 1; lifecycle 0/1. Startup zero Lua errors | Epoch name does not mean overall GREEN. Lifecycle phase label says unregister, but failure occurs before unregister; retained report wording overstates location |
| 96a44bb1f / unit-red | Real OnEvent target-preservation fails; player assertion reached; no missing SetProcessingUnit failure | Actual target text not printed. Injected attachedUnit fixture is not native unit registration |
| 3a / cleanup-diagnostic | Diagnostic failure: restore showing count=1 | Label diagnostic only; not new lifecycle pass or unregister-cleanup proof |

Latest scoped evidence: [e8879ea74 final-delta report](finaldelta-green/report.json), [fresh startup terminal inspection](finaldelta-green/lua-errors-terminal-inspection.txt), and [lifecycle stdout](finaldelta-green/integration.stdout). Actual **2/2 lifecycle PASS**, exit0, covers cleanup, timers/nested-row-widget pool release, manager/event unregister, post-unregister no reacquisition, OnUpdate(0.0) fixture flush, and modeled real OnEvent player refresh/target preservation using explicit attachedUnit input. Fresh normal startup: **zero unique/occurring Lua errors, zero warnings, zero AnimSetup diagnostics**, exit0. Compile exit0 retains six inherited manifest deprecations; startup cleanliness is not compiler warning-free proof.

Relevant prior **23 PASS at 67775404d** remain separate reused evidence, not rerun/revalidated at e887. Together these are **25 unique latest logical scoped PASS**, not all25 executed at e887. Original **23/24 FAIL** remains unchanged; its lifecycle label/count failure occurs **before Unregister**. Double timeout-child summaries count one test, not two. Model unit-notification testing is not native unit registration/unregistration or unit data isolation: read-only actual WTF EditMode cache access occurred despite no SavedVariables. Native parity and broader completion remain unknown/unclaimed.

## Authentication and controller deviations

Lifecycle RED authentication required two hash reads after a lost receipt; retained report says missing-output recovery, not exactly-once authentication. Historical seals/receipts are copied as text; no old executable rehash performed for this packet.

67775404d state launcher used --wait with RemainAfterExit=yes and blocked after test completion. Main retained exact process terminal in state-green/runtime-gate/state.main-retained-terminal.txt; completed service was stopped to rescue controller without test reexecution. Controller recovery and original launch text retained. Exact terminal text is authoritative; packet does not convert controller recovery into another test run.

## Provenance and transformations

manifest.json lists every copied payload, original byte size and text-file SHA-256, transformed size and SHA-256. Original digests describe private text bytes, not executable authentication. Home/repository paths and temporary paths use consistent placeholders; actual account/realm/player segments discovered from actual EditMode cache source startup lines (and WTF paths when present) are replaced consistently across all payloads. Private mapping is not published. Every retained text is UTF-8, CRLF/CR normalized to LF, with terminal newline run replaced by exactly one LF; empty text becomes LF. No truncation or line omission. JSON/text receipts retain historical field names and claims; redactions do not establish those claims independently.

Scope excludes large compilation output, source snapshots, binaries and absent native observations. finaldelta-green appends 14 complete small text files: scoped report, both stdout/stderr/argv/terminal records, two authentication receipts, compile.stderr, terminal-systemd and artifact-seals. Existing executable hash strings are retained only; no binary reads, hashes, new authentication or execution. Original public reports and frozen query evidence remain unchanged; latest startup/scoped-report references above supersede older startup pointers, not their historical contents. No commit, delegation, build/test or broader handoff acceptance performed.

## Bar/HUD RED append

[retention-manifest.json](retention-manifest.json) records **70 new complete text payloads** under [bar-hud-red/](bar-hud-red/), with original/public SHA-256, byte sizes, and transformations. Existing manifest.json and its **107 payloads** are untouched and not rehashed. New selected text totals **72,959 original bytes / 69,938 public bytes**. Private epoch: `ui-widget-bars-hud-red-current/20261013T083600769136Z`, source **87fecfd31**. Current source **0745** is producer implementation; **GREEN not run**. These retained RED observations do not establish its behavior.

Compilation terminal records exit0; compile.stderr retains **six manifest warnings**. Complete artifact-seals and normal/integration seal text retain selected compiler records; giant compile.stdout, executable/library bytes, archives, and source trees are excluded. Recovery receipts remain historical text, not new executable authentication.

Both runtime directories retain each of four runs' complete stdout, stderr, argv, result, admission, terminal, and launcher, plus aggregate results. Initial `runtime-red`: **four sandbox failures, zero test cases reached** (read-only snapshot lacked target mountpoint). `runtime-red-retry1`: **eight actual query-boundary FAIL**, split bars 2, HUD selections 3, cached bar consumer 1, HUD containers 2. Bars fail on snapshot type; selections fail on return arity; cached bar consumer fails on supplied-record getter; HUD containers fail on numeric selection before cached addon loading. Later consumer lifecycle/loading assertions remain **unreached**, not additional failures or proof. Timeout child/parent summaries are duplicates, not extra cases. Prior **2 + 23 PASS evidence remains unchanged**, not rerun or revalidated here.

Saved deviations are retained: sandbox mountpoint setup and retry; archive controller's incomplete first output and separate archive recovery; missing authentication receipts and independently saved recovery receipts; controller's flattened receipt-schema assumption failing before admission/launch, then actual-schema handling. No archive contents or executable bytes were read for this append. Paths use `<PRIVATE_VERIFICATION_ROOT>`, `<REPO>`, `<HOME>`, and `<TMP>` substitutions; no account/realm/player identities occur in selected text. Complete selected text has CRLF/CR normalized and exactly one final LF (including originally empty stdout); no lines omitted.

Main commits next, then independent text gate. Neither performed by this retention append; no new build/test, operational action, delegation, or verification claim.

## Initial bar/HUD GREEN-attempt append (not all-consumer GREEN)

[initial-green-manifest.json](initial-bar-hud-green/initial-green-manifest.json) registers **82 new complete sanitized TEXT payloads**, **122,306 original bytes / 119,463 public bytes**, under [initial-bar-hud-green/](initial-bar-hud-green/). Existing **177 payloads and manifests remain untouched and not rehashed**; prior failures remain retained. Private source epoch: `ui-widget-bars-hud-green-current/20261013T090502875604Z`, source **5a869f9a0**, production delta **0745fe84f**.

Initial eight cases: **5 PASS / 3 FAIL** (bars 2 PASS, HUD selections 3 PASS, cached bar consumer 1 FAIL, HUD containers 2 FAIL). Fresh core: state 6, security 1, lifecycle 2, header 9, prey 1 = **19 PASS**. Combined: **24 unique PASS + 3 FAIL**, not all-consumer GREEN. Timeout child/parent summaries are duplicate observations, not additional cases. Corrected **39ce three cases remain PENDING**; no corrected passing claim.

Fresh normal startup was **one execution**: **23 enum comparisons true**, proportions **0.2 / 0.1**, **four atlas table results**, trailing **[]**. Complete startup diagnostic stdout/stderr/argv/result/admission/launcher/terminal and `startup-enum-probe.lua` retained. Full-stderr cleanliness was established independently at `verification/ui-widget-bars-startup-probe-proof`; trailing [] alone does not prove stderr cleanliness.

Exact `systemd --wait` launcher records preserve `Finished with result: success` and `Main processes terminated with: code=exited, status=0/SUCCESS` where present. Successful units' post-show GC/default fields have no exit timestamp and `MemoryPeak=[not set]`; `ExecMainCode=0` defaults are not invented terminal proof. Failure terminal records remain complete.

Selected compile.stderr, compile-terminal, admission/revision/toolchain/submission/artifact-seals/receipt-integration/receipt-wow-sim, all four runtime-green test groups plus aggregate, startup diagnostic group/probe, and all five runtime-core groups plus aggregate are retained without excerpts or truncation. Original/public TEXT SHA-256 and sizes are registered per payload; parsed JSON fidelity checked after consistent private identity/home/repo redaction. No temporary paths occurred. CRLF/CR normalized to LF; exactly one final LF, including empty text. No giant compile.stdout, archives, source trees, executables or libraries read/hashed. Historical seal text is not new authentication. Public ownership is UID 1000; root ownership correction not needed. No specs/wiki/source changes, commit, build/test, delegation or operational action performed.

## Later consumer attempts

[Additive manifest](later-consumer-attempts/later-consumer-manifest.json) retains **72 new complete sanitized text payloads**, 94,350 public bytes. Previous259 payloads/manifests remain unchanged and were not rehashed. Compiler stderr/terminal, selected provenance/admission/authentication text and each actual consumer invocation's complete streams/argv/results/launcher/terminal are retained. No binary/library/archive/source tree or giant compile stdout is included; executable digest strings remain historical text only.

| Source / epoch | Unique cases | Reached boundary |
| --- | --- | --- |
| 39ce8969a / 093318797417Z | 0 PASS / 3 FAIL | Bar initial width assertion53; HUD prerequisite/combined load ordering. |
| 174b86505 / 095514586740Z | 0 PASS / 3 FAIL | Bar width actual55.599998474121094 vs55.6; HUD ordinary-global secure publisher assumption. |
| b28a45f91 / 100434449483Z | 2 PASS / 1 FAIL | HUD2 actual OnLoad PASS; bar initial neutral visibility assertion59 FAIL after geometry passed. |

All epochs are 20261013T-prefixed. Failed bar launcher exits101; b28 HUD launcher exits0 with exact successful process record. Timeout-child summaries are duplicate observations, not extra cases. Post-GC systemctl default fields do not replace that successful launcher receipt. Later bar incremental updates, hidden/restore, membership removal and unregister remain unreached. b28 compiler exits0, six inherited manifest deprecations, no source warnings/errors. Independent bounded runtime gate: `verification/ui-widget-b28-runtime-proof`.

The frozen initial append's 39ce PENDING statement remains historical preparation status, superseded only by the later attempt rows above. Production0745 is unchanged through b28. Query5/core19/normal-startup/defaultcheck proof is reused within original scopes, not newly executed here or added to overlapping historical core25. [Atlas application-result policy](../../../../docs/specs/texture-atlas-application-result.md) is a subsequent investigation, not a fix or native parity proof. Actual bar and broader parent acceptance remain OPEN.

Transforms: consistent private verification/repository/home prefix substitutions, CRLF/CR to LF, exactly one final LF including empty text; no truncation or omitted lines. New payload originals and public copies have recorded SHA-256/size; parsed JSON remains intact after substitutions. Main commits this append before independent retention verification; retention itself makes no new execution claim. Independent later-retention gate passed all72 new payloads and confirmed old259 unchanged without rehashing.

## Atlas application-result RED and Retail GREEN

[Application-result manifest](atlas-application-result/atlas-result-manifest.json) retains **113 new complete text payloads**, 127,071 public bytes; previous331 payloads/manifests untouched and not rehashed. Includes original delimiter compile failure (d1, zero cases), genuine21a RED, source5bd actual Retail streams, current compile/check/startup and authentication/admission/controller receipts. Binary/library/archive/source-tree/giant compile stdout excluded; historical digest strings are text only.

- `d1c5b4cb8`: build-harness parser FAIL from raw Lua string delimiter collision; zero behavioral cases. `21aabeaf1` fixes delimiter; direct rustfmt includes generated test sources skipped by Cargo fmt.
- `21aabeaf1`: **1 PASS / 2 FAIL** across three unique cases. Argument validation passes; new application-result test reports zero returns. Cached bar observes NeutralBar=true, Glow3=false, SparkNeutral=false with expected atlas names; later lifecycle unreached.
- `5bd208dba`: **20 unique PASS / 0 FAIL**: argument2, cached bar1, atlas-regression10, HUD2, core cached-lifecycle2 =17; separate known/unknown/numeric controls3. Actual bar passes initial geometry/visibility, updates, zero neutral zone, hidden/restore, membership removal and unregister. Timeout-child summaries deduplicated; all exact runtime launcher exits0.
- Fresh normal startup: **one execution**, exit0; all219 stderr lines inspected, zero actual Lua errors/warnings/AnimSetup; 23 enum matches, proportions0.2/0.1, four atlas tables, trailing[]. Fresh default check offline/locked-j2 exits0; six inherited manifest deprecations, zero source warnings/errors. Compile exits0 with the same inherited warnings. Initial check-controller NameError occurred before launch, retained and corrected without duplicate check execution.

Independent source/readability/directfmt/diff gate: `verification/atlas-result-source-gate`. Current startup/check gate: `verification/atlas-startup-check-proof`. Runtime gate: `verification/atlas-green-runtime-proof`. Retention gate follows commit; no suite/native acceptance follows these bounded scopes. Unaffected query proofs remain reused, not fresh or added to overlapping historical core25.

This is explicit [simulator application-result policy](../../../../docs/specs/texture-atlas-application-result.md), not native parity: cached generated docs/current Wowless omit returns, while unchanged cached TextureUtil consumes a success result for SetShown. No vendor, validation or existing atlas mutation changes. Read-only actual WTF cache access occurred during startup; public copies consistently replace account/realm/player identities derived from that actual stream. Private mapping excluded. Other transforms remain complete prefix substitutions, LF normalization and one final LF without truncation. Original/public text SHA-256 and sizes retained; parsed JSON preserved. At that historical checkpoint, affected PTR/profile, broader visualization/unit-state, owning sweeps, performance, GUI/external and parent handoff remained OPEN. Later bounded PTR/owning outcomes follow below.

## PTR and owning followup

[Followup manifest](ptr-and-owning-followup/followup-manifest.json) retains **143 new complete sanitized TEXT payloads**, 1,038,632 public bytes. Earlier444 payloads/four manifests are untouched and not rehashed; total587 retained payloads. Giant compile stdout, executables, libraries, archives, snapshots and private identity mapping are excluded. Historical digest strings are text, not new authentication. Original/public text SHA-256, sizes and JSON-equivalent transformations are registered; full selected streams retained without excerpts.

- Earlier PTR epoch102802, compiled34ef/production5bd: **26 unique PASS**, plus one fresh normal startup with full219 stderr CLEAN; [independent audit](ptr-and-owning-followup/independent/widget-atlas-ptr-proof/report.md). This scope excludes the three later consumers.
- Latest actual PTR epoch110748, compiled1804/production5bd: **bar1/HUD2 = three unique PASS**, both actual wait exits0, active `ptr/AddOns` diagnostics; [independent audit](ptr-and-owning-followup/independent/ptr-widget-real-consumer-runtime/report.md). Three child summaries excluded; prior denied admissions remain retained and are not test failures.
- Owning epoch103358, compiledb2f/production5bd: **four tests PASS**, rows11/419/59/48, OK11/305/56/46, **119 known row gaps**, zero new/stale; [independent audit](ptr-and-owning-followup/independent/owning-after-widget-proof/report.md). Publication only, not signatures/output/security/behavior/native parity.

Do not combine overlapping scopes or represent prior26/startup as replayed at1804. Mounted runtime snapshot39ce is distinct from compiled binary identity. Actual wait return/launcher termination evidence is authoritative; post-GC terminal defaults are not proof. Six inherited compile-manifest deprecations remain. Text-retention gate follows commit. Projection basename-collision failure occurred before a manifest was produced; only that new uncommitted partial projection was recreated with full selector subdirectories, without changing historical packets or runtime evidence.

[Current checkpoint SSOT](../../../../docs/wiki/investigations/integrated-source-and-factory-proof-2026-10-09.md#separate-ptr-and-owning-checkpoints--2026-10-13) owns current scope boundaries. Native return/unit parity, broader visualization state, other-profile/performance/GUI/external and full-parent acceptance remain OPEN.
