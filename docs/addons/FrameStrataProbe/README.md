# FrameStrataProbe

Captures retail behavior for four questions:

1. What effective strata does direct XML `frameStrata="PARENT"` expose during the child's `OnLoad`?
2. What effective results do base and derived templates produce for literal `LOW` versus special `PARENT`?
3. How do tested frame values differ before and after parent `SetFrameStrata("LOW")`?
4. How do tested frame values differ before and after moving frames from a `HIGH` parent to a `LOW` parent with `SetParent()`?

The XML children record `OnLoad` values before Lua-file execution. The parent-set and reparent groups use separate fixtures. Each operation group records direct children and grandchildren before and after the operation.

## Usage

The committed TOC targets retail `12.0.7` (`## Interface: 120007`). If probing a `12.0.5` client, change that line to `## Interface: 120005` before installation.

1. Copy the `FrameStrataProbe` directory to the target client's `Interface/AddOns/` directory and enable it.
2. Log in on any character; XML child values are recorded during `OnLoad`, and the complete database is assembled at `PLAYER_LOGIN`.
3. `/reload` or log out so WoW flushes the SavedVariables file.
4. Inspect:

   ```text
   WTF/Account/<ACCOUNT>/SavedVariables/FrameStrataProbe.lua
   ```

The addon prints only a capture confirmation at `PLAYER_LOGIN`; the observations are in `FrameStrataProbeDB`.

Each frame snapshot contains `name`, `parentName`, and `frameStrata`; `hasFixedFrameStrata` is included when the API returns a value. The top-level record also includes `addonName`, `build`, and `capturedAt`.

## Results to compare

### Direct XML `PARENT` effective result

Inspect `creationOnLoad` first:

- `parentChild.parentFrameStrata` records the actual parent's value during the `PARENT` child's `OnLoad`.
- `parentChild.frameStrata` records the `PARENT` child's effective value during `OnLoad`.
- `literalChild.parentFrameStrata` and `literalChild.frameStrata` provide the explicit `LOW` control.
- `creationAtPlayerLogin` records the same frames again before this probe performs any mutations.
- No result can return the raw string `PARENT`; `GetFrameStrata()` exposes only effective strata. Equality with the parent does not identify why the values match.
- Even a matching creation-time result would not establish a persistent symbolic `PARENT` binding; later before/after observations are recorded separately below.

### Template control values

The fixtures expose three distinct comparison values: `HIGH`, `LOW`, and `DIALOG`.

- `templateBase` declares base `HIGH`.
- `templateDerivedLow` declares base `HIGH` plus derived literal `LOW`. Compare its effective result for equality with `HIGH`, `LOW`, and `DIALOG`.
- `templateParent` declares base `HIGH` plus derived `PARENT`. Compare its effective result for equality with `templateBase`, `templateDerivedLow`, and `templateActualParent`. The probe records equality only; it does not expose the client's internal template-resolution mechanism.

The XML probe does not call `SetFixedFrameStrata(true)`; runtime-fixed behavior remains unproven.

### Before and after parent `SetFrameStrata()`

Compare `parentSetBefore` with `parentSetAfterLow`:

- Does the `PARENT` child become `LOW`?
- Does the default child become `LOW`?
- Does the explicit XML `MEDIUM` child also become `LOW`?
- Do `parentGrandchild` and the explicit XML `MEDIUM` `explicitGrandchild` also become `LOW`?
- Do any `hasFixedFrameStrata` flags change?

### Before and after `SetParent()`

Compare `reparentBefore` with `reparentAfterSetParentLow`:

- Does the `PARENT` child become `LOW`?
- Does the default child become `LOW`?
- Is the explicit XML `MEDIUM` child's later value `MEDIUM`, `LOW`, or something else?
- What happens to `parentGrandchild` and the explicit XML `MEDIUM` `explicitGrandchild` when their ancestor is reparented?
- Do any `hasFixedFrameStrata` flags change?

Use each snapshot's `hasFixedFrameStrata` value to determine fixed state. Compare direct-child and grandchild values before and after each operation without inferring an internal propagation mechanism.
