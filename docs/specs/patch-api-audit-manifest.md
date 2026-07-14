# Patch API audit manifest

Patch API audits use a checked-in JSON register for every patch-list occurrence. Draft rows have no final status. A row receives `implemented`, `best-effort`, or `exception-requested` only after item-specific evidence exists.

## What it must do

- [x] Preserve every patch-list occurrence using `change:symbol`, including names present in both added and removed lists.
- [x] Represent pending work as neutral `untriaged` rows with a null status, not as exception requests.
- [x] Restrict final statuses to `implemented`, `best-effort`, or `exception-requested`.
- [x] Restrict exception requests to `unsafe` or `impossible` resolutions.
- [x] Record target flavor/build, source owner, distinct LoD addon identity, lifecycle assertions, evidence file hashes, tests, commit, and per-item approval provenance.
- [x] Recompute patch-source, Blizzard manifest, and evidence hashes from repository files.
- [x] Verify focused test references exist and named tests occur in those files.
- [x] Verify implementation commits resolve and are ancestors of the audited checkout.
- [x] Reject unknown schema fields, blank values, invalid status/resolution combinations, and incomplete lifecycle contracts.
- [x] Ingest an observation artifact tied to the exact manifest hash and compare every assertion by row, flavor, phase, addon, presence, and Lua type.
- [x] Observe actual Lua global/table paths from a `WowLuaEnv` at caller-controlled lifecycle phases and record active profile, presence, and Lua type.
- [x] Generate initialization-phase observations only when the compiled profile matches the manifest target.
- [x] Index direct Lua function publications and flag mixin/metatable/dynamic-global/factory ambiguity without converting candidates into final statuses.
- [x] Fail synthetic vendor-present, LoD, cross-flavor, and removed-after-reset observations when flavor or phase is wrong.
- [x] Generate one compact checklist line per manifest occurrence and reject checklist or human-inventory drift.
- [ ] Generate real runtime observations for every resolved row during focused/profile test execution.
- [ ] Discover and validate 12.0.7 and 12.0.5 manifests without patch-specific Rust changes.

## Completion contract

`--complete` requires an observation artifact for the exact manifest bytes. It rejects:

- any `untriaged` row;
- missing or mismatched repository evidence;
- missing/fake named `#[test]` references or implementation commits;
- any exception whose resolution is not `unsafe` or `impossible`;
- any exception without a unique approval ID beginning `user-chat:<change:symbol>:`;
- missing, duplicated, extra, wrong-flavor, wrong-phase, wrong-addon, wrong-presence, or wrong-type observations.

The validator does not infer semantic behavior from a symbol name. Runtime observation generation remains required before this audit can complete.

## Implementation inventory

- `src/bin/wow_cli/audit_api/patch_manifest.rs` — schema, repository validation, completion gates, actual Lua-state observation primitive, initialization generator, observation comparison, and rendering.
- `src/bin/wow_cli/audit_api/patch_source_index.rs` — per-file direct-publication candidates and dynamic-publication ambiguity records.
- `data/patch-api/sources/12.1-framexml.json` — immutable raw 12.1 patch-list snapshot.
- `data/patch-api/12.1-framexml.json` — 432-row audit register.
- `docs/generated/patch-12-1-framexml-checklist.md` — generated compact checklist.
- `docs/wiki/investigations/patch-12-1-framexml-symbol-inventory.md` — human inventory whose symbol/status columns are drift-checked.

## Tests asserting this spec

- `src/bin/wow_cli/audit_api/patch_manifest.rs` — schema rejection, repository drift, exception eligibility, observation falsifiers, profile mismatch, and actual present/absent/LoD observation coverage. Post-reset remains synthetic until a concrete reset operation is defined.
- `src/bin/wow_cli/audit_api/patch_source_index.rs` — direct publication, local/comment/string exclusion, ambiguity, and file identity coverage.

## Out of scope

- Automatic semantic classification from symbol names.
- Treating source-text absence alone as proof that dynamic Lua publication cannot occur.
