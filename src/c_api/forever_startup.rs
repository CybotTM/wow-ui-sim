//! Host-supplied Forever inputs. These are simulator fixtures, not native defaults.

use std::collections::{HashMap, HashSet};

use crate::c_api::helpers::ensure_namespace;
use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table_with_fields, table_set_num,
};
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
    pub party_ping_restriction: u8,
    pub purchasable_bank_types: HashSet<i32>,
    pub bank_locked_reasons: HashMap<i32, i32>,
    pub max_bank_tab_types: HashSet<i32>,
    pub next_bank_tabs: HashMap<i32, PurchasableBankTab>,
    pub enabled_tts_settings: HashSet<i32>,
    pub match_complete: bool,
    pub target_spell_id: Option<i32>,
    pub character_tts_settings_saved: bool,
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
    register_character_queries(state)?;
    register_services(state)?;
    let cvar = ensure_namespace(state, "C_CVar")?;
    // Forever 70532 CVarDocumentation.lua:171 and :114; both return no values.
    table_set_rust_fn_static(state, cvar, "SetTempCVar", set_temporary_cvar)?;
    table_set_rust_fn_static(state, cvar, "RemoveTempCVar", remove_temporary_cvar)
}

fn set_temporary_cvar(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    let value = match crate::lua_bridge::stack_val(state, 2) {
        Val::Nil => {
            return Err(rilua::runtime_error(
                "SetTempCVar: unmodelled nil-value case; native semantics are unknown",
            ));
        }
        // Original Gamepad/Core.lua supplies numbers to the cstring argument.
        Val::Num(number) => number.to_string(),
        _ => String::from_stack(state, 2)?,
    };
    borrow_state(state)?.cvars.set_temporary(&name, &value);
    Ok(0)
}

fn remove_temporary_cvar(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    borrow_state(state)?.cvars.remove_temporary(&name);
    Ok(0)
}

/// Lua and Rust current-value readers share the same effective CVar storage.
pub(crate) fn read_current_cvar(state: &LuaState, name: &str) -> LuaResult<Option<String>> {
    Ok(borrow_state(state)?.cvars.get(name))
}

fn register_services(state: &mut LuaState) -> LuaResult<()> {
    let discord = ensure_namespace(state, "C_Discord")?;
    table_set_rust_fn_static(state, discord, "IsEnabled", discord_enabled)?;
    let ping = ensure_namespace(state, "C_Ping")?;
    table_set_rust_fn_static(state, ping, "IsPingSystemEnabled", ping_enabled)?;
    let party = ensure_namespace(state, "C_PartyInfo")?;
    table_set_rust_fn_static(state, party, "GetRestrictPings", party_ping_restriction)?;
    let housing = ensure_namespace(state, "C_Housing")?;
    table_set_rust_fn_static(state, housing, "IsInsideHouseOrPlot", inside_house_or_plot)?;
    register_tts_settings(state)
}

fn register_tts_settings(state: &mut LuaState) -> LuaResult<()> {
    let tts = ensure_namespace(state, "C_TTSSettings")?;
    table_set_rust_fn_static(state, tts, "GetSetting", tts_bool_setting)?;
    table_set_rust_fn_static(
        state,
        tts,
        "GetCharacterSettingsSaved",
        character_tts_settings_saved,
    )?;
    table_set_rust_fn_static(
        state,
        tts,
        "MarkCharacterSettingsSaved",
        mark_character_tts_settings_saved,
    )
}

fn discord_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = borrow_state(state)?.cvars.get_bool("discordClientEnabled");
    state.push(Val::Bool(enabled));
    Ok(1)
}

fn ping_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = borrow_state(state)?.forever_startup.ping_enabled;
    state.push(Val::Bool(enabled));
    Ok(1)
}

fn party_ping_restriction(state: &mut LuaState) -> LuaResult<u32> {
    let restriction = borrow_state(state)?.forever_startup.party_ping_restriction;
    state.push(Val::Num(f64::from(restriction)));
    Ok(1)
}

