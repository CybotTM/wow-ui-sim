# Patch 12.1.7 source accounting

Slice I0 freezes the public 12.1.7 source, its linked current 1.60.1 inventory and the TOC reference under `data/patch-api/sources/12.1.7-*`. Frozen page target: **12.1.7 build 70424, interface 120107**. The selected runtime source moved to **wowt 12.1.7.70493** on 2026-10-23; [source pin and regeneration](ptr-blizzard-ui-source.md) own that target. Frozen bodies/provenance remain historical, not 70493 evidence. Exact body SHA-256 identities replace unavailable MediaWiki revision IDs; no revision number is invented. [Investigation and reproduction](../wiki/investigations/patch-12-1-7-api-audit.md). This is source accounting, not implemented API coverage.

## What it must do

- [x] Validate frozen source bytes/hashes, source-linked P01–P12 and the supplemental patch-page-index entry offline; reject an empty API register substituted for the prose ledger.
- [x] Preserve all 1,876 inherited literal occurrences, including repeated names with distinct source-line IDs, defaults, command kinds and header disagreements; reject row identity/count corruption.
- [x] Keep inherited rows **UNKNOWN until slice I4** and return `runtime_complete: false`, even when every source row is accounted.
- [x] Retain GUID rows as a **deferred gap pending the user decision**, not as build-70424 behavior or implemented coverage.

### Direct prose and preservation rows

Source line numbers refer to the frozen wikitext files. P10–P11 incorporate the TOC reference rather than pretending the patch page contains an API table. The machine ledger retains exact source paths/lines for every row.

| ID | Requirement | Status at I0 |
|---|---|---|
| P01 | Includes API changes from 1.60.1. | Accounted; inherited applicability/publication/behavior **UNKNOWN until I4**. |
| P02 | Obfuscate Cast GUID server/spawn portions. | **DEFERRED**, pending user decision and future-build oracle. Existing `Cast-Sim-*` spelling is not native-format proof. |
| P03 | Obfuscate Creature GUID server/spawn portions. | **DEFERRED**, same decision/oracle boundary. |
| P04 | Obfuscate Vehicle GUID server/spawn portions. | **DEFERRED**; complete native GUID production is not established. |
| P05 | Obfuscate Pet GUID server/spawn portions. | **DEFERRED**; unsupported token/producer limits remain separate. |
| P06 | Obfuscate game object GUID server/spawn portions. | **DEFERRED**; supplied fixtures do not establish an identity producer. |
| P07 | Obfuscated values stable/unique in one session, incomparable across clients/restarts; GUID format otherwise unchanged. | **DEFERRED**; no algorithm, encoding, widths or secrecy changes invented. |
| P08 | Skip variable-bearing addon metadata directives entirely, rather than expanding them. | Accounted; **PENDING I2a** implementation and loader-facing proof. Not removal of file expansion. |
| P09 | Preserve conditional directives, including AllowLoadGameType. | Accounted; **PENDING I2a** preservation proof with metadata rejection. |
| P10 | Add RegionalTextLocale file variable from configured text locale and region; English/Portuguese US/EU examples. | Accounted from TOC reference; **PENDING I2b**, shared PTR/Forever capability. No blanket locale policy inferred. |
| P11 | Preserve existing Game/Family/TextLocale file variables. | Accounted from TOC reference; **PENDING I2b** ordered-loading preservation proof. |
| P12 | Interface/epoch 120107; page identifies build 70424. | Accounted; **PENDING I1/I3** selection/source-pin proof. Retail 120105 and Forever 16001 remain separate. |

The patch explicitly says GUID changes will be testable in a **future build of 12.1.7**. Documented build 70424 is not their native oracle. Deferral does not block the other slices and does not close P02–P07.

### Inherited 1.60.1 linkage

P01 links the current frozen 1.60.1 page, not the separately sealed historical audit. Its literal rows are retained without deduplication in `data/patch-api/sources/12.1.7-inherited-1.60.1.json`.

| Category | Added | Removed | Accounting boundary |
|---|---:|---:|---|
| Global API | 175 | 10 | Header says **176** additions; preserve disagreement, not a fabricated row. |
| FrameXML | 471 | 1,041 | Vendor changes, not 1,512 simulator Rust endpoints or native removals. |
| Widgets | 7 | 0 | Four ScriptRegion input methods, two Minimap methods and one 3D actor method; permanent 3D exclusion remains. |
| Events | 25 | 2 | Header says **24** additions; registerability alone is not payload/lifecycle proof. |
| CVar column | 138 | 7 | Added column contains **137 CVars plus command `dumpSmallAlloc`**. Command is not a CVar. Preserve literal defaults and absent defaults. |

No standalone enum table appears. **Every inherited row is UNKNOWN until I4** resolves row-specific PTR applicability, publication, behavior and native/original-consumer proof. Forever implementations are reuse candidates, not automatic PTR credit. Retail 12.1.5 remains unchanged without row-specific evidence. The [existing 1.60.1 spec](patch-1-60-1-source-accounting.md) and its immutable seals remain unchanged; source accounting never upgrades their behavioral gaps.

