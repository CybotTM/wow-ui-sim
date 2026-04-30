# Per-profile lua-errors baselines

Snapshots of `wow-sim --no-addons --no-saved-vars lua-errors` output for each
client profile. Captured 2026-04-30 at the heads documented in
`scripts/setup-blizzard-ui.sh`.

## Boot error counts

| Profile      | Distinct messages | Source ref |
|--------------|------------------:|------------|
| retail       |                 0 | `Gethe/wow-ui-source@b062d332` (12.0.5) |
| wrath        |                66 | `andrew6180/WoTLK-3.3.5-UI-Source@27334191` |
| mists        |                54 | `Gethe/wow-ui-source@33d87412` (classic) |
| era          |                98 | `Gethe/wow-ui-source@e0099491` (1.15.8 build 67156) |
| anniversary  |                93 | `Gethe/wow-ui-source@b29b0d0a` (2.5.5 build 67157) |

**Phase 7.5 reduction:** Era 124 → 98 (−26), Anniversary 118 → 93 (−25) after
adding `src/era/compat_bootstrap.lua` (~30 vanilla-only stub globals shared
between both profiles).

Snapshots are stored as `<profile>-lua-errors.json` next to this README.
Re-capture by running, for each profile:

```
cargo build --no-default-features --features "sound,gui,casc,client-<profile>" --bin wow-sim
WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 timeout 90 ./target/debug/wow-sim lua-errors > docs/baselines/<profile>-lua-errors.json
```

## Cluster analysis: where vanilla diverges

After collapsing each message to its leading line and stripping addon path /
line-number prefixes (set-based comparison on canonical keys):

| Cluster | Keys |
|---------|-----:|
| Era ∩ Anniversary (shared vanilla) | 84 |
| Era only | 34 |
| Anniversary only | 32 |
| Vanilla \ (Wrath ∪ Mists) — net new | 123 |
| Vanilla ∩ (Wrath ∪ Mists) | 27 |

The Era/Anniversary divergence is mostly format noise: anniversary's reporter
prefixes many messages with `Lua Error:`, so the same callsite shows up under
two distinct keys across the two profiles. The underlying missing-API set is
near-identical.

## Missing globals unique to vanilla profiles

29 globals are referenced by Era/Anniversary but absent from the active
runtime surface (and not stubbed by the wrath/mists compat bootstrap). These
are Phase 7.5 candidates for an `src/era/compat_bootstrap.lua`:

- `AddLuaErrorHandler`
- `AreHighResTexturesAvailable`
- `CreateForbiddenFrame`
- `FillLocalizedClassList`
- `GetActionBarPage`, `GetActionBarToggles`
- `GetComboPoints`
- `GetDisplayedAllyFrames`
- `GetNumQuestWatches`, `GetQuestTimers`
- `GetPVPYesterdayStats`
- `GetRaidProfileOption`
- `GetTabardCreationCost`
- `GuildControlGetRank`
- `HasKey`, `HasLoadedCUFProfiles`, `HasPetUI`
- `IsCommunitiesUIDisabledByTrialAccount`
- `IsInGlobalEnvironment`
- `IsKeyRingEnabled`
- `MoneyFrame_OnLoad`, `SmallMoneyFrame_OnLoad`
- `MoneyInputFrame_SetCompact`, `MoneyInputFrame_SetOnValueChangedFunc`,
  `MoneyInputFrame_SetPreviousFocus`
- `SecureMixin`
- `SetSelectedSkill`
- `SpellGetVisibilityInfo`
- `UIParent_OnLoad`

## How to use these baselines

`scripts/diff-lua-errors.sh BASELINE NEW` compares two snapshots and
reports `regressed` (in NEW but not BASELINE) and `fixed` (in BASELINE
but not NEW) message sets.

```
# Quick local diff after a change
WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 \
    ./target/debug/wow-sim lua-errors > /tmp/now.json
./scripts/diff-lua-errors.sh docs/baselines/wrath-lua-errors.json /tmp/now.json

# Quiet mode (just counts) — useful for scripting
./scripts/diff-lua-errors.sh BASELINE NEW --quiet
# → regressed=N fixed=M baseline=B current=C

# Fail the shell on any regression — for stricter CI gating later
./scripts/diff-lua-errors.sh BASELINE NEW --exit-on-regression
```

CI integration:

- `.github/workflows/test.yml` `client-profile-smoke` job runs the diff
  after capturing `lua-errors.json` and prints regressed/fixed messages
  in the run log. The diff is informational today (doesn't fail the
  job); flip the script to `--exit-on-regression` once the baselines
  stabilize.
- `.github/workflows/addon-harness.yml` runs the diff transitively via
  `scripts/test-classic-addons.sh`, which uses
  `scripts/diff-lua-errors.sh ... --quiet` to compute the
  "addon-induced errors" count surfaced per matrix entry.

The per-profile JSON artifacts uploaded by both workflows are the
machine-readable form; the committed snapshots in `docs/baselines/` are
the master-branch reference point those PRs are diffed against.
