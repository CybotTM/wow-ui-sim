//! Tests for `C_GossipInfo` probes backed by `SimState.gossip`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{GossipOption, GossipQuestRow};

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn event_listener_script(event_name: &str, flag_name: &str) -> String {
    format!(
        r#"
        {flag_name} = false
        local f = CreateFrame("Frame")
        f:RegisterEvent("{event_name}")
        f:SetScript("OnEvent", function(_, event)
            if event == "{event_name}" then
                {flag_name} = true
            end
        end)
        "#
    )
}

#[test]
fn get_options_returns_empty_array_by_default() {
    let env = env();
    let count: i32 = env.eval("return #C_GossipInfo.GetOptions()").unwrap();
    assert_eq!(count, 0);
}

#[test]
fn get_active_quests_returns_empty_array_by_default() {
    let env = env();
    let count: i32 = env.eval("return #C_GossipInfo.GetActiveQuests()").unwrap();
    assert_eq!(count, 0);
}

#[test]
fn get_available_quests_returns_empty_array_by_default() {
    let env = env();
    let count: i32 = env
        .eval("return #C_GossipInfo.GetAvailableQuests()")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn get_poi_for_ui_map_id_returns_nil() {
    let env = env();
    let result: Option<i32> = env
        .eval("return C_GossipInfo.GetPoiForUiMapID(1234)")
        .unwrap();
    assert!(result.is_none(), "GetPoiForUiMapID should return nil");
}

#[test]
fn friendship_reputation_defaults_are_zeroed_tables() {
    let env = env();
    let result: (i32, i32, i32, i32, i32, i32, i32) = env
        .eval(
            r#"
            local rep = C_GossipInfo.GetFriendshipReputation(1)
            local ranks = C_GossipInfo.GetFriendshipReputationRanks(1)
            return
                rep.friendshipFactionID,
                rep.reaction,
                rep.currentReactionThreshold,
                rep.nextReactionThreshold,
                rep.currentStanding,
                ranks.currentLevel,
                ranks.maxLevel
            "#,
        )
        .unwrap();
    assert_eq!(result, (0, 0, 0, 0, 0, 0, 0));
}

#[test]
fn get_text_returns_seeded_gossip_text() {
    let env = env();

    env.exec("A_Admin.OpenQuestNpc()").unwrap();

    let text: String = env.eval("return C_GossipInfo.GetText()").unwrap();
    assert_eq!(text, "How can I help you, adventurer?");
}

#[test]
fn get_options_returns_seeded_option() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.gossip.options.push(GossipOption {
            gossip_option_id: 42,
            order_index: 0,
            name: "Train me".into(),
            flags: 0,
            icon: 1,
            spell_id: None,
            select_option_when_only_option: false,
        });
    }
    let (count, id, name): (i32, i32, String) = env
        .eval(
            r#"
            local opts = C_GossipInfo.GetOptions()
            return #opts, opts[1].gossipOptionID, opts[1].name
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(id, 42);
    assert_eq!(name, "Train me");
}

#[test]
fn get_active_quests_returns_seeded_row() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.gossip.active_quests.push(GossipQuestRow {
            quest_id: 101,
            quest_info_id: 1,
            quest_level: 70,
            title: "Kill ten boars".into(),
            is_complete: Some(false),
            ..Default::default()
        });
    }
    let (count, quest_id, title): (i32, i32, String) = env
        .eval(
            r#"
            local q = C_GossipInfo.GetActiveQuests()
            return #q, q[1].questID, q[1].title
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(quest_id, 101);
    assert_eq!(title, "Kill ten boars");
}

#[test]
fn get_available_quests_returns_seeded_row() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.gossip.available_quests.push(GossipQuestRow {
            quest_id: 200,
            quest_info_id: 2,
            quest_level: 60,
            title: "Gather herbs".into(),
            is_legendary: true,
            ..Default::default()
        });
    }
    let (count, quest_id, is_legendary): (i32, i32, bool) = env
        .eval(
            r#"
            local q = C_GossipInfo.GetAvailableQuests()
            return #q, q[1].questID, q[1].isLegendary
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(quest_id, 200);
    assert!(is_legendary);
}

#[test]
fn get_options_rewards_field_is_empty_table() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.gossip.options.push(GossipOption {
            gossip_option_id: 1,
            name: "Hello".into(),
            ..Default::default()
        });
    }
    let rewards_count: i32 = env
        .eval("return #C_GossipInfo.GetOptions()[1].rewards")
        .unwrap();
    assert_eq!(rewards_count, 0, "rewards should be an empty table");
}

#[test]
fn get_options_spell_id_nil_when_absent() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.gossip.options.push(GossipOption {
            gossip_option_id: 7,
            name: "Vendor".into(),
            spell_id: None,
            ..Default::default()
        });
    }
    let is_nil: bool = env
        .eval("return C_GossipInfo.GetOptions()[1].spellID == nil")
        .unwrap();
    assert!(is_nil, "spellID should be nil when not set");
}

#[test]
fn admin_open_quest_npc_seeds_available_quest_and_fires_gossip_show() {
    let env = env();
    env.exec(&event_listener_script("GOSSIP_SHOW", "__gossip_show_fired"))
        .unwrap();

    env.exec("A_Admin.OpenQuestNpc()").unwrap();

    let (fired, active, count, quest_id, title): (bool, bool, i32, i32, String) = env
        .eval(
            r#"
            local quests = C_GossipInfo.GetAvailableQuests()
            return __gossip_show_fired, GetGossipNumAvailableQuests() == 1,
                #quests, quests[1].questID, quests[1].title
            "#,
        )
        .unwrap();
    assert!(
        fired,
        "OpenQuestNpc should dispatch GOSSIP_SHOW immediately"
    );
    assert!(
        active,
        "legacy gossip count should reflect the available quest"
    );
    assert_eq!(count, 1);
    assert_eq!(quest_id, 80000);
    assert_eq!(title, "The Lost Expedition");
}

#[test]
fn select_available_quest_sets_offer_and_fires_quest_detail() {
    let env = env();
    env.exec(&event_listener_script(
        "QUEST_DETAIL",
        "__quest_detail_fired",
    ))
    .unwrap();

    env.exec(
        r#"
        A_Admin.OpenQuestNpc(80002, "Supply Run")
        C_GossipInfo.SelectAvailableQuest(80002)
        "#,
    )
    .unwrap();

    let (fired, selected): (bool, i32) = env
        .eval("return __quest_detail_fired, C_QuestLog.GetSelectedQuest()")
        .unwrap();
    let state = env.state().borrow();
    assert!(fired, "SelectAvailableQuest should dispatch QUEST_DETAIL");
    assert_eq!(selected, 80002);
    assert_eq!(state.pending_quest_offer, Some(80002));
}
