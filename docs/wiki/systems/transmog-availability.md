# Supplied transmog availability

The [contract](../../specs/transmog-availability.md) removes the guessed modern true default. `SimState.transmog_availability.enabled` is an environment-local `Option<bool>` shared by Retail, PTR and Mists. `TransmogAvailability::require_enabled` returns the supplied value or the named unsupplied-state error. The shared registrar uses `client_profile::ACTIVE`; it does not publish the query on other profiles or ungate the modern outfit module.

## Data flow

The query returns one boolean. Modern `pending.rs::build_pending_record` combines the supplied observation with its existing collected-source check and stores `can_transmogrify` in the pending record. Both direct staging and set/custom-set imports use that helper. `commit_slots` independently checks the current observation before application. None is an error, not false, and there is no fallback or automatic supply.

`GetViewedOutfitSlotInfo` returns the stored record bit, not a fresh availability calculation. Copying an outfit preserves existing record values. The current observation still gates commit; this change does not redesign cached-slot invalidation or establish native entitlement semantics.

## Declarations and selected consumers

Retail 12.1.5.70448 and PTR 12.1.7.70493 declare `C_TransmogOutfitInfo.IsTransmogEnabled` at `Blizzard_APIDocumentationGenerated/TransmogOutfitInfoDocumentation.lua:631–638`: no arguments, one non-nil bool. Both declare CommitAndApplyAllPending at95, SetOutfitToCustomSet at700, SetOutfitToSet at720 and SetPendingTransmog at730. Mists 5.5.4.70268 declares the query at586–593; its [original display-settings proof](mists-transmog-enabled.md) remains separate.

Retail has no original executable direct query caller. PTR `SpellBookTransmogCategoryMixin:IsAvailable` reads it at `Blizzard_PlayerSpells/SpellBook/Blizzard_SpellBookCategory.lua:512`. Its shared file is selected, but the only original constructor is Camelot-only and excluded on PTR. Manually invoking that function would not establish a selected spellbook flow.

Both profiles select the shared `Blizzard_Transmog` files in the Game LoadOnDemand addon. Original set/custom-set model mouse handlers call the import APIs at Retail Templates.lua:1416/1619 and PTR:1489/1692. Their XML templates bind OnMouseDown at Retail Templates.xml:1277–1287 / PTR:1301–1311. Original discount popup callbacks apply at Blizzard_Transmog.lua:61/67 on both profiles. These are interaction callbacks, not unconditional startup calls.

## Proof status

All eight exact Retail/PTR selectors fail on the scaffold/old producer for the intended constant-true behavior and pass with the shared producer: original set/custom-set clicks produce false/true from supplied input, popup application checks current input after a true stage, and the direct query preserves one-result bool arity. Each exact selector lists one case and runs alone; tests are byte-identical between siblings. No separately compiled constant-mutant result is claimed.

Seventeen targeted legacy transmog cases pass with explicit input (nine Retail, eight PTR). Both original Mists display-branch selectors pass after the shared-model rename, with empty load diagnostics. Paired raw stock maps are unchanged: Retail0/0 and PTR122/138, with no added, removed or count-changed key. The RED baseline is the master-based scaffold with old modern producers, not an exact pristine-master rebuild. All74 launch CVar guards preserve the original stat/hash using fresh child-only copies. Seven-profile all-target checks remain pending; other-profile absence has source guard proof, not a four-profile runtime acceptance claim.

The closure reports an unrelated nil call at `Blizzard_Transmog_Bootstrap.lua:14` and its ScriptErrors reporter. The original shared methods and XML mouse wrappers still load and reach the asserted state transitions. Tests do not replace the consumers or APIs, and do not establish complete bootstrap or wardrobe acceptance. No unconditional selected startup availability read was found; any measured startup None error must be treated as a product tradeoff rather than hidden with a default.

## Sources

- [Contract](../../specs/transmog-availability.md)
- [Retail/PTR tests](../../../tests/retail_transmog_enabled.rs)
- [Mists tests](../../../tests/mists_transmog_enabled.rs)
- Pinned profile-scoped Blizzard generated documentation and original shared transmog Lua/XML.

## See Also

- [Mists display branches](mists-transmog-enabled.md)
- [Consumer selection and RED proof](../investigations/consumer-selection-and-red-proof.md)
