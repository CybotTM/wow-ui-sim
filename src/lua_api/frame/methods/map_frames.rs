//! rilua implementations for map, minimap, fog-of-war, and unit-position APIs.

use crate::lua_api::frame::methods::core_state;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, frame_id_from_stack, get_or_create_frame_fields, table_get,
    table_set, val_to_string,
};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

const MINIMAP_ZOOM_KEY: &str = "__minimapZoom";
const MINIMAP_ZOOM_LEVELS: i32 = 6;

pub fn register_all(state: &mut LuaState, mt: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, mt, "GetUiMapID", get_ui_map_id)?;
    table_set_rust_fn_static(state, mt, "SetUiMapID", set_ui_map_id)?;
    table_set_rust_fn_static(
        state,
        mt,
        "GetFogOfWarBackgroundAtlas",
        get_fog_of_war_background_atlas,
    )?;
    table_set_rust_fn_static(state, mt, "GetFogOfWarMaskAtlas", get_fog_of_war_mask_atlas)?;
    table_set_rust_fn_static(state, mt, "GetMaskScalar", get_fog_of_war_mask_scalar)?;
    table_set_rust_fn_static(
        state,
        mt,
        "SetFogOfWarBackgroundAtlas",
        set_fog_of_war_background_atlas,
    )?;
    table_set_rust_fn_static(state, mt, "SetFogOfWarMaskAtlas", set_fog_of_war_mask_atlas)?;
    table_set_rust_fn_static(state, mt, "SetMaskScalar", set_fog_of_war_mask_scalar)?;
    table_set_rust_fn_static(state, mt, "ClearUnits", clear_units)?;
    table_set_rust_fn_static(state, mt, "AddUnit", add_unit)?;
    table_set_rust_fn_static(state, mt, "FinalizeUnits", finalize_units)?;
    table_set_rust_fn_static(state, mt, "SetUnitColor", set_unit_color)?;
    table_set_rust_fn_static(state, mt, "GetMouseOverUnits", get_mouse_over_units)?;
    table_set_rust_fn_static(state, mt, "GetPlayerPingScale", get_player_ping_scale)?;
    table_set_rust_fn_static(state, mt, "SetPlayerPingTexture", set_player_ping_texture)?;
    table_set_rust_fn_static(state, mt, "SetPlayerPingScale", set_player_ping_scale)?;
    table_set_rust_fn_static(state, mt, "StartPlayerPing", start_player_ping)?;
    table_set_rust_fn_static(state, mt, "StopPlayerPing", stop_player_ping)?;
    table_set_rust_fn_static(state, mt, "SetBlipTexture", set_blip_texture)?;
    table_set_rust_fn_static(state, mt, "SetMaskTexture", set_minimap_mask_texture)?;
    table_set_rust_fn_static(state, mt, "SetIconTexture", set_minimap_icon_texture)?;
    table_set_rust_fn_static(state, mt, "SetPOIArrowTexture", set_poi_arrow_texture)?;
    table_set_rust_fn_static(
        state,
        mt,
        "SetCorpsePOIArrowTexture",
        set_corpse_poi_arrow_texture,
    )?;
    table_set_rust_fn_static(
        state,
        mt,
        "SetStaticPOIArrowTexture",
        set_static_poi_arrow_texture,
    )?;
    table_set_rust_fn_static(state, mt, "SetPlayerTexture", set_minimap_player_texture)?;
    table_set_rust_fn_static(state, mt, "SetZoom", set_zoom)?;
    table_set_rust_fn_static(state, mt, "GetZoom", get_zoom)?;
    table_set_rust_fn_static(state, mt, "GetZoomLevels", get_zoom_levels)?;
    table_set_rust_fn_static(state, mt, "PingLocation", ping_location)?;
    table_set_rust_fn_static(state, mt, "GetPingPosition", get_ping_position)?;
    table_set_rust_fn_static(state, mt, "UpdateBlips", update_blips)?;
    table_set_rust_fn_static(state, mt, "SetToDefaults", set_to_defaults)?;
    Ok(())
}

fn get_ui_map_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let map_id = {
        let sim = borrow_state(state)?;
        if is_fog_of_war_frame(&sim, id) {
            sim.fog_of_war_frames
                .get(&id)
                .and_then(|fog| fog.ui_map_id)
                .unwrap_or(0)
        } else if is_unit_position_frame(&sim, id) {
            sim.unit_position_frames
                .get(&id)
                .and_then(|unit| unit.ui_map_id)
                .unwrap_or(0)
        } else {
            drop(sim);
            return core_state::get_ui_map_id(state);
        }
    };
    state.push(Val::Num(map_id as f64));
    Ok(1)
}

