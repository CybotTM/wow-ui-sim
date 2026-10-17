# Explicit startup namespace empty states (staged)

## Era / Anniversary startup extension

- [x] `C_LootHistory.GetNumItems()` returns zero pending a Classic roll-history
  model; original OnLoad and full-update handlers clear/recycle stale UI rows.
- [x] `C_ProductChoice.GetChoices()` returns a fresh empty identifier array pending
  a product-entitlement catalog; original OnLoad initializes the reward panel,
  and login/update handlers complete without creating reward alerts.
- [x] `C_Engraving.IsEngravingEnabled()` returns false pending engraving availability
  state; the original rune control OnLoad hides an initially shown control.
- [x] Unknown members remain nil on these three Era/Anniversary namespaces, even
  after exported-constructor reuse. Existing non-strict factories remain unchanged.

Only IsEngravingEnabled has a cached generated signature (non-nil bool).
GetNumItems and GetChoices are consumer-inferred number/array queries: Classic
caches omit their generated documentation. These policies are not client defaults.

Profile-only temporary stubs live in `src/era/compat_bootstrap.lua` per the profile
placement rule; Era and Anniversary share that file. Retire each when its named
missing system is represented under c_api. Retail's encounter/time API and active
loot-roll state are not Classic completed-roll/player history; C_PlayerChoice is
not C_ProductChoice. Mists's pre-existing empty loot/product defaults are not a
backing model. This extension changes no Mists/Forever/Wrath/ Retail/PTR surface.

`tests/classic_startup_namespaces.rs` loads complete hash-pinned cached consumers
using `blizzard_ui_addons_dir()`. Five tests cover the four original-handler cases
plus complete-bootstrap strictness/factory preservation. Tests-only Era RED at
fbf25a24d reproduced missing C_LootHistory at LootHistory.lua:55, C_ProductChoice
at ProductChoice.lua:63 (OnLoad and dispatched events), C_Engraving at
PaperDollFrame.lua:951, and the absent raw namespace preservation control.
Producer GREEN passed all five tests on both Era and Anniversary. Sealed headless
startups on this branch each measured 87 unique records / 138 occurrences versus
the supplied 26877f296 captures' 95/150: eight records disappeared, no new records,
and the existing ScriptErrors.lua:83 reporter count fell 50→46. Removed records
are LootHistory OnLoad :55, ProductChoice OnLoad and OnEvent :63, and rune-control
OnLoad PaperDollFrame :951, each with its Lua Error mirror and count one.
These are still failing startups, not full-profile acceptance. Build/runtime
receipts and exact-message deltas live in the private classic-ns-apply handoff.
Populated loot history, product selection, engraving modes/runes, reporter repair,
and full startup acceptance are outside this bounded extension.

### Explicit consumer exclusions

`Blizzard_UIPanels_Game/Classic/ContainerFrame_Shared.lua:54,95,315-316,1078,1088-1089,1111`
bag/rune events and bag-generation paths remain unsupported:
`C_Engraving.GetEngravingModeEnabled`, `GetCurrentRuneCast`, and `RefreshRunesList`
remain nil. The first two are called without an availability guard;
`alwaysShowRuneIcons=1` can reach RefreshRunesList even with availability false.
Forced engraving UI and PaperDoll engraving interaction paths also remain excluded.
`Blizzard_SettingsDefinitions_Frame/Classic/InterfaceOverrides.lua:136` uses
IsEngravingEnabled and skips its setting when false; that is not bag/rune coverage.

### Revision-pinned startup comparisons

Units are unique records / total occurrences, not independent root causes.
Current comparison master **972eef0bed0dd81c2f2529d64366e35563dd7320** has
**Era 36/38 and Anniversary 36/38**. Each contains six affected loot/product
records (three primary, three mirrors), no engraving missing-namespace record,
and no ScriptErrors.lua:83 record. Removing only those six gives conditional
**30/32**, not a measured combined master + branch result.

