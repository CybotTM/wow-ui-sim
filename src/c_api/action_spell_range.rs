//! Supplied spell/target range snapshots shared by profile-specific API adapters.
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ActionSpellRange {
    pub has_range_requirements: bool,
    pub in_range_by_guid: HashMap<String, bool>,
}
