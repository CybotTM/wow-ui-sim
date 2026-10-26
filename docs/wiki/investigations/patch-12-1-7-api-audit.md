# Patch 12.1.7 API source audit

Runtime target update: 2026-10-23. [Selected source](../../specs/ptr-blizzard-ui-source.md) is now wowt **12.1.7.70493**; authenticated manifest regeneration and startup proof remain pending. The following I0 capture and all frozen 70424 references are historical, not new-build evidence.

Verified: 2026-10-22. Slice I0 accounts for the public source for **12.1.7 / 70424 / TOC 120107** without implementing its runtime changes. The [source-accounting spec](../../specs/patch-12-1-7-source-accounting.md) owns P01–P12, inherited category counts, header discrepancies and outstanding slices. All inherited applicability and behavior remain UNKNOWN until I4; GUID work remains deferred pending the user decision.

## I4 runtime test staging

Staged 2026-10-25 on `489df2822`: [runtime contract](../../specs/patch-12-1-7-inherited-runtime.md) and [ordinary integration selectors](../../../tests/patch_12_1_7_inherited_i4.rs) now replace the four absent recipe selectors. The three profile registers bind separate source bytes and observations. They preserve 1,876 occurrence IDs, probe 180 runtime plus 183 declaration candidates, and retain all 214 I4c PTR UNKNOWN rows. Five PTR original-consumer cases cover documented call/nullable shapes only; no native capability/default/security/event-order parity follows. Retail/Forever pre-repair baselines and actual RED execution remain pending. RegionalTextLocale already has bounded default-US original-consumer proof on master and is preservation, not a new missing producer. Glue-only witnesses do not create Game publication assertions.

## I4 first bounded producer repair

The first authenticated 70493 runtime discovery persisted all 1,876 observations and found 169 publication gaps. Only Collections `C_Spell.GetItemCooldown` and GhostFrame `CanPortGraveyard` reached their exact missing producer boundaries. SmartNavigation remained blocked by module import, and RecentAllies by missing enum publication; those are not downstream producer REDs.

The [bounded contract](../../specs/ptr-i4-first-green.md) publishes these two endpoints using the existing item cooldown records and supplied graveyard availability. The item query is explicitly shape-only where native eligibility is unknown. New supplied-state tests failed at both missing calls before implementation. Aggregate I4 tests remain unchanged and cannot pass while the other 167 shapes and three blocked consumers remain. Runtime GREEN and stock/check proof are recorded separately in the lane evidence, not implied by this implementation entry.

## I4 GameRules and RecentAllies prerequisites

Verified 2026-10-25: [batch2 contract](../../specs/ptr-i4-game-rules.md) and [original-consumer tests](../../../tests/ptr_i4_batch2.rs) bind actual game/glue TOC selection. Initial executions reach missing AccountHasSDEnabled at Barbershop line269, IsSDHDToggleEnabled at line261, IsSelfFoundAllowed at CharacterCreate line2366, and RecentAlliesInteractionCategoryFilter at RecentAlliesUtil line4. The SD setter is first blocked by its documented usage-location enum at line273; the preset default is first blocked by the toggle query at GraphicsOverrides line78. Those two are not downstream API REDs. The hardcore popup diagnostic is retained before its missing producer is repaired.

The first producer step publishes the two proven enum prerequisites and metadata from the pinned 70493 APIDocumentation. Its exact replay passes the original RecentAllies labels selector, then reaches the missing setter at Barbershop line273 and the missing hardcore query at popup line3512. Five state-backed functions are now implemented after these and the earlier direct query REDs. Their GREEN and the preset query remain pending. No native capability defaults, telemetry policy, argument restrictions or secret handling are inferred.

### Supplied WoWHack mode

The separate `IsWoWHack` extension reaches original RED at `GameRulesUtil.lua:207` through MicroMenu OnLoad/InitializeButtons and at `Blizzard_UIErrorsFrame/Mainline/UIErrorsFrame.lua:12`. Its PTR-only producer compares the existing supplied `game_rules.active_game_mode` with the existing GameMode enum declaration, without adding a numeric mode constant. Tests change Standard/WoWHack/Plunderstorm modes and observe `MicroMenu.addedButtons`, energy-message acceptance and the original error-frame anchor. Each selector rejects a constant query independently. GREEN and stock comparison remain pending; no native mode-selection policy is inferred.

## Empty inventory is not patch coverage

