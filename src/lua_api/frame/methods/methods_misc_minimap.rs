//! Minimap and WorldMap frame methods.

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::{frame_ref, get_sim_state};
use crate::widget::{MinimapBlobLayerStyle, MinimapBlobRingStyle, WidgetType};
use mlua::Value;

/// Minimap and WorldMap stubs.
pub(super) fn add_minimap_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_minimap_core_methods(methods);
    add_minimap_texture_setters(methods);
    add_minimap_blob_setters(methods);
    // GetCanvas() - for WorldMapFrame (returns self as the canvas)
    methods.add_method("GetCanvas", |lua, this, ()| frame_ref(lua, this.0));
}

/// Minimap core: zoom, ping, blips.
fn add_minimap_core_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetZoom", |lua, this, ()| get_frame_zoom(lua, this.0));
    methods.add_method("SetZoom", |lua, this, zoom: i32| {
        set_frame_zoom(lua, this.0, zoom)
    });
    methods.add_method("GetZoomLevels", |_, _this, ()| {
        Ok(minimap_zoom_level_count())
    });
    methods.add_method("GetPingPosition", |lua, this, ()| {
        Ok(read_minimap_ping_position(get_sim_state(lua), this.0))
    });
    methods.add_method("PingLocation", |lua, this, (x, y): (f64, f64)| {
        write_minimap_ping_position(get_sim_state(lua), this.0, x, y);
        Ok(())
    });
    methods.add_method("UpdateBlips", |lua, this, ()| {
        bump_minimap_blip_revision(get_sim_state(lua), this.0);
        Ok(())
    });
}

/// Minimap texture setters (no-op stubs).
fn add_minimap_texture_setters<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_minimap_texture_setter(methods, "SetBlipTexture", |frame, asset| {
        frame.minimap_blip_texture = asset;
    });
    add_minimap_texture_setter(methods, "SetMaskTexture", |frame, asset| {
        frame.minimap_mask_texture = asset;
    });
    add_minimap_texture_setter(methods, "SetIconTexture", |frame, asset| {
        frame.minimap_icon_texture = asset;
    });
    add_minimap_texture_setter(methods, "SetPlayerTexture", |frame, asset| {
        frame.minimap_player_texture = asset;
    });
    add_minimap_texture_setter(methods, "SetPOIArrowTexture", |frame, asset| {
        frame.minimap_poi_arrow_texture = asset;
    });
    add_minimap_texture_setter(methods, "SetCorpsePOIArrowTexture", |frame, asset| {
        frame.minimap_corpse_poi_arrow_texture = asset;
    });
    add_minimap_texture_setter(methods, "SetStaticPOIArrowTexture", |frame, asset| {
        frame.minimap_static_poi_arrow_texture = asset;
    });
}

pub(super) fn reset_frame_to_defaults(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<()> {
    reset_minimap_frame_to_defaults(get_sim_state(lua), frame_id);
    reset_frame_default_fields(lua, frame_id)
}

fn reset_minimap_frame_to_defaults(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
) {
    let mut state = state_rc.borrow_mut();
    let Some(frame) = state.widgets.get_mut(frame_id) else {
        return;
    };
    if frame.widget_type != WidgetType::Minimap {
        return;
    }

    frame.minimap_blip_texture = None;
    frame.minimap_mask_texture = None;
    frame.minimap_icon_texture = None;
    frame.minimap_player_texture = None;
    frame.minimap_poi_arrow_texture = None;
    frame.minimap_corpse_poi_arrow_texture = None;
    frame.minimap_static_poi_arrow_texture = None;
    frame.minimap_ping_position = None;
    frame.minimap_blip_update_revision = 0;
    frame.quest_blob_inside = MinimapBlobLayerStyle::default();
    frame.quest_blob_outside = MinimapBlobLayerStyle::default();
    frame.quest_blob_ring = MinimapBlobRingStyle::default();
    frame.task_blob_inside = MinimapBlobLayerStyle::default();
    frame.task_blob_outside = MinimapBlobLayerStyle::default();
    frame.task_blob_ring = MinimapBlobRingStyle::default();
    frame.arch_blob_inside = MinimapBlobLayerStyle::default();
    frame.arch_blob_outside = MinimapBlobLayerStyle::default();
    frame.arch_blob_ring = MinimapBlobRingStyle::default();
}

fn reset_frame_default_fields(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<()> {
    let fields = super::methods_misc::frame_fields(lua, frame_id)?;
    fields.set("zoom", Value::Nil)
}

fn add_minimap_texture_setter<M, F>(methods: &mut M, name: &'static str, setter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::widget::Frame, Option<String>) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, asset: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            setter(frame, super::methods_misc::texture_asset_to_string(&asset)?);
        }
        Ok(())
    });
}

