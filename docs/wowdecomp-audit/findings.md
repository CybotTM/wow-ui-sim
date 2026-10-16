# Finding catalogue

**46 source-comparison entries, not an exhaustive gap count or 46 native-confirmed bugs.** Initial nine plus 32 expanded entries and five [15595 follow-ups](cataclysm-15595.md). Each linked detail gives reconstructed/source and simulator evidence, a discriminating fixture, proposed behavior changes, regression scope and profile limits.

Current disposition of exchanged items: [implementation checklist and revision-scoped cross-host reports](README.md#exchanged-implementation-checklist). C7 is reported completed; old C2/C10 gap reports are superseded within the reported scope; missing spacing API is rejected. Genuine Wrath consumer and C11/C12/T7/T8 remain open. Catalogue entries below preserve historical comparisons, not current-tip defect claims.

| Kind | Interpretation |
|---|---|
| Capability | A compared state/dispatch/render consumer is absent or incomplete. Does not establish required availability on every modern profile. |
| Boundary | Existing behavior differs at a validation, lifecycle, security or timing boundary; select the supported-profile contract before changing it. |
| Historical/versioned | API availability, coordinates or resource model may have changed since the reconstructed client. |
| Existing contract conflict | Current tracked simulator requirements deliberately describe different behavior; historical source is not authority to overwrite them. |

| ID | Compared behavior | Kind | Detail |
|---|---|---|---|
| T1 | Reject invalid texture-coordinate arity and range before mutation | Boundary | [Evidence / rework / regression](textures-and-text.md#t1--reject-invalid-texture-coordinate-arity-and-range-before-mutation) |
| T2 | Blend-mode validation and rendering must agree | Capability | [Evidence / rework / regression](textures-and-text.md#t2--blend-mode-validation-and-rendering-must-agree) |
| T3 | Indented wrapping must reach layout/rendering, not private getter storage | Capability | [Evidence / rework / regression](textures-and-text.md#t3--indented-wrapping-must-reach-layoutrendering-not-private-getter-storage) |
| T4 | FontString alpha gradients never reach glyph rendering | Capability | [Evidence / rework / regression](additional-rendering-and-map.md#t4--fontstring-alpha-gradients-never-reach-glyph-rendering) |
| T5 | Font mutation claims success without validating a usable font | Boundary | [Evidence / rework / regression](additional-rendering-and-map.md#t5--font-mutation-claims-success-without-validating-a-usable-font) |
| T6 | Shared FontObject mutations do not update existing FontStrings | Capability | [Evidence / rework / regression](additional-rendering-and-map.md#t6--shared-fontobject-mutations-do-not-update-existing-fontstrings) |
| C1 | Slider value assignment ignores its step | Boundary | [Evidence / rework / regression](controls-and-layout.md#c1--slider-value-assignment-ignores-its-step) |
| C2 | EditBox accepts `OnCursorChanged` but does not produce it | Capability | [Evidence / rework / regression](controls-and-layout.md#c2--editbox-accepts-oncursorchanged-but-does-not-produce-it) |
| C3 | Reversed/negative `HighlightText` endpoints select the wrong range | Boundary | [Evidence / rework / regression](controls-and-layout.md#c3--reversednegative-highlighttext-endpoints-select-the-wrong-range) |
| C4 | Password EditBox renders plaintext | Capability | [Evidence / rework / regression](additional-controls-text-and-tooltip.md#c4--password-editbox-renders-plaintext) |
| C5 | History getter reports occupancy instead of configured capacity | Boundary | [Evidence / rework / regression](additional-controls-text-and-tooltip.md#c5--history-getter-reports-occupancy-instead-of-configured-capacity) |
| C6 | Changing message visibility duration does not reset active message age | Boundary | [Evidence / rework / regression](additional-controls-text-and-tooltip.md#c6--changing-message-visibility-duration-does-not-reset-active-message-age) |
| C7 | Button highlight/disabled fonts do not project into active text | Capability | [Evidence / rework / regression](additional-widget-state.md#c7--button-highlightdisabled-fonts-do-not-project-into-active-text) |
| C8 | Slider range changes lack their script capability and value-transition lifecycle | Capability | [Evidence / rework / regression](additional-widget-state.md#c8--slider-range-changes-lack-their-script-capability-and-value-transition-lifecycle) |
| C9 | Scroll child-range refresh crosses a different callback boundary | Boundary | [Evidence / rework / regression](additional-widget-state.md#c9--scroll-child-range-refresh-crosses-a-different-callback-boundary) |
| L1 | Protected frames can use region targets that the reconstruction rejects | Boundary | [Evidence / rework / regression](controls-and-layout.md#l1--protected-frames-can-use-region-targets-that-the-reconstruction-rejects) |
| F1 | `SetAllPoints` bypasses protected mutation and named-target validation | Boundary | [Evidence / rework / regression](additional-controls-text-and-tooltip.md#f1--setallpoints-bypasses-protected-mutation-and-named-target-validation) |
| F2 | Move/resize starts do not establish the full script-driven interaction | Capability | [Evidence / rework / regression](additional-controls-text-and-tooltip.md#f2--moveresize-starts-do-not-establish-the-full-script-driven-interaction) |
| A1 | Translation, rotation and scale configuration does not animate the target | Capability | [Evidence / rework / regression](additional-animation.md#a1--translation-rotation-and-scale-configuration-does-not-animate-the-target) |
| A2 | `GetLoopState` returns configured mode instead of live direction | Existing contract conflict | [Evidence / rework / regression](additional-animation.md#a2--getloopstate-returns-configured-mode-instead-of-live-direction) |
| A3 | Invalid loop mode silently resets configuration | Boundary | [Evidence / rework / regression](additional-animation.md#a3--invalid-loop-mode-silently-resets-configuration) |
| A4 | Animation order bounds are not enforced | Boundary | [Evidence / rework / regression](additional-animation.md#a4--animation-order-bounds-are-not-enforced) |
| A5 | Group initial offsets are neither exposed nor applied | Historical/versioned | [Evidence / rework / regression](additional-animation.md#a5--group-initial-offsets-are-neither-exposed-nor-applied) |
| H1 | SimpleHTML loses document structure, styles and hyperlink hit regions | Capability | [Evidence / rework / regression](additional-controls-text-and-tooltip.md#h1--simplehtml-loses-document-structure-styles-and-hyperlink-hit-regions) |
| U1 | Tooltip textures use the wrong attachment and argument model | Historical/versioned | [Evidence / rework / regression](additional-controls-text-and-tooltip.md#u1--tooltip-textures-use-the-wrong-attachment-and-argument-model) |
| U2 | Clearing an empty tooltip emits an extra callback | Boundary | [Evidence / rework / regression](additional-controls-text-and-tooltip.md#u2--clearing-an-empty-tooltip-emits-an-extra-callback) |
| U3 | Double-column tooltip wrapping remains enabled when right text exists | Boundary | [Evidence / rework / regression](additional-controls-text-and-tooltip.md#u3--double-column-tooltip-wrapping-remains-enabled-when-right-text-exists) |
| M1 | Minimap texture methods do not drive its 2D overlays | Capability | [Evidence / rework / regression](additional-rendering-and-map.md#m1--minimap-texture-methods-do-not-drive-its-2d-overlays) |
| M2 | Minimap pings are raw coordinate storage, not position-derived state/events | Capability | [Evidence / rework / regression](additional-rendering-and-map.md#m2--minimap-pings-are-raw-coordinate-storage-not-position-derived-stateevents) |
| M3 | Minimap zoom is disconnected from client view/CVar state | Historical/versioned | [Evidence / rework / regression](additional-rendering-and-map.md#m3--minimap-zoom-is-disconnected-from-client-viewcvar-state) |
| G1 | Attack-action classification is constant false | Capability | [Evidence / rework / regression](additional-action-state.md#g1--attack-action-classification-is-constant-false) |
| G2 | Active auto-repeat actions cannot be reported | Capability | [Evidence / rework / regression](additional-action-state.md#g2--active-auto-repeat-actions-cannot-be-reported) |
| G3 | Action usability equals occupancy; no-mana is always false | Capability | [Evidence / rework / regression](additional-action-state.md#g3--action-usability-equals-occupancy-no-mana-is-always-false) |
| G4 | `UseAction` bypasses cursor placement and non-spell actions | Capability | [Evidence / rework / regression](additional-action-state.md#g4--useaction-bypasses-cursor-placement-and-non-spell-actions) |
| B1 | Stack splitting is a callable no-op | Capability | [Evidence / rework / regression](additional-inventory-and-currency.md#b1--stack-splitting-is-a-callable-no-op) |
| B2 | Whole-item cursor transfer discards instance information and lacks item-state validation | Capability | [Evidence / rework / regression](additional-inventory-and-currency.md#b2--whole-item-cursor-transfer-discards-instance-information-and-lacks-item-state-validation) |
| B3 | Container item use has no stateful dispatch | Capability | [Evidence / rework / regression](additional-inventory-and-currency.md#b3--container-item-use-has-no-stateful-dispatch) |
| B4 | Quest-item metadata and quest-log relationship are always absent | Capability | [Evidence / rework / regression](additional-inventory-and-currency.md#b4--quest-item-metadata-and-quest-log-relationship-are-always-absent) |
| B5 | Currency list expansion/watch state is a static catalogue, not a mutable display list | Capability | [Evidence / rework / regression](additional-inventory-and-currency.md#b5--currency-list-expansionwatch-state-is-a-static-catalogue-not-a-mutable-display-list) |
| X1 | Non-frame virtual templates overwrite definitions without trust precedence | Boundary | [Evidence / rework / regression](xml-and-dispatch.md#x1--non-frame-virtual-templates-overwrite-definitions-without-trust-precedence) |
| X2 | Failed includes have inconsistent continuation and diagnostic behavior | Boundary | [Evidence / rework / regression](xml-and-dispatch.md#x2--failed-includes-have-inconsistent-continuation-and-diagnostic-behavior) |
| C10 | EditBox changed notifications are synchronous or absent rather than coalesced at dirty processing | Existing contract conflict / capability | [Evidence / rework / regression](cataclysm-15595.md#c10--editbox-text-notifications-cross-different-mutationupdate-boundaries) |
| C11 | ScrollingMessageFrame ignores access-ID filtered counts and loses their metadata | Capability | [Evidence / rework / regression](cataclysm-15595.md#c11--scrollingmessageframe-loses-access-id-filtered-counts) |
| C12 | Disabled checked button hides its only checked texture | Capability | [Evidence / rework / regression](cataclysm-15595.md#c12--disabled-checked-button-hides-its-only-checked-texture) |
| T7 | FontString justification accepts invalid input instead of rejecting it | Boundary | [Evidence / rework / regression](cataclysm-15595.md#t7--fontstring-justification-accepts-invalid-input-instead-of-rejecting-it) |
| T8 | FontString text-color defaults and clamping differ | Boundary | [Evidence / rework / regression](cataclysm-15595.md#t8--fontstring-text-color-defaults-and-clamping-differ) |

## Coverage, exclusions and unanswered questions

[Coverage ledger](coverage.md) counts exact compared/unresolved/unchecked registration rows. One finding may span several methods; X1/X2 concern XML loading rather than a literal exported method. Do not equate finding count, method count and native parity.

The [Include-depth note](xml-and-dispatch.md#include-depth--unmeasured-boundary-not-a-demonstrated-mismatch) records no explicit guard in the traced recursion paths but no measured depth limit; it is not added to the discrepancy count. Intentional 3D rendering exclusions and untraced/retired APIs are not silently promoted to defects.

[Audit provenance and interpretation](README.md) · [Static security assessment](security.md)
