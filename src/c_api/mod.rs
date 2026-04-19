//! C_* namespace implementations.
//!
//! Real/state-backed surfaces live at the root of this module. Shim-heavy
//! surfaces are grouped under `temporary_shims` or `permanent_shims`.

pub mod c_addon_profiler;
pub mod c_addons;
pub mod c_map;
pub mod c_spec;
pub mod c_spell;
pub mod c_texture;
pub mod c_xml_util;
pub mod item_spell;
pub mod permanent_shims;
pub mod temporary_shims;

mod helpers;

pub(crate) use helpers::{ensure_global_table, ensure_namespace, global_val, set_global_val};
pub use permanent_shims::c_map_api;