/// Minimap quest/task/arch blob setters.
fn add_minimap_blob_setters<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_minimap_blob_family(methods, BlobFamily::Quest);
    add_minimap_blob_family(methods, BlobFamily::Task);
    add_minimap_blob_family(methods, BlobFamily::Arch);
}

#[derive(Clone, Copy)]
enum BlobFamily {
    Quest,
    Task,
    Arch,
}

#[derive(Clone, Copy)]
enum BlobLayer {
    Inside,
    Outside,
}

struct BlobMethodNames {
    inside_texture: &'static str,
    inside_alpha: &'static str,
    outside_texture: &'static str,
    outside_alpha: &'static str,
    ring_texture: &'static str,
    ring_alpha: &'static str,
    ring_scalar: &'static str,
}

fn add_minimap_blob_family<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    family: BlobFamily,
) {
    let names = minimap_blob_method_names(family);
    add_minimap_blob_texture_setter(methods, names.inside_texture, family, BlobLayer::Inside);
    add_minimap_blob_alpha_setter(methods, names.inside_alpha, family, BlobLayer::Inside);
    add_minimap_blob_texture_setter(methods, names.outside_texture, family, BlobLayer::Outside);
    add_minimap_blob_alpha_setter(methods, names.outside_alpha, family, BlobLayer::Outside);
    add_minimap_blob_ring_texture_setter(methods, names.ring_texture, family);
    add_minimap_blob_ring_scalar_setter(methods, names.ring_scalar, family);
    add_minimap_blob_ring_alpha_setter(methods, names.ring_alpha, family);
}

fn minimap_blob_method_names(family: BlobFamily) -> BlobMethodNames {
    match family {
        BlobFamily::Quest => BlobMethodNames {
            inside_texture: "SetQuestBlobInsideTexture",
            inside_alpha: "SetQuestBlobInsideAlpha",
            outside_texture: "SetQuestBlobOutsideTexture",
            outside_alpha: "SetQuestBlobOutsideAlpha",
            ring_texture: "SetQuestBlobRingTexture",
            ring_alpha: "SetQuestBlobRingAlpha",
            ring_scalar: "SetQuestBlobRingScalar",
        },
        BlobFamily::Task => BlobMethodNames {
            inside_texture: "SetTaskBlobInsideTexture",
            inside_alpha: "SetTaskBlobInsideAlpha",
            outside_texture: "SetTaskBlobOutsideTexture",
            outside_alpha: "SetTaskBlobOutsideAlpha",
            ring_texture: "SetTaskBlobRingTexture",
            ring_alpha: "SetTaskBlobRingAlpha",
            ring_scalar: "SetTaskBlobRingScalar",
        },
        BlobFamily::Arch => BlobMethodNames {
            inside_texture: "SetArchBlobInsideTexture",
            inside_alpha: "SetArchBlobInsideAlpha",
            outside_texture: "SetArchBlobOutsideTexture",
            outside_alpha: "SetArchBlobOutsideAlpha",
            ring_texture: "SetArchBlobRingTexture",
            ring_alpha: "SetArchBlobRingAlpha",
            ring_scalar: "SetArchBlobRingScalar",
        },
    }
}

fn add_minimap_blob_texture_setter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    family: BlobFamily,
    layer: BlobLayer,
) {
    methods.add_method(name, move |lua, this, asset: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            let texture = super::methods_misc::texture_asset_to_string(&asset)?;
            let layer_style = minimap_blob_layer_mut(frame, family, layer);
            layer_style.texture = texture;
        }
        Ok(())
    });
}

