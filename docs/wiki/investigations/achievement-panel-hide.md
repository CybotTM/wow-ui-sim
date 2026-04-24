# Achievement Panel Hide

The achievement panel hide bug had two simulator-side causes: the `ToggleAchievementFrame` workaround bypassed Blizzard's managed panel path, and animation completion only fired group `OnFinished` handlers even though Blizzard alert XML puts hide scripts on child animations.

## Content

`ToggleAchievementFrame` is a managed UI panel flow. Blizzard's `AchievementFrame_ToggleAchievementFrame()` calls `ShowUIPanel(AchievementFrame)` and `HideUIPanel(AchievementFrame)`, which route through `FramePositionDelegate` and keep panel-manager state synchronized. The simulator workaround loaded `Blizzard_AchievementUI` but then directly called `AchievementFrame:Show()` / `AchievementFrame:Hide()`. That could flip the frame shown flag without exercising the managed-panel hide path.

The workaround now patches the summary empty-text overlap as before, then delegates to `AchievementFrame_ToggleAchievementFrame(stats, toggleGuildView)` when Blizzard has loaded it. The fallback path also prefers `ShowUIPanel` / `HideUIPanel` before direct frame show/hide.

Achievement alerts revealed a related animation gap. `AchievementAlertFrameTemplate` uses `waitAndAnimOut.animOut` with an `OnFinished` script on the child `Alpha` animation:

```lua
self:GetRegionParent():Hide();
```

The simulator advanced animation groups and fired group `OnFinished`, but child animation `OnFinished` handlers were skipped. Animation advancement now also fires `OnFinished` for the group's child animation frames when the group finishes, so XML-defined hide scripts run with the animation child as `self`.

Coverage:

- `achievement_frame_toggle_hides_visible_panel_tree` asserts the second achievement toggle calls `HideUIPanel(AchievementFrame)` and leaves no visible descendants.
- `tick_fires_child_animation_on_finished` asserts child animation `OnFinished` fires and `self:GetRegionParent()` resolves to the animated frame.

## Sources

- [workarounds.rs](../../../src/lua_api/workarounds.rs) — `ToggleAchievementFrame` workaround.
- [animations.rs](../../../src/lua_api/frame/methods/button_anchor_hierarchy/animations.rs) — animation advancement and completion dispatch.
- [achievement_panel_layout.rs](../../../tests/achievement_panel_layout.rs) — achievement panel regression.
- [animation_group.rs](../../../tests/animation_group.rs) — child animation completion regression.
- [Blizzard_AchievementUI.lua](../../../Interface/BlizzardUI/Blizzard_AchievementUI/Mainline/Blizzard_AchievementUI.lua) — Blizzard panel toggle implementation.
- [AlertFrameSystems.xml](../../../Interface/BlizzardUI/Blizzard_FrameXML/Mainline/AlertFrameSystems.xml) — achievement alert child animation hide script.

## See Also

- [[lua-api]] — frame methods and animation system context.
- [[event-system]] — script handler dispatch context.
- [[widget-system]] — frame visibility and ancestor visibility semantics.
