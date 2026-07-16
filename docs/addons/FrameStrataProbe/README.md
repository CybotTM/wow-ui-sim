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

## Retail capture

Captured on retail `12.0.7` build `68453` at `2026-07-16T01:21:08`.

### Direct XML `PARENT` effective result

During the XML children's `OnLoad`:

- The actual parent reported `DIALOG`.
- The `PARENT` child reported `DIALOG`.
- The explicit literal child reported `LOW` while its parent reported `DIALOG`.
- All tested frames reported `HasFixedFrameStrata() == false`.

`creationAtPlayerLogin` reported the same effective values before this probe performed any mutations. `GetFrameStrata()` did not return the raw string `PARENT`; equality with the parent does not identify why the values match or establish a persistent symbolic binding.

### Template comparison values

- The actual parent reported `DIALOG`.
- The base `HIGH` template instance reported `HIGH`.
- The base `HIGH` plus derived literal `LOW` template instance reported `LOW`.
- The base `HIGH` plus derived `PARENT` template instance reported `HIGH`.
- All tested template instances reported `HasFixedFrameStrata() == false`.

These values distinguish the observed results but do not expose the client's internal template-resolution mechanism.

### Before and after parent `SetFrameStrata()`

Before the operation, the parent, default child, `PARENT` child, and its grandchild reported `HIGH`; the explicit XML child and grandchild reported `MEDIUM`. After the parent was set to `LOW`, every tested frame reported `LOW`, including both explicit XML `MEDIUM` frames. All reported `HasFixedFrameStrata() == false`.

### Before and after `SetParent()`

Before reparenting, the tested children and grandchildren reported either their fixture's `HIGH` or explicit XML `MEDIUM` values. After moving the direct children to the `LOW` parent, every moved child and tested descendant reported `LOW`, including both explicit XML `MEDIUM` frames. All reported `HasFixedFrameStrata() == false`.

The XML probe does not call `SetFixedFrameStrata(true)`; runtime-fixed behavior remains unproven. The snapshots establish effective before/after values, not an internal propagation mechanism.
