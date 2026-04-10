//! UI-related C_* namespace API stubs.
//!
//! Contains C_ namespaces for UI systems:
//! - C_VignetteInfo, C_AreaPoiInfo, C_PlayerChoice, C_MajorFactions
//! - C_UIWidgetManager, C_GossipInfo, C_Calendar, C_CovenantCallings
//! - C_CovenantSanctumUI, C_WeeklyRewards, C_ContributionCollector, C_Scenario, C_Housing
//! - C_GameRules, C_ScriptedAnimations, C_Glue, C_UIColor, C_ClassColor
//! - C_SpecializationInfo, C_SuperTrack
//! - C_PlayerInteractionManager, C_PaperDollInfo, C_PerksProgram

use mlua::{Lua, MultiValue, Result, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub(super) fn register_all(lua: &Lua, state: Rc<RefCell<crate::lua_api::SimState>>) -> Result<()> {
    register_c_vignette_info(lua)?;
    register_c_area_poi(lua)?;
    register_c_player_choice(lua)?;
    register_c_major_factions(lua)?;
    register_c_ui_widget(lua)?;
    register_c_gossip_info(lua)?;
    register_c_calendar(lua)?;
    register_c_covenant_callings(lua)?;
    register_c_weekly_rewards(lua, Rc::clone(&state))?;
    register_c_contribution_collector(lua)?;
    register_c_scenario(lua)?;
    register_c_housing(lua)?;
    register_c_game_rules(lua)?;
    register_c_scripted_animations(lua)?;
    register_c_glue(lua)?;
    super::c_misc_api_ui_player::register_all(lua, state)?;
    Ok(())
}

fn register_c_vignette_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetVignettes",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetVignetteInfo",
        lua.create_function(|_, _g: String| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetVignettePosition",
        lua.create_function(|_, (_g, _m): (String, Option<i32>)| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetVignetteGUID",
        lua.create_function(|_, _g: String| Ok(Value::Nil))?,
    )?;
    lua.globals().set("C_VignetteInfo", t)?;
    Ok(())
}

fn register_c_area_poi(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetAreaPOIInfo",
        lua.create_function(|_, (_m, _id): (i32, i32)| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetAreaPOISecondsLeft",
        lua.create_function(|_, _id: i32| Ok(0i32))?,
    )?;
    t.set(
        "IsAreaPOITimed",
        lua.create_function(|_, _id: i32| Ok(false))?,
    )?;
    t.set(
        "GetAreaPOIForMap",
        lua.create_function(|lua, _m: i32| lua.create_table())?,
    )?;
    lua.globals().set("C_AreaPoiInfo", t)?;
    Ok(())
}

fn register_c_player_choice(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetCurrentPlayerChoiceInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetNumPlayerChoices",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetPlayerChoiceInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetPlayerChoiceOptionInfo",
        lua.create_function(|_, (_c, _o): (i32, i32)| Ok(Value::Nil))?,
    )?;
    t.set(
        "SendPlayerChoiceResponse",
        lua.create_function(|_, _id: i32| Ok(()))?,
    )?;
    t.set(
        "IsWaitingForPlayerChoiceResponse",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    lua.globals().set("C_PlayerChoice", t)?;
    Ok(())
}

fn register_c_major_factions(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetMajorFactionData",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetMajorFactionIDs",
        lua.create_function(|lua, _e: Option<i32>| lua.create_table())?,
    )?;
    t.set(
        "GetRenownLevels",
        lua.create_function(|lua, _id: i32| lua.create_table())?,
    )?;
    t.set(
        "GetCurrentRenownLevel",
        lua.create_function(|_, _id: i32| Ok(0i32))?,
    )?;
    t.set(
        "HasMaximumRenown",
        lua.create_function(|_, _id: i32| Ok(false))?,
    )?;
    t.set(
        "GetRenownRewardsForLevel",
        lua.create_function(|lua, (_f, _l): (i32, i32)| lua.create_table())?,
    )?;
    lua.globals().set("C_MajorFactions", t)?;
    Ok(())
}

