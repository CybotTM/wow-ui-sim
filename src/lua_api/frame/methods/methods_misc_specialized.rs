//! Specialized frame type methods (QuestPOI, FogOfWar, UnitPosition, Menu, etc.).

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::get_sim_state;
use mlua::{MultiValue, Value};

/// Methods for specialized frame types (QuestPOI, FogOfWar, UnitPosition, etc.).
pub(super) fn add_specialized_frame_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_menu_frame_stubs(methods);
    add_quest_poi_frame_methods(methods);
    add_ui_map_id_methods(methods);
    add_fog_of_war_frame_methods(methods);
    add_quest_blob_methods(methods);
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

fn add_menu_frame_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_is_menu_open_method(methods);
    add_set_owning_dialog_method(methods);
    add_register_menu_values_method(methods, "RegisterFontStrings", "fontStrings");
    add_register_menu_values_method(methods, "RegisterFrames", "frames");
    add_register_background_texture_method(methods);
}

fn add_is_menu_open_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsMenuOpen", |lua, this, ()| {
        let id = this.0;
        if let Some((func, ud)) = super::methods_helpers::get_mixin_override(lua, id, "IsMenuOpen")
        {
            return func.call::<bool>(ud);
        }
        menu_frame_is_menu_open(lua, id)
    });
}

fn add_set_owning_dialog_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetOwningDialog", |lua, this, dialog: Value| {
        let id = this.0;
        if let Some((func, ud)) =
            super::methods_helpers::get_mixin_override(lua, id, "SetOwningDialog")
        {
            return func.call::<()>((ud, dialog));
        }
        store_menu_frame_owning_dialog(lua, id, dialog)
    });
}

fn add_register_menu_values_method<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    method_name: &'static str,
    field_name: &'static str,
) {
    methods.add_method(method_name, move |lua, this, args: MultiValue| {
        let id = this.0;
        if call_menu_frame_override(lua, id, method_name, &args)? {
            return Ok(());
        }
        register_menu_frame_values(lua, id, field_name, args)
    });
}

fn add_register_background_texture_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "RegisterBackgroundTexture",
        |lua, this, args: MultiValue| {
            let id = this.0;
            if call_menu_frame_override(lua, id, "RegisterBackgroundTexture", &args)? {
                return Ok(());
            }
            store_background_texture_registration(lua, id, args)
        },
    );
}

fn menu_frame_is_menu_open(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<bool> {
    let fields = super::methods_misc::frame_fields(lua, frame_id)?;
    Ok(!matches!(fields.get::<Value>("menu")?, Value::Nil))
}

fn call_menu_frame_override(
    lua: &mlua::Lua,
    frame_id: u64,
    method_name: &str,
    args: &MultiValue,
) -> mlua::Result<bool> {
    let Some((func, ud)) = super::methods_helpers::get_mixin_override(lua, frame_id, method_name)
    else {
        return Ok(false);
    };
    let mut call_args = MultiValue::new();
    call_args.push_back(ud);
    for value in args.iter().cloned() {
        call_args.push_back(value);
    }
    func.call::<()>(call_args)?;
    Ok(true)
}

fn store_menu_frame_owning_dialog(
    lua: &mlua::Lua,
    frame_id: u64,
    dialog: Value,
) -> mlua::Result<()> {
    super::methods_misc::frame_fields(lua, frame_id)?.set("owningDialog", dialog)?;
    Ok(())
}

fn register_menu_frame_values(
    lua: &mlua::Lua,
    frame_id: u64,
    field_name: &str,
    args: MultiValue,
) -> mlua::Result<()> {
    let fields = super::methods_misc::frame_fields(lua, frame_id)?;
    let registered = lua.create_table()?;
    for (index, value) in args.into_iter().enumerate() {
        registered.raw_set(index + 1, value)?;
    }
    fields.set(field_name, registered)?;
    Ok(())
}

fn store_background_texture_registration(
    lua: &mlua::Lua,
    frame_id: u64,
    args: MultiValue,
) -> mlua::Result<()> {
    let mut args = args.into_iter();
    let texture = args.next().unwrap_or(Value::Nil);
    let texture_kit = args.next().unwrap_or(Value::Nil);
    let fields = super::methods_misc::frame_fields(lua, frame_id)?;
    fields.set("backgroundTexture", texture)?;
    fields.set("textureKit", texture_kit)?;
    Ok(())
}

fn add_quest_poi_frame_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_quest_blob_texture_setter(methods, "SetFillTexture", |blob, texture| {
        blob.fill_texture = texture;
    });
    add_quest_blob_texture_setter(methods, "SetBorderTexture", |blob, texture| {
        blob.border_texture = texture;
    });
    add_quest_blob_numeric_setter(methods, "SetFillAlpha", |blob, alpha| {
        blob.fill_alpha = Some(alpha);
    });
    add_quest_blob_numeric_setter(methods, "SetBorderAlpha", |blob, alpha| {
        blob.border_alpha = Some(alpha);
    });
    add_quest_blob_numeric_setter(methods, "SetBorderScalar", |blob, scalar| {
        blob.border_scalar = Some(scalar);
    });
    methods.add_method("UpdateMouseOverTooltip", |lua, this, (x, y): (f64, f64)| {
        update_mouse_over_tooltip(lua, this.0, x, y)
    });
}

