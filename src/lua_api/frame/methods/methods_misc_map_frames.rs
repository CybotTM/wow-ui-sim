//! Specialized map-related frame methods (FogOfWar, UnitPosition, UiMapID).

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::get_sim_state;
use mlua::{MultiValue, Value};

pub(super) fn add_map_specialized_frame_methods<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
) {
    add_ui_map_id_methods(methods);
    add_fog_of_war_frame_methods(methods);
    add_unit_position_frame_methods(methods);
}

fn add_ui_map_id_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetUiMapID", |lua, this, ()| {
        Ok(read_ui_map_id(get_sim_state(lua), this.0))
    });
    methods.add_method("SetUiMapID", |lua, this, map_id: i32| {
        store_ui_map_id(get_sim_state(lua), this.0, map_id);
        Ok(())
    });
}

fn add_fog_of_war_frame_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_fog_of_war_atlas_getter(methods, "GetFogOfWarBackgroundAtlas", |fog| {
        fog.background_atlas.clone()
    });
    add_fog_of_war_atlas_getter(methods, "GetFogOfWarMaskAtlas", |fog| {
        fog.mask_atlas.clone()
    });
    add_fog_of_war_mask_scalar_getter(methods);
    add_fog_of_war_atlas_setter(
        methods,
        "SetFogOfWarBackgroundAtlas",
        |fog, atlas| {
            fog.background_atlas = atlas.clone();
        },
        |frame, atlas| {
            frame.fog_of_war_background_atlas = atlas;
        },
    );
    add_fog_of_war_atlas_setter(
        methods,
        "SetFogOfWarMaskAtlas",
        |fog, atlas| {
            fog.mask_atlas = atlas.clone();
        },
        |frame, atlas| {
            frame.fog_of_war_mask_atlas = atlas;
        },
    );
    add_fog_of_war_mask_scalar_setter(methods);
}

fn add_fog_of_war_atlas_getter<M, F>(methods: &mut M, name: &'static str, read: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&crate::lua_api::state::FogOfWarFrameState) -> Option<String> + Copy + 'static,
{
    methods.add_method(name, move |lua, this, ()| {
        let atlas = read_fog_of_war_frame_state(get_sim_state(lua), this.0, read);
        fog_of_war_string_value(lua, atlas)
    });
}

fn add_fog_of_war_mask_scalar_getter<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetMaskScalar", |lua, this, ()| {
        Ok(
            read_fog_of_war_frame_state(get_sim_state(lua), this.0, |fog| fog.mask_scalar)
                .unwrap_or(1.0),
        )
    });
}

fn add_fog_of_war_atlas_setter<M, F, G>(
    methods: &mut M,
    name: &'static str,
    write_state: F,
    write_frame: G,
) where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::lua_api::state::FogOfWarFrameState, &Option<String>) + Copy + 'static,
    G: Fn(&mut crate::widget::Frame, Option<String>) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, atlas: Value| {
        let atlas = super::methods_misc::texture_asset_to_string(&atlas)?;
        write_fog_of_war_atlas(get_sim_state(lua), this.0, atlas, write_state, write_frame);
        Ok(())
    });
}

fn add_fog_of_war_mask_scalar_setter<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetMaskScalar", |lua, this, scalar: Option<f64>| {
        write_fog_of_war_mask_scalar(get_sim_state(lua), this.0, scalar);
        Ok(())
    });
}

fn write_fog_of_war_atlas<F, G>(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    atlas: Option<String>,
    write_state: F,
    write_frame: G,
) where
    F: Fn(&mut crate::lua_api::state::FogOfWarFrameState, &Option<String>),
    G: Fn(&mut crate::widget::Frame, Option<String>),
{
    write_fog_of_war_frame_state(std::rc::Rc::clone(&state_rc), frame_id, |fog| {
        write_state(fog, &atlas);
    });
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
        write_frame(frame, atlas);
    }
}

fn write_fog_of_war_mask_scalar(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    scalar: Option<f64>,
) {
    write_fog_of_war_frame_state(std::rc::Rc::clone(&state_rc), frame_id, |fog| {
        fog.mask_scalar = scalar;
    });
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
        frame.fog_of_war_mask_scalar = scalar.map(|value| value as f32);
    }
}

