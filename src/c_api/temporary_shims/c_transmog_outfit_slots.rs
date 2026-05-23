//! Temporary `C_TransmogOutfitInfo` fallback surface.
//!
//! Outfit slot-location metadata is static enough for Blizzard startup, but
//! the simulator does not yet model the full transmog outfit system. Keep
//! these compatibility defaults isolated until a real wardrobe model owns them.

use crate::c_api::ensure_namespace;
use crate::c_api::helpers::{global_val, set_table_array};
use crate::lua_api::methods::{create_string, create_table, table_get, table_set, val_to_string};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::LuaResult;
use rilua::Val;
use rilua::vm::state::LuaState;

const ACTIVE_OUTFIT_ID_KEY: &str = "__activeOutfitID";
const CURRENTLY_VIEWED_OUTFIT_ID_KEY: &str = "__currentlyViewedOutfitID";
const PENDING_SHEATHE_CATEGORIES_KEY: &str = "__pendingSheatheCategories";
const APPEARANCE_TYPE_FALLBACK: f64 = 0.0;
const ILLUSION_TYPE_FALLBACK: f64 = 1.0;
const NONE_COLLECTION_TYPE_FALLBACK: f64 = 0.0;
const VALID_SHEATHE_SLOT_TRANSMOG_ID: f64 = 190001.0;

#[derive(Clone, Copy)]
struct OutfitSlotSpec {
    slot_name: &'static str,
    inventory_slot_id: i32,
    collection_type_name: Option<&'static str>,
    collection_type_fallback: f64,
    is_secondary: bool,
}

const APPEARANCE_SLOTS: &[OutfitSlotSpec] = &[
    appearance_slot("HEADSLOT", 1, "Head", 1.0, false),
    appearance_slot("SHOULDERSLOT", 3, "Shoulder", 2.0, false),
    appearance_slot("BACKSLOT", 15, "Back", 3.0, false),
    appearance_slot("CHESTSLOT", 5, "Chest", 4.0, false),
    appearance_slot("SHIRTSLOT", 4, "Shirt", 5.0, false),
    appearance_slot("TABARDSLOT", 19, "Tabard", 6.0, false),
    appearance_slot("WRISTSLOT", 9, "Wrist", 7.0, false),
    appearance_slot("HANDSSLOT", 10, "Hands", 8.0, false),
    appearance_slot("WAISTSLOT", 6, "Waist", 9.0, false),
    appearance_slot("LEGSSLOT", 7, "Legs", 10.0, false),
    appearance_slot("FEETSLOT", 8, "Feet", 11.0, false),
    appearance_slot(
        "MAINHANDSLOT",
        16,
        "None",
        NONE_COLLECTION_TYPE_FALLBACK,
        false,
    ),
    appearance_slot(
        "SECONDARYHANDSLOT",
        17,
        "None",
        NONE_COLLECTION_TYPE_FALLBACK,
        false,
    ),
    appearance_slot("SHOULDERSLOT", 3, "Shoulder", 2.0, true),
];

const ILLUSION_SLOTS: &[OutfitSlotSpec] = &[
    appearance_slot(
        "MAINHANDSLOT",
        16,
        "None",
        NONE_COLLECTION_TYPE_FALLBACK,
        false,
    ),
    appearance_slot(
        "SECONDARYHANDSLOT",
        17,
        "None",
        NONE_COLLECTION_TYPE_FALLBACK,
        false,
    ),
];

const fn appearance_slot(
    slot_name: &'static str,
    inventory_slot_id: i32,
    collection_type_name: &'static str,
    collection_type_fallback: f64,
    is_secondary: bool,
) -> OutfitSlotSpec {
    OutfitSlotSpec {
        slot_name,
        inventory_slot_id,
        collection_type_name: Some(collection_type_name),
        collection_type_fallback,
        is_secondary,
    }
}

pub(crate) fn register_c_transmog_outfit_slot_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_TransmogOutfitInfo")?;
    register_outfit_state_methods(state, ns)?;
    register_outfit_slot_methods(state, ns)
}

