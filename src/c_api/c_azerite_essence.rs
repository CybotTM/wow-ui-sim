//! `C_AzeriteEssence` — milestone + essence surface for the
//! Heart-of-Azeroth panel. Backed by `state.azerite_essence`.
//!
//! Methods (19 total):
//! - `GetMilestones() → table<MilestoneInfo>` — radial-layout entries
//! - `GetEssences() → table<EssenceInfo>` — right-side scroll list
//! - `GetEssenceInfo(essenceID) → EssenceInfo|nil`
//! - `GetMilestoneEssence(milestoneID) → essenceID|nil`
//! - `GetMilestoneInfo(milestoneID) → MilestoneInfo|nil`
//! - `GetMilestoneSpell(milestoneID) → spellID|nil`
//! - `GetNumUnlockedEssences() → number`
//! - `GetPendingActivationEssence() → essenceID|nil`
//! - `HasPendingActivationEssence() → bool`
//! - `SetPendingActivationEssence(essenceID)` — fires
//!   `PENDING_AZERITE_ESSENCE_CHANGED(prev, new)`
//! - `ClearPendingActivationEssence()` — same event with `new=nil`
//! - `ActivateEssence(essenceID, milestoneID) → bool` — fires
//!   `AZERITE_ESSENCE_ACTIVATED(essenceID, milestoneID)` then
//!   `AZERITE_ESSENCE_CHANGED(essenceID, rank)` on success
//! - `CanActivateEssence(essenceID, milestoneID) → bool`
//! - `CanOpenUI() → bool` — gates `AzeriteEssenceUIMixin:TryShow`
//! - `IsAtForge() → bool`
//! - `CloseForge()` — fires `AZERITE_ESSENCE_FORGE_CLOSE`
//! - `UnlockMilestone(milestoneID) → bool` — fires
//!   `AZERITE_ESSENCE_MILESTONE_UNLOCKED(milestoneID)`
//! - `HasNeverActivatedAnyEssences() → bool`
//! - `GetEssenceHyperlink(essenceID, rank) → string|nil`

use crate::c_api::ensure_namespace;
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, create_table_with_fields,
    table_set_num,
};
use crate::lua_api::state_types::{AzeriteEssenceInfo, AzeriteEssenceMilestoneInfo};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

type AzeriteEssenceMethod = fn(&mut LuaState) -> LuaResult<u32>;

const AZERITE_ESSENCE_METHODS: &[(&str, AzeriteEssenceMethod)] = &[
    ("GetMilestones", get_milestones),
    ("GetEssences", get_essences),
    ("GetEssenceInfo", get_essence_info),
    ("GetMilestoneEssence", get_milestone_essence),
    ("GetMilestoneInfo", get_milestone_info),
    ("GetMilestoneSpell", get_milestone_spell),
    ("GetNumUnlockedEssences", get_num_unlocked_essences),
    (
        "GetPendingActivationEssence",
        get_pending_activation_essence,
    ),
    (
        "HasPendingActivationEssence",
        has_pending_activation_essence,
    ),
    (
        "SetPendingActivationEssence",
        set_pending_activation_essence,
    ),
    (
        "ClearPendingActivationEssence",
        clear_pending_activation_essence,
    ),
    ("ActivateEssence", activate_essence),
    ("CanActivateEssence", can_activate_essence),
    ("CanOpenUI", can_open_ui),
    ("IsAtForge", is_at_forge),
    ("CloseForge", close_forge),
    ("UnlockMilestone", unlock_milestone),
    (
        "HasNeverActivatedAnyEssences",
        has_never_activated_any_essences,
    ),
    ("GetEssenceHyperlink", get_essence_hyperlink),
];

pub(crate) fn register_c_azerite_essence_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_AzeriteEssence")?;
    for &(name, func) in AZERITE_ESSENCE_METHODS {
        table_set_rust_fn_static(state, ns, name, func)?;
    }
    Ok(())
}

