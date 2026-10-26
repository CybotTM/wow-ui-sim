# Mists supplied transmog availability

The [contract](../../specs/mists-transmog-enabled.md) adds no inferred native default. Mists owns a separate environment-local `Option<bool>` in `c_api`; the modern default-true field remains unchanged. Registration uses `client_profile::ACTIVE`; a supplied value returns one boolean, and absent input raises the named error instead of choosing a display branch. Both exact original-consumer RED cases fail at `InterfaceOverrides.lua:4` without the producer. GREEN execution is pending.

## Declaration and selected consumer

Pinned Mists 5.5.4.70268 declares `IsTransmogEnabled` with no arguments and one non-nil boolean return in `Blizzard_APIDocumentationGenerated/TransmogOutfitInfoDocumentation.lua:586–593`. Declaration SHA256: `dbed46d9dfcdfe8aa67ba293ccb44fd38b629328cc2d2ec24a69a77549a84917`. The declaration does not establish either default value.

`Blizzard_SettingsDefinitions_Frame_Classic.toc:5–8` admits Game/classic and loads `Classic/InterfaceOverrides.lua`. Its original `AdjustDisplaySettings:4` reads the query. False registers `PROXY_SHOW_HELM` and `PROXY_SHOW_CLOAK` boolean settings and checkboxes at16–18 and32–34; true omits them. `Interface.lua:139` calls the function during category registration.

## Bounded proof design

Both tests load the original selected addon closure without overrides and assert consumer provenance. A line hook stops at38, after the query's complete branch and before unrelated Hardcore/settings branches. It does not replace the query, registration or control factories. Tests count real layout initializers and inspect their setting names/types.

Original SettingsPanel rejects duplicate variable registration. Each environment therefore enters false once, while true can repeat; the two selectors reverse initial ordering and both discriminate constant true and constant false. Unset recovery supplies true after the named error. This is query/branch acceptance, not full settings registration or startup cleanliness.

## Separate gaps

Retail/PTR's existing `SimState.transmog_enabled` starts true; no native default evidence is established here. Forever's selected Camelot spellbook path also declares/uses this query but remains without this Mists-only producer. Era/Anniversary's generated documentation lacks this member; Wrath lacks the documentation file.

## Sources

- [Contract](../../specs/mists-transmog-enabled.md)
- [Tests](../../../tests/mists_transmog_enabled.rs)
- Pinned Mists generated documentation and `Blizzard_SettingsDefinitions_Frame/Classic/InterfaceOverrides.lua` in the profile-scoped Blizzard UI cache.

## See Also

- [Consumer selection and RED proof](../investigations/consumer-selection-and-red-proof.md)