fn register_c_ui_widget(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    register_c_ui_widget_queries(lua, &t)?;
    register_c_ui_widget_visualizations(lua, &t)?;
    register_c_ui_widget_set_ids(lua, &t)?;
    lua.globals().set("C_UIWidgetManager", t)?;
    Ok(())
}

fn register_c_ui_widget_queries(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetAllWidgetsBySetID",
        lua.create_function(|lua, _id: i32| lua.create_table())?,
    )?;
    table.set(
        "GetWidgetSetInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    Ok(())
}

fn register_c_ui_widget_visualizations(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetStatusBarWidgetVisualizationInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    table.set(
        "GetTextWithStateWidgetVisualizationInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    table.set(
        "GetIconAndTextWidgetVisualizationInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    table.set(
        "GetCaptureBarWidgetVisualizationInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    table.set(
        "GetDoubleStatusBarWidgetVisualizationInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    table.set(
        "GetSpellDisplayVisualizationInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    Ok(())
}

fn register_c_ui_widget_set_ids(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetTopCenterWidgetSetID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    table.set(
        "GetBelowMinimapWidgetSetID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    table.set(
        "GetObjectiveTrackerWidgetSetID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    Ok(())
}

fn make_friendship_reputation_info(lua: &Lua) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    info.set("friendshipFactionID", 0)?;
    info.set("standing", 0)?;
    info.set("maxRep", 0)?;
    info.set("name", Value::Nil)?;
    info.set("text", Value::Nil)?;
    info.set("texture", Value::Nil)?;
    info.set("reaction", Value::Nil)?;
    info.set("reactionThreshold", 0)?;
    info.set("nextThreshold", Value::Nil)?;
    Ok(info)
}

fn register_c_gossip_info(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    register_c_gossip_basic_methods(lua, &t)?;
    register_c_gossip_quest_methods(lua, &t)?;
    register_c_gossip_friendship_methods(lua, &t)?;
    register_c_gossip_poi_methods(lua, &t)?;
    lua.globals().set("C_GossipInfo", t)?;
    Ok(())
}

fn register_c_gossip_basic_methods(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set("GetNumOptions", lua.create_function(|_, ()| Ok(0i32))?)?;
    table.set(
        "GetOptions",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    table.set("GetText", lua.create_function(|_, ()| Ok(""))?)?;
    table.set(
        "SelectOption",
        lua.create_function(|_, (_id, _t, _c): (i32, Option<String>, Option<bool>)| Ok(()))?,
    )?;
    table.set("CloseGossip", lua.create_function(|_, ()| Ok(()))?)?;
    table.set("ForceGossip", lua.create_function(|_, ()| Ok(false))?)?;
    Ok(())
}

fn register_c_gossip_quest_methods(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set("GetNumActiveQuests", lua.create_function(|_, ()| Ok(0i32))?)?;
    table.set(
        "GetNumAvailableQuests",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    table.set(
        "GetActiveQuests",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    table.set(
        "GetAvailableQuests",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    table.set(
        "SelectActiveQuest",
        lua.create_function(|_, _i: i32| Ok(()))?,
    )?;
    table.set(
        "SelectAvailableQuest",
        lua.create_function(|_, _i: i32| Ok(()))?,
    )?;
    Ok(())
}

fn register_c_gossip_friendship_methods(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetFriendshipReputation",
        lua.create_function(|lua, _fid: Option<i32>| make_friendship_reputation_info(lua))?,
    )?;
    table.set(
        "GetFriendshipReputationRanks",
        lua.create_function(|lua, _fid: Option<i32>| {
            let info = lua.create_table()?;
            info.set("currentLevel", 0)?;
            info.set("maxLevel", 0)?;
            Ok(info)
        })?,
    )?;
    Ok(())
}

fn register_c_gossip_poi_methods(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetPoiForUiMapID",
        lua.create_function(|_, _map_id: i32| Ok(Value::Nil))?,
    )?;
    table.set(
        "GetPoiInfo",
        lua.create_function(|_, (_map_id, _poi_id): (i32, i32)| Ok(Value::Nil))?,
    )?;
    Ok(())
}

fn register_c_calendar(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    register_c_calendar_queries(lua, &t)?;
    register_c_calendar_day_events(lua, &t)?;
    register_c_calendar_actions(lua, &t)?;
    lua.globals().set("C_Calendar", t)?;
    Ok(())
}

fn register_c_calendar_queries(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetDate",
        lua.create_function(|_, ()| Ok((1i32, 1i32, 1i32, 2024i32)))?,
    )?;
    table.set("GetMonthInfo", create_get_calendar_month_info_fn(lua)?)?;
    table.set(
        "GetMinDate",
        lua.create_function(|_, ()| Ok((1i32, 1i32, 2004i32)))?,
    )?;
    table.set(
        "GetMaxDate",
        lua.create_function(|_, ()| Ok((12i32, 31i32, 2030i32)))?,
    )?;
    table.set(
        "GetNumPendingInvites",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    Ok(())
}

fn create_get_calendar_month_info_fn(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|lua, _o: Option<i32>| {
        let info = lua.create_table()?;
        info.set("month", 1)?;
        info.set("year", 2024)?;
        info.set("numDays", 31)?;
        info.set("firstWeekday", 1)?;
        Ok(info)
    })
}

fn register_c_calendar_day_events(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set(
        "GetNumDayEvents",
        lua.create_function(|_, (_o, _d): (i32, i32)| Ok(0i32))?,
    )?;
    table.set(
        "GetDayEvent",
        lua.create_function(|_, (_o, _d, _i): (i32, i32, i32)| Ok(Value::Nil))?,
    )?;
    Ok(())
}

fn register_c_calendar_actions(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.set("OpenCalendar", lua.create_function(|_, ()| Ok(()))?)?;
    table.set("CloseCalendar", lua.create_function(|_, ()| Ok(()))?)?;
    table.set("SetMonth", lua.create_function(|_, _o: i32| Ok(()))?)?;
    table.set(
        "SetAbsMonth",
        lua.create_function(|_, (_m, _y): (i32, i32)| Ok(()))?,
    )?;
    Ok(())
}

fn register_c_covenant_callings(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "AreCallingsUnlocked",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set("RequestCallings", lua.create_function(|_, ()| Ok(()))?)?;
    lua.globals().set("C_CovenantCallings", t)?;
    Ok(())
}

fn register_c_weekly_rewards(
    lua: &Lua,
    state: Rc<RefCell<crate::lua_api::SimState>>,
) -> Result<()> {
    let t = lua.create_table()?;
    let s = Rc::clone(&state);
    t.set(
        "HasAvailableRewards",
        lua.create_function(move |_, ()| Ok(s.borrow().world.great_vault_has_rewards))?,
    )?;
    let s = Rc::clone(&state);
    t.set(
        "CanClaimRewards",
        lua.create_function(move |_, ()| Ok(s.borrow().world.great_vault_can_claim))?,
    )?;
    let s = Rc::clone(&state);
    t.set(
        "GetActivities",
        lua.create_function(move |lua, filter: Option<i32>| {
            build_vault_activities_table(lua, &s.borrow(), filter)
        })?,
    )?;
    t.set(
        "GetNumCompletedDungeonRuns",
        lua.create_function(|_, ()| Ok((0i32, 0i32, 0i32)))?,
    )?;
    lua.globals().set("C_WeeklyRewards", t)?;
    Ok(())
}

fn build_vault_activities_table(
    lua: &Lua,
    state: &crate::lua_api::SimState,
    filter: Option<i32>,
) -> Result<mlua::Table> {
    let result = lua.create_table()?;
    let mut idx = 1;
    for a in &state.world.great_vault_activities {
        if let Some(f) = filter {
            if a.activity_type != f {
                continue;
            }
        }
        let entry = lua.create_table()?;
        entry.set("type", a.activity_type)?;
        entry.set("index", a.index)?;
        entry.set("threshold", a.threshold)?;
        entry.set("progress", a.progress)?;
        entry.set("level", a.level)?;
        entry.set("id", 0i32)?;
        entry.set("rewards", lua.create_table()?)?;
        result.set(idx, entry)?;
        idx += 1;
    }
    Ok(result)
}

fn register_c_contribution_collector(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set("GetState", lua.create_function(|_, _id: i32| Ok(0i32))?)?;
    t.set(
        "GetContributionCollector",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetManagedContributionsForCreatureID",
        lua.create_function(|lua, _id: i32| lua.create_table())?,
    )?;
    t.set(
        "GetContributionResult",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    t.set(
        "IsAwaitingRewardQuestData",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    lua.globals().set("C_ContributionCollector", t)?;
    Ok(())
}

fn register_c_scenario(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetInfo",
        lua.create_function(|_, ()| Ok((Value::Nil, 0i32, 0i32, 0i32, false, false)))?,
    )?;
    t.set(
        "GetStepInfo",
        lua.create_function(|_, _s: Option<i32>| Ok((Value::Nil, Value::Nil, 0i32, false, false)))?,
    )?;
    t.set(
        "GetCriteriaInfo",
        lua.create_function(|_, _i: i32| Ok(Value::Nil))?,
    )?;
    t.set("IsInScenario", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "ShouldShowCriteria",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    lua.globals().set("C_Scenario", t)?;
    Ok(())
}

fn make_c_housing_customize_mode(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set("IsHoveringDecor", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetHoveredDecorInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetDecorDyeSlots",
        lua.create_function(|lua, _id: i32| lua.create_table())?,
    )?;
    Ok(t)
}

fn make_c_dye_color(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set(
        "GetDyeColorInfo",
        lua.create_function(|lua, _id: i32| {
            let info = lua.create_table()?;
            info.set("name", "Dye")?;
            info.set("dyeColorID", 0)?;
            info.set("baseColor", 0xFFFFFFu32)?;
            info.set("highlightColor", 0xFFFFFFu32)?;
            info.set("shadowColor", 0x000000u32)?;
            Ok(info)
        })?,
    )?;
    Ok(t)
}

fn make_c_house_editor(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set(
        "IsHouseEditorActive",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "GetActiveHouseEditorMode",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "ActivateHouseEditorMode",
        lua.create_function(|_, _m: i32| Ok(()))?,
    )?;
    t.set(
        "GetHouseEditorModeAvailability",
        lua.create_function(|_, _m: i32| Ok(false))?,
    )?;
    t.set(
        "IsHouseEditorModeActive",
        lua.create_function(|_, _m: i32| Ok(false))?,
    )?;
    Ok(t)
}

fn register_c_housing(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "C_HousingCustomizeMode",
        make_c_housing_customize_mode(lua)?,
    )?;
    g.set("C_DyeColor", make_c_dye_color(lua)?)?;
    g.set("C_HouseEditor", make_c_house_editor(lua)?)?;
    g.set("C_HousingDecor", make_c_housing_decor(lua)?)?;
    g.set("C_Housing", make_c_housing_namespace(lua)?)?;

    let basic = lua.create_table()?;
    basic.set("IsDecorSelected", lua.create_function(|_, ()| Ok(false))?)?;
    basic.set(
        "GetSelectedDecorInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set("C_HousingBasicMode", basic)?;

    g.set("C_HousingCatalog", make_c_housing_catalog(lua, &g)?)?;

    Ok(())
}

fn fire_ui_event(lua: &Lua, event_name: &str, args: &[Value]) -> Result<()> {
    let fire: mlua::Function = lua.globals().get("FireEvent")?;
    let mut call_args = vec![Value::String(lua.create_string(event_name)?)];
    call_args.extend(args.iter().cloned());
    fire.call(MultiValue::from_vec(call_args))
}

fn make_c_housing_decor(lua: &Lua) -> Result<mlua::Table> {
    let decor = lua.create_table()?;
    decor.set(
        "GetHoveredDecorInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    decor.set("IsHoveringDecor", lua.create_function(|_, ()| Ok(false))?)?;
    decor.set(
        "GetDecorInfo",
        lua.create_function(|_, _id: i32| Ok(Value::Nil))?,
    )?;
    Ok(decor)
}

fn make_c_housing_namespace(lua: &Lua) -> Result<mlua::Table> {
    let housing = lua.create_table()?;
    housing.set(
        "GetTrackedHouseGuid",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    housing.set("IsInsideHouse", lua.create_function(|_, ()| Ok(false))?)?;
    housing.set(
        "IsInsideHouseOrPlot",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    housing.set(
        "IsHousingServiceEnabled",
        lua.create_function(|_, ()| Ok(true))?,
    )?;
    housing.set(
        "GetPlayerOwnedHouses",
        create_get_player_owned_houses_fn(lua)?,
    )?;
    Ok(housing)
}

fn create_get_player_owned_houses_fn(lua: &Lua) -> Result<mlua::Function> {
    lua.create_function(|lua, ()| {
        let house_list = lua.create_table()?;
        fire_ui_event(
            lua,
            "PLAYER_HOUSE_LIST_UPDATED",
            &[Value::Table(house_list.clone())],
        )?;
        Ok(house_list)
    })
}

#[derive(Clone)]
struct HousingCatalogVariantInfo {
    variant_id: i32,
    product_id: i32,
    name: &'static str,
}

#[derive(Clone)]
struct HousingCatalogEntryInfo {
    entry_id: i32,
    name: &'static str,
    featured_small: bool,
    variants: Vec<HousingCatalogVariantInfo>,
    was_viewed_in_store: bool,
}

#[derive(Clone)]
struct HousingCatalogBundleInfo {
    bundle_id: i32,
    name: &'static str,
    entry_ids: Vec<i32>,
    was_viewed: bool,
}

struct HousingCatalogState {
    entries: HashMap<i32, HousingCatalogEntryInfo>,
    bundles: HashMap<i32, HousingCatalogBundleInfo>,
    cart_counts: HashMap<i32, i32>,
}

impl HousingCatalogState {
    fn seeded() -> Self {
        let entries = [
            HousingCatalogEntryInfo {
                entry_id: 1001,
                name: "Sunspire Chair",
                featured_small: true,
                variants: vec![
                    HousingCatalogVariantInfo {
                        variant_id: 1,
                        product_id: 91001,
                        name: "Crimson Upholstery",
                    },
                    HousingCatalogVariantInfo {
                        variant_id: 2,
                        product_id: 91002,
                        name: "Azure Upholstery",
                    },
                ],
                was_viewed_in_store: false,
            },
            HousingCatalogEntryInfo {
                entry_id: 1002,
                name: "Moonwell Lamp",
                featured_small: true,
                variants: vec![HousingCatalogVariantInfo {
                    variant_id: 1,
                    product_id: 92001,
                    name: "Starlit Glass",
                }],
                was_viewed_in_store: false,
            },
            HousingCatalogEntryInfo {
                entry_id: 1003,
                name: "Grand Canopy Bed",
                featured_small: false,
                variants: vec![HousingCatalogVariantInfo {
                    variant_id: 1,
                    product_id: 93001,
                    name: "Royal Plumage",
                }],
                was_viewed_in_store: false,
            },
        ]
        .into_iter()
        .map(|entry| (entry.entry_id, entry))
        .collect();

        let bundles = [HousingCatalogBundleInfo {
            bundle_id: 5001,
            name: "Moonlit Lounge Set",
            entry_ids: vec![1001, 1002],
            was_viewed: false,
        }]
        .into_iter()
        .map(|bundle| (bundle.bundle_id, bundle))
        .collect();

        Self {
            entries,
            bundles,
            cart_counts: HashMap::new(),
        }
    }
}

fn make_c_housing_catalog(lua: &Lua, globals: &mlua::Table) -> Result<mlua::Table> {
    let catalog = ensure_c_housing_catalog_table(lua, globals)?;
    register_c_housing_catalog_searcher(lua, &catalog)?;
    register_c_housing_catalog_categories(lua, &catalog)?;
    register_c_housing_catalog_market_methods(lua, &catalog)?;
    Ok(catalog)
}

fn ensure_c_housing_catalog_table(lua: &Lua, globals: &mlua::Table) -> Result<mlua::Table> {
    match globals.get::<Value>("C_HousingCatalog")? {
        Value::Table(t) => Ok(t),
        _ => lua.create_table(),
    }
}

fn register_c_housing_catalog_searcher(lua: &Lua, catalog: &mlua::Table) -> Result<()> {
    catalog.set(
        "CreateCatalogSearcher",
        lua.create_function(|lua, _: MultiValue| {
            let searcher = lua.create_table()?;
            let ret_empty =
                lua.create_function(|lua, _: MultiValue| Ok(Value::Table(lua.create_table()?)))?;
            searcher.set("GetAllSearchItems", ret_empty.clone())?;
            searcher.set("GetCatalogSearchResults", ret_empty)?;
            let mt = lua.create_table()?;
            mt.set(
                "__index",
                lua.create_function(|lua, (_t, _key): (Value, Value)| {
                    lua.create_function(|_, _: MultiValue| Ok(Value::Nil))
                })?,
            )?;
            searcher.set_metatable(Some(mt));
            Ok(Value::Table(searcher))
        })?,
    )?;
    Ok(())
}

fn register_c_housing_catalog_categories(lua: &Lua, catalog: &mlua::Table) -> Result<()> {
    const ALL_CATEGORY_ID: i32 = 18;

    catalog.set(
        "SearchCatalogCategories",
        lua.create_function(|lua, _: MultiValue| {
            let t = lua.create_table()?;
            t.push(ALL_CATEGORY_ID)?;
            Ok(Value::Table(t))
        })?,
    )?;
    catalog.set(
        "GetCatalogCategoryInfo",
        lua.create_function(|lua, category_id: Option<i32>| {
            let id = category_id.unwrap_or(ALL_CATEGORY_ID);
            let info = lua.create_table()?;
            info.set("ID", id)?;
            info.set("name", "All")?;
            info.set("subcategoryIDs", lua.create_table()?)?;
            Ok(Value::Table(info))
        })?,
    )?;
    Ok(())
}

fn register_c_housing_catalog_market_methods(lua: &Lua, catalog: &mlua::Table) -> Result<()> {
    let state = Rc::new(RefCell::new(HousingCatalogState::seeded()));

    let state_ref = Rc::clone(&state);
    catalog.set(
        "GetCatalogEntryVariantInfo",
        lua.create_function(move |lua, (entry_id, variant_id): (i32, i32)| {
            get_housing_catalog_entry_variant_info(lua, &state_ref.borrow(), entry_id, variant_id)
        })?,
    )?;

    let state_ref = Rc::clone(&state);
    catalog.set(
        "GetAllVariantInfosForEntry",
        lua.create_function(move |lua, entry_id: i32| {
            get_all_housing_catalog_variant_infos(lua, &state_ref.borrow(), entry_id)
        })?,
    )?;

    let state_ref = Rc::clone(&state);
    catalog.set(
        "GetFeaturedSmallProducts",
        lua.create_function(move |lua, ()| {
            get_housing_catalog_featured_small_products(lua, &state_ref.borrow())
        })?,
    )?;

    let state_ref = Rc::clone(&state);
    catalog.set(
        "GetMarketInfoForDecor",
        lua.create_function(move |lua, entry_id: i32| {
            get_housing_catalog_market_info(lua, &state_ref.borrow(), entry_id)
        })?,
    )?;

    let state_ref = Rc::clone(&state);
    catalog.set(
        "GetBundleInfo",
        lua.create_function(move |lua, bundle_id: i32| {
            get_housing_catalog_bundle_info(lua, &state_ref.borrow(), bundle_id)
        })?,
    )?;

    let state_ref = Rc::clone(&state);
    catalog.set(
        "HousingMarketActionAddToCart",
        lua.create_function(move |_, entry_id: i32| {
            let mut state = state_ref.borrow_mut();
            if !state.entries.contains_key(&entry_id) {
                return Ok(false);
            }
            *state.cart_counts.entry(entry_id).or_insert(0) += 1;
            Ok(true)
        })?,
    )?;

    let state_ref = Rc::clone(&state);
    catalog.set(
        "HousingMarketActionRemoveFromCart",
        lua.create_function(move |_, entry_id: i32| {
            let mut state = state_ref.borrow_mut();
            let Some(count) = state.cart_counts.get_mut(&entry_id) else {
                return Ok(false);
            };
            *count -= 1;
            if *count <= 0 {
                state.cart_counts.remove(&entry_id);
            }
            Ok(true)
        })?,
    )?;

    let state_ref = Rc::clone(&state);
    catalog.set(
        "HousingMarketActionClearCart",
        lua.create_function(move |_, ()| {
            state_ref.borrow_mut().cart_counts.clear();
            Ok(())
        })?,
    )?;

    let state_ref = Rc::clone(&state);
    catalog.set(
        "HousingMarketActionViewInStore",
        lua.create_function(move |_, entry_id: i32| {
            let mut state = state_ref.borrow_mut();
            let Some(entry) = state.entries.get_mut(&entry_id) else {
                return Ok(false);
            };
            entry.was_viewed_in_store = true;
            Ok(true)
        })?,
    )?;

    catalog.set(
        "HousingMarketActionViewBundle",
        lua.create_function(move |_, bundle_id: i32| {
            let mut state = state.borrow_mut();
            let Some(bundle) = state.bundles.get_mut(&bundle_id) else {
                return Ok(false);
            };
            bundle.was_viewed = true;
            Ok(true)
        })?,
    )?;

    Ok(())
}

fn get_housing_catalog_entry_variant_info(
    lua: &Lua,
    state: &HousingCatalogState,
    entry_id: i32,
    variant_id: i32,
) -> Result<Value> {
    let Some(entry) = state.entries.get(&entry_id) else {
        return Ok(Value::Nil);
    };
    let Some(variant) = entry
        .variants
        .iter()
        .find(|variant| variant.variant_id == variant_id)
    else {
        return Ok(Value::Nil);
    };
    Ok(Value::Table(build_housing_catalog_variant_info_table(
        lua, entry, variant,
    )?))
}

fn get_all_housing_catalog_variant_infos(
    lua: &Lua,
    state: &HousingCatalogState,
    entry_id: i32,
) -> Result<mlua::Table> {
    let variants = lua.create_table()?;
    let Some(entry) = state.entries.get(&entry_id) else {
        return Ok(variants);
    };
    for (index, variant) in entry.variants.iter().enumerate() {
        variants.set(
            index + 1,
            build_housing_catalog_variant_info_table(lua, entry, variant)?,
        )?;
    }
    Ok(variants)
}

fn get_housing_catalog_featured_small_products(
    lua: &Lua,
    state: &HousingCatalogState,
) -> Result<mlua::Table> {
    let featured = lua.create_table()?;
    let mut entries = state
        .entries
        .values()
        .filter(|entry| entry.featured_small)
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.entry_id);
    for (index, entry) in entries.into_iter().enumerate() {
        let info = lua.create_table()?;
        let first_variant = entry.variants.first();
        info.set("entryID", entry.entry_id)?;
        info.set("name", entry.name)?;
        info.set(
            "productID",
            first_variant
                .map(|variant| variant.product_id)
                .unwrap_or_default(),
        )?;
        info.set(
            "variantID",
            first_variant
                .map(|variant| variant.variant_id)
                .unwrap_or_default(),
        )?;
        featured.set(index + 1, info)?;
    }
    Ok(featured)
}

fn get_housing_catalog_market_info(
    lua: &Lua,
    state: &HousingCatalogState,
    entry_id: i32,
) -> Result<Value> {
    let Some(entry) = state.entries.get(&entry_id) else {
        return Ok(Value::Nil);
    };
    let info = lua.create_table()?;
    let cart_count = state
        .cart_counts
        .get(&entry_id)
        .copied()
        .unwrap_or_default();
    info.set("entryID", entry.entry_id)?;
    info.set("cartCount", cart_count)?;
    info.set("isInCart", cart_count > 0)?;
    info.set("wasViewedInStore", entry.was_viewed_in_store)?;
    Ok(Value::Table(info))
}

fn get_housing_catalog_bundle_info(
    lua: &Lua,
    state: &HousingCatalogState,
    bundle_id: i32,
) -> Result<Value> {
    let Some(bundle) = state.bundles.get(&bundle_id) else {
        return Ok(Value::Nil);
    };
    let info = lua.create_table()?;
    let entry_ids = lua.create_table()?;
    for (index, entry_id) in bundle.entry_ids.iter().enumerate() {
        entry_ids.set(index + 1, *entry_id)?;
    }
    info.set("bundleID", bundle.bundle_id)?;
    info.set("name", bundle.name)?;
    info.set("entryIDs", entry_ids)?;
    info.set("wasViewed", bundle.was_viewed)?;
    Ok(Value::Table(info))
}

fn build_housing_catalog_variant_info_table(
    lua: &Lua,
    entry: &HousingCatalogEntryInfo,
    variant: &HousingCatalogVariantInfo,
) -> Result<mlua::Table> {
    let info = lua.create_table()?;
    info.set("entryID", entry.entry_id)?;
    info.set("variantID", variant.variant_id)?;
    info.set("name", variant.name)?;
    info.set("productID", variant.product_id)?;
    Ok(info)
}

fn register_c_game_rules(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "IsGameRuleActive",
        lua.create_function(|_, _r: Value| Ok(false))?,
    )?;
    t.set("GetActiveGameMode", lua.create_function(|_, ()| Ok(0))?)?;
    t.set(
        "GetGameRuleAsFloat",
        lua.create_function(|_, _r: Value| Ok(0.0f32))?,
    )?;
    t.set("IsStandard", lua.create_function(|_, ()| Ok(true))?)?;
    t.set("IsWoWHack", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetGameRuleAsFrameStrata",
        lua.create_function(|_, _r: Value| Ok(Value::Nil))?,
    )?;
    t.set(
        "IsPersonalResourceDisplayEnabled",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    t.set(
        "GetCurrentEventRealmQueues",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    lua.globals().set("C_GameRules", t)?;
    Ok(())
}

fn register_c_scripted_animations(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "GetAllScriptedAnimationEffects",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    lua.globals().set("C_ScriptedAnimations", t)?;
    Ok(())
}

fn register_c_glue(lua: &Lua) -> Result<()> {
    let t = lua.create_table()?;
    t.set(
        "IsOnGlueScreen",
        lua.create_function(|lua, ()| {
            let Some(state) = lua.app_data_ref::<Rc<RefCell<crate::lua_api::SimState>>>() else {
                return Ok(false);
            };
            Ok(state.borrow().screen_kind.is_glue())
        })?,
    )?;
    lua.globals().set("C_Glue", t)?;
    Ok(())
}
