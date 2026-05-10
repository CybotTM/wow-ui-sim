# Mists / Pandaria Classic Startup Plan

## Current State

- [x] Rebased `classic-profile-rollout` onto latest `origin/master`.
- [x] Mists Blizzard UI source is Gethe `wow-ui-source` at `33d87412` (`5.5.3.67158`).
- [x] Local WoW install active classic product is Pandaria Classic (`wow_classic` build `5.5.3.67158`).
- [x] Classic addon harness can use installed local addon sources with `local:<absolute-path>`.
- [x] Mists addon manifest points at installed Pandaria addons under `/syncthing/World of Warcraft/_classic_/Interface/AddOns`.
- [x] Feature-gated Pandaria addon source tests exist under `client-mists`.
- [x] Fixed Mists expansion helper state:
  - `GetExpansionLevel()` returns `LE_EXPANSION_MISTS_OF_PANDARIA`.
  - `ClassicExpansionAtLeast()` / `ClassicExpansionAtMost()` match MoP.
- [x] Fixed TOC `[Game]` token under `client-mists` so it resolves to `Mists/`, not `Standard/`.
- [x] Mists startup is still dirty: `lua-errors` reports `43` distinct errors with saved vars and third-party addons disabled.

## Reproduction Commands

Build Mists:

```bash
cargo build --bin wow-sim --no-default-features --features "sound,gui,casc,client-mists"
```

Headless startup error capture:

```bash
WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 timeout 90 \
  target/debug/wow-sim lua-errors \
  > /tmp/mists-lua-errors.json \
  2>/tmp/mists-lua-errors.stderr

jq 'length' /tmp/mists-lua-errors.json
```

GUI startup smoke:

```bash
WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 timeout 30 target/debug/wow-sim
```

Focused tests:

```bash
cargo test --no-default-features --features "sound,gui,casc,client-mists" --test mists_compat_bootstrap
cargo test --no-default-features --features "sound,gui,casc,client-mists" --test pandaria_installed_addons
```

## Verification Gate

- [x] `jq 'length' /tmp/mists-lua-errors.json` returns `0` for `WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1`.
- [x] GUI startup reaches first frame without printing Lua errors.
- [x] `mists_compat_bootstrap` passes.
- [x] `pandaria_installed_addons` passes.
- [x] `readability-audit` passes for changed Rust files.
- [x] `git diff --check` passes.

## Startup Error Triage

### 1. SkillFrame Skill Rank State

- [x] Reproduce `SkillFrame.lua:185: attempt to perform arithmetic on local 'skillRank' (a nil value)`.
- [x] Identify backing API/state feeding `SkillFrame_Update`.
- [x] Add a Mists-gated test that proves the selected skill API returns the shape Blizzard expects.
- [x] Fix the backing API/state model, not the `SkillFrame` output layer.
- [x] Verify the `SkillFrame` errors disappear from `lua-errors`.

Observed errors:

```text
Blizzard_UIPanels_Game/Classic/SkillFrame.lua:185:
attempt to perform arithmetic on local 'skillRank' (a nil value)
```

### 2. Honor Frame API Surface

- [x] Reproduce `HonorSystemEnabled` nil during `HonorFrame_Shared.lua` load.
- [x] Determine expected MoP Classic behavior for `HonorSystemEnabled()`.
- [x] Add a Mists-gated API contract test.
- [x] Implement or correct the backing honor/PvP API state.
- [x] Verify `HonorSystemEnabled` and `GetPVPThisWeekStats` errors disappear.

Observed errors:

```text
HonorFrame_Shared.lua:29: attempt to call global 'HonorSystemEnabled' (a nil value)
attempt to call global 'GetPVPThisWeekStats' (a nil value)
```

### 3. Money Frame Template Initialization

- [ ] Reproduce `TradePlayerInputMoneyFrame` missing `copper`.
- [ ] Inspect generated XML for the money input frame and its child-key wiring.
- [ ] Determine whether the issue is XML template inheritance, parentKey sync, or MoneyFrame API state.
- [ ] Add a focused XML/widget test for the missing `copper` child.
- [ ] Fix the upstream template/widget construction path.
- [ ] Verify money-frame errors disappear.

Observed errors:

```text
TradePlayerInputMoneyFrame:
attempt to index field 'copper' (a nil value)
```

### 4. Product Choice Data Model

- [ ] Reproduce `ProductChoice.lua:61: attempt to get length of a nil value`.
- [ ] Identify which product-choice table is nil.
- [ ] Determine whether MoP Classic expects empty data or an unavailable feature path.
- [ ] Add a focused Mists-gated test.
- [ ] Fix the backing data/API state or load gating.
- [ ] Verify ProductChoice errors disappear.

