# Supplied quest readiness and reward-dialog transition (QL-2)

Retail/Forever supplied readiness is distinct from history and reward acquisition.
Source: `src/c_api/quest_readiness.rs` and existing quest state. [Architecture](../wiki/systems/lua-api.md#supplied-readiness-and-reward-dialog-phase)
owns implementation details; [execution receipt](../baselines/quest-lifecycle.md)
owns saved proof and the current integration gate.

## What it must do

- [x] Final supplied objectives publish active readiness before log callbacks,
  without marking history; regression and separate environments remain independent.
- [x] Explicit non-objective readiness rejects unknown/failed targets; historical
  completion alone does not imply active completion/readiness.
- [x] Original gossip refresh/template click and legacy selector consume current
  readiness and open the appropriate progress/reward panel with real button gating.
- [x] Original progress button advances an eligible ready interaction once, without
  turn-in, membership/history/money/XP changes; closing invalidates further advancement.
- [x] Explicit opt-in creates one original COMPLETE popup; identical progress and
  regression/recompletion do not duplicate it. Default progress does not auto-notify.

`A_Admin.SetQuestReadiness(questID, ready, autoComplete)` supplies non-objective
completion and opt-in. Nonempty objective snapshots replace supplied completion;
empty/declared-count snapshots do not infer readiness. Active nonfailed completion
backs ReadyForTurnIn/IsQuestCompletable; legacy IsQuestComplete does not read history.

Dialog progress/rewards phase is separate from log selection. Public NPC selection
requires a current active NPC row and rich membership; selection changes, closing,
abandonment and turn-in invalidate the old interaction. Notification markers commit
before callbacks and last for one active lifetime; opting out does not reset them.
These are authored simulator policies, not native timing/reacceptance claims.

## Supplied major-faction reputation and error logging

Retail, PTR and Forever cached `QuestOfferDocumentation.lua` declare
`C_QuestOffer.GetQuestOfferMajorFactionReputationRewards()` with `MayReturnNothing`.
`QuestRewardsDocumentation.lua` declares numeric `factionID` and `rewardAmount`.

- [ ] Read ordered, separately supplied reputation rows for the active dialog,
  returning one fresh table of `{ factionID, rewardAmount }` records.
- [ ] Require matching dialog/selected identity and an active NPC quest; log
  selection alone is not a dialog. Closing invalidates the query.
- [ ] Return zero values when no rows are supplied, as **simulator policy**, not
  a measured native default. An explicitly supplied empty list returns a table.
- [ ] Keep supplied rows environment-local, live, and scoped to one active quest
  lifetime; abandonment/turn-in clear them without granting reputation.
- [ ] Publish Forever's real `C_Log.LogErrorMessage` sink: append the exact
  ordinary string once to environment-local console output, with zero returns.
  Preserve classic and Retail/PTR publication and string-handling behavior.

`src/c_api/c_quest_offer.rs::QuestOfferState.major_faction_rewards` is supplied
input keyed by quest ID, stored in `QuestLogState.offer`. Existing
`QuestLogEntry.reward_factions` pairs are **not** reclassified: their representation
and the major-faction catalog do not establish native reward eligibility. Native
classification/derivation remains unknown. PTR tests supply the existing dialog
context directly; this does not add a PTR QL-2 producer or widen a Retail epoch.

`tests/c_quest_offer.rs` covers absent/supplied output, identity, live snapshots,
close invalidation, lifetime cleanup and isolation. `tests/forever_log_sink.rs`
covers actual sink output/arity/isolation and Retail/PTR publication preservation.
Both are discovered automatically by the integration harness. Focused RED/GREEN
and implementation are pending; this section defines the bounded repair contract.
Native secret-input parity and additional namespace members remain out of scope.

## How it works

- [Readiness architecture](../wiki/systems/lua-api.md#supplied-readiness-and-reward-dialog-phase)
- [QL-1 objective contract](quest-objective-progress.md)
- [Separate turn-in contract](quest-turn-in-model.md)

## Implementation inventory

- `src/c_api/quest_readiness.rs`: readiness, phase and notification state/decisions.
- `src/lua_api/sim_substates/quest_log.rs`: environment-owned state and removal cleanup.
- `src/lua_api/globals/real/quest_dialog.rs`: modeled globals, popup storage and admin ingress.
- `globals/admin_quest_objectives.rs`, `admin_quests.rs`: pre-notification mutation seam/registration.
- `globals/quest_surface/`, `missing_surface/gossip_info.rs`, `quest_verbs.rs`: projections/selectors/cleanup.

Publication/live behavior uses QL-1's Retail/Forever gate; other profiles retain
prior readiness stubs, cached gossip flags and popup defaults.

## Tests asserting this spec

`tests/quest_readiness.rs` / `.lua`: five shared cases use whole cached closures,
not rewritten consumer bodies. Callback error-count guards reject swallowed errors.
Original consumers explicitly subscribed by fixtures do not prove automatic startup
subscription/scheduling. `quest_readiness_scope.rs` is an unexecuted non-target control.

Cached Retail sources: Mainline/GossipFrame.lua:50-69 and
Shared/GossipFrameShared.lua:62-63,269-273; Mainline/QuestFrame.lua:64-72,164-166,209-213;
QuestObjectiveTracker.lua:27-30 and AutoQuestPopUpTracker.lua:34-39. Forever counterparts
are loaded independently in the recorded profile executions.

Behavioral RED preceded producers. Initial closure/missing-API failures were kept
separate; [generic intrinsic](xml-intrinsic-tags.md), [widget values](xml-widget-values.md),
[material](quest-background-material.md), [temporary inputs](quest-progress-requirements-empty.md)
and [currency projection](quest-reward-currency-panel.md) own those prerequisites.
The historical [receipt](../baselines/quest-lifecycle.md#latest-base-integration-proof--2026-10-17)
binds its source revisions, not current master acceptance. On `5954007e4`, fresh
unchanged Forever original-gossip/progress cases both fail their swallowed-error
guards: missing `C_QuestOffer` at QuestInfo.lua:655, then missing `C_Log` at
ScriptErrors.lua:83. The supplied-input/query and sink repair above owns this gap.

## Known gaps (current cycle)

- [ ] Non-target profile runtime control remains unexecuted; not part of this two-profile matrix.

## Out of scope

Native parity/timing, full tracker scheduling/pixels, failure/deadline algorithms,
reward validation/grants, new acquisition/persistence or all-profile acceptance.
