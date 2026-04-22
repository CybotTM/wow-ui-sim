# Layout Lock Inventory

Canonical inventory of UI elements whose layout is explicitly locked down by regression tests.

## Content

This page treats a layout as "locked" when tests assert concrete geometry/anchor invariants (positions, sizes, relative attachment, visibility/alpha constraints tied to layout).

### Baseline global frame locks

`tests/frame_positions.rs` (`POSITION_TESTS`) locks these top-level frame rects at startup:

- Player/unit/group: `PlayerFrame`, `TargetFrame`, `FocusFrame`, `PaladinPowerBarFrame`, `PartyFrame`, `CompactPartyFrame`
- HUD/objective/minimap: `Minimap`, `MinimapCluster`, `ObjectiveTrackerFrame`, `UIParentRightManagedFrameContainer`
- Action/bag/micro/status: `MainActionBar`, `StatusTrackingBarManager`, `BagsBar`, `MicroButtonAndBagsBar`, `MicroMenu`, `MicroMenuContainer`
- Aura/chat: `BuffFrame`, `DebuffFrame`, `ChatFrame1`, `ChatFrame1EditBox`, `GeneralDockManager`
- Overlay/warnings: `UIErrorsFrame`, `PrivateRaidBossEmoteFrameAnchor`, `CriticalEncounterWarnings`, `MediumEncounterWarnings`, `MinorEncounterWarnings`
- Cast bar: `PlayerCastingBarFrame`
- Action button sentinel: `ActionButton1` x-position and alpha

### Subsystem-specific deep locks

`tests/objective_tracker_tree.rs` (`objective_tracker_frame_layout_is_locked`):

- `ObjectiveTrackerFrame` shell
- `ObjectiveTrackerFrame.Header` and `ObjectiveTrackerFrame.Header.Text` (`"All Objectives"`)
- `QuestObjectiveTracker`, `QuestObjectiveTracker.Header`, `QuestObjectiveTracker.ContentsFrame`
- Quest header decoration/controls: `Background`, `Shine`, `Glow`, `Text`, `MinimizeButton`

`tests/action_bar_tree.rs` (`main_action_bar_layout_is_locked`):

- `MainActionBar`, `MainActionBar.EndCaps`, `LeftEndCap`, `RightEndCap`
- `MainActionBar.ActionBarPageNumber`, `UpButton`, `DownButton`, `Text`
- `MainActionBarButtonContainer1..12`
- `ActionButton1..12`, `ActionButton1Icon..ActionButton12Icon`, `ActionButton1NormalTexture..ActionButton12NormalTexture`
- `ActionButton1HotKey`

`tests/action_bar.rs` (`test_status_tracking_xp_and_reputation_bars_layout_locked`):

- `StatusTrackingBarManager`
- `MainStatusTrackingBarContainer`, `SecondaryStatusTrackingBarContainer`
- Active XP/reputation bars in those containers and their nested `StatusBar` regions

`tests/action_bar.rs` (`test_bag_bar_layout_locked`):

- `BagsBar`
- `MicroButtonAndBagsBar` (bags anchor dependency)
- Bag chain: `MainMenuBarBackpackButton`, `BagBarExpandToggle`, `CharacterBag0Slot`, `CharacterBag1Slot`, `CharacterBag2Slot`, `CharacterBag3Slot`, `CharacterReagentBag0Slot`

`tests/micro_menu.rs` (`micro_menu_layout_stays_locked`):

- `MicroButtonAndBagsBar`, `MicroMenuContainer`, `MicroMenu`
- Micro buttons: `CharacterMicroButton`, `ProfessionMicroButton`, `PlayerSpellsMicroButton`, `AchievementMicroButton`, `QuestLogMicroButton`, `MainMenuMicroButton`

`tests/chat_frame.rs` (`chat_frame_layout_stays_locked`):

