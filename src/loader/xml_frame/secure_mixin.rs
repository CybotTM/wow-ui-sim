//! SecureMixin transformation applied to named mixin tables.
//!
//! In the real WoW client (and in wowless), when a frame has `secureMixin="Foo"`,
//! the mixin table `_G.Foo` has its methods moved into a hidden `__index` table,
//! leaving the mixin itself empty. `getmetatable(Foo)` returns a number (0).
//!
//! This mirrors the wowless `securemixin` handler:
//! ```lua
//! local vv = {}
//! for k, v in pairs(mv) do vv[k] = v; mv[k] = nil end
//! setmetatable(mv, { __index = vv, __metatable = 0 })
//! ```
//!
//! Additionally, we store the methods table in `__secureMixinMethods` (a registry table)
//! keyed by the mixin table reference, so that `Mixin()` can apply only the stable
//! methods (not user-added direct entries like test fixtures) when applying secure mixins
//! to new frame instances.

use crate::lua_api::LoaderEnv;

pub(super) fn apply_secure_mixins(env: &LoaderEnv<'_>, secure_mixin_attr: &str) {
    let transform = r#"
        local names = ...
        __secureMixinMethods = __secureMixinMethods or {}
        for _, name in ipairs(names) do
            local mv = _G[name] or (__secureenv and rawget(__secureenv, name))
            if mv and type(mv) == 'table' then
                local vv = {}
                for k, v in pairs(mv) do
                    vv[k] = v
                    mv[k] = nil
                end
                setmetatable(mv, { __index = vv, __metatable = 0 })
                __secureMixinMethods[mv] = vv
            end
        end
    "#;
    let names: Vec<String> = secure_mixin_attr
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if names.is_empty() {
        return;
    }
    let names_table = names
        .iter()
        .map(|name| format!("{name:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    let script = format!("local names = {{{names_table}}}\n{transform}");
    let _ = env.exec(&script);
}
