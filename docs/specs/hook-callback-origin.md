# Hook callback origin

Simulator-owned `HookScript` and `hooksecurefunc` adapters must not acquire the installing addon's taint. They invoke original and hook functions without changing those functions' own origin or the invocation caller's taint.

## What it must do

- [ ] A hook installed by addon code must leave a trusted original secure when invoked from a clean engine context; the addon hook remains insecure.
- [ ] A hooked function called from addon code remains subject to that caller's taint; constructing the adapter must not clear the addon caller.
- [ ] Preserve original-before-hook ordering, original multi-return values including nil gaps, and existing error behavior.
- [ ] Generate only the fixed engine adapter, not arbitrary caller-supplied source. Do not taint an originally clean global slot merely by installing its secure hook.
- [ ] Hooks belong to the exact target object and method key. Hooking an inherited or copied wrapper must not append that callback to the source target's hook list. Preserve inherited hooks and repeated-hook ordering on the actual target.
- [ ] A texture-metatable pixel-snap hook followed by instance-local arrow and action-border hooks must not repaint the arrow when another texture's border is repainted.

## How it works

- [Event system](../event-system.md)
- [Closure taint lifetime](closure-taint-lifetime.md)

## Implementation inventory

- `src/lua_api/frame/methods/text_attribute_event/events/script_binding_args.rs` — protected creation of the fixed HookScript adapter.
- `src/lua_api/env_init/shared_bootstrap.lua` — bootstrap-owned secure-hook factory, without runtime loadstring generation.

## Tests asserting this spec

- `tests/hook_script.rs` — original/hook/caller taint, result preservation and existing chaining behavior.
- `tests/security_api.rs` — existing global/table hook consumers.
- `tests/secure_hook_target.rs` and `tests/fixtures/ellesmere_hook_consumers.lua` — inherited/copied target isolation, ordering/nil returns, and real extracted Ellesmere consumers driven by a timer.

## Known gaps (current cycle)

- [ ] Current compiled regressions and real addon replay pending. Existing runtime RED reports both callbacks insecure and a secret secure-hook wrapper.

## Out of scope

Clearing callback taint, changing addon code, general-purpose trusted compilation, or full native hooking conformance.
