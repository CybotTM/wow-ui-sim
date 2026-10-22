//! Native 12340 auction selectors backed by the authenticated enUS DBC projection.
use super::wrath_spellbook::numeric_argument;
use crate::lua_api::methods::create_string;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};
use serde::Deserialize;
use std::sync::LazyLock;

const HIDDEN_SUBCLASS: u32 = 0x2;
const INVENTORY_FILTER_SUPPORTED: u32 = 0x200;
const ARMOR_CLASS: u32 = 4;
const SUBCLASS_USAGE: &str = "Usage: GetAuctionItemSubClasses(index)";
const INVENTORY_USAGE: &str = "Usage: GetAuctionInvTypes(classIndex, subClassIndex)";
const INVENTORY_TOKENS: [&str; 14] = [
    "INVTYPE_HEAD",
    "INVTYPE_NECK",
    "INVTYPE_SHOULDER",
    "INVTYPE_BODY",
    "INVTYPE_CHEST",
    "INVTYPE_WAIST",
    "INVTYPE_LEGS",
    "INVTYPE_FEET",
    "INVTYPE_WRIST",
    "INVTYPE_HAND",
    "INVTYPE_FINGER",
    "INVTYPE_TRINKET",
    "INVTYPE_CLOAK",
    "INVTYPE_HOLDABLE",
];

#[derive(Deserialize)]
struct Catalog {
    selectors: Vec<Selector>,
    classes: Vec<ItemClass>,
    subclasses: Vec<ItemSubclass>,
}

#[derive(Deserialize)]
struct Selector {
    class_id: u32,
    has_subclasses: bool,
}

#[derive(Deserialize)]
struct ItemClass {
    class_id: u32,
    name: String,
}

#[derive(Deserialize)]
struct ItemSubclass {
    class_id: u32,
    flags: u32,
    display_flags: u32,
    name: String,
}

static CATALOG: LazyLock<Catalog> = LazyLock::new(|| {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/wrath-auction-catalog.json"
    )))
    .expect("committed Wrath auction catalog must decode")
});

pub fn register(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "GetAuctionItemClasses", get_classes)?;
    LuaApiMut::register_function(lua, "GetAuctionItemSubClasses", get_subclasses)?;
    LuaApiMut::register_function(lua, "GetAuctionInvTypes", get_inventory_types)
}

fn get_classes(state: &mut LuaState) -> LuaResult<u32> {
    let data = &*CATALOG;
    let names = data.selectors.iter().filter_map(|selector| {
        data.classes
            .iter()
            .find(|row| row.class_id == selector.class_id)
            .map(|row| row.name.as_str())
    });
    Ok(push_names(state, names))
}

fn get_subclasses(state: &mut LuaState) -> LuaResult<u32> {
    let index = read_index(state, 1, SUBCLASS_USAGE)?;
    let data = &*CATALOG;
    let Some(selector) = subclass_selector(data, index) else {
        return Ok(0);
    };
    let names = visible_subclasses(data, selector.class_id).map(|row| row.name.as_str());
    Ok(push_names(state, names))
}

fn get_inventory_types(state: &mut LuaState) -> LuaResult<u32> {
    let class_index = read_index(state, 1, INVENTORY_USAGE)?;
    let subclass_index = read_index(state, 2, INVENTORY_USAGE)?;
    let data = &*CATALOG;
    let Some(selector) = subclass_selector(data, class_index) else {
        return Ok(0);
    };
    // Native indexes the same displayFlags-filtered ordinal stream as the name getter.
    let lacks_inventory_filter = visible_subclasses(data, selector.class_id)
        .nth(subclass_index)
        .is_some_and(|row| row.flags & INVENTORY_FILTER_SUPPORTED == 0);
    if lacks_inventory_filter {
        return Ok(0);
    }
    // Native deliberately falls through when no subclass ordinal is found.
    for token in INVENTORY_TOKENS {
        let value = create_string(state, token);
        state.push(value);
        let available = inventory_slot_available(selector.class_id, subclass_index, token);
        state.push(if available { Val::Num(1.0) } else { Val::Nil });
    }
    Ok((INVENTORY_TOKENS.len() * 2) as u32)
}

fn read_index(state: &LuaState, argument: i32, usage: &str) -> LuaResult<usize> {
    let number = numeric_argument(state, argument).ok_or_else(|| runtime_error(usage))?;
    Ok((number as u32).wrapping_sub(1) as usize)
}

fn subclass_selector(data: &Catalog, index: usize) -> Option<&Selector> {
    data.selectors
        .get(index)
        .filter(|selector| selector.has_subclasses)
}

fn visible_subclasses(data: &Catalog, class_id: u32) -> impl Iterator<Item = &ItemSubclass> {
    data.subclasses
        .iter()
        .filter(move |row| row.class_id == class_id)
        .filter(|row| row.display_flags & HIDDEN_SUBCLASS == 0)
}

fn push_names<'a>(state: &mut LuaState, names: impl Iterator<Item = &'a str>) -> u32 {
    let mut count = 0;
    for name in names {
        let value = create_string(state, name);
        state.push(value);
        count += 1;
    }
    count
}

fn inventory_slot_available(class_id: u32, subclass: usize, token: &str) -> bool {
    if class_id != ARMOR_CLASS {
        return true;
    }
    if subclass == 0 {
        return matches!(
            token,
            "INVTYPE_HEAD"
                | "INVTYPE_NECK"
                | "INVTYPE_BODY"
                | "INVTYPE_FINGER"
                | "INVTYPE_TRINKET"
                | "INVTYPE_HOLDABLE"
        );
    }
    let regular_armor_slot = matches!(
        token,
        "INVTYPE_HEAD"
            | "INVTYPE_SHOULDER"
            | "INVTYPE_CHEST"
            | "INVTYPE_WAIST"
            | "INVTYPE_LEGS"
            | "INVTYPE_FEET"
            | "INVTYPE_WRIST"
            | "INVTYPE_HAND"
    );
    let cloth_cloak = subclass == 1 && token == "INVTYPE_CLOAK";
    regular_armor_slot || cloth_cloak
}
