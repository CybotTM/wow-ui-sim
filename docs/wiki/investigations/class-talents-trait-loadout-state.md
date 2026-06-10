# Class talents trait loadout state

`PlayerSpells` was still leaning on placeholder `C_Traits` behavior even after the first pass of the talent restore. The simulator had live node rank/selection state, but several query APIs still returned hardcoded config IDs, fake flags, or empty staged-change results. That made the class-talent UI behave as if every tree was a static placeholder instead of a real editable loadout.

## Symptoms

- `C_Traits.GetConfigIDBySystemID()` and `GetConfigIDByTreeID()` returned placeholder IDs instead of the active class-talent loadout
- `C_Traits.CanPurchaseRank()` could disagree with `GetNodeInfo(...).canPurchaseRank`
- `C_Traits.ConfigHasStagedChanges()`, `GetStagedChanges()`, and `GetStagedChangesCost()` hid purchase/refund/selection deltas from Blizzard callers
- `PlayerSpells` paths that depend on staged edits or config mapping still treated the simulator like a display-only shell

## Root Cause

The simulator already tracked working talent state per config, plus committed snapshots, in `TalentState`. But the `C_Traits` surface in `src/lua_api/globals/missing_surface/traits.rs` was only partially wired to that data:

- config/tree/system mapping still used hardcoded IDs
- trait-system flags were hardcoded by config number instead of reading tree-backed state
- staged-change queries were not using the `TalentState` diff model directly

So the backing state existed, but the Blizzard-facing trait API was still exposing placeholder behavior.

## Fix

The restore landed in two layers:

1. `TalentState` now exposes explicit helpers for:
   - `is_active_config`
   - staged purchases
   - staged refunds
   - staged selection swaps
   - staged currency deltas
2. `C_Traits` now reads those helpers instead of returning constants:
   - active config mapping follows the live class-talent loadout
   - trait-system flags come from the mapped tree state
   - purchase gating checks live node state
   - staged-change APIs expose the working-vs-committed diff seen by Blizzard UI code

## Regression Coverage

Focused tests now pin the restored behavior:

- config/tree/system ID mapping follows loadout switches
- `CanPurchaseRank` matches live node gating and flips off when a node is maxed
- staged purchases expose visible cost deltas
- staged refunds and selection swaps each surface through `GetStagedChanges`

Follow-up restore coverage:

- `C_ClassTalents.GetTraitTreeForSpec(specID)` now returns `nil` for unknown specs instead of a forced Paladin tree id
- `C_ClassTalents.GetNextStarterBuildPurchase()` now feeds the real `PlayerSpellsFrame.TalentsFrame:UpdateStarterBuildHighlights()` path
- `C_ClassTalents.HasUnspentHeroTalentPoints()` now reports the active hero subtree's remaining points and gates the export callback in `Blizzard_ClassTalentsFrame`

## Hero subtree visibility regression (2026-04-21)

After restoring hero node icon assets, the hero panel still rendered with only a single node and missing connector edges. The issue was not texture loading.

### Symptom

- `HeroTalentsContainer.ExpandedContainer.NodesContainer` rendered one visible node button instead of the full subtree
- edge atlases (for example `talents-arrow-line-gray`) were mostly absent because upstream node visibility gated them out

### Root Cause

`check_spec_conditions_met()` in `src/lua_api/globals/missing_surface/traits.rs` treated multiple `cond_type == 1` spec-set conditions as strict `AND`.

Hero nodes in tree `790` frequently carry both Paladin spec-set conditions in `group_cond_ids`:

- Protection set (`49234` / spec set `28`)
- Holy set (`49235` / spec set `27`)

With `AND` semantics, these nodes were impossible to satisfy for a single active spec, so `GetNodeInfo(...).isVisible` was false for most hero nodes. The frame builder then had no nodes/edges to render.

### Fix

- Changed `check_spec_conditions_met()` to use `OR` semantics across spec-set conditions:
  - no spec conditions => visible
  - at least one spec condition => visible when any condition matches active spec
- Added regression coverage in `tests/hero_talents.rs`:
  - `test_active_hero_subtree_exposes_multiple_visible_nodes_and_edges`
  - verifies active hero subtree exposes many visible nodes (not just one) and has edge-ready nodes

### Result

`HeroTalentsContainer` now emits the full hero subtree again (multiple node buttons and connector-edge atlases), so border/edge visuals come back with the same render path.

Tests:

- `tests/class_talents_config.rs`
- `tests/class_talents_flags.rs`
- `tests/test_showuipanel_lod_player_spells.rs`

Relevant tests:

- `tests/admin_spec_talent_api.rs`
- `tests/spell_api.rs`

## Config-scoped node visibility regression (2026-06-10)

Full addon startup with live SavedVariables could open the Paladin talents panel with the class tree visible and the Protection tree absent. The missing nodes were not a renderer or anchor problem: Blizzard's `InstantiateTalentButton` path received every Protection/spec node with `nodeInfo.isVisible == false`.

### Symptom

- `C_ClassTalents.GetActiveConfigID()` still returned Protection config `201`
- `C_Traits.GetNodeInfo(201, protectionNodeID).isVisible` returned `false` after a stale Holy view loadout
- the resulting screenshot showed the Protection header and background, but no Protection node buttons

### Root Cause

`C_Traits.GetNodeInfo(configID, nodeID)` parsed `configID` and then discarded it. Node visibility fell back to the global viewer/player spec through `view_spec_id.or_else(player_spec_id)`. If an addon path or SavedVariables left `view_spec_id` on another spec, querying a real Protection config still evaluated Protection nodes against the stale viewer spec.

`C_Traits.GetConditionInfo(configID, condID)` had the same discarded-config shape for spec-set conditions, so it could disagree with config-scoped node queries too.

### Fix

Node and condition visibility now derive an effective spec from the supplied config ID when it is a real saved class-talent config (`101/102`, `201/202`, `301/302`). The view config (`Constants.TraitConsts.VIEW_TRAIT_CONFIG_ID`) still falls back to the current view/player spec.

Regression coverage:

- `get_node_info_uses_config_spec_visibility_over_stale_view_spec` initializes a Holy view loadout, queries a Protection config node, and requires it to stay visible.

## Sources

- [traits.rs](../../../src/lua_api/globals/missing_surface/traits.rs) — spec-condition and config-scoped node visibility logic
- [c_traits.rs](../../../src/lua_api/globals/missing_surface/traits/c_traits.rs) — `C_Traits.GetNodeInfo` / `GetConditionInfo` config handling
- [hero_talents.rs](../../../tests/hero_talents.rs) — regression coverage for hero subtree node visibility
- [class_talents_config.rs](../../../tests/class_talents_config.rs) — regression coverage for config-scoped node visibility

## See Also

- [[talent-performance]] — earlier `PlayerSpells` and talent-frame restore work
- [[hero-spec-icon-bug]] — another class-talent investigation that depended on real node state