fn add_fog_of_war_frame_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetFogOfWarBackgroundAtlas", |lua, this, ()| {
        let atlas = read_fog_of_war_frame_state(get_sim_state(lua), this.0, |fog| {
            fog.background_atlas.clone()
        });
        fog_of_war_string_value(lua, atlas)
    });
    methods.add_method("GetFogOfWarMaskAtlas", |lua, this, ()| {
        let atlas =
            read_fog_of_war_frame_state(get_sim_state(lua), this.0, |fog| fog.mask_atlas.clone());
        fog_of_war_string_value(lua, atlas)
    });
    methods.add_method("GetMaskScalar", |lua, this, ()| {
        Ok(
            read_fog_of_war_frame_state(get_sim_state(lua), this.0, |fog| fog.mask_scalar)
                .unwrap_or(1.0),
        )
    });
    methods.add_method("SetFogOfWarBackgroundAtlas", |lua, this, atlas: Value| {
        let atlas = super::methods_misc::texture_asset_to_string(&atlas)?;
        let state_rc = get_sim_state(lua);
        write_fog_of_war_frame_state(std::rc::Rc::clone(&state_rc), this.0, |fog| {
            fog.background_atlas = atlas.clone();
        });
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.fog_of_war_background_atlas = atlas;
        }
        Ok(())
    });
    methods.add_method("SetFogOfWarMaskAtlas", |lua, this, atlas: Value| {
        let atlas = super::methods_misc::texture_asset_to_string(&atlas)?;
        let state_rc = get_sim_state(lua);
        write_fog_of_war_frame_state(std::rc::Rc::clone(&state_rc), this.0, |fog| {
            fog.mask_atlas = atlas.clone();
        });
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.fog_of_war_mask_atlas = atlas;
        }
        Ok(())
    });
    methods.add_method("SetMaskScalar", |lua, this, scalar: Option<f64>| {
        let state_rc = get_sim_state(lua);
        write_fog_of_war_frame_state(std::rc::Rc::clone(&state_rc), this.0, |fog| {
            fog.mask_scalar = scalar;
        });
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.fog_of_war_mask_scalar = scalar.map(|value| value as f32);
        }
        Ok(())
    });
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

fn add_quest_blob_texture_setter<M, F>(methods: &mut M, name: &'static str, setter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::lua_api::state::QuestBlobState, Option<String>) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, texture: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        setter(
            quest_blob_state_mut(&mut state, this.0),
            super::methods_misc::texture_asset_to_string(&texture)?,
        );
        Ok(())
    });
}

fn add_quest_blob_numeric_setter<M, F>(methods: &mut M, name: &'static str, setter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::lua_api::state::QuestBlobState, f64) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, value: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        setter(quest_blob_state_mut(&mut state, this.0), value);
        Ok(())
    });
}

fn quest_blob_state_mut(
    state: &mut crate::lua_api::state::SimState,
    frame_id: u64,
) -> &mut crate::lua_api::state::QuestBlobState {
    state.quest_blobs.entry(frame_id).or_default()
}

