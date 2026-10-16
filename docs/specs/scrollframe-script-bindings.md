# ScrollFrame script bindings

Scroll offset and range changes dispatch declared scripts through the existing binding registry. The simulator parses the unchanged Blizzard EventScrollFrame declarations rather than invoking similarly named properties as an alternate path. See [XML template system](../xml-template-system.md).

## What it must do

- [x] Parse and register XML `OnHorizontalScroll`, `OnVerticalScroll`, and `OnScrollRangeChanged` in ordinary and runtime-template construction.
- [x] Run registered precall, normal/hooks, and postcall handlers in order, with committed scroll values and existing payloads. Preserve unchanged-offset/range no-ops.
- [x] Report handler errors and continue later bindings.
- [x] Deliver unchanged EventScrollFrame callback-registry events once, before normal scripts as its XML declares; do not automatically invoke unregistered `*_Intrinsic` properties.
- [ ] Let empty `function=""` clear only its selected inherited script binding; preserve other bindings, nonempty functions, and methods.

## C9 range update boundary

The user-supplied functional requirement moves explicit range refresh to the ScrollFrame update pass. Its historical evidence is 3.3.5a/build12340, not executed native or modern Retail timing proof; no reconstructed implementation was consulted.

- [ ] `UpdateScrollChildRect()` marks the ScrollFrame dirty and returns without recalculating ranges or dispatching `OnScrollRangeChanged` on that API call stack.
- [ ] Process the dirty ScrollFrame after its OnUpdate and animation processing. A request from OnUpdate or animation processing can complete later in that same frame; an arbitrary timer or extra tick does not satisfy this boundary.
- [ ] Preserve resolved subtree range arithmetic, commit both ranges before dispatch, and notify only when the resulting pair changed. Repeated unchanged refresh remains silent.
- [ ] Preserve registered-binding order/error reporting, independent synchronous offset callbacks, scroll-child ownership, anchors and presentation. Timing changes do not authorize vendor patches or a layout redesign.

`tests/scroll_range_update_pass.rs` adds three **UNCOMPILED / UNEXECUTED** cases: outside-call return versus update pass, request inside OnUpdate versus later same-frame delivery, and a real Alpha animation OnFinished request versus delivery after animation processing. Callback entry is recorded before protected assertions; traces and the error sink are checked outside callbacks. No behavioral RED, producer, animation-order acceptance or cross-profile/native proof yet. Existing synchronous refresh assumptions require deliberate timing reconciliation without weakening their arithmetic/binding assertions.

## How it works

- [Event dispatch](../event-system.md)
- [Scroll offsets](scrollframe-offsets.md)
- [Scroll presentation](scrollframe-presentation.md)

## Implementation inventory

- `src/xml/types_support.rs`: parsed script declarations.
- `src/loader/helpers.rs`, `src/lua_api/globals/create_frame/template_chain.rs`: ordinary/runtime script enumeration and registration.
- `src/lua_api/frame/methods/widgets/slider.rs`: offset/range mutation and registered binding dispatch.

## Tests asserting this spec

`tests/scroll_widgets/script_bindings.rs`, inside the existing grouped integration target, proves registration and dispatch through an ordinary XML `<ScrollFrame>` and through cached EventScrollFrame XML/Lua. Both paths verify public `GetScript` bindings, committed payloads, callback order, and zero recorded Lua errors. Existing `tests/scroll_widgets.rs` retains offset/range/presentation controls.

## Known gaps (current cycle)

Tests-only `1e4706e2a` originally reproduced four failures: three missing XML registrations because `ScriptsXml` dropped the declarations, plus automatic invocation of an unregistered property. These were not dispatch-only RED failures. `c82ed91af` closes that boundary with 60 unique relevant passing cases, including ordinary XML and strengthened cached EventScrollFrame coverage; `/tmp/cross-version-scroll-script-bindings-verification-ledger.md` records full registration and runtime proof. Pre-existing readability caps remain reported there: `input_handlers_with_options` length and `template_chain.rs` file size.

`6325ae0d4` established RED for empty `function=""` clearing in ordinary normal, runtime precall, and cached FauxScrollFrame inheritance; postcall and retained-binding assertions followed the first failure, so they were not independently RED. Bounded default-profile GREEN at `3e67e7b6e` passes 47 test instances plus format/check/integration compilation, proving ordinary normal and explicit precall/postcall clearing. Intrinsic-default selection, whitespace-only classification, and `method=` precedence are source-inspected only. See [XML empty script-function clearing](xml-empty-script-clearing.md).

Blizzard's cached `Blizzard_SharedXML/Shared/Frame/EventScrollFrame.xml` explicitly binds the three `*_Intrinsic` methods as precalls. The corresponding Lua methods emit callback-registry events. Both files remain unchanged; this is consumer/source corroboration, not a fresh native-client probe.

## Out of scope

Mouse-wheel/size/lifecycle dispatch, new XML implicit argument names, offset clamping, layout or ownership changes, opposite-state recursive callback policy, and native/GPU/all-profile parity.