fn get_milestones(state: &mut LuaState) -> LuaResult<u32> {
    let milestones = borrow_state(state)?.azerite_essence.milestones.clone();
    let table_val = create_table(state);
    let Val::Table(table_ref) = table_val else {
        unreachable!("create_table must return a table");
    };
    for (index, milestone) in milestones.iter().enumerate() {
        let entry = build_milestone_table(state, milestone);
        table_set_num(state, table_ref, (index + 1) as f64, entry);
    }
    state.push(table_val);
    Ok(1)
}

fn get_essences(state: &mut LuaState) -> LuaResult<u32> {
    let ordered = ordered_essences(state)?;
    let table_val = create_table(state);
    let Val::Table(table_ref) = table_val else {
        unreachable!("create_table must return a table");
    };
    for (index, essence) in ordered.iter().enumerate() {
        let entry = build_essence_table(state, essence);
        table_set_num(state, table_ref, (index + 1) as f64, entry);
    }
    state.push(table_val);
    Ok(1)
}

fn ordered_essences(state: &LuaState) -> LuaResult<Vec<AzeriteEssenceInfo>> {
    let sim = borrow_state(state)?;
    let order = &sim.azerite_essence.essence_order;
    let map = &sim.azerite_essence.essences;
    let mut out = Vec::with_capacity(order.len());
    for id in order {
        if let Some(entry) = map.get(id) {
            out.push(entry.clone());
        }
    }
    Ok(out)
}

fn get_essence_info(state: &mut LuaState) -> LuaResult<u32> {
    let essence_id = i32::from_stack(state, 1)?;
    let Some(essence) = borrow_state(state)?
        .azerite_essence
        .essences
        .get(&essence_id)
        .cloned()
    else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let entry = build_essence_table(state, &essence);
    state.push(entry);
    Ok(1)
}

fn get_milestone_essence(state: &mut LuaState) -> LuaResult<u32> {
    let milestone_id = i32::from_stack(state, 1)?;
    let active = {
        borrow_state(state)?
            .azerite_essence
            .milestones
            .iter()
            .find(|m| m.id == milestone_id)
            .and_then(|m| m.active_essence_id)
    };
    state.push(optional_id_val(active));
    Ok(1)
}

fn get_milestone_info(state: &mut LuaState) -> LuaResult<u32> {
    let milestone_id = i32::from_stack(state, 1)?;
    let Some(milestone) = find_milestone(state, milestone_id)? else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let entry = build_milestone_table(state, &milestone);
    state.push(entry);
    Ok(1)
}

fn get_milestone_spell(state: &mut LuaState) -> LuaResult<u32> {
    let milestone_id = i32::from_stack(state, 1)?;
    let Some(milestone) = find_milestone(state, milestone_id)? else {
        state.push(Val::Nil);
        return Ok(1);
    };
    state.push(Val::Num(milestone.spell_id as f64));
    Ok(1)
}

