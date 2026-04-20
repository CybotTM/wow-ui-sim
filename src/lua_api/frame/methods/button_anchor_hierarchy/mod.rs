//! rilua RustFn equivalents of button, anchor, hierarchy, and create methods.
//!
//! Submodules:
//! - `shared`      — opt_f32, opt_string, resolve_anchor_target_id, frame_global_or_ref, etc.
//! - `anchors`     — SetPoint, GetPoint, ClearAllPoints, line endpoints, etc.
//! - `buttons`     — button state, enable/disable, click, font objects, pushed text offset
//! - `textures`    — button texture setters/getters, atlas setters, clear methods
//! - `font_strings`— GetFontString, SetFontString, CreateFontString
//! - `animations`  — animation group/animation creation and control
//! - `hierarchy`   — parent/children/regions hierarchy, CreateTexture, CreateLine, masks

mod anchors;
mod animations;
mod buttons;
mod font_strings;
mod hierarchy;
mod shared;
mod textures;

pub(crate) use animations::advance_animation_groups;
pub(crate) use font_strings::{
    apply_font_object_snapshot, ensure_button_text_child, read_font_object_fields,
};

use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;

fn register_buttons(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    register_button_font_objects(state, table)?;
    register_button_text(state, table)?;
    register_button_enable(state, table)?;
    register_button_clicks(state, table)?;
    register_button_misc(state, table)?;
    Ok(())
}

fn register_button_font_objects(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    let entries: &[(&'static str, rilua::vm::closure::RustFn)] = &[
        ("SetNormalFontObject", buttons::set_normal_font_object),
        ("GetNormalFontObject", buttons::get_normal_font_object),
        ("SetHighlightFontObject", buttons::set_highlight_font_object),
        ("GetHighlightFontObject", buttons::get_highlight_font_object),
        ("SetDisabledFontObject", buttons::set_disabled_font_object),
        ("GetDisabledFontObject", buttons::get_disabled_font_object),
    ];
    for (name, func) in entries {
        table_set_rust_fn_static(state, table, name, *func)?;
    }
    Ok(())
}

fn register_button_text(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "SetPushedTextOffset",
        buttons::set_pushed_text_offset,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetPushedTextOffset",
        buttons::get_pushed_text_offset,
    )?;
    table_set_rust_fn_static(state, table, "GetFontString", font_strings::get_font_string)?;
    table_set_rust_fn_static(state, table, "SetFontString", font_strings::set_font_string)?;
    Ok(())
}

fn register_button_enable(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "IsEnabled", buttons::is_enabled)?;
    table_set_rust_fn_static(state, table, "SetEnabled", buttons::set_enabled)?;
    table_set_rust_fn_static(state, table, "Enable", buttons::enable)?;
    table_set_rust_fn_static(state, table, "Disable", buttons::disable)?;
    Ok(())
}

fn register_button_clicks(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "RegisterForClicks",
        buttons::register_for_clicks,
    )?;
    table_set_rust_fn_static(state, table, "SetButtonState", buttons::set_button_state)?;
    table_set_rust_fn_static(state, table, "GetButtonState", buttons::get_button_state)?;
    table_set_rust_fn_static(state, table, "Click", buttons::click)?;
    Ok(())
}

fn register_button_misc(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "IsDownOver", buttons::is_down_over)?;
    table_set_rust_fn_static(state, table, "IsDown", buttons::is_down)?;
    table_set_rust_fn_static(state, table, "IsOver", buttons::is_over)?;
    table_set_rust_fn_static(
        state,
        table,
        "SetItemButtonScale",
        buttons::set_item_button_scale,
    )?;
    table_set_rust_fn_static(state, table, "CalculateAction", buttons::calculate_action)?;
    Ok(())
}

fn register_textures(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    register_texture_button_slots(state, table)?;
    register_texture_checked(state, table)?;
    register_texture_atlas(state, table)?;
    register_texture_clear(state, table)?;
    register_texture_three_slice(state, table)?;
    Ok(())
}

