use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    let toc = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("Interface/AddOns/SimCommands/SimCommands.toc");
    load_addon(&env.loader_env(), &toc).expect("Failed to load SimCommands");
    env
}

#[test]
fn sim_commands_addon_loads() {
    let env = env();
    let exists: bool = env.eval("return type(SimCommands) == 'table'").unwrap();
    assert!(exists, "SimCommands global should exist after addon load");
}

#[test]
fn sim_commands_register_and_list() {
    let env = env();
    let added: i32 = env
        .eval(
            r#"
            local before = #SimCommands:GetCommands()
            SimCommands:Register("Test Command", "A test", function() end, "Debug")
            SimCommands:Register("Another", "Second test", function() end)
            return #SimCommands:GetCommands() - before
            "#,
        )
        .unwrap();
    assert_eq!(added, 2, "Should have added 2 commands");
}

#[test]
fn sim_commands_entry_fields() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            SimCommands:Register("Test Entry", "A test entry", function() end, "Debug")
            local cmds = SimCommands:GetCommands()
            local cmd = cmds[#cmds]  -- last registered
            return cmd.name .. "|" .. cmd.description .. "|" .. cmd.category
            "#,
        )
        .unwrap();
    assert_eq!(result, "Test Entry|A test entry|Debug");
}

#[test]
fn sim_commands_default_category() {
    let env = env();
    let cat: String = env
        .eval(
            r#"
            SimCommands:Register("No Category", "desc", function() end)
            local cmds = SimCommands:GetCommands()
            return cmds[#cmds].category
            "#,
        )
        .unwrap();
    assert_eq!(cat, "General", "Default category should be 'General'");
}

#[test]
fn sim_commands_filter_by_name() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            SimCommands:Register("Zzz Open Alpha", "Show alpha UI", function() end)
            SimCommands:Register("Set Level", "Change player level", function() end)
            SimCommands:Register("Zzz Open Beta", "Show beta UI", function() end)
            -- Filter for "zzz open" to match only our test entries
            local matches = SimCommands:Filter("zzz open")
            return tostring(#matches)
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "2",
        "Filter 'zzz open' should match 2 test commands"
    );
}

#[test]
fn sim_commands_filter_by_description() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            SimCommands:Register("Do Thing", "xyzzy unique desc", function() end)
            SimCommands:Register("Other", "unrelated", function() end)
            return tostring(#SimCommands:Filter("xyzzy"))
            "#,
        )
        .unwrap();
    assert_eq!(result, "1", "Filter 'xyzzy' should match description");
}

#[test]
fn sim_commands_filter_empty_returns_all() {
    let env = env();
    let result: bool = env
        .eval(
            r#"
            local total = #SimCommands:GetCommands()
            local filtered = #SimCommands:Filter("")
            return total == filtered and total > 0
            "#,
        )
        .unwrap();
    assert!(result, "Empty filter should return all commands");
}

#[test]
fn sim_commands_toggle_shows_frame() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            SimCommands:Toggle()
            local shown = SimCommands:IsShown()
            SimCommands:Toggle()
            local hidden = not SimCommands:IsShown()
            if shown and hidden then return "ok" end
            return "shown=" .. tostring(shown) .. " hidden=" .. tostring(hidden)
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Toggle should show then hide: {result}");
}

#[test]
fn sim_commands_palette_frame_exists() {
    let env = env();
    let exists: bool = env
        .eval(
            r#"
            SimCommands:Show()
            return SimCommandsFrame ~= nil
            "#,
        )
        .unwrap();
    assert!(exists, "SimCommandsFrame should exist after Show()");
}

#[test]
fn sim_commands_search_box_exists() {
    let env = env();
    let exists: bool = env
        .eval(
            r#"
            SimCommands:Show()
            return SimCommandsSearchBox ~= nil
            "#,
        )
        .unwrap();
    assert!(exists, "SimCommandsSearchBox should exist after Show()");
}