fn get_num_unlocked_essences(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.azerite_essence.num_unlocked;
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn get_pending_activation_essence(state: &mut LuaState) -> LuaResult<u32> {
    let pending = borrow_state(state)?
        .azerite_essence
        .pending_activation_essence;
    state.push(optional_id_val(pending));
    Ok(1)
}

fn has_pending_activation_essence(state: &mut LuaState) -> LuaResult<u32> {
    let has_pending = borrow_state(state)?
        .azerite_essence
        .pending_activation_essence
        .is_some();
    state.push(Val::Bool(has_pending));
    Ok(1)
}

fn set_pending_activation_essence(state: &mut LuaState) -> LuaResult<u32> {
    let new_id = i32::from_stack(state, 1)?;
    let previous = {
        let mut sim = borrow_state_mut(state)?;
        let prev = sim.azerite_essence.pending_activation_essence;
        sim.azerite_essence.pending_activation_essence = Some(new_id);
        prev
    };
    fire_pending_changed(state, previous, Some(new_id))?;
    Ok(0)
}

fn clear_pending_activation_essence(state: &mut LuaState) -> LuaResult<u32> {
    let previous = {
        let mut sim = borrow_state_mut(state)?;
        let prev = sim.azerite_essence.pending_activation_essence;
        sim.azerite_essence.pending_activation_essence = None;
        prev
    };
    if previous.is_none() {
        return Ok(0);
    }
    fire_pending_changed(state, previous, None)?;
    Ok(0)
}

fn fire_pending_changed(
    state: &mut LuaState,
    previous: Option<i32>,
    new: Option<i32>,
) -> LuaResult<()> {
    let prev_val = optional_id_val(previous);
    let new_val = optional_id_val(new);
    dispatch_event_now(
        state,
        "PENDING_AZERITE_ESSENCE_CHANGED",
        &[prev_val, new_val],
    )
}

fn activate_essence(state: &mut LuaState) -> LuaResult<u32> {
    let essence_id = i32::from_stack(state, 1)?;
    let milestone_id = i32::from_stack(state, 2)?;
    let activation = try_activate_essence(state, essence_id, milestone_id)?;
    let Some(rank) = activation else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    dispatch_event_now(
        state,
        "AZERITE_ESSENCE_ACTIVATED",
        &[Val::Num(essence_id as f64), Val::Num(milestone_id as f64)],
    )?;
    dispatch_event_now(
        state,
        "AZERITE_ESSENCE_CHANGED",
        &[Val::Num(essence_id as f64), Val::Num(rank as f64)],
    )?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn try_activate_essence(
    state: &mut LuaState,
    essence_id: i32,
    milestone_id: i32,
) -> LuaResult<Option<i32>> {
    let mut sim = borrow_state_mut(state)?;
    let azerite = &mut sim.azerite_essence;
    let Some(essence_rank) = azerite.essences.get(&essence_id).map(|e| e.rank) else {
        return Ok(None);
    };
    let Some(milestone) = azerite.milestones.iter_mut().find(|m| m.id == milestone_id) else {
        return Ok(None);
    };
    if !milestone.unlocked {
        return Ok(None);
    }
    milestone.active_essence_id = Some(essence_id);
    if azerite.pending_activation_essence == Some(essence_id) {
        azerite.pending_activation_essence = None;
    }
    azerite.has_never_activated = false;
    if let Some(entry) = azerite.essences.get_mut(&essence_id) {
        entry.has_never_activated = false;
    }
    Ok(Some(essence_rank))
}

fn can_activate_essence(state: &mut LuaState) -> LuaResult<u32> {
    let essence_id = i32::from_stack(state, 1)?;
    let milestone_id = i32::from_stack(state, 2)?;
    let allowed = essence_activation_allowed(state, essence_id, milestone_id)?;
    state.push(Val::Bool(allowed));
    Ok(1)
}

fn essence_activation_allowed(
    state: &LuaState,
    essence_id: i32,
    milestone_id: i32,
) -> LuaResult<bool> {
    let sim = borrow_state(state)?;
    if sim.player.in_combat {
        return Ok(false);
    }
    let Some(essence) = sim.azerite_essence.essences.get(&essence_id) else {
        return Ok(false);
    };
    if !essence.unlocked {
        return Ok(false);
    }
    let Some(milestone) = sim
        .azerite_essence
        .milestones
        .iter()
        .find(|m| m.id == milestone_id)
    else {
        return Ok(false);
    };
    Ok(milestone.unlocked)
}

fn can_open_ui(state: &mut LuaState) -> LuaResult<u32> {
    let allowed = borrow_state(state)?.azerite_essence.has_neck_equipped;
    state.push(Val::Bool(allowed));
    Ok(1)
}

fn is_at_forge(state: &mut LuaState) -> LuaResult<u32> {
    let at_forge = borrow_state(state)?.azerite_essence.is_at_forge;
    state.push(Val::Bool(at_forge));
    Ok(1)
}

fn close_forge(state: &mut LuaState) -> LuaResult<u32> {
    {
        let mut sim = borrow_state_mut(state)?;
        sim.azerite_essence.is_at_forge = false;
    }
    dispatch_event_now(state, "AZERITE_ESSENCE_FORGE_CLOSE", &[])?;
    Ok(0)
}

fn unlock_milestone(state: &mut LuaState) -> LuaResult<u32> {
    let milestone_id = i32::from_stack(state, 1)?;
    let unlocked = {
        let mut sim = borrow_state_mut(state)?;
        match sim
            .azerite_essence
            .milestones
            .iter_mut()
            .find(|m| m.id == milestone_id)
        {
            Some(m) => {
                m.unlocked = true;
                m.can_unlock = false;
                true
            }
            None => false,
        }
    };
    if !unlocked {
        state.push(Val::Bool(false));
        return Ok(1);
    }
    dispatch_event_now(
        state,
        "AZERITE_ESSENCE_MILESTONE_UNLOCKED",
        &[Val::Num(milestone_id as f64)],
    )?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn has_never_activated_any_essences(state: &mut LuaState) -> LuaResult<u32> {
    let never = borrow_state(state)?.azerite_essence.has_never_activated;
    state.push(Val::Bool(never));
    Ok(1)
}

fn get_essence_hyperlink(state: &mut LuaState) -> LuaResult<u32> {
    let essence_id = i32::from_stack(state, 1)?;
    let rank = i32::from_stack(state, 2)?;
    let Some(essence) = borrow_state(state)?
        .azerite_essence
        .essences
        .get(&essence_id)
        .cloned()
    else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let link = format!(
        "|cffa335ee|Hazessence:{}:{}|h[{}]|h|r",
        essence_id, rank, essence.name
    );
    let link_val = create_string(state, &link);
    state.push(link_val);
    Ok(1)
}

fn find_milestone(
    state: &LuaState,
    milestone_id: i32,
) -> LuaResult<Option<AzeriteEssenceMilestoneInfo>> {
    Ok(borrow_state(state)?
        .azerite_essence
        .milestones
        .iter()
        .find(|m| m.id == milestone_id)
        .cloned())
}

fn build_milestone_table(state: &mut LuaState, milestone: &AzeriteEssenceMilestoneInfo) -> Val {
    create_table_with_fields(
        state,
        &[
            ("ID", Val::Num(milestone.id as f64)),
            ("requiredLevel", Val::Num(milestone.required_level as f64)),
            ("slot", optional_id_val(milestone.slot)),
            ("unlocked", Val::Bool(milestone.unlocked)),
            ("canUnlock", Val::Bool(milestone.can_unlock)),
            ("isMajorSlot", Val::Bool(milestone.is_major_slot)),
            ("swirlScale", Val::Num(milestone.swirl_scale as f64)),
            ("requiresOnlyAura", Val::Bool(milestone.requires_only_aura)),
            ("spellID", Val::Num(milestone.spell_id as f64)),
            ("rank", optional_id_val(milestone.rank)),
        ],
    )
}

fn build_essence_table(state: &mut LuaState, essence: &AzeriteEssenceInfo) -> Val {
    let name_val = create_string(state, &essence.name);
    create_table_with_fields(
        state,
        &[
            ("ID", Val::Num(essence.id as f64)),
            ("name", name_val),
            ("rank", Val::Num(essence.rank as f64)),
            ("icon", Val::Num(essence.icon as f64)),
            ("unlocked", Val::Bool(essence.unlocked)),
            ("valid", Val::Bool(essence.valid)),
            ("accessRank", Val::Num(essence.access_rank as f64)),
            ("hasNeverActivated", Val::Bool(essence.has_never_activated)),
        ],
    )
}

fn optional_id_val(value: Option<i32>) -> Val {
    match value {
        Some(n) => Val::Num(n as f64),
        None => Val::Nil,
    }
}
