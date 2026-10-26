# Wrath bag name

Wrath's `GetBagName` reads the existing container metadata for bag indices 1–11. Index 0 returns the current `BACKPACK_TOOLTIP` string; an out-of-range index returns nil. A nonnumeric argument raises a usage error. The original Wrath container frame uses the result as its bag title.

The pinned Wrath `FrameXML.toc` selects `ContainerFrame.xml`, which loads `ContainerFrame.lua`; line 506 calls this API. Exact 3.4.x native semantics are unknown. The argument, index and backpack choices use historical 3.3.5a comparison evidence, not proof of Wrath Classic parity. Bag names are supplied simulator state, not invented native item data.