fn register_texture_button_slots(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    let entries: &[(&'static str, rilua::vm::closure::RustFn)] = &[
        ("GetNormalTexture", textures::get_normal_texture),
        ("GetHighlightTexture", textures::get_highlight_texture),
        ("GetPushedTexture", textures::get_pushed_texture),
        ("GetDisabledTexture", textures::get_disabled_texture),
        ("SetNormalTexture", textures::set_normal_texture),
        ("SetHighlightTexture", textures::set_highlight_texture),
        ("SetPushedTexture", textures::set_pushed_texture),
        ("SetDisabledTexture", textures::set_disabled_texture),
    ];
    for (name, func) in entries {
        table_set_rust_fn_static(state, table, name, *func)?;
    }
    Ok(())
}

fn register_texture_checked(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "GetCheckedTexture",
        textures::get_checked_texture,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "SetCheckedTexture",
        textures::set_checked_texture,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "SetDisabledCheckedTexture",
        textures::set_disabled_checked_texture,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetDisabledCheckedTexture",
        textures::get_disabled_checked_texture,
    )?;
    Ok(())
}

fn register_texture_atlas(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "SetNormalAtlas", textures::set_normal_atlas)?;
    table_set_rust_fn_static(state, table, "SetPushedAtlas", textures::set_pushed_atlas)?;
    table_set_rust_fn_static(
        state,
        table,
        "SetDisabledAtlas",
        textures::set_disabled_atlas,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "SetHighlightAtlas",
        textures::set_highlight_atlas,
    )?;
    Ok(())
}

fn register_texture_clear(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "ClearNormalTexture",
        textures::clear_normal_texture,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "ClearHighlightTexture",
        textures::clear_highlight_texture,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "ClearPushedTexture",
        textures::clear_pushed_texture,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "ClearDisabledTexture",
        textures::clear_disabled_texture,
    )?;
    Ok(())
}

fn register_texture_three_slice(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "SetLeftTexture", textures::set_left_texture)?;
    table_set_rust_fn_static(
        state,
        table,
        "SetMiddleTexture",
        textures::set_middle_texture,
    )?;
    table_set_rust_fn_static(state, table, "SetRightTexture", textures::set_right_texture)?;
    Ok(())
}

fn register_anchors(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "SetPoint", anchors::set_point)?;
    table_set_rust_fn_static(state, table, "SetStartPoint", anchors::set_start_point)?;
    table_set_rust_fn_static(state, table, "SetEndPoint", anchors::set_end_point)?;
    table_set_rust_fn_static(state, table, "ClearAllPoints", anchors::clear_all_points)?;
    table_set_rust_fn_static(state, table, "ClearPoint", anchors::clear_point)?;
    table_set_rust_fn_static(
        state,
        table,
        "ClearPointsOffset",
        anchors::clear_points_offset,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "AdjustPointsOffset",
        anchors::adjust_points_offset,
    )?;
    table_set_rust_fn_static(state, table, "SetAllPoints", anchors::set_all_points)?;
    table_set_rust_fn_static(state, table, "GetPoint", anchors::get_point)?;
    table_set_rust_fn_static(state, table, "GetStartPoint", anchors::get_start_point)?;
    table_set_rust_fn_static(state, table, "GetEndPoint", anchors::get_end_point)?;
    table_set_rust_fn_static(state, table, "GetNumPoints", anchors::get_num_points)?;
    table_set_rust_fn_static(state, table, "GetPointByName", anchors::get_point_by_name)?;
    Ok(())
}

fn register_hierarchy(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    register_hierarchy_parent_children(state, table)?;
    register_hierarchy_regions(state, table)?;
    register_hierarchy_creation(state, table)?;
    register_hierarchy_masks(state, table)?;
    Ok(())
}

fn register_hierarchy_parent_children(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "GetParent", hierarchy::get_parent)?;
    table_set_rust_fn_static(state, table, "SetParent", hierarchy::set_parent)?;
    table_set_rust_fn_static(state, table, "GetNumChildren", hierarchy::get_num_children)?;
    table_set_rust_fn_static(state, table, "GetChildren", hierarchy::get_children)?;
    Ok(())
}

fn register_hierarchy_regions(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "GetNumRegions", hierarchy::get_num_regions)?;
    table_set_rust_fn_static(state, table, "GetRegions", hierarchy::get_regions)?;
    table_set_rust_fn_static(
        state,
        table,
        "GetAdditionalRegions",
        hierarchy::get_additional_regions,
    )?;
    table_set_rust_fn_static(state, table, "GetParentKey", hierarchy::get_parent_key)?;
    table_set_rust_fn_static(state, table, "SetParentKey", hierarchy::set_parent_key)?;
    Ok(())
}

