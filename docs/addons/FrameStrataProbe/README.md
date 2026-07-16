# FrameStrataProbe

Captures retail behavior for three questions:

1. What effective strata and fixed flag does explicit XML `frameStrata="PARENT"` produce?
2. Does parent `SetFrameStrata()` overwrite descendants with `PARENT`, default, and explicit `MEDIUM` strata?
3. Does `SetParent()` reset those same three child categories when moving from a `HIGH` parent to a `LOW` parent?

The groups are separate so the `SetFrameStrata()` cascade cannot affect the later `SetParent()` observations. Each group includes grandchildren to test recursive subtree behavior. A base-only template control distinguishes normal literal inheritance from a derived template's `PARENT` override.

## Usage

The committed TOC targets retail `12.0.7` (`## Interface: 120007`). If probing a `12.0.5` client, change that line to `## Interface: 120005` before installation.

1. Copy the `FrameStrataProbe` directory to the target client's `Interface/AddOns/` directory and enable it.
2. Log in on any character; the probe captures automatically at `PLAYER_LOGIN`.
3. `/reload` or log out so WoW flushes the SavedVariables file.
4. Inspect:

   ```text
   WTF/Account/<ACCOUNT>/SavedVariables/FrameStrataProbe.lua
   ```

The addon prints only a capture confirmation at `PLAYER_LOGIN`; the observations are in `FrameStrataProbeDB`.

Each frame snapshot contains `name`, `parentName`, `frameStrata`, and `hasFixedFrameStrata`. The top-level record also includes `addonName`, `build`, and `capturedAt`.

## Results to compare

### XML `PARENT`

- `cascadeBefore.parentChild.frameStrata`: effective strata at creation; expected parent value is `HIGH`.
- `cascadeBefore.parentChild.hasFixedFrameStrata`: whether explicit `PARENT` is classified as fixed.
- Compare with `cascadeBefore.defaultChild` and `cascadeBefore.fixedChild` to distinguish inherited, default, and explicit/fixed children.
- `templateBase.frameStrata`: control value; should remain the base template's literal `HIGH` despite its `LOW` parent.
- `templateParent.frameStrata`: should be the actual parent's `LOW` if derived `PARENT` overrides the base template.
- Compare `templateBase` with `templateParent`; this prevents a false conclusion if template strata inheritance is ignored entirely.
- Their `hasFixedFrameStrata` fields show how literal and derived-`PARENT` results are classified.

### Parent `SetFrameStrata()` cascade

Compare `cascadeBefore` with `cascadeAfterParentSetLow`:

- Does the `PARENT` child become `LOW`?
- Does the default child become `LOW`?
- Does the explicit/fixed `MEDIUM` child also become `LOW`?
- Do `parentGrandchild` and fixed `MEDIUM` `fixedGrandchild` also become `LOW`?
- Do any `hasFixedFrameStrata` flags change?

### `SetParent()` reset

Compare `reparentBefore` with `reparentAfterSetParentLow`:

- Does the `PARENT` child become `LOW`?
- Does the default child become `LOW`?
- Does the explicit/fixed `MEDIUM` child remain `MEDIUM` or reset to `LOW`?
- What happens to `parentGrandchild` and fixed `MEDIUM` `fixedGrandchild` when their ancestor is reparented?
- Do any `hasFixedFrameStrata` flags change?

The fixed-child result distinguishes unconditional frame reset from non-fixed recomputation. The grandchildren show whether reparenting also propagates through the moved subtree.
