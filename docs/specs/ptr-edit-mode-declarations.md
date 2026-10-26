# PTR Edit Mode declaration families

PTR publishes declaration-backed Edit Mode enums through `src/c_api/ptr_edit_mode_enums.rs`. The [investigation](../wiki/investigations/ptr-edit-mode-declarations.md) explains the original consumer boundaries and initialization limits.

## What it must do

- [x] Publish PTR `Enum.BossFrameGrowDirection` as Left=0, Center=1, Right=2, with NumValues=3, MinValue=0, MaxValue=2 metadata.
- [x] Preserve those three distinct values through the original boss-growth dropdown constructor.
- [x] Publish PTR `Enum.EditModePresetLayouts` as Modern=0, Classic=1, Gamepad=2, with NumValues=3, MinValue=0, MaxValue=2 metadata.
- [x] Let the original preset manager index supplied Modern, Classic and Gamepad maps separately, preserve Gamepad system data and return independent anchor copies.

- [x] Publish PTR `Enum.EditModeSwingTimerSetting` as Scale=0, Opacity=1, Visibility=2, Width=3, Height=4, ShowBarTitle=5, ShowTime=6, with NumValues=7, MinValue=0, MaxValue=6 metadata.
- [x] Publish PTR `Enum.EditModeSwingTimerVisibility` as Always=0, InCombat=1, Hidden=2, with NumValues=3, MinValue=0, MaxValue=2 metadata.
- [x] Preserve all seven setting identifiers and three visibility choices through the complete original SwingTimer constructor.

## Complete PTR display-info initialization

- Extend existing enum tables without replacing their earlier members, only when the active profile is PTR.
- Publish the pinned members read by the complete original display-info file: system MainActionBarEndCap=26, GroupFinder=27, LossOfControl=28, SwingTimer=29 and GamepadMainActionBar=30; Gamepad settings Compact=0, ShowEmptyBars=1, ShowButtonPrompts=2, ShowHighlight=3 and ShouldExpand=4; GroupFinder Size=0; EndCap Hidden=0; PersonalResourceDisplay AllowMouseoverThrough=15; UnitFrame DebuffIconSize=19, UseHorizontalLayout=23 and GrowDirection=24.
- The unchanged whole `EditModeSettingDisplayInfo.lua` must complete and publish `GetSystemSettingDisplayInfoMap`, with the original per-system keys and setting values.
- Do not publish unread account settings or metadata for this boundary. Stop at a non-declaration failure rather than replacing Blizzard functions or strings.

## How it works

- [Declaration and consumer evidence](../wiki/investigations/ptr-edit-mode-declarations.md).

## Implementation inventory

- `src/c_api/ptr_edit_mode_enums.rs`: declaration-backed tables and metadata.
- `src/c_api/mod.rs`: unconditional module declaration, compiled on every profile.
- `src/lua_api/env_init/enums.rs`: unconditional hook after generated and compatibility enum initialization; the module returns without changes unless `ACTIVE == ClientProfile::Ptr`.

## Tests asserting this spec

- `tests/ptr_edit_mode_enums.rs`: original selected dropdowns, complete SwingTimer settings constructor and independently reached original preset manager.

## Known gaps (current cycle)

- [ ] Complete display-info proof and downstream LossOfControl initialization proof are pending the declaration producer's GREEN execution. Native preset defaults and other Edit Mode lifecycle behavior remain outside this contract.

## Out of scope

Other declaration families, GraphicsOverrides and its preset, GameRules, replacement Edit Mode Lua, frame-method patches, and native preset defaults. Other profiles retain their existing registration paths.
