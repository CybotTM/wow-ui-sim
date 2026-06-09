# Frame Surrogate Identity Slot

Frame method dispatch now models WoW's frame identity slot so Restricted Environment-style surrogate tables carry identity through `self[0]` instead of the simulator's older `self[1]` unwrap convention.

## Content

wow-ui-sim frames are rilua tables with VM-level backing metadata that points to the Rust `widget::Frame` in `WidgetRegistry.widgets`. Frame methods are shared through one frame metatable, and receiver resolution uses the table backing to recover the frame id.

Blizzard's Restricted Environment creates frame-handle surrogates with the real frame's identity token in slot `0` and the original frame in slot `1`. The simulator previously left `frame[0]` nil and let universal dispatch recover surrogate identity from `self[1]`. That made Restricted Environment probes work, but it hardcoded one Blizzard surrogate layout into every frame method call.

The simulator now seeds each frame table's numeric slot `0` with a tiny backed identity table. Method receiver resolution reads `self[0]` before the table's direct backing, so a surrogate shaped like `{ [0] = frame[0] }` can call shared frame methods such as `GetName()` and `IsProtected()` without carrying the original frame in slot `1`. A `[1]`-only surrogate no longer dispatches; code that wants frame identity must copy the identity token.

Real-client probe on retail `12.0.5.67823` confirmed slot `0` is the dispatch identity: after `a[0] = b[0]`, `a:IsProtected()` returned `b`'s protected state and `a:GetName()` returned `b`'s name. On the real client `frame[0]` is a userdata token; wow-ui-sim currently uses a tiny backed table as the token.

Known fidelity gap: real WoW uses a userdata identity token; wow-ui-sim uses a tiny backed table. Lua-visible behavior changes from `frame[0] == nil` to `frame[0] ~= nil`, so regressions should first check code that inspected slot `0` directly.

Regression coverage lives in `security_api::test_frame_identity_slots_dispatch_surrogate_methods`.

## Sources

- [methods.rs](../../../src/lua_api/methods.rs) - frame receiver and surrogate backing resolution
- [table_builder.rs](../../../src/lua_bridge/table_builder.rs) - backed frame table and identity-token creation
- [security_api.rs](../../../tests/security_api.rs) - regression coverage for `[0]` surrogate dispatch
- [FrameIdentityProbe](../../../docs/addons/FrameIdentityProbe) - real-client SavedVariables probe for slot `0` reassignment behavior
- [[taint-system]] - Restricted Environment and secure-handler context

## See Also

- [[taint-system]] - protected-frame and Restricted Environment behavior
- [[lua-api]] - frame method dispatch and Lua API surface
- [[frame-data-flow]] - Lua/Rust frame identity and data flow