The **historical 92/98** approved baselines (both blob SHA-256
`2cf8194055fa25eb3aae79882b450671904e9febab0dd5335f12bca0b4db8b1b`)
would similarly become **86/92** if unrelated records stayed unchanged. These
historical numbers are not the current master comparison.

The supplied wrath-fixes **26877f29636686760594a95ef9a3c63747fd8984** captures
are **95/150** for both profiles. Eight affected records (four primary failures
plus four Lua Error mirrors) predict **87/142** with reporter count held fixed;
if one reporter occurrence disappears per primary, **87/138** (reporter 50→46).
Those conditional branch totals versus current master are **+51/+104** and
**+51/+100**, respectively, not measured stage regressions or combined-branch
predictions. Historical branch-versus-92/98 arithmetic **-5/+40** applies only
to that older comparison. Execution must compare sealed branch startups to the
supplied captures at 26877f296, not substitute master's different baseline.

## Forever and classic extension (source-only)

Share existing combat-text active-unit selection and spell-overlay membership with
Mists, Era, Anniversary and Forever, whose cached API docs declare those methods.
Keep Wrath namespace absence. Share the existing local PlayerChoice model with
Forever; share RecentAllies storage/query code with Forever without publishing it
to additional classic profiles. Forever's disabled GetRecentAllies returns an
empty array as declared, rather than the Retail disabled no-result policy.
Share the existing SearchRecentAllies producer with Forever without changing its
Retail epoch gate or adding a second search model. No RecentAllies service,
request completion, readiness or note/pin mutation behavior is added. Forever's
IsRecentAllyByGUID and IsRecentAllyByFullName query enabled supplied rows by exact
GUID/full-name identity; disabled input returns false, including retained rows.

Forever-only temporary defaults: no flyout artwork (GetFlyoutTexture returns no
values); no mentor membership (None), no newcomer status, restricted mentoring,
no required level, fresh empty achievement arrays plus zero optional count; no
RecentAllies game-type support or supported filter categories. Social UI reuses
the existing temporary disabled-service policy with Forever admitted explicitly.
Mists/Era/Anniversary/Forever GetCurrentEventInfo has no returns pending modeled
combat-text event routing. These defaults are simulator choices, not native facts.

Retire defaults when c_api models flyout artwork, mentorship membership/eligibility,
social-UI availability, recent-allies game-type support, or combat-text event
payload routing. Do not add missing request/mutation no-ops to fill out namespaces.

Ten original-consumer tests are staged: three shared Mists/Era/Anniversary/Forever
cases and seven Forever-only cases. Tests load complete hash-pinned cached chunks
from blizzard_ui_addons_dir and assert alert visibility under supplied membership,
deprecated alias state effects, vehicle unit transitions, hidden choice toggle,
empty supported-filter list, false mentor/social predicates, cleared unavailable
flyout texture, disabled/supplied RecentAllies list rows, and text/status-filtered
social-view trees. Only consumer-required dependencies are loaded.

Preserve the tip's generic namespace member factory, constructor, merger and final
C_* seed. Only newly published surfaces are strict: C_CombatText and
C_SpellActivationOverlay in Mists/Era/Anniversary/Forever, plus C_PlayerChoice,
C_RecentAllies, C_PlayerMentorship, C_SocialUI and C_Flyout in Forever, plus
C_LootHistory, C_ProductChoice and C_Engraving in Era/Anniversary. Keep the
exemption inventory in one bootstrap location. Their missing members return nil
through ordinary lookup without changing membership, including after reusing the
exported constructor. Other namespaces retain synthesized cached nil-returning
functions. A complete-bootstrap test covers only these strict surfaces; a separate
C_CVar/constructor control asserts unchanged non-strict behavior. Retail/PTR/Wrath
receive no strict exemptions. Generic member-factory retirement is out of scope.

Predicted RED/GREEN only; no executed tests/builds. Mists store selected-character errors
and downstream error-reporter behavior remain separate, unresolved startup gates.
No clean startup, full-profile or native acceptance is asserted.


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