The direct page has prose and a link to 1.60.1, but no function/event/enum/CVar/widget tables. The existing `tools/gen_patch_wikitext_register.py` successfully emits `entries: []` and `header_counts: []` for it. This correctly describes the direct inventory, not complete patch coverage. No dummy API rows or invented aliases are added to make it nonempty.

A separate source-accounting ledger retains direct prose and the full linked inventory. `tools/validate_patch_12_1_7_source.py` rejects an empty API register in place of that ledger. Its successful JSON result explicitly reports `source_accounted: true`, `runtime_complete: false`, 12 direct rows and 1,876 inherited UNKNOWN occurrences. The shared generator and historical source-only seals are unchanged.

## Frozen identities and scope

Three byte-exact raw bodies were imported after matching SHA-256 against the fetch receipts. [Provenance](../../../data/patch-api/sources/12.1.7-api-changes.provenance.json) records each public URL, body size/hash, retrieval date and target. MediaWiki revision IDs were not observed and remain null; body hashes are the identities, not fabricated revision IDs. Direct HTTP requests returned 403; browser raw fetching succeeded without authentication. The optional `/notes` raw fetch returned 404 and is not imported as patch text.

The patch-page index gains a supplemental raw-page discovery entry. Its original allpages date/count/batches remain historical; the addition is not represented as a fresh exhaustive census or an observed page ID.

The TOC reference distinguishes file variables from metadata directives. P10/P11 preserve that distinction: rejecting metadata variables does not authorize removing Game/Family/TextLocale file expansion. RegionalTextLocale belongs to both PTR and Forever according to the reference, but this ledger does not prove its runtime implementation.

## Literal inherited rows, not inherited implementation credit

The fresh linked 1.60.1 body is retained separately from the [historical audit](patch-1-60-1-api-audit.md). The inherited JSON preserves every original `wt-...-<line>` ID, source line, direction, annotation, optional default and command kind. Repeated symbols remain separate occurrences; no suffix matching or deduplication grants credit.

The imported triage is a static discovery snapshot from revision `935d69191c540077ab76033d9afebaf55362c1e3`: 122 rows have lexical file candidates and 1,754 do not. These are **not gap counts**. Candidate file references are repository-relative; status remains `UNKNOWN; not dynamically assessed`. I4 must inspect the exact generated docs/original consumers and actual published surface before promoting any row. Global/event header discrepancies remain visible; the command inside the CVar column remains a command. See the spec for exact counts and category limits.

## Reproduction and proof boundary

From repository root:

```text
python3 -B tools/test_patch_12_1_7_source.py
python3 -B tools/validate_patch_12_1_7_source.py
```

The regression was written RED before the validator: no source-accounting validator existed to reject empty-register substitution with an explicit coverage diagnostic. Nine targeted Python fixtures then passed after implementation. Tests copy only the frozen source artifacts and patch index into a temporary root and exercise the real validator CLI; omissions and false-credit mutations do not alter tracked files. Validation re-parses inherited raw text with the existing parser and compares complete occurrence records and header counts, not just totals.

No Cargo, simulator startup, native execution, vendor synchronization or profile acceptance was performed. Source accounting is the proof level, not API compatibility. Future I1–I6 work must not reinterpret I0's accounted rows as completed runtime work.

## I4a: immutable companion, not applicability acceptance

