# Patch API audit manifest

Patch API audits use a checked-in JSON register for every patch-list occurrence. Draft rows have no final status. A row receives `implemented`, `best-effort`, or `exception-requested` only after item-specific evidence exists.

## What it must do

- [x] Preserve every patch-list occurrence using `change:symbol`, including `added`, `changed`, and `removed` directions and names present in multiple direction lists.
- [x] Represent pending work as neutral `untriaged` rows with a null status, not as exception requests.
- [x] Restrict final statuses to `implemented`, `best-effort`, or `exception-requested`.
- [x] Represent test-backed behavior contracts that are not simple Lua-path presence checks with `behavioral` resolution: implemented/best-effort only, at least one hashed test-evidence item plus a focused named test, and no fabricated Lua-path assertion or runtime observation.
- [x] Restrict exception requests to `unsafe` or `impossible` resolutions.
- [x] Allow unsafe/impossible exception rows to omit fabricated presence assertions only when item-specific evidence concerns a non-Lua boundary; item-specific evidence and a unique per-row approval remain mandatory for completion.
- [x] Record target flavor/build, source owner, distinct LoD addon identity, lifecycle assertions, evidence file hashes, tests, commit, and per-item approval provenance.
- [x] Recompute patch-source, Blizzard manifest, and evidence hashes from repository files.
- [x] Accept legacy direction arrays or a generic categorized `occurrences` array; validate each occurrence's direction, nonblank category, optional nonblank change detail, symbol path, and deterministic added/changed/removed ordering.
- [x] Verify focused test references exist and named tests occur in those files.
- [x] Verify implementation commits resolve and are ancestors of the audited checkout.
- [x] Reject unknown schema fields, blank values, invalid status/resolution combinations, and incomplete lifecycle contracts.
- [x] Ingest an observation artifact tied to the exact manifest hash and compare every assertion by row, flavor, phase, addon, presence, and Lua type.
- [x] Observe actual Lua global/table paths from a `WowLuaEnv` at caller-controlled lifecycle phases and record active profile, presence, and Lua type.
- [x] Generate initialization-phase observations only when the compiled profile matches the manifest target.
- [x] Lexically exclude Lua comments, strings, and file-local namespaces while indexing direct function/alias publications; normalize `_G` names; retain per-source SHA-256 identity; and flag every mixin/metatable/dynamic-global/factory ambiguity on a line without converting candidates into final statuses.
- [x] Restrict tree candidates to Lua files selected from TOCs by the active-profile resolver and per-file environment rules; follow XML references through the loader's fallback and case-insensitive resolver; preserve unresolved Lua/XML paths.
- [ ] Apply dependency order and startup/LoD lifecycle timing separately; TOC reachability does not establish either.
- [x] Fail synthetic vendor-present, LoD, cross-flavor, and removed-after-reset observations when flavor or phase is wrong.
- [x] Generate one compact checklist line per manifest occurrence and reject checklist or human-inventory drift.
- [ ] Generate real runtime observations for every resolved row during focused/profile test execution.
- [ ] Discover and validate 12.0.7 and 12.0.5 manifests without patch-specific Rust changes. Generic source validation already accepts an optional `changed` array and `changed_count` while older added/removed-only manifests default that count to zero.

## Completion contract

`--complete` requires an observation artifact for the exact manifest bytes. Behavioral rows are proven by their repository-validated test evidence and intentionally require no Lua-path observation. Unsafe/impossible rows may likewise omit observations only when item-specific evidence concerns provenance or another non-Lua boundary; item-specific evidence and a unique per-row approval remain mandatory. It rejects:

- any `untriaged` row;
- missing or mismatched repository evidence;
- missing/fake named `#[test]` references or implementation commits;
- any exception whose resolution is not `unsafe` or `impossible`;
- any exception without a unique approval ID beginning `user-chat:<change:symbol>:`;
- missing, duplicated, extra, wrong-flavor, wrong-phase, wrong-addon, wrong-presence, or wrong-type observations.

The validator does not infer semantic behavior from a symbol name. Runtime observation generation remains required before this audit can complete.

## Implementation inventory

- `src/bin/wow_cli/audit_api/patch_manifest.rs` — generic added/changed/removed schema, repository validation, completion gates, actual Lua-state observation primitive, initialization generator, observation comparison, and rendering.
- `src/bin/wow_cli/audit_api/patch_source_index.rs` — per-file direct-publication candidates, dynamic-publication ambiguity records, all-source tree indexing, and active-profile TOC/XML reachability indexing.
- `data/patch-api/sources/12.1-framexml.json` — immutable raw 12.1 direction-array snapshot.
- `data/patch-api/sources/12.1-behaviors.json` — 53 independently testable non-FrameXML behavior boundaries, grouped by the existing broader audit families; candidate disposition remains separate from machine status.
- `data/patch-api/sources/12.0.7-register.json` — categorized raw register for the 131 named 12.0.7 occurrences; changed rows keep a normalized symbol plus exact detail, while unresolved unnamed CVar claims remain metadata rather than invented rows.
- `data/patch-api/12.0.7.json` — 131-row occurrence manifest: 29 implemented, 101 best-effort, one unapproved impossible exception candidate, and no untriaged rows.
- `docs/generated/patch-12-0-7-checklist.md` — generated occurrence checklist.
- `docs/wiki/investigations/patch-12-0-7-occurrence-inventory.md` — human-readable occurrence inventory.
- `data/patch-api/sources/12.0.5-probes.json` — categorized source for 38 retained probe subfindings, with prior documented states preserved separately from machine status.
- `data/patch-api/12.0.5-probes.json` — 38-row probe manifest: 31 best-effort rows with direct evidence and 7 untriaged rows.
- `docs/generated/patch-12-0-5-probe-checklist.md` — generated probe checklist.
- `docs/wiki/investigations/patch-12-0-5-probe-inventory.md` — human-readable probe inventory.
- `data/patch-api/12.1-framexml.json` — complete 432-row FrameXML symbol audit register.
- `docs/generated/patch-12-1-framexml-checklist.md` — generated FrameXML checklist.
- `docs/wiki/investigations/patch-12-1-framexml-symbol-inventory.md` — FrameXML human inventory whose symbol/status columns are drift-checked.
- `data/patch-api/12.1-behaviors.json` — 54-row broader behavior manifest: 15 direct-test-backed best-effort rows and 39 untriaged rows.
- `docs/generated/patch-12-1-behavior-checklist.md` — generated broader behavior checklist.
- `docs/wiki/investigations/patch-12-1-behavior-inventory.md` — broader behavior inventory with 30 safe-best-effort, 21 unsafe, and 3 impossible candidate dispositions.

## Tests asserting this spec

- `src/bin/wow_cli/audit_api/patch_manifest.rs` — schema rejection, repository drift, exception eligibility, observation falsifiers, profile mismatch, and actual present/absent/LoD observation coverage. Post-reset remains synthetic until a concrete reset operation is defined.
- `src/bin/wow_cli/audit_api/patch_source_index.rs` — direct publication, local/comment/string exclusion, ambiguity, file identity, active-profile TOC selection, and recursive XML include coverage.

## Out of scope

- Automatic semantic classification from symbol names.
- Treating source-text absence alone as proof that dynamic Lua publication cannot occur.
