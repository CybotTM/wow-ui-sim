//! `C_QuestChoice` surface for the legacy Mists reward-choice dialog.

use super::ensure_namespace;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::closure::RustFn;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

const DEFAULT_CHOICE_ID: u32 = 9_001;
const DEFAULT_QUESTION: &str = "Choose an ally for the Jade Forest campaign.";

struct QuestChoiceOption {
    response_id: u32,
    answer: &'static str,
    description: &'static str,
    icon_id: u32,
    money: u32,
    item_id: u32,
    item_name: &'static str,
}

const QUEST_CHOICE_OPTIONS: &[QuestChoiceOption] = &[
    QuestChoiceOption {
        response_id: 101,
        answer: "Aid the Shado-Pan",
        description: "Stand with the Shado-Pan defenders and secure the temple grounds.",
        icon_id: 236695,
        money: 12_500,
        item_id: 87565,
        item_name: "Shado-Pan Field Kit",
    },
    QuestChoiceOption {
        response_id: 102,
        answer: "Support the Golden Lotus",
        description: "Help the Golden Lotus protect sacred Pandaren relics.",
        icon_id: 236681,
        money: 9_500,
        item_id: 87566,
        item_name: "Golden Lotus Field Kit",
    },
];

const QUEST_CHOICE_METHODS: &[(&str, RustFn)] = &[
    ("CloseQuestChoice", close_quest_choice),
    ("GetQuestChoiceInfo", get_quest_choice_info),
    ("GetQuestChoiceOptionInfo", get_quest_choice_option_info),
    (
        "GetQuestChoiceRewardCurrency",
        get_quest_choice_reward_currency,
    ),
    (
        "GetQuestChoiceRewardFaction",
        get_quest_choice_reward_faction,
    ),
    ("GetQuestChoiceRewardInfo", get_quest_choice_reward_info),
    ("GetQuestChoiceRewardItem", get_quest_choice_reward_item),
    ("SendQuestChoiceResponse", send_quest_choice_response),
];

pub(super) fn register_quest_choice_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_QuestChoice")?;
    register_quest_choice_methods(state, ns)?;
    Ok(())
}

fn register_quest_choice_methods(state: &mut LuaState, ns: GcRef<Table>) -> LuaResult<()> {
    for (name, function) in QUEST_CHOICE_METHODS {
        table_set_rust_fn_static(state, ns, name, *function)?;
    }
    Ok(())
}

fn close_quest_choice(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    sim.quest_choice_id = None;
    sim.quest_choice_response_id = None;
    Ok(0)
}

fn get_quest_choice_info(state: &mut LuaState) -> LuaResult<u32> {
    let choice_id = borrow_state(state)?
        .quest_choice_id
        .unwrap_or(DEFAULT_CHOICE_ID);
    state.push(Val::Num(choice_id as f64));
    push_str(state, DEFAULT_QUESTION);
    state.push(Val::Num(QUEST_CHOICE_OPTIONS.len() as f64));
    Ok(3)
}

fn get_quest_choice_option_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(option) = option_from_stack(state, 1)? else {
        return Ok(0);
    };
    state.push(Val::Num(option.response_id as f64));
    push_str(state, option.answer);
    push_str(state, option.description);
    state.push(Val::Num(option.icon_id as f64));
    Ok(4)
}

fn get_quest_choice_reward_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(option) = option_from_stack(state, 1)? else {
        return Ok(0);
    };
    push_str(state, option.answer);
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(option.money as f64));
    state.push(Val::Num(42_000.0));
    state.push(Val::Num(1.0));
    state.push(Val::Num(1.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(9)
}

fn get_quest_choice_reward_item(state: &mut LuaState) -> LuaResult<u32> {
    let Some(option) = option_from_stack(state, 1)? else {
        return Ok(0);
    };
    if i32::from_stack(state, 2)? != 0 {
        return Ok(0);
    }
    state.push(Val::Num(option.item_id as f64));
    push_str(state, option.item_name);
    state.push(Val::Num(option.icon_id as f64));
    state.push(Val::Num(1.0));
    Ok(4)
}

fn get_quest_choice_reward_currency(state: &mut LuaState) -> LuaResult<u32> {
    if option_from_stack(state, 1)?.is_none() || i32::from_stack(state, 2)? != 0 {
        return Ok(0);
    }
    state.push(Val::Num(396.0));
    state.push(Val::Num(463446.0));
    state.push(Val::Num(5.0));
    Ok(3)
}

fn get_quest_choice_reward_faction(state: &mut LuaState) -> LuaResult<u32> {
    if option_from_stack(state, 1)?.is_none() || i32::from_stack(state, 2)? != 0 {
        return Ok(0);
    }
    state.push(Val::Num(1091.0));
    state.push(Val::Num(250.0));
    Ok(2)
}

fn send_quest_choice_response(state: &mut LuaState) -> LuaResult<u32> {
    let response_id = u32::from_stack(state, 1)?;
    borrow_state_mut(state)?.quest_choice_response_id = Some(response_id);
    Ok(0)
}

fn option_from_stack(
    state: &mut LuaState,
    stack_index: i32,
) -> LuaResult<Option<&'static QuestChoiceOption>> {
    let option_index = i32::from_stack(state, stack_index)?;
    let Some(option) = QUEST_CHOICE_OPTIONS.get(option_index as usize) else {
        return Ok(None);
    };
    Ok(Some(option))
}

fn push_str(state: &mut LuaState, value: &str) {
    let value = create_string(state, value);
    state.push(value);
}
