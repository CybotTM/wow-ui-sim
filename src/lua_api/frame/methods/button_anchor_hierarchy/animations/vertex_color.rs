//! RGBA endpoint snapshots and texture application. Lifecycle and smoothing
//! policies are modeled inferences; native parity beyond the literal example is unknown.
use super::fields::with_animation_state_mut;
use super::resolve_anim_target_id;
use crate::lua_api::animation::{AnimGroupState, AnimState, AnimationType};
use crate::lua_api::globals::font_strings_collection::colors::make_rilua_color_table;
use crate::lua_api::methods::{borrow_state, frame_id_from_stack, table_get};
use crate::lua_bridge::stack_val;
use crate::widget::{Color, WidgetType};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_set_start_color(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let color = read_rgba_argument(state)?;
    with_animation_state_mut(state, |animation| animation.start_color = color)?;
    Ok(0)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_set_end_color(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let color = read_rgba_argument(state)?;
    with_animation_state_mut(state, |animation| animation.end_color = color)?;
    Ok(0)
}

fn read_rgba_argument(state: &mut LuaState) -> LuaResult<[f64; 4]> {
    let input = stack_val(state, 2);
    if !matches!(input, Val::Table(_)) {
        return Err(rilua::runtime_error(
            "colorRGBA must be a table with numeric RGBA channels",
        ));
    }
    let mut color = [0.0; 4];
    for (index, channel) in ["r", "g", "b", "a"].iter().enumerate() {
        let Val::Num(value) = table_get(state, input, channel) else {
            return Err(rilua::runtime_error(format!(
                "colorRGBA.{channel} must be numeric"
            )));
        };
        color[index] = value;
    }
    Ok(color)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_get_start_color(
    state: &mut LuaState,
) -> LuaResult<u32> {
    push_endpoint_color(state, |animation| animation.start_color)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_get_end_color(
    state: &mut LuaState,
) -> LuaResult<u32> {
    push_endpoint_color(state, |animation| animation.end_color)
}

fn push_endpoint_color(
    state: &mut LuaState,
    select: impl FnOnce(&AnimState) -> [f64; 4],
) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let channels = {
        let sim = borrow_state(state)?;
        let animation = sim
            .anim_frame_to_anim
            .get(&frame_id)
            .and_then(|(group_id, index)| {
                sim.animation_groups.get(group_id)?.animations.get(*index)
            })
            .ok_or_else(|| rilua::runtime_error("expected an animation"))?;
        select(animation)
    };
    let [r, g, b, a] = channels;
    let color = make_rilua_color_table(state, r, g, b, a)?;
    state.push(color);
    Ok(1)
}

pub(super) fn apply_group_vertex_color_state(sim: &mut crate::lua_api::SimState, group_id: u64) {
    let Some(group) = sim.animation_groups.get(&group_id) else {
        return;
    };
    if group.paused {
        return;
    }
    if !group.playing && group.done {
        restore_group_vertex_colors(sim, group_id);
        return;
    }
    if !group.playing {
        return;
    }
    let updates: Vec<_> = group
        .animations
        .iter()
        .filter_map(|animation| {
            if animation.anim_type != AnimationType::VertexColor {
                return None;
            }
            let target_id =
                resolve_anim_target_id(sim, group.owner_frame_id, animation.child_key.as_deref())?;
            let frame = sim.widgets.get(target_id)?;
            if frame.widget_type != WidgetType::Texture {
                return None;
            }
            let within_order = elapsed_within_order(group, animation.order);
            let color = interpolate_rgba(animation, within_order);
            Some((target_id, frame.vertex_color, color))
        })
        .collect();
    for (target_id, baseline, color) in updates {
        if let Some(group) = sim.animation_groups.get_mut(&group_id) {
            group
                .saved_vertex_colors
                .entry(target_id)
                .or_insert(baseline);
        }
        write_vertex_color(sim, target_id, Some(color));
    }
}

fn restore_group_vertex_colors(sim: &mut crate::lua_api::SimState, group_id: u64) {
    let saved = sim
        .animation_groups
        .get_mut(&group_id)
        .map(|group| std::mem::take(&mut group.saved_vertex_colors))
        .unwrap_or_default();
    for (target_id, color) in saved {
        write_vertex_color(sim, target_id, color);
    }
}

fn write_vertex_color(sim: &mut crate::lua_api::SimState, target_id: u64, color: Option<Color>) {
    let changed = sim
        .widgets
        .get(target_id)
        .is_some_and(|frame| frame.vertex_color != color);
    if changed && let Some(frame) = sim.widgets.get_mut_visual(target_id) {
        frame.vertex_color = color;
    }
}

fn elapsed_within_order(group: &AnimGroupState, order: u32) -> f64 {
    let mut durations = std::collections::BTreeMap::<u32, f64>::new();
    for animation in &group.animations {
        durations
            .entry(animation.order)
            .and_modify(|duration| *duration = duration.max(animation.total_time()))
            .or_insert(animation.total_time());
    }
    let preceding: f64 = durations.range(..order).map(|(_, duration)| duration).sum();
    (group.elapsed - preceding).max(0.0)
}

fn interpolate_rgba(animation: &AnimState, elapsed: f64) -> Color {
    let progress = if elapsed <= animation.start_delay {
        0.0
    } else if animation.duration <= 0.0 {
        1.0
    } else {
        ((elapsed - animation.start_delay) / animation.duration).clamp(0.0, 1.0)
    };
    let progress = smooth_progress(progress, &animation.smoothing);
    let channels: [f32; 4] = std::array::from_fn(|index| {
        let start = animation.start_color[index];
        let end = animation.end_color[index];
        (start + (end - start) * progress) as f32
    });
    let [r, g, b, a] = channels;
    Color::new(r, g, b, a)
}

fn smooth_progress(progress: f64, smoothing: &str) -> f64 {
    match smoothing {
        "IN" => progress * progress,
        "OUT" => 1.0 - (1.0 - progress).powi(2),
        "IN_OUT" if progress < 0.5 => 2.0 * progress * progress,
        "IN_OUT" => 1.0 - 2.0 * (1.0 - progress).powi(2),
        _ => progress,
    }
}