fn set_ui_map_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let map_id = stack_i32(state, 2).unwrap_or(0);
    let mut sim = borrow_state_mut(state)?;
    if is_fog_of_war_frame(&sim, id) {
        store_fog_of_war_ui_map_id(&mut sim, id, map_id);
        return Ok(0);
    }
    if is_unit_position_frame(&sim, id) {
        unit_position_state_mut(&mut sim, id).ui_map_id = Some(map_id);
        return Ok(0);
    }
    drop(sim);
    core_state::set_map_id(state)
}

fn get_fog_of_war_background_atlas(state: &mut LuaState) -> LuaResult<u32> {
    get_fog_of_war_atlas(state, |fog| fog.background_atlas.clone())
}

fn get_fog_of_war_mask_atlas(state: &mut LuaState) -> LuaResult<u32> {
    get_fog_of_war_atlas(state, |fog| fog.mask_atlas.clone())
}

fn get_fog_of_war_atlas<F>(state: &mut LuaState, read: F) -> LuaResult<u32>
where
    F: Fn(&crate::lua_api::state::FogOfWarFrameState) -> Option<String>,
{
    let id = frame_id_from_stack(state, 1)?;
    let atlas = {
        let sim = borrow_state(state)?;
        sim.fog_of_war_frames.get(&id).and_then(read)
    };
    match atlas {
        Some(value) => {
            let value = crate::lua_api::methods::create_string(state, &value);
            state.push(value);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn get_fog_of_war_mask_scalar(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let scalar = {
        let sim = borrow_state(state)?;
        sim.fog_of_war_frames
            .get(&id)
            .and_then(|fog| fog.mask_scalar)
            .unwrap_or(1.0)
    };
    state.push(Val::Num(scalar));
    Ok(1)
}

fn set_fog_of_war_background_atlas(state: &mut LuaState) -> LuaResult<u32> {
    set_fog_of_war_atlas(
        state,
        |fog, atlas| fog.background_atlas = atlas.clone(),
        |frame, atlas| {
            frame.fog_of_war_background_atlas = atlas;
        },
    )
}

fn set_fog_of_war_mask_atlas(state: &mut LuaState) -> LuaResult<u32> {
    set_fog_of_war_atlas(
        state,
        |fog, atlas| fog.mask_atlas = atlas.clone(),
        |frame, atlas| {
            frame.fog_of_war_mask_atlas = atlas;
        },
    )
}

fn set_fog_of_war_atlas<F, G>(
    state: &mut LuaState,
    write_state: F,
    write_frame: G,
) -> LuaResult<u32>
where
    F: Fn(&mut crate::lua_api::state::FogOfWarFrameState, &Option<String>),
    G: Fn(&mut crate::widget::Frame, Option<String>),
{
    let id = frame_id_from_stack(state, 1)?;
    let atlas = val_to_string(state, stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    let fog = sim.fog_of_war_frames.entry(id).or_default();
    write_state(fog, &atlas);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        write_frame(frame, atlas);
    }
    Ok(0)
}

fn set_fog_of_war_mask_scalar(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let scalar = stack_num(state, 2);
    let mut sim = borrow_state_mut(state)?;
    let fog = sim.fog_of_war_frames.entry(id).or_default();
    fog.mask_scalar = scalar;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.fog_of_war_mask_scalar = scalar.map(|value| value as f32);
    }
    Ok(0)
}

fn clear_units(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    let unit_state = unit_position_state_mut(&mut sim, id);
    unit_state.units.clear();
    unit_state.unit_colors.clear();
    unit_state.mouse_over_units.clear();
    unit_state.is_finalized = false;
    Ok(0)
}

fn add_unit(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::state::UnitPositionUnit;

    let id = frame_id_from_stack(state, 1)?;
    let Some(unit) = stack_string(state, 2) else {
        return Ok(0);
    };
    let asset = val_to_string(state, stack_val(state, 3));
    let width = stack_num(state, 4);
    let height = stack_num(state, 5);
    let color = stack_color(state, 6);
    let sublevel = stack_i32(state, 10);
    let show_facing = stack_bool(state, 11);

    let mut sim = borrow_state_mut(state)?;
    let unit_state = unit_position_state_mut(&mut sim, id);
    unit_state.units.push(UnitPositionUnit {
        unit,
        asset,
        width,
        height,
        color,
        sublevel,
        show_facing,
    });
    unit_state.is_finalized = false;
    Ok(0)
}

fn finalize_units(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    unit_position_state_mut(&mut sim, id).is_finalized = true;
    Ok(0)
}

fn set_unit_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(unit) = stack_string(state, 2) else {
        return Ok(0);
    };
    let Some(color) = stack_color(state, 3) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    let unit_state = unit_position_state_mut(&mut sim, id);
    unit_state.unit_colors.insert(unit.clone(), color);
    for pin in &mut unit_state.units {
        if pin.unit == unit {
            pin.color = Some(color);
        }
    }
    Ok(0)
}

fn get_mouse_over_units(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let units = {
        let sim = borrow_state(state)?;
        sim.unit_position_frames
            .get(&id)
            .map(|unit_state| unit_state.mouse_over_units.clone())
            .unwrap_or_default()
    };
    let count = units.len() as u32;
    for unit in units {
        let unit = crate::lua_api::methods::create_string(state, &unit);
        state.push(unit);
    }
    Ok(count)
}

fn get_player_ping_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let scale = {
        let sim = borrow_state(state)?;
        sim.unit_position_frames
            .get(&id)
            .map(|unit_state| unit_state.player_ping_scale)
            .unwrap_or(1.0)
    };
    state.push(Val::Num(scale));
    Ok(1)
}