Verified static accounting: 2026-10-24. The [I4a contract](../../specs/patch-12-1-7-source-accounting.md#i4a-inherited-applicability-worklist) owns counts, pending source identity and acceptance limits. The [companion](../../../data/patch-api/sources/12.1.7-inherited-applicability.json) retains every literal occurrence with three independent profile records. No I0 bytes or historical seals change. No source-cache, decompiled-source or runtime observations were used.

The validator first verifies frozen I0 hashes and reuses its raw-text reparse, then compares exact occurrence records, kind/domain, header disagreements, profile/proof states and historical joins. Source identity fields include optional defaults verbatim. Candidate test references require exact full-symbol occurrence at the recorded line and carry `LEXICAL_ONLY`; neither those references nor the old source candidates establish behavior, registration or selected ownership. Existing narrower behavior tests are not erased, but they receive no new inherited acceptance credit here.

Historical sweep joins are recomputed by exact `(section, symbol)` from 14 bounded primary registers, retaining old ID/direction/profile and known-gap membership. Every intersection opposes the inherited direction. Gap membership is not a fresh measurement. The separate 2.4.0 register-only identity is checked without inventing a sweep. Later unrelated patch files are ignored, not admitted to historical scope.

I4a deliberately rejects any claimed selected-source receipt or promoted proof status: I3 has not been accepted and this slice supplies no such observation. Resolving one later requires an evidenced follow-up schema/validator change, not changing `UNKNOWN` to `PASS`. [Consumer selection](consumer-selection-and-red-proof.md) governs TOC flavor, Game/Family, restrictions, XML/Lua chain, owner/screen/LoD and callable-consumer proof; a cached Standard or Camelot hit alone cannot satisfy it.

### I4b grouped worklist (proposed selectors, not registered tests)

| Group | Rows per profile | Pending evidence |
|---|---:|---|
| `p1217_inherited_factory_publication` | 364 | 185 globals, seven widgets, 27 events, 144 CVars, one typed command; raw vs ordinary lookup, constructible factory, accepted name vs real producer, stated default obligations separately. Do not execute allocator dump or enable disabled TTS/hardware. |
| `p1217_inherited_game_publication` | 1,512 candidates | Accepted selected Game publisher/chain, local/global identity, removal/successor/alias provenance; reconcile all six repeated-banner IDs. |
| `p1217_inherited_lod_publication` | Same 1,512, partition pending | Selected LoD owner and explicit successful load receipt; startup is not LoD acceptance. |
| `p1217_inherited_glue_publication` | Same 1,512, partition pending | Genuinely selected Glue publisher, separately from Game; never force-load unselected files. |
| `p1217_inherited_negative_controls` | All 1,876 | Identity omissions, raw-fallback trap, wrong defaults, permissive unknown-event control, unsupported factory; each later claimed producer RED independently rejects constants/no-ops. |

These candidate publisher groups overlap until ownership is resolved; they are not 4,536 additional rows. PTR, Forever and Retail need separate accepted configurations, at most three first-gate configurations after harness preparation, not per-row builds. Runtime work must list actual discovered selectors and record source/build/fixture identities. The I4a deliverable performs none of those builds and reserves no slot.

## I4b: conditional selected-source obligations

Verified static staging: 2026-10-24. The [conditional contract](../../specs/patch-12-1-7-source-accounting.md#i4b-conditional-static-selection) owns counts and limitations. The [I4b overlay](../../../data/patch-api/sources/12.1.7-inherited-static.json) supplements, rather than mutates, I0/I4a. Its applicability decisions depend on unmerged I3 commit `e30d17b106ce76a872c2bdfe69f95b4ec215fd06`; no decision is effective on the stale 69594 I4b base.

The initial blocker was source identity, not missing simulator APIs. The parent authorized read-only use of I3's committed 70493 pin/manifest and private cache. Every one of its 4,578 files matched the pinned content size and MD5 content key. `tools/patch_12_1_7_static.py` requires exact pin/manifest SHA-256 values, matching cache provenance/completion and unchanged I0 hashes. An incomplete cache fails with exact missing paths instead of producing an empty scan or false absence result. Prefix-style `[AllowLoadTextLocale: zhCN]` browser entries are locale exclusions, not missing paths; a focused RED fixture prevents that diagnostic confusion.

The tool traces runtime-selected PTR TOC flavor, Game/Family, allow/exclude filters, locale, public environment and document-level XML chains. It records concrete selected declarations, exact native API calls, structured generated documentation and literal event/CVar consumers. Game, LoD and Glue remain separate; a source-selected LoD publisher is not a successfully loaded addon. Generated API documentation establishes only its documented source contract, not a callable factory or native screen binding. Source text is scanned with comments/strings separated and local owners excluded; it is not a full Lua resolver. Dynamic aliases/arguments/assignments and secure-pass ordering remain unproved.

Each occurrence keeps its own disposition. The three added PvPObjectiveBanner declarations have selected witnesses; the three removed occurrences retain surviving-definition evidence but remain UNKNOWN. No native retirement is inferred from a public diff list, and no source absence is inferred from missing supported witnesses. Retail and Forever remain independent, unpromoted profiles. All original-consumer execution, behavior, publication and native proof fields remain UNKNOWN.

The [static fixtures](../../../tools/test_patch_12_1_7_static.py) reject constant/accept-all outputs, unselected flavors, wrong Game/Family/filter/locale/environment, private/local/comment/string observations, namespace suffix attribution, nested non-document XML scripts, stale pins, missing/changed cache files and receipt/dependency corruption. RED used a disposable accept-all scaffold, then focused counterexamples before implementation fixes. These are classifier/validator controls, not producer RED or runtime acceptance. No builds, slots or vendor writes were used.

## I4c: distinguish removal intent from selected source

Verified static classification: 2026-10-24. The [I4c contract](../../specs/patch-12-1-7-source-accounting.md#i4c-removal-contradictions-and-excluded-publishers) owns counts and status meanings. Its [overlay](../../../data/patch-api/sources/12.1.7-inherited-i4c.json) is now the static disposition source; the earlier overlays remain immutable historical evidence. I3's exact source dependency is present in the I4c base, and every pinned cache file was authenticated again. No runtime was launched.

The main UNKNOWN class was not a missing implementation: most inherited FrameXML removals still have original selected Mainline definitions. Those definitions contradict a literal-source retirement obligation on PTR. Two extra surviving names use direct assignments rather than function declaration syntax. These observations cannot identify why a historical diff called a name removed, nor establish actual post-load globals. A repeated added/removed occurrence therefore receives separate source applicability rather than contradictory simultaneous runtime assertions.

[The classifier](../../../tools/patch_12_1_7_i4c.py) reuses I4b's authentication and selected-source tracing. It adds exact assignments, conservative local/parameter exclusion and rejected-TOC tracing. Excluded publishers need original pinned file edges, selected-TOC hashes and a concrete flavor/game restriction or substituted sibling. One excluded definition cannot hide another declaration with unresolved environment selection. An excluded consumer of a native function does not establish native absence.

A complete authenticated text inventory also records whether each exact literal spelling occurs anywhere, including comments and strings. **Absent literal spelling is not retirement proof.** Dynamic global construction, aliases, native registration and unresolved script environments remain explicit questions. The remaining removed-row work must observe original loaded owners or native catalogs; it must not synthesize names, force-load Classic/Camelot files, or infer aliases from similar spellings.

[Static fixtures](../../../tools/test_patch_12_1_7_i4c.py) exercise selected aliases, opposing occurrences, all four explicit exclusion routes, local/parameter/comment/string traps, unresolved secure publishers, native-consumer non-credit and receipt/question/status/ID mutations. Concrete checked-in examples cover both filter aliases, secure delegates, the two excluded removed publishers and opposite banner directions. These are static-classifier tests, not simulator producer RED/GREEN. [Consumer-selection rules](consumer-selection-and-red-proof.md) still govern any later runtime acceptance.

## Sources

- [12.1.7 raw patch text](../../../data/patch-api/sources/12.1.7-api-changes.wikitext) — direct prose, target and inherited link.
- [TOC reference raw text](../../../data/patch-api/sources/12.1.7-toc-format.wikitext) — file variables, RegionalTextLocale and metadata restriction.
- [Current linked 1.60.1 raw text](../../../data/patch-api/sources/12.1.7-inherited-1.60.1.wikitext) — inherited literal inventory and unchanged header disagreements.
- [Source ledger](../../../data/patch-api/sources/12.1.7-source-accounting.json) — P01–P12 and deferred/UNKNOWN statuses.
- [Inherited occurrence triage](../../../data/patch-api/sources/12.1.7-inherited-1.60.1.json) — full per-occurrence linkage and bounded lexical candidates.
- [I4a validator](../../../tools/validate_patch_12_1_7_inherited.py) and [disposable controls](../../../tools/test_patch_12_1_7_inherited.py) — accounting/proof-separation boundary only.
- [I4b classifier/validator](../../../tools/patch_12_1_7_static.py) and [static overlay](../../../data/patch-api/sources/12.1.7-inherited-static.json) — exact I3-dependent selected-source obligations, not runtime credit.
- [I4c classifier/validator](../../../tools/patch_12_1_7_i4c.py) and [static dispositions](../../../data/patch-api/sources/12.1.7-inherited-i4c.json) — source survival, explicit publisher exclusions and exact unresolved questions; no native retirement credit.
- [Runtime TOC selection](../../../src/loader/mod.rs), [TOC filters](../../../src/toc/mod.rs) and [XML path resolution](../../../src/loader/helpers.rs) — current simulator source-selection semantics.

## See Also

- [[patch-1-60-1-api-audit]] — separate historical Forever source-only seals and behavioral gaps.
- [[patch-12-1-5-api-audit]] — previous Mainline source/publication/behavior separation.
- [[client-profiles]] — selected profile/epoch boundary.
