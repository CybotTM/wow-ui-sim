# Wrath addon-load fixture ordering

Coverage fixtures omitted the profile's top-level FrameXML TOC. Production loads it before discovering Blizzard addons. Restoring that same prerequisite is a fixture correction, not a producer shim or a change to error budgets.

## Root cause

`addon_coverage_lod::load_startup_blizzard_ui` and the force-load case now call `client_profile::blizzard_ui_framexml_toc()` after resetting templates and before Blizzard addon discovery/loading. Profiles without a top-level TOC retain their existing path.

The cached Wrath core publishes MoneyFrame helpers, scroll/check-button templates, dropdown Lua, SparkleFrame, UIMenus, combat-log constants and chat textures. Omitting those producers caused downstream original-addon failures. TimeManager supplies a numeric hour to the original dropdown's FontString; the incomplete fixture instead reached the simulator's strict string-only dropdown bootstrap.

## Bounded evidence

Source-bound before receipts execute one test per case: Retail all-addon plus16 LoD shards pass; Wrath all-addon, prior-runtime, TimeManager and shards5,8–16 fail. The exact original TimeManager case fails again after the [harness source-binding fix](../systems/prefork-test-harness.md#shared-target-source-binding--2026-10-20), confirming that relocation repair alone does not fix fixture ordering.

FrameXML-corrected receipts at `a1130d799`, then unchanged-model auction publication at `dec4eda85`, execute all 13 original Wrath consumers. Each exact case executes one test (nested re-exec summaries are not additional cases):

| Consumer | Final result | Retained cause |
|---|---|---|
| TimeManager numeric-hour dropdown | PASS | — |
| Force-load all addons | RETAINED FAIL | Calendar 8 errors against baseline 1. |
| LoD shard 5 | PASS | — |
| LoD shard 8 | PASS | — |
| LoD shard 9 | PASS | — |
| LoD shard 10 | RETAINED FAIL | AuctionUI 1 error against baseline 0. |
| LoD shard 11 | RETAINED FAIL | BattlefieldMinimap 1 and TalentUI 4 against baseline 0. |
| LoD shard 12 | PASS | — |
| LoD shard 13 | RETAINED FAIL | Calendar 4 against baseline 1; TradeSkillUI 2 against baseline 0. |
| LoD shard 14 | PASS | — |
| LoD shard 15 | PASS | — |
| LoD shard 16 | PASS | — |
| Shard 14 after prior runtime shards | RETAINED FAIL | Stops at shard 10's remaining AuctionUI error. |

Retail before/after preservation passes 17/17 each: force-load plus all 16 LoD shards. Sealed ELF hashes match; both final integration builds embed 1,791 current-worktree module paths. Full-feature stock Wrath and Retail `--no-addons --no-saved-vars lua-errors` return `[]`. Final Wrath `cargo check` and workspace `cargo fmt --all -- --check` pass; five successful build/check phases emit zero compiler warnings. These are bounded receipts, not all-profile or full-suite acceptance.

Original CVars remain byte-identical (SHA256 `3704e092febe9a79363a3d83f31c6b2e89c8cea1165b212c6a0fafaf2a22e4a0`), with matching before/after stat/hash snapshots for each runtime. No errors are filtered or baseline counts increased.

Local evidence: `/home/osso/.local/state/wow-ui-sim/handoff/wrath-addon-load/takeover-3-proof-ledger.json`; earlier RED/before evidence remains in `takeover-proof-ledger.json` beside it. The pre-existing `Cargo.lock` diff is package ordering only and is preserved uncommitted.

## Residual registration bug

The original AuctionUI consumer (shard 10 and prior-runtime) fails at `Blizzard_AuctionUI.lua:482` (`SortAuctionClearSort`), `:281` (`GetBidderAuctionItems`) and `:187` (`CalculateAuctionDeposit`): each calls nil. These functions already exist in the simulator's legacy auction model, but both its registration and module compilation were Mists-only. Commits `ecc7db346` and `dec4eda85` publish and compile that unchanged model on Wrath; no auction semantics change. The original shard is the RED. Post-correction AuctionUI errors decrease from 5 to 1: the consumer now reaches `Blizzard_AuctionUI.lua:380`, where `GetAuctionItemClasses` is missing. The shard remains RED for that separate catalog gap; no full AuctionUI GREEN is claimed. An intermediate build caught the initially omitted module cfg (E0433); its zero-test launcher failures are explicitly excluded from behavioral proof.

## Retained legacy gaps

These are separate from fixture ordering and auction publication. They remain unchanged, with no budget relaxation:

| Original cached consumer boundary | Failure | Missing modeled contract |
|---|---|---|
| `Blizzard_AuctionUI.lua:380` | `GetAuctionItemClasses` nil | Legacy item-class display-name catalog for the original filter initialization. |
| `Blizzard_BattlefieldMinimap.lua:49` | `CreateMiniWorldMapArrowFrame` nil | Map-arrow frame creation and position presentation. |
| `Blizzard_TalentUI.lua:266,971` | `GetActiveTalentGroup` nil | Active/player/pet talent-group state and group-count projections. |
| `Blizzard_Calendar.lua:2340` | `ArenaTeamRoster` nil | Arena-team roster request/state/event lifecycle. |
| `Blizzard_Calendar.lua:3771` | `CalendarEventGetTypes` nil | Calendar event-type catalog matching the original positional consumer. |
| `Blizzard_Calendar.lua:4521` | `CalendarDefaultGuildFilter` nil | Guild invite level-filter defaults and persistence. |
| `Blizzard_Calendar.lua:1193` | captured local `random` nil | Legacy global math alias publication before Calendar chunk execution; not a calendar-state workaround. |
| `Blizzard_TradeSkillUI.lua:463,508` | `SetTradeSkillSubClassFilter`, `SetTradeSkillInvSlotFilter` nil | Selected trade-skill subclass/inventory-filter state and filtered recipe projection. |

Shard 11 retains BattlefieldMinimap/TalentUI; shard 13 retains Calendar/TradeSkillUI. Force-load also observes these below-budget errors; they are not hidden by the five retained-failure classification:

| Original cached consumer boundary | Failure | Required investigation or model |
|---|---|---|
| `Blizzard_BarberShopUI.lua:81` | `GetHairCustomization` nil | Player customization-category projection for hair labels. |
| `Blizzard_CombatText.lua:537` | `CombatTextSetActiveUnit` nil | Active-unit selection and combat-text event routing. |
| `FrameXML/MoneyFrame.lua:112` | `GetGuildBankMoney() - GetCursorMoney()` contains a nil call; exact operand unproven | Guild-bank and cursor-money projections. |
| `Blizzard_GuildBankUI.lua:657` | `GetGuildBankWithdrawMoney` nil | Permitted withdrawal-limit projection. |
| `Blizzard_GuildBankUI.lua:677` | `GetGuildTabardFileNames` nil | Original six-position file tuple; inspect existing publication before changing it. |
| `Blizzard_InspectUI/PVPFrameTemplates.xml` | XML include file missing in cache | Trace manifest/include source and cache resolution; no blind vendor edits. |
| `FrameXML/UIPanelTemplates.lua:365` | nil `self.cursorOffset` arithmetic | Trace original thumb input/state ordering; no guard justified. |

Full raw receipts and `retained-legacy-boundaries.json` remain beside the proof ledger. These gaps do not justify expanding this lane into legacy system implementation.

## Native limits and laptop questions

The lane target is cached FrameXML Interface30300. These receipts execute simulator consumers, not a native client. The following native ordering/value claims remain **INFERRED** pending laptop evidence:

- Does core FrameXML publish these prerequisites before Blizzard LoD addons?
- Does original TimeManager pass its numeric hour unchanged through dropdown Lua to FontString text `1`?

Residual failures must be traced after the corrected fixture executes; missing-core symptoms alone do not justify API shims.

## Sources

- [Production startup](../../../src/bin/wow_sim/addon_loading.rs) — optional profile FrameXML precedes Blizzard addon iteration.
- [Profile TOC lookup](../../../src/client_profile.rs) — profile-cache top-level TOC discovery.
- [LoD coverage](../../../tests/addon_coverage_lod.rs) — original runtime consumers and concrete TimeManager assertion.
- [Force-load coverage](../../../tests/addon_coverage.rs) — per-addon collected-error accounting.

## See Also

- [Addon loading](../../addon-loading-pipeline.md) — loader behavior.
- [Harness source binding](../systems/prefork-test-harness.md#shared-target-source-binding--2026-10-20) — separate stale-source and zero-selection failure.
