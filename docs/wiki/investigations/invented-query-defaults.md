# Invented query defaults

The first audit batch separates finite filter bounds from environment-local inventories. A callable query or a seeded demonstration row is not evidence of a native default. The Mists tab and RAF version producers now use empty-by-default supplied catalogs. Original-consumer producer-only RED/GREEN, four stock comparisons, formatting and all seven zero-warning all-target checks pass. The preserved old Mists storage flow has a separately reproduced pre-existing tutorial failure.

## Transmog: unresolved filter bound, unchanged

`GetNumTransmogSources` is not the number of appearance-source records. Original `Blizzard_Collections/Mainline/Blizzard_Wardrobe.lua:67–69` uses its result as a source-filter index bound and calls `IsValidTransmogSource` for each index. `bf08cd368` changed an appearance-count implementation to the fixed bound 7 for this consumer.

Pinned generated declarations do not fix the query's numeric value:

| Profile | `TransmogItemsDocumentation.lua` query | `TransmogSharedDocumentation.lua` enum |
|---|---|---|
| Retail, PTR | 573–580 | 85–102 |
| Forever | 574–581 | 85–102 |
| Mists, Era, Anniversary | 544–551 | 84–101 |
| Wrath | File absent | File absent |

The query returns a non-nil number. The enum declares `NumValues=11`, `MinValue=0`, and `MaxValue=10`; no declaration equates the query with either metadata value. Thus the exact bound remains unresolved. Neither changing it to an appearance count nor asserting native parity for 7 is justified. Both requested transmog functions remain unchanged.

## Mists guild-bank tab catalog

History: `56aa5abd3` introduced a fixed General tab, its icon, and fixed access/withdrawal fields. `bank_storage_verbs::register_all` is registered only under `client-mists`; other profiles' similarly named original consumers do not use these handlers.

The pinned generated `GuildBankDocumentation.lua` does not declare these legacy globals. With explicit approval, their tuple shape is taken from the selected original consumer rather than invented generated documentation. `Blizzard_GuildBankUI_Mists.toc:5–6` selects `TBC/Blizzard_GuildBankUI.lua` and `Mists/Blizzard_GuildBankUI.xml`. The Lua file destructures name/icon/view/deposit/withdrawals/remaining at line291 and reads field7 (filtered) at line591. Empty tabs are hidden at lines322–323; supplied tab names reach the button tooltip at line328. See the [catalog contract](../../specs/guild-bank-tab-catalog.md).

## RAF version catalog

History: `e043996eb` introduced a version1 row inside a helper named `empty_versions`. Pinned `RecruitAFriendDocumentation.lua:71–77` returns a non-nil `RafInfo`, not a mandatory version1 record. `RafInfo.versions` is a table of `RafVersionInfo`; its declaration does not require a nonempty table.

Pinned `RecruitAFriendDocumentation.lua` declares the query/outer and month records/version record at these lines:

| Profiles | GetRAFInfo | RafInfo and RafMonthCount | RafVersionInfo |
|---|---|---|---|
| Retail, PTR, Forever | 71–77 | 329–348 | 456–467 |
| Mists, Era, Anniversary | 66–72 | 311–330 | 438–449 |
| Wrath | File absent | No native shape claim | No native shape claim |

Retail, PTR and Forever select the game/mainline `Blizzard_RecruitAFriend.toc`. The original frame system-enable event calls `GetRAFInfo` and passes it to `UpdateRAFInfo`; that function's nonempty-version guard controls whether a version is published to the frame. Classic pinned caches have no RAF addon. See the [version contract](../../specs/raf-version-catalog.md).

## Verified scope — 2026-10-26

Native producer code is `1b37aed01`; Mists fixture `85871611f` preserves the prior General icon bytes with a Lua long-bracket string. Producer-only RED `1b5ea6622` has byte-identical tests and input scaffolding and differs only in the two query-producer files. RAF paired RED `60728a196` is unchanged by that Mists-only fixture edit.

| Profile | Original consumer proof | Raw stock keys/occurrences, base → GREEN | Compiler proof |
|---|---|---|---|
| Mists | Original `GuildBankFrame:UpdateTabs` empty → supplied → empty; fake row rejected | 6/12 → 6/12 | All-target, zero warnings |
| Retail | Original RAF enable event; empty rejected by old producer, supplied version published by new producer | 10/12 → 10/12 | All-target, zero warnings |
| PTR | Same selected original event with profile-pinned UI | 216/282 → 216/282 | All-target, zero warnings |
| Forever | Same selected original event with profile-pinned UI | 26/26 → 26/26 | All-target, zero warnings |
| Wrath, Era, Anniversary | No selected RAF addon and no changed bank-handler registration; no forced original-consumer claim | Not recaptured for these unconsumed queries | Each profile all-target, zero warnings |

Every completed stock comparison has zero added/removed keys and zero occurrence-count changes; startup is not clean. The original Mists storage-flow test fails at missing `TriggerTutorial`, `Blizzard_UIPanels_Game/Classic/ContainerFrame_Shared.lua:976`, before the guild-bank assertions, on both base and GREEN. Its General tab is explicitly supplied and no old assertion is weakened. Fixing that separate producer is outside this batch.

The approved fresh RAF assertion changes `versions_len` from 1 to 0. Its explicit one-version sibling retains the old scalar/shape checks and adds supplied month/count assertions. No other old assertion changes. Empty defaults are simulator policy, not a claim about native server defaults; nonempty reward/recruit catalogs and enablement remain out of scope.

## Sources

- [Guild-bank tab contract](../../specs/guild-bank-tab-catalog.md)
- [RAF version contract](../../specs/raf-version-catalog.md)
- [Client profile selection](../systems/client-profiles.md)
- `src/lua_api/globals/register.rs` — Mists-only guild-bank registration.
- Profile-pinned Blizzard generated documentation and original Lua/TOCs, cited above.

## See Also

- [[consumer-selection-and-red-proof]] — selected original consumers and independently discriminating RED tests.
