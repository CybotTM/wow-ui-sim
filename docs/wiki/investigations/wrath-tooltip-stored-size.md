# Wrath Tooltip Stored Size Versus Parent-Fill Geometry

Wrath tooltip texture tests must distinguish stored dimensions from anchor-derived geometry. A queried-size oracle incorrectly diagnosed the XML loader; the loader change was reverted. The [owning contract](../../specs/wrath-widget-labels.md) records reconstructed 12340 authority and the separate runtime limits.

## Content

### The false oracle

The stock GameTooltip XML slots declare 12x12. A simulator diagnostic against `ccfbb8d42` observed:

| Stage | Stored size | Active points | Queried size |
|---|---|---|---|
| Original XML slot, zero-size tooltip parent | 12x12 | Parent TOPLEFT/BOTTOMRIGHT | 0x0 |
| After explicit SetSize(18,7) | 18x7 | Same parent corners | 0x0 |
| After AddTexture | 18x7 | TOPRIGHT to the left text's TOPLEFT | 18x7 |

Comparing the first queried size with the final queried size does not test dimension retention. The stored size was never lost. Likewise, comparing an attached slot's effective width with an unused parent-filled slot's effective width is not a valid template-size oracle.

### Native anchoring boundary

The laptop's independently source-verified 12340 reconstruction confirms XML PostLoadXML parent fill when no active point exists, regardless of Size. AddTexture replaces those points with a single text-relative point; the stored dimensions then supply the size. This is reconstruction evidence, not native execution. Do not extrapolate it to Lua CreateTexture, FontStrings or current Retail. Empty/inherited-anchor combinations are not covered by this batch's simulator fixtures.

The corrected original-consumer tests inspect stored Frame dimensions, query the two parent corners, and then query the single attachment point and effective dimensions. Those corrected controls passed against the unchanged pre-loader-fix library: no XML-loader or point-deactivation product fix was required. The unsupported loader change was fully reverted before further acceptance work.

### A separate cached-layout failure

Replacing `Frame.anchors` also requires updating the registry's reverse anchor index. Otherwise a later caption move can leave its sibling texture's cached rectangle unchanged despite the correct single active point. This is distinct from stored-size retention and XML defaults; the attachment producer now removes old dependencies and registers the text target before marking layout dirty.

## Sources

- [Wrath widget contracts](../../specs/wrath-widget-labels.md) — clean-room report identities, scope and proof limits
- [Texture loader](../../../src/loader/xml_texture.rs) — XML anchor generation
- [Resolved-size queries](../../../src/widget/registry/size_changes.rs) — stored versus resolved-size projection
- [Wrath attachment producer](../../../src/lua_api/frame/methods/widgets/tooltip/wrath.rs) — slot state and anchor dependencies
- [Behavioral fixtures](../../../tests/widgets_b6_u1.rs) — original LFR continuation, stored size and cached movement

## See Also

- [[widget-system]] — Frame storage and registry ownership
- [[layout-system]] — anchor-derived rectangles
- [[xml-template-system]] — template loading boundaries
