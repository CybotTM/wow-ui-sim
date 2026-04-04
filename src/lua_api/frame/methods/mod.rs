//! Frame methods registered into the FrameRef UserData methods table.
//!
//! Each submodule exports `add_*_methods<M: UserDataMethods<FrameRef>>(methods: &mut M)`
//! which registers method functions on the UserData methods container.

pub(crate) mod combat_lockdown;
mod methods_anchor;
mod methods_anim_proxy;
mod methods_attribute;
mod methods_backdrop;
mod methods_button;
mod methods_button_state;
pub(crate) mod methods_core;
mod methods_create;
mod methods_event;
pub(crate) mod methods_helpers;
pub(crate) mod methods_hierarchy;
mod methods_line;
mod methods_misc;
mod methods_rect;
mod methods_script;
mod methods_text;
mod methods_texture;
pub(crate) mod methods_visibility;
mod methods_widget;
mod widget_cooldown;
mod widget_editbox;
mod widget_message_frame;
mod widget_misc;
mod widget_model;
mod widget_scroll;
mod widget_slider;
mod widget_tooltip;

pub(crate) use methods_visibility::fire_on_show_recursive;

/// Register all ~200 frame methods into the FrameRef UserData methods container.
pub fn register_all_methods<M: mlua::UserDataMethods<super::handle::FrameRef>>(methods: &mut M) {
    methods_core::add_core_methods(methods);
    methods_hierarchy::add_hierarchy_methods(methods);
    methods_misc::add_misc_methods(methods);
    methods_anchor::add_anchor_methods(methods);
    methods_event::add_event_methods(methods);
    methods_script::add_script_methods(methods);
    methods_attribute::add_attribute_methods(methods);
    methods_backdrop::add_backdrop_methods(methods);
    methods_create::add_create_methods(methods);
    methods_texture::add_texture_methods(methods);
    methods_text::add_text_methods(methods);
    methods_button::add_button_methods(methods);
    methods_widget::add_widget_methods(methods);
    methods_line::add_line_methods(methods);
    methods_anim_proxy::add_anim_proxy_methods(methods);
}
