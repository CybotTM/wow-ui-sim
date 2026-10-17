# Bounded QL-1 / QL-2 execution

Approved slices: supplied objective progress, readiness/dialog phase and opt-in
completion notification. Additional original-consumer prerequisites were explicitly
authorized. No vendor Lua edits, reward grants, native/pixel or all-profile claim.

## Saved 9fc3d15f9 checkpoint

Base: `9fc3d15f90f615006e0e42c95b121f5864c7f5e3`. Both rebuilt libraries/binaries and
both copied test components use source `f21aa7a060bdf839dfe07e2bd0d28be0f86df511`.
Rebase resolved only appended wiki-log entries, preserving both histories; no source
conflict or B5 integration. Later documentation-only commits do not change runtime
or test bodies.

| Capability | Retail | Forever | Proof |
|---|---:|---:|---|
| QL-1 counts/identity/notifications/original tracker/percentage/criteria/world projections | 6 PASS | 6 PASS | Actual callbacks and original cached widgets |
| QL-2 readiness/history/gossip/progress→reward/popup | 5 PASS | 5 PASS | Original consumers, positive state and error-count guards |
| Material, temporary empty requirements/portrait, populated currency rows | 7 PASS | 7 PASS | Exact Parchment/Stone, hidden/detached portrait, modeled currency display/count/context |
| Existing quest preservation | 64 PASS | 63 PASS | All requested component case identities retained |
| Startup `lua-errors` | `[]`, exit0 | `[]`, exit0 | Exact clean gates against master |

Complete requested component sets: **82/82 Retail** (51.15s), **81/81 Forever**
(49.15s); zero failed/ignored/filtered cases. No test Lua-error lines. Existing
persisted CVar bytes/stat/hash were copied into fresh child-only XDG homes and
unchanged before/after every execution; never reset or restored.

## Build and execution scope

Normal, non-test libraries (`profile.test=false`), headless feature bundles
`casc,client-retail` and `casc,client-wowforever`; no sound/GUI acceptance.
Build command per profile:

```text
CARGO_TARGET_DIR=~/.cache/wow-ui-sim-targets/slot-0 CARGO_INCREMENTAL=0 RUSTC=/usr/bin/rustc
/usr/bin/cargo build --offline --locked --no-default-features --features casc,client-<profile> --lib --bin wow-sim --message-format=json -j 1
```

Real test files/shared harness were copied unchanged, compiled with rustc against
the matching sealed normal library and emitted cfgs, then run serially under an86s
limit. Startup used the sealed `wow-sim --no-addons --no-saved-vars lua-errors`,
`WOW_SIM_CASC=0`, `WOW_SIM_NO_SOUND=1`. These are bounded components, not the whole
Cargo integration target. Actual toolchain/argv/source hashes/seals are in receipts.

Private evidence root: `/home/osso/.local/state/wow-ui-sim/handoff/quest-lifecycle-build/`.

- Retail: `rebased-green-retail-20261016t235514565067z/`;
  `rebased-quest-component-20261017t000240640016z/rebased-quest/` and startup receipts.
- Forever: `rebased-green-wowforever-20261017t000350809762z/`;
  `rebased-quest-component-20261017t001124186653z/rebased-quest/` and startup receipts.
- `rebased-series.txt`, `rebase-range-diff.txt`, `rebased-coverage.json`,
  `proof-ledger.json`, `eight-global-inventory.json` and final `report.md` bind details.

## RED history and corrections

Original QL tests committed separately before producers. Both pre-producer profiles
reached QL-1 real count/percentage/notification REDs and QL-2 readiness/opt-in REDs;
initial closure/missing-function failures were not credited as behavioral RED.
Generic intrinsic-tag, XML range/default and material fixes each have separate
focused original-consumer RED→GREEN receipts. The material ingress's initial E0063
compile error is not RED.

Five unmodeled dialogue/requirements/portrait getters are explicitly temporary,
with a retirement condition. Three original nil-call boundaries were reached;
money/currency-count calls were initially behind the earlier item-count failure,
not separately credited. The first no-values portrait draft reached a real original
nil-number comparison; the approved typed empty is `0, "", ""`. Tests prove the
original no-portrait hide/detach branch.

The currency bridge has actual absent-row RED with empty-panel control PASS.
Original CurrencyContainerUtil converts1166 to `Timewarped Badge`; corrected tests
assert that exact modeled label and count5 **and** retained supplied currencyInfo.
No arbitrary nonempty-label oracle or invented rows.

The old branch-base Forever10-record/14-occurrence comparison is **historical and
superseded**. It predates master's gamepad fix `03e1ce7d3`; it is not final acceptance.
No baseline was rewritten to bless errors. Rebased Forever is clean `[]`.

## Current integration gate

Master advanced to `275f9f09b` with the approved batch/B5. The saved checkpoint above
is not acceptance on that later base. Rebase/rebuild and repeat Retail82/Forever81,
both exact clean startups, plus the approved ledger's B5 Retail cases
(`currency_displayed_indices` two; `currency_watched_mutations` one). Main's verifier
then owns final acceptance.

## Limits

Unsupported-profile negative control exists but was not executed; GUI, native,
full-suite and whole-parent acceptance remain separate. Reward metadata is limited
to represented fields; reward spells, required-item/currency/cost and portrait-input
systems are not modeled. Main owns independent verification and merging.

## Owners

- [Objectives](../specs/quest-objective-progress.md), [readiness/dialog](../specs/quest-readiness-dialog.md).
- [Intrinsic tags](../specs/xml-intrinsic-tags.md), [widget values](../specs/xml-widget-values.md).
- [Material](../specs/quest-background-material.md), [temporary inputs](../specs/quest-progress-requirements-empty.md), [currency panel](../specs/quest-reward-currency-panel.md).