fn register_outfit_state_methods(
    state: &mut LuaState,
    ns: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(state, ns, "GetActiveOutfitID", get_active_outfit_id)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetCurrentlyViewedOutfitID",
        get_currently_viewed_outfit_id,
    )?;
    table_set_rust_fn_static(state, ns, "GetOutfitInfo", get_outfit_info)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetAllTransmogOutfitOptionSheatheCategoryInfo",
        get_all_transmog_outfit_option_sheathe_category_info,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "SetPendingTransmogSheatheCategory",
        set_pending_transmog_sheathe_category,
    )?;
    table_set_rust_fn_static(state, ns, "ChangeToOutfit", change_to_outfit)?;
    table_set_rust_fn_static(state, ns, "ClearOutfit", clear_outfit)
}

fn register_outfit_slot_methods(
    state: &mut LuaState,
    ns: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        ns,
        "GetTransmogOutfitSlotFromInventorySlot",
        get_transmog_outfit_slot_from_inventory_slot,
    )?;
    table_set_rust_fn_static(state, ns, "GetLinkedSlotInfo", get_linked_slot_info)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetAllSlotLocationInfo",
        get_all_slot_location_info,
    )
}

fn get_active_outfit_id(state: &mut LuaState) -> LuaResult<u32> {
    let outfit_id = outfit_id_value(state, ACTIVE_OUTFIT_ID_KEY);
    state.push(outfit_id);
    Ok(1)
}

fn get_currently_viewed_outfit_id(state: &mut LuaState) -> LuaResult<u32> {
    let outfit_id = outfit_id_value(state, CURRENTLY_VIEWED_OUTFIT_ID_KEY);
    state.push(outfit_id);
    Ok(1)
}

fn outfit_id_value(state: &mut LuaState, key: &str) -> Val {
    let namespace = global_val(state, "C_TransmogOutfitInfo");
    match table_get(state, namespace, key) {
        Val::Num(id) => Val::Num(id),
        _ => Val::Num(0.0),
    }
}

fn get_outfit_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn get_all_transmog_outfit_option_sheathe_category_info(state: &mut LuaState) -> LuaResult<u32> {
    let slot_transmog_id = number_or_numeric_string(state, stack_val(state, 1));
    if slot_transmog_id != Some(VALID_SHEATHE_SLOT_TRANSMOG_ID) {
        state.push(Val::Nil);
        return Ok(1);
    }

    let categories = create_table(state);
    let sheathe_categories = [
        ("Default", 0.0),
        ("Back", 1.0),
        ("Side", 2.0),
        ("Hide", 3.0),
    ];
    for (index, (name, fallback)) in sheathe_categories.iter().enumerate() {
        let entry = sheathe_category_info(state, name, *fallback);
        set_table_array(state, categories, index as i64 + 1, entry);
    }
    state.push(categories);
    Ok(1)
}

fn sheathe_category_info(state: &mut LuaState, name: &'static str, fallback: f64) -> Val {
    let entry = create_table(state);
    let category = enum_variant_number(
        state,
        "TransmogOutfitSlotOptionSheatheCategory",
        name,
        fallback,
    );
    table_set(state, entry, "sheatheCategory", Val::Num(category));
    let category_name = create_string(state, name);
    table_set(state, entry, "categoryName", category_name);
    entry
}

fn set_pending_transmog_sheathe_category(state: &mut LuaState) -> LuaResult<u32> {
    let slot_id = lua_key_string(state, stack_val(state, 1));
    let option_id = lua_key_string(state, stack_val(state, 2));
    let category = stack_val(state, 3);
    let pending = pending_sheathe_categories(state);
    let key = format!("{slot_id}:{option_id}");
    table_set(state, pending, &key, category);
    Ok(0)
}

fn lua_key_string(state: &mut LuaState, value: Val) -> String {
    match value {
        Val::Num(number) if number.fract() == 0.0 => format!("{}", number as i64),
        other => val_to_string(state, other).unwrap_or_else(|| "nil".to_string()),
    }
}

fn pending_sheathe_categories(state: &mut LuaState) -> Val {
    let namespace = global_val(state, "C_TransmogOutfitInfo");
    match table_get(state, namespace, PENDING_SHEATHE_CATEGORIES_KEY) {
        table @ Val::Table(_) => table,
        _ => {
            let table = create_table(state);
            table_set(state, namespace, PENDING_SHEATHE_CATEGORIES_KEY, table);
            table
        }
    }
}

