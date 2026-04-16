# PartyFrame tree: master reference vs rilua-migration regression

## Status
**rilua-migration regression** — PartyFrame renders as `(4x2)` at
`(22, 147)` with zero member frames. On `master` at commit `322eba4a` the
same environment produces a fully-sized `(120x244)` frame with four
visible `MemberFrame1..4` children.

Test: [`tests/party_frame_tree.rs`](../../../tests/party_frame_tree.rs) —
three assertions pinned against the master dump. Currently fails on this
branch and will start passing once the underlying bug is fixed.

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

Full 2316-line dump saved at `/tmp/claude/master_partyframe.txt` during
the capture session.

## Current branch output

```
PartyFrame          [Frame]  (4x2) visible LOW:2 x=22  y=147  alpha=1.00
  (no member frames, no decorative children)
```

## Contributing causes (in order discovered)

### 1. intern_string_static / intern_string ref mismatch (fixed)

The unit-frame load path lost the shared frame metatable partway through
Blizzard UI bootstrap. `registry_set` stored the metatable under
`intern_string_static("__rilua_frame_mt")` while `attach_frame_metatable`
read via `intern_string("__rilua_frame_mt")`; those returned different
`GcRef<LuaString>` values whenever the static pointer cache diverged
from the content-keyed intern table, so new frames silently got no
metatable. Landed in the rilua static-intern fix — tests 1 and 3
(shape, decorative children) now pass.

### 2. Missing group-state globals (fixed)

`UnitExists("partyN")`, `GetNumGroupMembers`, `GetNumSubgroupMembers`,
`IsInGroup`, `UnitName`, `UnitClass`, `UnitLevel` had no real
implementations — env_init.rs only covered `UnitExists` as a
player-only stub. `PartyFrame:ShouldShow()` returned `nil` so the
members never rendered even with the metatable wired up.

Fix: `src/lua_api/globals/rilua_group_queries.rs` now backs these
globals by `SimState::party_members`. Additionally
`A_Admin.SetPartySize(N)` pushes a synthetic `GROUP_ROSTER_UPDATE`
event so `PartyFrameMixin:OnEvent` runs its `Layout()` pass after a
test bumps the group size.

### 3. SetParentKey is a no-op (open)

`src/lua_api/frame/methods/rilua_button_anchor_hierarchy.rs::set_parent_key`
reads the stack args, extracts the parent id, then throws the result
away — it never does `parent_table[key] = child_val`. As a result
`PartyFrameMixin:InitializePartyMemberFrames`'s
`memberFrame:SetParentKey("MemberFrame1")` calls are silent no-ops,
and `PartyFrame.MemberFrame1 == nil` in Lua even though the pool has
four active frames.

Attempting a direct fix (wire up via `sync_child_to_rilua` +
`children_keys.insert`) made the Blizzard UI load hang indefinitely —
all three tests timed out at 120s. Suspect a downstream layout or
`Setup()` / `UpdateAuras` recursion triggered by `MemberFrame1..4`
becoming addressable for the first time. Needs bisection:

1. Try sync_child_to_rilua alone (no `children_keys.insert`) — isolates
   Lua-side impact from Rust-side layout-dirty propagation.
2. Try `children_keys.insert` alone (no Lua sync) — conversely.
3. If both hang, add timing logs in `PartyFrameMixin:Layout` and
   `PartyMemberFrameMixin:Setup` to see which Blizzard path loops once
   the key becomes visible.

## Verification plan

1. Resolve the SetParentKey hang per the bisection above.
2. Run `cargo test --test party_frame_tree -- --test-threads=1` — expect
   all three tests to pass.
3. Render check: `wow-sim screenshot` should show four stacked party
   member frames on the left side of the screen, matching the master
   reference screenshot saved during the initial investigation.
