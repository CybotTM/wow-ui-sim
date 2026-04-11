//! C_Housing and related namespace API stubs.
//!
//! Contains:
//! - C_HousingCustomizeMode, C_DyeColor, C_HouseEditor
//! - C_HousingDecor, C_Housing, C_HousingNeighborhood
//! - C_HousingBasicMode, C_HousingCatalog, C_HouseExterior

use mlua::{Lua, MultiValue, Result, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct HousingCustomizeModeState {
    selected_decor_info: Option<HousingSelectedDecorInfo>,
}

impl HousingCustomizeModeState {
    fn seeded() -> Self {
        Self {
            selected_decor_info: Some(HousingSelectedDecorInfo {
                decor_guid: "Decor-Selection-1001",
                name: "Sunspire Chair",
                can_be_customized: true,
                can_be_removed: true,
                is_locked: false,
            }),
        }
    }
}

struct HousingSelectedDecorInfo {
    decor_guid: &'static str,
    name: &'static str,
    can_be_customized: bool,
    can_be_removed: bool,
    is_locked: bool,
}

struct HousingDecorState {
    selected_decor_info: Option<HousingDecorInstanceInfo>,
    decor_hyperlinks: HashMap<i32, &'static str>,
}

impl HousingDecorState {
    fn seeded() -> Self {
        let decor_hyperlinks = [(
            91002,
            "|cff66bbff|Hhousingdecor:91002|h[Azure Upholstery]|h|r",
        )]
        .into_iter()
        .collect();

        Self {
            selected_decor_info: Some(HousingDecorInstanceInfo {
                decor_guid: "Decor-Selection-2001",
                name: "Azure Reading Lamp",
                can_be_removed: true,
                is_locked: false,
            }),
            decor_hyperlinks,
        }
    }
}

struct HousingDecorInstanceInfo {
    decor_guid: &'static str,
    name: &'static str,
    can_be_removed: bool,
    is_locked: bool,
}

struct HousingNeighborhoodState {
    cornerstone_house_info: Option<CornerstoneHouseInfo>,
    cornerstone_neighborhood_info: Option<CornerstoneNeighborhoodInfo>,
}

impl HousingNeighborhoodState {
    fn seeded() -> Self {
        Self {
            cornerstone_house_info: Some(CornerstoneHouseInfo {
                plot_id: 27,
                owner_name: "Simhero",
                house_name: "Sunspire Retreat",
            }),
            cornerstone_neighborhood_info: Some(CornerstoneNeighborhoodInfo {
                neighborhood_name: "Dawnmeadow",
                neighborhood_type: "Public",
            }),
        }
    }
}

struct CornerstoneHouseInfo {
    plot_id: i32,
    owner_name: &'static str,
    house_name: &'static str,
}

struct CornerstoneNeighborhoodInfo {
    neighborhood_name: &'static str,
    neighborhood_type: &'static str,
}

fn register_customize_mode_selected_decor(
    lua: &Lua,
    t: &mlua::Table,
    state: &Rc<RefCell<HousingCustomizeModeState>>,
) -> Result<()> {
    let state_ref = Rc::clone(state);
    t.set(
        "IsDecorSelected",
        lua.create_function(move |_, ()| Ok(state_ref.borrow().selected_decor_info.is_some()))?,
    )?;
    let state_ref = Rc::clone(state);
    t.set(
        "GetSelectedDecorInfo",
        lua.create_function(move |lua, ()| {
            let state = state_ref.borrow();
            let Some(info) = &state.selected_decor_info else {
                return Ok(Value::Nil);
            };
            let decor = lua.create_table()?;
            decor.set("decorGUID", info.decor_guid)?;
            decor.set("name", info.name)?;
            decor.set("canBeCustomized", info.can_be_customized)?;
            decor.set("canBeRemoved", info.can_be_removed)?;
            decor.set("isLocked", info.is_locked)?;
            Ok(Value::Table(decor))
        })?,
    )?;
    Ok(())
}

fn make_c_housing_customize_mode(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    let state = Rc::new(RefCell::new(HousingCustomizeModeState::seeded()));
    t.set("IsHoveringDecor", lua.create_function(|_, ()| Ok(false))?)?;
    t.set(
        "GetHoveredDecorInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    register_customize_mode_selected_decor(lua, &t, &state)?;
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

pub(super) fn register_c_housing(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "C_HousingCustomizeMode",
        make_c_housing_customize_mode(lua)?,
    )?;
    g.set("C_DyeColor", make_c_dye_color(lua)?)?;
    g.set("C_HouseEditor", make_c_house_editor(lua)?)?;
    g.set("C_HousingDecor", make_c_housing_decor(lua)?)?;
    g.set("C_Housing", make_c_housing_namespace(lua)?)?;
    g.set("C_HousingNeighborhood", make_c_housing_neighborhood(lua)?)?;

    let basic = lua.create_table()?;
    basic.set("IsDecorSelected", lua.create_function(|_, ()| Ok(false))?)?;
    basic.set(
        "GetSelectedDecorInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    g.set("C_HousingBasicMode", basic)?;

    g.set("C_HousingCatalog", make_c_housing_catalog(lua, &g)?)?;
    g.set("C_HouseExterior", make_c_house_exterior(lua, &g)?)?;

    Ok(())
}

fn fire_ui_event(lua: &Lua, event_name: &str, args: &[Value]) -> Result<()> {
    let fire: mlua::Function = lua.globals().get("FireEvent")?;
    let mut call_args = vec![Value::String(lua.create_string(event_name)?)];
    call_args.extend(args.iter().cloned());
    fire.call(MultiValue::from_vec(call_args))
}

fn register_housing_decor_queries(
    lua: &Lua,
    decor: &mlua::Table,
    state: &Rc<RefCell<HousingDecorState>>,
) -> Result<()> {
    let state_ref = Rc::clone(state);
    decor.set(
        "IsDecorSelected",
        lua.create_function(move |_, ()| Ok(state_ref.borrow().selected_decor_info.is_some()))?,
    )?;
    let state_ref = Rc::clone(state);
    decor.set(
        "GetDecorHyperlink",
        lua.create_function(move |lua, decor_id: i32| {
            let state = state_ref.borrow();
            let Some(link) = state.decor_hyperlinks.get(&decor_id) else {
                return Ok(Value::Nil);
            };
            Ok(Value::String(lua.create_string(*link)?))
        })?,
    )?;
    let state_ref = Rc::clone(state);
    decor.set(
        "GetSelectedDecorInfo",
        lua.create_function(move |lua, ()| {
            let state = state_ref.borrow();
            let Some(info) = &state.selected_decor_info else {
                return Ok(Value::Nil);
            };
            let t = lua.create_table()?;
            t.set("decorGUID", info.decor_guid)?;
            t.set("name", info.name)?;
            t.set("canBeRemoved", info.can_be_removed)?;
            t.set("isLocked", info.is_locked)?;
            Ok(Value::Table(t))
        })?,
    )?;
    Ok(())
}

fn make_c_housing_decor(lua: &Lua) -> Result<mlua::Table> {
    let decor = lua.create_table()?;
    let state = Rc::new(RefCell::new(HousingDecorState::seeded()));
    decor.set(
        "GetHoveredDecorInfo",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    decor.set("IsHoveringDecor", lua.create_function(|_, ()| Ok(false))?)?;
    register_housing_decor_queries(lua, &decor, &state)?;
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

fn register_neighborhood_cornerstone_queries(
    lua: &Lua,
    neighborhood: &mlua::Table,
    state: &Rc<RefCell<HousingNeighborhoodState>>,
) -> Result<()> {
    let state_ref = Rc::clone(state);
    neighborhood.set(
        "GetCornerstoneHouseInfo",
        lua.create_function(move |lua, ()| {
            let state = state_ref.borrow();
            let Some(info) = &state.cornerstone_house_info else {
                return Ok(Value::Nil);
            };
            let house = lua.create_table()?;
            house.set("plotID", info.plot_id)?;
            house.set("ownerName", info.owner_name)?;
            house.set("houseName", info.house_name)?;
            Ok(Value::Table(house))
        })?,
    )?;
    let state_ref = Rc::clone(state);
    neighborhood.set(
        "GetCornerstoneNeighborhoodInfo",
        lua.create_function(move |lua, ()| {
            let state = state_ref.borrow();
            let Some(info) = &state.cornerstone_neighborhood_info else {
                return Ok(Value::Nil);
            };
            let t = lua.create_table()?;
            t.set("neighborhoodName", info.neighborhood_name)?;
            t.set("neighborhoodType", info.neighborhood_type)?;
            Ok(Value::Table(t))
        })?,
    )?;
    Ok(())
}

fn make_c_housing_neighborhood(lua: &Lua) -> Result<mlua::Table> {
    let neighborhood = lua.create_table()?;
    let state = Rc::new(RefCell::new(HousingNeighborhoodState::seeded()));
    register_neighborhood_cornerstone_queries(lua, &neighborhood, &state)?;
    neighborhood.set(
        "OnCornerstoneClosed",
        lua.create_function(move |_, ()| {
            let mut state = state.borrow_mut();
            state.cornerstone_house_info = None;
            state.cornerstone_neighborhood_info = None;
            Ok(())
        })?,
    )?;
    Ok(neighborhood)
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

struct HouseExteriorState {
    decor_hidden: bool,
    house_exterior_has_attached_decor: bool,
    door_has_attached_decor: bool,
    selected_fixture_point_has_attached_decor: bool,
    core_fixture_attached_decor: HashMap<i32, bool>,
}

impl HouseExteriorState {
    fn seeded() -> Self {
        let core_fixture_attached_decor = [
            (house_exterior_base_fixture_type(), true),
            (house_exterior_roof_fixture_type(), false),
        ]
        .into_iter()
        .collect();

        Self {
            decor_hidden: false,
            house_exterior_has_attached_decor: true,
            door_has_attached_decor: true,
            selected_fixture_point_has_attached_decor: true,
            core_fixture_attached_decor,
        }
    }
}

const fn house_exterior_base_fixture_type() -> i32 {
    9
}

const fn house_exterior_roof_fixture_type() -> i32 {
    10
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

fn seed_catalog_entries() -> HashMap<i32, HousingCatalogEntryInfo> {
    [
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
    .collect()
}

fn seed_catalog_bundles() -> HashMap<i32, HousingCatalogBundleInfo> {
    [HousingCatalogBundleInfo {
        bundle_id: 5001,
        name: "Moonlit Lounge Set",
        entry_ids: vec![1001, 1002],
        was_viewed: false,
    }]
    .into_iter()
    .map(|bundle| (bundle.bundle_id, bundle))
    .collect()
}

impl HousingCatalogState {
    fn seeded() -> Self {
        Self {
            entries: seed_catalog_entries(),
            bundles: seed_catalog_bundles(),
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

fn make_c_house_exterior(lua: &Lua, globals: &mlua::Table) -> Result<mlua::Table> {
    let exterior = ensure_c_house_exterior_table(lua, globals)?;
    register_c_house_exterior_methods(lua, &exterior)?;
    Ok(exterior)
}

fn ensure_c_housing_catalog_table(lua: &Lua, globals: &mlua::Table) -> Result<mlua::Table> {
    match globals.get::<Value>("C_HousingCatalog")? {
        Value::Table(t) => Ok(t),
        _ => lua.create_table(),
    }
}

fn ensure_c_house_exterior_table(lua: &Lua, globals: &mlua::Table) -> Result<mlua::Table> {
    match globals.get::<Value>("C_HouseExterior")? {
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

fn register_catalog_market_queries(
    lua: &Lua,
    catalog: &mlua::Table,
    state: &Rc<RefCell<HousingCatalogState>>,
) -> Result<()> {
    let s = Rc::clone(state);
    catalog.set(
        "GetCatalogEntryVariantInfo",
        lua.create_function(move |lua, (entry_id, variant_id): (i32, i32)| {
            get_housing_catalog_entry_variant_info(lua, &s.borrow(), entry_id, variant_id)
        })?,
    )?;
    let s = Rc::clone(state);
    catalog.set(
        "GetAllVariantInfosForEntry",
        lua.create_function(move |lua, entry_id: i32| {
            get_all_housing_catalog_variant_infos(lua, &s.borrow(), entry_id)
        })?,
    )?;
    let s = Rc::clone(state);
    catalog.set(
        "GetFeaturedSmallProducts",
        lua.create_function(move |lua, ()| {
            get_housing_catalog_featured_small_products(lua, &s.borrow())
        })?,
    )?;
    let s = Rc::clone(state);
    catalog.set(
        "GetMarketInfoForDecor",
        lua.create_function(move |lua, entry_id: i32| {
            get_housing_catalog_market_info(lua, &s.borrow(), entry_id)
        })?,
    )?;
    let s = Rc::clone(state);
    catalog.set(
        "GetBundleInfo",
        lua.create_function(move |lua, bundle_id: i32| {
            get_housing_catalog_bundle_info(lua, &s.borrow(), bundle_id)
        })?,
    )?;
    Ok(())
}

fn register_catalog_market_actions(
    lua: &Lua,
    catalog: &mlua::Table,
    state: Rc<RefCell<HousingCatalogState>>,
) -> Result<()> {
    let s = Rc::clone(&state);
    catalog.set(
        "HousingMarketActionAddToCart",
        lua.create_function(move |_, entry_id: i32| {
            let mut state = s.borrow_mut();
            if !state.entries.contains_key(&entry_id) {
                return Ok(false);
            }
            *state.cart_counts.entry(entry_id).or_insert(0) += 1;
            Ok(true)
        })?,
    )?;
    let s = Rc::clone(&state);
    catalog.set(
        "HousingMarketActionRemoveFromCart",
        lua.create_function(move |_, entry_id: i32| {
            let mut state = s.borrow_mut();
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
    let s = Rc::clone(&state);
    catalog.set(
        "HousingMarketActionClearCart",
        lua.create_function(move |_, ()| {
            s.borrow_mut().cart_counts.clear();
            Ok(())
        })?,
    )?;
    let s = Rc::clone(&state);
    catalog.set(
        "HousingMarketActionViewInStore",
        lua.create_function(move |_, entry_id: i32| {
            let mut state = s.borrow_mut();
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

fn register_c_housing_catalog_market_methods(lua: &Lua, catalog: &mlua::Table) -> Result<()> {
    let state = Rc::new(RefCell::new(HousingCatalogState::seeded()));
    register_catalog_market_queries(lua, catalog, &state)?;
    register_catalog_market_actions(lua, catalog, state)?;
    Ok(())
}

fn register_house_exterior_decor_queries(
    lua: &Lua,
    exterior: &mlua::Table,
    state: &Rc<RefCell<HouseExteriorState>>,
) -> Result<()> {
    let state_ref = Rc::clone(state);
    exterior.set(
        "IsAnyDecorAttachedToHouseExterior",
        lua.create_function(move |_, ()| Ok(state_ref.borrow().house_exterior_has_attached_decor))?,
    )?;
    let state_ref = Rc::clone(state);
    exterior.set(
        "IsAnyDecorAttachedToDoor",
        lua.create_function(move |_, ()| Ok(state_ref.borrow().door_has_attached_decor))?,
    )?;
    let state_ref = Rc::clone(state);
    exterior.set(
        "IsAnyDecorAttachedToSelectedFixturePoint",
        lua.create_function(move |_, ()| {
            Ok(state_ref.borrow().selected_fixture_point_has_attached_decor)
        })?,
    )?;
    let state_ref = Rc::clone(state);
    exterior.set(
        "IsAnyDecorAttachedToCoreFixture",
        lua.create_function(move |_, core_fixture_type: i32| {
            Ok(*state_ref
                .borrow()
                .core_fixture_attached_decor
                .get(&core_fixture_type)
                .unwrap_or(&false))
        })?,
    )?;
    Ok(())
}

fn register_c_house_exterior_methods(lua: &Lua, exterior: &mlua::Table) -> Result<()> {
    let state = Rc::new(RefCell::new(HouseExteriorState::seeded()));
    register_house_exterior_decor_queries(lua, exterior, &state)?;
    let state_ref = Rc::clone(&state);
    exterior.set(
        "IsExteriorDecorHidden",
        lua.create_function(move |_, ()| Ok(state_ref.borrow().decor_hidden))?,
    )?;
    exterior.set(
        "SetExteriorDecorHidden",
        lua.create_function(move |_, hidden: bool| {
            state.borrow_mut().decor_hidden = hidden;
            Ok(())
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
