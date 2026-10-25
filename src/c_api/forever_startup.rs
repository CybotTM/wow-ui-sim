//! Host-supplied Forever inputs. These are simulator fixtures, not native defaults.

use std::collections::{HashMap, HashSet};

use crate::c_api::helpers::ensure_namespace;
use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::{borrow_state, create_table_with_fields, table_set_num};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

/// Explicit inputs for 70532 consumers whose native policy is not established.
/// Empty collections mean the host supplied no entries; false flags describe
/// the empty simulator world, not an inferred native account or character.
#[derive(Debug, Default)]
pub struct StartupInputs {
    pub ammo_required: bool,
    pub cursor_visible: bool,
    pub ping_enabled: bool,
    pub purchasable_bank_types: HashSet<i32>,
    pub match_complete: bool,
    pub target_spell_id: Option<i32>,
    pub character_tts_settings_saved: bool,
    pub temporary_cvars: HashMap<String, String>,
    pub external_event: Option<ExternalEvent>,
    pub group_buff_alerts: Vec<GroupBuffAlert>,
}

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    if ACTIVE != ClientProfile::WowForever {
        return Ok(());
    }
    let paper_doll = ensure_namespace(state, "C_PaperDollInfo")?;
    table_set_rust_fn_static(state, paper_doll, "AmmoNeeded", ammo_needed)?;
    register_group_buffs(state)?;
    register_external_event(state)?;
    register_character_queries(state)
}

fn register_character_queries(state: &mut LuaState) -> LuaResult<()> {
    LuaApiMut::register_function(state, "IsCursorVisible", cursor_visible)?;
    let bank = ensure_namespace(state, "C_Bank")?;
    table_set_rust_fn_static(state, bank, "CanPurchaseBankTab", can_purchase_bank_tab)?;
    let pvp = ensure_namespace(state, "C_PvP")?;
    table_set_rust_fn_static(state, pvp, "IsMatchComplete", match_complete)?;
    let spell = ensure_namespace(state, "C_Spell")?;
    table_set_rust_fn_static(state, spell, "GetTargetSpellID", target_spell_id)
}

fn cursor_visible(state: &mut LuaState) -> LuaResult<u32> {
    let visible = borrow_state(state)?.forever_startup.cursor_visible;
    state.push(Val::Bool(visible));
    Ok(1)
}

fn can_purchase_bank_tab(state: &mut LuaState) -> LuaResult<u32> {
    let bank_type = i32::from_stack(state, 1)?;
    let allowed = borrow_state(state)?
        .forever_startup
        .purchasable_bank_types
        .contains(&bank_type);
    state.push(Val::Bool(allowed));
    Ok(1)
}

fn match_complete(state: &mut LuaState) -> LuaResult<u32> {
    let complete = borrow_state(state)?.forever_startup.match_complete;
    state.push(Val::Bool(complete));
    Ok(1)
}

fn target_spell_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = borrow_state(state)?.forever_startup.target_spell_id;
    let id = id.ok_or_else(|| rilua::runtime_error(
        "native-unknown: C_Spell.GetTargetSpellID has no supplied target or documented no-target value"
    ))?;
    state.push(Val::Num(f64::from(id)));
    Ok(1)
}

fn register_external_event(state: &mut LuaState) -> LuaResult<()> {
    let external = ensure_namespace(state, "C_ExternalEventURL")?;
    table_set_rust_fn_static(state, external, "CanShow", external_can_show)?;
    table_set_rust_fn_static(state, external, "HasURL", external_has_url)?;
    table_set_rust_fn_static(state, external, "IsNew", external_is_new)
}

fn external_can_show(state: &mut LuaState) -> LuaResult<u32> {
    push_external_flag(state, |event| event.can_show)
}

fn external_has_url(state: &mut LuaState) -> LuaResult<u32> {
    push_external_flag(state, |event| !event.url.is_empty())
}

fn external_is_new(state: &mut LuaState) -> LuaResult<u32> {
    push_external_flag(state, |event| event.is_new)
}

fn push_external_flag(state: &mut LuaState, query: fn(&ExternalEvent) -> bool) -> LuaResult<u32> {
    let value = borrow_state(state)?
        .forever_startup
        .external_event
        .as_ref()
        .is_some_and(query);
    state.push(Val::Bool(value));
    Ok(1)
}

fn register_group_buffs(state: &mut LuaState) -> LuaResult<()> {
    let auras = ensure_namespace(state, "C_UnitAuras")?;
    table_set_rust_fn_static(state, auras, "GetGroupBuffVisualAlerts", group_buff_alerts)
}

fn group_buff_alerts(state: &mut LuaState) -> LuaResult<u32> {
    let alerts = borrow_state(state)?
        .forever_startup
        .group_buff_alerts
        .clone();
    let rows = state.gc.alloc_table(rilua::vm::table::Table::new());
    for (index, alert) in alerts.iter().enumerate() {
        let row = create_table_with_fields(
            state,
            &[
                ("spellID", Val::Num(f64::from(alert.spell_id))),
                ("visualValue", Val::Num(f64::from(alert.visual_value))),
            ],
        );
        table_set_num(state, rows, (index + 1) as f64, row);
    }
    state.push(Val::Table(rows));
    Ok(1)
}

/// 70532 APIDocumentation event names; registration is not event generation.
pub(crate) fn accepts_event(name: &str) -> bool {
    ACTIVE == ClientProfile::WowForever
        && matches!(
            name,
            "SECURE_TRANSFER_CONFIRM_TAKE_MAIL_MONEY"
                | "SHARD_TRANSFER_CANCELLED"
                | "EXTERNAL_EVENT_LAUNCH_URL_RESULT"
                | "EXTERNAL_EVENT_URL_CHANGED"
                | "SOCIAL_UI_SOCIAL_QUEUE_SYSTEM_STATUS_UPDATED"
        )
}

fn ammo_needed(state: &mut LuaState) -> LuaResult<u32> {
    let required = borrow_state(state)?.forever_startup.ammo_required;
    state.push(Val::Bool(required));
    Ok(1)
}

#[derive(Debug)]
pub struct ExternalEvent {
    pub url: String,
    pub can_show: bool,
    pub is_new: bool,
}

#[derive(Debug, Clone)]
pub struct GroupBuffAlert {
    pub spell_id: i32,
    pub visual_value: i32,
}
