# Crafted item quality information

`C_TradeSkillUI.GetItemCraftedQualityInfo` queries crafted-item quality on Retail/PTR. Current cached declarations require `itemInfo: ItemInfo`, allow secret arguments only when untainted, and return nullable `CraftingQualityInfo`. See [Lua API architecture](../lua-api.md). An empty environment-owned per-item map now supplies passive inputs using the existing thirteen-field record. No crafted-item getter exists yet.

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

- `src/c_api/c_trade_skill_quality.rs`: existing recipe-keyed quality queries; no crafted-item query model.
- `src/lua_api/env_init/runtime_surface_bootstrap.lua`: missing namespace functions may become nil-returning compatibility callables; callability is not modeled behavior.
- `src/lua_api/state/sim_state.rs` and `state.rs`: separate per-item input map, empty by default.
- `tests/crafted_quality_api.rs`: public required-input and authentic VM-secret-nil boundary tests.
- `tests/crafted_quality_owned.rs` and `crafted_quality_owned/assertions.lua`: positive projection, selector, lifecycle, isolation and secret-selector tests.
- `tests/crafted_quality_item_button.rs`: whole original cached consumer candidate with real Button/Texture; fixture validity unproved.
- `Cargo.toml`: pins rilua; `rilua::table_security::unwrap_secret` supplies the existing secret-access guard.

## Tests asserting this spec

`tests/crafted_quality_api.rs` authors two cases: omitted/nil contextual rejection and secret-nil ordering/taint recovery. Both executed on Retail and PTR: each profile has two first-call rejection failures (actual exit 101). Explicit-nil/context and later tainted-denial/recovery assertions were unreached; return payloads were not printed. Five supplied-model cases and one whole-consumer candidate are authored but unexecuted. Both use statement-chunk execution, without query overrides or synthetic secrets.

### Initial crafted API RED checkpoint

[Initial whole-text evidence](../../data/patch-api/evidence/crafted-item-quality-initial-api-red-2026-10-13/README.md) owns the Retail/PTR required-input failure matrix and reached/unreached boundaries. Both profiles have executed first-call rejection failures; positive-data and whole-consumer proof remain separate. Public-copy acceptance is pending.

## Known gaps (current cycle)

- [ ] Execute positive supplied-record and whole-consumer RED on Retail/PTR before getter implementation; establish fixture baselines first. Initial required-input RED is verified separately, not positive-data or downstream security proof.
- [ ] Establish accepted ItemInfo selector representations and valid secret-selector behavior from declarations, existing parsers and real callers.
- [ ] Establish positive-data RED for complete results before implementing projection. A separate explicit per-item supplied record is a proposed simulator model, not a declared native catalog.
- [ ] Verify detached results, replacement/removal, retained snapshots, isolation and the whole original ItemButton overlay consumer. These are planned, not passing tests.

## Out of scope

Native item-catalog/classification and exact-error parity remain unproved. Recipe-quality inputs do not establish crafted-item quality; no recipe, equipment or catalog inference is authorized by this contract. Rendered/native acceptance is separate from public Lua behavior.

## Declaration evidence

Profile-scoped cache sources for both Retail and PTR: `Blizzard_APIDocumentationGenerated/TradeSkillUIDocumentation.lua` and `TradeSkillUITypesDocumentation.lua`. Required signature and thirteen-field result are local declaration observations, not authenticated native captures. Numeric IDs, numeric strings, item strings and hyperlinks remain unestablished in this stage.
