//! Lua API bindings implementing WoW's addon API.

mod addon_scan;
pub(crate) mod proxy_helpers;
#[allow(dead_code)] // Phase 3 infrastructure — callers added during VM switch
pub(crate) mod rilua_methods;
#[allow(dead_code)] // Phase 3 infrastructure — callers added during VM switch
pub(crate) mod rilua_script_helpers;
#[allow(dead_code)] // Phase 3 infrastructure — callers added during VM switch
pub(crate) mod rilua_timer_layout;
#[allow(dead_code)] // Phase 3 infrastructure — callers added during VM switch
pub(crate) mod rilua_taint;
pub mod animation;
mod builtin_frames;
mod cfunc_wrap;
pub(crate) mod chat_init;
mod diagnostics;
mod env;
mod env_rilua;
mod env_init;
pub(crate) mod frame;
mod frame_methods;
pub(crate) mod game_data;
pub mod globals;
mod globals_legacy;
mod key_dispatch;
pub(crate) mod keybindings;
mod layout;
pub(crate) mod loader_env;
pub mod message_frame;
pub(crate) mod on_update;
pub(crate) mod script_helpers;
pub(crate) mod secure_env;
pub mod simple_html;
pub(crate) mod state;
mod state_defaults;
pub(crate) mod state_render;
pub(crate) mod state_types;
mod string_format;
pub(crate) mod talent_state;
mod timer_processing;
pub mod tooltip;
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
