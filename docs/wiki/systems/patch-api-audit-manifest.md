# Patch API Audit Manifest

Patch API audits use a checked-in JSON register for every patch-list occurrence. The register preserves duplicate names across added, changed, and removed directions, records evidence and lifecycle expectations, and generates the compact checklist used for final chat review.

## Content

### Files

- `tools/gen_patch_12_0_0_register.py`, `data/patch-api/sources/12.0.0-register.json`, `data/patch-api/12.0.0.json`, `docs/generated/patch-12-0-0-checklist.md`, and [[patch-12-0-0-occurrence-inventory]] preserve the current 12.0.0 wowless snapshot audit: six 12.0.0 snapshots, 60 best-effort, 91 evidence-required, and 3259 untriaged occurrences (3410 total), and 2554 added / 313 changed / 543 removed. The bounded FunctionContainer slice covers named userdata_proxy behavior; exact retail callback validation, metatable/equality identity beyond tests, timer integration, lifecycle/GC, and API metadata fidelity remain unproven. The bounded C_Timer slice covers NewTimer/NewTicker function/container acceptance, returned container identity/proxy equality, cancellation, and independent ticker counts from named tests; exact scheduling, lifecycle, GC, and edge semantics remain unproven. C_Timer.After remains evidence-required unsafe because its only focused-looking test is ignored; callback/lifecycle semantics require a correct modeled implementation and executable behavioral proof, with no approval path. The bounded duration slice covers only factory/object shape, default-zero behavior, per-instance fields, tostring, and exact StatusBar duration-object identity round-trip; the 21 unsafe duration rows remain open because current behavior is constant/no-op/incomplete. The nine best-effort curve rows cover only tested factory/table shape, scalar interpolation/copy behavior, per-instance fields/tostring, and color-object/copy shape; the 23 unresolved curve contracts remain evidence-required and cannot be approved closed because the current generic proxy omits or does not faithfully establish them. 17 curve-family metadata rows remain untriaged. The bounded C_StringUtil slice adds one best-effort behavioral row limited to tested quoted-code pipe escaping and eight evidence-required unsafe rows because the current model does not publish them; authoritative semantics or correct implementations are required and no approval can close them. The bounded C_ColorUtil slice adds two best-effort behavioral rows limited to the tested RGB-to-ffRRGGBB code and explicit color-code text wrapping; five conversion/WrapTextInColor rows remain evidence-required unsafe because current behavior is absent, placeholder/identity/max-channel, or lacks focused proof, and edge/secret/localization/clamping semantics remain unproven. The twelve-row C_Spell slice covers duration-object lifecycle, spell metadata/display, and boolean contracts without focused proof; authoritative evidence or correct models are required, and no approval can close these rows. The bounded C_DamageMeter slice adds exactly 19 best-effort behavioral rows limited to exact seeded/empty/shape/type/lookup assertions; 10 seeded-but-unasserted/reset rows remain evidence-required unsafe with no approval path; six metadata-only structure rows remain untriaged; no complete retail aggregation/lifecycle/secret fidelity is claimed. Each normalized occurrence may also preserve typed `before`/`after` payloads with category, value, and metadata for exact triage; this does not change row identity or status. The source covers wowless schema surfaces, not historical FrameXML or live runtime behavior; the active retail cache manifest is validation metadata only.
- `data/patch-api/sources/12.1-framexml.json` preserves the legacy raw 320-added/112-removed direction-array snapshot.
- `data/patch-api/sources/12.1-behaviors.json`, `data/patch-api/12.1-behaviors.json`, `docs/generated/patch-12-1-behavior-checklist.md`, and [[patch-12-1-behavior-inventory]] preserve 54 independently testable non-FrameXML behavior boundaries: 33 best-effort and 21 evidence-required; candidate disposition is 33 behavioral, 21 unsafe, and 0 impossible.
- `data/patch-api/sources/12.0.7-register.json` preserves 131 named occurrences as categorized `{direction, category, symbol, detail?}` objects without inventing the unnamed CVar additions/removals omitted by the crawler excerpt.
- `data/patch-api/12.0.7.json`, `docs/generated/patch-12-0-7-checklist.md`, and [[patch-12-0-7-occurrence-inventory]] form the occurrence register; all 131 named rows are classified as 29 implemented, 101 best-effort, and one impossible exception-requested row authorized by the repository's permanent no-3D project scope, with no untriaged rows.
- `data/patch-api/sources/12.0.5-probes.json`, `data/patch-api/12.0.5-probes.json`, `docs/generated/patch-12-0-5-probe-checklist.md`, and [[patch-12-0-5-probe-inventory]] preserve 38 probe subfindings: 33 best-effort, 4 evidence-required, 1 approved provenance-only exception-requested, and 0 untriaged.
- `data/patch-api/12.1-framexml.json` stores all 432 rows keyed by `change:symbol`.
- `docs/generated/patch-12-1-framexml-checklist.md` contains one generated line per row.
- `src/bin/wow_cli/audit_api/patch_manifest.rs` owns generic added/changed/removed parsing, repository validation, completion validation, observation comparison, and rendering.

### Draft and final state

`untriaged` is neutral draft state and has a null status. `evidence-required` is triaged unresolved `unsafe`/`impossible` behavior requiring item-specific authoritative/live evidence; it needs no approval, commit, or focused test and cannot pass `--complete`. `exception-requested` is a distinct exception-handling path with approval or allowlisted-scope requirements. Final status vocabulary is `implemented`, `best-effort`, `evidence-required`, and `exception-requested`.

### Occurrence payloads and identity

