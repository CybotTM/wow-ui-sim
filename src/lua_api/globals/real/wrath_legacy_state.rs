//! Explicit legacy consumer inputs. Skill storage is shared with Era/Anniversary;
//! other projections remain Wrath-only. Empty inputs are simulator policy.
use std::collections::{HashMap, HashSet};

pub use super::legacy_skills::Skill;

/// INFERRED simulator inputs, not authenticated native cache initialization.
#[cfg(feature = "client-wrath")]
#[derive(Debug)]
pub struct CalendarContext {
    pub player_present: bool,
    /// 0=base, 1=Burning Crusade, 2=Wrath.
    pub account_expansion: u8,
    pub action_pending: bool,
    /// INFERRED cached substitute for the native throttle CVar's unknown public name.
    pub invite_throttle_bypass: bool,
    /// Milliseconds on the existing GetTime monotonic clock; zero means no invite.
    pub last_invite_time_ms: u64,
}

#[cfg(feature = "client-wrath")]
impl Default for CalendarContext {
    fn default() -> Self {
        Self {
            player_present: true,
            account_expansion: 2,
            action_pending: false,
            invite_throttle_bypass: false,
            last_invite_time_ms: 0,
        }
    }
}

#[cfg(feature = "client-wrath")]
impl CalendarContext {
    const INVITE_THROTTLE_MS: u64 = 2000;

    /// Native read-only predicate; an explicit clock value makes boundary tests exact.
    pub fn can_send_invite_at(&self, now_ms: u64) -> bool {
        if self.action_pending {
            return false;
        }
        if self.invite_throttle_bypass {
            return true;
        }
        let never_invited = self.last_invite_time_ms == 0;
        let elapsed_ms = now_ms.wrapping_sub(self.last_invite_time_ms);
        never_invited || elapsed_ms >= Self::INVITE_THROTTLE_MS
    }
}

/// Supplied talent-count caches; defaults are INFERRED simulator initialization.
#[cfg(feature = "client-wrath")]
#[derive(Debug)]
pub struct TalentGroupCounts {
    pub player: u32,
    pub inspected: u32,
    pub pet_tab_present: bool,
    pub pet_tab_count: u32,
}

#[cfg(feature = "client-wrath")]
impl Default for TalentGroupCounts {
    fn default() -> Self {
        Self {
            player: 1,
            inspected: 1,
            pet_tab_present: false,
            pet_tab_count: 0,
        }
    }
}

/// Native fresh filter masks/selection are all sentinel FFFFFFFF.
#[cfg(feature = "client-wrath")]
#[derive(Debug)]
pub struct TradeSkillFilters {
    pub subclass_mask: u32,
    pub selected_subclass: u32,
    pub inventory_mask: u32,
}

#[cfg(feature = "client-wrath")]
impl Default for TradeSkillFilters {
    fn default() -> Self {
        Self {
            subclass_mask: u32::MAX,
            selected_subclass: u32::MAX,
            inventory_mask: u32::MAX,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Landmark {
    pub name: String,
    pub description: String,
    pub texture_index: i32,
    pub x: f64,
    pub y: f64,
    pub map_link_id: i32,
}
#[derive(Clone, Debug)]
pub struct Battlefield {
    pub name: String,
    pub description: String,
    /// Separate explicit input: never copied from max_players.
    pub max_group: i32,
}
/// Explicit host creature-cache input, not journal IDs or native default metadata.
#[derive(Clone, Debug)]
pub struct CompanionMetadata {
    pub creature_id: u32,
    pub name: Option<String>,
    pub icon_path: Option<String>,
    pub active: bool,
}

#[derive(Debug, Default)]
pub struct WrathLegacyState {
    /// Cached zero-based dual-spec groups, not class specialization indices.
    /// INFERRED simulator default: one talent group; native initialization unproved.
    #[cfg(feature = "client-wrath")]
    pub player_talent_group: u32,
    #[cfg(feature = "client-wrath")]
    pub inspected_talent_group: u32,
    #[cfg(feature = "client-wrath")]
    pub talent_group_counts: TalentGroupCounts,
    /// Lazily allocated model handles, retained within this environment.
    #[cfg(feature = "client-wrath")]
    pub mini_map_arrows: Option<(u64, u64)>,
    #[cfg(feature = "client-wrath")]
    pub calendar: CalendarContext,
    #[cfg(feature = "client-wrath")]
    pub trade_skill_filters: TradeSkillFilters,
    pub skills: Vec<Skill>,
    /// Zero denotes no selection; otherwise one-based into skills.
    pub selected_skill: usize,
    /// Explicit legacy (continent, zone) key; not a retail uiMapID.
    pub selected_map: (i32, i32),
    pub landmarks: HashMap<(i32, i32), Vec<Landmark>>,
    pub battlefields: Vec<Battlefield>,
    /// One-based index last requested through RequestBattlegroundInstanceInfo.
    pub requested_battlefield: Option<usize>,
    /// Explicit eligibility IDs in the shared mount/species identity domain.
    pub eligible_mounts: HashSet<i32>,
    pub eligible_critters: HashSet<i32>,
    /// Spell -> supplied creature metadata; empty until the host supplies it.
    /// Native metadata for the default mount catalog is unresolved.
    pub companion_metadata: HashMap<u32, CompanionMetadata>,
}
