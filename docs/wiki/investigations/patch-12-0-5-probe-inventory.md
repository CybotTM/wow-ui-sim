# Patch 12.0.5 Probe Inventory

Neutral subfinding-level register for the 12.0.5 probe audit. The machine status column mirrors the neutral manifest; documented probe states are preserved separately for review.

## Content

- **Source:** `data/patch-api/sources/12.0.5-probes.json`
- **Source SHA-256:** `ba32c953e6232d408eea328fc7bfb77a33a1fff803e310aefa6a76512178b344`
- **Target:** retail build `12.0.5`
- **Rows:** 38 changed probe subfindings — 30 resolved, 4 best-effort, 4 unresolved in documented source state

| Symbol | Machine Status | Documented Status | Category | Direction | Detail |
|---|---|---|---|---|---|
| `AnimScriptProbe.HandlerMatrix` | untriaged | resolved | script support | changed | Frame, AnimationGroup, and all nine Animation subtypes accept the documented handler matrix; HasScript and SetScript agree. |
| `AttributeDispatchProbe.ScalarRepeat` | untriaged | resolved | attributes | changed | Repeated scalar SetAttribute dispatches OnAttributeChanged; explicit false is preserved and delivered as false. |
| `AttributeDispatchProbe.PanelPulse` | untriaged | best-effort | panel lifecycle | changed | Repeated ShowUIPanel pulse followed by CloseAllWindows preserves the expected panel-stack behavior. |
| `CoreBehaviorProbe.ForbiddenState` | untriaged | resolved | forbidden frames | changed | Normal addon frames remain un-forbidden; SetForbidden does not create forbidden state. |
| `CoreBehaviorProbe.ForbiddenConstructor` | untriaged | resolved | forbidden frames | changed | CreateForbiddenFrame availability and EnumerateFrames behavior are captured. |
| `CoreBehaviorProbe.UnitEventFilter` | untriaged | resolved | events | changed | Valid RegisterUnitEvent registration works; invalid unit filters are dropped while the event remains registered. |
| `CoreBehaviorProbe.AttributeWildcardFalse` | untriaged | resolved | attributes | changed | Wildcard GetAttribute preserves explicit stored false. |
| `CoreBehaviorProbe.AttributeWildcardValues` | untriaged | resolved | attributes | changed | Wildcard true/string values and one-, two-, and three-argument lookup behavior are preserved. |
| `CoreBehaviorProbe.RaiseLowerLevel` | untriaged | resolved | frame ordering | changed | Raise/Lower affect same-level ties but cannot overtake a higher frame level. |
| `CoreBehaviorProbe.MouseFocusOrder` | untriaged | best-effort | mouse focus | changed | GetMouseFoci/GetMouseFocus shape and hit ordering after Raise/Lower. |
| `DevToolsDumpProbe.FrameArrayDump` | untriaged | resolved | frame identity | changed | tinsert(frame, foo), frame slot contents, and DevTools_Dump output metadata. |
| `FrameIdentityProbe.IdentitySlot` | untriaged | resolved | identity | changed | Frame slot [0] contains the identity userdata token. |
| `FrameIdentityProbe.SurrogateDispatch` | untriaged | resolved | identity | changed | Replacing [0] redirects protection and method dispatch; [1] alone does not. |
| `FrameIdentityProbe.DuplicateFreshness` | untriaged | resolved | identity | changed | Duplicate named frames receive fresh Lua objects, identity tokens, and custom-field state. |
| `HookScriptBindingProbe.IndexedHooks` | untriaged | resolved | scripts | changed | HookScript accepts indices 0, 1, and 2; GetScript retrieves indexed hooks. |
| `IsProtectedProbe.LegacySetters` | untriaged | resolved | protection | changed | Legacy Protect and SetProtected methods are absent and calls fail. |
| `IsProtectedProbe.SecureTemplate` | untriaged | resolved | protection | changed | Secure-template buttons report protected state while ordinary frames do not. |
| `IsProtectedProbe.DescendantAnchorPropagation` | untriaged | best-effort | protection | changed | Child, grandchild, and protected-anchor return values are captured. |
| `JustifyProbe.FrameFontStrings` | untriaged | resolved | FontString layout | changed | Direct unanchored frame-layer FontStrings receive the observed default anchors and justification. |
| `JustifyProbe.ButtonText` | untriaged | resolved | FontString layout | changed | Implicit ButtonText FontString behavior matches the probe matrix. |
| `JustifyProbe.SizeVariants` | untriaged | resolved | FontString layout | changed | No-size, width-only, height-only, and width+height variants are captured. |
| `JustifyProbe.ExplicitAnchors` | untriaged | resolved | FontString layout | changed | TOP/BOTTOM/LEFT/RIGHT/TOPLEFT controls distinguish missing from partial anchoring. |
| `JustifyProbe.EditBoxRegions` | untriaged | resolved | FontString layout | changed | EditBox FontStrings, including sized and inset variants, are captured. |
| `JustifyProbe.MessageRegions` | untriaged | resolved | FontString layout | changed | MessageFrame and ScrollingMessageFrame owner/region behavior and TextInsets effects. |
| `ProtectedRetailProbe.PlainFrame` | untriaged | resolved | protection | changed | Plain frame protection/forbidden state and legacy setter behavior. |
| `ProtectedRetailProbe.XmlProtected` | untriaged | resolved | protection | changed | XML protected=true frame state and setters. |
| `ProtectedRetailProbe.SecureStore` | untriaged | best-effort | protection | changed | Secure-template child state and Blizzard Store frame observations. |
| `ScaleEventProbe.OrderedEvents` | untriaged | resolved | scale events | changed | DISPLAY_SIZE_CHANGED, UI_SCALE_CHANGED, and relevant CVAR_UPDATE ordering. |
| `ScaleEventProbe.SameSizeDuplicatePair` | untriaged | unresolved | scale events | changed | Same-size maximize/restore duplicate display/scale event pair. |
| `SetAtlasProbe.InvalidArguments` | untriaged | resolved | texture atlas | changed | nil, no-argument, boolean, numeric, empty, and unknown atlas inputs. |
| `TextureSetTextureProbe.PathFdid` | untriaged | resolved | texture | changed | UI-Panel-Button-Up path assignment and retained FDID 130828. |
| `TextureSetTextureProbe.Clear` | untriaged | resolved | texture | changed | SetTexture(nil) and no-argument clearing behavior. |
| `XmlFrameLevelProbe.BareAndFixed` | untriaged | resolved | XML frame level | changed | Bare frameLevel versus fixedFrameLevel=true semantics. |
| `XmlFrameLevelProbe.ParentReparent` | untriaged | resolved | XML frame level | changed | Parent-level changes, unfixed-child propagation, and Lua SetFrameLevel reparenting. |
| `XmlFrameLevelProbe.Flags` | untriaged | resolved | XML frame level | changed | HasFixedFrameLevel and IsUsingParentLevel observations. |
| `XmlFrameLevelProbe.RawCaptureProvenance` | untriaged | unresolved | provenance | changed | Raw SavedVariables capture was not retained; only documented behavior remains. |
| `StoreForbiddenProbe.DropdownPopulation` | untriaged | unresolved | Store lifecycle | changed | Real and synthetic Store dropdown population plus button count/state. |
| `StoreForbiddenProbe.ForbiddenDescendants` | untriaged | unresolved | Store lifecycle | changed | Store frame/descendant forbidden/protected scan via /sfp. |

## Documented state totals

- resolved: 30
- best-effort: 4
- unresolved: 4

## Sources

- `data/patch-api/sources/12.0.5-probes.json` — categorized probe subfindings and preserved documented state metadata.
- `docs/wiki/investigations/patch-12-0-5-api-audit.md` — prior probe audit and status rationale.

## See Also

- [[patch-12-0-5-api-audit]] — broader patch audit context.
- [[patch-api-audit-manifest]] — register schema and validation contract.
