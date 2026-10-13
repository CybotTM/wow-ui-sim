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

No new e887 GREEN claim: pending. Native unit registration, native client parity, and broader completion remain unknown/unclaimed. Double timeout-child summaries count one test, not two.

## Authentication and controller deviations

Lifecycle RED authentication required two hash reads after a lost receipt; retained report says missing-output recovery, not exactly-once authentication. Historical seals/receipts are copied as text; no old executable rehash performed for this packet.

67775404d state launcher used --wait with RemainAfterExit=yes and blocked after test completion. Main retained exact process terminal in state-green/runtime-gate/state.main-retained-terminal.txt; completed service was stopped to rescue controller without test reexecution. Controller recovery and original launch text retained. Exact terminal text is authoritative; packet does not convert controller recovery into another test run.

## Provenance and transformations

manifest.json lists every copied payload, original byte size and text-file SHA-256, transformed size and SHA-256. Original digests describe private text bytes, not executable authentication. Home/repository paths and temporary paths use consistent placeholders; actual account/realm/player segments discovered from actual EditMode cache source startup lines (and WTF paths when present) are replaced consistently across all payloads. Private mapping is not published. Every retained text is UTF-8, CRLF/CR normalized to LF, with terminal newline run replaced by exactly one LF; empty text becomes LF. No truncation or line omission. JSON/text receipts retain historical field names and claims; redactions do not establish those claims independently.

Scope excludes compilation output too large for bounded packet, source snapshots, binaries, native observations not present in these epochs, and pending later GREEN evidence. Main must commit before any independent gate; neither action was performed here.
