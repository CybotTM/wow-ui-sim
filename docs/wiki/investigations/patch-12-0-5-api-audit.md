# Patch 12.0.5 API Audit

Patch 12.0.5 work in wow-ui-sim is probe-driven rather than a single additive API-diff bridge pass. Retail `12.0.5.67823` live probes pinned core frame, event, attribute, identity, scale-event, and XML frame-level behavior; the simulator already models the safe findings with regression coverage. No dedicated `patch_12_0_5_inert_defaults` module exists.

## Content

### Source scope

The 12.0.5 audit sources are live-client probe addons under `docs/addons/` and the corresponding wiki investigation pages. Unlike the later 12.0.7 and 12.1 passes, this audit did not start from a patch-specific API-change page with a large additive namespace list.

Primary retained 12.0.5 probe sources (13 SavedVariables captures): `AnimScriptProbe`, `AttributeDispatchProbe`, `CoreBehaviorProbe`, `DevToolsDumpProbe`, `FrameIdentityProbe`, `HookScriptBindingProbe`, `IsProtectedProbe`, `JustifyProbe`, `ProtectedRetailProbe`, `ScaleEventProbe`, `SetAtlasProbe`, `StoreForbiddenProbe`, and `TextureSetTextureProbe`. `XmlFrameLevelProbe` findings are documented, but its raw capture was not retained.

### Itemized probe status

**Implemented with focused regression coverage:** animation handler matrix/rejection (`AnimScriptProbe`); unchanged scalar attribute dispatch (`AttributeDispatchProbe`); retail forbidden-frame behavior, invalid unit-event filters, wildcard false attributes and arity, Raise/Lower ordering (`CoreBehaviorProbe`); frame iteration/dump identity, `[0]` surrogate dispatch, duplicate-frame freshness (`DevToolsDumpProbe` / `FrameIdentityProbe`); HookScript binding validation/chaining; absent legacy `Protect`/`SetProtected` methods plus normal-frame `SetForbidden` no-op; XML FontString justify/default anchors; no-arg/invalid `SetAtlas`; XML frame-level propagation; ordered display/scale event pairs.

**Best-effort with existing subsystem coverage:** `ShowUIPanel` pulse/`CloseAllWindows`; `GetMouseFoci`/`GetMouseFocus` live return shape; protected-template descendant/anchor non-propagation; same-size maximize/restore duplicate scale-event pair; Store forbidden/dropdown observations. These remain best-effort because their retained capture is not matched by an exact focused simulator regression.

### Approved exceptions

User approved the documented exception set on 2026-07-14; approval records acceptance of these non-implemented gaps, not their implementation.

1. **Exact retained-probe regressions:** `ShowUIPanel` pulse then `CloseAllWindows`; exact `GetMouseFoci` shape; protected-template descendant/anchor return values (not mutation-block propagation); Store forbidden/dropdown observations. `Texture:SetTexture("Interface\\Buttons\\UI-Panel-Button-Up")` now exact-replays FDID `130828`, and no-arg `SetTexture()` clears. `SecureActionButtonTemplate:IsProtected()` explicit `(true, true)` is now replayed after real `Blizzard_FrameXML` loading. Existing subsystem tests are not exact probe replays. Repeated `false` attribute dispatch now has focused coverage for two writes, handler `false` arguments, nil lookup during dispatch, and final stored `false`, but is not a literal probe-addon replay.
2. **Store forbidden lifecycle:** `StoreDropdown_SetDropdown` was nil in the retained capture, so intended dropdown population and forbidden-descendant scan were never exercised. Do not invent lifecycle behavior; need a live capture where this API exists.
3. **XmlFrameLevel provenance:** bare/template/fixed/reparent findings are regression-tested, but the raw SavedVariables capture is missing. User approval on 2026-07-14 authorizes retaining this provenance gap as an exception pending a fresh live capture; the documented result is not a substitute for the missing raw capture.
4. **Same-size window transitions:** iced supplies no observable same-size maximize/restore event, so duplicate display/scale pair emission cannot be modeled without a platform window-state signal.
5. **Unscoped generic defaults:** no 12.0.5 patch API diff snapshot exists locally. Generic fallbacks cannot be claimed as 12.0.5-complete without a concrete probe/addon contract.

### Completed modeled work

Retail `12.0.5.67823` probe results are modeled in these areas:

