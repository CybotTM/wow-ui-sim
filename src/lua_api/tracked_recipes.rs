//! Tracked recipe state for the Profession Recipe Tracker.

use std::collections::HashSet;

/// Recipe IDs the Profession Recipe Tracker is currently tracking,
/// split by recraft state. WoW's `C_TradeSkillUI` keeps two distinct
/// lists: regular crafts and recrafts, queried with `isRecrafting`.
#[derive(Debug, Default, Clone)]
pub struct TrackedRecipes {
    /// Recipes tracked for normal crafting (`isRecrafting == false`).
    pub normal: Vec<u32>,
    /// Recipes tracked for recrafting (`isRecrafting == true`).
    pub recrafting: Vec<u32>,
    normal_lookup: HashSet<u32>,
    recrafting_lookup: HashSet<u32>,
}

impl TrackedRecipes {
    /// Returns the bucket for the given `isRecrafting` flag.
    pub fn list(&self, is_recrafting: bool) -> &[u32] {
        if is_recrafting {
            &self.recrafting
        } else {
            &self.normal
        }
    }

    /// Returns the mutable bucket for the given `isRecrafting` flag.
    fn list_mut(&mut self, is_recrafting: bool) -> &mut Vec<u32> {
        if is_recrafting {
            &mut self.recrafting
        } else {
            &mut self.normal
        }
    }

    fn lookup(&self, is_recrafting: bool) -> &HashSet<u32> {
        if is_recrafting {
            &self.recrafting_lookup
        } else {
            &self.normal_lookup
        }
    }

    fn lookup_mut(&mut self, is_recrafting: bool) -> &mut HashSet<u32> {
        if is_recrafting {
            &mut self.recrafting_lookup
        } else {
            &mut self.normal_lookup
        }
    }

    /// Whether `recipe_id` is tracked under the given `isRecrafting` flag.
    pub fn contains(&self, recipe_id: u32, is_recrafting: bool) -> bool {
        self.lookup(is_recrafting).contains(&recipe_id)
    }

    /// Adds or removes a recipe from the bucket. Returns true if the
    /// list changed (i.e. SetRecipeTracked should fire `TRACKED_RECIPE_UPDATE`).
    pub fn set(&mut self, recipe_id: u32, tracked: bool, is_recrafting: bool) -> bool {
        if tracked {
            return self.add(recipe_id, is_recrafting);
        }
        self.remove(recipe_id, is_recrafting)
    }

    fn add(&mut self, recipe_id: u32, is_recrafting: bool) -> bool {
        if !self.lookup_mut(is_recrafting).insert(recipe_id) {
            return false;
        }
        self.list_mut(is_recrafting).push(recipe_id);
        true
    }

    fn remove(&mut self, recipe_id: u32, is_recrafting: bool) -> bool {
        if !self.lookup_mut(is_recrafting).remove(&recipe_id) {
            return false;
        }
        let bucket = self.list_mut(is_recrafting);
        if let Some(idx) = bucket.iter().position(|&r| r == recipe_id) {
            bucket.remove(idx);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracked_recipes_default_to_empty_lists() {
        let tracked = TrackedRecipes::default();

        assert!(tracked.normal.is_empty());
        assert!(tracked.recrafting.is_empty());
        assert!(!tracked.contains(100001, false));
        assert!(!tracked.contains(100001, true));
    }

    #[test]
    fn tracked_recipes_keep_normal_and_recrafting_buckets_separate() {
        let mut tracked = TrackedRecipes::default();

        assert!(tracked.set(100001, true, false));
        assert!(tracked.set(100001, true, true));
        assert!(tracked.set(100002, true, true));

        assert_eq!(tracked.list(false), &[100001]);
        assert_eq!(tracked.list(true), &[100001, 100002]);
        assert!(tracked.contains(100001, false));
        assert!(tracked.contains(100001, true));
        assert!(tracked.contains(100002, true));
        assert!(!tracked.contains(100002, false));

        assert!(tracked.set(100001, false, false));
        assert!(tracked.list(false).is_empty());
        assert_eq!(tracked.list(true), &[100001, 100002]);
        assert!(!tracked.contains(100001, false));
        assert!(tracked.contains(100001, true));
    }

    #[test]
    fn tracked_recipes_set_is_idempotent_per_bucket() {
        let mut tracked = TrackedRecipes::default();

        assert!(tracked.set(100005, true, false));
        assert!(!tracked.set(100005, true, false));
        assert!(tracked.set(100005, true, true));
        assert!(tracked.set(100005, false, true));
        assert!(!tracked.set(100005, false, true));
        assert!(tracked.set(100005, false, false));
        assert!(!tracked.contains(100005, true));
        assert!(!tracked.contains(100005, false));
    }
}
