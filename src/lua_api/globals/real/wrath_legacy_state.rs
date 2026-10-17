//! Explicit legacy consumer inputs. Skill storage is shared with Era/Anniversary;
//! other projections remain Wrath-only. Empty inputs are simulator policy.
use std::collections::{HashMap, HashSet};

pub use super::legacy_skills::Skill;
#[derive(Clone, Debug)]
pub struct Landmark {
    pub name: String,
    pub description: String,
    pub texture_index: i32,
    pub x: f64,
    pub y: f64,
    pub map_link_id: i32,
}
#[derive(Clone, Debug)]
pub struct Battlefield {
    pub name: String,
    pub description: String,
    /// Separate explicit input: never copied from max_players.
    pub max_group: i32,
}
#[derive(Debug, Default)]
pub struct WrathLegacyState {
    pub skills: Vec<Skill>,
    /// Zero denotes no selection; otherwise one-based into skills.
    pub selected_skill: usize,
    /// Explicit legacy (continent, zone) key; not a retail uiMapID.
    pub selected_map: (i32, i32),
    pub landmarks: HashMap<(i32, i32), Vec<Landmark>>,
    pub battlefields: Vec<Battlefield>,
    /// One-based index last requested through RequestBattlegroundInstanceInfo.
    pub requested_battlefield: Option<usize>,
    /// Explicit eligibility IDs in the shared mount/species identity domain.
    pub eligible_mounts: HashSet<i32>,
    pub eligible_critters: HashSet<i32>,
}
