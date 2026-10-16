//! Explicit Wrath consumer inputs. Empty inputs are simulator policy, not
//! observations of native startup. No retail table-to-legacy inference.
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Default)]
pub struct Skill {
    pub name: String,
    pub is_header: bool,
    pub is_expanded: bool,
    pub rank: i32,
    pub temporary_points: i32,
    pub modifier: i32,
    pub max_rank: i32,
    pub abandonable: bool,
    pub step_cost: Option<i32>,
    pub rank_cost: Option<i32>,
    pub min_level: i32,
    pub cost_type: i32,
    pub description: String,
}
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
