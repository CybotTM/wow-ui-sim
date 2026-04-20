//! `C_TransmogCollection` probe surface backed by `WorldState.transmog_appearances`
//! and `WorldState.collected_transmogs`.

use super::ensure_namespace;
use crate::lua_api::methods::{
    borrow_state, create_string, create_table, table_set, table_set_num, val_to_string,
};
use crate::lua_api::state_types::TransmogAppearance;
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const MAX_OUTFITS: i32 = 20;

pub(super) fn register_transmog_collection_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_TransmogCollection")?;
    register_transmog_collection_appearance_queries(state, table_ref)?;
    register_transmog_collection_category_queries(state, table_ref)?;
    register_transmog_collection_flags(state, table_ref)?;
    register_transmog_collection_outfits(state, table_ref)?;
    Ok(())
}

fn register_transmog_collection_appearance_queries(
    state: &mut LuaState,
    table_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetAppearanceSources",
        get_appearance_sources,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetSourceInfo", get_source_info)?;
    table_set_rust_fn_static(state, table_ref, "PlayerHasTransmog", player_has_transmog)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "PlayerHasTransmogByItemInfo",
        player_has_transmog_by_item_info,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "PlayerHasTransmogItemModifiedAppearance",
        player_has_transmog_item_modified_appearance,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetNumTransmogSources",
        get_num_transmog_sources,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetAllAppearanceSources",
        get_all_appearance_sources,
    )?;
    Ok(())
}

fn register_transmog_collection_category_queries(
    state: &mut LuaState,
    table_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetCategoryAppearances",
        get_category_appearances,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetCategoryInfo", get_category_info)?;
    table_set_rust_fn_static(state, table_ref, "PlayerKnowsSource", player_knows_source)?;
    Ok(())
}

fn register_transmog_collection_flags(
    state: &mut LuaState,
    table_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table_ref,
        "IsAppearanceHiddenVisual",
        is_appearance_hidden_visual,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "IsSourceTypeFilterChecked",
        is_source_type_filter_checked,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetShowMissingSourceInItemTooltips",
        get_show_missing_source_in_item_tooltips,
    )?;
    Ok(())
}

fn register_transmog_collection_outfits(
    state: &mut LuaState,
    table_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(state, table_ref, "GetIllusions", get_illusions)?;
    table_set_rust_fn_static(state, table_ref, "GetOutfits", get_outfits)?;
    table_set_rust_fn_static(state, table_ref, "GetNumMaxOutfits", get_num_max_outfits)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetAppearanceCameraID",
        get_appearance_camera_id,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetOutfitInfo", get_outfit_info)?;
    Ok(())
}

fn get_appearance_sources(state: &mut LuaState) -> LuaResult<u32> {
    let visual_id = i32::from_stack(state, 1)?;
    let appearances = transmog_appearances(state, Some(visual_id), None);
    let array = create_table(state);
    let Val::Table(array_ref) = array else {
        state.push(Val::Nil);
        return Ok(1);
    };

    for (index, appearance) in appearances.iter().enumerate() {
        let row = appearance_row(state, appearance, None);
        table_set_num(state, array_ref, (index + 1) as f64, row);
    }

    state.push(array);
    Ok(1)
}

fn get_source_info(state: &mut LuaState) -> LuaResult<u32> {
    let source_id = i32::from_stack(state, 1)?;
    let appearance = transmog_appearance(state, source_id).or_else(|| {
        if has_collected_transmog(state, source_id) {
            Some(TransmogAppearance {
                source_id,
                visual_id: 0,
                category_id: 0,
                item_id: source_id,
                is_collected: true,
                source_type: 0,
                item_mod_id: 0,
            })
        } else {
            None
        }
    });

    let Some(appearance) = appearance else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let row = appearance_row(state, &appearance, None);
    state.push(row);
    Ok(1)
}

fn player_has_transmog(state: &mut LuaState) -> LuaResult<u32> {
    let source_id = i32::from_stack(state, 1)?;
    state.push(Val::Bool(has_collected_transmog(state, source_id)));
    Ok(1)
}

fn player_has_transmog_by_item_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(item_id) = parse_item_id_from_val(state, stack_val(state, 1)) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    state.push(Val::Bool(has_collected_transmog(state, item_id as i32)));
    Ok(1)
}

fn player_has_transmog_item_modified_appearance(state: &mut LuaState) -> LuaResult<u32> {
    let appearance_id = i32::from_stack(state, 1)?;
    state.push(Val::Bool(has_collected_transmog(state, appearance_id)));
    Ok(1)
}

