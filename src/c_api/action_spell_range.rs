//! Supplied spell/target range snapshots shared by profile-specific API adapters.
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ActionSpellRange {
    pub has_range_requirements: bool,
    pub in_range_by_guid: HashMap<String, bool>,
}

pub(crate) fn lookup_range(sim: &crate::lua_api::SimState, slot: u32, unit: &str) -> Option<bool> {
    let target = crate::lua_api::globals::targeting_verbs::resolve_unit_snapshot(sim, unit)?;
    let spell = sim.action_bars.get(&slot)?;
    sim.action_spell_ranges
        .get(spell)?
        .in_range_by_guid
        .get(&target.guid)
        .copied()
}