Observed errors:

```text
Blizzard_UIPanels_Game/Classic/ProductChoice.lua:61:
attempt to get length of a nil value
```

### 5. World Map Opacity State

- [ ] Reproduce `WorldMapFrame_SetOpacity` nil `opacity`.
- [ ] Find the CVar or saved setting that should seed map opacity.
- [ ] Add a focused test for the default value path.
- [ ] Fix the backing setting/CVar state.
- [ ] Verify world-map opacity errors disappear.

Observed errors:

```text
Blizzard_WorldMap/Cata/Blizzard_WorldMap.lua:790:
attempt to perform arithmetic on local 'opacity' (a nil value)
```

### 6. Nameplate Vertical Scale State

- [ ] Reproduce `namePlateVerticalScale` nil in `Blizzard_NamePlates.lua:293`.
- [ ] Find the CVar or nameplate option that should provide the value.
- [ ] Add a focused Mists-gated test.
- [ ] Fix the backing setting/CVar state.
- [ ] Verify nameplate scale errors disappear.

Observed errors:

```text
Blizzard_NamePlates/TBC/Blizzard_NamePlates.lua:293:
attempt to perform arithmetic on local 'namePlateVerticalScale' (a nil value)
```

### 7. Guild Roster Selection State

- [ ] Reproduce `SetGuildRosterSelection` nil.
- [ ] Do not add a no-op stub: prior note says that can hang guild roster retry logic.
- [ ] Implement real selected-guild-roster-index state with matching getter/setter behavior.
- [ ] Add a focused Mists/Wrath-compatible test.
- [ ] Verify guild roster errors disappear and startup does not hang.

Observed errors:

```text
FriendsFrame.lua: attempt to call global 'SetGuildRosterSelection' (a nil value)
```

### 8. Currency List Compatibility

- [ ] Reproduce `GetCurrencyListSize` nil.
- [ ] Confirm whether Mists Blizzard Lua calls legacy `GetCurrencyListSize` while simulator only exposes `C_CurrencyInfo.GetCurrencyListSize`.
- [ ] Add a focused compatibility test.
- [ ] Implement the legacy global as a wrapper over the C API if semantics match.
- [ ] Verify currency errors disappear.

Observed errors:

```text
attempt to call global 'GetCurrencyListSize' (a nil value)
```

### 9. Dialog and Popup Text Helpers

- [ ] Reproduce `SetBasicMessageDialogText` nil.
- [ ] Locate expected helper definition in Blizzard UI or legacy API surface.
- [ ] Add a focused test for dialog text mutation.
- [ ] Implement the helper against the real dialog frame state.
- [ ] Verify dialog helper errors disappear.

Observed errors:

```text
attempt to call global 'SetBasicMessageDialogText' (a nil value)
```

### 10. Class Color and Miscellaneous Nil Data

- [ ] Reproduce `classColor` nil in `Blizzard_Communities/ClubFinder.lua`.
- [ ] Identify which class token lacks a color.
- [ ] Add a focused class-color/default data test.
- [ ] Fix the backing class-color data.
- [ ] Verify class-color errors disappear.

Observed errors:

```text
Blizzard_Communities/ClubFinder.lua:564:
attempt to index local 'classColor' (a nil value)
```

## Addon Harness Follow-Up

- [ ] After base Mists startup is clean, run each installed Pandaria addon through `scripts/test-classic-addons.sh --profile mists`.
- [ ] For each addon, record addon-induced errors separately from base startup.
- [ ] Promote shared missing APIs to `src/mists/compat_bootstrap.lua` or Rust backing systems.
- [ ] Keep per-addon quirks under `tools/classic-addon-compat/<addon>/`.
- [ ] Do not update `docs/baselines/mists-lua-errors.json` to bless known startup errors; use it only after the base startup is clean or intentionally documented.

Installed Mists targets:

- [ ] `AllTheThings`
- [ ] `Auctionator`
- [ ] `BlizzMove`
- [ ] `DeModal`
- [ ] `DialogueUI`
- [ ] `Leatrix_Maps`
- [ ] `Leatrix_Plus`
- [ ] `Plater`
- [ ] `SimpleItemLevel`

## Notes

- `PLAN.md` remains the repo dispatch board and currently contains an unrelated `src/paths.rs` readability refactor item.
- `PLAN.mists.md` tracks the Pandaria Classic startup/addon effort only.
- Prefer upstream simulator state/model fixes over downstream Blizzard Lua shims.
- Shims are acceptable only when they represent a real legacy API compatibility surface or an explicitly temporary stopgap with a retirement path.
