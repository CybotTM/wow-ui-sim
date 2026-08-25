use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn collections_journal_opened_to_heirlooms_closes_on_escape() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.apply_post_load_workarounds();

    let gamepad_cursor_defaults: (String, String) = env
        .eval(
            "return type(CanAutoSetGamePadCursorControl), \
                type(SetGamePadCursorControl)",
        )
        .expect("post-cleanup gamepad cursor-control globals should be readable");
    assert_eq!(
        gamepad_cursor_defaults,
        ("function".into(), "function".into()),
        "ToggleGameMenu must retain its gamepad cursor-control dependencies after cleanup"
    );

    env.exec(
        r#"
        COLLECTIONS_JOURNAL_TAB_INDEX_HEIRLOOMS = 4

        local menuManager = {
            HandleESC = function()
                return false
            end,
        }
        Menu = {
            GetManager = function()
                return menuManager
            end,
        }

        ModelPreviewFrame = CreateFrame("Frame", "ModelPreviewFrame", UIParent)
        ModelPreviewFrame:Hide()
        GameMenuFrame:Hide()
        HelpFrame:Hide()
        EditModeManagerFrame:Hide()
        SettingsPanel:Hide()
        DISALLOW_SPELL_FLYOUTS = true

        CollectionsJournal = CreateFrame("Frame", "CollectionsJournal", UIParent)
        CollectionsJournal:Hide()

        local selectedTab = nil
        managedCollectionsJournalOpen = false

        function PanelTemplates_GetSelectedTab(frame)
            if frame == CollectionsJournal then
                return selectedTab
            end
        end

        function SetCollectionsJournalShown(shown, tabIndex)
            managedCollectionsJournalOpen = shown and true or false
            if tabIndex then
                selectedTab = tabIndex
            end
            if shown then
                CollectionsJournal:Show()
            else
                CollectionsJournal:Hide()
            end
        end

        function CloseAllWindows()
            if managedCollectionsJournalOpen then
                SetCollectionsJournalShown(false)
                return 1
            end
            return nil
        end
        "#,
    )
    .expect("test panel setup should execute");

    env.exec("ToggleCollectionsJournal(COLLECTIONS_JOURNAL_TAB_INDEX_HEIRLOOMS)")
        .expect("heirlooms tab should open");
    let opened_to_heirlooms: bool = env
        .eval(
            "return CollectionsJournal:IsShown() \
                and PanelTemplates_GetSelectedTab(CollectionsJournal) == COLLECTIONS_JOURNAL_TAB_INDEX_HEIRLOOMS \
                and managedCollectionsJournalOpen == true",
        )
        .expect("open state query should succeed");
    assert!(
        opened_to_heirlooms,
        "ToggleCollectionsJournal(4) should open the managed CollectionsJournal on the Heirlooms tab"
    );

    env.send_key_press("ESCAPE", None)
        .expect("ESCAPE dispatch should succeed");
    let closed_without_menu: bool = env
        .eval(
            "return not CollectionsJournal:IsShown() \
                and (GameMenuFrame == nil or not GameMenuFrame:IsShown())",
        )
        .expect("closed state query should succeed");
    assert!(
        closed_without_menu,
        "ESCAPE should close the managed CollectionsJournal instead of opening GameMenuFrame"
    );
}
