//! `C_Garrison` talent probes consumed by `Blizzard_AnimaDiversionUI`.
//!
//! Backs the two probes documented in
//! `vendor/wow-ui-source/Interface/AddOns/Blizzard_APIDocumentationGenerated/GarrisonInfoDocumentation.lua`:
//! `GetTalentInfo(talentID) → GarrisonTalentInfo` (line 314) and
//! `GetTalentUnlockWorldQuest(talentID) → number?` (line 413). The
//! struct shape mirrors `GarrisonTalentInfo` from
//! `GarrisonSharedDocumentation.lua` (lines 43-73), surfacing only the
//! fields the AnimaDiversion provider actually reads.
//!
//! Also registers the global helper `GetGarrisonTalentCostString` —
//! defined in `vendor/wow-ui-source/Interface/AddOns/Blizzard_FrameXMLUtil/Mainline/GarrisonBaseUtils.lua`
//! — which formats the per-currency cost row that
//! `AnimaDiversionDataProvider.lua:330` renders into the pin tooltip.

use super::{ensure_namespace, set_table_array};
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_get, table_set};
use crate::lua_api::state::GarrisonTalentInfo;
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_garrison_talent_surface(state: &mut LuaState) -> LuaResult<()> {
    let namespace = ensure_namespace(state, "C_Garrison")?;
    table_set_rust_fn_static(state, namespace, "GetTalentInfo", get_talent_info)?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetTalentUnlockWorldQuest",
        get_talent_unlock_world_quest,
    )?;
    table_set_rust_fn_static(
        state,
        namespace,
        "GetAllEncounterThreats",
        get_all_encounter_threats,
    )?;
    let globals = state.global;
    table_set_rust_fn_static(
        state,
        globals,
        "GetGarrisonTalentCostString",
        get_garrison_talent_cost_string,
    )?;
    Ok(())
}

fn get_all_encounter_threats(state: &mut LuaState) -> LuaResult<u32> {
    let empty = create_table(state);
    state.push(empty);
    Ok(1)
}

fn get_talent_info(state: &mut LuaState) -> LuaResult<u32> {
    let talent_id = i64::from_stack(state, 1)?;
    let talent = borrow_state(state)?
        .garrison_talents
        .talents
        .get(&talent_id)
        .cloned();
    let Some(info) = talent else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let entry = build_talent_info_table(state, &info);
    state.push(entry);
    Ok(1)
}

