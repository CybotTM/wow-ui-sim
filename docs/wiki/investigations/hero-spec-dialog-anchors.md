# Hero Spec Dialog Anchors

`HeroTalentsSelectionDialog`'s LIGHTSMITH/TEMPLAR panels rendered with the
spec icon, description, and talent node tree collapsed to the spec frame
edge. Talent node icons appeared at the bottom of the screen instead of
inside their panels. Two independent bugs in anchor resolution timing.

## Symptoms

Inspector dump (before fix) for `HeroTalentSpecContentTemplate` instances:

```
specFrame: top=1109 bottom=300
SpecName:        top=1070 bottom=1041
SpecImage:       top=298  bottom=136   (anchored to specFrame, not SpecName)
NodesContainer:  top=247  bottom=-53   (anchored to specFrame, not Description)
```

After fix:

```
SpecName:        top=1070 bottom=1041
SpecImage:       top=1013 bottom=851   (TOP -> SpecName.BOTTOM)
Description:     top=820  bottom=740   (TOP -> SpecImage.BOTTOM)
NodesContainer:  top=687  bottom=387   (TOP -> Description.BOTTOM)
```

## XML Layout

`Blizzard_HeroTalentsSelectionDialog.xml` defines layers + frames in this
document order:

```
<Layers>
  <FontString parentKey="SpecName"/>
  <Texture    parentKey="SpecImage"    relativeKey="$parent.SpecName"/>
  <FontString parentKey="Description"  relativeKey="$parent.SpecImage"/>
  ...
</Layers>
<Frames>
  <Frame parentKey="NodesContainer" relativeKey="$parent.Description"/>
  ...
</Frames>
```

`relativeKey` is resolved at `SetPoint` time by reading the sibling from
the parent's `children_keys` table. The sibling must be created first.

## Bug 1: Layer Element Reordering

`xml_layer_batch.rs` collected all textures, then all fontstrings, into
the same Lua batch. SpecImage's `SetPoint("TOP", parent["SpecName"], ...)`
ran before SpecName existed; the lookup returned `nil` and the anchor
silently fell back to the parent spec frame.

**Fix**: iterate `layer.elements` directly so textures and fontstrings
are emitted in XML document order.

## Bug 2: Runtime Template Anchor Resolution

The loader path (`finalize_frame`) calls `resolve_named_anchor_targets_for_frame`
on each child after both child frames and layer children are created, so
unresolved anchors (stored as relative-to strings via `set_point_with_name`)
get re-bound once their target sibling exists.

The runtime template path
(`apply_runtime_template_chain_impl`) had no such pass. Spec frames are
runtime-instantiated, so NodesContainer's `$parent.Description` anchor
stayed as an unresolved string and rendered against the spec frame.

**Fix**: add `resolve_runtime_template_named_anchors` and call it from
`finalize_template_frame` and `finalize_runtime_template_child` after
layers and child frames are both in place.

## Verification

`--exec-lua` script dumps `dlg.specFramesBySubTreeID[*]` geometry. After
the fixes:

- SpecImage anchors to SpecName (1013-851)
- Description anchors to SpecImage (820-740)
- NodesContainer anchors to Description (687-387)
- Talent node icons render inside their LIGHTSMITH/TEMPLAR panels

## Sources

- `src/loader/xml_layer_batch.rs` — layer element ordering fix
- `src/lua_api/globals/create_frame/template_chain.rs` — runtime anchor re-resolution
- `src/lua_api/globals/create_frame/template_chain/runtime.rs` — same fix on nested children
- `Interface/BlizzardUI/Blizzard_PlayerSpells/ClassTalents/Blizzard_HeroTalentsSelectionDialog.xml`

## See Also

- [[hero-spec-icon-bug]] — separate hero spec icon position investigation
- [[explicit-xml-parent-anchors]] — related parent-anchor resolution work
