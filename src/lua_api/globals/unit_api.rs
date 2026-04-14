//! Minimal unit helpers still referenced outside the deleted mlua module.

pub fn parse_party_index(unit: &str) -> Option<usize> {
    let suffix = unit.strip_prefix("party")?;
    let index: usize = suffix.parse().ok()?;
    index.checked_sub(1)
}