fn change_to_outfit(state: &mut LuaState) -> LuaResult<u32> {
    if lua_truthy(stack_val(state, 2)) {
        reset_outfit_state(state);
        return Ok(0);
    }

    let outfit_id = number_or_numeric_string(state, stack_val(state, 1)).unwrap_or(0.0);
    set_outfit_ids(state, outfit_id);
    Ok(0)
}

fn clear_outfit(state: &mut LuaState) -> LuaResult<u32> {
    reset_outfit_state(state);
    Ok(0)
}

fn lua_truthy(value: Val) -> bool {
    !matches!(value, Val::Nil | Val::Bool(false))
}

fn reset_outfit_state(state: &mut LuaState) {
    set_outfit_ids(state, 0.0);
    let namespace = global_val(state, "C_TransmogOutfitInfo");
    let empty_pending = create_table(state);
    table_set(
        state,
        namespace,
        PENDING_SHEATHE_CATEGORIES_KEY,
        empty_pending,
    );
}

fn set_outfit_ids(state: &mut LuaState, outfit_id: f64) {
    let namespace = global_val(state, "C_TransmogOutfitInfo");
    table_set(state, namespace, ACTIVE_OUTFIT_ID_KEY, Val::Num(outfit_id));
    table_set(
        state,
        namespace,
        CURRENTLY_VIEWED_OUTFIT_ID_KEY,
        Val::Num(outfit_id),
    );
}

fn get_transmog_outfit_slot_from_inventory_slot(state: &mut LuaState) -> LuaResult<u32> {
    let slot = number_or_numeric_string(state, stack_val(state, 1));
    let Some(slot) = slot else {
        state.push(Val::Nil);
        return Ok(1);
    };
    if slot < 0.0 {
        state.push(Val::Nil);
        return Ok(1);
    }
    state.push(Val::Num(slot));
    Ok(1)
}

fn number_or_numeric_string(state: &mut LuaState, value: Val) -> Option<f64> {
    match value {
        Val::Num(number) => Some(number),
        other => val_to_string(state, other).and_then(|text| text.parse::<f64>().ok()),
    }
}

fn get_linked_slot_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn get_all_slot_location_info(state: &mut LuaState) -> LuaResult<u32> {
    let appearance_type = enum_variant_number(
        state,
        "TransmogType",
        "Appearance",
        APPEARANCE_TYPE_FALLBACK,
    );
    let illusion_type =
        enum_variant_number(state, "TransmogType", "Illusion", ILLUSION_TYPE_FALLBACK);
    let appearance_slots = build_slot_info_array(state, APPEARANCE_SLOTS, appearance_type)?;
    let illusion_slots = build_slot_info_array(state, ILLUSION_SLOTS, illusion_type)?;
    state.push(appearance_slots);
    state.push(illusion_slots);
    Ok(2)
}

fn build_slot_info_array(
    state: &mut LuaState,
    specs: &[OutfitSlotSpec],
    transmog_type: f64,
) -> LuaResult<Val> {
    let array = create_table(state);
    for (index, spec) in specs.iter().enumerate() {
        let entry = build_slot_info_table(state, *spec, transmog_type)?;
        set_table_array(state, array, index as i64 + 1, entry);
    }
    Ok(array)
}

fn build_slot_info_table(
    state: &mut LuaState,
    spec: OutfitSlotSpec,
    transmog_type: f64,
) -> LuaResult<Val> {
    let entry = create_table(state);
    table_set(
        state,
        entry,
        "slot",
        Val::Num((spec.inventory_slot_id - 1).max(0) as f64),
    );
    table_set(state, entry, "type", Val::Num(transmog_type));
    let collection_type = collection_type(state, spec);
    table_set(state, entry, "collectionType", Val::Num(collection_type));
    let slot_name = create_string(state, spec.slot_name);
    table_set(state, entry, "slotName", slot_name);
    table_set(state, entry, "isSecondary", Val::Bool(spec.is_secondary));
    Ok(entry)
}

fn collection_type(state: &mut LuaState, spec: OutfitSlotSpec) -> f64 {
    let Some(name) = spec.collection_type_name else {
        return spec.collection_type_fallback;
    };
    enum_variant_number(
        state,
        "TransmogCollectionType",
        name,
        spec.collection_type_fallback,
    )
}

