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
    /// Optional spell objective shown in `QuestInfo_ShowSpecialObjectives`.
    pub criteria_spell_id: Option<i32>,
    pub criteria_spell_name: Option<String>,
    pub criteria_spell_texture: Option<String>,
    pub criteria_spell_finished: bool,
    /// Item rewards iterated by `GetNumQuestLogRewards` /
    /// `GetQuestLogRewardInfo`. Empty for quests with no item rewards.
    pub reward_items: Vec<QuestRewardItem>,
}

/// One item-shaped reward entry surfaced by `GetQuestLogRewardInfo`.
/// `quality` is the `Enum.ItemQuality` ordinal (0=Poor … 7=Heirloom).
#[derive(Debug, Clone)]
pub struct QuestRewardItem {
    pub name: String,
    pub texture: String,
    pub count: i32,
    pub quality: i32,
    pub is_usable: bool,
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
        Self {
            entries: vec![
                lost_expedition_entry(),
                defending_the_gates_entry(),
                glittering_geodes_entry(),
                earthen_relic_recovery_entry(),
            ],
            completed_quest_ids: seeded_completed_quest_ids(),
        }
    }
}

/// Shared field defaults for seeded entries: level 80, Khaz Algar map
/// (2248), tag 0, all bool flags false, no waypoint / details theme.
/// Per-quest builders override the distinctive fields via struct-update
/// syntax.
fn seed_quest_defaults() -> QuestLogEntry {
    QuestLogEntry {
        quest_id: 0,
        title: String::new(),
        level: 80,
        is_complete: false,
        is_failed: false,
        is_meta: false,
        is_world_quest: false,
        is_replayable: false,
        is_flagged_completed: false,
        map_id: Some(2248),
        waypoint: None,
        tag_id: Some(0),
        details_theme: None,
        criteria_spell_id: None,
        criteria_spell_name: None,
        criteria_spell_texture: None,
        criteria_spell_finished: false,
        reward_items: Vec::new(),
    }
}

fn lost_expedition_entry() -> QuestLogEntry {
    QuestLogEntry {
        quest_id: 80000,
        title: "The Lost Expedition".into(),
        waypoint: Some((0.45, 0.35)),
        ..seed_quest_defaults()
    }
}

fn defending_the_gates_entry() -> QuestLogEntry {
    QuestLogEntry {
        quest_id: 80001,
        title: "Defending the Gates".into(),
        is_complete: true,
        ..seed_quest_defaults()
    }
}

fn earthen_relic_recovery_entry() -> QuestLogEntry {
    QuestLogEntry {
        quest_id: 90101,
        title: "Earthen Relic Recovery".into(),
        is_world_quest: true,
        is_replayable: true,
        waypoint: Some((0.62, 0.58)),
        tag_id: Some(2),
        ..seed_quest_defaults()
    }
}

fn glittering_geodes_entry() -> QuestLogEntry {
    QuestLogEntry {
        quest_id: 90001,
        title: "Glittering Geodes".into(),
        is_world_quest: true,
        is_replayable: true,
        map_id: Some(2025),
        waypoint: Some((0.52, 0.63)),
        tag_id: Some(2),
        ..seed_quest_defaults()
    }
}

fn seeded_completed_quest_ids() -> HashSet<i32> {
    HashSet::from([79999, 80001])
}
