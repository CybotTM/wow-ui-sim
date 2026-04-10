use crate::lua_api::frame::handle::{FrameRef, extract_frame_id, frame_ref, get_sim_state};
use crate::lua_api::tooltip::{TooltipLine, build_cursor_anchor};
use crate::widget::{Anchor, AnchorPoint};
use mlua::Value;

mod copy;
mod fontstrings;
mod framestack;
mod info;
mod lines;
mod owner;
mod shared;

pub(crate) use copy::copy_tooltip_impl;
pub(crate) use fontstrings::add_get_line_methods;
pub(crate) use framestack::set_frame_stack_impl;
pub(crate) use info::{add_tooltip_info_methods, add_tooltip_state_methods};
pub(crate) use lines::add_double_line_impl;
pub(crate) use owner::{set_anchor_type_impl, set_object_tooltip_position_impl, set_owner_impl};
pub(crate) use shared::{fire_tooltip_script, val_to_f32};
