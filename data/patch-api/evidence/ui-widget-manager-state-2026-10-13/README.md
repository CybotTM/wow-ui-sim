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
