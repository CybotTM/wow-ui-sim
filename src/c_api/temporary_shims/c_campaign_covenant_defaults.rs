//! Temporary campaign and covenant sanctum defaults.
//!
//! These small data surfaces are queried by legacy Blizzard UI panels. They do
//! not have backing progression state yet, so this module keeps their
//! compatibility defaults explicit and out of the runtime bootstrap Lua.

use crate::c_api::{ensure_namespace, global_val};
use crate::lua_api::methods::{
    create_string, create_string_static, create_table, create_table_with_fields, table_get_static,
    table_set_num,
};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_campaign_covenant_default_shims(state: &mut LuaState) -> LuaResult<()> {
    register_campaign_info(state)?;
    register_covenant_sanctum_ui(state)
}

fn register_campaign_info(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_CampaignInfo")?;
    table_set_rust_fn_static(state, namespace, "GetCampaignID", get_campaign_id)?;
    table_set_rust_fn_static(state, namespace, "GetCampaignInfo", get_campaign_info)?;
    table_set_rust_fn_static(state, namespace, "GetState", get_campaign_state)
}

fn register_covenant_sanctum_ui(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_CovenantSanctumUI")?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetRenownRewardsForLevel",
        get_renown_rewards_for_level,
    )?;
    table_set_rust_fn_static(state, namespace, "GetSoulCurrencies", return_empty_table)?;
    table_set_rust_fn_static(state, namespace, "GetAnimaInfo", get_anima_info)?;
    table_set_rust_fn_static(state, namespace, "CanDepositAnima", return_false)?;
    table_set_rust_fn_static(state, namespace, "DepositAnima", return_no_values)?;
    table_set_rust_fn_static(state, namespace, "EndInteraction", return_no_values)?;
    table_set_rust_fn_static(state, namespace, "GetFeatures", return_empty_table)?;
    table_set_rust_fn_static(state, namespace, "GetCurrentTalentTreeID", return_zero)
}

fn get_campaign_id(state: &mut LuaState) -> LuaResult<u32> {
    let campaign_id = campaign_id_arg(state, 1);
    state.push(Val::Num(campaign_id as f64));
    Ok(1)
}

fn get_campaign_info(state: &mut LuaState) -> LuaResult<u32> {
    let campaign_id = campaign_id_arg(state, 1);
    let name = campaign_name(state, campaign_id);
    let info = create_table_with_fields(
        state,
        &[
            ("campaignID", Val::Num(campaign_id as f64)),
            ("id", Val::Num(campaign_id as f64)),
            ("name", name),
        ],
    );
    state.push(info);
    Ok(1)
}

fn campaign_id_arg(state: &mut LuaState, index: i32) -> i32 {
    f64::from_stack(state, index).unwrap_or(0.0) as i32
}

fn campaign_name(state: &mut LuaState, campaign_id: i32) -> Val {
    if campaign_id == 290 {
        return create_string_static(state, "Broken Shore");
    }

    create_string(state, &format!("Campaign {campaign_id}"))
}

fn get_campaign_state(state: &mut LuaState) -> LuaResult<u32> {
    let state_value = campaign_state_invalid(state);
    state.push(state_value);
    Ok(1)
}

fn campaign_state_invalid(state: &mut LuaState) -> Val {
    let enum_table = global_val(state, "Enum");
    let campaign_state = table_get_static(state, enum_table, "CampaignState");
    match table_get_static(state, campaign_state, "Invalid") {
        Val::Nil => Val::Num(0.0),
        invalid => invalid,
    }
}

fn get_renown_rewards_for_level(state: &mut LuaState) -> LuaResult<u32> {
    let faction_id = f64::from_stack(state, 1).unwrap_or(0.0) as i32;
    let level = f64::from_stack(state, 2).unwrap_or(0.0) as i32;
    let rewards = create_table(state);

    if faction_id == 1 && level == 5 {
        let reward = create_path_of_ascension_reward(state);
        if let Val::Table(rewards_ref) = rewards {
            table_set_num(state, rewards_ref, 1.0, reward);
        }
    }

    state.push(rewards);
    Ok(1)
}

fn create_path_of_ascension_reward(state: &mut LuaState) -> Val {
    let name = create_string_static(state, "Path of Ascension");
    let description = create_string_static(state, "Unlocks a new covenant activity.");
    let toast_description = create_string_static(state, "Path of Ascension unlocked");
    create_table_with_fields(
        state,
        &[
            ("name", name),
            ("description", description),
            ("toastDescription", toast_description),
            ("icon", Val::Num(4_089_529.0)),
        ],
    )
}

fn return_empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn get_anima_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(2)
}

fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn return_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn return_no_values(_: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
