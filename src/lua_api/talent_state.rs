//! Talent tree interactive state (ranks purchased, selections, currency mappings).

use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct SeededTalentConfig {
    pub id: i32,
}

const HOLY_CONFIGS: [SeededTalentConfig; 2] = [
    SeededTalentConfig { id: 101 },
    SeededTalentConfig { id: 102 },
];

const PROTECTION_CONFIGS: [SeededTalentConfig; 2] = [
    SeededTalentConfig { id: 201 },
    SeededTalentConfig { id: 202 },
];

const RETRIBUTION_CONFIGS: [SeededTalentConfig; 2] = [
    SeededTalentConfig { id: 301 },
    SeededTalentConfig { id: 302 },
];

pub fn seeded_class_talent_configs(spec_id: u32) -> &'static [SeededTalentConfig] {
    match spec_id {
        65 => &HOLY_CONFIGS,
        66 => &PROTECTION_CONFIGS,
        70 => &RETRIBUTION_CONFIGS,
        _ => &[],
    }
}

pub fn default_class_talent_config_id(spec_id: u32) -> Option<i32> {
    seeded_class_talent_configs(spec_id)
        .first()
        .map(|config| config.id)
}

fn build_group_currency_map() -> HashMap<u32, u32> {
    use crate::traits::TRAIT_COND_DB;
    let mut map = HashMap::new();
    for (_, cond) in TRAIT_COND_DB.entries() {
        if cond.cond_type == 0 && cond.group_id != 0 && cond.currency_id != 0 {
            map.insert(cond.group_id, cond.currency_id);
        }
    }
    map
}

fn build_node_currency_map(group_currency_map: &HashMap<u32, u32>) -> HashMap<u32, u32> {
    use crate::traits::TRAIT_NODE_DB;
    let mut map = HashMap::new();
    for (&node_id, node) in TRAIT_NODE_DB.entries() {
        for &gid in node.group_ids {
            if let Some(&cid) = group_currency_map.get(&gid) {
                map.insert(node_id, cid);
                break;
            }
        }
    }
    map
}

fn seed_hero_spec_nodes(active_spec_id: u32) -> (HashMap<u32, u32>, HashMap<u32, u32>) {
    let mut node_ranks = HashMap::new();
    let mut node_selections = HashMap::new();
    super::globals::hero_talents::auto_select_hero_spec_for_spec(
        active_spec_id,
        &mut node_ranks,
        &mut node_selections,
    );
    (node_ranks, node_selections)
}

fn tally_currency_spent(
    node_ranks: &HashMap<u32, u32>,
    node_currency_map: &HashMap<u32, u32>,
) -> HashMap<u32, u32> {
    let mut spent = HashMap::new();
    for (&node_id, &ranks) in node_ranks {
        if let Some(&currency_id) = node_currency_map.get(&node_id) {
            *spent.entry(currency_id).or_insert(0) += ranks;
        }
    }
    spent
}

fn detect_active_hero_subtree(node_selections: &HashMap<u32, u32>) -> Option<u32> {
    node_selections.values().find_map(|entry_id| {
        crate::traits::TRAIT_ENTRY_DB
            .get(entry_id)
            .and_then(|entry| (entry.sub_tree_id != 0).then_some(entry.sub_tree_id))
    })
}

fn default_last_selected_config_ids() -> HashMap<u32, i32> {
    [65u32, 66, 70]
        .into_iter()
        .filter_map(|spec_id| {
            default_class_talent_config_id(spec_id).map(|config_id| (spec_id, config_id))
        })
        .collect()
}

/// Talent tree interactive state.
pub struct TalentState {
    /// Per-node purchased ranks: node_id → ranks_purchased (default 0).
    pub node_ranks: HashMap<u32, u32>,
    /// Per-node selected entry (for choice nodes): node_id → entry_id.
    pub node_selections: HashMap<u32, u32>,
    /// Group → currency mapping (built at init from cond_type=0 conditions).
    pub group_currency_map: HashMap<u32, u32>,
    /// Node → currency mapping (built at init from group membership).
    pub node_currency_map: HashMap<u32, u32>,
    /// Cached spent totals by currency for fast talent condition checks.
    pub currency_spent: HashMap<u32, u32>,
    /// Cached currently selected hero subtree, if any.
    pub active_hero_subtree_id: Option<u32>,
    /// Currently active seeded config for the active specialization.
    pub active_config_id: i32,
    /// Last selected seeded config per specialization.
    pub last_selected_config_id_by_spec_id: HashMap<u32, i32>,
    /// Whether the player can currently change talents. Drives
    /// `C_ClassTalents.CanChangeTalents`. Seeded true (out of combat,
    /// not in arena preparation). Flip off before firing combat enter
    /// events in tests.
    pub can_change_talents: bool,
    /// Whether the active class has a starter build available. Drives
    /// `C_ClassTalents.GetHasStarterBuild`. Seeded false.
    pub has_starter_build: bool,
    /// Whether the active talent config is the starter build. Drives
    /// `C_ClassTalents.IsStarterBuildActive`. Seeded false.
    pub is_starter_build_active: bool,
}