- `ChatFrame1`, `ChatFrame1Background`, `ChatFrame1EditBox`
- `ChatFrame1.ScrollBar`, `ChatFrame1.ScrollToBottomButton`
- `ChatFrame1ResizeButton`, `ChatFrame1ButtonFrame`, `ChatFrameMenuButton`, `ChatFrameChannelButton`

`tests/compact_raid_manager_visibility.rs` (`compact_raid_manager_layout_stays_locked_when_collapsed_and_expanded`):

- `CompactRaidFrameManager` in both collapsed and expanded states
- `CompactRaidFrameManager.displayFrame`
- Toggle buttons: `toggleButtonForward`, `toggleButtonBack`
- `CompactRaidFrameManager.BottomButtons`
- `CompactRaidFrameContainer`

`tests/test_showuipanel.rs` (`show_ui_panel_locks_character_frame_layout`):

- `CharacterFrame` shell and equipment-slot grid
- Left column: `CharacterHeadSlot`, `CharacterNeckSlot`, `CharacterShoulderSlot`, `CharacterChestSlot`, `CharacterShirtSlot`, `CharacterTabardSlot`, `CharacterWristSlot`
- Right column: `CharacterHandsSlot`, `CharacterWaistSlot`, `CharacterLegsSlot`, `CharacterFeetSlot`, `CharacterFinger0Slot`, `CharacterFinger1Slot`, `CharacterTrinket0Slot`, `CharacterTrinket1Slot`
- Bottom weapons: `CharacterMainHandSlot`, `CharacterSecondaryHandSlot`

`tests/test_showuipanel.rs` (`show_ui_panel_locks_reputation_frame_layout`):

- `CharacterFrame` + `ReputationFrame` panel geometry coupling
- `ReputationFrame.filterDropdown`, `ReputationFrame.ScrollBox`, `ReputationFrame.ScrollBar`
- `ReputationFrame.ReputationDetailFrame`
- Detail internals: `Title`, `AtWarCheckbox`, `MakeInactiveCheckbox`, `WatchFactionCheckbox`

`tests/blizzard_ui_unit.rs` (`buff_frame_icons_and_durations_stay_locked`):

- Active helpful `BuffFrame` aura buttons (seeded to 3 in test)
- Per-button `Icon` size/anchor/texture constraints
- Per-button `Duration` visibility/text/anchor constraints
- Row alignment and horizontal spacing across visible buff buttons

### Maintenance checklist

- When adding a new layout lock test, update this page and `docs/wiki/index.md`.
- Prefer adding a dedicated subsystem lock test when a failure mode is more specific than baseline frame rect drift.
- Keep `tests/frame_positions.rs` for broad startup coverage and subsystem tests for intra-tree invariants.

## Sources

- [tests/frame_positions.rs](../../../tests/frame_positions.rs) — baseline frame lock list (`POSITION_TESTS`)
- [tests/objective_tracker_tree.rs](../../../tests/objective_tracker_tree.rs) — objective tracker structure and header/title locks
- [tests/action_bar_tree.rs](../../../tests/action_bar_tree.rs) — main action bar tree/layout lock
- [tests/action_bar.rs](../../../tests/action_bar.rs) — XP/reputation status bars and bag bar chain locks
- [tests/micro_menu.rs](../../../tests/micro_menu.rs) — micro menu container/button geometry locks
- [tests/chat_frame.rs](../../../tests/chat_frame.rs) — chat frame/edit/scroll/button frame lock
- [tests/compact_raid_manager_visibility.rs](../../../tests/compact_raid_manager_visibility.rs) — compact raid manager collapsed/expanded lock
- [tests/test_showuipanel.rs](../../../tests/test_showuipanel.rs) — character/reputation panel lock coverage
- [tests/blizzard_ui_unit.rs](../../../tests/blizzard_ui_unit.rs) — buff icon/duration lock coverage

## See Also

- [[blizzard-ui-test-lanes]] — where these regression tests fit in the broader test-lane split
- [[chatframe-scrollbar-anchor-reapply]] — root-cause investigation that motivated stricter chat layout lock assertions
- [[editmode-layout]] — prior layout regressions from EditMode override paths
