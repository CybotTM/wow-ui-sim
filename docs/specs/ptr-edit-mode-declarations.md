# PTR Edit Mode declaration families

PTR publishes two finite Edit Mode enum families through `src/c_api/ptr_edit_mode_enums.rs`. The [investigation](../wiki/investigations/ptr-edit-mode-declarations.md) explains the original consumer boundaries and initialization limits.

## What it must do

- [x] Publish PTR `Enum.BossFrameGrowDirection` as Left=0, Center=1, Right=2, with NumValues=3, MinValue=0, MaxValue=2 metadata.
- [x] Preserve those three distinct values through the original boss-growth dropdown constructor.
- [x] Publish PTR `Enum.EditModePresetLayouts` as Modern=0, Classic=1, Gamepad=2, with NumValues=3, MinValue=0, MaxValue=2 metadata.
- [x] Let the original preset manager index supplied Modern, Classic and Gamepad maps separately, preserve Gamepad system data and return independent anchor copies.

## How it works

- [Declaration and consumer evidence](../wiki/investigations/ptr-edit-mode-declarations.md).

## Implementation inventory

- `src/c_api/ptr_edit_mode_enums.rs`: declaration-backed tables and metadata.
- `src/c_api/mod.rs`: unconditional module declaration, compiled on every profile.
- `src/lua_api/env_init/enums.rs`: unconditional hook after generated and compatibility enum initialization; the module returns without changes unless `ACTIVE == ClientProfile::Ptr`.

## Tests asserting this spec

- `tests/ptr_edit_mode_enums.rs`: original selected dropdown and independently reached original preset manager.

## Known gaps (current cycle)

- [ ] Complete Edit Mode initialization remains blocked by other declaration families. This slice does not establish availability of `GetSystemSettingDisplayInfoMap`.

## Out of scope

Other declaration families, GraphicsOverrides and its preset, GameRules, replacement Edit Mode Lua, frame-method patches, and native preset defaults. Other profiles retain their existing registration paths.
