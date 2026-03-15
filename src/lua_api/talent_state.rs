//! Talent tree interactive state (ranks purchased, selections, currency mappings).

use std::collections::HashMap;

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
}

impl TalentState {
    /// Build talent state with currency mappings derived from the trait databases.
    pub fn new() -> Self {
        use crate::traits::{TRAIT_COND_DB, TRAIT_NODE_DB};

        // Build group → currency map from gate conditions (cond_type == 0).
        let mut group_currency_map = HashMap::new();
        for (_, cond) in TRAIT_COND_DB.entries() {
            if cond.cond_type == 0 && cond.group_id != 0 && cond.currency_id != 0 {
                group_currency_map.insert(cond.group_id, cond.currency_id);
            }
        }

        // Build node → currency map from each node's group membership.
        let mut node_currency_map = HashMap::new();
        for (&node_id, node) in TRAIT_NODE_DB.entries() {
            for &gid in node.group_ids {
                if let Some(&cid) = group_currency_map.get(&gid) {
                    node_currency_map.insert(node_id, cid);
                    break;
                }
            }
        }

        let mut node_ranks = HashMap::new();
        let mut node_selections = HashMap::new();

        // Auto-select the first hero spec so hero talent UI displays by default.
        super::globals::hero_talents::auto_select_hero_spec(&mut node_ranks, &mut node_selections);

        let mut currency_spent = HashMap::new();
        for (&node_id, &ranks) in &node_ranks {
            if let Some(&currency_id) = node_currency_map.get(&node_id) {
                *currency_spent.entry(currency_id).or_insert(0) += ranks;
            }
        }
        let active_hero_subtree_id = node_selections.values().find_map(|entry_id| {
            crate::traits::TRAIT_ENTRY_DB
                .get(entry_id)
                .and_then(|entry| (entry.sub_tree_id != 0).then_some(entry.sub_tree_id))
        });

        Self {
            node_ranks,
            node_selections,
            group_currency_map,
            node_currency_map,
            currency_spent,
            active_hero_subtree_id,
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
        match entry_id {
            Some(entry_id) => {
                self.node_selections.insert(node_id, entry_id);
                if let Some(entry) = crate::traits::TRAIT_ENTRY_DB.get(&entry_id)
                    && entry.sub_tree_id != 0
                {
                    self.active_hero_subtree_id = Some(entry.sub_tree_id);
                }
            }
            None => {
                let removed = self.node_selections.remove(&node_id);
                if removed
                    .and_then(|entry_id| crate::traits::TRAIT_ENTRY_DB.get(&entry_id))
                    .is_some_and(|entry| Some(entry.sub_tree_id) == self.active_hero_subtree_id)
                {
                    self.active_hero_subtree_id = self.node_selections.values().find_map(|entry_id| {
                        crate::traits::TRAIT_ENTRY_DB
                            .get(entry_id)
                            .and_then(|entry| (entry.sub_tree_id != 0).then_some(entry.sub_tree_id))
                    });
                }
            }
        }
    }

    /// Return the currently selected hero subtree, if any.
    pub fn active_hero_subtree(&self) -> Option<u32> {
        self.active_hero_subtree_id
    }
}
