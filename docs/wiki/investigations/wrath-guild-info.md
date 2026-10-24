# Wrath guild-info consumer and producer staging

Verified: 2026-10-24 (static inspection only). [Contract](../../specs/wrath-guild-info.md). No builds, tests or slot use; no executed RED/GREEN claim.

## Selected original consumer

`tests/wrath_guild_info.rs` is registered only for client-wrath; the integration harness discovers top-level test modules (`crates/test-harness/build.rs`). It calls `common::load_profile_framexml`, which uses `client_profile::blizzard_ui_framexml_toc` and the production `load_addon` path, not a hand-parsed XML excerpt. Wrath resolves the flat native `FrameXML/FrameXML.toc`, not a Mainline addon flavor. There is no competing flavor in this explicit core loader path.

That TOC has Interface30300 and an unannotated `FriendsFrame.xml` entry at line101. Neither the entry nor the TOC has AllowLoad/ExcludeLoad game-type, environment, bootstrap or LoD restrictions; no `[Game]`/`[Family]` substitution is present on the selected entry. `ParsedFileEntries::push_file_entry` retains it. `load_addon_files` has no Wrath skip for it (`should_skip_addon_file` only handles Mists wardrobe). FriendsFrame.xml loads FriendsFrame.lua and declares the native GuildInfoFrame with OnShow at4124–4145. Tests explicitly supply membership before `ToggleFriendsFrame(3)`, because that native function refuses the guild tab when `IsInGuild()` is false. Hide/Show invokes the actual attached handler; no replacement handler or producer monkey-patch is installed.

Wrath cache SHA256:

| File | SHA256 |
|---|---|
| FrameXML.toc | `5fe5a00e98806be245dd9564c766ba6eaf9917573c429f577f4234bd202ba18d` |
| FriendsFrame.xml | `97a2b0f11858e552636abe30874b62480d1a207ee15cb79f1e0138b4cec17b90` |
| FriendsFrame.lua | `b5c136066c8e0df4ba46b48c9b37249509e6c315f7e02d10f4428c42a1deab69` |

This is static runtime-selection evidence, not proof the staged tests loaded/executed successfully.

## Static contradiction table

Each row is one independently run selector in module `wrath_guild_info`. Missing-member failures alone are not the discrimination proof. `cachedText` is cleared before each producer observation; edit text is poisoned before Show. Rights are explicitly supplied at player rank1 while admin-selected rank0 is unprivileged; officer input contradicts permission.

| Selector | Required observations | Constant text / no-op cache reader | Constant allowed permission | Constant denied / no-op permission | Old officer shortcut |
|---|---|---|---|---|---|
| `native_info_onshow_projects_empty_and_supplied_cache_in_both_permission_branches` | empty+allowed label/white/enabled/mouse; A+denied grey/disabled; B+allowed white/enabled; empty+denied empty/grey | Any single string (including empty), nil/false or no values cannot display both distinct A/B strings. No-op leaves poison or empty. | Denied A and empty observations reject it. | First empty editable label/white/enabled rejects it. | Officer is false when rights allow, and true when denied: first allowed observation rejects old shortcut. |
| `native_info_onshow_uses_player_presence_guild_id_and_player_rank_rights` | allowed A; absent player denied; guildID0 denied despite rights; alternate player rank denied; allowed B | Two distinct supplied texts reject every constant, defensive empty/nil/no-op included. | Absent-player, no-guild and wrong-rank observations reject it. | Initial/final allowed observations reject it. | Allowed observation has officer false; rank is not selected admin rank. |
| `getter_results_preserve_native_arity_types_ignored_arguments_and_cache_independence` (API contract, not consumer RED) | fresh empty/nil; supplied text/number1; no-guild retained different text/nil; ignored args; no emitted events | Empty plus two distinct texts reject every constant/no-op and membership-gated cache getter. | Initial and final nil reject it, including boolean true. | Supplied number1 rejects nil/false/no-return. | Supplied number1 with officer false rejects it; boolean result also fails exact type. |

