//! Quest-log state for `C_QuestLog.*` probes.
//!
//! The `quest_log: Vec<u32>` in `SimState` is the lightweight
//! accept/abandon list used by `quest_verbs.rs`; `QuestLogState` carries
//! the rich metadata surfaced to the 17 ported probe methods.

use super::GossipQuestRow;
use std::collections::HashSet;
use std::num::TryFromIntError;

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
    /// Currency rewards returned by
    /// `C_QuestLog.GetQuestRewardCurrencies`. Empty for quests with no
    /// currency rewards.
    pub currency_rewards: Vec<QuestRewardCurrency>,
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

/// One currency-reward entry surfaced by
/// `C_QuestLog.GetQuestRewardCurrencies`. `texture` is a Blizzard-style
/// `Interface\Icons\…` path; the dialog passes it through `SetTexture`.
/// `base_reward_amount` is the pre-bonus value retail surfaces only
/// when known — `None` means "no base/bonus distinction for this
/// currency".
#[derive(Debug, Clone)]
pub struct QuestRewardCurrency {
    pub currency_id: i32,
    pub name: String,
    pub texture: String,
    pub total_reward_amount: i32,
    pub base_reward_amount: Option<i32>,
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
    /// Authored simulator transition, not native validation/reward behavior.
    /// History alone does not block a represented completed row (including seeds).
    pub fn turn_in(&mut self, quest_id: i32) -> bool {
        let eligible = quest_id > 0
            && self
                .entries
                .iter()
                .any(|entry| entry.quest_id == quest_id && entry.is_complete && !entry.is_failed);
        if !eligible {
            return false;
        }
        self.entries.retain(|entry| entry.quest_id != quest_id);
        self.completed_quest_ids.insert(quest_id);
        true
    }

    /// Snapshot the selected offer, retaining known metadata for this exact quest.
    /// Unknown ordinary offers use authored simulator policy, not native defaults.
    pub fn snapshot_gossip_offer(
        &self,
        offer: &GossipQuestRow,
    ) -> Result<QuestLogEntry, TryFromIntError> {
        let quest_id = i32::try_from(offer.quest_id)?;
        let mut snapshot = self
            .entries
            .iter()
            .find(|entry| entry.quest_id == quest_id)
            .cloned()
            .unwrap_or_else(|| ordinary_offer_entry(quest_id, offer));
        snapshot.title = offer.title.clone();
        snapshot.level = offer.quest_level;
        snapshot.is_complete = offer.is_complete == Some(true);
        snapshot.is_meta = offer.is_meta;
        snapshot.is_flagged_completed =
            snapshot.is_flagged_completed || self.completed_quest_ids.contains(&quest_id);
        Ok(snapshot)
    }

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

/// Explicit initial state for simulator-authored ordinary gossip offers.
/// POI polygons supply a map, never a quest-giver waypoint; other unsupported
/// metadata stays absent and this scenario has no item/currency rewards.
fn ordinary_offer_entry(quest_id: i32, offer: &GossipQuestRow) -> QuestLogEntry {
    QuestLogEntry {
        quest_id,
        title: offer.title.clone(),
        level: offer.quest_level,
        is_complete: offer.is_complete == Some(true),
        is_failed: false,
        is_meta: offer.is_meta,
        is_world_quest: false,
        is_replayable: false,
        is_flagged_completed: false,
        map_id: crate::quest_poi_blobs::get_quest_blobs(offer.quest_id)
            .first()
            .map(|blob| blob.map_id as i32),
        waypoint: None,
        tag_id: None,
        details_theme: None,
        criteria_spell_id: None,
        criteria_spell_name: None,
        criteria_spell_texture: None,
        criteria_spell_finished: false,
        reward_items: Vec::new(),
        currency_rewards: Vec::new(),
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
        currency_rewards: Vec::new(),
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
