# Crafted item quality information

`C_TradeSkillUI.GetItemCraftedQualityInfo` queries crafted-item quality on Retail/PTR. Current cached declarations require `itemInfo: ItemInfo`, allow secret arguments only when untainted, and return nullable `CraftingQualityInfo`. See [Lua API architecture](../lua-api.md). An empty environment-owned per-item map supplies the existing thirteen-field record. The native getter now projects those inputs after executed positive-query and whole-original-consumer RED on Retail/PTR; GREEN remains unverified.

## What it must do

### Required inputs and security
- [ ] Reject omitted and explicit nil `itemInfo` before lookup. Simulator diagnostics identify the method and selector; native exact wording is unproved.
- [ ] Authenticate the original VM secret before validation: untainted secret nil reaches required-input rejection; tainted secret nil is denied by the existing VM access guard. Preserve the wrapper, caller taint and restored untainted access after errors. Ordering is the simulator's interpretation of `AllowedWhenUntainted`, not a native observation.

### Declared result
- [ ] A present result supplies all thirteen non-nil declared fields: numeric `quality`; texture-atlas strings `icon`, `iconSmall`, `iconInventory`, `iconMixed`, `iconAppear`, `iconDissolve`, `barFill`, `barBackground`, `barBackgroundCap`, `barHighlight`, `iconChat`, `iconQuestObjective`.

### Supplied-model policy — unverified
- [ ] Query explicit `crafted_item_quality` records independently of recipe-pair inputs. Reuse the existing numeric/decimal-string/item-string/hyperlink parser; no name catalog or native validation claim.
- [ ] Return one nil for accepted selectors without supplied data; project present records into fresh detached tables with all thirteen fields.
- [ ] Preserve retained snapshots through returned-table mutation, replacement/removal and independent environments.
- [ ] Authenticate present/absent VM-secret selectors before type/lookup and preserve wrapper/taint recovery.
- [ ] Let the whole original cached ItemButton handler consume supplied `iconInventory`, flags, anchors and visibility. Its visibility input is a real host-created Frame, not the original professions panel; native/rendered panel acceptance remains separate.

## How it works

- [C API and Lua surface inventory](../wiki/systems/lua-api.md)
- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/c_api/c_trade_skill_quality.rs`: item-keyed native getter, original-secret validation, Retail/PTR registration and shared fresh snapshot projection; recipe queries retain their distinct input map.
- `src/lua_api/env_init/runtime_surface_bootstrap.lua`: missing namespace functions may become nil-returning compatibility callables; callability is not modeled behavior.
- `src/lua_api/state/sim_state.rs` and `state.rs`: separate per-item input map, empty by default.
- `tests/crafted_quality_api.rs`: public required-input and authentic VM-secret-nil boundary tests.
- `tests/crafted_quality_owned.rs` and `crafted_quality_owned/assertions.lua`: positive projection, selector, lifecycle, isolation and secret-selector tests.
- `tests/crafted_quality_item_button.rs`: whole original cached consumer with real Button/Texture; Retail/PTR closure and empty-input baselines passed before positive branch RED. Later overlay assertions remain unproved.
- `Cargo.toml`: pins rilua; `rilua::table_security::unwrap_secret` supplies the existing secret-access guard.

## Tests asserting this spec

`tests/crafted_quality_api.rs` authors two cases: omitted/nil contextual rejection and secret-nil ordering/taint recovery. Both executed on Retail and PTR: each profile has two first-call rejection failures (actual exit 101). Explicit-nil/context and later tainted-denial/recovery assertions were unreached; return payloads were not printed. Five supplied-model cases and the whole consumer also executed on both profiles: five first-positive table-check failures plus one original-consumer positive-flag failure per profile. Exact field/alias/lifecycle/security/overlay checks later in those tests were unreached. Fixtures execute statement chunks without query overrides or synthetic secrets.

### Initial crafted API RED checkpoint

[Initial whole-text evidence](../../data/patch-api/evidence/crafted-item-quality-initial-api-red-2026-10-13/README.md) owns the Retail/PTR required-input failure matrix and reached/unreached boundaries. Both profiles have executed first-call rejection failures; both profiles subsequently reached five positive-query failures and one genuine original ItemButton consumer failure, after successful closure/empty-input baselines. Downstream field/security/visibility/clear assertions remained unreached. Initial public-copy and retained-gate copy checks pass at their recorded scopes; producer GREEN remains pending.

## Known gaps (current cycle)

- [ ] Verify producer GREEN on Retail/PTR for the two initial, five owned and one whole-consumer cases. Preserve executed initial/positive RED and its exact reached boundaries; do not infer downstream security or visibility acceptance from earlier failures.
- [ ] Establish accepted ItemInfo selector representations and valid secret-selector behavior from declarations, existing parsers and real callers.
- [ ] Verify detached results, replacement/removal, retained snapshots, isolation and the whole original ItemButton overlay consumer. These are planned, not passing tests.

## Out of scope

Native item-catalog/classification and exact-error parity remain unproved. Recipe-quality inputs do not establish crafted-item quality; no recipe, equipment or catalog inference is authorized by this contract. Rendered/native acceptance is separate from public Lua behavior.

## Declaration evidence

Profile-scoped cache sources for both Retail and PTR: `Blizzard_APIDocumentationGenerated/TradeSkillUIDocumentation.lua` and `TradeSkillUITypesDocumentation.lua`. Required signature and thirteen-field result are local declaration observations, not authenticated native captures. Numeric IDs, decimal strings, item strings and hyperlinks are the selected existing-parser model forms. Their complete runtime alias behavior and native validation remain unproved until appropriate GREEN/native evidence.
