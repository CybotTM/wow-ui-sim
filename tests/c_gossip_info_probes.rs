//! Tests for `C_GossipInfo` probes backed by `SimState.gossip`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{GossipOption, GossipQuestRow};

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
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
