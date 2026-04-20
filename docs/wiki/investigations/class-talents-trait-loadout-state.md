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

Relevant tests:

- `tests/admin_spec_talent_api.rs`
- `tests/spell_api.rs`

## See Also

- [[talent-performance]] — earlier `PlayerSpells` and talent-frame restore work
- [[hero-spec-icon-bug]] — another class-talent investigation that depended on real node state