fn update_mouse_over_tooltip(
    lua: &mlua::Lua,
    frame_id: u64,
    x: f64,
    y: f64,
) -> mlua::Result<(Value, Value)> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let blob_state = match state.quest_blobs.get(&frame_id) {
        Some(bs) if !bs.active_quests.is_empty() => bs,
        _ => return Ok((Value::Nil, Value::Nil)),
    };
    match crate::quest_poi_blobs::hit_test_blobs(
        &blob_state.active_quests,
        blob_state.map_id,
        x as f32,
        y as f32,
    ) {
        Some((quest_id, count)) => Ok((
            Value::Integer(quest_id as i64),
            Value::Integer(count as i64),
        )),
        None => Ok((Value::Nil, Value::Nil)),
    }
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
        let unit_state = state
            .unit_position_frames
            .entry(this.0)
            .or_insert_with(new_unit_position_frame_state);
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
        let unit_state = state
            .unit_position_frames
            .entry(this.0)
            .or_insert_with(new_unit_position_frame_state);
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
        let unit_state = state
            .unit_position_frames
            .entry(this.0)
            .or_insert_with(new_unit_position_frame_state);
        unit_state.is_finalized = true;
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
        let unit_state = state
            .unit_position_frames
            .entry(this.0)
            .or_insert_with(new_unit_position_frame_state);
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
        clear_fog_of_war_frame(state, frame_id);
        return;
    };

    let Some(fog_info) = crate::lua_api::globals::c_map_api::fog_of_war_info_for_id(fog_id) else {
        clear_fog_of_war_frame(state, frame_id);
        return;
    };

    let fog_state = state.fog_of_war_frames.entry(frame_id).or_default();
    fog_state.ui_map_id = Some(map_id);
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

fn clear_fog_of_war_frame(state: &mut crate::lua_api::SimState, frame_id: u64) {
    let fog_state = state.fog_of_war_frames.entry(frame_id).or_default();
    fog_state.ui_map_id = None;
    fog_state.background_atlas = None;
    fog_state.mask_atlas = None;
    fog_state.mask_scalar = None;
    if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
        frame.fog_of_war_ui_map_id = None;
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
    let unit_state = state
        .unit_position_frames
        .entry(frame_id)
        .or_insert_with(new_unit_position_frame_state);
    unit_state.player_ping_scale = scale;
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
    let unit_state = state
        .unit_position_frames
        .entry(frame_id)
        .or_insert_with(new_unit_position_frame_state);
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
    let unit_state = state
        .unit_position_frames
        .entry(frame_id)
        .or_insert_with(new_unit_position_frame_state);
    unit_state.player_ping_active = true;
    unit_state.player_ping_duration = Some(duration);
    unit_state.player_ping_fade_duration = Some(fade_duration);
}

fn stop_unit_position_ping(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
) {
    let mut state = state_rc.borrow_mut();
    let unit_state = state
        .unit_position_frames
        .entry(frame_id)
        .or_insert_with(new_unit_position_frame_state);
    unit_state.player_ping_active = false;
}

pub(super) fn multi_value_string_arg(args: &MultiValue, index: usize) -> Option<String> {
    match args.get(index) {
        Some(Value::String(value)) => Some(value.to_string_lossy().to_string()),
        _ => None,
    }
}

pub(super) fn multi_value_texture_arg(
    args: &MultiValue,
    index: usize,
) -> mlua::Result<Option<String>> {
    match args.get(index) {
        Some(value) => super::methods_misc::texture_asset_to_string(value),
        None => Ok(None),
    }
}

pub(super) fn multi_value_number_arg(args: &MultiValue, index: usize) -> Option<f64> {
    match args.get(index) {
        Some(Value::Integer(value)) => Some(*value as f64),
        Some(Value::Number(value)) => Some(*value),
        _ => None,
    }
}

pub(super) fn multi_value_i32_arg(args: &MultiValue, index: usize) -> Option<i32> {
    match args.get(index) {
        Some(Value::Integer(value)) => Some(*value as i32),
        Some(Value::Number(value)) => Some(*value as i32),
        _ => None,
    }
}

pub(super) fn multi_value_bool_arg(args: &MultiValue, index: usize) -> Option<bool> {
    match args.get(index) {
        Some(Value::Boolean(value)) => Some(*value),
        _ => None,
    }
}

pub(super) fn multi_value_color_arg(
    args: &MultiValue,
    start_index: usize,
) -> Option<(f64, f64, f64, f64)> {
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

/// Quest blob methods for QuestPOIFrame (DrawBlob, DrawNone, SetMapID).
fn add_quest_blob_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("DrawBlob", |lua, this, args: mlua::MultiValue| {
        let mut iter = args.into_iter();
        let quest_id = match iter.next() {
            Some(Value::Integer(n)) => n as u32,
            Some(Value::Number(n)) => n as u32,
            _ => return Ok(()),
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let blob = state.quest_blobs.entry(this.0).or_default();
        if !blob.active_quests.contains(&quest_id) {
            blob.active_quests.push(quest_id);
        }
        Ok(())
    });

    methods.add_method("DrawNone", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(blob) = state.quest_blobs.get_mut(&this.0) {
            blob.active_quests.clear();
        }
        Ok(())
    });

    // GetTooltipIndex(i) → POI index for tooltip line ordering.
    // Identity mapping: tooltip index equals the input index.
    methods.add_method("GetTooltipIndex", |_, _, index: i32| Ok(index));
}
