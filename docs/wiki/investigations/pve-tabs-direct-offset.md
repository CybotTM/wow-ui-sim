# PVE Tabs Direct Offset

The Dungeons & Raids bottom tabs were anchored with the wrong base offset because Blizzard XML used direct `<Offset x="..." y="..."/>` attributes, while the simulator only read nested `<AbsDimension>` offsets.

## Content

`PVEFrameTab1` in `Blizzard_GroupFinder/Mainline/PVEFrame.xml` anchors to `PVEFrame:BOTTOMLEFT` with:

```xml
<Offset x="19" y="-30"/>
```

The simulator deserialized `OffsetXml` with only an optional `AbsDimension`, so direct `x` and `y` attributes were dropped. Both initial XML loading and direct runtime template creation shared that blind spot through their anchor-offset helpers.

The fix adds direct `x`/`y` fields to `OffsetXml` and makes anchor-offset extraction use nested `AbsDimension` when present, falling back to direct offset attributes otherwise. Regression coverage now checks the XML loader path and runtime template path.

Verification dump after the fix shows:

```text
PVEFrameTab1 ... x=35, y=542
  [anchor] BOTTOMLEFT -> PVEFrame:BOTTOMLEFT offset(19,-30) -> (35,574)
```

## Sources

- [PVEFrame.xml](../../../Interface/BlizzardUI/Blizzard_GroupFinder/Mainline/PVEFrame.xml) — `PVEFrameTab1` direct offset anchor
- [types_support.rs](../../../src/xml/types_support.rs) — XML offset deserialization
- [helpers.rs](../../../src/loader/helpers.rs) — XML loader anchor offset extraction
- [direct.rs](../../../src/lua_api/globals/template/direct.rs) — runtime template anchor offset extraction

## See Also

- [[layout-system]] — anchor resolution model
- [[xml-template-system]] — XML parsing and template creation paths
- [[lfd-dungeon-list-empty]] — nearby Dungeons & Raids panel investigation
