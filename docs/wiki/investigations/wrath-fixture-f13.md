# Wrath F13 loader fixtures

[Contract](../../specs/wrath-fixture-f13.md): preserve the original 27 retained consumers and assertions. Fresh pre-repair Wrath0/27PASS, Retail26/27PASS (pre-existing missing Archaeology Mainline TOC). Five additive companions pass on both profiles and do not replace missing modern consumers. Final repair proof pending.

## Root cause and bounded repair

Two generic TOC tests address the nonexistent modern SharedXMLBase directory on Wrath. Select the original profile FrameXML TOC instead; keep absolute/existing-file and positive Lua/XML counts verbatim. The PaperDoll symbol test discovers split addons without loading native core first. Match production with `common::load_profile_framexml` before that discovery, retaining the original consumer and assertion; Retail has no top-level FrameXML TOC and this helper changes nothing there.

The other24 original rows pin modern addon names, modern dependencies/metadata, or modern split-addon ordering. They remain enabled, byte-identical report-only failures. Test-owned empty-body/scope/environment/dependency specimens are additive contracts, never aliases for those Blizzard addons.

## Native evidence limits

Shipped Wrath TimeManager/AchievementUI/TradeSkillUI TOCs declare LoadOnDemand:1 and no dependencies. Shipped FrameXML.toc includes PaperDollFrame.xml before MainMenuBarBagButtons.xml; PaperDoll XML includes the Lua declaration, bag XML calls that function in OnLoad. This is shipped-file ordering evidence. Laptop static12340 loader reconstruction reports synchronous TOC order, immediate per-frame PostLoadXML OnLoad, base FrameXML before addon enumeration and demand-load required/optional dependencies. It is not native execution. LoadManagers/LoadWith behavior is outside these failures; no speculative producer model added.

Simulator source calls frame finalization and lifecycle dispatch before returning from XML frame creation. The native companion executes original core and original TimeManager/AchievementUI (plus Wrath TradeSkillUI) and requires no recorded Lua errors. No vendor, rilua or producer change.

## Sources

- [F13 contract](../../specs/wrath-fixture-f13.md)
- [TOC probes](../../../tests/toc_parsing.rs)
- [Original load-order consumers](../../../tests/load_order.rs)
- [Additive specimens](../../../tests/loader_specimens_f13.rs)
- [XML frame construction](../../../src/loader/xml_frame.rs)
- [XML lifecycle](../../../src/loader/xml_lifecycle.rs)

## See Also

- [[addon-loading]] — production discovery and file loading
- [[client-profiles]] — profile-scoped runtime inventory
