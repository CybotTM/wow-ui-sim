//! Minimal global registration bridge during the rilua migration.

use super::SimState;
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_globals<T>(_lua: &T, _state: Rc<RefCell<SimState>>) -> crate::Result<()> {
    Ok(())
}
