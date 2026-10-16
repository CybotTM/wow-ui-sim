# Approved batch bounded proof — 2026-10-16

Code revision: `7d18829ccba768ea2d1e038c353ea72dced3518e`, branch `approved-batch`. Separate tests/scaffold commits preceded every producer; compile/setup failures were not counted as behavioral RED. No Blizzard/vendor source was edited.

## Coverage

| Slice | Behavioral proof | Limits |
|---|---|---|
| SS-P1 | Retail seven secret-selector/empty-result/original ordered-consumer cases RED → GREEN; nine existing charge and five ordinary cooldown cases preserved | PTR documentation only; no native argument-domain or restricted-tainted widget-setter expansion |
| G3 | Retail/Wrath two cases each: original color consumer white/blue/gray/white, bindings, empty slots, macro/outfit occupancy | Spell kind explicitly approved; Retail uses its deprecated wrapper plus Wrath color consumer, not Retail ActionButtonMixin acceptance |
| G2 | Retail three meaningful cases; Wrath four: supplied queries, original checked/START/STOP/Flash/state-event behavior and input/environment isolation | Retail legacy-only early-return case has no behavioral credit; PTR/Mists/Forever not executed |
| B5 base/watch | Retail original TokenUI collapse/projection two cases and watched mutation one case RED → GREEN; all 16 existing currency probes preserved | Unused grouping deferred; modified-click consumer itself not exercised |
| U1 | Corrected Retail original divider/geometry/alpha/lifecycle test RED → GREEN; numeric-row control and 31 existing tooltip cases preserved | User-selected unchanged blank-row height, centered image; deferred texture requests, not decoded assets/GPU pixels |
| T4 | Retail/Wrath one API case and three glyph-render cases each RED → GREEN | Original Wrath quest consumers blocked by legacy PlaySound string support, to rerun after `wrath-fixes` and this branch merge; Mists unexecuted |
| H1 | Retail original XML font/block geometry, all 32 emitted link-glyph gestures, original callback/payload and negative/lifecycle cases; Wrath original ITEM_TEXT_READY geometry/width/height; RED → GREEN; 18 existing SimpleHTML cases per profile preserved | Isolated dependencies, not full glue/game routing or native metrics |
| CheckButton prerequisite | Tests `c4f79d9d6`, producer `7d18829cc`: Wrath direct numeric flags and original UpdateState branches RED → GREEN; original G2 event consumer GREEN; Retail direct contract plus 19 existing CheckButton tests GREEN | Local reader only; strings remain unchecked; shared boolean extraction unchanged |

132 Retail and 41 Wrath unique selected profile/test pairs passed. One Retail G2 early-return pair carries no behavioral credit: 172 meaningful pairs, not 173 behavior claims. Historical proof revisions remain explicit; subsequent disjoint XML-role and checked-input changes have dedicated regression/preservation evidence. No milestone-only reruns were used.

## Startup observations

Both commands: sealed normal binary, `--no-addons --no-saved-vars lua-errors`, CASC/sound disabled, fresh private copy of current CVars; original CVar stat/hash unchanged.

- Retail final: exit 0, `[]`, zero unique/occurrences.
- Wrath final: exit 1, 44 unique/55 occurrences. Exact message/count-map equality to the committed [failing inventory](wrath-lua-errors.json) after its [documented cache-prefix normalization](wrath-lua-errors.meta.json); zero new/missing messages. This is not a clean Wrath startup claim.

## Evidence and remaining gates

Private ledger: `/home/osso/.local/state/wow-ui-sim/handoff/approved-batch-build/report.md`; machine name/proof accounting: `final-proof-ledger.json` beside it. Epoch submission/toolchain/seal/source-snapshot/runtime records retain exact commands, revisions and results outside build caches. Final Retail epoch `approved-retail-checkbutton-final-20261016T234603127773Z`; final Wrath normal epoch `approved-wrath-checkbutton-green-20261016T233106709665Z`; Wrath unit epoch `approved-wrath-unit-green-20261016T233850044445Z`.

No all-profile/full-suite/native/GPU-pixel acceptance. Final lint/readability/check gate belongs to the parent verifier, not this build task. Standalone copied test wrappers expose existing unused shared-helper warnings; no warning suppressions were added. Ordinary Cargo library/binary/unit builds provide compilation evidence.
