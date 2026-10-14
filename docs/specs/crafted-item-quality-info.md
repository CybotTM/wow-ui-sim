# Crafted item quality information

`C_TradeSkillUI.GetItemCraftedQualityInfo` queries crafted-item quality on Retail/PTR. Current cached declarations require `itemInfo: ItemInfo`, allow secret arguments only when untainted, and return nullable `CraftingQualityInfo`. See [Lua API architecture](../lua-api.md). This stage adds boundary tests only; no crafted-item producer exists yet.

## What it must do

### Required inputs and security
- [ ] Reject omitted and explicit nil `itemInfo` before lookup. Simulator diagnostics identify the method and selector; native exact wording is unproved.
- [ ] Authenticate the original VM secret before validation: untainted secret nil reaches required-input rejection; tainted secret nil is denied by the existing VM access guard. Preserve the wrapper, caller taint and restored untainted access after errors. Ordering is the simulator's interpretation of `AllowedWhenUntainted`, not a native observation.

### Declared result
- [ ] A present result supplies all thirteen non-nil declared fields: numeric `quality`; texture-atlas strings `icon`, `iconSmall`, `iconInventory`, `iconMixed`, `iconAppear`, `iconDissolve`, `barFill`, `barBackground`, `barBackgroundCap`, `barHighlight`, `iconChat`, `iconQuestObjective`.

## How it works

- [C API and Lua surface inventory](../wiki/systems/lua-api.md)
- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/c_api/c_trade_skill_quality.rs`: existing recipe-keyed quality queries; no crafted-item query model.
- `src/lua_api/env_init/runtime_surface_bootstrap.lua`: missing namespace functions may become nil-returning compatibility callables; callability is not modeled behavior.
- `tests/crafted_quality_api.rs`: public required-input and authentic VM-secret-nil boundary tests.
- `Cargo.toml`: pins rilua; `rilua::table_security::unwrap_secret` supplies the existing secret-access guard.

## Tests asserting this spec

`tests/crafted_quality_api.rs` authors two cases: omitted/nil contextual rejection and secret-nil ordering/taint recovery. Neither has executed yet. Both use statement-chunk execution, without query overrides or synthetic secrets.

## Known gaps (current cycle)

- [ ] Capture genuine public-call RED on Retail/PTR before producer edits; distinguish assertion failures from setup/compiler/admission failures.
- [ ] Establish accepted ItemInfo selector representations and valid secret-selector behavior from declarations, existing parsers and real callers.
- [ ] Establish positive-data RED for complete results before implementing projection. A separate explicit per-item supplied record is a proposed simulator model, not a declared native catalog.
- [ ] Verify detached results, replacement/removal, retained snapshots, isolation and the whole original ItemButton overlay consumer. These are planned, not passing tests.

## Out of scope

Native item-catalog/classification and exact-error parity remain unproved. Recipe-quality inputs do not establish crafted-item quality; no recipe, equipment or catalog inference is authorized by this contract. Rendered/native acceptance is separate from public Lua behavior.

## Declaration evidence

Profile-scoped cache sources for both Retail and PTR: `Blizzard_APIDocumentationGenerated/TradeSkillUIDocumentation.lua` and `TradeSkillUITypesDocumentation.lua`. Required signature and thirteen-field result are local declaration observations, not authenticated native captures. Numeric IDs, numeric strings, item strings and hyperlinks remain unestablished in this stage.
