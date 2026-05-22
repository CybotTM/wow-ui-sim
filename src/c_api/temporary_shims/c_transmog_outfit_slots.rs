//! Temporary `C_TransmogOutfitInfo` slot-location fallback surface.
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

const APPEARANCE_TYPE_FALLBACK: f64 = 0.0;
const ILLUSION_TYPE_FALLBACK: f64 = 1.0;
const NONE_COLLECTION_TYPE_FALLBACK: f64 = 0.0;

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
}