fn fog_of_war_string_value(lua: &mlua::Lua, value: Option<String>) -> mlua::Result<Value> {
    match value {
        Some(value) => Ok(Value::String(lua.create_string(&value)?)),
        None => Ok(Value::Nil),
    }
}

fn read_fog_of_war_frame_state<T, F>(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    read: F,
) -> T
where
    F: FnOnce(&crate::lua_api::state::FogOfWarFrameState) -> T,
    T: Default,
{
    let state = state_rc.borrow();
    state
        .fog_of_war_frames
        .get(&frame_id)
        .map(read)
        .unwrap_or_default()
}

fn write_fog_of_war_frame_state<F>(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    write: F,
) where
    F: FnOnce(&mut crate::lua_api::state::FogOfWarFrameState),
{
    let mut state = state_rc.borrow_mut();
    let fog = state.fog_of_war_frames.entry(frame_id).or_default();
    write(fog);
}

fn add_unit_position_frame_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_unit_position_clear_units_method(methods);
    add_unit_position_add_unit_method(methods);
    add_unit_position_finalize_units_method(methods);
    add_unit_position_set_unit_color_method(methods);
    add_unit_position_get_mouse_over_units_method(methods);
    add_unit_position_ping_methods(methods);
}

fn add_unit_position_clear_units_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("ClearUnits", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let unit_state = unit_position_state_mut(&mut state, this.0);
        unit_state.units.clear();
        unit_state.unit_colors.clear();
        unit_state.mouse_over_units.clear();
        unit_state.is_finalized = false;
        Ok(())
    });
}

fn add_unit_position_add_unit_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    use crate::lua_api::state::UnitPositionUnit;

    methods.add_method("AddUnit", |lua, this, args: mlua::MultiValue| {
        let Some(unit) = multi_value_string_arg(&args, 0) else {
            return Ok(());
        };
        let asset = multi_value_texture_arg(&args, 1)?;
        let width = multi_value_number_arg(&args, 2);
        let height = multi_value_number_arg(&args, 3);
        let color = multi_value_color_arg(&args, 4);
        let sublevel = multi_value_i32_arg(&args, 8);
        let show_facing = multi_value_bool_arg(&args, 9);

        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let unit_state = unit_position_state_mut(&mut state, this.0);
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
        Ok(())
    });
}

fn add_unit_position_finalize_units_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("FinalizeUnits", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        unit_position_state_mut(&mut state, this.0).is_finalized = true;
        Ok(())
    });
}

fn store_ui_map_id(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    map_id: i32,
) {
    let mut state = state_rc.borrow_mut();
    if is_fog_of_war_frame(&state, frame_id) {
        let fog_state = state.fog_of_war_frames.entry(frame_id).or_default();
        fog_state.ui_map_id = Some(map_id);
        apply_fog_of_war_map_change(&mut state, frame_id, map_id);
        return;
    }

    let unit_state = state
        .unit_position_frames
        .entry(frame_id)
        .or_insert_with(new_unit_position_frame_state);
    unit_state.ui_map_id = Some(map_id);
}

fn add_unit_position_set_unit_color_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetUnitColor", |lua, this, args: mlua::MultiValue| {
        let Some(unit) = multi_value_string_arg(&args, 0) else {
            return Ok(());
        };
        let Some(color) = multi_value_color_arg(&args, 1) else {
            return Ok(());
        };

        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let unit_state = unit_position_state_mut(&mut state, this.0);
        unit_state.unit_colors.insert(unit.clone(), color);
        update_unit_pin_color(unit_state, &unit, color);
        Ok(())
    });
}

fn add_unit_position_get_mouse_over_units_method<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
) {
    // Blizzard's UnitPositionFrame expects varargs unit tokens from
    // GetMouseOverUnits(); return no values when no units are hovered.
    methods.add_method("GetMouseOverUnits", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let Some(unit_state) = state.unit_position_frames.get(&this.0) else {
            return Ok(MultiValue::new());
        };
        let mut result = MultiValue::new();
        for unit in &unit_state.mouse_over_units {
            result.push_back(Value::String(lua.create_string(unit)?));
        }
        Ok(result)
    });
}