#[test]
fn sim_commands_shows_on_player_login_event() {
    let env = env();
    let initially_hidden: bool = env.eval("return not SimCommands:IsShown()").unwrap();
    assert!(
        initially_hidden,
        "Palette should stay hidden until PLAYER_LOGIN fires"
    );

    env.fire_event("PLAYER_LOGIN")
        .expect("PLAYER_LOGIN dispatch failed");
    let shown: bool = env.eval("return SimCommands:IsShown()").unwrap();
    assert!(shown, "PLAYER_LOGIN should auto-open the command palette");
}

#[test]
fn sim_commands_ctrl_p_toggles() {
    let env = env();
    env.send_key_press("CTRL-P", None)
        .expect("CTRL-P keybind failed");
    let shown: bool = env.eval("return SimCommands:IsShown()").unwrap();
    assert!(shown, "CTRL-P should open the command palette");

    env.send_key_press("CTRL-P", None)
        .expect("CTRL-P keybind failed");
    let hidden: bool = env.eval("return not SimCommands:IsShown()").unwrap();
    assert!(hidden, "CTRL-P again should close the command palette");
}

#[test]
fn sim_commands_minimap_button_exists() {
    let env = env();
    let exists: bool = env.eval("return SimCommandsMinimapButton ~= nil").unwrap();
    assert!(exists, "Minimap button should be created at load time");
}

#[test]
fn sim_commands_minimap_button_toggles_palette() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local btn = SimCommandsMinimapButton
            if not btn then return "no_button" end
            local click = btn:GetScript("OnClick")
            if not click then return "no_onclick" end
            click(btn)
            if not SimCommands:IsShown() then return "not_shown" end
            click(btn)
            if SimCommands:IsShown() then return "not_hidden" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "Minimap button click should toggle palette: {result}"
    );
}

#[test]
fn builtin_open_mailbox_registered() {
    let env = env();
    let found: bool = env
        .eval(
            r#"
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Open Mailbox" then return true end
            end
            return false
            "#,
        )
        .unwrap();
    assert!(found, "Open Mailbox command should be registered");
}

#[test]
fn builtin_open_mailbox_fires_event() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("MAIL_SHOW")
            f:SetScript("OnEvent", function(self, event)
                if event == "MAIL_SHOW" then fired = true end
            end)
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Open Mailbox" then
                    cmd.action()
                    break
                end
            end
            return fired and "ok" or "not_fired"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Open Mailbox should fire MAIL_SHOW: {result}");
}

#[test]
fn builtin_talk_to_quest_npc_opens_available_quest_gossip() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("GOSSIP_SHOW")
            f:SetScript("OnEvent", function(self, event)
                if event == "GOSSIP_SHOW" then fired = true end
            end)
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Talk to Quest NPC" then
                    cmd.action()
                    break
                end
            end
            local quests = C_GossipInfo.GetAvailableQuests()
            if not fired then return "not_fired" end
            if #quests ~= 1 then return "wrong_count" end
            return quests[1].questID .. ":" .. quests[1].title
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "80000:The Lost Expedition",
        "Talk to Quest NPC should open seeded quest gossip: {result}"
    );
}

#[test]
fn builtin_talk_to_quest_npc_is_marked_old() {
    let env = env();
    let description: String = env
        .eval(
            r#"
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Talk to Quest NPC" then
                    return cmd.description
                end
            end
            return "missing"
            "#,
        )
        .unwrap();
    assert!(
        description.to_ascii_lowercase().contains("old"),
        "Talk to Quest NPC should be marked old, got: {description}"
    );
}

