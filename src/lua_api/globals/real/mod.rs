//! Real modeled Lua global surfaces.
//!
//! Modules here expose non-`C_*` Lua globals or mixins backed by simulator
//! state/behavior. Unmodeled compatibility defaults belong under
//! `lua_api::workarounds::{temporary,permanent}` instead.

pub mod container_legacy;
pub mod guild_logo;
pub mod item_legacy;
pub mod locale_info;
pub mod net_stats;
pub mod specialization_helpers;
pub mod specialization_legacy;
pub mod spell_flyout_legacy;
pub mod ui_widget_container;
