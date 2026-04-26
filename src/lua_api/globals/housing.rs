//! `C_Housing` namespace — housing service flag plus the favor-bar
//! surface read by `Blizzard_ActionBar/Mainline/HouseFavorBar.lua`.
//!
//! MainMenuBarMicroButtons gates the Housing micro-button on the service
//! probe. The sim defaults this to `true` so the live UI can open the Housing
//! dashboard. Admin `A_Admin.SetHousingServiceEnabled(b?)` flips the flag for
//! tests that need disabled-service behavior.
//!
//! Favor-bar lookups (`GetTrackedHouseGuid`, `GetCurrentHouseLevelFavor`,
//! `GetHouseLevelFavorForLevel`, `GetMaxHouseLevel`) are driven by
//! `state.housing` — `HouseFavorBarMixin:Update` reads all four to populate
//! the bar.
//!
//! `C_Housing` namespace table is provided by the Lua bootstrap
//! `__wow_merge_namespace` so other unimplemented members still fall through
//! to the no-op metamethod.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, create_table};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::closure::RustFn;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

const HOUSING_METHODS: &[(&str, RustFn)] = &[
    ("IsHousingServiceEnabled", is_housing_service_enabled),
    ("GetMaxHouseLevel", get_max_house_level),
    ("GetVisitCooldownInfo", get_visit_cooldown_info),
    ("HasHousingExpansionAccess", has_housing_expansion_access),
    ("GetTrackedHouseGuid", get_tracked_house_guid),
    ("GetCurrentHouseLevelFavor", get_current_house_level_favor),
    (
        "GetHouseLevelFavorForLevel",
        get_house_level_favor_for_level,
    ),
];

pub fn is_housing_service_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = borrow_state(state)?.housing_service_enabled;
    state.push(Val::Bool(enabled));
    Ok(1)
}

/// `C_Housing.GetMaxHouseLevel` — `HouseFavorBarMixin:OnLoad` reads this in
/// the `IsMaxLevel` override to decide whether the bar should clamp at the
/// current level. Drives from `state.housing.max_level` (default `0`).
pub fn get_max_house_level(state: &mut LuaState) -> LuaResult<u32> {
    let max_level = borrow_state(state)?.housing.max_level;
    state.push(Val::Num(max_level as f64));
    Ok(1)
}

/// `C_Housing.GetVisitCooldownInfo` — dashboard teleport button probes this.
/// No visit cooldown is simulated yet.
pub fn get_visit_cooldown_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

/// `C_Housing.HasHousingExpansionAccess` — gates the Midnight housing
/// dashboard. Blizzard_HousingDashboard/Blizzard_HousingDashboardHouseInfoContent
/// blocks dashboard access when this returns false; the sim grants access so
/// the dashboard renders populated.
pub fn has_housing_expansion_access(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

/// `C_Housing.GetTrackedHouseGuid()` — returns the GUID string of the house
/// the favor bar should display. Returns nil when no house is tracked, which
/// keeps `HouseFavorBarMixin:OnShow`'s pre-fetch a no-op.
pub fn get_tracked_house_guid(state: &mut LuaState) -> LuaResult<u32> {
    let guid = borrow_state(state)?.housing.tracked_house_guid.clone();
    match guid {
        Some(guid) => {
            let value = create_string(state, &guid);
            state.push(value);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

/// `C_Housing.GetCurrentHouseLevelFavor(houseGuid)` — returns the
/// `(currentLevel, currentFavor, nextLevelThreshold)` triple for the tracked
/// house. Lookup is keyed by the supplied GUID matching
/// `state.housing.tracked_house_guid`; mismatches return three zeros so the
/// bar treats the house as untracked.
pub fn get_current_house_level_favor(state: &mut LuaState) -> LuaResult<u32> {
    let requested = Option::<String>::from_stack(state, 1)?;
    let housing = borrow_state(state)?.housing.clone();
    let matches_tracked = matches!(
        (&requested, &housing.tracked_house_guid),
        (Some(req), Some(tracked)) if req == tracked
    );
    let (level, favor, threshold) = if matches_tracked {
        (
            housing.current_level,
            housing.current_favor,
            housing.next_threshold,
        )
    } else {
        (0, 0, 0)
    };
    state.push(Val::Num(level as f64));
    state.push(Val::Num(favor as f64));
    state.push(Val::Num(threshold as f64));
    Ok(3)
}

/// `C_Housing.GetHouseLevelFavorForLevel(level)` — returns the favor
/// threshold for the given level. `HouseFavorBarMixin:Update` calls this
/// twice (current level + next level) to derive the bar's min/max. Levels
/// outside the configured table return `0`, the sentinel the mixin uses to
/// skip `SetBarValues`.
pub fn get_house_level_favor_for_level(state: &mut LuaState) -> LuaResult<u32> {
    let threshold = level_threshold_from_stack(state, 1);
    state.push(Val::Num(threshold as f64));
    Ok(1)
}

fn level_threshold_from_stack(state: &LuaState, index: i32) -> i64 {
    let Val::Num(level) = stack_val(state, index) else {
        return 0;
    };
    if level < 1.0 {
        return 0;
    }
    let zero_indexed = (level as usize).saturating_sub(1);
    let Ok(thresholds) = borrow_state(state) else {
        return 0;
    };
    thresholds
        .housing
        .level_thresholds
        .get(zero_indexed)
        .copied()
        .unwrap_or(0)
}

fn ensure_c_housing_table(state: &mut LuaState) -> GcRef<Table> {
    let key = state.gc.intern_string_static(b"C_Housing");
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(r)) = existing {
        return r;
    }
    let new_val = create_table(state);
    let Val::Table(new_ref) = new_val else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), new_val, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    new_ref
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    let table_ref = ensure_c_housing_table(state);
    for &(name, func) in HOUSING_METHODS {
        table_set_rust_fn_static(state, table_ref, name, func)?;
    }
    Ok(())
}

/// `A_Admin.SetHousingServiceEnabled(enabled?)` — missing arg defaults to
/// `true` so `A_Admin.SetHousingServiceEnabled()` opens housing.
pub fn admin_set_housing_service_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = Option::<bool>::from_stack(state, 1)?.unwrap_or(true);
    borrow_state_mut(state)?.housing_service_enabled = enabled;
    Ok(0)
}
