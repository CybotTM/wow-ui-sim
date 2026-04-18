//! Anchor dependency tracking: reverse index and cycle detection.

use std::collections::{HashMap, VecDeque};

use rustc_hash::{FxHashMap, FxHashSet};

use super::WidgetRegistry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnchorCyclePath {
    pub relative_to_id: u64,
    pub dependent_id: u64,
    pub dependent_ancestors: Vec<u64>,
}

impl WidgetRegistry {
    /// Check if setting a point from `frame_id` to `relative_to_id` would create a cycle.
    /// A cycle exists if relative_to (or any of its anchor dependencies) already
    /// depends on frame_id.
    pub fn would_create_anchor_cycle(&self, frame_id: u64, relative_to_id: u64) -> bool {
        frame_id == relative_to_id || self.anchor_dependency_reaches_frame(relative_to_id, frame_id)
    }

    pub fn describe_anchor_cycle(
        &self,
        frame_id: u64,
        relative_to_id: u64,
    ) -> Option<AnchorCyclePath> {
        if frame_id == relative_to_id {
            return Some(AnchorCyclePath {
                relative_to_id,
                dependent_id: relative_to_id,
                dependent_ancestors: Vec::new(),
            });
        }

        let path = self.anchor_dependency_path(relative_to_id, frame_id)?;
        let dependent_id = path.iter().rev().nth(1).copied().unwrap_or(relative_to_id);
        let dependent_ancestors = path[..path.len().saturating_sub(2)]
            .iter()
            .rev()
            .copied()
            .collect();

        Some(AnchorCyclePath {
            relative_to_id,
            dependent_id,
            dependent_ancestors,
        })
    }

    fn anchor_dependency_reaches_frame(&self, start_id: u64, target_id: u64) -> bool {
        let mut queue = VecDeque::from([start_id]);
        let mut seen = FxHashSet::from_iter([start_id]);

        while let Some(check_id) = queue.pop_front() {
            if self.enqueue_anchor_dependencies(check_id, target_id, &mut queue, &mut seen) {
                return true;
            }
        }

        false
    }

    fn anchor_dependency_path(&self, start_id: u64, target_id: u64) -> Option<Vec<u64>> {
        let mut queue = VecDeque::from([start_id]);
        let mut seen = FxHashSet::from_iter([start_id]);
        let mut parents = HashMap::from([(start_id, start_id)]);

        while let Some(check_id) = queue.pop_front() {
            for anchor_target_id in self.anchor_target_ids(check_id) {
                if seen.insert(anchor_target_id) {
                    parents.insert(anchor_target_id, check_id);
                    if anchor_target_id == target_id {
                        return Some(rebuild_anchor_cycle_path(&parents, start_id, target_id));
                    }
                    queue.push_back(anchor_target_id);
                }
            }
        }

        None
    }

    fn enqueue_anchor_dependencies(
        &self,
        frame_id: u64,
        target_id: u64,
        queue: &mut VecDeque<u64>,
        seen: &mut FxHashSet<u64>,
    ) -> bool {
        for anchor_target_id in self.anchor_target_ids(frame_id) {
            if anchor_target_id == target_id {
                return true;
            }
            if seen.insert(anchor_target_id) {
                queue.push_back(anchor_target_id);
            }
        }

        false
    }

    fn anchor_target_ids(&self, frame_id: u64) -> impl Iterator<Item = u64> + '_ {
        self.widgets
            .get(&frame_id)
            .into_iter()
            .flat_map(|frame| frame.anchors.iter())
            .filter_map(|anchor| anchor.relative_to_id.map(|target_id| target_id as u64))
    }

    /// Record that `frame_id` is anchored to `target_id`.
    pub fn add_anchor_dependent(&mut self, target_id: u64, frame_id: u64) {
        self.anchor_dependents
            .entry(target_id)
            .or_default()
            .insert(frame_id);
    }

    /// Remove `frame_id` from `target_id`'s dependents.
    pub fn remove_anchor_dependent(&mut self, target_id: u64, frame_id: u64) {
        if let Some(set) = self.anchor_dependents.get_mut(&target_id) {
            set.remove(&frame_id);
            if set.is_empty() {
                self.anchor_dependents.remove(&target_id);
            }
        }
    }

    /// Remove `frame_id` from all reverse-index entries by reading its current
    /// anchors to find the targets.
    pub fn remove_all_anchor_dependents_for(&mut self, frame_id: u64) {
        let targets: Vec<u64> = self
            .widgets
            .get(&frame_id)
            .map(|f| {
                f.anchors
                    .iter()
                    .filter_map(|a| a.relative_to_id.map(|t| t as u64))
                    .collect()
            })
            .unwrap_or_default();
        for target in targets {
            self.remove_anchor_dependent(target, frame_id);
        }
    }

    /// Get frame IDs anchored to `target_id`.
    pub fn get_anchor_dependents(&self, target_id: u64) -> Option<&FxHashSet<u64>> {
        self.anchor_dependents.get(&target_id)
    }

    /// Rebuild the reverse anchor index from all existing anchors.
    /// Call once after initial load to capture anchors set during XML parsing
    /// and frame creation.
    pub fn rebuild_anchor_index(&mut self) {
        self.anchor_dependents.clear();
        let entries: Vec<(u64, u64)> = self
            .widgets
            .values()
            .flat_map(|f| {
                f.anchors
                    .iter()
                    .filter_map(move |a| a.relative_to_id.map(|target| (target as u64, f.id)))
            })
            .collect();
        for (target, frame_id) in entries {
            self.anchor_dependents
                .entry(target)
                .or_default()
                .insert(frame_id);
        }
    }
}

pub(super) fn hash_map_u64_hash_set_u64_bytes(values: &FxHashMap<u64, FxHashSet<u64>>) -> usize {
    values.capacity() * std::mem::size_of::<(u64, FxHashSet<u64>)>()
        + values
            .values()
            .map(|s| s.capacity() * std::mem::size_of::<u64>())
            .sum::<usize>()
}

fn rebuild_anchor_cycle_path(
    parents: &HashMap<u64, u64>,
    start_id: u64,
    target_id: u64,
) -> Vec<u64> {
    let mut path = vec![target_id];
    let mut current = target_id;

    while current != start_id {
        current = parents.get(&current).copied().unwrap_or(start_id);
        path.push(current);
    }

    path.reverse();
    path
}
