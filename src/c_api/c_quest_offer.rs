//! Supplied quest-dialog major-faction reputation, separate from legacy rewards.

use std::collections::HashMap;

/// Environment-owned input. Native classification of legacy faction pairs is unknown.
#[derive(Debug, Clone, Default)]
pub struct QuestOfferState {
    /// Quest ID -> ordered (faction ID, reward amount) rows; absence is not an empty list.
    pub major_faction_rewards: HashMap<i32, Vec<(i32, i32)>>,
}
