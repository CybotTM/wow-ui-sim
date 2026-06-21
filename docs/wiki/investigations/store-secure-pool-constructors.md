# Store Secure Pool Constructors

Retail Store cards rendered as blank/red placeholders and left the Store stuck behind the "Connecting to Shop" dialog because secure Store code used stale simulator pool constructors.

## Content

The Store addon runs with `UseSecureEnvironment: 1`, so `StoreFrame_OnLoad` resolves constructors through `__secureenv`. The simulator installed fallback pool constructors during bootstrap before `Blizzard_SharedXMLBase` loaded the real Blizzard pool surface. `_G.CreateFramePoolCollection` was later replaced by Blizzard's proxy-backed constructor, but `__secureenv.CreateFramePoolCollection` still pointed at the fallback implementation.

That stale secure constructor produced a private collection shape with an exposed `pools` table and no `GetPool` method. Store product cards then came from the fallback collection instead of Blizzard's pool proxy path, which broke normal card setup and made the visible Store card area look corrupt even though the Blizzard source files were valid.

The original simulator fix synced the real pool/factory constructors from `_G` into `__secureenv` immediately after `Blizzard_SharedXMLBase` loaded. Retail probing later showed `secureenv` has no `_G` fallback, so the replacement model is to load Blizzard Lua library files in the secure environment pass as well. That makes secureenv receive the Blizzard constructors directly instead of copying them from `_G` after the fact. The runtime Store data fallback also avoids relying on Store enum globals that `Blizzard_EnvironmentCleanup` removes, and keeps Store secure state in a local table so namespace fallbacks cannot turn `_state` into a function.

Regression coverage:

- `test_frame_pool_collection_returns_proxy_surface` asserts both global and secure `CreateFramePoolCollection()` return Blizzard-style proxy collections.
- `store_tree` exercises Store loading with the discovered Blizzard addon dependency closure instead of a hand-picked partial addon list.
- `text_manifest_entries_reject_binary_cache_data` guards against binary data entering text cache paths.
- `wow-sim --no-addons --no-saved-vars lua-errors` and a Store-open `lua-errors` probe both return `[]`.

## Sources

- [runtime_surface_bootstrap.lua](../../src/lua_api/env_init/runtime_surface_bootstrap.lua) — Store fallback state, enum restoration, and secure Store defaults
- [pool_constructor_defaults.rs](../../src/lua_api/workarounds/temporary/pool_constructor_defaults.rs) — fallback pool constructors for isolated partial loads
- [store_tree.rs](../../tests/store_tree.rs) — Store regression coverage
- [pool_api.rs](../../tests/pool_api.rs) — pool collection surface regression coverage

## See Also

- [[addon-load-order]] — addon load timing can require post-load repairs
- [[lib-test-failure-sweep-2026-06]] — records why post-load repairs must not rely on `hooksecurefunc(C_AddOns, "LoadAddOn")`
