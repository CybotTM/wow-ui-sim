# PartyFrame tree: master reference vs rilua-migration regression

## Status

Startup hang is fixed. PartyFrame renders at `(120x244)` matching master.
1 of 3 regression tests passes (`party_frame_has_master_reference_shape`);
remaining failures are follow-up issues unrelated to the hang.

Test: [`tests/party_frame_tree.rs`](../../../tests/party_frame_tree.rs) —
three assertions pinned against the master dump at commit `322eba4a`.

## Reference dump (master, commit `322eba4a`)

```
PartyFrame          [Frame]  (120x244) visible LOW:2 x=22  y=147  alpha=1.00
  .Selection        [Frame]  (120x244) hidden  LOW:3 x=22  y=147  alpha=1.00
  .Background       [Frame]  (144x250) hidden  LOW:3 x=22  y=141  alpha=0.50
  .MemberFrame1     [Button] (120x53)  visible LOW:2 x=22  y=147  alpha=1.00
  .MemberFrame2     [Button] (120x53)  visible LOW:2 x=22  y=210  alpha=1.00
  .MemberFrame3     [Button] (120x53)  visible LOW:2 x=22  y=273  alpha=1.00
  .MemberFrame4     [Button] (120x53)  visible LOW:2 x=22  y=336  alpha=1.00
```

Collected with:
```bash
cd /syncthing/Sync/Projects/wow/wow-ui-sim   # master worktree
LD_LIBRARY_PATH=target/debug/deps \
  WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 \
  ./target/debug/wow-sim dump-tree --filter-key PartyFrame
```

## Resolved issues

### 1. `intern_string_static` / `intern_string` ref mismatch (fixed)

The unit-frame load path lost the shared frame metatable partway through
Blizzard UI bootstrap. `registry_set` stored the metatable under
`intern_string_static("__rilua_frame_mt")` while `attach_frame_metatable`
read via `intern_string("__rilua_frame_mt")`; those returned different
`GcRef<LuaString>` values whenever the static pointer cache diverged
from the content-keyed intern table. Fixed upstream in the rilua static-
intern landing.

### 2. Missing group-state globals (fixed)

`UnitExists("partyN")`, `GetNumGroupMembers`, `GetNumSubgroupMembers`,
`IsInGroup`, `UnitName`, `UnitClass`, `UnitLevel` had no real
implementations. `PartyFrame:ShouldShow()` returned `nil` so the members
never rendered. Backed by `SimState::party_members` in
`src/lua_api/globals/rilua_group_queries.rs`. Additionally
`A_Admin.SetPartySize(N)` pushes a synthetic `GROUP_ROSTER_UPDATE` event.

### 3. `SetParentKey` stub (fixed)

`src/lua_api/frame/methods/rilua_button_anchor_hierarchy/hierarchy.rs::set_parent_key`
now calls `sync_child_to_rilua(parent_id, key, child_id)` so Lua-side
`parent.key == child` resolves. A short-circuit in `sync_child_to_rilua`
avoids redundant writes when `parent[key]` already points at the child —
important because `PartyFrame:InitializePartyMemberFrames` re-runs on
every `OnShow` pass.

### 4. `C_UnitAuras.GetAuraSlots` infinite loop (fixed)

`AuraUtil.ForEachAura` drives its iteration via a `repeat ... until
continuationToken == nil` loop (Blizzard_FrameXMLUtil/AuraUtil.lua:114-117).
The first return value of `GetAuraSlots` becomes `continuationToken`.
Our stub returned an empty table, which is truthy in Lua, so the loop
never terminated — hanging `EditModeManagerFrame:UpdateSystems()` at
`TargetFrame:UpdateSystem -> UpdateSystemSettingBuffsOnTop -> UpdateAuras
-> ParseAllAuras`.

Fix: `stub_nil` in `src/lua_api/globals/rilua_stubs/namespace_stubs.rs`
so the continuation token is `nil` and the loop exits on first iteration.

### Bisection that found it

Added per-call timestamp prints through `apply_post_load_workarounds →
workarounds::apply → init_edit_mode_layout → apply_system_anchors`, then
manually iterated `emm.registeredSystemFrames` calling each frame's
`UpdateSystem` one by one. Frame 29 (`PartyFrame`) completed. Frame 30
(`TargetFrame`) hung. Bisected `EditModeSystemMixin:UpdateSystem`'s body
down to the settings loop where setting id 2 (`BuffsOnTop`) hangs in
`ParseAllAuras`.

## Open follow-ups (post-hang)

### A. `PartyFrame.Selection:GetWidth()` returns 4, not 120

The Selection child has explicit `TOPLEFT/BOTTOMRIGHT -> PartyFrame` so
its resolved rect matches the parent (`120x244`). But `GetWidth()`
returns the *stored* width (the explicit size set via `SetWidth/SetSize`)
which is 4. Master returns 120. Either something on master forces a
stored size on Selection, or master's GetWidth returns the anchor-
resolved rect when no explicit size was set.

Affects `party_frame_has_background_and_selection_children`.

### B. MemberFrame y offsets have inverted sign

Test computes `MF2:GetTop() - MF1:GetTop()` and expects +63 (master
dump values 147, 210, 273, 336 — visually top-down increasing). WoW
coordinate Y is bottom-up though, so MF2 below MF1 yields a negative
delta. Either the test's `expected_y` table is in dump-coordinate space
and the test should match +63 by taking the absolute, or master's
`GetTop` returned dump-space values too.

Affects `party_frame_member_frames_render_at_master_offsets`.

## Verification

```bash
cargo test --test party_frame_tree -- --test-threads=1
# Expected: 1 passed; 2 failed; 31s (no timeouts).
```

`party_frame_has_master_reference_shape` passes (120x244 visible at
x=22). The other two tests fail but no longer time out at 120s.