impl TalentState {
    /// Build talent state with currency mappings derived from the trait databases.
    pub fn new() -> Self {
        Self::for_spec_id(66)
    }

    /// Build talent state seeded for a specific specialization.
    pub fn for_spec_id(active_spec_id: u32) -> Self {
        let group_currency_map = build_group_currency_map();
        let node_currency_map = build_node_currency_map(&group_currency_map);
        let (node_ranks, node_selections) = seed_hero_spec_nodes(active_spec_id);
        let currency_spent = tally_currency_spent(&node_ranks, &node_currency_map);
        let active_hero_subtree_id = detect_active_hero_subtree(&node_selections);
        let last_selected_config_id_by_spec_id = default_last_selected_config_ids();
        let active_config_id = last_selected_config_id_by_spec_id
            .get(&active_spec_id)
            .copied()
            .unwrap_or(1);

        Self {
            node_ranks,
            node_selections,
            group_currency_map,
            node_currency_map,
            currency_spent,
            active_hero_subtree_id,
            active_config_id,
            last_selected_config_id_by_spec_id,
            can_change_talents: true,
            has_starter_build: false,
            is_starter_build_active: false,
        }
    }

    /// Total points spent for a given currency across all nodes.
    pub fn spent_for_currency(&self, currency_id: u32) -> u32 {
        self.currency_spent.get(&currency_id).copied().unwrap_or(0)
    }

    /// Set a node's purchased rank and update cached currency totals.
    pub fn set_node_rank(&mut self, node_id: u32, new_rank: u32) {
        let old_rank = self.node_ranks.get(&node_id).copied().unwrap_or(0);
        if old_rank == new_rank {
            return;
        }

        if new_rank == 0 {
            self.node_ranks.remove(&node_id);
        } else {
            self.node_ranks.insert(node_id, new_rank);
        }

        if let Some(&currency_id) = self.node_currency_map.get(&node_id) {
            let entry = self.currency_spent.entry(currency_id).or_insert(0);
            if new_rank >= old_rank {
                *entry += new_rank - old_rank;
            } else {
                *entry -= old_rank - new_rank;
            }
            if *entry == 0 {
                self.currency_spent.remove(&currency_id);
            }
        }
    }

    /// Clear all purchased ranks and cached spent totals.
    pub fn clear_ranks(&mut self) {
        self.node_ranks.clear();
        self.currency_spent.clear();
    }

    /// Update a node's selected entry and refresh the cached hero subtree when relevant.
    pub fn set_node_selection(&mut self, node_id: u32, entry_id: Option<u32>) {
        let Some(entry_id) = entry_id else {
            self.deselect_node(node_id);
            return;
        };
        self.node_selections.insert(node_id, entry_id);
        if let Some(entry) = crate::traits::TRAIT_ENTRY_DB.get(&entry_id) {
            if entry.sub_tree_id != 0 {
                self.active_hero_subtree_id = Some(entry.sub_tree_id);
            }
        }
    }

    fn deselect_node(&mut self, node_id: u32) {
        let removed_entry_id = self.node_selections.remove(&node_id);
        let removed_sub_tree = removed_entry_id
            .and_then(|eid| crate::traits::TRAIT_ENTRY_DB.get(&eid))
            .map(|entry| entry.sub_tree_id);
        let was_active = removed_sub_tree
            .zip(self.active_hero_subtree_id)
            .is_some_and(|(removed, active)| removed == active);
        if was_active {
            self.active_hero_subtree_id = detect_active_hero_subtree(&self.node_selections);
        }
    }

    /// Return the currently selected hero subtree, if any.
    pub fn active_hero_subtree(&self) -> Option<u32> {
        self.active_hero_subtree_id
    }

    pub fn switch_to_spec(&mut self, spec_id: u32) {
        let last_selected = self.last_selected_config_id_by_spec_id.clone();
        let can_change = self.can_change_talents;
        let has_starter = self.has_starter_build;
        *self = Self::for_spec_id(spec_id);
        self.last_selected_config_id_by_spec_id
            .extend(last_selected);
        self.active_config_id = self
            .last_selected_config_id_by_spec_id
            .get(&spec_id)
            .copied()
            .or_else(|| default_class_talent_config_id(spec_id))
            .unwrap_or(self.active_config_id);
        self.can_change_talents = can_change;
        self.has_starter_build = has_starter;
    }

    pub fn switch_to_loadout(&mut self, spec_id: u32, config_id: i32) {
        self.active_config_id = config_id;
        self.last_selected_config_id_by_spec_id
            .insert(spec_id, config_id);
    }
}