- `CreateForbiddenFrame` is absent on current retail, and `SetForbidden(true)` on addon-created normal frames succeeds without making the frame forbidden.
- `RegisterUnitEvent("UNIT_HEALTH", "not_a_unit")` registers the event but drops the invalid unit filter; `IsEventRegistered("UNIT_HEALTH")` returns registered with no unit filter.
- Wildcard `GetAttribute` preserves an explicit `false` stored with `SetAttribute("*type1", false)`.
- `Raise()` / `Lower()` only affect same-raw-level tie ordering and do not let a lower frame level overtake a higher one.
- Frame identity dispatch uses `frame[0]` userdata tokens; surrogate tables shaped with `[0] = frame[0]` dispatch shared frame methods, while `[1]`-only surrogates do not.
- Duplicate named `CreateFrame` calls produce fresh Lua objects and fresh identity tokens rather than copying stale custom fields from the prior global binding.
- XML bare `frameLevel` is an absolute initial value, not a parent-relative offset, but remains non-fixed so later parent level changes shift the child by the captured parent delta; `fixedFrameLevel="true"` pins the level.
- `DISPLAY_SIZE_CHANGED` and `UI_SCALE_CHANGED` fire as an ordered pair for observable size/scale recalculations, with startup pairs before `PLAYER_LOGIN`.

Key implementation locations:

- `src/lua_api/frame/methods/text_attribute_event/events.rs` — invalid `RegisterUnitEvent` filter fallback and animation handler validation.
- `src/lua_api/frame/methods/text_attribute_event/attributes.rs` — retail forbidden-frame and attribute behavior.
- `src/lua_api/methods.rs`, `src/lua_bridge/table_builder.rs`, `src/lua_api/globals/create_frame/helpers_shared.rs` — frame identity token dispatch and duplicate named-frame behavior.
- `src/lua_api/globals/template/direct/frame_level.rs` — XML frame-level resolution and fixed/non-fixed propagation.
- `src/lua_api/env_runtime.rs`, `src/startup.rs`, `src/iced_app/resize_event_tests.rs` — display/scale event pair behavior.

### Verification

Regression coverage exists in:

- `tests/admin_event_api.rs` — invalid unit-filter registration fallback.
- `tests/protected_frame_enforcement.rs` — retail `SetForbidden` no-op behavior.
- `tests/protected_attribute_enforcement.rs` — wildcard explicit-false lookup and repeated-false dispatch ordering.
- `tests/frame_level.rs` — Raise/Lower and raised-frame-level ordering.
- `tests/security_api.rs`, `tests/frame_table_iteration.rs`, `tests/globals_legacy.rs` — frame identity slot, surrogate dispatch, opaque identity userdata, duplicate named-frame freshness.
- `tests/xml_frame_strata.rs` — XML `frameLevel` and `fixedFrameLevel` semantics.
- `src/iced_app/resize_event_tests.rs` — display/scale ordered-pair behavior.

### Remaining inert/default surface

There is no 12.0.5-specific inert-default module. Broad compatibility defaults still live in `src/lua_api/workarounds/temporary/` and permanent unsupported C API shims, but the 12.0.5 probe-backed findings listed above have modeled behavior and tests rather than patch-scoped inert stubs.

The remaining generic defaults are intentionally outside this 12.0.5 audit unless a probe or addon failure ties one to a 12.0.5 retail behavior contract. Examples include unsupported 3D/model domains, loose/placeholder namespace defaults, and compatibility fallbacks that are tracked by their own subsystem investigations.

### Audit state

This audit is complete for the approved source scope: remaining fidelity gaps are the approved exceptions above. No 12.0.5-specific inert-default module remains, but absence of a patch shim is not proof that every retained probe result has exact regression coverage.

## Sources

- [[retail-core-behavior-probes]] — core 12.0.5 live-client behavior findings.
- [[frame-surrogate-identity-slot]] — frame `[0]` identity-token behavior.
- [[display-size-ui-scale-events]] — display/scale event pair behavior.
- [XmlFrameLevelProbe](../../../docs/addons/XmlFrameLevelProbe/README.md) — live XML frame-level probe notes.
- [CoreBehaviorProbe](../../../docs/addons/CoreBehaviorProbe/README.md) — live core behavior probe notes.
- [FrameIdentityProbe](../../../docs/addons/FrameIdentityProbe/README.md) — live frame identity probe notes.
- [ScaleEventProbe](../../../docs/addons/ScaleEventProbe/README.md) — live display/scale event probe notes.

## See Also

- [[patch-12-0-7-api-audit]] — later additive API bridge audit pattern.
- [[patch-12-1-api-audit]] — PTR API bridge audit pattern.
- [[lua-api]] — Lua runtime surface and frame method dispatch.
- [[retail-core-behavior-probes]] — retained 12.0.5 core probe evidence.
- [[event-system]] — event registration/dispatch behavior.
- [[xml-template-system]] — XML template and frame-level handling.
