//! Temporary inert defaults for additive 12.1 API names.
//!
//! These bridge addon-facing probes for systems the simulator does not model
//! yet: Discord linking and housing blueprint/editor state. The defaults are deliberately inert and
//! version-gated to 12.1+ so they do not widen the 12.0 retail surface.

const PATCH_12_1_INERT_DEFAULTS_LUA: &str = r#"
if type(GetBuildInfo) == "function" and select(4, GetBuildInfo()) >= 120100 then
    local function noop() end
    local function return_false() return false end
    local function return_nil() return nil end
    local function return_zero() return 0 end
    local function return_empty_table() return {} end
    local function discord_enabled()
        return type(GetCVar) == "function" and GetCVar("discordClientEnabled") == "1"
    end

    local function ensure_namespace(name)
        _G[name] = _G[name] or __wow_namespace()
        return _G[name]
    end

    local function set_default(namespace, key, fn)
        if rawget(namespace, key) == nil then
            namespace[key] = fn
        end
    end

    local battleNet = ensure_namespace("C_BattleNet")
    set_default(battleNet, "BNCheckTitleFriendInviteToUnit", return_false)
    set_default(battleNet, "SetAppearOffline", noop)

    local discord = ensure_namespace("C_Discord")
    set_default(discord, "Authorize", noop)
    set_default(discord, "GetDiscordChannelName", return_nil)
    set_default(discord, "GetDiscordUserID", return_nil)
    set_default(discord, "GetDisplayNameType", return_zero)
    set_default(discord, "GetGuildLinkStatus", return_nil)
    set_default(discord, "GetNumDiscordChannels", return_zero)
    set_default(discord, "GetNumDiscordServers", return_zero)
    set_default(discord, "GetServerLinkableChannels", return_empty_table)
    set_default(discord, "GetServerName", return_nil)
    set_default(discord, "GuildLink", noop)
    set_default(discord, "GuildUnlink", noop)
    set_default(discord, "IsEnabled", discord_enabled)
    set_default(discord, "IsGuildChannelLinked", return_false)
    set_default(discord, "IsGuildSettingSet", return_false)
    set_default(discord, "IsUserOAuthed", return_false)
    set_default(discord, "RefreshAuth", noop)
    set_default(discord, "SetGuildSetting", noop)
    set_default(discord, "UpdateDiscordServers", noop)
    set_default(discord, "UpdateGuildLobby", noop)

end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(PATCH_12_1_INERT_DEFAULTS_LUA)?;
    Ok(())
}
