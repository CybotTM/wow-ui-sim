# TextWithState producer — bounded source gate

Date: 2026-10-13. Scope: `f6284928c..2bb6cdde1`, exactly four changed files. **PASS: source, format, readability and spec gate only. GREEN, compiler/runtime, native and full-parent acceptance remain OPEN.**

## Scope and commands

Read both requested skills completely. No delegation, builds/checks, runtime launches, hashes, operations, commits or canonical source edits. Only verification artifacts written. `git show 2bb6cdde1:<path>` content equals current content for all four files; no cryptographic provenance claim. Initial status showed only untracked `.code-index.db` and `verification/`.

| Gate | Result | Evidence |
|---|---|---|
| Direct three-file format | PASS, exit 0; empty stdout/stderr | `rustfmt --edition 2024 --check --config skip_children=true src/c_api/ui_widget_manager.rs src/c_api/ui_widget_manager/queries.rs src/c_api/ui_widget_manager/text_state.rs` |
| Workspace format | PASS, exit 0; empty stdout/stderr | `cargo fmt --check` |
| Source/spec | PASS, bounded inspection | Four-file revision comparison; cached schema and saved RED inspection below |
| Readability | No changed-code violations | Manual checklist; metrics CLI unavailable on PATH; no compilation inferred |

Each format command ran exactly once on this new source scope. Earlier input-stage format gate was not rerun. Complete format streams and command ledger are in `proof.json` and `format-*.stdout/stderr`.

## Artifact/source coverage

[EXIST] PASS — four files present and substantive: spec 45 lines, module 77 lines, queries 165 lines, new producer 91 lines (exact counts in proof).

[SUBSTANTIVE] PASS — `text_state.rs:11-23` registers an actual Rust query; `:25-46` validates, reads supplied state, clones, handles absent input and projects a snapshot; `:48-91` writes every field. Not a compatibility placeholder.

[WIRED] PASS — `ui_widget_manager.rs:14` declares `mod text_state`; `queries.rs:22` calls `super::text_state::register(state)?`; existing `lua_api/globals/missing_surface.rs:258` calls `ui_widget_manager::queries::register`. New publication uses `matches!(ACTIVE, ClientProfile::Retail | ClientProfile::Ptr)` (`text_state.rs:12`) and returns before namespace access for Wrath/Mists/Era/Anniversary/WowForever. It does not inherit candidate ButtonHeader's adjacent cfg gate. Those five profiles' existing publication paths are unchanged by this delta; **not a native-absence or executed profile-publication claim**.

[ANTI-PATTERN] PASS — no TODO/FIXME/HACK/XXX in added producer; no added empty catches, commented-out implementation, warning suppression, fallback/default or clamp.

| Contract | Source evidence | Proof level |
|---|---|---|
| Strict numeric/secret validation | `text_state.rs:26-29` uses unchanged `queries::read_numeric_id`; `queries.rs:26-32` unwraps secrets before requiring `Val::Num`, before lookup | Source only; policy execution pending |
| Numeric keys/environment-local lookup | `text_state.rs:30-34` reads this environment's map via existing `WidgetId::from_number`; no integer cast/new range rule | Source only |
| Fresh snapshot / absent return | Owned clone ends state borrow before projection; absent path pushes one nil and returns 1; present path allocates new row, pushes it before field/string allocation, returns 1 (`:30-45`) | Source only, not runtime/GC parity |
| Complete schema-preserving projection | Seven text numbers + eight presentation numbers + five strings + one bool, all supplied directly (`:48-91`); f64 fields preserved, u32 enums widened exactly | Source + declaration comparison, not GREEN |
| Feed/consumer lifecycle | Existing supplied tests inspected; feed unchanged by delta | Earlier rejection control only executed; remaining acceptance pending |

Both profile caches' `Blizzard_APIDocumentationGenerated/UIWidgetManagerDocumentation.lua:1868-1893` declare the same **21 non-nil fields**, exactly matching projection with no duplicate/missing/extra key: 15 numeric/enum, 5 string/textureKit and 1 bool. Both getter declarations at `:413` specify `SecretArguments = "AllowedWhenUntainted"`, required number input and nilable record return. Cached declarations are not native measurements. Exact fields/types are retained in proof.

## Readability

Manually audited all changed Rust lines under requested skill, including clarity, effects, complexity, length/nesting, state accumulation, suppressions, naming, early returns, duplication/sibling similarity and parameter overload. New functions are below 30 body lines, maximum control nesting one level; no long iterator chain or complex boolean expression. `write_*` names and `&mut LuaState` make mutation explicit; named projection groups avoid dense monolithic construction. Shared existing numeric/string projection idiom is not a newly duplicated body requiring abstraction. Existing unmodified functions are not new findings in this bounded delta. No violations to report. Metrics binary unavailable; no numeric metric result fabricated.

## Spec and supplied test boundary

Spec accurately records independently retained genuine RED at declared input revision `64a2a57b2`: **7 executed, 1 passed, 6 failed, 0 ignored, 10977 filtered out, 0.72s, actual wait exit 101**. Five seeded direct cases passed publication then failed `missing supplied TextWithState record`; feed callback failed supplied-table assertion and immediate event-result assertion. Only rejected-feed preservation passed. Saved/current seven test/helper files and feed source compare equal; saved execution provenance remains metadata, not newly authenticated bytes.

Complete21 values/types/count, snapshot freshness, unknown arity/nil, numeric identity/rejection, environment isolation, real-secret wrapper policy/recovery, later feed transitions and cached activation/update/Hidden/restore/removal assertions were **unreached** in that RED. The spec leaves GREEN and cached-consumer acceptance pending and does not convert these into passes.

Fixtures assert fractional padding/order; replacements assert negative padding/order, fractional width/effect ID, query-only kits and timer true. Numeric-key cases include negative/fractional/zero/large IDs. Consumer uses negative padding with unchanged template clamp-to-zero; producer does not clamp. Consumer and security prerequisites follow the initial supplied-table boundary. Security case retains the existing `retail-12-1-0` gate. Tests were read, not rerun.

Historical input-source gate remains historical; no retroactive PASS. Concurrent Retail/PTR compiler units and unrun runtime proof remain main-owned and were neither polled nor altered. No GREEN, native, rendered, profile-runtime or full-parent claim.

## Inspection incidents

First Pyrun git calls omitted cwd and failed outside repository; no source changed. Subsequent git reads used explicit repository cwd. One format setup evaluation failed on undefined `repo` before any command ran; corrected evaluation executed the two requested format gates once. File-search tool lacked `fd`; known test paths were read directly. These incidents are retained in proof, not counted as executed format gates.