The isolated UI override observation is preservation only: truthy `GuildInfoFrame.cachedText` intentionally bypasses the producer. The two consumer selectors remain discriminating without that observation. An arbitrary no-op at the producer boundary (no Lua returns) gives nil to the consumer, which cannot produce A/B. A stale reader returning the first received string fails the later distinct string in each selector.

## Model and boundaries

Existing `world.guild_info_text` is the only text cache. Existing `world.guild_ranks[].flags[12]` is the only edit-info right; the [rank-rights contract](../../specs/wrath-guild-rank-flags.md) maps native `0x10000` to positional right13. New `world.guild_id` and zero-based `guild_player_rank` supply numeric identity/rank absent from the previous display-name model. `player.present` is an explicit guild predicate input, true in PlayerState::seeded; this change does not claim to unify other legacy domains' presence semantics. These are host inputs, not inferred server rights. The default WorldState has zero ID/rank and empty cache/rights. Wrath's seeded world additionally drops the synthetic membership/description/ranks; non-Wrath seeded data and registration retain their old path.

Existing A_Admin cache/ranks inputs remain usable; numeric ID/rank and presence can be supplied through Rust state like the staged tests. Display-only SetGuildInfo/JoinGuild and officer setters do not invent numeric identity or rights. No second store, packet receiver, auto-request, cache validity flag, setter behavior, or UNKNOWN transition policy is added.

## Batch3 path and blockers

The historical slot at `97e759f7d` passed the three guild-info selectors, but F1 failed at FriendsFrame.lua:309: it supplied name/cache, not the display rank formatted before the guild subframe opens. The static repair adds `Some("Member")` before native loading; the original F1 Lua fixture and assertions stay unchanged. The [17-row classification](../../baselines/wrath-guild-info-green-blockers.md) supplies membership/ranks/roster in affected setups instead of restoring synthetic Wrath defaults. All 17 are class(a); none establishes a native-default producer contradiction. These repairs have no executed GREEN proof.

The existing rank-settings dropdown row remains blocked on missing bank permission and withdrawal-limit getters after valid leader startup selects Officer. Original FriendsFrame.lua:1647/:1678 requires both. Existing bank state has money/current tab/items, but no rank/tab permissions or rank withdrawal limit. Parent holds implementation until laptop12340 facts arrive; no stub/default is invented. See the classification page for exact source boundaries and re-proof requirements.

## Sources

- Accepted laptop answer: `/home/osso/.local/state/wow-ui-sim/handoff/laptop-labels-answers-20261018/laptop-answers-req-12.md` (read fully; static reconstruction, not native runtime certification).
- Batch3 diagnosis: `/home/osso/.local/state/wow-ui-sim/handoff/wrath-excl-batch3/FAIL-DIAG.md` and unchanged `tests/fixtures/wrath_f03/guild_info.lua`.
- `src/client_profile.rs`, `src/toc/mod.rs`, `src/loader/addon.rs`, `tests/common/mod.rs` — selection and execution chain.
- `src/lua_api/globals/real/wrath_guild_info.rs`, `guild_probes.rs`, `guild_info.rs`, `state_types/character_world{,_defaults}.rs` — producer/model/defaults.
- [Consumer selection and RED proof](consumer-selection-and-red-proof.md); [[addon-loading]]; [[lua-api]].

## Deferred proof

Tests scaffold: `f4b84832e`; producers: `a88a41395`; adapted integrated source: `97e759f7d`; static explicit-state repairs: `c55f58ae5`. The saved slot REPORT records three GREEN producer cases, failing F1 and17 preservation regressions, not accepted GREEN for the repair. The new original nonleader dropdown completion selector is staged before any bank producer. It is a missing-call boundary test, not yet a permission-data discriminator; native facts and concrete varying inputs/widget assertions must precede either getter.

Current instructions and exact per-profile lists are in [the repair matrix](../../baselines/wrath-guild-info-green-blockers.md) and `/home/osso/.local/state/wow-ui-sim/handoff/wrath-guild-info/slot0-run/static-green-repairs/`. The old SLOT-RECIPE is historical: its36/42 Retail map and earlier source binding are superseded by REPORT's requested master34/38 reference and the new bound revision. This continuation uses no builds, slots, runtime tests or agents; bank getters remain an explicit held gap.