fn new_unit_position_frame_state() -> crate::lua_api::state::UnitPositionFrameState {
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
}

fn unit_position_state_mut(
    state: &mut crate::lua_api::SimState,
    frame_id: u64,
) -> &mut crate::lua_api::state::UnitPositionFrameState {
    state
        .unit_position_frames
        .entry(frame_id)
        .or_insert_with(new_unit_position_frame_state)
}

fn add_unit_position_ping_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetPlayerPingScale", |lua, this, ()| {
        Ok(read_unit_position_ping_scale(get_sim_state(lua), this.0))
    });
    methods.add_method(
        "SetPlayerPingTexture",
        |lua, this, args: mlua::MultiValue| {
            write_unit_position_ping_texture(get_sim_state(lua), this.0, args)
        },
    );
    methods.add_method("SetPlayerPingScale", |lua, this, scale: f64| {
        write_unit_position_ping_scale(get_sim_state(lua), this.0, scale);
        Ok(())
    });
    methods.add_method("StartPlayerPing", |lua, this, args: mlua::MultiValue| {
        start_unit_position_ping(get_sim_state(lua), this.0, args);
        Ok(())
    });
    methods.add_method("StopPlayerPing", |lua, this, ()| {
        stop_unit_position_ping(get_sim_state(lua), this.0);
        Ok(())
    });
}

fn read_unit_position_ping_scale(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
) -> f64 {
    let state = state_rc.borrow();
    state
        .unit_position_frames
        .get(&frame_id)
        .map(|unit_state| unit_state.player_ping_scale)
        .unwrap_or(1.0)
}

fn read_ui_map_id(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
) -> i32 {
    let state = state_rc.borrow();
    if is_fog_of_war_frame(&state, frame_id) {
        return state
            .fog_of_war_frames
            .get(&frame_id)
            .and_then(|fog_state| fog_state.ui_map_id)
            .unwrap_or(0);
    }

    state
        .unit_position_frames
        .get(&frame_id)
        .and_then(|unit_state| unit_state.ui_map_id)
        .unwrap_or(0)
}

fn is_fog_of_war_frame(state: &crate::lua_api::SimState, frame_id: u64) -> bool {
    state
        .widgets
        .get(frame_id)
        .and_then(|frame| frame.object_type_name.as_deref())
        .is_some_and(|name| name.eq_ignore_ascii_case("FogOfWarFrame"))
}

fn apply_fog_of_war_map_change(state: &mut crate::lua_api::SimState, frame_id: u64, map_id: i32) {
    let fog_id = crate::lua_api::globals::c_map_api::fog_of_war_id_for_map(map_id);

    let Some(fog_id) = fog_id else {
        write_fog_of_war_ui_map_id(state, frame_id, Some(map_id));
        clear_fog_of_war_visuals(state, frame_id);
        return;
    };

    let Some(fog_info) = crate::lua_api::globals::c_map_api::fog_of_war_info_for_id(fog_id) else {
        write_fog_of_war_ui_map_id(state, frame_id, Some(map_id));
        clear_fog_of_war_visuals(state, frame_id);
        return;
    };

    write_fog_of_war_ui_map_id(state, frame_id, Some(map_id));
    let fog_state = state.fog_of_war_frames.entry(frame_id).or_default();
    fog_state.background_atlas = fog_info.background_atlas.map(str::to_string);
    fog_state.mask_atlas = fog_info.mask_atlas.map(str::to_string);
    fog_state.mask_scalar = Some(fog_info.mask_scalar);
    if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
        frame.fog_of_war_ui_map_id = Some(map_id);
        frame.fog_of_war_background_atlas = fog_state.background_atlas.clone();
        frame.fog_of_war_mask_atlas = fog_state.mask_atlas.clone();
        frame.fog_of_war_mask_scalar = Some(fog_info.mask_scalar as f32);
    }
    state.set_frame_visible(frame_id, true);
}

fn write_fog_of_war_ui_map_id(
    state: &mut crate::lua_api::SimState,
    frame_id: u64,
    map_id: Option<i32>,
) {
    let fog_state = state.fog_of_war_frames.entry(frame_id).or_default();
    fog_state.ui_map_id = map_id;
    if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
        frame.fog_of_war_ui_map_id = map_id;
    }
}

