use super::{assert_ptr_source_omits_symbols, load_game_ui_without_player_choice};

const SNAPSHOT_ONLY_GLOBALS: &[&str] = &[
    "GuildControlUI_Discord_HideAll",
    "GuildControlUI_Discord_Update",
    "GuildControlUI_DiscordFrame_OnLoad",
    "GuildControlUI_LoadUI",
    "GuildControlUI_OnShow",
    "GuildControlUI_SetupDiscord",
    "GuildControlUI_SetupSelected",
    "GuildControlUI_Setup",
    "GuildControlUI_Show",
    "GuildControlUI_UnlinkDiscord",
];

/// Proves all proposed GuildControlUI globals are absent from PTR source and runtime.
#[test]
fn snapshot_only_guild_control_globals_remain_absent() {
    assert_ptr_source_omits_symbols(SNAPSHOT_ONLY_GLOBALS);

    let env = load_game_ui_without_player_choice();
    let absent_count: i32 = env
        .eval(
            r#"
            local names = {
                "GuildControlUI_Discord_HideAll",
                "GuildControlUI_Discord_Update",
                "GuildControlUI_DiscordFrame_OnLoad",
                "GuildControlUI_LoadUI",
                "GuildControlUI_OnShow",
                "GuildControlUI_SetupDiscord",
                "GuildControlUI_SetupSelected",
                "GuildControlUI_Setup",
                "GuildControlUI_Show",
                "GuildControlUI_UnlinkDiscord",
            }
            local absentCount = 0
            for _, name in ipairs(names) do
                if _G[name] == nil then
                    absentCount = absentCount + 1
                end
            end
            return absentCount
            "#,
        )
        .expect("GuildControlUI runtime probe succeeds");

    assert_eq!(absent_count, SNAPSHOT_ONLY_GLOBALS.len() as i32);
}
