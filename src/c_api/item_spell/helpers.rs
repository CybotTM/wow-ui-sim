use crate::lua_api::methods::{create_table, table_get};
use rilua::Val;
use rilua::vm::state::LuaState;

const ITEM_CLASS_NAMES: &[(i32, &str)] = &[
    (0, "Consumable"),
    (1, "Container"),
    (2, "Weapon"),
    (3, "Gem"),
    (4, "Armor"),
    (5, "Reagent"),
    (6, "Projectile"),
    (7, "Tradeskill"),
    (8, "Item Enhancement"),
    (9, "Recipe"),
    (10, "Money"),
    (11, "Quiver"),
    (12, "Quest"),
    (13, "Key"),
    (14, "Permanent"),
    (15, "Miscellaneous"),
    (16, "Glyph"),
    (17, "Battle Pets"),
    (18, "WoW Token"),
    (19, "Profession"),
    (20, "Housing"),
];

const ITEM_SUBCLASS_NAMES: &[(i32, i32, &str)] = &[
    (2, 0, "Axe"),
    (2, 1, "Axe"),
    (2, 2, "Bow"),
    (2, 3, "Gun"),
    (2, 4, "Mace"),
    (2, 5, "Mace"),
    (2, 6, "Polearm"),
    (2, 7, "One-Handed Swords"),
    (2, 8, "Two-Handed Swords"),
    (2, 9, "Warglaives"),
    (2, 10, "Staves"),
    (2, 11, "Bear Claws"),
    (2, 12, "Cat Claws"),
    (2, 13, "Unarmed"),
    (2, 14, "Generic"),
    (2, 15, "Daggers"),
    (2, 16, "Thrown"),
    (2, 18, "Crossbows"),
    (2, 19, "Wands"),
    (2, 20, "Fishing Poles"),
    (4, 1, "Cloth"),
    (4, 2, "Leather"),
    (4, 3, "Mail"),
    (4, 4, "Plate"),
    (4, 6, "Shield"),
    (7, 4, "Cooking"),
];

const INV_TYPE_EQUIP_LOCS: &[(u8, &str)] = &[
    (1, "INVTYPE_HEAD"),
    (2, "INVTYPE_NECK"),
    (3, "INVTYPE_SHOULDER"),
    (4, "INVTYPE_BODY"),
    (5, "INVTYPE_CHEST"),
    (6, "INVTYPE_WAIST"),
    (7, "INVTYPE_LEGS"),
    (8, "INVTYPE_FEET"),
    (9, "INVTYPE_WRIST"),
    (10, "INVTYPE_HAND"),
    (11, "INVTYPE_FINGER"),
    (12, "INVTYPE_TRINKET"),
    (13, "INVTYPE_WEAPON"),
    (14, "INVTYPE_SHIELD"),
    (15, "INVTYPE_RANGED"),
    (16, "INVTYPE_CLOAK"),
    (17, "INVTYPE_2HWEAPON"),
    (20, "INVTYPE_ROBE"),
    (21, "INVTYPE_WEAPONMAINHAND"),
    (22, "INVTYPE_WEAPONOFFHAND"),
    (23, "INVTYPE_HOLDABLE"),
];

pub(crate) fn item_class_from_inv_type(inv_type: u8) -> &'static str {
    match inv_type {
        13 | 15 | 17 | 21 | 22 | 25 | 26 => "Weapon",
        1..=12 | 14 | 16 | 23 => "Armor",
        _ => "Miscellaneous",
    }
}

pub(crate) fn inv_type_to_class_id(inv_type: u8) -> i32 {
    match inv_type {
        13 | 15 | 17 | 21 | 22 | 25 | 26 => 2,
        1..=12 | 14 | 16 | 23 => 4,
        _ => 15,
    }
}

pub(crate) fn item_class_name(class_id: i32) -> &'static str {
    ITEM_CLASS_NAMES
        .iter()
        .find_map(|(id, name)| (*id == class_id).then_some(*name))
        .unwrap_or("Unknown")
}

pub(super) fn item_subclass_name(class_id: i32, subclass_id: i32) -> &'static str {
    ITEM_SUBCLASS_NAMES
        .iter()
        .find_map(|(class, subclass, name)| {
            (*class == class_id && *subclass == subclass_id).then_some(*name)
        })
        .unwrap_or("Unknown")
}

pub(crate) fn inv_type_to_subclass(inv_type: u8) -> &'static str {
    match inv_type {
        1 => "Head",
        2 => "Neck",
        3 => "Shoulder",
        4 => "Shirt",
        5 => "Chest",
        6 => "Waist",
        7 => "Legs",
        8 => "Feet",
        9 => "Wrist",
        10 => "Hands",
        11 => "Finger",
        12 => "Trinket",
        14 => "Shield",
        16 => "Back",
        _ => "Junk",
    }
}

pub(crate) fn inv_type_to_equip_loc(inv_type: u8) -> &'static str {
    INV_TYPE_EQUIP_LOCS
        .iter()
        .find_map(|(id, equip_loc)| (*id == inv_type).then_some(*equip_loc))
        .unwrap_or("")
}

pub(crate) fn global_table(state: &mut LuaState, name: &str) -> Val {
    let key_ref = state.gc.intern_string(name.as_bytes());
    let current = state
        .gc
        .tables
        .get(state.global)
        .map(|globals| globals.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    if matches!(current, Val::Table(_)) {
        return current;
    }
    let table = create_table(state);
    let global = state.global;
    if let Some(globals) = state.gc.tables.get_mut(global) {
        let _ = globals.raw_set(Val::Str(key_ref), table, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    table
}

pub(crate) fn current_item_upgrade_location(state: &mut LuaState) -> Option<(i32, i32)> {
    let storage = global_table(state, "__item_upgrade_state");
    let location = table_get(state, storage, "location");
    let Val::Table(_) = location else { return None };
    let bag = match table_get(state, location, "bagID") {
        Val::Num(value) => value as i32,
        _ => return None,
    };
    let slot = match table_get(state, location, "slotIndex") {
        Val::Num(value) => value as i32,
        _ => return None,
    };
    Some((bag, slot))
}
