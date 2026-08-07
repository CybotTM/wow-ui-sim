# Patch API Audit Manifest

Patch API audits use a checked-in JSON register for every patch-list occurrence. The register preserves duplicate names across added, changed, and removed directions, records evidence and lifecycle expectations, and generates the compact checklist used for final chat review.

## Content

### Files

- `data/patch-api/sources/12.1-framexml.json` preserves the legacy raw 320-added/112-removed direction-array snapshot.
- `data/patch-api/sources/12.0.7-register.json` preserves 131 named occurrences as categorized `{direction, category, symbol, detail?}` objects without inventing the unnamed CVar additions/removals omitted by the crawler excerpt.
- `data/patch-api/12.0.7.json`, `docs/generated/patch-12-0-7-checklist.md`, and [[patch-12-0-7-occurrence-inventory]] form the occurrence register; all 131 named rows are classified as 29 implemented, 101 best-effort, and one unapproved impossible 3D-model exception candidate, with no untriaged rows.
- `data/patch-api/sources/12.0.5-probes.json`, `data/patch-api/12.0.5-probes.json`, `docs/generated/patch-12-0-5-probe-checklist.md`, and [[patch-12-0-5-probe-inventory]] preserve 38 probe subfindings. Fifteen rows have direct best-effort behavioral evidence and 23 remain untriaged; prior documented states remain separate from machine status.
- `data/patch-api/12.1-framexml.json` stores all 432 rows keyed by `change:symbol`.
- `docs/generated/patch-12-1-framexml-checklist.md` contains one generated line per row.
- `src/bin/wow_cli/audit_api/patch_manifest.rs` owns generic added/changed/removed parsing, repository validation, completion validation, observation comparison, and rendering.

### Draft and final state

`untriaged` is neutral draft state and has a null status. It is not an exception request. Final status vocabulary is fixed to `implemented`, `best-effort`, and `exception-requested`.

Resolution explains the evidence outcome: vendor-present, simulator compatibility behavior, test-backed behavioral behavior, load-on-demand ownership, removal, cross-flavor contamination, stale/reversed snapshot data, unsafe behavior, or impossible behavior. `behavioral` is for contracts that are not simple Lua-path presence checks, such as stateful globals, event registration, widget methods, CVars, and probe outcomes: it requires hashed test evidence plus a focused named test, permits implemented/best-effort status, and forbids fabricated Lua-path assertions. Source owner and the addon passed to `LoadAddOn` are separate fields; LoD lifecycle assertions must match the declared addon. Only `unsafe` and `impossible` may use `exception-requested`.

Current 12.1 FrameXML totals are **1 implemented, 431 best-effort, 0 exception-requested, and 0 untriaged**. The objective remains open.

### Repository evidence

Normal manifest validation recomputes:

- raw patch-list and Blizzard cache-manifest SHA-256 values;
- every resolved row's evidence-file SHA-256;
- focused test file and exact named `#[test]` definition existence;
- implementation commit existence and ancestry;
- raw source row order/count for either direction arrays or categorized `occurrences`; object sources validate direction, nonblank category, optional nonblank change detail, symbol path, and deterministic added/changed/removed grouping; older manifests omit `changed_count` and default it to zero;
- generated checklist and human inventory status drift.

Unknown JSON fields, blank evidence, invalid lifecycle vocabulary, and incompatible status/resolution combinations fail validation.

### Runtime observations

Completion additionally consumes an observation artifact bound to the exact manifest hash. Each Lua-path assertion requires exactly one observation matching row, flavor, phase, addon owner, presence, and Lua type. Behavioral rows intentionally carry no Lua-path assertions or observations; their focused test/evidence/commit references remain repository-validated. Unsafe/impossible rows may also omit assertions only when item-specific evidence concerns provenance or another non-Lua boundary; item-specific evidence and a unique per-row approval remain mandatory. Missing, extra, duplicated, or mismatched observations fail.

The production observation primitive reads actual global/table paths from `WowLuaEnv` and records the active compiled profile, presence, and Lua type. Lifecycle phase and addon are caller-supplied assertion labels, not independently inferred facts. Focused coverage exercises present, absent, and a real TOC addon whose directory/TOC identity matches the declared LoD addon. `--observe-initialization` writes assertions available immediately after environment construction and rejects a manifest built for another profile. Full manifest-driven post-core/post-load/LoD/reset orchestration and checked-in per-row artifact generation remain open. Post-reset coverage is currently a synthetic validator falsifier only; it does not claim a simulator reset operation.

### Source candidates

`--index-lua-source <file> --source-addon <addon>` emits direct global/table function and alias publication candidates for one file. `--index-lua-tree <AddOns>` emits the same records with deterministic relative paths and first-directory addon ownership across all Lua files. Add `--active-tocs` to restrict the tree to TOCs selected by the active compiled profile's `find_toc_file` resolver and recursively follow XML `<Script>`/`<Include>` paths through the loader's addon-root fallback and case-insensitive resolver. Missing Lua/XML references, including TOC-listed Lua/XML entries and XML `<Script>`/`<Include>` targets, are retained in the output's `missing` list. The lexer masks comments, quoted/long strings, and file-local namespaces; `_G.Name` is normalized to `Name`; source identities carry SHA-256 hashes; and multiple mixin/metatable/dynamic-`_G`/factory ambiguities on one line are retained. Active-TOC output identifies source candidates selected by active TOCs and per-file environment rules, but does not infer dependency order or startup versus load-on-demand timing. This is candidate evidence only: neither a text match nor absence changes a final manifest status.

### Exception approval

Each exception must:

1. resolve to `unsafe` or `impossible`;
2. retain item-specific evidence, plus lifecycle assertions when the exception concerns Lua-path presence;
3. have a unique approval ID beginning `user-chat:<change:symbol>:`.

One approval token cannot silently approve multiple rows.

### CLI

```text
wow-cli audit-api --patch-manifest data/patch-api/12.1-framexml.json
wow-cli audit-api --patch-manifest data/patch-api/12.1-framexml.json --format plan
wow-cli audit-api --patch-manifest data/patch-api/12.1-framexml.json \
  --observe-initialization observations.json
wow-cli audit-api --index-lua-source path/to/file.lua --source-addon Blizzard_AddOn
wow-cli audit-api --index-lua-tree path/to/AddOns --active-tocs \
  --source-index-output source-index.json
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
