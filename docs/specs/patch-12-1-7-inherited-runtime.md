# Patch 12.1.7 inherited I4 runtime staging

The ordinary integration fixture in `tests/patch_12_1_7_inherited_i4.rs` measures the inherited inventory against selected original Blizzard sources. This is a test-first staging packet based on `489df2822`, not an implementation or native-parity claim. See the [I4 investigation](../wiki/investigations/patch-12-1-7-api-audit.md) and [source-accounting contract](patch-12-1-7-source-accounting.md).

## What it must do

- [ ] Expose the exact ordinary selectors `patch_12_1_7_inherited_i4::{ptr_discovery,ptr_preservation,retail_preservation,forever_preservation}` on their named profiles. Gates are explicit in the test file; there is no early-return success for an unrelated profile. `capture_profile_baseline` is a separate measurement selector on each of these three profiles.
- [ ] Bind PTR to wowt **12.1.7.70493**, Gethe `2cdbc7ff511ec6a8c2f4f8341accd1da44a3c9af`, the committed manifest/content-index identities, completion marker and SHA-256 of every asserted source witness and directly invoked original consumer. Use a private matching cache as `XDG_CACHE_HOME`, read-only. Do not use the obsolete 69594 cache.
- [ ] Retain all **1,876 literal occurrence IDs**, including opposing directions. Probe the recipe's **180 runtime rows** and **183 declaration candidates** separately; retain **1,512 vendor rows** and the **one 3D exclusion** as static-only, never fictitious runtime passes. The register retains all **214 I4c PTR UNKNOWN IDs** without changing their frozen dispositions.
- [ ] Require the **173 source-backed callable/method shapes** listed in the PTR register. Raw namespace lookup must not turn a generic namespace fallback into publication. Method lookup must use the attached ScriptRegion receiver or original selected Minimap instance. Historical added/removed direction must not supply a native absence oracle.
- [ ] Invoke selected original consumers with their original function provenance. Assert only documented nullable/non-null shapes and literal Lua consequences. Unknown native defaults, capability values, focus/click ordering, restrictions and server behavior remain `native-unknown`; accepting a no-op under a shape-only contract must not be reported as behavioral compatibility.
- [ ] Persist the complete observation map before aggregate shape or preservation assertions. Consumer errors are recorded separately and remain unclassified until execution proves the original selection and exact producer boundary. Do not call a syntax, dependency or fixture error producer RED.
- [ ] Require an independently captured, pre-repair **own-profile** baseline for preservation. Compare exact raw publications, attached methods, event registration, current/default CVar values, command catalog observations, original-consumer outcomes and selected load outcomes. PTR alone may replace a pre-repair shape gap with its explicit target shape and a listed failed consumer with success; all other observations remain equal. Retail/Forever allow no such repair waiver. Reject a missing baseline; never generate expected gaps from a failing repaired run. Do not impose PTR's SDHD argument or SwingTimer retirement candidates on Retail/Forever.

## Consumer and expected-RED register

Paths below are relative to the authenticated PTR `AddOns` root. These are **expected failures from source inspection, not executed RED receipts**. The committed profile registers include source hashes; no vendor bytes are copied or edited.

| Surface / IDs | Original consumer file:line | Expected on `489df2822` | Preservation / proof limit |
|---|---|---|---|
| `ScriptRegion:FocusEnter` / `FocusExit`, `wt-widgets-…-1825/1826` | `Blizzard_GamepadSmartNavigation/SmartNavigation.lua:662–670`; TOC lines 8–9 | Original calls encounter missing methods: no named production registration was found. | PTR and Forever use the same consumer cases with their own source hashes. Retail has no selected SmartNavigation addon in its cache and must not acquire it. Call-shape only; native focus/event order unknown. |
| `ScriptRegion:MouseDown` / `MouseUp`, `…-1827/1828` | Same file `:1502–1524`, original Click/RightClick after a real clickable receiver check | Missing methods at the original call boundary, not `SimulateMouseDown`, which is a different global. | Same profile separation. Default/RightButton calls are original Lua inputs, not invented native return values. Callback sequence and security remain unknown. |
| `CanPortGraveyard`, `wt-global-api-CanPortGraveyard-249` | `Blizzard_FrameXML/GhostFrame.lua:3–5`; selected `Blizzard_FrameXML.toc` | PTR registration is absent; `player_identity.rs` registers only Forever. Original true input reaches the query. | Forever preserves its own boolean-backed consumer; Retail preserves its own older visibility consumer without the new query. No default native capability value is required. |
| `RegionalUniqueNamesEnabled`, `…-265` | `Blizzard_RecentAllies/Blizzard_RecentAlliesUtil.lua:41–43` | PTR registration is absent; Forever-only implementation must not count as PTR proof. | Forever original name selection; Retail original null-time consumer and raw publication snapshot. Only boolean shape and membership in two supplied names, not a regional naming policy. |
| `C_Spell.GetItemCooldown`, `…-214` (declaration-candidate class) | `Blizzard_Collections/Shared/Blizzard_CollectionTemplates.lua:30–48` | No named `C_Spell` implementation found; `C_Item`/`C_Container` implementations are not this endpoint. Actual raw publication must still be measured. | Query concrete item input 6948 without assuming native item/cooldown state. Accept nil or a cooldown record with numeric start/duration and boolean enabled. Retail/Forever retain own raw surface; no forced new original-consumer branch there. |

