//! Tracked recipe state for the Profession Recipe Tracker.

/// Recipe IDs the Profession Recipe Tracker is currently tracking,
/// split by recraft state. WoW's `C_TradeSkillUI` keeps two distinct
/// lists: regular crafts and recrafts, queried with `isRecrafting`.
#[derive(Debug, Default, Clone)]
pub struct TrackedRecipes {
    /// Recipes tracked for normal crafting (`isRecrafting == false`).
    pub normal: Vec<u32>,
    /// Recipes tracked for recrafting (`isRecrafting == true`).
    pub recrafting: Vec<u32>,
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
    pub fn list_mut(&mut self, is_recrafting: bool) -> &mut Vec<u32> {
        if is_recrafting {
            &mut self.recrafting
        } else {
            &mut self.normal
        }
    }

    /// Whether `recipe_id` is tracked under the given `isRecrafting` flag.
    pub fn contains(&self, recipe_id: u32, is_recrafting: bool) -> bool {
        self.list(is_recrafting).contains(&recipe_id)
    }

    /// Adds or removes a recipe from the bucket. Returns true if the
    /// list changed (i.e. SetRecipeTracked should fire `TRACKED_RECIPE_UPDATE`).
    pub fn set(&mut self, recipe_id: u32, tracked: bool, is_recrafting: bool) -> bool {
        let bucket = self.list_mut(is_recrafting);
        let pos = bucket.iter().position(|&r| r == recipe_id);
        match (tracked, pos) {
            (true, None) => {
                bucket.push(recipe_id);
                true
            }
            (false, Some(idx)) => {
                bucket.remove(idx);
                true
            }
            _ => false,
        }
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