#[test]
fn builtin_talk_to_multi_quest_npc_opens_quest_greeting() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("QUEST_GREETING")
            f:SetScript("OnEvent", function(self, event)
                if event == "QUEST_GREETING" then fired = true end
            end)
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Talk to Multi-Quest NPC" then
                    cmd.action()
                    break
                end
            end

            if not fired then return "not_fired" end
            if GetNumAvailableQuests() ~= 1 then return "available_count=" .. GetNumAvailableQuests() end
            if GetNumActiveQuests() ~= 2 then return "active_count=" .. GetNumActiveQuests() end
            if UnitName("questnpc") ~= "Quest Giver" then return "questnpc=" .. tostring(UnitName("questnpc")) end

            local activeTitle, activeComplete = GetActiveTitle(1)
            local secondTitle, secondComplete = GetActiveTitle(2)
            local availableTitle = GetAvailableTitle(1)
            local _, _, _, _, availableQuestID = GetAvailableQuestInfo(1)
            if activeComplete ~= true then return "first_not_complete" end
            if secondComplete ~= false then return "second_not_incomplete" end

            SelectActiveQuest(1)
            local name, texture, count, quality, usable = GetQuestItemInfo("reward", 1)
            if name ~= "Earthen Lockbox" then return "reward_name=" .. tostring(name) end
            if texture ~= "Interface\\Icons\\INV_Box_01" then return "reward_texture=" .. tostring(texture) end
            return GetQuestID() .. ":" .. activeTitle .. ":" ..
                GetActiveQuestID(2) .. ":" .. secondTitle .. ":" ..
                availableQuestID .. ":" .. availableTitle .. ":" ..
                count .. ":" .. quality .. ":" .. tostring(usable)
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "80001:Defending the Gates:80000:The Lost Expedition:80002:Supply Run:1:3:true",
        "Talk to Multi-Quest NPC should open mixed quest gossip with rewards: {result}"
    );
}

#[test]
fn builtin_talk_to_multi_quest_npc_selects_available_quest_detail() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local detailFired = false
            local completeFired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("QUEST_DETAIL")
            f:RegisterEvent("QUEST_COMPLETE")
            f:SetScript("OnEvent", function(self, event)
                if event == "QUEST_DETAIL" then detailFired = true end
                if event == "QUEST_COMPLETE" then completeFired = true end
            end)
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Talk to Multi-Quest NPC" then
                    cmd.action()
                    break
                end
            end

            SelectAvailableQuest(1)
            if not detailFired then return "detail_not_fired" end
            if completeFired then return "complete_fired" end
            if GetQuestID() ~= 80002 then return "quest_id=" .. tostring(GetQuestID()) end
            if GetTitleText() ~= "Supply Run" then return "title=" .. tostring(GetTitleText()) end
            if GetQuestText() == "" then return "missing_description" end
            if GetObjectiveText() == "" then return "missing_objective" end
            if GetRewardText() == "" then return "missing_reward_text" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "Selecting available quest should open QuestFrame detail data: {result}"
    );
}

#[test]
fn builtin_open_bank_fires_event() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("BANKFRAME_OPENED")
            f:SetScript("OnEvent", function(self, event)
                if event == "BANKFRAME_OPENED" then fired = true end
            end)
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Open Bank" then
                    cmd.action()
                    break
                end
            end
            return fired and "ok" or "not_fired"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "Open Bank should fire BANKFRAME_OPENED: {result}"
    );
}

#[test]
fn builtin_open_merchant_fires_event() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("MERCHANT_SHOW")
            f:SetScript("OnEvent", function(self, event)
                if event == "MERCHANT_SHOW" then fired = true end
            end)
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Open Merchant" then
                    cmd.action()
                    break
                end
            end
            return fired and "ok" or "not_fired"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "Open Merchant should fire MERCHANT_SHOW: {result}"
    );
}

#[test]
fn builtin_open_guild_bank_fires_event() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local fired = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("GUILDBANKFRAME_OPENED")
            f:SetScript("OnEvent", function(self, event)
                if event == "GUILDBANKFRAME_OPENED" then fired = true end
            end)
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Open Guild Bank" then
                    cmd.action()
                    break
                end
            end
            return fired and "ok" or "not_fired"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "Open Guild Bank should fire GUILDBANKFRAME_OPENED: {result}"
    );
}

#[test]
fn builtin_set_player_level_shows_prompt() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Set Player Level" then
                    cmd.action()
                    break
                end
            end
            local prompt = SimCommandsPrompt
            if not prompt then return "no_prompt" end
            if not prompt:IsShown() then return "not_shown" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "Set Player Level should show prompt dialog: {result}"
    );
}

