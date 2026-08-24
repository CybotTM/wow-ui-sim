# Lua Call-Frame Restoration After Errors

Commit `ff01991aa` fixed stale `LuaState` call-frame state after direct Lua errors. Before the fix, failed `call_function_state` calls could leave `ci` pointing at a dead frame; later execution then cascaded into `expected Lua closure in execute`. The slice is fixed and focused-tested, but remaining default-retail startup errors are unresolved.

## Content

### Root cause

`call_function_state` and `call_function_state_multi` temporarily changed the active Lua state to invoke a function. On Lua failure, the direct-call path returned the error without restoring the saved call-frame state. `LuaState.ci` could remain nonzero, with stale stack/frame state visible to the next call.

### Fix

`call_function_state` now delegates to the multi-return implementation. `call_function_state_multi` saves `base` and `ci` before `precall`. On error it restores:

- `top` to the temporary function slot,
- `base` to the caller frame,
- `ci` to the saved call frame,
- `ci_overflow` when the frame stack is no longer full.

Successful calls retain their existing result collection and caller-state restoration.

### Proof

Focused test: `src/lua_api/script_helpers/tests.rs::direct_state_call_restores_call_frame_after_lua_error`.

The regression test:

1. directly calls a Lua function that raises `direct-state boom`,
2. asserts the error is returned and `lua.state().ci == 0`,
3. performs a subsequent direct call returning `42`.

The RED state reproduced stale-frame behavior; the GREEN state passed after `ff01991aa`.

### Remaining scope

This fixes call-frame restoration after direct Lua errors only. It does **not** claim that default-retail startup is clean. Other startup error families remain under investigation, including EditMode initialization, XML lifecycle dispatch, CooldownViewer state, and independent missing/default state.

## Sources

- [src/lua_api/methods.rs](../../src/lua_api/methods.rs) — direct Lua call state setup and restoration
- [src/lua_api/script_helpers/tests.rs](../../src/lua_api/script_helpers/tests.rs) — focused RED/GREEN regression test
- [playerspells-runtime-load](playerspells-runtime-load.md) — related nested addon-load call-frame preservation
- [rilua-mlua-gap-audit](rilua-mlua-gap-audit.md) — rilua migration gap context

## See Also

- [[playerspells-runtime-load]] — nested addon loading must preserve the active caller frame
- [[rilua-mlua-gap-audit]] — broader rilua migration gaps
- [[retail-ptr-full-startup-lua-errors]] — startup error investigations and bounded fixes
