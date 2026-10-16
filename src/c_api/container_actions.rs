//! Passive supplied inputs and observable requests; not native use effects.
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct ContainerActions {
    /// Simulator-authored eligibility, independent of the native item catalog.
    pub usable_items: HashSet<u32>,
    /// Accepted dispatches only; no consumption, casting, or cooldown implied.
    pub use_requests: Vec<ContainerUseRequest>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainerUseRequest {
    pub bag: i32,
    pub slot: i32,
    pub item_id: u32,
}