## I4a inherited applicability worklist

The companion `data/patch-api/sources/12.1.7-inherited-applicability.json` extends accounting without changing any I0 source, ledger or status bytes. Base revision: `571284aaa8bb29f17d27e90110e4f863921d9330`. Its 1,876 records retain complete literal occurrences, all six opposite-direction PvPObjectiveBanner IDs, 816 additions/1,060 removals, header disagreements, 144 CVars and the distinct `dumpSmallAlloc` command.

- [x] Validate exact IDs and literal fields against the frozen source, not counts alone; retain independent PTR/Forever/Retail applicability, publication, behavior, original-consumer and native statuses.
- [x] Retain 63 exact opposing-direction sweep intersections across 14 Retail modules, plus the register-only `GetArmorPenetration` reference. These provide no inherited-direction completion credit.
- [x] Retain 233 rows with exact-symbol lexical test candidates, separately from the 122 original source-candidate rows. Strings/comments and old assertions are discovery only; namespace suffix matches grant zero credit.
- [x] Reject disposable occurrence/default/kind corruption, direction collapse, command confusion, wrong namespace, unselected consumers, lexical upgrades, wrong-profile/build evidence and old-direction completion credit.
- [x] Preserve permanent unsupported 3D effects for the actor method; publication/factory status remains UNKNOWN.

**Accounting resolved: 1,876/1,876. Applicability resolved: 0/5,628 profile cells; unresolved: 1,876 per profile.** Selected publisher ownership, TOC chains and source binding are unresolved for every row. `literal_owner` records only a spelling prefix, not a proven selected addon. Proposed selector names are grouping work, not existing Rust selectors or applicability decisions. A fully UNKNOWN map is valid accounting, not closure of I4.

I3 has not accepted source revision `e30d17b106ce76a872c2bdfe69f95b4ec215fd06`: required PTR **wowt 12.1.7.70493**, Gethe `2cdbc7ff511ec6a8c2f4f8341accd1da44a3c9af`, 4,578 files. The base still pins **wowxptr 12.1.5.69594**, Gethe `49b69918fcdc77e109813281e4f537d45ec7dcbf`, 4,025 files. No PTR cache was read as 12.1.7. Public page build 70424 remains separate from the pending runtime pin. Forever needs its own accepted 1.60.1 source/selection receipts; Retail remains 12.1.5 preservation, never a surrogate.

`python3 -B tools/test_patch_12_1_7_inherited.py` exercises real validator CLI mutations in disposable roots. `python3 -B tools/validate_patch_12_1_7_inherited.py` validates the companion and unchanged I0 boundary. Historical joins use bounded existing registers, so unrelated later patch additions do not expand this audit. I4b owns accepted-source binding, selected Game/Glue/LoD ownership and grouped runtime observations; no producer or native-parity implementation belongs to I4a.

## How it works

- [Source import, validator and evidence limits](../wiki/investigations/patch-12-1-7-api-audit.md).
- [Historical Forever accounting](patch-1-60-1-source-accounting.md).

## Implementation inventory

- `data/patch-api/sources/12.1.7-api-changes.provenance.json`: public fetch identities, body hashes and target; three matching frozen wikitext files.
- `data/patch-api/sources/12.1.7-source-accounting.json`: direct prose, inherited linkage, source-only status and GUID hold.
- `data/patch-api/sources/12.1.7-inherited-1.60.1.json`: all literal rows, header counts and UNKNOWN triage; repository-relative lexical candidates only.
- `data/patch-api/patch-page-index.json`: supplemental 12.1.7 discovery, explicitly not a new allpages census.
- `tools/validate_patch_12_1_7_source.py`: Python-only validation using the existing literal parser; no shared generator changes or historical resealing.

## Tests asserting this spec

`tools/test_patch_12_1_7_source.py`: nine direct CLI fixtures cover empty-register substitution, accepted source-only output, omitted direct row, malformed inherited counts, repeated-name identity, missing index entry, changed source bytes, false inherited implementation credit and false direct completion credit. Run directly with `python3 -B tools/test_patch_12_1_7_source.py`; no Cargo build.

## Known gaps (current cycle)

- [ ] I1–I3: epoch selection, metadata rejection, regional file loading and exact PTR source refresh/runtime proof.
- [x] I4a: lossless inherited accounting and per-profile UNKNOWN worklist; zero Cargo.
- [ ] I4b and later: all 1,876 inherited applicability/behavior classifications remain UNKNOWN; lexical candidates are not implementation credit.
- [ ] I5: GUID timing decision and native oracle; P02–P07 remain deferred.
- [ ] I6: independent runtime/profile acceptance. A source-only validator success cannot satisfy it.

## Out of scope

Runtime implementation, vendor changes, new GUID policy, native parity, builds, source-cache synchronization and historical seal changes. The optional `/notes` fetch returned 404 and is excluded as non-authoritative. This ledger does not claim rendered/transcluded MediaWiki completeness.