fn register_hierarchy_creation(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "CreateTexture", hierarchy::create_texture)?;
    table_set_rust_fn_static(
        state,
        table,
        "CreateMaskTexture",
        hierarchy::create_mask_texture,
    )?;
    table_set_rust_fn_static(state, table, "CreateLine", hierarchy::create_line)?;
    table_set_rust_fn_static(
        state,
        table,
        "CreateFontString",
        font_strings::create_font_string,
    )?;
    Ok(())
}

fn register_hierarchy_masks(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "AddMaskTexture", hierarchy::add_mask_texture)?;
    table_set_rust_fn_static(
        state,
        table,
        "RemoveMaskTexture",
        hierarchy::remove_mask_texture,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetNumMaskTextures",
        hierarchy::get_num_mask_textures,
    )?;
    table_set_rust_fn_static(state, table, "GetMaskTexture", hierarchy::get_mask_texture)?;
    table_set_rust_fn_static(state, table, "AttachTexture", hierarchy::attach_texture)?;
    table_set_rust_fn_static(
        state,
        table,
        "AttachFontString",
        hierarchy::attach_font_string,
    )?;
    Ok(())
}

fn register_animations(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    register_animation_creation(state, table)?;
    register_animation_group_control(state, table)?;
    register_animation_timing(state, table)?;
    register_animation_config(state, table)?;
    register_animation_target(state, table)?;
    register_animation_flipbook(state, table)?;
    table_set_rust_fn_static(
        state,
        table,
        "CreateControlPoint",
        animations::create_control_point,
    )?;
    Ok(())
}

fn register_animation_creation(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "GetAnimationGroups",
        animations::get_animation_groups,
    )?;
    table_set_rust_fn_static(state, table, "GetAnimations", animations::get_animations)?;
    table_set_rust_fn_static(
        state,
        table,
        "CreateAnimationGroup",
        animations::create_animation_group,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "CreateAnimation",
        animations::create_animation,
    )?;
    Ok(())
}

fn register_animation_group_control(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "Play", animations::animation_group_play)?;
    table_set_rust_fn_static(state, table, "PlaySynced", animations::animation_group_play)?;
    table_set_rust_fn_static(state, table, "Pause", animations::animation_group_pause)?;
    table_set_rust_fn_static(state, table, "Restart", animations::animation_group_restart)?;
    table_set_rust_fn_static(state, table, "Stop", animations::animation_group_stop)?;
    table_set_rust_fn_static(state, table, "Finish", animations::animation_group_finish)?;
    table_set_rust_fn_static(
        state,
        table,
        "SetPlaying",
        animations::animation_group_set_playing,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "IsPlaying",
        animations::animation_group_is_playing,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "IsPaused",
        animations::animation_group_is_paused,
    )?;
    table_set_rust_fn_static(state, table, "IsDone", animations::animation_group_is_done)?;
    table_set_rust_fn_static(
        state,
        table,
        "IsPendingFinish",
        animations::animation_group_is_pending_finish,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "IsReverse",
        animations::animation_group_is_reverse,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetDuration",
        animations::animation_group_get_duration,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetElapsed",
        animations::animation_group_get_elapsed,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetProgress",
        animations::animation_group_get_progress,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetSmoothProgress",
        animations::animation_group_get_smooth_progress,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "SetLooping",
        animations::animation_group_set_looping,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetLooping",
        animations::animation_group_get_looping,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetLoopState",
        animations::animation_group_get_loop_state,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "SetAnimationSpeedMultiplier",
        animations::animation_group_set_animation_speed_multiplier,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetAnimationSpeedMultiplier",
        animations::animation_group_get_animation_speed_multiplier,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "SetToFinalAlpha",
        animations::animation_group_set_to_final_alpha,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "IsSetToFinalAlpha",
        animations::animation_group_is_set_to_final_alpha,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetToFinalAlpha",
        animations::animation_group_get_to_final_alpha,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "RemoveAnimations",
        animations::animation_group_remove_animations,
    )?;
    Ok(())
}

