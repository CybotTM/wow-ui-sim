# Patch 12.1.7 API source audit

Runtime target update: 2026-10-23. [Selected source](../../specs/ptr-blizzard-ui-source.md) is now wowt **12.1.7.70493**; authenticated manifest regeneration and startup proof remain pending. The following I0 capture and all frozen 70424 references are historical, not new-build evidence.

Verified: 2026-10-22. Slice I0 accounts for the public source for **12.1.7 / 70424 / TOC 120107** without implementing its runtime changes. The [source-accounting spec](../../specs/patch-12-1-7-source-accounting.md) owns P01–P12, inherited category counts, header discrepancies and outstanding slices. All inherited applicability and behavior remain UNKNOWN until I4; GUID work remains deferred pending the user decision.

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

## Sources

- [12.1.7 raw patch text](../../../data/patch-api/sources/12.1.7-api-changes.wikitext) — direct prose, target and inherited link.
- [TOC reference raw text](../../../data/patch-api/sources/12.1.7-toc-format.wikitext) — file variables, RegionalTextLocale and metadata restriction.
- [Current linked 1.60.1 raw text](../../../data/patch-api/sources/12.1.7-inherited-1.60.1.wikitext) — inherited literal inventory and unchanged header disagreements.
- [Source ledger](../../../data/patch-api/sources/12.1.7-source-accounting.json) — P01–P12 and deferred/UNKNOWN statuses.
- [Inherited occurrence triage](../../../data/patch-api/sources/12.1.7-inherited-1.60.1.json) — full per-occurrence linkage and bounded lexical candidates.
- [I4a validator](../../../tools/validate_patch_12_1_7_inherited.py) and [disposable controls](../../../tools/test_patch_12_1_7_inherited.py) — accounting/proof-separation boundary only.

## See Also

- [[patch-1-60-1-api-audit]] — separate historical Forever source-only seals and behavioral gaps.
- [[patch-12-1-5-api-audit]] — previous Mainline source/publication/behavior separation.
- [[client-profiles]] — selected profile/epoch boundary.
