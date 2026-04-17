//! Quest-log state for `C_QuestLog.*` probes.
//!
//! The `quest_log: Vec<u32>` in `SimState` is the lightweight
//! accept/abandon list used by `quest_verbs.rs`; `QuestLogState` carries
//! the rich metadata surfaced to the 17 ported probe methods.

use std::collections::HashSet;

/// Rich quest-log entry used by `C_QuestLog.*` probes.
#[derive(Debug, Clone)]
pub struct QuestLogEntry {
    pub quest_id: i32,
    pub title: String,
    pub level: i32,
    pub is_complete: bool,
    pub is_failed: bool,
    pub is_meta: bool,
    pub is_world_quest: bool,
    pub is_replayable: bool,
    pub is_flagged_completed: bool,
    pub map_id: Option<i32>,
    /// Normalised map x/y (0.0–1.0).
    pub waypoint: Option<(f64, f64)>,
    /// `tagID` value for `GetQuestTagInfo`.
    pub tag_id: Option<i32>,
    /// Theme key returned by `GetQuestDetailsTheme`.
    pub details_theme: Option<String>,
}

/// Backing state for `C_QuestLog.*` probes.
#[derive(Debug, Clone, Default)]
pub struct QuestLogState {
    pub entries: Vec<QuestLogEntry>,
    /// Quest IDs already turned in / permanently completed.
    /// Drives `GetAllCompletedQuestIDs` and `IsQuestFlaggedCompleted`.
    pub completed_quest_ids: HashSet<i32>,
}

impl QuestLogState {
    pub fn seeded() -> Self {
        let entries = vec![
            QuestLogEntry {
                quest_id: 80000,
                title: "The Lost Expedition".into(),
                level: 80,
                is_complete: false,
                is_failed: false,
                is_meta: false,
                is_world_quest: false,
                is_replayable: false,
                is_flagged_completed: false,
                map_id: Some(2248),
                waypoint: Some((0.45, 0.35)),
                tag_id: Some(0),
                details_theme: None,
            },
            QuestLogEntry {
                quest_id: 80001,
                title: "Defending the Gates".into(),
                level: 80,
                is_complete: true,
                is_failed: false,
                is_meta: false,
                is_world_quest: false,
                is_replayable: false,
                is_flagged_completed: false,
                map_id: Some(2248),
                waypoint: None,
                tag_id: Some(0),
                details_theme: None,
            },
            QuestLogEntry {
                quest_id: 90101,
                title: "Earthen Relic Recovery".into(),
                level: 80,
                is_complete: false,
                is_failed: false,
                is_meta: false,
                is_world_quest: true,
                is_replayable: true,
                is_flagged_completed: false,
                map_id: Some(2248),
                waypoint: Some((0.62, 0.58)),
                tag_id: Some(2),
                details_theme: None,
            },
        ];
        let mut completed_quest_ids = HashSet::new();
        completed_quest_ids.insert(79999);
        completed_quest_ids.insert(80001);
        Self {
            entries,
            completed_quest_ids,
        }
    }
}
