//! Guild tabard snapshot queries, using the documented GuildTabardInfo DTO.
#![cfg(not(feature = "client-wrath"))]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn guild_tabard_info_is_nil_until_numeric_metadata_is_supplied() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(select('#', C_GuildInfo.GetGuildTabardInfo()) == 1)
        assert(C_GuildInfo.GetGuildTabardInfo() == nil)
        A_Admin.SetGuildEmblem('Interface\\GuildFrame\\GuildEmblem_01',
            0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9)
        assert(C_GuildInfo.GetGuildTabardInfo('player') == nil)
        A_Admin.SetGuildTabardMetadata(180159, 4)
        local info = C_GuildInfo.GetGuildTabardInfo('player')
        assert(info.emblemFileID == 180159 and info.emblemStyle == 4)
        assert(C_GuildInfo.GetGuildTabardInfo('self').emblemFileID == 180159)
        assert(C_GuildInfo.GetGuildTabardInfo('not-a-unit') == nil)
        local r, g, b = info.backgroundColor:GetRGB()
        assert(r == 0.1 and g == 0.2 and b == 0.3)
        r, g, b = info.borderColor:GetRGB()
        assert(r == 0.4 and g == 0.5 and b == 0.6)
        r, g, b = info.emblemColor:GetRGB()
        assert(r == 0.7 and g == 0.8 and b == 0.9)
        assert(select(10, GetGuildLogoInfo()) == 'Interface\\GuildFrame\\GuildEmblem_01')
        "#,
    )
    .unwrap();
}

#[test]
fn guild_tabard_queries_return_independent_color_objects() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(C_GuildInfo.GetGuildTabardInfo() == nil)
        A_Admin.SetGuildEmblem('crest', 0.25, 0.5, 0.75)
        A_Admin.SetGuildTabardMetadata(180159, 7)
        local first = C_GuildInfo.GetGuildTabardInfo()
        first.backgroundColor:SetRGB(1, 1, 1)
        first.emblemStyle = 99
        collectgarbage('collect')
        local second = C_GuildInfo.GetGuildTabardInfo()
        local r, g, b = second.backgroundColor:GetRGB()
        assert(r == 0.25 and g == 0.5 and b == 0.75)
        assert(second.emblemStyle == 7)
        "#,
    )
    .unwrap();
}

#[test]
fn guild_tabard_metadata_rejects_invalid_updates_and_clears_on_logo_replacement() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(C_GuildInfo.GetGuildTabardInfo() == nil)
        A_Admin.SetGuildTabardMetadata(180159, 3)
        for _, args in ipairs({{-1, 3}, {180159, -1}, {0.5, 3}, {180159, 0.5},
                              {4294967296, 3}, {180159}, {nil, 3}}) do
            local ok = pcall(A_Admin.SetGuildTabardMetadata, unpack(args, 1, 2))
            assert(not ok)
            local info = C_GuildInfo.GetGuildTabardInfo()
            assert(info.emblemFileID == 180159 and info.emblemStyle == 3)
        end
        A_Admin.SetGuildEmblem('replacement', 0.8, 0.6, 0.4)
        assert(C_GuildInfo.GetGuildTabardInfo() == nil)
        A_Admin.SetGuildTabardMetadata(180158, 2)
        A_Admin.SetGuildTabardMetadata()
        assert(C_GuildInfo.GetGuildTabardInfo() == nil)
        local r, g, b = GetGuildLogoInfo()
        assert(r == 0.8 and g == 0.6 and b == 0.4)
        "#,
    )
    .unwrap();
}
