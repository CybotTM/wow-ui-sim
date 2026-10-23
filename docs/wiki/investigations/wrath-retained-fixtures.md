# Wrath retained-failure fixture programme

Programme-level source of truth for the retained Wrath failures, verified against handoff triage, queue updates and recipe inventory as of **2026-10-23**, at master `4c40bdfcf`. Lane contracts own exact test membership, receipts and proof limits; this page does not duplicate them. A merged fixture lane is not whole-Wrath GREEN, and a recipe or static acceptance is not runtime proof.

## Applicability and standing exclusion rule

The [client-profile applicability contract](../../specs/client-profiles.md#startup-warning-test-applicability) already distinguishes absent modern consumers from applicable Wrath consumers and requires authentic FrameXML prerequisites. That principle remains there; the following programme-wide mechanics record the standing user decision of 2026-10-23:

- Exclude modern-only tests on Wrath without asking, using `post_wrath_ui_fixture!`. Preserve original test names and byte-identical bodies; other profiles remain unchanged. Each exclusion requires verification.
- Mixed tests whose generic and modern assertions need splitting remain user-decided; do not silently discard the generic contract. Apply approved splits, not invented replacement consumers.
- Expectations contradicting native Wrath behaviour require a user decision, not an automatic exclusion or assertion rewrite.

An exclusion means **not applicable on Wrath**, never PASS or a fixture repair. Historical lane specs may still describe enabled modern-only failures: later exclusion authorization does not retroactively change their recorded execution.

## Repair classification

A **class (a) repair** fixes test setup or oracle selection for an applicable contract: load original Wrath FrameXML in committed TOC order before dependent consumers, select native legacy globals/children, supply required modeled state, or bind snapshot expectations to the correct profile/revision. Preserve the behavior under test and its assertion strength. Test-owned specimens may exercise genuinely generic contracts, but additive companions cannot replace a missing original Blizzard consumer.

A modern-only consumer belongs to the exclusion rule, not class (a). A defect exposed after correct setup is a producer repair with its own behavioral RED/GREEN proof, not permission to weaken the test. Never patch vendor Lua, manufacture modern mixins, substitute mocks for native consumers, clear errors, or infer runtime success from a source edit. The [F07 contract](../../specs/wrath-fixture-f07.md) and [F13 contract](../../specs/wrath-fixture-f13.md) preserve concrete boundary examples and exact receipts.

## Batch register

The original triage assigns 746 fixture-gap names once across F01–F22. These are triage batches, not current compiled test counts: later splits and companions change inventories. Status below distinguishes integration from pending work; no acceptance is inferred for unrecorded lanes.

| Batch | Fixture scope | Status as of 2026-10-23 |
|---|---|---|
| F01 | Core FrameXML/window identities, part 1 | Pending; no merge receipt established by the consulted status sources |
| F02 | Core FrameXML/window identities, part 2 | Merged |
| F03 | Core FrameXML/window identities, part 3 | Merged |
| F04 | Action/stance/pet/aura callbacks, part 1 | Merged |
| F05 | Action/stance/pet/aura callbacks, part 2 | Pending runtime verification |
| F06 | Scrolling/control/templates, part 1 | Merged |
| F07 | Scrolling/control/templates, part 2 | Merged; approved 22-row exclusion follow-up EXCL-2 pending |
| F08 | Achievement closure/legacy children | Merged; approved modern-only exclusions do not imply Wrath passes |
| F09 | Legacy AutoCompleteBox callbacks | Merged; modern-only exclusions integrated through EXCL-1; glue-scope gap remains |
| F10 | Native spellbook versus talent panel | Pending runtime verification; implementation complete, not merged |
| F11 | Legacy tooltip/stored-size oracle | Pending runtime proof |
| F12 | Legacy AuctionUI browse/owned/bid | Pending independent verification; lane implementation complete |
| F13 | Native TOC/discovery/loader specimens | Merged; approved 24-row exclusion follow-up EXCL-F13 pending |
| F14 | Modeled surface/explicit namespaces | Pending runtime and producer proof |
| F15 | Secure headers/input targeting | Pending runtime proof |
| F16 | Generic geometry/art | Pending runtime proof; mixed cases held |
| F17 | Frame pools without SharedXML | Pending exclusion verification; all 22 rows excluded, no positive Wrath cases |
| F18 | Legacy bags/trade-skill bootstrap | Pending runtime proof; mixed cases and F18-P1 open |
| F19 | WorldFrame/WorldMap selectors | Pending runtime proof; mixed case held |
| F20 | Profile oracle/state setup | Pending runtime proof; mixed tooltip-cost oracle held |
| F21 | Revision/profile snapshots/provenance | Pending; evidence drift and publication oracles unresolved |
| F22 | ReloadUI/SavedVariables lifecycle | Pending; native reload-payload contradiction blocks whole-lane GREEN |

Merged F07 and F13 details remain in their [F07](../../specs/wrath-fixture-f07.md) and [F13](../../specs/wrath-fixture-f13.md) specs and linked investigations. F09's native callback contract and remaining scope limits live in [Wrath autocomplete results](../../specs/wrath-autocomplete-results.md). Pending recipes are prospective, not receipts. Later queue updates supersede recipe claims about missing master integration: F11 and F14–F19 integrated master `4c40bdfcf`; that resolves source integration, not runtime acceptance.

## Open classes and producer gap

Mixed-test decisions remain open for F16 (three), F18 (eight), F19 (one) and F20 (one). Native-contradiction tests remain a separate decision class, including F22's reload boolean oracle versus native zero-argument payload and existing Wrath container-info table assertions. Prior explicit exclusions do not authorize changing other native-conflicting expectations.

**F18-P1:** on Wrath, legacy `GetContainerItemInfo` returns a table while original `ContainerFrame.lua` expects positional values. Static inspection establishes a producer/consumer contract mismatch, not a successful runtime repair. The separate container-iteminfo lane has an edit-only RED fixture; execution-bound RED, producer GREEN and preservation proof remain pending. Two unchanged Wrath table-return assertions conflict with the proposed positional projection and require a user decision. Keep the original bag consumer; do not adapt vendor Lua to the simulator's incorrect shape.

## Sources

- Programme handoff batch triage, slot-queue decisions and slot-recipe index, consulted 2026-10-23 — batch scope, integration state, exclusions and pending proof; operational evidence remains outside tracked docs.
- [Client profiles contract](../../specs/client-profiles.md#startup-warning-test-applicability) — existing applicability and native-prerequisite rule.
- [F07 contract](../../specs/wrath-fixture-f07.md) and [F07 diagnosis](wrath-fixture-f07.md) — original-consumer and assertion boundaries.
- [F13 contract](../../specs/wrath-fixture-f13.md) and [F13 diagnosis](wrath-fixture-f13.md) — native setup versus additive specimens.
- [Autocomplete contract](../../specs/wrath-autocomplete-results.md) — F09 native adapter and evidence limits.

## See Also

- [[client-profiles]] — runtime profile and source inventory.
- [[wrath-fixture-f07]] — scrolling/template lane diagnosis.
- [[wrath-fixture-f13]] — loader lane diagnosis.
- [[wrath-autocomplete-fixture]] — native callback fixture.