Categorized source occurrence objects accept only their defined fields: `direction`, `category`, `symbol`, optional `detail`, and optional typed `before`/`after` JSON payloads. Payloads preserve normalized `category`, `value`, and `metadata` data for exact enum, constant, signature, and structure triage. Added occurrences carry `after`; removed occurrences carry `before`; changed occurrences carry both; transient add/remove rows carry the corresponding side. Row identity remains `direction+symbol`, so payload changes do not create new rows. Unknown fields remain validation errors.

Resolution explains the evidence outcome: vendor-present, simulator compatibility behavior, test-backed behavioral behavior, load-on-demand ownership, removal, cross-flavor contamination, stale/reversed snapshot data, unsafe behavior, or impossible behavior. `behavioral` is for contracts that are not simple Lua-path presence checks, such as stateful globals, event registration, widget methods, CVars, and probe outcomes: it requires hashed test evidence plus a focused named test, permits implemented/best-effort status, and forbids fabricated Lua-path assertions. Source owner and the addon passed to `LoadAddOn` are separate fields; LoD lifecycle assertions must match the declared addon. Only `unsafe` and `impossible` may use `evidence-required` or `exception-requested`.

Current 12.1 FrameXML totals are **1 implemented, 431 best-effort, 0 evidence-required, 0 exception-requested, and 0 untriaged**. The separate broader behavior register is **0 implemented, 33 best-effort, 21 evidence-required, 0 exception-requested, and 0 untriaged**. The 21 unsafe rows remain open for authoritative/live evidence; they are not approval candidates.

### Repository evidence

Normal manifest validation recomputes:

- raw patch-list and Blizzard cache-manifest SHA-256 values;
- every resolved row's evidence-file SHA-256;
- focused test file and exact named `#[test]` definition existence;
- implementation commit existence and ancestry;
- raw source row order/count for either direction arrays or categorized `occurrences`; object sources validate direction, nonblank category, optional nonblank change detail, symbol path, optional typed `before`/`after` payloads, and deterministic added/changed/removed grouping; older manifests omit `changed_count` and default it to zero;
- generated checklist and human inventory status drift.

Unknown JSON fields, blank evidence, invalid lifecycle vocabulary, and incompatible status/resolution combinations fail validation.

### Runtime observations

Completion additionally consumes an observation artifact bound to the exact manifest hash. Each Lua-path assertion requires exactly one observation matching row, flavor, phase, addon owner, presence, and Lua type. Behavioral rows intentionally carry no Lua-path assertions or observations; their focused test/evidence/commit references remain repository-validated. Evidence-required unsafe/impossible rows may also omit assertions when item-specific evidence concerns provenance or another non-Lua boundary; they require no approval, commit, or focused test and remain completion blockers. Exception-requested rows retain their approval or allowlisted scope requirements. Missing, extra, duplicated, or mismatched observations fail.

The production observation primitive reads actual global/table paths from `WowLuaEnv` and records the active compiled profile, presence, and Lua type. Lifecycle phase and addon are caller-supplied assertion labels, not independently inferred facts. Focused coverage exercises present, absent, and a real TOC addon whose directory/TOC identity matches the declared LoD addon. `--observe-initialization` writes assertions available immediately after environment construction and rejects a manifest built for another profile. Full manifest-driven post-core/post-load/LoD/reset orchestration and checked-in per-row artifact generation remain open. Post-reset coverage is currently a synthetic validator falsifier only; it does not claim a simulator reset operation.

### Source candidates

`--index-lua-source <file> --source-addon <addon>` emits direct global/table function and alias publication candidates for one file. `--index-lua-tree <AddOns>` emits the same records with deterministic relative paths and first-directory addon ownership across all Lua files. Add `--active-tocs` to restrict the tree to TOCs selected by the active compiled profile's `find_toc_file` resolver and recursively follow XML `<Script>`/`<Include>` paths through the loader's addon-root fallback and case-insensitive resolver. Missing Lua/XML references, including TOC-listed Lua/XML entries and XML `<Script>`/`<Include>` targets, are retained in the output's `missing` list. The lexer masks comments, quoted/long strings, and file-local namespaces; `_G.Name` is normalized to `Name`; source identities carry SHA-256 hashes; and multiple mixin/metatable/dynamic-`_G`/factory ambiguities on one line are retained. Active-TOC output identifies source candidates selected by active TOCs and per-file environment rules, but does not infer dependency order or startup versus load-on-demand timing. This is candidate evidence only: neither a text match nor absence changes a final manifest status.

### Evidence and exception provenance

Evidence-required rows must:

1. resolve to `unsafe` or `impossible`;
2. retain item-specific evidence;
3. omit approval, commit, and focused-test requirements until authoritative/live evidence resolves the row.

Each `exception-requested` row must:

1. resolve to `unsafe` or `impossible`;
2. retain item-specific evidence, plus lifecycle assertions when the exception concerns Lua-path presence;
3. require a unique `user-chat:<change:symbol>:` approval for `unsafe` rows;
4. require either that user approval or an allowlisted repository scope exception for `impossible` rows.

`approval_id` and `scope_exception` are mutually exclusive. Scope exceptions are accepted only for impossible rows and are repository-validated against the allowlisted rule, `AGENTS.md#intentional-gaps`, and the required no-3D rule text. The repository rule is the authority for permanent project-scope gaps; it is not a newly granted user exception.

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

`--complete` remains blocked while any row is untriaged, evidence-required, or real observations are missing.

## Sources

- [Patch API audit manifest spec](../../specs/patch-api-audit-manifest.md) — testable contract.
- [[patch-12-1-api-audit]] — current patch investigation.
- [[patch-12-1-framexml-symbol-inventory]] — human-readable inventory.

## See Also

- [[client-profiles]] — flavor and retail epoch selection.
- [[addon-loading]] — load phases and load-on-demand ownership.
- [[lua-api]] — simulator runtime API surface.