fn get_talent_unlock_world_quest(state: &mut LuaState) -> LuaResult<u32> {
    let talent_id = i64::from_stack(state, 1)?;
    let world_quest = borrow_state(state)?
        .garrison_talents
        .unlock_world_quests
        .get(&talent_id)
        .copied();
    match world_quest {
        Some(id) => state.push(Val::Num(id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn build_talent_info_table(state: &mut LuaState, info: &GarrisonTalentInfo) -> Val {
    let entry = create_table(state);
    set_identity_fields(state, entry, info);
    set_progression_fields(state, entry, info);
    set_research_fields(state, entry, info);
    let costs = build_currency_costs_table(state, info);
    table_set(state, entry, "researchCurrencyCosts", costs);
    entry
}

fn set_identity_fields(state: &mut LuaState, entry: Val, info: &GarrisonTalentInfo) {
    let name = create_string(state, &info.name);
    let description = create_string(state, &info.description);
    table_set(state, entry, "id", Val::Num(info.id as f64));
    table_set(state, entry, "name", name);
    table_set(state, entry, "description", description);
    table_set(state, entry, "icon", Val::Num(info.icon as f64));
    table_set(state, entry, "tier", Val::Num(info.tier as f64));
    table_set(state, entry, "uiOrder", Val::Num(info.ui_order as f64));
}

fn set_progression_fields(state: &mut LuaState, entry: Val, info: &GarrisonTalentInfo) {
    table_set(
        state,
        entry,
        "talentRank",
        Val::Num(info.talent_rank as f64),
    );
    table_set(
        state,
        entry,
        "talentMaxRank",
        Val::Num(info.talent_max_rank as f64),
    );
    table_set(
        state,
        entry,
        "isBeingResearched",
        Val::Bool(info.is_being_researched),
    );
    table_set(state, entry, "researched", Val::Bool(info.researched));
    table_set(state, entry, "selected", Val::Bool(info.selected));
}

fn set_research_fields(state: &mut LuaState, entry: Val, info: &GarrisonTalentInfo) {
    table_set(
        state,
        entry,
        "perkSpellID",
        Val::Num(info.perk_spell_id as f64),
    );
    table_set(
        state,
        entry,
        "talentAvailability",
        Val::Num(info.talent_availability as f64),
    );
    set_research_timing_fields(state, entry, info);
}

fn set_research_timing_fields(state: &mut LuaState, entry: Val, info: &GarrisonTalentInfo) {
    table_set(
        state,
        entry,
        "researchDuration",
        Val::Num(info.research_duration as f64),
    );
    table_set(state, entry, "startTime", Val::Num(info.start_time as f64));
    table_set(
        state,
        entry,
        "timeRemaining",
        Val::Num(info.time_remaining as f64),
    );
    table_set(
        state,
        entry,
        "researchGoldCost",
        Val::Num(info.research_gold_cost as f64),
    );
}

fn build_currency_costs_table(state: &mut LuaState, info: &GarrisonTalentInfo) -> Val {
    let costs = create_table(state);
    for (cost_index, cost) in info.research_currency_costs.iter().enumerate() {
        let cost_entry = create_table(state);
        table_set(
            state,
            cost_entry,
            "currencyType",
            Val::Num(cost.currency_type as f64),
        );
        table_set(
            state,
            cost_entry,
            "currencyQuantity",
            Val::Num(cost.currency_quantity as f64),
        );
        set_table_array(state, costs, cost_index as i64 + 1, cost_entry);
    }
    costs
}

fn get_garrison_talent_cost_string(state: &mut LuaState) -> LuaResult<u32> {
    let talent_info = stack_val(state, 1);
    let abbreviate_cost = bool::from_stack(state, 2).unwrap_or(false);
    let color_code = String::from_stack(state, 3).ok();
    let cost_rows = read_cost_rows(state, talent_info);
    if cost_rows.is_empty() {
        state.push(Val::Nil);
        return Ok(1);
    }
    let formatted = format_cost_rows(&cost_rows, abbreviate_cost, color_code.as_deref());
    let value = create_string(state, &formatted);
    state.push(value);
    Ok(1)
}

struct CostRow {
    currency_type: i64,
    currency_quantity: i64,
}

fn read_cost_rows(state: &mut LuaState, talent_info: Val) -> Vec<CostRow> {
    let costs = table_get(state, talent_info, "researchCurrencyCosts");
    if !matches!(costs, Val::Table(_)) {
        return Vec::new();
    }
    let mut rows = Vec::new();
    let mut index = 1i64;
    loop {
        let row = table_get_int(state, costs, index);
        if !matches!(row, Val::Table(_)) {
            break;
        }
        let currency_type = match table_get(state, row, "currencyType") {
            Val::Num(n) => n as i64,
            _ => 0,
        };
        let currency_quantity = match table_get(state, row, "currencyQuantity") {
            Val::Num(n) => n as i64,
            _ => 0,
        };
        rows.push(CostRow {
            currency_type,
            currency_quantity,
        });
        index += 1;
    }
    rows
}

fn table_get_int(state: &LuaState, table: Val, index: i64) -> Val {
    let Val::Table(table_ref) = table else {
        return Val::Nil;
    };
    state
        .gc
        .tables
        .get(table_ref)
        .map(|t| t.get_int(index))
        .unwrap_or(Val::Nil)
}

fn format_cost_rows(rows: &[CostRow], abbreviate_cost: bool, color_code: Option<&str>) -> String {
    let prefix = color_code.unwrap_or("");
    let suffix = if color_code.is_some() { "|r" } else { "" };
    rows.iter()
        .map(|row| {
            let quantity = if abbreviate_cost {
                abbreviate_quantity(row.currency_quantity)
            } else {
                row.currency_quantity.to_string()
            };
            format!(
                "{prefix}{quantity}|T{currency}:14|t{suffix}",
                currency = row.currency_type,
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn abbreviate_quantity(quantity: i64) -> String {
    const MILLION: i64 = 1_000_000;
    const THOUSAND: i64 = 1_000;
    if quantity >= MILLION {
        format!("{:.1}m", quantity as f64 / MILLION as f64)
    } else if quantity >= THOUSAND {
        format!("{:.1}k", quantity as f64 / THOUSAND as f64)
    } else {
        quantity.to_string()
    }
}
