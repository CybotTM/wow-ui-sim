//! Minimal hero talent helpers used by the current talent-state bootstrap.

use crate::lua_api::SimState;
use crate::traits::{TRAIT_COND_DB, TRAIT_ENTRY_DB, TRAIT_NODE_DB, TRAIT_TREE_DB};
use std::collections::{HashMap, HashSet};

pub(crate) fn spec_id_to_spec_set(spec_id: u32) -> u32 {
    match spec_id {
        65 => 27,
        66 => 28,
        70 => 29,
        _ => 0,
    }
}

fn find_spec_set_condition(cond_ids: &[u32]) -> u32 {
    for &cond_id in cond_ids {
        if let Some(cond) = TRAIT_COND_DB.get(&cond_id)
            && cond.cond_type == 1
        {
            return cond.spec_set_id;
        }
    }
    0
}

pub fn auto_select_hero_spec_for_spec(
    active_spec_id: u32,
    node_ranks: &mut HashMap<u32, u32>,
    node_selections: &mut HashMap<u32, u32>,
) {
    let spec_set = spec_id_to_spec_set(active_spec_id);
    if spec_set == 0 {
        return;
    }

    // The current in-tree talent data used here is for the paladin tree.
    let Some(tree) = TRAIT_TREE_DB.get(&790) else {
        return;
    };

    for &node_id in tree.node_ids {
        let Some(node) = TRAIT_NODE_DB.get(&node_id) else {
            continue;
        };
        if node.node_type != 3 || find_spec_set_condition(node.cond_ids) != spec_set {
            continue;
        }

        for &entry_id in node.entry_ids {
            let Some(entry) = TRAIT_ENTRY_DB.get(&entry_id) else {
                continue;
            };
            if entry.sub_tree_id != 0 {
                node_selections.insert(node_id, entry_id);
                node_ranks.insert(node_id, 1);
                return;
            }
        }
    }
}

pub fn selection_node_ids_for_subtree(sub_tree_id: u32) -> Vec<u32> {
    let mut node_ids = Vec::new();
    let mut seen_node_ids = HashSet::new();
    for node in TRAIT_NODE_DB.values() {
        if node.node_type != 3 {
            continue;
        }
        for &entry_id in node.entry_ids {
            let Some(entry) = TRAIT_ENTRY_DB.get(&entry_id) else {
                continue;
            };
            if entry.sub_tree_id == sub_tree_id && seen_node_ids.insert(node.id) {
                node_ids.push(node.id);
            }
        }
    }
    node_ids.sort_unstable();
    node_ids
}

pub fn subtree_position(sub_tree_id: u32) -> (i32, i32) {
    let mut sum_x = 0_i64;
    let mut min_y = i32::MAX;
    let mut count = 0_u32;

    for node in TRAIT_NODE_DB.values() {
        if node.sub_tree_id != sub_tree_id || node.node_type == 3 {
            continue;
        }
        sum_x += node.pos_x as i64;
        min_y = min_y.min(node.pos_y);
        count += 1;
    }

    if count == 0 {
        (0, 0)
    } else {
        ((sum_x / count as i64) as i32, min_y)
    }
}

pub fn get_active_hero_subtree(state: &SimState) -> Option<u32> {
    let active_spec_id = crate::specializations::specs_for_class(state.player.class_index as u32)
        .nth((state.player.active_spec_index - 1).max(0) as usize)
        .map(|spec| spec.id)
        .unwrap_or(66);
    let active_spec_set = spec_id_to_spec_set(active_spec_id);
    if active_spec_set == 0 {
        return None;
    }

    let tree = TRAIT_TREE_DB.get(&790)?;
    for &node_id in tree.node_ids {
        let Some(node) = TRAIT_NODE_DB.get(&node_id) else {
            continue;
        };
        if node.node_type != 3 || find_spec_set_condition(node.cond_ids) != active_spec_set {
            continue;
        }
        let Some(&entry_id) = state.talents.node_selections.get(&node_id) else {
            continue;
        };
        let Some(entry) = TRAIT_ENTRY_DB.get(&entry_id) else {
            continue;
        };
        if entry.sub_tree_id != 0 {
            return Some(entry.sub_tree_id);
        }
    }

    None
}
