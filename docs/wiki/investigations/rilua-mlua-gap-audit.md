# Rilua Mlua Gap Audit

Audit of Lua API handling that existed on `master`'s mlua path but is still missing or partially downgraded on the current rilua path.

## Summary

The migration did not drop one giant subsystem; it left a handful of specific gap classes:

1. bootstrap / sandbox cleanup that master applied explicitly,
2. whole frame-method families that no longer register at all,
3. migrated methods that exist but still carry TODO-level semantics,
4. namespace/runtime patch hooks that exist but are not wired into startup.

The biggest concrete regression found in this audit is `MessageFrame`: master had a dedicated method surface, while the current rilua widget registrar has no message-frame module.

## Bootstrap / Environment Gaps

### Sandbox globals are no longer removed

Current `remove_sandbox_globals()` is a no-op in [`src/lua_api/env_init/mod.rs`](../../../src/lua_api/env_init/mod.rs). On the mlua path, bootstrap removed globals and helpers that WoW does not expose, including `dofile`, `load`, `loadfile`, `module`, `require`, `string.dump`, `math.randomseed`, and several internal helper globals.

Impact:
- addon environment is less WoW-like than master,
- hidden helper globals can leak into addon code,
- sandbox-sensitive behavior can diverge in subtle ways.

### Runtime namespace patch hook is not wired

[`src/lua_api/globals/system_api_runtime.rs`](../../../src/lua_api/globals/system_api_runtime.rs) still says `patch_namespace_stubs()` should be called from `register_globals()`, but the current registration path in [`src/lua_api/globals/register.rs`](../../../src/lua_api/globals/register.rs) does not call it.

Impact:
- late runtime overrides can silently stay on stub values,
- this is easy to miss because the patch code exists and looks "done".

## Missing Method Families

### MessageFrame surface dropped from registration

Master had dedicated mlua modules for:
- `widget_message_frame.rs`
- `widget_message_frame_callbacks.rs`
- `widget_message_frame_scroll.rs`

Those covered methods such as:
- `AddMessage`, `AddMsg`, `_AddMessageSilent`, `BackFillMessage`
- `GetNumMessages`, `GetMessageInfo`
- display-refresh callbacks and message transforms
- scrolling/history helpers

Current widget registration in [`src/lua_api/frame/methods/widgets/mod.rs`](../../../src/lua_api/frame/methods/widgets/mod.rs) registers texture, cooldown, editbox, slider, statusbar, model, and tooltip methods, but no message-frame module.

Impact:
- chat-style/message-frame widgets lost a real implementation path,
- missing behavior is not just "stub quality"; the registration family is absent.

## Partial Method Migrations

### Attribute / secure handling still incomplete

[`src/lua_api/frame/methods/text_attribute_event/attributes.rs`](../../../src/lua_api/frame/methods/text_attribute_event/attributes.rs) still carries core TODOs:
- `ExecuteAttribute` does not implement full callback/snippet semantics,
- frame refs are not preserved in the Lua-side attribute table,
- combat-lockdown checks are still missing for protected operations.

Related misc fallbacks in [`src/lua_api/frame/methods/misc/attribute_stubs.rs`](../../../src/lua_api/frame/methods/misc/attribute_stubs.rs) remain trivial.

### Event callback behavior still incomplete

[`src/lua_api/frame/methods/text_attribute_event/events.rs`](../../../src/lua_api/frame/methods/text_attribute_event/events.rs) still notes missing full unit-event callback handling and missing combat-lockdown checks.

Impact:
- registration exists, but behavior is still below master for secure/protected flows.

### Text handling downgraded in several places

[`src/lua_api/frame/methods/text_attribute_event/text.rs`](../../../src/lua_api/frame/methods/text_attribute_event/text.rs) still has TODOs for:
- button `Text` child creation,
- `SetFormattedText` parity,
- `SimpleHTML` per-textType dispatch,
- tooltip-line-related text behavior.

Impact:
- text APIs may appear present while still missing the higher-level WoW behavior Blizzard code expects.

### Cooldown / tooltip / input edges still have explicit TODOs

Current rilua code still documents missing behavior in:
- [`widgets/cooldown.rs`](../../../src/lua_api/frame/methods/widgets/cooldown.rs): countdown child creation, duration object parsing, vector args
- [`widgets/tooltip.rs`](../../../src/lua_api/frame/methods/widgets/tooltip.rs): `AddDoubleLine` still falls back to single-line handling
- [`widgets/slider.rs`](../../../src/lua_api/frame/methods/widgets/slider.rs): missing `OnValueChanged` firing
- [`widgets/editbox.rs`](../../../src/lua_api/frame/methods/widgets/editbox.rs): focus-script parity gaps

These are not missing registrations, but they are still missing master-era handling.

## Not Regressions

### 3D model stubs remain intentional

[`src/lua_api/frame/methods/widgets/model.rs`](../../../src/lua_api/frame/methods/widgets/model.rs) still contains many stubbed model / model-scene methods, but that matches the project's intentional 3D-rendering gap. This audit should not treat those stubs as migration regressions by default.

## Fix Order

1. Restore sandbox cleanup parity in `env_init`.
2. Reintroduce `MessageFrame` registration on the rilua path.
3. Finish attribute / event secure semantics before chasing cosmetic widget parity.
4. Wire `patch_namespace_stubs()` into `register_globals()`.
5. Clean up smaller widget TODOs (`tooltip`, `cooldown`, `slider`, `editbox`, `text`).

## Sources

- [src/lua_api/env_init/mod.rs](../../../src/lua_api/env_init/mod.rs) — current bootstrap path and no-op sandbox cleanup
- [src/lua_api/globals/register.rs](../../../src/lua_api/globals/register.rs) — current global registration path
- [src/lua_api/globals/system_api_runtime.rs](../../../src/lua_api/globals/system_api_runtime.rs) — unwired runtime namespace patch hook
- [src/lua_api/frame/methods/widgets/mod.rs](../../../src/lua_api/frame/methods/widgets/mod.rs) — current widget-method registration set
- [src/lua_api/frame/methods/text_attribute_event/attributes.rs](../../../src/lua_api/frame/methods/text_attribute_event/attributes.rs) — attribute TODOs
- [src/lua_api/frame/methods/text_attribute_event/events.rs](../../../src/lua_api/frame/methods/text_attribute_event/events.rs) — event TODOs
- [src/lua_api/frame/methods/text_attribute_event/text.rs](../../../src/lua_api/frame/methods/text_attribute_event/text.rs) — text TODOs

## See Also

- [[method-dispatch-refactor]] — prior method-surface / dispatch cleanup context
- [[generated-stubs-audit]] — similar audit pattern for remaining stubbed API surface
