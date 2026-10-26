# Supplied challenge run and scenario criteria

## Contract

The environment owns at most one active challenge run. Admin input supplies its challenge-map ID, key level, affix IDs, charged flag, deaths and time lost. No catalogue or difficulty policy is inferred. Starting stores the snapshot before `CHALLENGE_MODE_START`. Clearing removes the active run and its timers before `WORLD_STATE_TIMER_STOP(timerID)` and `CHALLENGE_MODE_RESET`. Completion first stores the supplied completion snapshot, removes the run/timers, then emits `CHALLENGE_MODE_COMPLETED`.

Map metadata is an environment-local supplied catalogue. Existing metadata is not expanded with invented entries. A missing supplied map has no result on the modeled profiles. Ordered criteria belong to a supplied scenario step. Updates replace that step's ordered rows before one `SCENARIO_CRITERIA_UPDATE(criteriaID)` per changed supplied row. Returned records and affix arrays are detached copies. The modern namespace is used; retired `C_Scenario.GetCriteriaInfo*` members remain retired.

World elapsed timer records belong to the active run and contain the supplied timer ID, display name, type and elapsed seconds. They are exposed before `WORLD_STATE_TIMER_START(timerID)`. Set/advance are explicit, nonnegative finite inputs; no wall clock is used. Missing timers have no result. Run reset/completion removes its timers.

## Pinned declarations and publication

Paths below are relative to each profile's pinned `AddOns/Blizzard_APIDocumentationGenerated/` cache; the committed manifests select the inputs.

| Profiles | File and lines | Members |
|---|---|---|
| Retail, PTR, Forever | `ChallengeModeInfoDocumentation.lua:34,43,81,172,273` | `GetActiveChallengeMapID`, `GetActiveKeystoneInfo`, `GetDeathCount`, `GetMapUIInfo`, `IsChallengeModeActive` |
| Mists, Era, Anniversary | `ChallengeModeInfoDocumentation.lua:11,202` | `GetActiveChallengeMapID`, `IsChallengeModeActive`; no key-info or death-count additions |
| Retail, PTR, Forever | `ScenarioInfoDocumentation.lua:11-41,263-284` | modern criteria readers and 14-field record |
| Mists, Era, Anniversary | `ScenarioInfoDocumentation.lua:11-39,256-277` | same modern criteria readers and record |
| Wrath | No corresponding pinned files | No new namespace members |

The already-present elapsed-timer globals keep their publication scope. `WorldStateInfoDocumentation.lua:64-83` declares start/stop payloads; `WorldElapsedTimerConstantsDocumentation.lua:14` declares ChallengeMode=1 on modern profiles. Admin input supplies timer types; it does not infer them.

## Selected consumers

Retail selects `Blizzard_ObjectiveTracker.toc` and `Blizzard_ChallengesUI_Mainline.toc`. PTR selects unified `Blizzard_ObjectiveTracker.toc` and `Blizzard_ChallengesUI.toc` (Standard family). Forever selects the objective tracker but its `camelot` game type excludes the ChallengesUI TOC (`standard, mists` only). Scenario criteria and timer proof use the unmodified selected `Blizzard_ScenarioObjectiveTracker.lua`, original XML frames and method identity. Mists has a Mists ChallengesUI and legacy WorldStateFrame timer consumer; Era/Anniversary exclude that mists-only TOC. Wrath is excluded.

## InTimeBL boundary

Core reads map ID and active flag on addon load/start/resync, key level and deaths on start, `(name, elapsed)` from world timer 1, and map time limit from result 3. Tracker reads the current scenario step count and each indexed criteria record; it distinguishes weighted forces from boss criteria (165, or 0 with total=1), uses `quantityString`, `totalQuantity`, `assetID`, completion and description. Recorder reads the last completion snapshot. Forecast refresh uses a one-second `C_Timer.NewTicker` and `GetTime()-startTime`, so a controllable common clock is required for deterministic minutes-long acceptance.

## Proof requirements

Each original-consumer selector must contradict constant/no-op producers independently. Preserve byte-identical tests on a producer-subtracted RED sibling. Add external InTimeBL forecast acceptance without changing its sources. Final gates include all seven zero-warning checks, default-mode Retail/PTR stock with bound binary/revision/argv/receipt, individual added-key tracing, formatting and changed-code readability.
