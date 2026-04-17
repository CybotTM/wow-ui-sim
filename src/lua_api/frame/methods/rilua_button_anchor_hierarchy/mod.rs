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

use crate::lua_bridge::table_set_rust_fn;
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
    table_set_rust_fn(
        state,
        table,
        "SetNormalFontObject",
        buttons::set_normal_font_object,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetNormalFontObject",
        buttons::get_normal_font_object,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetHighlightFontObject",
        buttons::set_highlight_font_object,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetHighlightFontObject",
        buttons::get_highlight_font_object,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetDisabledFontObject",
        buttons::set_disabled_font_object,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetDisabledFontObject",
        buttons::get_disabled_font_object,
    )?;
    Ok(())
}

fn register_button_text(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(
        state,
        table,
        "SetPushedTextOffset",
        buttons::set_pushed_text_offset,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetPushedTextOffset",
        buttons::get_pushed_text_offset,
    )?;
    table_set_rust_fn(state, table, "GetFontString", font_strings::get_font_string)?;
    table_set_rust_fn(state, table, "SetFontString", font_strings::set_font_string)?;
    Ok(())
}

fn register_button_enable(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, table, "IsEnabled", buttons::is_enabled)?;
    table_set_rust_fn(state, table, "SetEnabled", buttons::set_enabled)?;
    table_set_rust_fn(state, table, "Enable", buttons::enable)?;
    table_set_rust_fn(state, table, "Disable", buttons::disable)?;
    Ok(())
}

fn register_button_clicks(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(
        state,
        table,
        "RegisterForClicks",
        buttons::register_for_clicks,
    )?;
    table_set_rust_fn(state, table, "SetButtonState", buttons::set_button_state)?;
    table_set_rust_fn(state, table, "GetButtonState", buttons::get_button_state)?;
    table_set_rust_fn(state, table, "Click", buttons::click)?;
    Ok(())
}

fn register_button_misc(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(
        state,
        table,
        "SetItemButtonScale",
        buttons::set_item_button_scale,
    )?;
    table_set_rust_fn(state, table, "CalculateAction", buttons::calculate_action)?;
    Ok(())
}

fn register_textures(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(
        state,
        table,
        "GetNormalTexture",
        textures::get_normal_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetHighlightTexture",
        textures::get_highlight_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetPushedTexture",
        textures::get_pushed_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetDisabledTexture",
        textures::get_disabled_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetCheckedTexture",
        textures::get_checked_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetNormalTexture",
        textures::set_normal_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetHighlightTexture",
        textures::set_highlight_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetPushedTexture",
        textures::set_pushed_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetDisabledTexture",
        textures::set_disabled_texture,
    )?;
    table_set_rust_fn(state, table, "SetNormalAtlas", textures::set_normal_atlas)?;
    table_set_rust_fn(state, table, "SetPushedAtlas", textures::set_pushed_atlas)?;
    table_set_rust_fn(
        state,
        table,
        "SetDisabledAtlas",
        textures::set_disabled_atlas,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetHighlightAtlas",
        textures::set_highlight_atlas,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetCheckedTexture",
        textures::set_checked_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetDisabledCheckedTexture",
        textures::set_disabled_checked_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetDisabledCheckedTexture",
        textures::get_disabled_checked_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "ClearNormalTexture",
        textures::clear_normal_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "ClearHighlightTexture",
        textures::clear_highlight_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "ClearPushedTexture",
        textures::clear_pushed_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "ClearDisabledTexture",
        textures::clear_disabled_texture,
    )?;
    table_set_rust_fn(state, table, "SetLeftTexture", textures::set_left_texture)?;
    table_set_rust_fn(
        state,
        table,
        "SetMiddleTexture",
        textures::set_middle_texture,
    )?;
    table_set_rust_fn(state, table, "SetRightTexture", textures::set_right_texture)?;
    Ok(())
}

fn register_anchors(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, table, "SetPoint", anchors::set_point)?;
    table_set_rust_fn(state, table, "SetStartPoint", anchors::set_start_point)?;
    table_set_rust_fn(state, table, "SetEndPoint", anchors::set_end_point)?;
    table_set_rust_fn(state, table, "ClearAllPoints", anchors::clear_all_points)?;
    table_set_rust_fn(state, table, "ClearPoint", anchors::clear_point)?;
    table_set_rust_fn(
        state,
        table,
        "ClearPointsOffset",
        anchors::clear_points_offset,
    )?;
    table_set_rust_fn(
        state,
        table,
        "AdjustPointsOffset",
        anchors::adjust_points_offset,
    )?;
    table_set_rust_fn(state, table, "SetAllPoints", anchors::set_all_points)?;
    table_set_rust_fn(state, table, "GetPoint", anchors::get_point)?;
    table_set_rust_fn(state, table, "GetStartPoint", anchors::get_start_point)?;
    table_set_rust_fn(state, table, "GetEndPoint", anchors::get_end_point)?;
    table_set_rust_fn(state, table, "GetNumPoints", anchors::get_num_points)?;
    table_set_rust_fn(state, table, "GetPointByName", anchors::get_point_by_name)?;
    Ok(())
}

