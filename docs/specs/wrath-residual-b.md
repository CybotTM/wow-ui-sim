# Wrath residual producer boundaries B

Wrath-only legacy projections must satisfy unchanged cached FrameXML positional
reads. These requirements are bounded simulator contracts, not native parity.

## What it must do

- [ ] Supply targetOfTargetMode UI default `5`, grounded in shipped
  FrameXML/InterfaceOptionsFrame.lua:319; preserve overrides and unknown-key nil.
- [ ] Invert explicit BaseMip inputs through the shipped control getter
  (VideoOptionsPanels.xml:684-689). No unestablished default may be seeded.
- [ ] Select modeled skill records and project the thirteen fields read by
  SkillFrame.lua:192; rank/temporary points must support :194 arithmetic.
- [ ] Store numeric SetWhoToUI(0/1) transitions from FriendsFrame.xml:2635-2643;
  share state with C_FriendList.SetWhoToUi. Explicit query response fixtures
  route to UI results/event or chat, according to that state.
- [ ] Enumerate current legacy-map landmarks as the six fields consumed by
  WorldMapFrame.lua:276-300; map selection must change count and rows together.
- [ ] Request selected battleground data before GetBattlefieldInfo reads name,
  description, and explicit max-group input (PVPBattlegroundFrame.lua:97,147-151).
- [ ] Count eligible owned companions, not every journal/catalog row, for the
  PetPaperDollFrame.lua:51-64 availability branch.
- [ ] Project Wrath LFG texture/difficulty/description/holiday at positions
  10/11/13/14 (LFDFrame.lua:814-826); leave other profiles' tuples unchanged.

## How it works

- [Lua API](../lua-api.md)
- [Client profiles](../wiki/systems/client-profiles.md)

## Implementation inventory

- `src/lua_api/globals/real/wrath_legacy_state.rs`: explicit legacy records.
- `src/lua_api/globals/real/wrath_legacy.rs`: legacy read/write adapters.
- `src/c_api/who_routing.rs`: shared who routing state and delivery.
- `src/cvars.rs`: source-grounded historical UI default.
- `src/lua_api/globals/real/wrath_legacy_lfg.rs`: profile-specific tuple.

## Tests asserting this spec

`tests/wrath_residual_b.rs`, with unchanged cache inputs pinned in
`tests/wrath_residual_b_pins.tsv` and explicit surrounding UI fixtures.

## Known gaps (current cycle)

- [ ] Build/run RED and GREEN: forbidden in staging; all results predicted only.
- [ ] BaseMip default remains a decision; fresh Wrath startup is not fixed by
  registering an explicit value in a test.
- [ ] No-selection skill tuple, map-selection identifiers, companion eligibility,
  response delivery timing/defaults require policy review. See handoff POLICY.md.
- [ ] Full Skill/Who XML initialization and complete startup inventory remain
  separate acceptance work; tests isolate original callbacks/functions.
- [ ] Populated LFD rewards are outside this patch's zero-reward renderer case.

## Out of scope

Clusters 3/4/6/8/9/15; Blizzard/vendor edits; native parity; complete legacy
map/skill/collection implementation; guessed historical defaults.
