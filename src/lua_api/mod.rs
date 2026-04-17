//! Lua API bindings implementing WoW's addon API.

mod addon_scan;
pub mod animation;
mod builtin_frames;
#[allow(dead_code)] // Phase 3 infrastructure — callers added during VM switch
pub mod chat_init;
mod diagnostics;
mod env;
mod env_init;
mod env_rilua;
pub(crate) mod frame;
pub(crate) mod frame_substates;
pub(crate) mod game_data;
pub mod globals;
mod layout;
pub(crate) mod loader_env;
pub mod message_frame;
#[allow(dead_code)] // Phase 3 infrastructure — callers added during VM switch
pub(crate) mod methods;
pub(crate) mod on_update;
#[allow(dead_code)] // Phase 3 infrastructure — callers added during VM switch
pub(crate) mod script_helpers;
pub(crate) mod sim_substates;
pub mod simple_html;
pub(crate) mod state;
mod state_defaults;
pub(crate) mod state_render;
pub(crate) mod state_types;
pub(crate) mod string_format;
#[allow(dead_code)] // Phase 3 infrastructure — callers added during VM switch
pub(crate) mod taint;
pub(crate) mod talent_state;
#[allow(dead_code)] // Phase 3 infrastructure — callers added during VM switch
pub(crate) mod timer_layout;
mod timer_processing;
pub mod tooltip;
pub(crate) mod tracked_recipes;
pub(crate) mod workarounds;
pub(crate) mod workarounds_editmode;

// Re-export public types
pub use env::WowLuaEnv;
pub use globals::global_frames::hide_runtime_hidden_frames;
pub use layout::{
    LayoutRect, anchor_position, compute_frame_rect, frame_position_from_anchor, get_parent_depth,
};
pub use loader_env::LoaderEnv;
pub use message_frame::MessageFrameData;
pub use simple_html::SimpleHtmlData;
pub use state::{AddonInfo, PendingTimer, SimState, tick_party_health};
pub use tooltip::TooltipData;

// Crate-internal re-exports
pub(crate) use env::next_timer_id;
