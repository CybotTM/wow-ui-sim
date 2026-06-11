# Journeys Renown Card Text Hidden (Keyed Anchor Fell Back to Parent)

The Adventure Guide Journeys tab's "Renowns" cards (`RenownCardButtonTemplate`, `Blizzard_Journeys.xml`) showed no faction name/level text — only letters peeking past each card's right edge. Reported as "text z-index ordering", but z-order was fine: the FontStrings rendered at the WRONG X. Their `relativeKey="$parent.IconFrame"` anchor had silently fallen back to the parent button, placing the text at `card_right + 5`, where the next column's card covered it.

## Content

### Symptoms

- Card plaque (`ui-journeys-renown-button` NormalTexture) and icon render; name/level text invisible.
- A few glyphs visible just past each card's right edge — the text exists, anchored to the card's RIGHT instead of the icon's RIGHT.
- Probe: `name:GetPoint(1)` returned `relTo == card` (should be `card.IconFrame`); `name:GetLeft() == card:GetRight() + 5`.

### Root cause

1. The XML loader processes `<Layers>` regions before child `<Frames>` regardless of document order (`create_layer_children` before `create_child_frames` in `src/loader/xml_frame/finalize.rs`). `RenownCardButtonTemplate` anchors a Layers FontString to `$parent.IconFrame`, a `<Frames>` child.
2. `SetPoint` resolved the key eagerly (`resolve_anchor_target_id`); on failure it silently fell back to the parent frame and **discarded the key**.
3. The registry already has a lazy mechanism — `resolve_named_anchor_targets_for_frame` reads `Anchor.relative_to` expressions and is invoked by the finalize pass *after* child frames exist — but SetPoint never stored the expression, so there was nothing to re-resolve.

### Fix (commit 7c0c0f987)

When eager resolution of a `$parent`-style key fails, `SetPoint` stores the expression via `Frame::set_point_with_name` (`Anchor.relative_to = Some(expr)`, `relative_to_id = None`). Layout falls back to parent-relative until the existing finalize pass resolves the key and reindexes anchor dependents. Regression test: `test_layer_region_anchored_to_child_frame_defined_after_layers` (`src/loader/tests/runtime_template_xml.rs`), verified to fail with the fix disabled.

### Diagnostic shortcut

When text/regions appear shifted to a parent edge instead of "behind" something, check `GetPoint()` before suspecting render order: a keyed anchor that fell back to the parent produces exactly this off-by-one-frame layout.

### Known remaining gap

`relativeKey` **without** a `$parent` prefix (a self-relative key path like `relativeKey="Icon"`) is still resolved as a global name, not as a path from the region itself. No Blizzard usage has surfaced yet; fix in `resolve_anchor_relative_expr` if one does.

## Sources

- `~/.cache/wow-ui-sim/blizzard-ui/Blizzard_EncounterJournal/Mainline/Blizzard_Journeys.xml` — RenownCardButtonTemplate
- `src/lua_api/frame/methods/button_anchor_hierarchy/{anchors,shared}.rs` — SetPoint pending-key path
- `src/widget/registry/anchor.rs` — `resolve_named_anchor_targets_for_frame`
- `src/loader/xml_frame/finalize.rs` — post-children resolution pass

## See Also

- `docs/anchor-resolution.md` — anchor resolution walkthrough
- `docs/button-text-rendering.md` — the genuine text-behind-texture bug family this was initially mistaken for
