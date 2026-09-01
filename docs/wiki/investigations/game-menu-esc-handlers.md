# Game Menu Esc Handlers

`RegisterGameMenuEscHandler` and `GameMenuEscPriority` were missing, so twenty-five Blizzard files raised at file scope and silently lost every definition below that line — including the edit-mode manager's `RegisterSystemFrame`, which left every `EditModeSystemTemplate` frame without a completed `OnLoad`.

## Content

Symptoms:

- `ChatFrame1Background` rendered as an opaque white box. Real WoW draws it black at 25% alpha.
- `GetChatWindowInfo(1)` reported `r=0 g=0 b=0 alpha=0.25`, but the texture carried `GetAlpha() == 1` and `GetVertexColor() == 1,1,1,1`.
- `ChatFrame1:IsEventRegistered("UPDATE_CHAT_WINDOWS")` was `false` and `CHAT_FRAMES` did not contain `ChatFrame1`, so `FloatingChatFrame_Update` never ran and never applied the window colour or alpha.
- `EditModeManagerFrameMixin` held 4 methods. Blizzard's `EditModeManager.lua` defines 138.

Root cause:

`RegisterGameMenuEscHandler(priority, handler)` and the `GameMenuEscPriority` table are client-side globals. Nothing in the Blizzard UI defines them and `Blizzard_APIDocumentationGenerated` carries no entry, so the simulator had no source to pick them up from. All twenty-five call sites across the Blizzard UI sit at **file scope**, so each file raised at that line and lost everything below it.

`Blizzard_EditMode/Shared/EditModeManager.lua` calls it on line 21, two lines before the first `EditModeManagerFrameMixin` method. The mixin therefore kept only the four methods that `EditModeManagerOverrides.lua` adds later, and `EditModeManagerFrame:RegisterSystemFrame` did not exist. `EditModeSystemMixin:OnSystemLoad` calls it on line 21 of `EditModeSystemTemplates.lua`, so every frame inheriting `EditModeSystemTemplate` aborted its `OnLoad` there. For `ChatFrame1` that is `PrimaryChatFrameMixin:OnLoad`, whose very first statement is `EditModeSystemMixin.OnSystemLoad(self)` — the event registration on line 202 was never reached.

Clearing that abort exposed two further gaps in the same chain, both in code that had been unreachable:

- `Enum.EditModeLossOfControlSetting` was absent entirely, and `Enum.EditModeSystem` was missing `RaidWarning` and `LossOfControl`. `EditModeSystem` also numbered `TotemActionBar` as 24; `EditModeManagerConstantsDocumentation.lua` gives `RaidWarning = 24`, `TotemActionBar = 25`, `LossOfControl = 26`. `EditModeSettingDisplayInfo.lua` uses those members as table keys, so it raised `table index is nil` at file scope and never defined `GetSystemSettingDisplayInfoMap`.
- `Enum.DamageMeterVisibility` was missing `InGroup = 3`, used as a key in the same table.

Fix:

- `runtime_surface_bootstrap.lua` defines `GameMenuEscPriority` and a `RegisterGameMenuEscHandler` that stores handlers with their priority and registration order. Blizzard documents no numeric values for the priorities, so the relative order is modelled from the member names and their call sites and the numbers themselves are arbitrary.
- `key_dispatch.rs` consults the registered handlers between the focused EditBox's `OnEscapePressed` and the ESCAPE keybinding, matching the order the client uses before it reaches `ToggleGameMenu`. A handler returning true consumes the press.
- `enum_data/edit_mode.rs` adds `EditModeLossOfControlSetting`, inserts `RaidWarning` and appends `LossOfControl` to `EditModeSystem` (which corrects `TotemActionBar` from 24 to 25), and adds `DamageMeterVisibility.InGroup`. `missing_enums.lua` updates the matching `*Meta` counts.

`src/ptr/compat_bootstrap.lua` still appends `RaidWarning` for the 12.1.0 profile. `__wow_fill_enum` skips keys that already exist, so that line is now a no-op and `test_patch_12_1_cvars_and_enums_exist` keeps passing.

Measured on the `client-ptr` profile with `--no-addons --no-saved-vars`:

| | before | after |
| --- | --- | --- |
| startup Lua errors | 659 | 475 |
| distinct error messages | 56 | 39 |
| `EditModeManagerFrameMixin` methods | 4 | 138 |
| `ChatFrame1Background` alpha / vertex | 1.0 / 1,1,1 | 0.25 / 0,0,0 |

Ten of the remaining messages are new, all in code that the abort had kept unreachable: `ManagedFrameMixin` and `CreateSecureAuraInstanceMap` are absent from the Blizzard UI cache, and `ArenaUtil` and `PartyMemberFrame`'s `ProcessAura` fail for unrelated reasons. Those are separate gaps this change exposes rather than causes.

## Sources

- [runtime_surface_bootstrap.lua](../../src/lua_api/env_init/runtime_surface_bootstrap.lua) — `GameMenuEscPriority` and the handler registry
- [key_dispatch.rs](../../src/lua_api/key_dispatch.rs) — Escape dispatch order
- [edit_mode.rs](../../src/lua_api/globals/enum_data/edit_mode.rs) — edit-mode enum definitions
- [missing_enums.lua](../../src/lua_api/globals/enum_data/missing_enums.lua) — `*Meta` counts

## See Also

- [[addon-loading]] — file-scope errors abort the rest of a Lua file
- [[chatframe-scrollbar-anchor-reapply]] — another chat frame layout defect