fn register_animation_timing(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "SetDuration",
        animations::animation_set_duration,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetDuration",
        animations::animation_get_duration,
    )?;
    table_set_rust_fn_static(state, table, "SetOrder", animations::animation_set_order)?;
    table_set_rust_fn_static(state, table, "GetOrder", animations::animation_get_order)?;
    table_set_rust_fn_static(
        state,
        table,
        "SetStartDelay",
        animations::animation_set_start_delay,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetStartDelay",
        animations::animation_get_start_delay,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "SetEndDelay",
        animations::animation_set_end_delay,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetEndDelay",
        animations::animation_get_end_delay,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetElapsed",
        animations::animation_get_elapsed,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetProgress",
        animations::animation_get_progress,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetSmoothProgress",
        animations::animation_get_smooth_progress,
    )?;
    table_set_rust_fn_static(state, table, "IsStopped", animations::animation_is_stopped)?;
    table_set_rust_fn_static(
        state,
        table,
        "IsDelaying",
        animations::animation_is_delaying,
    )?;
    Ok(())
}

fn register_animation_config(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    for name in [
        "SetSmoothing",
        "GetSmoothing",
        "SetFromAlpha",
        "GetFromAlpha",
        "SetToAlpha",
        "GetToAlpha",
        "SetChange",
        "SetOffset",
        "SetScaleFrom",
        "SetScaleTo",
        "SetDegrees",
        "SetOrigin",
    ] {
        let func = match name {
            "SetSmoothing" => animations::animation_set_smoothing,
            "GetSmoothing" => animations::animation_get_smoothing,
            "SetFromAlpha" => animations::animation_set_from_alpha,
            "GetFromAlpha" => animations::animation_get_from_alpha,
            "SetToAlpha" => animations::animation_set_to_alpha,
            "GetToAlpha" => animations::animation_get_to_alpha,
            "SetChange" => animations::animation_set_change,
            _ => animations::animation_config_noop,
        };
        table_set_rust_fn_static(state, table, name, func)?;
    }
    table_set_rust_fn_static(state, table, "SetScale", animations::set_scale_dispatch)?;
    Ok(())
}

fn register_animation_target(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, table, "GetTarget", animations::get_animation_target)?;
    table_set_rust_fn_static(
        state,
        table,
        "GetRegionParent",
        animations::get_region_parent,
    )?;
    table_set_rust_fn_static(state, table, "SetTarget", animations::animation_config_noop)?;
    table_set_rust_fn_static(
        state,
        table,
        "SetChildKey",
        animations::set_animation_child_key,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "SetTargetName",
        animations::animation_config_noop,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "SetTargetKey",
        animations::animation_config_noop,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "SetTargetParent",
        animations::animation_config_noop,
    )?;
    Ok(())
}

fn register_animation_flipbook(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    register_flipbook_grid(state, table)?;
    register_flipbook_frames(state, table)?;
    register_flipbook_frame_dimensions(state, table)?;
    Ok(())
}

fn register_flipbook_grid(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "SetFlipBookRows",
        animations::animation_set_flipbook_rows,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetFlipBookRows",
        animations::animation_get_flipbook_rows,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "SetFlipBookColumns",
        animations::animation_set_flipbook_columns,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetFlipBookColumns",
        animations::animation_get_flipbook_columns,
    )?;
    Ok(())
}

fn register_flipbook_frames(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "SetFlipBookFrames",
        animations::animation_set_flipbook_frames,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetFlipBookFrames",
        animations::animation_get_flipbook_frames,
    )?;
    Ok(())
}

fn register_flipbook_frame_dimensions(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table,
        "SetFlipBookFrameWidth",
        animations::animation_set_flipbook_frame_width,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetFlipBookFrameWidth",
        animations::animation_get_flipbook_frame_width,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "SetFlipBookFrameHeight",
        animations::animation_set_flipbook_frame_height,
    )?;
    table_set_rust_fn_static(
        state,
        table,
        "GetFlipBookFrameHeight",
        animations::animation_get_flipbook_frame_height,
    )?;
    Ok(())
}

/// Register all button, anchor, hierarchy, and create methods on the given metatable.
pub fn register_all(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    register_buttons(state, table)?;
    register_textures(state, table)?;
    register_anchors(state, table)?;
    register_hierarchy(state, table)?;
    register_animations(state, table)?;
    Ok(())
}
