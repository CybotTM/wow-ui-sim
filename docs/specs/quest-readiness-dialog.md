# Supplied quest readiness and reward-dialog transition (QL-2)

## Contract

Authored simulator policy, conditional Retail/Forever consumers; not native parity.
Depends on [QL-1 live objectives](quest-objective-progress.md). Modeled publication
and live gossip/history behavior use QL-1's Retail/Forever gate; other profiles
retain their prior readiness stub, cached gossip flags and popup defaults.
`tests/quest_readiness_scope.rs` supplies an unsupported-profile negative control
(not executed in the current Retail/Forever builds).

- A nonempty supplied objective snapshot sets active completion when every row is
  finished. An empty snapshot does not infer readiness. Failed/unknown rows are
  never ready; history is neither readiness nor active completion.
- `A_Admin.SetQuestReadiness(questID, ready, autoComplete)` supplies completion
  for conditions not modeled by objectives and explicitly configures auto-complete
  (default false). Later nonempty objective snapshots replace supplied completion.
  Rejected unknown/failed rows return false without mutation. Identical settings
  are inert. The producer returns true only for a changed accepted state.
- `C_QuestLog.ReadyForTurnIn` and selected `IsQuestCompletable` read active complete,
  nonfailed state. Legacy `IsQuestComplete` reads active completion, not history.
- Current NPC active rows and legacy completion tuples project that live readiness.
  Objective/readiness/gossip state commits before QL-1 notifications. Explicit
  readiness emits `QUEST_LOG_UPDATE()` after commit. After log notifications,
  a still-selected current progress interaction receives `QUEST_PROGRESS()` to
  refresh original progress-panel button gating before any opt-in auto notification.
- Public active-quest selection requires a current active NPC row and rich active
  quest. It opens progress if not ready, rewards if ready. `CompleteQuest` only
  advances the selected current progress interaction when ready, once. Selection
  alone via log/map does not authorize a dialog. Closing, abandonment and committed
  turn-in invalidate its interaction. Changing selection invalidates its eligibility.
- Reward-dialog entry emits `QUEST_COMPLETE()` but does not remove membership,
  modify history, watches, money, XP, inventory or grant rewards. Existing
  [turn-in model](quest-turn-in-model.md) remains separate.
- Auto-complete is opt-in per active quest. The first ready state reached while
  opted in publishes `QUEST_AUTOCOMPLETE(questID)` once per active lifetime, after
  log notifications. The notification is consumed by original tracker popup logic;
  popup add/read/remove APIs retain that resulting state. Regression/recompletion
  does not re-notify. No auto-grant/turn-in or universal automatic completion.

## Consumer grounding (cached Retail source, inspected 2026-10-16)

Cache prefix: `~/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/`.

- `Blizzard_UIPanels_Game/Mainline/GossipFrame.lua:50-69`: original
  `QUEST_LOG_UPDATE` refresh; `Shared/GossipFrameShared.lua:269-273` consumes live
  active rows; `:62-63` button click invokes `C_GossipInfo.SelectActiveQuest`.
- `Blizzard_UIPanels_Game/Mainline/QuestFrame.lua:64-72`: progress/reward panel
  reaction; `:164-166` original progress button calls `CompleteQuest`;
  `:209-213` progress button gating uses `IsQuestCompletable`.
- `Blizzard_ObjectiveTracker/Blizzard_QuestObjectiveTracker.lua:27-30`: original
  `QUEST_AUTOCOMPLETE` handler calls `AddAutoQuestPopUp(..., "COMPLETE")`;
  `Blizzard_AutoQuestPopUpTracker.lua:34-39` retains popup via public add API.

Forever cache source counterparts (reachability still unproved):
`Blizzard_UIPanels_Game/Mainline/GossipFrame.lua:51,69-71`,
`Shared/GossipFrameShared.lua:61-62,268-272`,
`Mainline/QuestFrame.lua:66-74,170-172,215-219`; original tracker
`Blizzard_QuestObjectiveTracker.lua:28-31`, popup helper
`Blizzard_AutoQuestPopUpTracker.lua:34-39`.

## Implementation

`src/c_api/quest_readiness.rs` owns opt-in/notification markers, progress/reward
interaction and readiness decisions on existing rich rows. QL-1 commits objective-
derived readiness inside its mutation scope before notifying. Modeled Lua globals
live in `globals/real/quest_dialog.rs`; original cached consumers remain unchanged.
The separate [generic XML intrinsic-tag fix](xml-intrinsic-tags.md) supplies the
missing original theme mixin/preload prerequisite; fixture closure also loads the
original AccessibilityTemplates, FrameXML and SplashFrame addons.

## Tests and proof boundary

`tests/quest_readiness.rs` / `.lua` exercise committed query callbacks, original
cached gossip data-provider refresh/click, original QuestFrame panel/button path,
and original tracker popup event path. Consumer fixtures explicitly register
`QUEST_LOG_UPDATE` on original GossipFrame and `QUEST_AUTOCOMPLETE` on original
QuestObjectiveTracker; this proves consumer reactions, not automatic startup
subscription/scheduling. Error-count snapshots reject swallowed callback errors.
No copied function bodies or API replacement.

Predicted behavioral RED after QL-1 alone: final objectives leave
`C_QuestLog.ReadyForTurnIn(990002) == false`; gossip provider retains false;
progress cannot enter rewards; opt-in completion emits zero notifications. Missing
admin publication is probed without a missing-function error. Consumer prerequisite
failures are blockers, not behavioral RED evidence. No tests/builds executed while
staging; staged producer must not land until a real in-scope RED is recorded.

Forever runs use only the current-profile cache and must establish actual closure
reachability separately. Retail source citations do not prove Forever runtime
loading, startup health, rendering or all-profile/native acceptance.
