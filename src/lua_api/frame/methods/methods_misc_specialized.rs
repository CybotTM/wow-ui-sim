//! Specialized frame type methods (QuestPOI, FogOfWar, UnitPosition, Menu, etc.).

use super::super::handle::FrameRef;
use super::methods_misc_map_frames::add_map_specialized_frame_methods;
use crate::lua_api::frame::handle::get_sim_state;
use mlua::{MultiValue, Value};

/// Methods for specialized frame types (QuestPOI, FogOfWar, UnitPosition, etc.).
pub(super) fn add_specialized_frame_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_menu_frame_stubs(methods);
    add_quest_poi_frame_methods(methods);
    add_map_specialized_frame_methods(methods);
    add_quest_blob_methods(methods);
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
        add_quest_blob_active_quest(state.quest_blobs.entry(this.0).or_default(), quest_id);
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

fn add_quest_blob_active_quest(blob: &mut crate::lua_api::state::QuestBlobState, quest_id: u32) {
    if !blob.active_quests.contains(&quest_id) {
        blob.active_quests.push(quest_id);
    }
}
