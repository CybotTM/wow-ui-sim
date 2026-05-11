#![cfg(feature = "client-mists")]

use std::path::PathBuf;
use std::process::Command;

fn wow_sim_binary() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_wow-sim")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target")
                .join("debug")
                .join("wow-sim")
        })
}

#[test]
fn mists_social_panels_support_friends_who_guild_and_communities() {
    let output = Command::new("timeout")
        .arg("90")
        .arg(wow_sim_binary())
        .args([
            "--no-addons",
            "--no-saved-vars",
            "--exec-lua",
            r#"
            ToggleFriendsFrame(1)
            if not (FriendsFrame and FriendsFrame:IsShown()) then
                error("FriendsFrame did not open")
            end

            FriendsFrame_ShowSubFrame("FriendsListFrame")
            FriendsFrame_Update()
            if C_FriendList.GetNumFriends() < 1 then
                error("friend list has no seeded rows")
            end

            FriendsFrame_ShowSubFrame("WhoFrame")
            WhoList_Update()
            if not WhoFrame:IsShown() then
                error("WhoFrame did not show")
            end

            local shownWhos, totalWhos = C_FriendList.GetNumWhoResults()
            if shownWhos < 1 or totalWhos < shownWhos then
                error("who counts invalid")
            end

            local whoInfo = C_FriendList.GetWhoInfo(1)
            if not whoInfo or not whoInfo.fullName then
                error("who row missing")
            end

            PanelTemplates_SetTab(FriendsFrame, FRIEND_TAB_GUILD)
            FriendsFrame_Update()
            if not GuildFrame:IsShown() then
                error("GuildFrame did not show")
            end
            if GetGuildRosterSize() < 1 then
                error("guild roster has no rows")
            end

            local ok, reason = LoadAddOn("Blizzard_Communities")
            if ok == false then
                error("Blizzard_Communities failed to load: " .. tostring(reason))
            end
            if not CommunitiesFrame then
                error("CommunitiesFrame missing")
            end

            CommunitiesFrame:Show()
            if CommunitiesFrame.GuildFinderFrame then
                CommunitiesFrame.GuildFinderFrame:Show()
            end
            if CommunitiesFrame.CommunityFinderFrame then
                CommunitiesFrame.CommunityFinderFrame:Show()
            end
            "#,
            "lua-errors",
        ])
        .output()
        .expect("failed to run wow-sim");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "wow-sim failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert_no_lua_errors(&stdout, &stderr);
}

fn assert_no_lua_errors(stdout: &str, stderr: &str) {
    assert!(
        !stdout.contains("Lua error") && !stderr.contains("Lua error"),
        "social panel flow emitted Lua errors\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}
