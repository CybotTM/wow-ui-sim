# Wrath guild-info GREEN blocker classification

Verified: 2026-10-24, static work at `97e759f7d` plus the fixture changes below. No build, slot or runtime test was run. Saved pre-change runtime evidence is `/home/osso/.local/state/wow-ui-sim/handoff/wrath-guild-info/slot0-run/preservation-comparison.json` and its per-selector streams. The saved results are not post-change GREEN proof.

## Decision and evidence

All 17 pass→fail rows are **(a): required guild inputs were implicit in the old synthetic seed**. No row establishes (b). The lane's no-guild initialization remains the [explicit contract](../specs/wrath-guild-info.md), not a claim that every real Wrath character has no guild. Supplied membership, rank rights and received roster data are distinct from initialized caches.

Accepted laptop facts: `laptop-answers-req-12.md` and `guild-control-rank-flags-12340.md` under `/home/osso/.local/state/wow-ui-sim/handoff/laptop-labels-answers-20261018/`. Req12 says `CanEditGuildInfo` requires active player, nonzero guild ID and player-rank right13; officer status alone is not its native predicate. The rank-flags answer requires supplied selected stored rights and returns 17 positional numbers/nils. Neither answer requires a synthetic guild/roster in an initialized simulator. No oracle assertion was found to contradict these facts. The officer fixture supplies the right as an input at the officer transition; it no longer assumes that the officer flag supplies native rights.

Production evidence: `guild_control.rs::resolve_rank_index`, `guild_control_set_rank`, `guild_control_get_num_ranks` and `get_num_members_in_rank` reject missing membership. `guild_probes.rs` projects membership and first roster rank. `real/wrath_guild_info.rs` reads numeric guild identity and player-rank right13. `state_backed_queries.rs::get_guild_info` returns the supplied display rank; native FriendsFrame.lua:307–309 formats it before opening the guild subframe. `character_world_defaults.rs` clears only Wrath's synthetic membership/ranks/roster and info cache.

## Exact 17-row classification

All selectors below are integration test identities. Assertions remain unchanged.

| Saved identity | Class | Input repair and evidence |
|---|---|---|
| `guild_control::c_guild_info_rank_flags_use_same_backing_state` | a | `install_three_ranks` now supplies membership before ranks; namespace table values still come from the existing rank model. |
| `guild_control::default_seeded_guild_has_rank_names` | a | Explicitly install membership and the three named ranks. Rename to `guild_control::supplied_guild_has_rank_names`; all four original assertions retained. The old name described a removed fixture policy, not a native default oracle. |
| `guild_control::get_allowed_shifts_reports_rank_order_movement` | a | Existing three-rank fixture now supplies membership; membership gate previously returned all false. |
| `guild_control::get_num_members_in_rank_counts_roster_members_by_rank` | a | Supply membership alongside the existing Alpha/Beta/Gamma roster; preserve `(1,2,0)`. |
| `guild_control::get_rank_name_by_explicit_index` | a | Supply membership before existing three ranks; preserve all names. |
| `guild_control::rank_flags_round_trip_as_boolean_array` | a | Supply membership before selecting Officer; retain the Wrath positional oracle and other-profile table oracle. |
| `guild_control::set_rank_selects_which_getter_returns_by_default` | a | Supply membership before existing ranks/selection; preserve Officer then Member. |
| `guild_control::set_ranks_updates_count` | a | Supply membership before existing three ranks; preserve count3. |
| `guild_probes::can_guild_invite_tracks_guild_membership` | a | Supply name before true observation, then remove it before false observation. |
| `guild_probes::guild_edit_permissions_track_officer_status` | a | Supply membership, active player, guildID73, player rank0 and disabled rights. At officer transition also supply right13. Preserve both denied and both allowed assertions; native predicate follows Req12, not officer-only inference. |
| `guild_probes::is_in_guild_true_when_guild_name_is_seeded` | a | Supply name before the existing state and API assertions. |
| `guild_probes::seeded_guild_roster_has_one_online_and_one_offline_member` | a | Supply Uther online/Jaina offline before all four existing roster observations. |
| `test_guild_panel::c_club_returns_guild` | a | Supply Heroes of Azeroth membership before the existing C_Club projection oracle. |
| `test_guild_panel::guild_dropdown_materialized_buttons_have_labels` | a | Supply concrete leader membership/display rank/first roster row before native load. Existing leader/rank1 branch bypasses bank-right editing, as FriendsFrame.lua:1594 shows; all three label assertions stay. Missing nonleader API is a separate consumer gap, not justification for restoring defaults. |
| `test_guild_panel::guild_panel_opens_without_unavailable_error` | a | Supply membership and display rank before load. Native ToggleFriendsFrame requires membership and formats the display rank. Preserve visibility and empty-error assertions. |
| `wrath_guild_rank_flags::native_guild_control_consumer_accepts_positional_flags` | a | Supply membership in `seed_selected_rank` before selecting the existing concrete rights. Preserve original checkbox observations. |
| `wrath_guild_rank_flags::selected_rank_flags_return_seventeen_number_or_nil_values_ignoring_arguments` | a | Same explicit membership before existing selected rights; retain arity, every position, types and ignored-argument assertions. |

