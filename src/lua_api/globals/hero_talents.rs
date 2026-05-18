//! Minimal hero talent helpers used by the current talent-state bootstrap.

use crate::lua_api::SimState;
use crate::traits::{TRAIT_COND_DB, TRAIT_ENTRY_DB, TRAIT_NODE_DB, TRAIT_TREE_DB};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

static SPEC_SET_BY_SPEC_ID: LazyLock<HashMap<u32, u32>> = LazyLock::new(build_spec_set_map);

pub(crate) fn spec_id_to_spec_set(spec_id: u32) -> u32 {
    SPEC_SET_BY_SPEC_ID.get(&spec_id).copied().unwrap_or(0)
}

pub(crate) fn class_tree_for_spec(spec_id: u32) -> Option<u32> {
    let class_id = crate::specializations::spec_by_id(spec_id)?.class_id;
    class_tree_id_for_class(class_id)
}

fn build_spec_set_map() -> HashMap<u32, u32> {
    let mut spec_sets = HashMap::new();
    for class_id in 1..=13 {
        let specs = crate::specializations::specs_for_class(class_id).collect::<Vec<_>>();
        let Some(tree_id) = class_tree_id_for_class(class_id) else {
            continue;
        };
        let Some(tree) = TRAIT_TREE_DB.get(&tree_id) else {
            continue;
        };
        for (spec, spec_set_id) in specs.into_iter().zip(tree_spec_sets(tree)) {
            spec_sets.insert(spec.id, spec_set_id);
        }
    }
    spec_sets
}

fn class_tree_id_for_class(class_id: u32) -> Option<u32> {
    match class_id {
        1 => Some(1000), // Warrior
        2 => Some(790),  // Paladin
        3 => Some(795),  // Hunter
        4 => Some(852),  // Rogue
        5 => Some(720),  // Priest
        6 => Some(850),  // Death Knight
        7 => Some(872),  // Shaman
        8 => Some(658),  // Mage
        9 => Some(786),  // Warlock
        10 => Some(774), // Monk
        11 => Some(793), // Druid
        12 => Some(701), // Demon Hunter
        13 => Some(854), // Evoker
        _ => None,
    }
}

fn tree_spec_sets(tree: &crate::traits::TraitTreeInfo) -> Vec<u32> {
    let mut spec_sets = HashSet::new();
    for &node_id in tree.node_ids {
        let Some(node) = TRAIT_NODE_DB.get(&node_id) else {
            continue;
        };
        for &cond_id in node.cond_ids.iter().chain(node.group_cond_ids.iter()) {
            let Some(cond) = TRAIT_COND_DB.get(&cond_id) else {
                continue;
            };
            if cond.cond_type == 1 && cond.spec_set_id != 0 {
                spec_sets.insert(cond.spec_set_id);
            }
        }
    }
    let mut spec_sets = spec_sets.into_iter().collect::<Vec<_>>();
    spec_sets.sort_unstable();
    spec_sets
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