fn set_player_ping_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(texture_type) = stack_i32(state, 2) else {
        return Ok(0);
    };
    let asset = val_to_string(state, stack_val(state, 3));
    let width = stack_num(state, 4).unwrap_or(0.0);
    let height = stack_num(state, 5).unwrap_or(0.0);
    let mut sim = borrow_state_mut(state)?;
    unit_position_state_mut(&mut sim, id)
        .player_ping_textures
        .insert(
            texture_type,
            crate::lua_api::state::UnitPositionPlayerPingTexture {
                asset,
                width,
                height,
            },
        );
    Ok(0)
}

fn set_player_ping_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let scale = stack_num(state, 2).unwrap_or(1.0);
    let mut sim = borrow_state_mut(state)?;
    unit_position_state_mut(&mut sim, id).player_ping_scale = scale;
    Ok(0)
}

fn start_player_ping(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let duration = stack_num(state, 2).unwrap_or(0.0);
    let fade_duration = stack_num(state, 3).unwrap_or(0.0);
    let mut sim = borrow_state_mut(state)?;
    let unit_state = unit_position_state_mut(&mut sim, id);
    unit_state.player_ping_active = true;
    unit_state.player_ping_duration = Some(duration);
    unit_state.player_ping_fade_duration = Some(fade_duration);
    Ok(0)
}

fn stop_player_ping(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    unit_position_state_mut(&mut sim, id).player_ping_active = false;
    Ok(0)
}

fn set_blip_texture(state: &mut LuaState) -> LuaResult<u32> {
    set_minimap_texture_field(state, |frame, texture| frame.minimap_blip_texture = texture)
}

fn set_minimap_mask_texture(state: &mut LuaState) -> LuaResult<u32> {
    set_minimap_texture_field(state, |frame, texture| frame.minimap_mask_texture = texture)
}

fn set_minimap_icon_texture(state: &mut LuaState) -> LuaResult<u32> {
    set_minimap_texture_field(state, |frame, texture| frame.minimap_icon_texture = texture)
}

fn set_minimap_player_texture(state: &mut LuaState) -> LuaResult<u32> {
    set_minimap_texture_field(state, |frame, texture| {
        frame.minimap_player_texture = texture
    })
}

fn set_poi_arrow_texture(state: &mut LuaState) -> LuaResult<u32> {
    set_minimap_texture_field(state, |frame, texture| {
        frame.minimap_poi_arrow_texture = texture
    })
}

fn set_corpse_poi_arrow_texture(state: &mut LuaState) -> LuaResult<u32> {
    set_minimap_texture_field(state, |frame, texture| {
        frame.minimap_corpse_poi_arrow_texture = texture
    })
}

fn set_static_poi_arrow_texture(state: &mut LuaState) -> LuaResult<u32> {
    set_minimap_texture_field(state, |frame, texture| {
        frame.minimap_static_poi_arrow_texture = texture
    })
}

fn set_minimap_texture_field<F>(state: &mut LuaState, write: F) -> LuaResult<u32>
where
    F: Fn(&mut crate::widget::Frame, Option<String>),
{
    let id = frame_id_from_stack(state, 1)?;
    let texture = val_to_string(state, stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        write(frame, texture);
    }
    Ok(0)
}

fn set_zoom(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let requested = stack_i32(state, 2).unwrap_or(0);
    let clamped = requested.clamp(0, MINIMAP_ZOOM_LEVELS - 1);
    let fields = get_or_create_frame_fields(state, id);
    table_set(state, fields, MINIMAP_ZOOM_KEY, Val::Num(clamped as f64));
    Ok(0)
}

