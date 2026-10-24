# Club membership

Club state lives in `SimState.clubs`, implemented under `src/c_api/`. Lua receives opaque strings, not roster indices. Contract: [club membership](../../specs/club-membership.md).

## Identity and management

`ClubState` allocates member/club/invitation tokens. Guild roster projection matches full character names, preserving IDs under reordering; removal/rejoin allocates a fresh ID. This mapping is INFERRED because the existing GuildMember producer has no GUID. Guild ranks do not confer community privileges. The legacy producer lacks GUIDs, so projected members also receive stable simulator `Player-0-<hex>` GUIDs (INFERRED). These are separate from opaque member IDs; cached `GuildRoster.lua:17` requires a GUID before building rank menus. Host updates retain an omitted prior GUID.

Community creation installs the player as owner. INFERRED policy: owner manages lower roles; leader manages moderators/members; moderator manages members. Owner is required/unique; transfer demotes old owner to leader atomically. Member notes distinguish own/other privileges. Invitations reference server-supplied candidates and never invent accepted membership. Restrictions and initialization are explicit host inputs; denied mutations are silent and have no side effects.

Successful changed notes emit MEMBER_UPDATED; role/kick operations emit documented role/removal payloads. Local invitation changes emit INVITATIONS_RECEIVED_FOR_CLUB. Local completion, change-only event policy and role privilege mapping are INFERRED, not native service claims.

## Host events and messages

`c_club::receive_member` and `receive_member_removal` apply explicit host snapshots/departures before dispatching synchronous callbacks. New IDs emit ADDED; changed presence/role emit their detailed events; changed names/notes emit UPDATED. Membership arrival clears pending invitations. Repeated snapshots/departures emit nothing. Guild host inputs also update the legacy roster producer so subsequent queries cannot undo them.

Guild chat stores opaque author IDs and retains historical author snapshots. Seeded conversations bind authors once, not by current roster order. Missing author snapshots report errors rather than fabricating Unknown members. String representation, projection-time fixture binding and initial missing guild rank 1 are INFERRED.

## Incoming self invitations (CF-2)

Edit-only staged 2026-10-24; not runtime-accepted. [Bounded contract and standalone contradiction table](../../specs/club-self-invitations.md) own the requirements and deferred proof.

`ClubState.self_invitations` is an explicit supplied active-player snapshot list, empty by user requirement. It is separate from each club's outgoing `Invitation` list and subscribed membership: receiving an invitation does not create a club or member. `club_self_invitations.rs` projects fresh arrays/records without allocating opaque IDs, syncing the guild, mutating state or emitting events. Required ClubInfo fields come from InvitedClub; existing Member projection handles inviter/leaders. Nilable ClubInfo metadata stays absent. Native token encoding and uninitialized-club failure semantics are not inferred.

Retail selection is `_Mainline` TOC → Mainline/MainMenuBarMicroButtons.lua + XML → XML-created GuildMicroButton → original HasUnseenInvitations :1230. The staged selector loads the normal selected dependency closure, checks provenance, and changes supplied invitations/displayed club IDs through both outcomes; no Lua API or consumer is replaced. Other profiles have no claimed consumer proof from this lane. Prior 2026-10-05 membership evidence does not cover this addition.

## Sources

- [Contract](../../specs/club-membership.md).
- [Self-invitation contract](../../specs/club-self-invitations.md): ClubDocumentation.lua :580–588, :1760–1776, :1808–1844, :1938–1946 and selected Retail MicroMenu :1229–1238; only these generated docs and consumer ground the new contract.
- `~/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/Blizzard_APIDocumentationGenerated/ClubDocumentation.lua`: declarations, privileges, event payloads.
- Cached `Blizzard_Communities/CommunitiesMemberList.lua:632–743`: ID-keyed lookups, presence/role updates, invitation cancellation.

## See Also

- [[patch-12-0-7-api-audit]] — five opaque member event rows; ledger remains unchanged.