fn tts_bool_setting(state: &mut LuaState) -> LuaResult<u32> {
    let setting = i32::from_stack(state, 1)?;
    let enabled = borrow_state(state)?
        .forever_startup
        .enabled_tts_settings
        .contains(&setting);
    state.push(Val::Bool(enabled));
    Ok(1)
}

fn inside_house_or_plot(state: &mut LuaState) -> LuaResult<u32> {
    let inside = borrow_state(state)?.housing.current_housing_area.is_some();
    state.push(Val::Bool(inside));
    Ok(1)
}

fn character_tts_settings_saved(state: &mut LuaState) -> LuaResult<u32> {
    let saved = borrow_state(state)?
        .forever_startup
        .character_tts_settings_saved;
    state.push(Val::Bool(saved));
    Ok(1)
}

fn mark_character_tts_settings_saved(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?
        .forever_startup
        .character_tts_settings_saved = true;
    Ok(0)
}

fn register_character_queries(state: &mut LuaState) -> LuaResult<()> {
    LuaApiMut::register_function(state, "IsCursorVisible", cursor_visible)?;
    let bank = ensure_namespace(state, "C_Bank")?;
    table_set_rust_fn_static(state, bank, "CanPurchaseBankTab", can_purchase_bank_tab)?;
    table_set_rust_fn_static(state, bank, "FetchBankLockedReason", bank_locked_reason)?;
    table_set_rust_fn_static(state, bank, "HasMaxBankTabs", has_max_bank_tabs)?;
    table_set_rust_fn_static(
        state,
        bank,
        "FetchNextPurchasableBankTabData",
        next_bank_tab,
    )?;
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

fn next_bank_tab(state: &mut LuaState) -> LuaResult<u32> {
    let bank_type = i32::from_stack(state, 1)?;
    let offer = borrow_state(state)?
        .forever_startup
        .next_bank_tabs
        .get(&bank_type)
        .cloned();
    let value = offer
        .as_ref()
        .map_or(Val::Nil, |offer| create_bank_offer_table(state, offer));
    state.push(value);
    Ok(1)
}

fn create_bank_offer_table(state: &mut LuaState, offer: &PurchasableBankTab) -> Val {
    let title = create_string(state, &offer.purchase_prompt_title);
    let body = create_string(state, &offer.purchase_prompt_body);
    let confirmation = create_string(state, &offer.purchase_prompt_confirmation);
    create_table_with_fields(
        state,
        &[
            ("tabCost", Val::Num(offer.tab_cost as f64)),
            ("canAfford", Val::Bool(offer.can_afford)),
            ("purchasePromptTitle", title),
            ("purchasePromptBody", body),
            ("purchasePromptConfirmation", confirmation),
        ],
    )
}

fn has_max_bank_tabs(state: &mut LuaState) -> LuaResult<u32> {
    let bank_type = i32::from_stack(state, 1)?;
    let has_max = borrow_state(state)?
        .forever_startup
        .max_bank_tab_types
        .contains(&bank_type);
    state.push(Val::Bool(has_max));
    Ok(1)
}

fn bank_locked_reason(state: &mut LuaState) -> LuaResult<u32> {
    let bank_type = i32::from_stack(state, 1)?;
    let reason = borrow_state(state)?
        .forever_startup
        .bank_locked_reasons
        .get(&bank_type)
        .copied();
    state.push(reason.map_or(Val::Nil, |reason| Val::Num(f64::from(reason))));
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
                | "SOCIAL_UI_RAID_LIST_SYSTEM_STATUS_UPDATED"
        )
}

fn ammo_needed(state: &mut LuaState) -> LuaResult<u32> {
    let required = borrow_state(state)?.forever_startup.ammo_required;
    state.push(Val::Bool(required));
    Ok(1)
}

/// Supplied 70532 PurchasableBankTabData; no price or affordability policy is inferred.
#[derive(Debug, Clone)]
pub struct PurchasableBankTab {
    pub tab_cost: u64,
    pub can_afford: bool,
    pub purchase_prompt_title: String,
    pub purchase_prompt_body: String,
    pub purchase_prompt_confirmation: String,
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