fn get_zoom(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let fields = get_or_create_frame_fields(state, id);
    let zoom = match table_get(state, fields, MINIMAP_ZOOM_KEY) {
        Val::Num(value) => value as i32,
        _ => 0,
    };
    state.push(Val::Num(zoom as f64));
    Ok(1)
}

fn get_zoom_levels(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(MINIMAP_ZOOM_LEVELS as f64));
    Ok(1)
}

fn ping_location(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let x = stack_num(state, 2).unwrap_or(0.0) as f32;
    let y = stack_num(state, 3).unwrap_or(0.0) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.minimap_ping_position = Some((x, y));
    }
    Ok(0)
}

fn get_ping_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let (x, y) = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|frame| frame.minimap_ping_position)
            .unwrap_or((0.0, 0.0))
    };
    state.push(Val::Num(x as f64));
    state.push(Val::Num(y as f64));
    Ok(2)
}

fn update_blips(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.minimap_blip_update_revision = frame.minimap_blip_update_revision.saturating_add(1);
    }
    Ok(0)
}

fn set_to_defaults(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.minimap_blip_texture = None;
            frame.minimap_mask_texture = None;
            frame.minimap_icon_texture = None;
            frame.minimap_player_texture = None;
            frame.minimap_poi_arrow_texture = None;
            frame.minimap_corpse_poi_arrow_texture = None;
            frame.minimap_static_poi_arrow_texture = None;
            frame.minimap_ping_position = None;
            frame.minimap_blip_update_revision = 0;
            frame.quest_blob_inside = Default::default();
            frame.quest_blob_outside = Default::default();
            frame.quest_blob_ring = Default::default();
            frame.task_blob_inside = Default::default();
            frame.task_blob_outside = Default::default();
            frame.task_blob_ring = Default::default();
            frame.arch_blob_inside = Default::default();
            frame.arch_blob_outside = Default::default();
            frame.arch_blob_ring = Default::default();
        }
    }
    let fields = get_or_create_frame_fields(state, id);
    table_set(state, fields, MINIMAP_ZOOM_KEY, Val::Num(0.0));
    Ok(0)
}

fn is_fog_of_war_frame(sim: &crate::lua_api::SimState, id: u64) -> bool {
    sim.widgets
        .get(id)
        .and_then(|frame| frame.object_type_name.as_deref())
        .is_some_and(|name| name.eq_ignore_ascii_case("FogOfWarFrame"))
}

fn is_unit_position_frame(sim: &crate::lua_api::SimState, id: u64) -> bool {
    sim.widgets
        .get(id)
        .and_then(|frame| frame.object_type_name.as_deref())
        .is_some_and(|name| name.eq_ignore_ascii_case("UnitPositionFrame"))
}

fn store_fog_of_war_ui_map_id(sim: &mut crate::lua_api::SimState, id: u64, map_id: i32) {
    let fog_state = sim.fog_of_war_frames.entry(id).or_default();
    fog_state.ui_map_id = Some(map_id);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.fog_of_war_ui_map_id = Some(map_id);
    }
}

fn unit_position_state_mut(
    sim: &mut crate::lua_api::SimState,
    id: u64,
) -> &mut crate::lua_api::state::UnitPositionFrameState {
    sim.unit_position_frames.entry(id).or_insert_with(|| {
        crate::lua_api::state::UnitPositionFrameState {
            ui_map_id: None,
            units: Vec::new(),
            unit_colors: std::collections::HashMap::new(),
            mouse_over_units: Vec::new(),
            player_ping_scale: 1.0,
            player_ping_textures: std::collections::HashMap::new(),
            player_ping_active: false,
            player_ping_duration: None,
            player_ping_fade_duration: None,
            is_finalized: false,
        }
    })
}

fn stack_string(state: &LuaState, index: i32) -> Option<String> {
    val_to_string(state, stack_val(state, index))
}

fn stack_num(state: &LuaState, index: i32) -> Option<f64> {
    match stack_val(state, index) {
        Val::Num(value) => Some(value),
        _ => None,
    }
}

fn stack_i32(state: &LuaState, index: i32) -> Option<i32> {
    stack_num(state, index).map(|value| value as i32)
}

fn stack_bool(state: &LuaState, index: i32) -> Option<bool> {
    match stack_val(state, index) {
        Val::Bool(value) => Some(value),
        _ => None,
    }
}

fn stack_color(state: &LuaState, start_index: i32) -> Option<(f64, f64, f64, f64)> {
    Some((
        stack_num(state, start_index)?,
        stack_num(state, start_index + 1)?,
        stack_num(state, start_index + 2)?,
        stack_num(state, start_index + 3)?,
    ))
}