#[test]
fn builtin_set_player_level_applies() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- Trigger the command to set up the prompt
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Set Player Level" then
                    cmd.action()
                    break
                end
            end
            -- Simulate typing and pressing Enter
            local input = SimCommandsPromptInput
            if not input then return "no_input" end
            input:SetText("42")
            local enter = input:GetScript("OnEnterPressed")
            if not enter then return "no_enter_handler" end
            enter(input)
            -- Verify
            local level = UnitLevel("player")
            if level == 42 then return "ok" end
            return "level=" .. tostring(level)
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "Set Player Level should apply level 42: {result}"
    );
}

#[test]
fn builtin_add_gold_applies() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- Set starting money to 0
            A_Admin.SetMoney(0)
            -- Trigger the command
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Add Gold" then
                    cmd.action()
                    break
                end
            end
            -- Simulate entering 100 gold
            local input = SimCommandsPromptInput
            if not input then return "no_input" end
            input:SetText("100")
            local enter = input:GetScript("OnEnterPressed")
            enter(input)
            -- 100 gold = 1000000 copper
            local money = GetMoney()
            if money == 1000000 then return "ok" end
            return "money=" .. tostring(money)
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "ok",
        "Add Gold should add 100g (1000000 copper): {result}"
    );
}

#[test]
fn builtin_equip_gear_presets_registered() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local found_seraph = false
            local found_naked = false
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Equip: Entombed Seraph (ilvl 571)" then found_seraph = true end
                if cmd.name == "Equip: Naked (no gear)" then found_naked = true end
            end
            if not found_seraph then return "missing_seraph" end
            if not found_naked then return "missing_naked" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Gear presets should be registered: {result}");
}

#[test]
fn builtin_equip_naked_clears_gear() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- Default gear should have items
            local before = GetInventoryItemID("player", 1)
            if not before then return "no_default_gear" end
            -- Run naked preset
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Equip: Naked (no gear)" then
                    cmd.action()
                    break
                end
            end
            local after = GetInventoryItemID("player", 1)
            if after then return "slot1_not_cleared=" .. tostring(after) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Naked preset should clear all gear: {result}");
}

#[test]
fn builtin_join_guild() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Join Guild" then
                    cmd.action()
                    break
                end
            end
            -- Simulate entering guild name
            local input = SimCommandsPromptInput
            if not input then return "no_input" end
            input:SetText("Test Guild")
            local enter = input:GetScript("OnEnterPressed")
            enter(input)
            if IsInGuild() then return "ok" end
            return "not_in_guild"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Join Guild should set guild: {result}");
}

#[test]
fn builtin_leave_guild() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- Start in a guild (default state)
            if not IsInGuild() then return "not_in_guild_initially" end
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Leave Guild" then
                    cmd.action()
                    break
                end
            end
            if IsInGuild() then return "still_in_guild" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Leave Guild should clear guild: {result}");
}

#[test]
fn builtin_set_honor_level() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Set Honor Level" then
                    cmd.action()
                    break
                end
            end
            local input = SimCommandsPromptInput
            if not input then return "no_input" end
            input:SetText("25")
            local enter = input:GetScript("OnEnterPressed")
            enter(input)
            local level = UnitHonorLevel("player")
            if level == 25 then return "ok" end
            return "level=" .. tostring(level)
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Set Honor Level should apply: {result}");
}

#[test]
fn builtin_add_mount_registered() {
    let env = env();
    let found: bool = env
        .eval(
            r#"
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Add Mount" then return true end
            end
            return false
            "#,
        )
        .unwrap();
    assert!(found, "Add Mount command should be registered");
}

#[test]
fn builtin_add_battle_pet_registered() {
    let env = env();
    let found: bool = env
        .eval(
            r#"
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Add Battle Pet" then return true end
            end
            return false
            "#,
        )
        .unwrap();
    assert!(found, "Add Battle Pet command should be registered");
}

#[test]
fn builtin_add_toy_registered() {
    let env = env();
    let found: bool = env
        .eval(
            r#"
            for _, cmd in ipairs(SimCommands:GetCommands()) do
                if cmd.name == "Add Toy" then return true end
            end
            return false
            "#,
        )
        .unwrap();
    assert!(found, "Add Toy command should be registered");
}