fn enum_variant_number(
    state: &mut LuaState,
    enum_name: &'static str,
    variant: &'static str,
    fallback: f64,
) -> f64 {
    let enum_root = global_val(state, "Enum");
    let enum_table = table_get(state, enum_root, enum_name);
    match table_get(state, enum_table, variant) {
        Val::Num(value) => value,
        _ => fallback,
    }
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn outfit_slot_from_inventory_slot_preserves_valid_nonnegative_values() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let result: (i32, bool, bool) = env
            .eval(
                r#"
                return
                    C_TransmogOutfitInfo.GetTransmogOutfitSlotFromInventorySlot(16),
                    C_TransmogOutfitInfo.GetTransmogOutfitSlotFromInventorySlot(-1) == nil,
                    C_TransmogOutfitInfo.GetLinkedSlotInfo(16) == nil
                "#,
            )
            .expect("slot conversion should be queryable");

        assert_eq!(result, (16, true, true));
    }

    #[test]
    fn all_slot_location_info_matches_startup_contract() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let result: (i32, i32, String, i32, bool, String) = env
            .eval(
                r#"
                local appearanceSlotInfo, illusionSlotInfo = C_TransmogOutfitInfo.GetAllSlotLocationInfo()
                local first = appearanceSlotInfo[1]
                local offShoulder = appearanceSlotInfo[#appearanceSlotInfo]
                local firstIllusion = illusionSlotInfo[1]
                return
                    #appearanceSlotInfo,
                    #illusionSlotInfo,
                    first.slotName,
                    first.slot,
                    offShoulder.isSecondary,
                    firstIllusion.slotName
                "#,
            )
            .expect("slot location info should be queryable");

        assert_eq!(
            result,
            (
                14,
                2,
                "HEADSLOT".to_string(),
                0,
                true,
                "MAINHANDSLOT".to_string()
            )
        );
    }

    #[test]
    fn outfit_state_methods_track_active_outfit_and_pending_sheathe_categories() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let result: String = env
            .eval(
                r#"
                if C_TransmogOutfitInfo.GetActiveOutfitID() ~= 0 then return "active" end
                if C_TransmogOutfitInfo.GetCurrentlyViewedOutfitID() ~= 0 then return "viewed" end
                if C_TransmogOutfitInfo.GetOutfitInfo(7) ~= nil then return "outfit_info" end

                local categoryInfo = C_TransmogOutfitInfo.GetAllTransmogOutfitOptionSheatheCategoryInfo(190001)
                if #categoryInfo ~= 4 then return "category_count" end
                if categoryInfo[1].categoryName ~= "Default" then return "default_category" end
                if categoryInfo[4].categoryName ~= "Hide" then return "hide_category" end
                if C_TransmogOutfitInfo.GetAllTransmogOutfitOptionSheatheCategoryInfo(0) ~= nil then
                    return "unexpected_category"
                end

                C_TransmogOutfitInfo.ChangeToOutfit(7, false)
                if C_TransmogOutfitInfo.GetActiveOutfitID() ~= 7 then return "changed_active" end
                if C_TransmogOutfitInfo.GetCurrentlyViewedOutfitID() ~= 7 then return "changed_viewed" end

                C_TransmogOutfitInfo.SetPendingTransmogSheatheCategory(16, 2, Enum.TransmogOutfitSlotOptionSheatheCategory.Side)
                local pending = rawget(C_TransmogOutfitInfo, "__pendingSheatheCategories")
                if pending["16:2"] ~= Enum.TransmogOutfitSlotOptionSheatheCategory.Side then return "pending" end

                C_TransmogOutfitInfo.ChangeToOutfit(7, true)
                if C_TransmogOutfitInfo.GetActiveOutfitID() ~= 0 then return "cleared_active" end
                if next(rawget(C_TransmogOutfitInfo, "__pendingSheatheCategories")) ~= nil then return "cleared_pending" end

                C_TransmogOutfitInfo.ChangeToOutfit(9, false)
                C_TransmogOutfitInfo.ClearOutfit()
                if C_TransmogOutfitInfo.GetCurrentlyViewedOutfitID() ~= 0 then return "clear_outfit" end
                return "ok"
                "#,
            )
            .expect("outfit state methods should be queryable");

        assert_eq!(result, "ok");
    }
}