The existing valid-selected-rank/no-guild case still removes membership after selection. Other no-guild controls remain unchanged. `wrath_guild_info::initialized_world_has_no_synthetic_guild_membership_or_roster` separately pins the lane's initialized-state policy; it is a preservation contract, not a new producer RED.

## F1 and remaining dropdown gap

`test_guild_panel::native_guild_info_panel_populates_motd_details` now supplies `guild_rank = Some("Member")` with name/cache before `load_guild_ui`. Its fixture and all visibility/MOTD/details assertions are byte-unchanged. Saved F1 RED reached FriendsFrame.lua:309 with nil title; post-change GREEN is pending.

`test_guild_panel::guild_control_rank_settings_dropdown_shows_rank_rows` was already failing in both saved RED and GREEN. It now gets valid leader startup inputs, but its original second-row OnClick invokes `GuildControlPopupframe_Update` for rank2. That branch requires `GetGuildBankTabPermissions` at FriendsFrame.lua:1647 and then `GetGuildBankWithdrawLimit` at :1678. Neither is registered. This row remains blocked; it is not one of the 17 regressions and is not waived.

The existing bank state is only open/money/current-tab/items (`state/sim_state.rs:606–610`); tab metadata/count and several permission functions are constants in `bank_storage_verbs.rs`. There is no rank/tab permission or rank withdrawal-limit model to reuse. Parent instruction: stage original-dropdown RED, but do not implement either getter until laptop12340 facts arrive. No constant stub, test-only function override or invented native default is permitted.

Staged selector: `test_guild_panel::native_nonleader_rank_dropdown_completes_bank_permission_update`. It loads the original selected FrameXML through the existing production loader and initializes the leader/rank1 branch first. It then supplies a nonleader roster rank and invokes the materialized Officer button's original OnClick. Native `GuildControlPopupFrameDropDownButton_OnClick` calls `GuildControlPopupframe_Update`; its rank2/nonleader branch reaches :1647 then :1678. No handler/getter replacement is installed. It observes original selected ID/text and an empty recorded-error list. This is a staged missing-function consumer RED, not executed RED or a permission-data discriminator: constants could satisfy its current completion observations. Once native facts arrive, supply concrete varying rank/tab values and assert their original widget projections before implementing either getter. The :1678 boundary is statically downstream and cannot be observed until :1647 is resolved; do not claim two independently executed REDs.

## Proof limits

All fixture repairs are static candidates, not passing-test claims. Existing cfg! branches in touched fixture files use runtime `client_profile::ACTIVE` comparisons instead; declaration cfg attributes remain registration boundaries. Non-Wrath panel loading and seed behavior are unchanged. Next-slot exact lists are `/home/osso/.local/state/wow-ui-sim/handoff/wrath-guild-info/slot0-run/static-green-repairs/{profile}-selectors.txt`, with counts/features in `selector-manifest.json`: Wrath53 and Retail53 are required lane gates; the other five profiles each have a separate53-identity list for optional shared-fixture preservation, not extra runtime authorization. Wrath53 is49 preservation identities (old48 plus new dropdown completion) plus four guild-info identities (old3 plus initialized-state preservation). Retail53 omits all nine Wrath-only identities and includes the nine unchanged batch3 exclusions. The one historical rename is recorded above; the nine exclusions are not passes. Every identity must list exactly once immediately before its separate `--exact --nocapture` run. Use the current bound revision, fresh child-only CVar copies and the existing O_NOATIME receipts. No prefix-only or zero-case PASS.

The next authorized proof must retain all statuses/first failures, separately report the two still-blocked dropdown selectors, run stock/check gates under new slot admission, and compare actual Retail stock to the requested master34/38 reference (`cf2-supertrack/slot3-run/retail-green-stock.stdout`), not the superseded36/42 map. No current Retail equivalence or club-invite delta has been measured. See the [guild-info investigation](../wiki/investigations/wrath-guild-info.md).
