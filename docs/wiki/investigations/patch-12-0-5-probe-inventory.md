# Patch 12.0.5 Probe Inventory
Probe-subfinding register for the retained 12.0.5 live-client audit. Machine status advances only with row-specific behavioral evidence; prior documented states remain visible separately.

## Content
- **Source:** `data/patch-api/sources/12.0.5-probes.json`
- **Source SHA-256:** `35da3c796e6976464394dc8486d7748eb0b0cdd790b13cb2e1d845377b2f14e2`
- **Target:** retail build `12.0.5`
- **Rows:** 38 changed probe subfindings — 0 implemented, 20 best-effort, 0 exception-requested, 18 untriaged

| Symbol | Machine Status | Documented Status | Category | Direction | Detail |
|---|---|---|---|---|---|
| `AnimScriptProbe.HandlerMatrix` | best-effort | resolved | script support | changed | Frame, AnimationGroup, and all nine Animation subtypes match the documented matrix: unsupported HasScript calls succeed and return false, while SetScript rejects those handlers. |
| `AttributeDispatchProbe.ScalarRepeat` | best-effort | resolved | attributes | changed | Repeated scalar SetAttribute dispatches OnAttributeChanged; explicit false is preserved and delivered as false. |
| `AttributeDispatchProbe.PanelPulse` | untriaged | best-effort | panel lifecycle | changed | Repeated ShowUIPanel pulse followed by CloseAllWindows preserves the expected panel-stack behavior. |
| `CoreBehaviorProbe.ForbiddenState` | best-effort | resolved | forbidden frames | changed | Normal addon frames remain un-forbidden; SetForbidden does not create forbidden state. |
| `CoreBehaviorProbe.ForbiddenConstructor` | untriaged | resolved | forbidden frames | changed | CreateForbiddenFrame availability and EnumerateFrames behavior are captured. |
| `CoreBehaviorProbe.UnitEventFilter` | best-effort | resolved | events | changed | Valid RegisterUnitEvent registration works; invalid unit filters are dropped while the event remains registered. |
| `CoreBehaviorProbe.AttributeWildcardFalse` | best-effort | resolved | attributes | changed | Wildcard GetAttribute preserves explicit stored false. |
| `CoreBehaviorProbe.AttributeWildcardValues` | best-effort | resolved | attributes | changed | Wildcard true/string values and one-, two-, and three-argument lookup behavior are preserved. |
| `CoreBehaviorProbe.RaiseLowerLevel` | best-effort | resolved | frame ordering | changed | Raise/Lower affect same-level ties but cannot overtake a higher frame level. |
| `CoreBehaviorProbe.MouseFocusOrder` | untriaged | best-effort | mouse focus | changed | GetMouseFoci/GetMouseFocus shape and hit ordering after Raise/Lower. |
| `DevToolsDumpProbe.FrameArrayDump` | untriaged | resolved | frame identity | changed | tinsert(frame, foo), frame slot contents, and DevTools_Dump output metadata. |
| `FrameIdentityProbe.IdentitySlot` | best-effort | resolved | identity | changed | Frame slot [0] contains the identity userdata token. |
| `FrameIdentityProbe.SurrogateDispatch` | best-effort | resolved | identity | changed | Replacing [0] redirects protection and method dispatch; [1] alone does not. |
| `FrameIdentityProbe.DuplicateFreshness` | best-effort | resolved | identity | changed | Duplicate named frames receive fresh Lua objects, identity tokens, and custom-field state. |
| `HookScriptBindingProbe.IndexedHooks` | best-effort | resolved | scripts | changed | HookScript accepts indices 0, 1, and 2; GetScript retrieves indexed hooks. |
| `IsProtectedProbe.LegacySetters` | best-effort | resolved | protection | changed | Legacy Protect and SetProtected methods are absent and calls fail. |
| `IsProtectedProbe.SecureTemplate` | best-effort | resolved | protection | changed | Secure-template buttons report protected state while ordinary frames do not. |
| `IsProtectedProbe.DescendantAnchorPropagation` | untriaged | best-effort | protection | changed | Child, grandchild, and protected-anchor return values are captured. |
| `JustifyProbe.FrameFontStrings` | untriaged | resolved | FontString layout | changed | Direct unanchored frame-layer FontStrings receive the observed default anchors and justification. |
| `JustifyProbe.ButtonText` | best-effort | resolved | FontString layout | changed | Implicit ButtonText FontString behavior matches the probe matrix. |
| `JustifyProbe.SizeVariants` | untriaged | resolved | FontString layout | changed | No-size, width-only, height-only, and width+height variants are captured. |
| `JustifyProbe.ExplicitAnchors` | untriaged | resolved | FontString layout | changed | TOP/BOTTOM/LEFT/RIGHT/TOPLEFT controls distinguish missing from partial anchoring. |
| `JustifyProbe.EditBoxRegions` | untriaged | resolved | FontString layout | changed | EditBox FontStrings, including sized and inset variants, are captured. |
| `JustifyProbe.MessageRegions` | untriaged | resolved | FontString layout | changed | MessageFrame and ScrollingMessageFrame owner/region behavior and TextInsets effects. |
| `ProtectedRetailProbe.PlainFrame` | untriaged | resolved | protection | changed | Plain frame protection/forbidden state and legacy setter behavior. |
| `ProtectedRetailProbe.XmlProtected` | untriaged | resolved | protection | changed | XML protected=true frame state and setters. |
| `ProtectedRetailProbe.SecureStore` | untriaged | best-effort | protection | changed | Secure-template child state and Blizzard Store frame observations. |
| `ScaleEventProbe.OrderedEvents` | untriaged | resolved | scale events | changed | DISPLAY_SIZE_CHANGED, UI_SCALE_CHANGED, and relevant CVAR_UPDATE ordering. |
| `ScaleEventProbe.SameSizeDuplicatePair` | untriaged | unresolved | scale events | changed | Same-size maximize/restore duplicate display/scale event pair. |
| `SetAtlasProbe.InvalidArguments` | best-effort | resolved | texture atlas | changed | nil, no-argument, boolean, numeric, empty, and unknown atlas inputs. |
| `TextureSetTextureProbe.PathFdid` | best-effort | resolved | texture | changed | UI-Panel-Button-Up path assignment and retained FDID 130828. |
| `TextureSetTextureProbe.Clear` | best-effort | resolved | texture | changed | SetTexture(nil) and no-argument clearing behavior. |
| `XmlFrameLevelProbe.BareAndFixed` | best-effort | resolved | XML frame level | changed | Bare frameLevel versus fixedFrameLevel=true semantics. |
| `XmlFrameLevelProbe.ParentReparent` | best-effort | resolved | XML frame level | changed | Parent-level changes, unfixed-child propagation, and Lua SetFrameLevel reparenting. |
| `XmlFrameLevelProbe.Flags` | best-effort | resolved | XML frame level | changed | HasFixedFrameLevel and IsUsingParentLevel observations. |
| `XmlFrameLevelProbe.RawCaptureProvenance` | untriaged | unresolved | provenance | changed | Raw SavedVariables capture was not retained; only documented behavior remains. |
| `StoreForbiddenProbe.DropdownPopulation` | untriaged | unresolved | Store lifecycle | changed | Real and synthetic Store dropdown population plus button count/state. |
| `StoreForbiddenProbe.ForbiddenDescendants` | untriaged | unresolved | Store lifecycle | changed | Store frame/descendant forbidden/protected scan via /sfp. |

## Machine state totals

- implemented: 0
- best-effort: 20
- exception-requested: 0
- untriaged: 18

## Sources

- `data/patch-api/sources/12.0.5-probes.json` — categorized probe subfindings and preserved documented state metadata.
- `docs/wiki/investigations/patch-12-0-5-api-audit.md` — broader patch audit context.

## See Also

- [[patch-12-0-5-api-audit]] — broader patch audit context.
- [[patch-api-audit-manifest]] — register schema and validation contract.
