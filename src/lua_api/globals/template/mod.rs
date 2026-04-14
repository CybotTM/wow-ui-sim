//! Minimal template bridge during the rilua cutover.

pub(crate) mod direct;

use crate::lua_api::SimState;
use std::cell::RefCell;
use std::rc::Rc;

pub fn apply_templates_from_registry<T>(
    _lua: &T,
    _state: &Rc<RefCell<SimState>>,
    _frame_name: &str,
    _template_names: &str,
) {
}

pub fn fire_deferred_child_onloads<T>(_lua: &T) -> usize {
    0
}

pub(super) fn get_size_values(size: &crate::xml::SizeXml) -> (Option<f32>, Option<f32>) {
    if size.x.is_some() || size.y.is_some() {
        (size.x, size.y)
    } else if let Some(abs) = &size.abs_dimension {
        (abs.x, abs.y)
    } else {
        (None, None)
    }
}
