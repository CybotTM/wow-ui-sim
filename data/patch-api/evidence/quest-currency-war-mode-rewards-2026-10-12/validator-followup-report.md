# Validator source/readability follow-up

Date: 2026-10-12. Repo: `/home/osso/Projects/wow/wow-ui-sim`.
Target: `a5eaf4fc63e44027b1e9757b146b398c62e3a4b3`; parent: `7d82b44a6271b93e8c77a8812faae5cac438b37e`.
Both requested skills read completely. Bounded artifact/manual readability verification only; no delegation or implementation.

## Exact scope

Full target commit diff inspected: ONLY `src/c_api/item_spell/c_currency.rs`, 4 additions and 12 removals. Every added/deleted line inspected manually. The old 11-line validator plus separating blank line is removed; its call is replaced with four lines calling the existing shared validator. No other changes in the commit.

Comparison against prior source endpoint `067fdbb90743e0c16159888a3754fc71fcdfcee5` across all eight prior scoped Rust files produces exactly this same refactor. Seven files are byte-unchanged; the currency file differs only by this refactor. Reuse prior four-getter and registration-route SOURCE proof, not prior runtime evidence. Currency faction getter shifts from lines 103–117 to 95–109; its body and registration are unchanged.

## Coverage matrix — source only

| Check | Evidence at target revision | Result |
|---|---|---|
| Duplicate validator removed | Full diff deletes `validate_war_mode_currency_id`; currency query now calls `helpers::validate_supplied_relation_id` at `c_currency.rs:79–82` | PASS source; prior DUPLICATE finding resolved |
| Identical validation/error | Deleted body and `helpers.rs:11–16`: same Val::Num match, inclusive i32 range, finite/integral predicate, lazy cast; error literal at `c_currency.rs:81` exactly matches deleted literal, shared helper passes it unchanged to `rilua::runtime_error` | PASS source equivalence |
| Authentication order | `c_currency.rs:75–84`: stack read → unchanged feature-gated `unwrap_secret` → shared validator → state lookup; both fallible calls retain `?` | PASS source; secret-before-validation preserved |
| Two independently nullable returns | Unchanged `c_currency.rs:83–92`: lookup copies `(applies, once)` or `(None, None)`; each Option separately maps to nil/bool and pushes in original order; `Ok(2)` unchanged | PASS source; no conflation or return-count change |
| Four getters/wiring retained | Eight-path diff from prior endpoint contains only validator refactor; prior `source-readability-report.md` / `source-readability-proof.json` | PASS reused source proof only |

## Readability

**Resolved:** prior `[DUPLICATE] helpers.rs:7` versus removed `c_currency.rs:74–83` validator. One validation algorithm remains for these callers; no copied replacement.

**New violations: none found in changed lines.** Existing error literal is caller-specific compatibility text, not unexplained magic data. Shared helper name predicts pure validation; invocation introduces no hidden effects, new branches, mutation, suppressions, or additional responsibilities. Updated query has 18 body lines and maximum brace depth 2 including function body; no length/nesting threshold violation. Manual complexity inspection only; `rust-code-analysis-cli` unavailable on PATH, no metric command run. Unrelated existing code was not promoted into findings.

Added-line scan: TODO/FIXME/HACK/XXX, clippy/dead-code allowances: zero each. No vendor changes, stub/commented-out implementation, or suppression introduced.

**Outcome: validator source equivalence PASS; bounded readability PASS (previous duplicate resolved; zero new findings).** No cargo/build/tests/checks, binary hashes, operations, questions, delegation, or repository edits performed. Only requested report/proof files written. Runtime, compile, startup, native parity, queued workers, and final/full completion remain outside this verification.
