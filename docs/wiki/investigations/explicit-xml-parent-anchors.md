# Explicit XML Parent Anchors

Nested XML frames with their own `parent="..."` attribute must use that explicit parent for both parenting and implicit anchor targets. The visible symptom was `PaperDollSidebarTabs` rendering too high above the CharacterFrame because its no-`relativeTo` anchor resolved to the containing `PaperDollFrame` instead of `CharacterFrameInsetRight`.

## Content

Blizzard defines `PaperDollSidebarTabs` inside `PaperDollFrame`'s `<Frames>` block, but the frame also declares `parent="CharacterFrameInsetRight"`. Its anchor has no `relativeTo`, so WoW resolves it against the actual parent:

`BOTTOMRIGHT -> CharacterFrameInsetRight:TOPRIGHT offset(-6,-1)`.

The loader previously preferred the recursive containing-frame override over the child XML `parent` attribute in `prepare_frame_creation()`. That created `PaperDollSidebarTabs` as a child of `PaperDollFrame`, moving the tab row into the top title area. The fix gives `frame.parent` precedence over the recursive parent override when choosing the frame parent, `$parent` name-substitution parent, and anonymous subst parent.

Regression coverage lives in `test_nested_xml_frame_parent_attribute_overrides_containing_frame`, which creates the same shape: a nested child with `parent="ExplicitParent"` and an implicit-parent `SetPoint`.

## Sources

- [xml-template-system](../systems/xml-template-system.md) — XML frame creation pipeline context
- [layout-system](../systems/layout-system.md) — implicit-parent anchor resolution context

## See Also

- [[xml-template-system]] — XML loading and template conversion
- [[layout-system]] — anchor resolution behavior
- [[chatframe-scrollbar-anchor-reapply]] — another `$parent`/anchor substitution issue
