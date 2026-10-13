//! NONE Path motion uses the existing target resolver and presentation offsets.
use super::resolve_anim_target_id;
use crate::lua_api::SimState;
use crate::lua_api::animation::{AnimState, AnimationType, LoopType};

pub(super) fn apply_group_path_state(sim: &mut SimState, group_id: u64) {
    let Some(group) = sim.animation_groups.get(&group_id) else {
        return;
    };
    if group.paused {
        return;
    }
    if !group.playing && group.done {
        restore_group_path_offsets(sim, group_id);
        return;
    }
    if !group.playing || group.looping != LoopType::None {
        return;
    }
    let updates: Vec<_> = group
        .animations
        .iter()
        .filter_map(|animation| {
            let displacement = interpolate_none_path(animation)?;
            let target_id =
                resolve_anim_target_id(sim, group.owner_frame_id, animation.child_key.as_deref())?;
            let frame = sim.widgets.get(target_id)?;
            let baseline = [frame.anim_offset_x, frame.anim_offset_y];
            Some((target_id, baseline, displacement))
        })
        .collect();
    apply_path_offsets(sim, group_id, updates);
}

fn apply_path_offsets(sim: &mut SimState, group_id: u64, updates: Vec<(u64, [f32; 2], [f32; 2])>) {
    for (target_id, baseline, displacement) in updates {
        let Some(group) = sim.animation_groups.get_mut(&group_id) else {
            return;
        };
        let saved = *group
            .saved_path_offsets
            .entry(target_id)
            .or_insert(baseline);
        let offset = [saved[0] + displacement[0], saved[1] + displacement[1]];
        write_path_offset(sim, target_id, offset);
    }
}

fn restore_group_path_offsets(sim: &mut SimState, group_id: u64) {
    let saved = sim
        .animation_groups
        .get_mut(&group_id)
        .map(|group| std::mem::take(&mut group.saved_path_offsets))
        .unwrap_or_default();
    for (target_id, offset) in saved {
        write_path_offset(sim, target_id, offset);
    }
}

fn write_path_offset(sim: &mut SimState, target_id: u64, offset: [f32; 2]) {
    let changed = sim
        .widgets
        .get(target_id)
        .is_some_and(|frame| [frame.anim_offset_x, frame.anim_offset_y] != offset);
    if changed && let Some(frame) = sim.widgets.get_mut_visual(target_id) {
        [frame.anim_offset_x, frame.anim_offset_y] = offset;
    }
}

fn interpolate_none_path(animation: &AnimState) -> Option<[f32; 2]> {
    let is_none_path = animation.anim_type == AnimationType::Path
        && animation.curve == "NONE"
        && animation.smoothing == "NONE";
    if !is_none_path || animation.duration <= 0.0 {
        return None;
    }
    let points = animation.ordered_control_points();
    if points.len() < 2 {
        return None;
    }
    let segment_count = points.len() - 1;
    let progress = (animation.elapsed / animation.duration).clamp(0.0, 1.0);
    let segment_position = progress * segment_count as f64;
    let index = (segment_position.floor() as usize).min(segment_count - 1);
    let fraction = segment_position - index as f64;
    let start = points[index].offset;
    let end = points[index + 1].offset;
    Some(std::array::from_fn(|axis| {
        (start[axis] + (end[axis] - start[axis]) * fraction) as f32
    }))
}
