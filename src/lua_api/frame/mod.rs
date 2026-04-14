//! Frame handle and methods for WoW frame UserData.

mod handle;
pub(crate) mod metatable;
pub(crate) mod method_registry;
pub(crate) mod methods;

pub use handle::{
    FrameRef, extract_frame_id, frame_fields, frame_ref, get_sim_state,
    sync_child_to_lua,
};
pub(crate) use methods::UnitTooltipInfo;
pub(crate) use methods::fire_on_show_recursive;
pub(crate) use methods::methods_hierarchy::propagate_strata_level_pub;
pub(crate) use methods::resolve_unit_tooltip_info;