fn clear_fog_of_war_visuals(state: &mut crate::lua_api::SimState, frame_id: u64) {
    let fog_state = state.fog_of_war_frames.entry(frame_id).or_default();
    fog_state.background_atlas = None;
    fog_state.mask_atlas = None;
    fog_state.mask_scalar = None;
    if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
        frame.fog_of_war_background_atlas = None;
        frame.fog_of_war_mask_atlas = None;
        frame.fog_of_war_mask_scalar = None;
    }
    state.set_frame_visible(frame_id, false);
}

fn write_unit_position_ping_scale(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    scale: f64,
) {
    let mut state = state_rc.borrow_mut();
    unit_position_state_mut(&mut state, frame_id).player_ping_scale = scale;
}

fn write_unit_position_ping_texture(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    args: mlua::MultiValue,
) -> mlua::Result<()> {
    let Some(texture_type) = multi_value_i32_arg(&args, 0) else {
        return Ok(());
    };
    let asset = multi_value_texture_arg(&args, 1)?;
    let width = multi_value_number_arg(&args, 2).unwrap_or(0.0);
    let height = multi_value_number_arg(&args, 3).unwrap_or(0.0);

    let mut state = state_rc.borrow_mut();
    let unit_state = unit_position_state_mut(&mut state, frame_id);
    unit_state.player_ping_textures.insert(
        texture_type,
        crate::lua_api::state::UnitPositionPlayerPingTexture {
            asset,
            width,
            height,
        },
    );
    Ok(())
}

fn start_unit_position_ping(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    args: mlua::MultiValue,
) {
    let duration = multi_value_number_arg(&args, 0).unwrap_or(0.0);
    let fade_duration = multi_value_number_arg(&args, 1).unwrap_or(0.0);

    let mut state = state_rc.borrow_mut();
    let unit_state = unit_position_state_mut(&mut state, frame_id);
    unit_state.player_ping_active = true;
    unit_state.player_ping_duration = Some(duration);
    unit_state.player_ping_fade_duration = Some(fade_duration);
}

fn stop_unit_position_ping(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
) {
    let mut state = state_rc.borrow_mut();
    unit_position_state_mut(&mut state, frame_id).player_ping_active = false;
}

fn multi_value_string_arg(args: &MultiValue, index: usize) -> Option<String> {
    match args.get(index) {
        Some(Value::String(value)) => Some(value.to_string_lossy().to_string()),
        _ => None,
    }
}

fn multi_value_texture_arg(args: &MultiValue, index: usize) -> mlua::Result<Option<String>> {
    match args.get(index) {
        Some(value) => super::methods_misc::texture_asset_to_string(value),
        None => Ok(None),
    }
}

fn multi_value_number_arg(args: &MultiValue, index: usize) -> Option<f64> {
    match args.get(index) {
        Some(Value::Integer(value)) => Some(*value as f64),
        Some(Value::Number(value)) => Some(*value),
        _ => None,
    }
}

fn multi_value_i32_arg(args: &MultiValue, index: usize) -> Option<i32> {
    match args.get(index) {
        Some(Value::Integer(value)) => Some(*value as i32),
        Some(Value::Number(value)) => Some(*value as i32),
        _ => None,
    }
}

fn multi_value_bool_arg(args: &MultiValue, index: usize) -> Option<bool> {
    match args.get(index) {
        Some(Value::Boolean(value)) => Some(*value),
        _ => None,
    }
}

fn multi_value_color_arg(args: &MultiValue, start_index: usize) -> Option<(f64, f64, f64, f64)> {
    Some((
        multi_value_number_arg(args, start_index)?,
        multi_value_number_arg(args, start_index + 1)?,
        multi_value_number_arg(args, start_index + 2)?,
        multi_value_number_arg(args, start_index + 3)?,
    ))
}

fn update_unit_pin_color(
    unit_state: &mut crate::lua_api::state::UnitPositionFrameState,
    unit: &str,
    color: (f64, f64, f64, f64),
) {
    for pin in &mut unit_state.units {
        if pin.unit == unit {
            pin.color = Some(color);
        }
    }
}
