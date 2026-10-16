# Explicit startup namespace empty states (staged)

## Contract

Replace reliance on removed generic C_* namespace synthesis with explicit,
profile-scoped members. No arbitrary namespace/member fabrication; no Blizzard
source changes. These temporary defaults describe simulator empty state, not
native behavior or defaults established by API documentation.

| Namespace | Empty behavior | Profiles with cached declaration |
|---|---|---|
| C_QuestSession | Start/stop unavailable; no session, join, pending command, begin details, or supertracked session quest; command None (0); proposed level 0 | Retail, PTR, Mists, Era, Anniversary, Forever |
| C_ClientScene | No active scene | IsSceneTypeActive: Retail, PTR, Forever; namespace-only declaration in Mists/Era/Anniversary |
| C_FrameManager | Managed feature frames hidden until event says otherwise; query refresh returns empty-policy false | Retail, PTR, Forever |
| C_LobbyMatchmakerInfo | Not queued; mainline queue disabled | IsInQueue: all except Wrath; GetQueueFromMainlineEnabled: Retail, PTR, Forever |
| C_CovenantCallings | Locked; request synchronously emits COVENANT_CALLINGS_UPDATED with a fresh empty BountyInfo array | Retail, PTR, Forever |

Namespace absence in Wrath must remain absence. Declaration alone does not
permit methods absent from that profile's documentation. Preserve raw existing
members; do not install metatable catchalls. Unsupported queue/session mutation
commands remain absent rather than accepting and discarding inputs. Queued-only
playlist/state/time queries remain outside this empty startup slice.

## Consumer-inferred public namespace

Cached BoostTutorial.lua:224 in Retail/PTR/Forever calls
`C_CharacterServicesPublic.ShouldSeeControlPopup()`. No cached profile's API
documentation declares the signature. User-approved temporary policy returns
false on those observed profiles; original BoostTutorial initialization leaves
its control popup hidden. This is consumer-inferred compatibility, not an
API-doc-declared signature or native default. Existing active boost fields are
not used to invent a popup rule. Retire this default when character boost/services
popup state is modeled.

## Retirement

Replace each temporary namespace with a c_api-backed producer when the quest
session coordinator, client-scene lifecycle, frame-manager visibility policy,
lobby matchmaking service, or covenant calling snapshots are modeled. Callings
requests must retain an observable completion effect, not become no-ops.

## Proof level

Source-only staging against wrath-fixes 6a86078991b16790755631d268aa861742631bd3.
Original complete cached Lua chunks are hash-pinned by the consumer tests.
Tests assert dialog/mode/frame visibility, LFD button label and role-warning
state, and calling completion payload/count/throttle. Profile surface tests
assert declared presence, empty query outputs, and unsupported member absence.
No build, test execution, clean startup, all-profile runtime, or native-parity
claim. Public popup policy was subsequently authorized from its cached consumer;
its original-consumer regression remains required, not ignored.
