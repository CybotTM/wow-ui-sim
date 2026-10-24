# Wrath guild-info readers

## What it must do

- Wrath `GetGuildInfoText` ignores arguments and returns exactly one cached string, initially `""`, even without membership; it requests nothing.
- Wrath `CanEditGuildInfo` ignores arguments and returns exactly one number `1` only with an active player, nonzero player guild ID and that player's rank right `0x10000`; otherwise exactly one nil. Officer/selected-admin-rank shortcuts are not equivalent.
- Reuse `WorldState.guild_info_text` and `guild_ranks[].flags`: position 13 already represents `0x10000`. Store player guild ID/rank alongside existing world guild data; active-player presence is an explicit `PlayerState` input. Initialize without guild or rights; do not seed Wrath with Retail description text.
- Preserve all non-Wrath registration/default behavior using runtime `client_profile::ACTIVE` selection.
- Execute original selected FriendsFrame.xml `GuildInfoFrame` OnShow: empty editable text becomes `GUILD_INFO_EDITLABEL`, supplied text is retained; white/enabled/mouse-editable versus grey/disabled/noneditable follow permission. Native `cachedText` UI override stays intact.

## Evidence and boundaries

Accepted laptop answer: `/home/osso/.local/state/wow-ui-sim/handoff/laptop-labels-answers-20261018/laptop-answers-req-12.md`, read fully. Static reconstructed build12340 evidence, not native execution. No decompiled source read in this lane.

Native SetGuildInfoText does not mutate the received cache. This change implements no setter, packets, request throttling, automatic server refresh, leave/rejoin reset policy or unknown behavior. Direct simulator state / existing `A_Admin.SetGuildInfoText` supplies received cache content; it is not a client setter or a modeled network reception. Existing C_GuildInfo mutation semantics are unchanged.

## Tests and proof

`tests/wrath_guild_info.rs`: two independently discriminating original-consumer selectors and one exact return-contract selector. Tests/model input scaffolding precede producer commit. Historical slot proof at `97e759f7d` passed all three selectors but found F1 and preservation input gaps. The [17-row classification and fixture repairs](../baselines/wrath-guild-info-green-blockers.md) preserve all existing assertions, supply membership/ranks/roster explicitly and add F1's display rank before load. The initialized-state preservation case separately pins absent synthetic membership/ranks/roster. No builds/tests are authorized for these repairs: their GREEN remains pending. Selection trace and proof limits are in [investigation](../wiki/investigations/wrath-guild-info.md).

The separate [Wrath guild-bank rank-permission contract](wrath-guild-bank-rank-permissions.md) now covers the two dropdown blockers with explicit selected-rank inputs and staged getters. It does not change this guild-info permission predicate; compiled proof for both additions remains pending.

## Implementation

`globals/real/wrath_guild_info.rs` registers the existing cache reader and native permission projection, selected by `guild_probes.rs`. `character_world.rs` adds explicit inputs; `character_world_defaults.rs` removes Wrath's synthetic guild seed only. No parallel cache or rights store.
