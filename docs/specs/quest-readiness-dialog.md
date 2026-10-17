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
Saved profile GREEN and latest-base acceptance are distinguished in the receipt.

## Known gaps (current cycle)

- [ ] Close the latest-base quest/startup/B5 integration gate in the execution receipt.
- [ ] Non-target profile runtime control remains unexecuted; not part of this two-profile matrix.

## Out of scope

Native parity/timing, full tracker scheduling/pixels, failure/deadline algorithms,
reward validation/grants, new acquisition/persistence or all-profile acceptance.
