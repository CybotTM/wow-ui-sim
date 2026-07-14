# Patch API Audit Manifest

Patch API audits use a checked-in JSON register for every patch-list occurrence. The register preserves duplicate names across added/removed directions, records evidence and lifecycle expectations, and generates the compact checklist used for final chat review.

## Content

### Files

- `data/patch-api/sources/12.1-framexml.json` preserves the raw 320-added/112-removed snapshot.
- `data/patch-api/12.1-framexml.json` stores all 432 rows keyed by `change:symbol`.
- `docs/generated/patch-12-1-framexml-checklist.md` contains one generated line per row.
- `src/bin/wow_cli/audit_api/patch_manifest.rs` owns parsing, repository validation, completion validation, observation comparison, and rendering.

### Draft and final state

`untriaged` is neutral draft state and has a null status. It is not an exception request. Final status vocabulary is fixed to `implemented`, `best-effort`, and `exception-requested`.

Resolution explains the evidence outcome: vendor-present, simulator compatibility behavior, load-on-demand ownership, removal, cross-flavor contamination, stale/reversed snapshot data, unsafe behavior, or impossible behavior. Source owner and the addon passed to `LoadAddOn` are separate fields; LoD lifecycle assertions must match the declared addon. Only `unsafe` and `impossible` may use `exception-requested`.

Current 12.1 FrameXML totals are **1 implemented, 19 best-effort, 0 exception-requested, and 412 untriaged**. The objective remains open.

### Repository evidence

Normal manifest validation recomputes:

- raw patch-list and Blizzard cache-manifest SHA-256 values;
- every resolved row's evidence-file SHA-256;
- focused test file and exact named `#[test]` definition existence;
- implementation commit existence and ancestry;
- raw source row order/count;
- generated checklist and human inventory status drift.

Unknown JSON fields, blank evidence, invalid lifecycle vocabulary, and incompatible status/resolution combinations fail validation.

### Runtime observations

Completion additionally consumes an observation artifact bound to the exact manifest hash. Each assertion requires exactly one observation matching row, flavor, phase, addon owner, presence, and Lua type. Missing, extra, duplicated, or mismatched observations fail.

The production observation primitive reads actual global/table paths from `WowLuaEnv` and records the active compiled profile, presence, and Lua type. Lifecycle phase and addon are caller-supplied assertion labels, not independently inferred facts. Focused coverage exercises present, absent, and a real TOC addon whose directory/TOC identity matches the declared LoD addon. `--observe-initialization` writes assertions available immediately after environment construction and rejects a manifest built for another profile. Full manifest-driven post-core/post-load/LoD/reset orchestration and checked-in per-row artifact generation remain open. Post-reset coverage is currently a synthetic validator falsifier only; it does not claim a simulator reset operation.

### Source candidates

`--index-lua-source <file> --source-addon <addon>` emits direct global/table function-publication candidates with file/line ownership and separately flags mixin, metatable, dynamic `_G`, and factory constructs as ambiguities. This is candidate evidence only: it does not turn a text match or absence into a final manifest status.

### Exception approval

Each exception must:

1. resolve to `unsafe` or `impossible`;
2. retain item-specific evidence and lifecycle assertions;
3. have a unique approval ID beginning `user-chat:<change:symbol>:`.

One approval token cannot silently approve multiple rows.

### CLI

```text
wow-cli audit-api --patch-manifest data/patch-api/12.1-framexml.json
wow-cli audit-api --patch-manifest data/patch-api/12.1-framexml.json --format plan
wow-cli audit-api --patch-manifest data/patch-api/12.1-framexml.json \
  --observe-initialization observations.json
wow-cli audit-api --index-lua-source path/to/file.lua --source-addon Blizzard_AddOn
wow-cli audit-api --patch-manifest data/patch-api/12.1-framexml.json \
  --observations path/to/observations.json --complete
```

`--complete` remains blocked while any row is untriaged or real observations are missing.

## Sources

- [Patch API audit manifest spec](../../specs/patch-api-audit-manifest.md) — testable contract.
- [[patch-12-1-api-audit]] — current patch investigation.
- [[patch-12-1-framexml-symbol-inventory]] — human-readable inventory.

## See Also

- [[client-profiles]] — flavor and retail epoch selection.
- [[addon-loading]] — load phases and load-on-demand ownership.
- [[lua-api]] — simulator runtime API surface.