fn add_minimap_blob_alpha_setter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    family: BlobFamily,
    layer: BlobLayer,
) {
    methods.add_method(name, move |lua, this, alpha: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            let layer_style = minimap_blob_layer_mut(frame, family, layer);
            layer_style.alpha = alpha;
        }
        Ok(())
    });
}

fn add_minimap_blob_ring_texture_setter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    family: BlobFamily,
) {
    methods.add_method(name, move |lua, this, asset: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            let texture = super::methods_misc::texture_asset_to_string(&asset)?;
            let ring_style = minimap_blob_ring_mut(frame, family);
            ring_style.texture = texture;
        }
        Ok(())
    });
}

fn add_minimap_blob_ring_alpha_setter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    family: BlobFamily,
) {
    methods.add_method(name, move |lua, this, alpha: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            let ring_style = minimap_blob_ring_mut(frame, family);
            ring_style.alpha = alpha;
        }
        Ok(())
    });
}

fn add_minimap_blob_ring_scalar_setter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    family: BlobFamily,
) {
    methods.add_method(name, move |lua, this, scalar: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            let ring_style = minimap_blob_ring_mut(frame, family);
            ring_style.scalar = scalar;
        }
        Ok(())
    });
}

fn minimap_blob_layer_mut(
    frame: &mut crate::widget::Frame,
    family: BlobFamily,
    layer: BlobLayer,
) -> &mut crate::widget::MinimapBlobLayerStyle {
    match (family, layer) {
        (BlobFamily::Quest, BlobLayer::Inside) => &mut frame.quest_blob_inside,
        (BlobFamily::Quest, BlobLayer::Outside) => &mut frame.quest_blob_outside,
        (BlobFamily::Task, BlobLayer::Inside) => &mut frame.task_blob_inside,
        (BlobFamily::Task, BlobLayer::Outside) => &mut frame.task_blob_outside,
        (BlobFamily::Arch, BlobLayer::Inside) => &mut frame.arch_blob_inside,
        (BlobFamily::Arch, BlobLayer::Outside) => &mut frame.arch_blob_outside,
    }
}

fn minimap_blob_ring_mut(
    frame: &mut crate::widget::Frame,
    family: BlobFamily,
) -> &mut crate::widget::MinimapBlobRingStyle {
    match family {
        BlobFamily::Quest => &mut frame.quest_blob_ring,
        BlobFamily::Task => &mut frame.task_blob_ring,
        BlobFamily::Arch => &mut frame.arch_blob_ring,
    }
}

fn get_frame_zoom(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<i32> {
    let fields = super::methods_misc::frame_fields(lua, frame_id)?;
    match fields.get::<Value>("zoom")? {
        Value::Integer(zoom) => Ok(zoom as i32),
        Value::Number(zoom) => Ok(zoom as i32),
        _ => Ok(0),
    }
}

fn set_frame_zoom(lua: &mlua::Lua, frame_id: u64, zoom: i32) -> mlua::Result<()> {
    let fields = super::methods_misc::frame_fields(lua, frame_id)?;
    fields.set("zoom", zoom.clamp(0, minimap_max_zoom_index()))
}

fn minimap_zoom_level_count() -> i32 {
    minimap_max_zoom_index() + 1
}

fn minimap_max_zoom_index() -> i32 {
    5
}

fn read_minimap_ping_position(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
) -> (f64, f64) {
    let state = state_rc.borrow();
    state
        .widgets
        .get(frame_id)
        .and_then(|frame| frame.minimap_ping_position)
        .map(|(x, y)| (x as f64, y as f64))
        .unwrap_or((0.0, 0.0))
}

fn write_minimap_ping_position(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    x: f64,
    y: f64,
) {
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
        frame.minimap_ping_position = Some((x as f32, y as f32));
    }
}

fn bump_minimap_blip_revision(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
) {
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
        frame.minimap_blip_update_revision = frame.minimap_blip_update_revision.saturating_add(1);
    }
}