fn register_hierarchy(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, table, "GetParent", hierarchy::get_parent)?;
    table_set_rust_fn(state, table, "SetParent", hierarchy::set_parent)?;
    table_set_rust_fn(state, table, "GetNumChildren", hierarchy::get_num_children)?;
    table_set_rust_fn(state, table, "GetChildren", hierarchy::get_children)?;
    table_set_rust_fn(state, table, "GetNumRegions", hierarchy::get_num_regions)?;
    table_set_rust_fn(state, table, "GetRegions", hierarchy::get_regions)?;
    table_set_rust_fn(
        state,
        table,
        "GetAdditionalRegions",
        hierarchy::get_additional_regions,
    )?;
    table_set_rust_fn(state, table, "GetParentKey", hierarchy::get_parent_key)?;
    table_set_rust_fn(state, table, "SetParentKey", hierarchy::set_parent_key)?;
    table_set_rust_fn(state, table, "CreateTexture", hierarchy::create_texture)?;
    table_set_rust_fn(
        state,
        table,
        "CreateMaskTexture",
        hierarchy::create_mask_texture,
    )?;
    table_set_rust_fn(state, table, "AddMaskTexture", hierarchy::add_mask_texture)?;
    table_set_rust_fn(
        state,
        table,
        "RemoveMaskTexture",
        hierarchy::remove_mask_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetNumMaskTextures",
        hierarchy::get_num_mask_textures,
    )?;
    table_set_rust_fn(state, table, "GetMaskTexture", hierarchy::get_mask_texture)?;
    table_set_rust_fn(state, table, "CreateLine", hierarchy::create_line)?;
    table_set_rust_fn(
        state,
        table,
        "CreateFontString",
        font_strings::create_font_string,
    )?;
    table_set_rust_fn(state, table, "AttachTexture", hierarchy::attach_texture)?;
    table_set_rust_fn(
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
    table_set_rust_fn(
        state,
        table,
        "CreateControlPoint",
        animations::create_control_point,
    )?;
    Ok(())
}

fn register_animation_creation(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(
        state,
        table,
        "GetAnimationGroups",
        animations::get_animation_groups,
    )?;
    table_set_rust_fn(state, table, "GetAnimations", animations::get_animations)?;
    table_set_rust_fn(
        state,
        table,
        "CreateAnimationGroup",
        animations::create_animation_group,
    )?;
    table_set_rust_fn(
        state,
        table,
        "CreateAnimation",
        animations::create_animation,
    )?;
    Ok(())
}

fn register_animation_group_control(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, table, "Play", animations::animation_group_play)?;
    table_set_rust_fn(state, table, "Restart", animations::animation_group_restart)?;
    table_set_rust_fn(state, table, "Stop", animations::animation_group_stop)?;
    table_set_rust_fn(state, table, "Finish", animations::animation_group_finish)?;
    table_set_rust_fn(
        state,
        table,
        "IsPlaying",
        animations::animation_group_is_playing,
    )?;
    table_set_rust_fn(state, table, "IsDone", animations::animation_group_is_done)?;
    table_set_rust_fn(
        state,
        table,
        "SetLooping",
        animations::animation_group_set_looping,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetToFinalAlpha",
        animations::animation_group_set_to_final_alpha,
    )?;
    Ok(())
}

fn register_animation_timing(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(
        state,
        table,
        "SetDuration",
        animations::animation_set_duration,
    )?;
    table_set_rust_fn(state, table, "SetOrder", animations::animation_set_order)?;
    table_set_rust_fn(
        state,
        table,
        "SetStartDelay",
        animations::animation_set_start_delay,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetEndDelay",
        animations::animation_set_end_delay,
    )?;
    Ok(())
}

fn register_animation_config(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    for name in [
        "SetSmoothing",
        "SetFromAlpha",
        "SetToAlpha",
        "SetOffset",
        "SetScaleFrom",
        "SetScaleTo",
        "SetDegrees",
    ] {
        table_set_rust_fn(state, table, name, animations::animation_config_noop)?;
    }
    table_set_rust_fn(state, table, "SetScale", animations::set_scale_dispatch)?;
    Ok(())
}

fn register_animation_target(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, table, "GetTarget", animations::get_animation_target)?;
    table_set_rust_fn(
        state,
        table,
        "GetRegionParent",
        animations::get_region_parent,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetChildKey",
        animations::set_animation_child_key,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetTargetName",
        animations::animation_config_noop,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetTargetKey",
        animations::animation_config_noop,
    )?;
    Ok(())
}

fn register_animation_flipbook(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(
        state,
        table,
        "SetFlipBookRows",
        animations::animation_set_flipbook_rows,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetFlipBookRows",
        animations::animation_get_flipbook_rows,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetFlipBookColumns",
        animations::animation_set_flipbook_columns,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetFlipBookColumns",
        animations::animation_get_flipbook_columns,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetFlipBookFrames",
        animations::animation_set_flipbook_frames,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetFlipBookFrames",
        animations::animation_get_flipbook_frames,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetFlipBookFrameWidth",
        animations::animation_set_flipbook_frame_width,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetFlipBookFrameWidth",
        animations::animation_get_flipbook_frame_width,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetFlipBookFrameHeight",
        animations::animation_set_flipbook_frame_height,
    )?;
    table_set_rust_fn(
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