The closure loader uses the ordinary Game screen, active TOC selection and dependencies. Roots are profile-specific. Original functions must report their cached file via `debug.getinfo`; a hand-written replacement fails this provenance condition. Full startup cleanliness is not asserted by these bounded consumers.

## Profile preservation register

| Profile | Register / invariant | Pre-repair baseline |
|---|---|---|
| PTR 70493 | `tests/data/p1217-i4/ptr.json`: 363 observations, 173 shape obligations, five original cases, 214 unresolved IDs | Not captured in this edit-only packet. `ptr_preservation` also retains the PTR shape assertions; it cannot bless the current gaps. |
| Retail selected 12.1.5 | `tests/data/p1217-i4/retail.json`: own sources, 363 observations, older original ghost visibility and null-time cases | Not captured; mandatory before repairs. PTR source hashes and shapes are not reused as Retail expectations. |
| Forever selected 1.60.1 | `tests/data/p1217-i4/wowforever.json`: own sources, 363 observations, original focus/mouse/ghost/name/null-time cases | Not captured; mandatory before repairs. Current Forever failures may be preserved as failures, not called correct. |
| Wrath, Mists, Era, Anniversary | Must remain unaffected by any eventual shared producer change; existing profile surface/consumer tests remain their preservation gate | No new expectations or selectors imposed here. No claim of seven-profile runtime proof. |

Register identity and loads must match the own-profile baseline. Observations and consumer outcomes must match except for the explicit PTR target-shape/failed-consumer transitions above. Source revision and ELF hashes intentionally differ across repaired artifacts and are evidence fields, not equality targets. A source/cache change requires a new, explicitly reviewed preservation epoch, never rewriting a baseline to match a repair.

## How it works

- [I4 static selection and limits](../wiki/investigations/patch-12-1-7-api-audit.md)
- [Original-consumer selection and RED proof](../wiki/investigations/consumer-selection-and-red-proof.md)
- [RegionalTextLocale bounded contract](client-profiles.md)

## Implementation inventory

- `tests/patch_12_1_7_inherited_i4.rs`: ordinary profile-specific selectors and explicit baseline capture selector.
- `tests/patch_12_1_7_inherited_i4/runtime.rs`: source binding, ordinary closure loading, complete result persistence and strict own-profile comparison.
- `tests/patch_12_1_7_inherited_i4/probe.lua`: raw publication, real receiver methods, registration and read-only CVar/command observations.
- `tests/patch_12_1_7_inherited_i4/consumers.lua`: original cached-function invocations, without replacing C APIs or vendor functions.
- `tests/data/p1217-i4/{ptr,retail,wowforever}.json`: independent source/expectation/preservation registers. Literal row data remains single-source in the existing inherited inventory.

## Tests asserting this spec

`tests/patch_12_1_7_inherited_i4.rs` supplies the four recipe selectors. They are staged, not runtime-verified. All checkboxes remain open until the exact source-bound executable is listed and run.

`tests/patch_12_1_7_toc.rs::p1217_original_loadlocale_default_us_consumer` already proves bounded default US RegionalTextLocale consumption on PTR/Forever in the parent receipts. Its original fixture is `tests/fixtures/p1217-loadlocale/`. It is preservation, not a new missing producer or RED. Alternate locale/region configuration remains outside that bounded proof; do not claim the old wider audit proposal was fully implemented.

## Known gaps (current cycle)

- [ ] Run exact discovery and classify each failure at the original selected consumer boundary. No executed RED/GREEN is supplied by staging or a compile check.
- [ ] Capture Retail/Forever baselines before producer repairs; then execute exact preservation selectors. Four historical proposed runs did not include these missing captures.
- [ ] Add safe original behavioral contexts for the remaining inherited surfaces. The 363 probes include publication/registration measurements, not 363 original-consumer behavior tests. Vendor rows, the residual 214 static UNKNOWN rows, unsafe/Glue-only paths and the 3D publication question are not closed by this packet.
- [ ] `C_GameRules.SetSDHDToggleValue` remains publication-shape only here. Original `Blizzard_Settings_Shared/Mainline/GraphicsOverrides.lua:77–119` gates its nested setter on `IsSDHDToggleEnabled`; `:98` supplies `Enum.SdToggleUsageLocationUI.OptionsMenu`. The Camelot CharacterCreate hit is not PTR-selected. A real modeled capability/settings context is required before original setter, invalid/secret/error behavior can be asserted. Do not force the gate true or choose an enum number.
- [ ] SwingTimer functions/events remain observations, not removal assertions: missing PTR declarations do not prove native absence. Preserve Forever and Retail independently. `PLAYER_SWING` is still registered in simulator source; that contradiction alone is not a native-removal RED.
- [ ] CVar values/defaults are preserved exactly, including nil versus empty; native PTR registration/default/scope is unresolved where source does not establish it. Scope is explicitly `native-unknown`, not a false assertion that all CVars persist. `dumpSmallAlloc` is queried from the command catalog and never executed.

## Out of scope

Native values not established by documented shapes/original consumers; production repairs; GUID obfuscation P02–P07; forced unsupported Glue contexts; 3D rendering; vendor edits; generated-source rewrites; suppression/ignore attributes; test-only production seams; merge, push or complete I4/native acceptance claims.
