# Wrath guild-info consumers and producers

Verified: 2026-10-24. [Contract](../../specs/wrath-guild-info.md). [Accepted runtime proof](#accepted-runtime-proof) supersedes the earlier static staging; fresh current-master integration remains separate.

## Selected original consumer

`tests/wrath_guild_info.rs` is registered only for client-wrath; the integration harness discovers top-level test modules (`crates/test-harness/build.rs`). It calls `common::load_profile_framexml`, which uses `client_profile::blizzard_ui_framexml_toc` and the production `load_addon` path, not a hand-parsed XML excerpt. Wrath resolves the flat native `FrameXML/FrameXML.toc`, not a Mainline addon flavor. There is no competing flavor in this explicit core loader path.

That TOC has Interface30300 and an unannotated `FriendsFrame.xml` entry at line101. Neither the entry nor the TOC has AllowLoad/ExcludeLoad game-type, environment, bootstrap or LoD restrictions; no `[Game]`/`[Family]` substitution is present on the selected entry. `ParsedFileEntries::push_file_entry` retains it. `load_addon_files` has no Wrath skip for it (`should_skip_addon_file` only handles Mists wardrobe). FriendsFrame.xml loads FriendsFrame.lua and declares the native GuildInfoFrame with OnShow at4124–4145. Tests explicitly supply membership before `ToggleFriendsFrame(3)`, because that native function refuses the guild tab when `IsInGuild()` is false. Hide/Show invokes the actual attached handler; no replacement handler or producer monkey-patch is installed.

Wrath cache SHA256:

| File | SHA256 |
|---|---|
| FrameXML.toc | `5fe5a00e98806be245dd9564c766ba6eaf9917573c429f577f4234bd202ba18d` |
| FriendsFrame.xml | `97a2b0f11858e552636abe30874b62480d1a207ee15cb79f1e0138b4cec17b90` |
| FriendsFrame.lua | `b5c136066c8e0df4ba46b48c9b37249509e6c315f7e02d10f4428c42a1deab69` |

This selection evidence is complemented by the accepted exact runtime receipts below.

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

The historical slot at `97e759f7d` passed the three guild-info selectors, but F1 failed at FriendsFrame.lua:309: it supplied name/cache, not the display rank formatted before the guild subframe opens. The static repair adds `Some("Member")` before native loading; the original F1 Lua fixture and assertions stay unchanged. The [17-row classification](../../baselines/wrath-guild-info-green-blockers.md) supplies membership/ranks/roster in affected setups instead of restoring synthetic Wrath defaults. All 17 are class(a); none establishes a native-default producer contradiction. These repairs pass in the accepted 56-row Wrath GREEN matrix; see [accepted runtime proof](#accepted-runtime-proof).

The existing rank-settings dropdown row was blocked on missing bank permission and withdrawal-limit getters after valid leader startup selected Officer. Original FriendsFrame.lua:1647/:1677 requires both. The accepted laptop answer now permits the staged [bank permission model](#bank-permission-model); both dropdown rows now have accepted compiled RED/GREEN proof.

## Bank permission model

Verified 2026-10-24: saved compiled RED/GREEN accepted by verifier8738; current-master refresh remains separate. [Bank contract](../../specs/wrath-guild-bank-rank-permissions.md). Laptop answer `guildbank-perms-12340.md` (SHA256 `f58e304a7d9c20813f23d871f9666547d0af3924d66f341eeed044b0b1588951`) supplies authoritative reconstructed native semantics; no decompiled source was read here.

The existing native TOC/loader selection above also selects the dropdown consumer. `GuildControlPopupFrameDropDownButton_OnClick` selects the control rank, calls `GuildControlPopupframe_Update`, then sets the closed label. `GuildBankTabPermissionsTab_OnClick` selects the tab and calls that same original update. Tests do not replace either consumer or getter. The existing leader setup avoids the special rank1 UI branch by selecting Officer/Member for observations; the second selector changes the player's rank to Officer before clicking Member as well.

`WorldState.guild_bank_rank_permissions` is an explicit host input map keyed by zero-based control rank. Each `GuildBankRankPermissions` row contains `money_per_day`, six `tab_rights` and six `tab_stacks_per_day`; no `Default` fills unknown server rows. The Wrath seeded world models the initial world-entry image with only rank0: money0, masks/stacks -1. It does not create membership or rank-name rows. Host receipt replaces map entries directly; no packet/event/reset lifecycle is fabricated. Keeping this input independent of display rank names also lets rank0 exist before a roster arrives. Bank server updates after rank mutations remain outside scope.

`globals/real/wrath_guild_bank.rs` registers the two legacy modeled getters inside `guild_probes.rs`'s existing runtime `ClientProfile::Wrath` branch. It uses the VM numeric conversion helper, then integer/subtract/uint8 normalization. Out-of-range tabs avoid reading rank inputs. Valid tabs and money require a supplied selected row; unselected/unsupplied reads raise an explicit simulator error rather than silently inventing permissions. Neither membership nor player rank nor open-bank state gates these getters. This is a legacy global model, not a `C_*` contract, so no `c_api` module or shim is added.

### Static contradiction table

Each of the two `test_guild_panel` selectors independently executes all observations below through the original consumers. Both selectors fail in producer-minus RED at the missing tab getter in original FriendsFrame.lua:1647 and pass in GREEN. The varying observations below reject constant/no-op outputs. The downstream withdrawal boundary is not an independently reached RED.

| Selected rank/tab | Required checkboxes view/deposit/edit; stack; gold | Contradiction |
|---|---|---|
| Officer/1 | true/false/false;11;12 | Reject all-enabled, swapped edit/stack and zero/no-op output. |
| Officer/2 | true/false/true;12;12 | Same rank, different tab rejects tab-constant masks/stacks. |
| Member/2 | false/false/false;22;98 | Reject any rank-constant money or permissions, always-enabled and player-rank lookup (player remains leader or Officer). |
| Member/1 | true/true/true;21;98 | Reject all-disabled or tab-only projections. |
| Officer/1 again | true/false/false;11;12 | Reject stale rank/last-value readers; dropdown selected ID and displayed name must also change back. |

Every constant tab tuple contradicts at least two rows; every constant money output contradicts12 versus98. No-return/no-op cannot yield distinct numeric editbox strings. Direct API cases separately reject boolean permission markers, validate nil slots and arity, isolate rights bits2/4 even without view, exercise normalization, usage errors, negative money and host replacement of rank0. Missing-input errors are explicitly simulator policy, not native parity.

## Sources

- Accepted laptop answer: `~/.local/state/wow-ui-sim/handoff/laptop-labels-answers-20261018/laptop-answers-req-12.md` (read fully; static reconstruction, not native runtime certification).
- Batch3 diagnosis: `~/.local/state/wow-ui-sim/handoff/wrath-excl-batch3/FAIL-DIAG.md` and unchanged `tests/fixtures/wrath_f03/guild_info.lua`.
- `src/client_profile.rs`, `src/toc/mod.rs`, `src/loader/addon.rs`, `tests/common/mod.rs` — selection and execution chain.
- `src/lua_api/globals/real/wrath_guild_info.rs`, `guild_probes.rs`, `guild_info.rs`, `state_types/character_world{,_defaults}.rs` — producer/model/defaults.
- [Consumer selection and RED proof](consumer-selection-and-red-proof.md); [[addon-loading]]; [[lua-api]].

## Accepted runtime proof

Verified 2026-10-24 by verifier8738: GREEN `469f6ffed`, producer-minus RED `9f180b154`; identical tests trees `8d0252ed8`. Evidence: `~/.local/state/wow-ui-sim/handoff/wrath-guild-info/bank-perms/slot3-run-20261024/`; independent report: `~/.local/state/wow-ui-sim/verification/wrath-guild-bank/report.md`.

Wrath RED51 PASS/5 bank FAIL; GREEN56 PASS. Retail53 PASS on each revision. All218 exact runs have prior exact listings and CVar-safe receipts. Both original dropdown selectors reach the selected original consumer and pass varying rank/tab widget assertions in GREEN. The three direct API cases cover projection, invalid arguments and initial/replacement inputs; the invalid-argument case alone does not prove successful value projection. Wrath stock on master `12fb7c3b3` and GREEN is empty. Retail stock is equal at32 unique/36 occurrences on that exact pair, not clean startup. Both all-target checks and formatting pass without warnings.

Current-master integration is a separate open gate: merge exact master `08b423c5c`, run merged Wrath/Retail all-target checks, the five bank selectors, fresh merged Wrath stock and fresh Retail stock on both master and merged HEAD. Master's combat-log filter change invalidates the old Retail count as an expectation. The full218-run RED/GREEN matrix need not repeat. See [baseline classification](../../baselines/wrath-guild-info-green-blockers.md#proof-limits). No native runtime certification or seven-profile/full-suite acceptance is claimed.
