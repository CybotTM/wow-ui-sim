//! `C_GuildInfo` missing-member diagnostics.
//!
//! Guild state methods are Rust-backed; unmodeled members remain nil.

const GUILD_INFO_NAMESPACE_FALLBACK_LUA: &str = r#"
if type(C_GuildInfo) ~= "table" then
    C_GuildInfo = {}
end

local function GuildInfoNamespaceFallback(t, key)
    if type(__wow_record_nil_symbol_access) == "function" then
        __wow_record_nil_symbol_access("C_GuildInfo", key, nil, nil)
    end
    return nil
end

local mt = getmetatable(C_GuildInfo)
if mt == nil then
    setmetatable(C_GuildInfo, { __index = GuildInfoNamespaceFallback })
elseif mt.__index == nil then
    mt.__index = GuildInfoNamespaceFallback
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(GUILD_INFO_NAMESPACE_FALLBACK_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn preserves_rust_backed_methods_and_leaves_unmodeled_members_nil() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("A_Admin.SetGuildClubId('guild-123')")
            .expect("admin guild club id should set");

        let result: String = env
            .eval(
                r#"
                if C_GuildInfo.GetClubId() ~= "guild-123" then
                    return "lost_rust_method"
                end
                if C_GuildInfo.SomeUnimplementedMember ~= nil then
                    return "fabricated_member"
                end
                return "ok"
                "#,
            )
            .expect("guild info fallback probe should run");

        assert_eq!(result, "ok");
    }
}