fn get_num_transmog_sources(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.world.transmog_appearances.len() as i32;
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn get_all_appearance_sources(state: &mut LuaState) -> LuaResult<u32> {
    let array = empty_array(state);
    state.push(array);
    Ok(1)
}

fn get_category_appearances(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = i32::from_stack(state, 1)?;
    let appearances = transmog_appearances(state, None, Some(category_id));
    let array = create_table(state);
    let Val::Table(array_ref) = array else {
        state.push(Val::Nil);
        return Ok(1);
    };

    for (index, appearance) in appearances.iter().enumerate() {
        let row = appearance_row(state, appearance, Some((index + 1) as i32));
        table_set_num(state, array_ref, (index + 1) as f64, row);
    }

    state.push(array);
    Ok(1)
}

fn get_category_info(state: &mut LuaState) -> LuaResult<u32> {
    let category_id = i32::from_stack(state, 1)?;
    let (name, is_weapon, can_enchant, can_main_hand, can_off_hand) = category_info(category_id);
    let category_name = create_string(state, name);
    state.push(category_name);
    state.push(Val::Bool(is_weapon));
    state.push(Val::Bool(can_enchant));
    state.push(Val::Bool(can_main_hand));
    state.push(Val::Bool(can_off_hand));
    Ok(5)
}

fn player_knows_source(state: &mut LuaState) -> LuaResult<u32> {
    let _source_id = i32::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn is_appearance_hidden_visual(state: &mut LuaState) -> LuaResult<u32> {
    let _visual_id = i32::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn is_source_type_filter_checked(state: &mut LuaState) -> LuaResult<u32> {
    let _source_type = i32::from_stack(state, 1)?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn get_show_missing_source_in_item_tooltips(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn get_illusions(state: &mut LuaState) -> LuaResult<u32> {
    let array = empty_array(state);
    state.push(array);
    Ok(1)
}

fn get_outfits(state: &mut LuaState) -> LuaResult<u32> {
    let array = empty_array(state);
    state.push(array);
    Ok(1)
}

fn get_num_max_outfits(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(MAX_OUTFITS as f64));
    Ok(1)
}

fn get_appearance_camera_id(state: &mut LuaState) -> LuaResult<u32> {
    let _appearance_id = i32::from_stack(state, 1)?;
    state.push(Val::Num(0.0));
    Ok(1)
}

fn get_outfit_info(state: &mut LuaState) -> LuaResult<u32> {
    let _outfit_id = i32::from_stack(state, 1)?;
    state.push(Val::Nil);
    Ok(1)
}

fn appearance_row(
    state: &mut LuaState,
    appearance: &TransmogAppearance,
    ui_order: Option<i32>,
) -> Val {
    let row = create_table(state);
    table_set(
        state,
        row,
        "sourceID",
        Val::Num(appearance.source_id as f64),
    );
    table_set(
        state,
        row,
        "visualID",
        Val::Num(appearance.visual_id as f64),
    );
    table_set(
        state,
        row,
        "categoryID",
        Val::Num(appearance.category_id as f64),
    );
    table_set(state, row, "itemID", Val::Num(appearance.item_id as f64));
    table_set(
        state,
        row,
        "isCollected",
        Val::Bool(appearance.is_collected),
    );
    table_set(
        state,
        row,
        "sourceType",
        Val::Num(appearance.source_type as f64),
    );
    table_set(
        state,
        row,
        "itemModID",
        Val::Num(appearance.item_mod_id as f64),
    );
    if let Some(ui_order) = ui_order {
        table_set(state, row, "uiOrder", Val::Num(ui_order as f64));
    }
    row
}

fn empty_array(state: &mut LuaState) -> Val {
    create_table(state)
}

fn has_collected_transmog(state: &LuaState, id: i32) -> bool {
    borrow_state(state)
        .map(|sim| sim.world.collected_transmogs.contains(&id))
        .unwrap_or(false)
}

fn transmog_appearance(state: &LuaState, source_id: i32) -> Option<TransmogAppearance> {
    borrow_state(state)
        .ok()?
        .world
        .transmog_appearances
        .iter()
        .find(|appearance| appearance.source_id == source_id)
        .cloned()
}

fn transmog_appearances(
    state: &LuaState,
    visual_id: Option<i32>,
    category_id: Option<i32>,
) -> Vec<TransmogAppearance> {
    borrow_state(state)
        .ok()
        .map(|sim| {
            sim.world
                .transmog_appearances
                .iter()
                .filter(|appearance| visual_id.is_none_or(|id| appearance.visual_id == id))
                .filter(|appearance| category_id.is_none_or(|id| appearance.category_id == id))
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn category_info(category_id: i32) -> (&'static str, bool, bool, bool, bool) {
    match category_id {
        1 => ("Head", false, false, false, false),
        2 => ("Shoulder", false, false, false, false),
        3 => ("Back", false, false, false, false),
        4 => ("Chest", false, false, false, false),
        5 => ("Shirt", false, false, false, false),
        6 => ("Tabard", false, false, false, false),
        7 => ("Wrist", false, false, false, false),
        8 => ("Hands", false, false, false, false),
        9 => ("Waist", false, false, false, false),
        10 => ("Legs", false, false, false, false),
        11 => ("Feet", false, false, false, false),
        14 => ("One-Handed Swords", true, true, true, true),
        18 => ("Shield", true, false, false, true),
        23 => ("Staff", true, true, true, false),
        _ => ("", false, false, false, false),
    }
}

fn parse_item_id_from_val(state: &LuaState, value: Val) -> Option<u32> {
    match value {
        Val::Num(number) if number > 0.0 => Some(number as u32),
        Val::Str(_) => {
            let text = val_to_string(state, value)?;
            parse_prefixed_id(&text, "item").or_else(|| text.parse().ok())
        }
        _ => None,
    }
}

fn parse_prefixed_id(value: &str, prefix: &str) -> Option<u32> {
    let prefixed = format!("|H{prefix}:");
    if let Some(start) = value.find(&prefixed) {
        let digits: String = value[start + prefixed.len()..]
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .collect();
        return digits.parse().ok();
    }

    let bare = format!("{prefix}:");
    if let Some(start) = value.find(&bare) {
        let digits: String = value[start + bare.len()..]
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .collect();
        return digits.parse().ok();
    }

    None
}
