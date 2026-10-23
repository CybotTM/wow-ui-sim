# Patch 12.1.7 API source audit

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

## Sources

- [12.1.7 raw patch text](../../../data/patch-api/sources/12.1.7-api-changes.wikitext) — direct prose, target and inherited link.
- [TOC reference raw text](../../../data/patch-api/sources/12.1.7-toc-format.wikitext) — file variables, RegionalTextLocale and metadata restriction.
- [Current linked 1.60.1 raw text](../../../data/patch-api/sources/12.1.7-inherited-1.60.1.wikitext) — inherited literal inventory and unchanged header disagreements.
- [Source ledger](../../../data/patch-api/sources/12.1.7-source-accounting.json) — P01–P12 and deferred/UNKNOWN statuses.
- [Inherited occurrence triage](../../../data/patch-api/sources/12.1.7-inherited-1.60.1.json) — full per-occurrence linkage and bounded lexical candidates.

## See Also

- [[patch-1-60-1-api-audit]] — separate historical Forever source-only seals and behavioral gaps.
- [[patch-12-1-5-api-audit]] — previous Mainline source/publication/behavior separation.
- [[client-profiles]] — selected profile/epoch boundary.
