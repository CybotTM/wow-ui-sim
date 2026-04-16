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

## Likely root cause

The unit-frame load path loses the shared frame metatable partway through
Blizzard UI bootstrap. See the cascade `Lua error: ... attempt to call
method 'SetForbidden' (a nil value)` starting at
`Blizzard_SharedXMLBase/CallbackRegistry.lua:25`.

The mismatch comes from `rilua_methods.rs::registry_set` storing the
frame metatable under `intern_string_static("__rilua_frame_mt")` while
`attach_frame_metatable` reads via `intern_string("__rilua_frame_mt")`.
These return different `GcRef<LuaString>` values whenever the static
pointer cache diverges from the content-keyed intern table, so new
frames silently get no metatable. Fix belongs in rilua (the static cache
needs to stay in sync with the content intern), not in wow-ui-sim — see
commit `8d28bf11` ("Reverted the migration pending a rilua-side repro").

## Verification plan

1. Land rilua fix so `intern_string_static` and `intern_string` return
   the same `GcRef` for matching content.
2. Re-enable the `intern_string_static` optimization in `registry_set`.
3. Run `cargo test --test party_frame_tree` — expect all three tests to
   pass (size, member offsets, decorative children).
4. Render check: `wow-sim screenshot` should show four stacked party
   member frames on the left side of the screen (Blizzard's left-edge
   group frame layout). Current branch renders nothing in that region.
