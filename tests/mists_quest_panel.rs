#![cfg(feature = "client-mists")]

use std::process::Command;

#[test]
fn mists_quest_log_and_objective_tracker_open_cleanly() {
    let output = Command::new("timeout")
        .arg("90")
        .arg(env!("CARGO_BIN_EXE_wow-sim"))
        .args([
            "--no-addons",
            "--no-saved-vars",
            "--exec-lua",
            r#"
            ToggleQuestLog()
            if QuestLogFrame_Update then
                QuestLogFrame_Update()
            end

            local entries = GetNumQuestLogEntries()
            if entries < 2 then
                error("quest log has no quest entries")
            end

            local headerTitle, _, _, isHeader = GetQuestLogTitle(1)
            if headerTitle == nil or not isHeader then
                error("quest log first row is not a header")
            end

            local questTitle, _, _, questIsHeader, _, _, _, questID = GetQuestLogTitle(2)
            if questTitle == nil or questTitle == "" or questIsHeader or not questID then
                error("quest log second row is not a quest")
            end

            SelectQuestLogEntry(2)
            if GetQuestLogSelection() ~= 2 then
                error("SelectQuestLogEntry did not update the selected log index")
            end

            if GetAbandonQuestName() ~= questTitle then
                error("GetAbandonQuestName did not return the selected quest title")
            end

            if not CanAbandonQuest(questID) then
                error("selected quest should be abandonable")
            end

            if GetNumQuestWatches() < 1 then
                error("objective tracker has no watched quests")
            end

            if GetQuestIndexForWatch(1) ~= 2 then
                error("first watched quest does not resolve to the first quest row")
            end

            if not IsQuestWatched(2) then
                error("first quest row should be tracked")
            end

            WatchFrame_Update()
            if not WatchFrameHeader:IsShown() then
                error("watch frame header did not show tracked objectives")
            end

            local trackerTitle = WatchFrameTitle:GetText()
            if not trackerTitle or not trackerTitle:find("Objectives", 1, true) then
                error("watch frame title did not render objectives text")
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
        "quest log opened with Lua errors\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
}
