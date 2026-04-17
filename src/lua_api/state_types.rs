//! Plain data types used by SimState.

use crate::lua_api::game_data::AuraInfo;
use std::collections::{HashMap, HashSet, VecDeque};

pub use crate::lua_api::timer_layout::RiluaPendingTimer as PendingTimer;

/// What is currently held on the cursor (drag-and-drop state).
#[derive(Debug, Clone)]
pub enum CursorInfo {
    /// An action bar spell: PickupAction(slot) removes it from the bar.
    Action { slot: u32, spell_id: u32 },
    /// A spell from the spellbook (doesn't remove from spellbook).
    Spell { spell_id: u32 },
    /// A talent picked from the talent frame. `pvp=true` when sourced
    /// from the PvP talent pane.
    Talent { talent_id: u32, pvp: bool },
    /// A pet-action spell picked from the pet action bar.
    PetAction { slot: u32, spell_id: u32 },
    /// A macro picked up by slot index.
    Macro { macro_index: u32 },
    /// An item picked up from a bag slot, equipment slot, or merchant.
    Item {
        item_id: u32,
        stack_count: i32,
        origin: CursorItemOrigin,
    },
}

/// Where a cursor-carried item came from — used to route drops back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorItemOrigin {
    Bag { bag: i32, slot: i32 },
    Equipped { slot: i32 },
    Merchant { index: u32 },
    Unknown,
}

/// Per-addon runtime profiler metrics, updated each frame.
#[derive(Debug, Clone)]
pub struct AddonRuntimeMetrics {
    /// Time spent in this addon's handlers during the current frame (accumulator).
    pub current_frame_ms: f64,
    /// Rolling window of per-frame times (last 60 frames) for RecentAverageTime.
    pub recent_frames: VecDeque<f64>,
    /// Peak time ever recorded in a single frame.
    pub peak_ms: f64,
    /// Session total time (ms) across all frames.
    pub session_total_ms: f64,
    /// Number of frames where this addon had handlers fire.
    pub session_frame_count: u64,
    /// Threshold counters: frames where addon time exceeded N ms.
    pub count_over_1ms: u32,
    pub count_over_5ms: u32,
    pub count_over_10ms: u32,
    pub count_over_50ms: u32,
    pub count_over_100ms: u32,
    pub count_over_500ms: u32,
    pub count_over_1000ms: u32,
}

impl Default for AddonRuntimeMetrics {
    fn default() -> Self {
        Self {
            current_frame_ms: 0.0,
            recent_frames: VecDeque::with_capacity(60),
            peak_ms: 0.0,
            session_total_ms: 0.0,
            session_frame_count: 0,
            count_over_1ms: 0,
            count_over_5ms: 0,
            count_over_10ms: 0,
            count_over_50ms: 0,
            count_over_100ms: 0,
            count_over_500ms: 0,
            count_over_1000ms: 0,
        }
    }
}

/// Application-level frame timing for profiler (total frame time, not just addon time).
#[derive(Debug, Clone, Default)]
pub struct AppFrameMetrics {
    /// Rolling window of total frame times in ms (last 60 frames).
    pub recent_frame_ms: VecDeque<f64>,
    /// Peak frame time ever recorded.
    pub peak_ms: f64,
    /// Session total frame time in ms.
    pub session_total_ms: f64,
    /// Number of frames recorded.
    pub session_frame_count: u64,
}

/// Information about a loaded addon.
#[derive(Debug, Clone, Default)]
pub struct AddonInfo {
    /// Folder name (used as addon identifier).
    pub folder_name: String,
    /// Display title from TOC metadata.
    pub title: String,
    /// Notes/description from TOC metadata.
    pub notes: String,
    /// Whether the addon is currently enabled.
    pub enabled: bool,
    /// Whether the addon was successfully loaded.
    pub loaded: bool,
    /// Load on demand flag.
    pub load_on_demand: bool,
    /// Whether the addon loads Lua/XML chunks in the secure environment.
    pub use_secure_env: bool,
    /// Total load time in seconds (for profiler metrics).
    pub load_time_secs: f64,
    /// Runtime profiler metrics (updated per frame).
    pub runtime: AddonRuntimeMetrics,
}

/// A collected Lua error with optional addon attribution.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LuaErrorRecord {
    /// Raw collected error message.
    pub message: String,
    /// Addon name inferred from the loading/executing context or Lua stack.
    pub addon_name: Option<String>,
}

/// A missing symbol access captured through `_G` or `C_*` namespace `__index` hooks.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NilSymbolAccess {
    /// Addon name inferred from the loading/executing context.
    pub addon_name: Option<String>,
    /// Container table where the miss happened (`_G` or `C_*` namespace name).
    pub container: String,
    /// Missing key that resolved to nil.
    pub key: String,
    /// Raw Lua chunk source reported by `debug.getinfo`, if available.
    pub source: Option<String>,
    /// 1-based source line where the nil access happened, if available.
    pub line: Option<i32>,
}

/// A Great Vault activity slot (one row/tier in the weekly rewards UI).
#[derive(Debug, Clone)]
pub struct GreatVaultActivity {
    /// WeeklyRewardChestThresholdType: 1=Activities, 2=Raid, 4=RankedPvP, 5=World.
    pub activity_type: i32,
    /// Slot index within the row (1-3).
    pub index: i32,
    /// Number of activities required to unlock this slot.
    pub threshold: i32,
    /// Current progress toward the threshold.
    pub progress: i32,
    /// Key level, boss difficulty, or rating.
    pub level: i32,
}

/// A mount in the mount journal.
#[derive(Debug, Clone)]
pub struct MountData {
    pub mount_id: u32,
    pub name: String,
    pub spell_id: u32,
    pub icon: u32,
    pub is_collected: bool,
    pub is_usable: bool,
    pub mount_type: u32,
}

/// A battle pet in the pet journal.
#[derive(Debug, Clone)]
pub struct PetData {
    pub pet_id: String,
    pub species_id: u32,
    pub name: String,
    pub icon: u32,
    pub pet_type: i32,
    pub level: i32,
    pub quality: i32,
    pub is_collected: bool,
}

/// A toy in the toy box.
#[derive(Debug, Clone)]
pub struct ToyData {
    pub item_id: u32,
    pub name: String,
    pub icon: u32,
    pub is_collected: bool,
    pub is_usable: bool,
}

/// An heirloom item in the collection.
#[derive(Debug, Clone)]
pub struct HeirloomData {
    pub item_id: u32,
    pub name: String,
    /// Equipment location string (e.g. "INVTYPE_HEAD", "INVTYPE_SHOULDER").
    pub equip_loc: String,
    /// Icon fileDataID.
    pub icon: u32,
    /// Current upgrade level (0 = base, max varies by expansion).
    pub upgrade_level: i32,
    /// Source description (e.g. "Vendor", "Achievement").
    pub source: String,
    /// Minimum effective level.
    pub min_level: i32,
    /// Maximum effective level at current upgrade.
    pub max_level: i32,
}

/// One chat bubble visible in the world (NPC speech, emote, say, etc.).
/// Drives `C_ChatBubbles.GetAllChatBubbles`. Since we don't render 3D
/// world chat bubbles the frame_id is advisory only — callers get a
/// lightweight table per bubble rather than a real Frame.
#[derive(Debug, Clone)]
pub struct ChatBubble {
    /// The message text shown in the bubble.
    pub message: String,
    /// Name of the unit speaking (e.g. "Thrall").
    pub sender: String,
    /// Chat type string (e.g. "SAY", "YELL", "EMOTE").
    pub chat_type: String,
    /// Frame id of the owning frame when one exists.
    pub frame_id: Option<u64>,
}

/// One row of the Auction House Browse results list. Drives
/// `C_AuctionHouse.GetBrowseResults`. Seeded with a couple of
/// representative entries.
#[derive(Debug, Clone)]
pub struct AuctionBrowseResult {
    pub item_id: i32,
    pub item_level: i32,
    /// Seller-posted minimum price in copper.
    pub min_price: i64,
    pub total_quantity: i32,
    pub contains_owner_item: bool,
}

/// One row of the Auction House replicate scan (commodities listing
/// snapshot). Drives `C_AuctionHouse.GetReplicateItemInfo`.
#[derive(Debug, Clone)]
pub struct AuctionReplicateItem {
    pub name: String,
    pub texture: u32,
    pub count: i32,
    pub quality_id: i32,
    pub usable: bool,
    pub level: i32,
    pub level_type: String,
}

/// Metadata for an LFG category (Dungeons, Raids, etc.). Drives
/// `C_LFGInfo.GetLFGCategoryInfo(categoryID)`. Seeded with the two
/// standard retail categories: 2 = Dungeons, 3 = Raids.
#[derive(Debug, Clone)]
pub struct LfgCategoryInfo {
    /// Display name shown in the Group Finder tab.
    pub name: String,
    /// Whether the category is available to the player.
    pub order: i32,
}

/// One affix in the active Mythic+ season. Drives
/// `C_MythicPlus.GetCurrentAffixes`.
#[derive(Debug, Clone)]
pub struct MythicPlusAffix {
    /// Dungeon Journal affix id (e.g. 9 = Tyrannical).
    pub id: i32,
    /// Season in which this affix rotates.
    pub season_id: i32,
}

/// Weekly best result for a specific Mythic+ map. Drives
/// `C_MythicPlus.GetWeeklyBestForMap`.
#[derive(Debug, Clone)]
pub struct MythicPlusWeeklyBest {
    /// Challenge-mode dungeon id.
    pub map_challenge_mode_id: i32,
    /// Keystone level completed this week.
    pub level: i32,
    /// Completion time in seconds.
    pub duration_sec: i32,
    /// Mythic+ score contribution for this run.
    pub score: f64,
}

/// One completed or in-progress Mythic+ run. Drives
/// `C_MythicPlus.GetRunHistory`.
#[derive(Debug, Clone)]
pub struct MythicPlusRun {
    /// Challenge-mode dungeon id.
    pub map_challenge_mode_id: i32,
    /// Keystone level.
    pub level: i32,
    /// True when the run was completed in time.
    pub completed: bool,
    /// Season the run belongs to.
    pub season: i32,
    /// Run score contribution.
    pub run_score: f64,
    /// Whether this run is from the current week.
    pub this_week: bool,
    /// Completion time in seconds.
    pub duration_sec: i32,
}

/// Backing state for `C_MythicPlus.*` probes.
#[derive(Debug, Clone)]
pub struct MythicPlusState {
    /// Active affixes for the current weekly rotation.
    pub current_affixes: Vec<MythicPlusAffix>,
    /// Current M+ season id (e.g. 14 = Dragonflight Season 4).
    pub current_season: i32,
    /// Keystone level of the key the player currently owns.
    /// 0 = no key.
    pub owned_keystone_level: i32,
    /// Run history for the player.
    pub run_history: Vec<MythicPlusRun>,
    /// Per-map weekly best, keyed by mapChallengeModeID.
    pub weekly_best_per_map: HashMap<i32, MythicPlusWeeklyBest>,
    /// Whether a Mythic+ run is currently in progress.
    pub is_active: bool,
    /// Whether the weekly Mythic+ reward is available to claim.
    pub is_weekly_reward_available: bool,
}

impl Default for MythicPlusState {
    fn default() -> Self {
        Self {
            current_affixes: vec![MythicPlusAffix {
                id: 9,
                season_id: 14,
            }],
            current_season: 14,
            owned_keystone_level: 0,
            run_history: Vec::new(),
            weekly_best_per_map: HashMap::new(),
            is_active: false,
            is_weekly_reward_available: false,
        }
    }
}

/// One killing-blow entry within a death recap event. Drives
/// `C_DeathRecap.GetKillingBlows`. The spell_id / ability_name /
/// caster_name / amount fields mirror the retail multiret documented
/// on Wowhead/WoWDB.
#[derive(Debug, Clone)]
pub struct KillingBlowInfo {
    /// Spell or ability ID responsible for the killing blow.
    pub spell_id: u32,
    /// Display name of the spell/ability (may be empty if unknown).
    pub ability_name: String,
    /// Name of the caster that delivered the blow.
    pub caster_name: String,
    /// Raw damage or healing amount for the killing hit.
    pub amount: i64,
    /// True when the hit was an overkill (damage > remaining health).
    pub is_overkill: bool,
}

/// One death event on the `SimState.death_recaps` list. Each entry
/// represents a single death and carries zero or more killing blows.
/// Drives `C_DeathRecap.GetKillingBlows` (blows for the most recent
/// entry) and `C_DeathRecap.GetMostRecentDeathRecap`.
#[derive(Debug, Clone)]
pub struct DeathRecapEntry {
    /// Internal recap id — mirrors the `recapID` concept used by
    /// `C_DeathRecap.GetRecapEvents`; 1-based.
    pub recap_id: u32,
    /// Zone / encounter name where the player died (informational).
    pub zone_name: String,
    /// Killing blows list (ordered from most damaging to least).
    pub killing_blows: Vec<KillingBlowInfo>,
}

/// Minimal area-POI metadata keyed by area poi id in
/// `SimState.area_pois`. Drives `C_AreaPoiInfo.GetAreaPOIInfo` and
/// `GetAreaPOISecondsLeft`. Only the subset of retail fields used by
/// Blizzard UI is carried; everything else is returned as nil /
/// default.
#[derive(Debug, Clone)]
pub struct AreaPoiInfo {
    pub area_poi_id: i32,
    pub name: String,
    /// UI-map id the POI is bound to. `None` when the POI is only
    /// looked up by id (the `uiMapID` arg to `GetAreaPOIInfo` is
    /// nilable too, so both sides can be unbound).
    pub ui_map_id: Option<i32>,
    /// Normalized 0..=1 screen position on the map.
    pub position: (f64, f64),
    pub atlas_name: Option<String>,
    pub description: Option<String>,
    pub faction_id: Option<i32>,
    pub icon_widget_set: Option<i32>,
    pub linked_ui_map_id: Option<i32>,
    pub is_current_event: bool,
    pub should_glow: bool,
    /// Seconds remaining until the POI expires. Drives
    /// `GetAreaPOISecondsLeft`. `None` for permanent POIs.
    pub seconds_left: Option<i32>,
}

/// Seeded achievement metadata keyed by achievement id in
/// `SimState.achievements`. Drives
/// `C_AchievementInfo.GetAchievementInfo`, `GetRewardItemID`, and
/// `IsValidAchievement`. Only the commonly-referenced few are
/// seeded; everything else returns `IsValidAchievement == false`.
///
/// `completed` / `was_earned_by_me` are derived from
/// `WorldState.earned_achievements` at read time, not stored on the
/// row, so `admin::mark_achievement_earned` flips the right bits
/// without having to touch this map.
#[derive(Debug, Clone)]
pub struct AchievementInfo {
    pub achievement_id: i32,
    pub name: String,
    pub points: i32,
    pub description: String,
    pub flags: i32,
    pub icon: i32,
    pub reward_text: String,
    pub is_guild: bool,
    pub is_statistic: bool,
    /// Item id rewarded when the achievement is earned; `None` when
    /// there is no item reward.
    pub reward_item_id: Option<i32>,
}

/// Per-map metadata keyed by ui-map id in `SimState.maps`. Drives
/// `C_Map.GetMapArtID`, `GetMapChildrenInfo`, and
/// `GetPlayerMapPosition`. Only the handful of ids commonly referenced
/// by Blizzard UI / addons are seeded (Azeroth world map, Eastern
/// Kingdoms continent, Stormwind City zone).
#[derive(Debug, Clone)]
pub struct MapData {
    pub ui_map_id: i32,
    pub name: String,
    /// `UIMapType` token (1=World, 2=Continent, 3=Zone, 4=Dungeon).
    pub map_type: i32,
    /// Parent map id in the hierarchy. `0` for the Cosmic root.
    pub parent_map_id: i32,
    /// Art tileset id returned by `GetMapArtID`. Non-zero for real
    /// zones; `0` for purely-logical maps (Cosmic).
    pub art_id: i32,
    /// `UIMapFlag` bitmask (0 = no flags).
    pub flags: i32,
    /// Direct children of this map in display order. Drives
    /// `GetMapChildrenInfo` (filtered by mapType when the caller
    /// passes one).
    pub child_map_ids: Vec<i32>,
}

/// Per-currency info keyed by currency id in `SimState.currency_info`.
/// Drives `C_CurrencyInfo.GetCurrencyInfo`, `GetCurrencyInfoFromLink`,
/// and `GetCurrencyContainerInfo`. Matches the 25-field retail struct;
/// most fields default to 0 / false for simplicity.
#[derive(Debug, Clone)]
pub struct CurrencyInfo {
    pub currency_id: i32,
    pub name: String,
    pub description: String,
    pub icon_file_id: u32,
    pub quantity: i32,
    pub max_quantity: i32,
    pub quality: i32,
    pub is_header: bool,
    pub is_header_expanded: bool,
    pub is_show_in_backpack: bool,
    pub discovered: bool,
    pub can_earn_per_week: bool,
    pub max_weekly_quantity: i32,
    pub quantity_earned_this_week: i32,
    pub is_account_transferable: bool,
    pub is_account_wide: bool,
    pub is_tradeable: bool,
    pub is_type_unused: bool,
    pub currency_list_depth: i32,
    pub recharging_amount_per_cycle: i32,
    pub recharging_cycle_duration_ms: i32,
    pub total_earned: i32,
    pub tracked_quantity: i32,
    pub transfer_percentage: Option<f64>,
    pub use_total_earned_for_max_qty: bool,
}

impl Default for CurrencyInfo {
    fn default() -> Self {
        Self {
            currency_id: 0,
            name: String::new(),
            description: String::new(),
            icon_file_id: 0,
            quantity: 0,
            max_quantity: 0,
            quality: 0,
            is_header: false,
            is_header_expanded: false,
            is_show_in_backpack: false,
            discovered: true,
            can_earn_per_week: false,
            max_weekly_quantity: 0,
            quantity_earned_this_week: 0,
            is_account_transferable: false,
            is_account_wide: false,
            is_tradeable: false,
            is_type_unused: false,
            currency_list_depth: 0,
            recharging_amount_per_cycle: 0,
            recharging_cycle_duration_ms: 0,
            total_earned: 0,
            tracked_quantity: 0,
            transfer_percentage: None,
            use_total_earned_for_max_qty: false,
        }
    }
}

/// A transmog appearance source (one way to obtain a visual appearance).
///
/// WoW's transmog system has three levels:
/// - **Visual**: the actual look (shared across items with identical models)
/// - **Source**: a specific item that grants a visual (e.g. "Heroic Garrosh's Helmet")
/// - **Category**: equipment slot grouping (Head=1, Shoulder=2, ..., MainHand=12, etc.)
#[derive(Debug, Clone)]
pub struct TransmogAppearance {
    pub source_id: i32,
    pub visual_id: i32,
    pub category_id: i32,
    pub item_id: i32,
    pub is_collected: bool,
    /// Source type from Enum.TransmogSource (Boss Drop=1, Quest=2, Vendor=3, etc.)
    pub source_type: i32,
    /// Item modification ID (difficulty variant: Normal=0, Heroic=1, Mythic=3, etc.)
    pub item_mod_id: i32,
}

/// A premade group listing in the Group Finder.
#[derive(Debug, Clone)]
pub struct PremadeListing {
    pub search_result_id: u32,
    pub name: String,
    pub comment: String,
    pub leader_name: String,
    pub activity_id: u32,
    pub num_members: i32,
    pub max_members: i32,
    pub voice_chat: bool,
    pub auto_accept: bool,
    pub is_delisted: bool,
}

/// An item attachment in a mail message.
#[derive(Debug, Clone)]
pub struct MailAttachment {
    pub item_id: u32,
    pub count: i32,
    pub quality: i32,
}

/// A mail message in the player's inbox.
#[derive(Debug, Clone)]
pub struct MailMessage {
    pub id: u64,
    pub sender: String,
    pub subject: String,
    pub body: String,
    pub money: u64,
    pub cod_amount: u64,
    pub items: Vec<MailAttachment>,
    pub days_left: f32,
    pub was_read: bool,
    pub was_returned: bool,
    pub can_reply: bool,
    pub is_gm: bool,
    pub stationery_icon: u32,
}

/// An item in a bag slot.
#[derive(Debug, Clone)]
pub struct BagItem {
    pub item_id: u32,
    pub stack_count: i32,
}

/// An equipped item in an inventory slot.
#[derive(Debug, Clone)]
pub struct EquippedItem {
    pub item_id: u32,
    pub enchant_id: u32,
    pub gem_ids: [u32; 3],
}

/// Computed character stats (base + gear).
#[derive(Debug, Clone, Default)]
pub struct CharacterStats {
    pub strength: f64,
    pub agility: f64,
    pub stamina: f64,
    pub intellect: f64,
    pub armor: i32,
    pub crit_rating: i32,
    pub haste_rating: i32,
    pub mastery_rating: i32,
    pub versatility_rating: i32,
    pub speed_rating: i32,
    pub avoidance_rating: i32,
    pub leech_rating: i32,
}

impl CharacterStats {
    /// Compute stats from base class stats + equipped item levels.
    /// Uses a simplified model: each equipped piece contributes stats proportional to ilvl.
    pub fn compute(equipped_items: &HashMap<i32, EquippedItem>, class_index: i32) -> Self {
        let mut stats = Self::base_stats();
        let total_ilvl = Self::total_equipped_ilvl(equipped_items);
        Self::apply_primary_stats(&mut stats, class_index, total_ilvl);
        Self::apply_secondary_and_armor(&mut stats, equipped_items, total_ilvl);
        stats
    }

    fn base_stats() -> Self {
        Self {
            strength: 120.0,
            agility: 100.0,
            stamina: 350.0,
            intellect: 100.0,
            armor: 1200,
            ..Self::default()
        }
    }

    fn total_equipped_ilvl(equipped_items: &HashMap<i32, EquippedItem>) -> f64 {
        equipped_items
            .values()
            .filter_map(|e| crate::items::get_item(e.item_id))
            .map(|item| item.item_level as f64)
            .sum()
    }

    /// Add primary stat from gear based on class (Str/Agi/Int).
    fn apply_primary_stats(stats: &mut Self, class_index: i32, total_ilvl: f64) {
        let primary_from_gear = total_ilvl * 1.2;
        let stamina_from_gear = total_ilvl * 1.8;
        match class_index {
            1 | 2 | 6 => stats.strength += primary_from_gear, // Warrior/Paladin/DK
            3 | 4 | 10 | 12 => stats.agility += primary_from_gear, // Hunter/Rogue/Monk/DH
            _ => stats.intellect += primary_from_gear,        // casters
        }
        stats.stamina += stamina_from_gear;
    }

    fn apply_secondary_and_armor(
        stats: &mut Self,
        equipped_items: &HashMap<i32, EquippedItem>,
        total_ilvl: f64,
    ) {
        let secondary = total_ilvl * 0.15;
        stats.crit_rating = secondary as i32;
        stats.haste_rating = (secondary * 0.9) as i32;
        stats.mastery_rating = (secondary * 1.1) as i32;
        stats.versatility_rating = (secondary * 0.7) as i32;
        let armor_slots = equipped_items
            .keys()
            .filter(|s| matches!(**s, 1 | 3 | 5 | 6 | 7 | 8 | 9 | 10))
            .count();
        stats.armor += (armor_slots as i32) * 4500;
    }

    pub fn crit_pct(&self) -> f64 {
        self.crit_rating as f64 / 180.0
    }
    pub fn haste_pct(&self) -> f64 {
        self.haste_rating as f64 / 170.0
    }
    pub fn mastery_pct(&self) -> f64 {
        self.mastery_rating as f64 / 130.0
    }
    pub fn versatility_pct(&self) -> f64 {
        self.versatility_rating as f64 / 205.0
    }
}

/// Player character state: identity, combat, power, health, buffs, spec.
#[derive(Debug, Clone, Copy, Default)]
pub struct SecondaryPowerState {
    pub current: i32,
    pub max: i32,
}

/// Player character state: identity, combat, power, health, buffs, spec.
#[derive(Debug, Clone)]
pub struct PlayerState {
    pub name: String,
    pub health: i32,
    pub health_max: i32,
    pub class_index: i32,
    pub race_index: usize,
    pub level: i32,
    pub sex: i32,
    pub power: i32,
    pub power_max: i32,
    pub power_type: i32,
    /// Secondary power pools keyed by Enum.PowerType value (e.g. Holy Power).
    pub secondary_powers: HashMap<i32, SecondaryPowerState>,
    pub in_combat: bool,
    pub is_resting: bool,
    pub money: i64,
    pub item_level: f32,
    pub equipped_items: HashMap<i32, EquippedItem>,
    pub stats: CharacterStats,
    pub pvp_enabled: bool,
    pub honor_level: i32,
    pub buffs: Vec<AuraInfo>,
    pub movement: MovementState,
    pub active_spec_index: i32,
    pub pending_spec_change: Option<i32>,
    /// Mail inbox.
    pub inbox: Vec<MailMessage>,
    /// Items attached to outgoing mail (12 slots max).
    pub send_mail_items: [Option<MailAttachment>; 12],
    /// Money attached to outgoing mail (copper).
    pub send_mail_money: u64,
    /// COD amount on outgoing mail (copper).
    pub send_mail_cod: u64,
    /// Counter for generating unique mail IDs.
    pub next_mail_id: u64,
    /// Current experience within the player's level. Drives `UnitXP("player")`.
    pub xp: i64,
    /// Experience required to ding the next level. Drives `UnitXPMax("player")`.
    pub xp_max: i64,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            name: String::new(),
            health: 100_000,
            health_max: 100_000,
            class_index: 2,
            race_index: 0,
            level: 70,
            sex: 2,
            power: 100,
            power_max: 100,
            power_type: 0,
            secondary_powers: HashMap::new(),
            in_combat: false,
            is_resting: false,
            money: 0,
            item_level: 615.0,
            equipped_items: default_equipped_items(),
            stats: CharacterStats::compute(&default_equipped_items(), 2),
            pvp_enabled: false,
            honor_level: 0,
            buffs: Vec::new(),
            movement: MovementState::default(),
            active_spec_index: 2,
            pending_spec_change: None,
            inbox: Vec::new(),
            send_mail_items: Default::default(),
            send_mail_money: 0,
            send_mail_cod: 0,
            next_mail_id: 1,
            xp: 0,
            xp_max: 180_000,
        }
    }
}

use super::state_defaults::*;

/// World/instance state: zone, guild, collections, vault, loot.
///
/// `Default::default()` produces a fully-empty / zero-valued state. The
/// sim's seeded defaults (Stormwind zone, "Heroes of Azeroth" guild,
/// populated collections) live in `seeded_world_state` and are applied
/// by `SimState::Default`.
/// A mirror timer (breath / exhaustion / feign death) currently active
/// on the player. Drives `GetMirrorTimerInfo` /
/// `GetMirrorTimerProgress`.
#[derive(Debug, Default, Clone)]
pub struct MirrorTimer {
    /// Timer name token ("BREATH", "EXHAUSTION", "FEIGNDEATH").
    pub name: String,
    /// Starting value when the timer was last reset.
    pub start_value: f64,
    /// Maximum value this timer counts toward.
    pub max_value: f64,
    /// Rate of change per second (negative = counting down).
    pub scale: f64,
    /// 0 when ticking, 1 when paused.
    pub paused: i32,
    /// Localised label shown on the HUD.
    pub label: String,
    /// Spell id associated with the timer (0 when none).
    pub spell_id: i32,
    /// Current progress reading. Drives `GetMirrorTimerProgress`.
    pub progress: f64,
}

#[derive(Debug, Default, Clone)]
pub struct WorldState {
    pub zone_name: String,
    pub zone_id: i32,
    pub sub_zone_name: String,
    pub instance_name: String,
    pub instance_type: String,
    pub instance_difficulty: i32,
    pub instance_max_players: i32,
    pub in_instance: bool,
    /// Difficulty display string (e.g. "Normal", "Heroic", "Mythic").
    /// Drives the 4th return of `GetInstanceInfo`. Empty outside instances.
    pub instance_difficulty_name: String,
    /// `dynamicDifficulty` from `GetInstanceInfo` (6th return). `0` when
    /// the instance does not have dynamic scaling.
    pub instance_dynamic_difficulty: i32,
    /// Whether the instance dynamically scales its difficulty. 7th return
    /// of `GetInstanceInfo`.
    pub instance_is_dynamic: bool,
    /// Instance map id (mapID). 8th return of `GetInstanceInfo`.
    pub instance_id: i32,
    /// Current group size inside the instance. 9th return of
    /// `GetInstanceInfo`. Usually matches `instance_max_players` or 0.
    pub instance_group_size: i32,
    /// LFG dungeon id for the current instance, if queued via the Group
    /// Finder. 10th return of `GetInstanceInfo` (nilable).
    pub instance_lfg_dungeon_id: Option<i32>,
    /// Mirror timers (underwater breath, exhaustion, feign death).
    /// `GetMirrorTimerInfo(index)` reads by 1-based index;
    /// `GetMirrorTimerProgress(name)` reads by name.
    pub mirror_timers: Vec<MirrorTimer>,
    /// Whether an encounter is currently in progress (boss pull active).
    /// Drives `IsEncounterInProgress`.
    pub encounter_in_progress: bool,
    /// Whether the current area allows flying. Drives `IsFlyableArea`.
    pub flyable_area: bool,
    /// Whether the current instance is an arena battleground. Drives
    /// `IsBattlefieldArena`.
    pub battlefield_arena: bool,
    pub guild_name: Option<String>,
    pub guild_rank: Option<String>,
    pub guild_num_members: i32,
    pub great_vault_activities: Vec<GreatVaultActivity>,
    pub great_vault_has_rewards: bool,
    pub great_vault_can_claim: bool,
    pub loot_rolls: HashMap<i32, LootRollInfo>,
    pub collected_transmogs: HashSet<i32>,
    pub transmog_appearances: Vec<TransmogAppearance>,
    /// Applied transmog per equipment slot: slotID → sourceID.
    pub applied_transmog_slots: HashMap<i32, i32>,
    pub collected_mounts: HashSet<i32>,
    pub mounts: Vec<MountData>,
    pub collected_pets: HashSet<i32>,
    pub pets: Vec<PetData>,
    pub collected_toys: HashSet<i32>,
    pub toys: Vec<ToyData>,
    pub favorite_toys: HashSet<u32>,
    pub heirlooms: Vec<HeirloomData>,
    pub collected_heirlooms: HashSet<u32>,
    pub earned_achievements: HashSet<i32>,
    pub premade_listings: Vec<PremadeListing>,
    /// Current zone's PvP type, returned by `C_PvP.GetZonePVPInfo()` as its
    /// first value. Canonical WoW tokens: `"contested"`, `"sanctuary"`,
    /// `"arena"`, `"friendly"`, `"hostile"`, `"combat"`. Default `"contested"`.
    pub pvp_type: String,
    /// Whether the current sub-zone applies its own PvP rules (e.g. a PvP
    /// district inside a contested zone). Returned as the second value.
    pub is_sub_zone_pvp: bool,
    /// For faction-locked zones, the faction whose PvP rules apply
    /// (`"Alliance"` / `"Horde"`); `None` on neutral zones. Third return.
    pub pvp_faction_name: Option<String>,
    /// Guild tabard crest — `GetGuildLogoInfo()` returns nine colour
    /// channels (background RGB, border RGB, emblem RGB) plus the emblem
    /// texture filename. All zeros + empty filename when no guild.
    pub guild_logo: GuildLogo,
    /// Guild ranks in display order (index 0 = rank 1). Empty when no
    /// guild. Drives `GuildControlGetNumRanks` / `GuildControlGetRankName` /
    /// `GuildControlGetRankFlags`.
    pub guild_ranks: Vec<GuildRank>,
    /// 1-based rank index currently "selected" by `GuildControlSetRank`.
    /// `GuildControlGetRankName()` / `GetRankFlags()` without an explicit
    /// index return the selected rank's fields. `0` = nothing selected.
    pub guild_selected_rank: i32,
    /// Club id exposed by `C_GuildInfo.GetClubId()`. WoW returns a string
    /// (Battle.net community id) or nil when the player has no guild.
    pub guild_club_id: Option<String>,
    /// Whether the player holds officer rank in their guild. Drives
    /// `C_GuildInfo.IsGuildOfficer()`. Default false (no guild).
    pub guild_is_officer: bool,
    /// Whether the player can speak in guild chat. `C_GuildInfo.CanSpeakInGuildChat()`.
    /// Default true — matches retail's "no explicit mute" baseline so addons
    /// that gate chat input on this probe don't silence themselves on startup.
    pub guild_can_speak_in_chat: bool,
    /// Guild Message of the Day. Empty string when not set.
    pub guild_motd: String,
    /// Guild members (names + 1-based rank indices). Populated by
    /// `GuildInvite` / `GuildUninvite` / `GuildKick` / `GuildPromote`.
    /// Empty when the player has no guild.
    pub guild_members: Vec<GuildMember>,
}

/// A guild member: display name + 1-based rank index. Rank 1 is highest
/// (Guild Master). Higher values = lower rank.
#[derive(Debug, Clone)]
pub struct GuildMember {
    pub name: String,
    pub rank_index: i32,
}

/// A macro slot. Matches the `/macro` addon view: name, icon, body text.
/// `icon` is the texture path passed to `EditMacro` (retail uses a mix
/// of texture ids and paths; the sim stores whatever the caller provides).
#[derive(Debug, Default, Clone)]
pub struct MacroInfo {
    pub name: String,
    pub icon: String,
    pub body: String,
}

/// A single guild rank row. `name` is the display name; `flags` is a bag of
/// arbitrary permission booleans — callers iterate and index by flag name
/// (WoW's real API returns a dense numeric-keyed table of flag values).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GuildRank {
    pub name: String,
    pub flags: Vec<bool>,
}

/// Guild tabard crest data returned by `GetGuildLogoInfo()`.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GuildLogo {
    pub background: (f64, f64, f64),
    pub border: (f64, f64, f64),
    pub emblem: (f64, f64, f64),
    pub emblem_filename: String,
}

/// Build the sim's seeded default `WorldState` — Stormwind zone,
/// "Heroes of Azeroth" guild, populated collections. `WorldState::default`
/// still returns a fully zero/empty state (the derived `Default`); this
/// function is what `SimState::Default` reaches for.
pub fn seeded_world_state() -> WorldState {
    let mut ws = WorldState {
        zone_name: "Stormwind City".into(),
        zone_id: 1519,
        sub_zone_name: "Trade District".into(),
        instance_type: "none".into(),
        instance_difficulty_name: String::new(),
        guild_name: Some("Heroes of Azeroth".into()),
        guild_rank: Some("Member".into()),
        guild_num_members: 150,
        pvp_type: "contested".into(),
        guild_can_speak_in_chat: true,
        ..WorldState::default()
    };
    apply_collection_defaults(&mut ws);
    ws
}

fn apply_collection_defaults(ws: &mut WorldState) {
    let heirlooms = default_heirlooms();
    ws.collected_heirlooms = heirlooms.iter().map(|h| h.item_id).collect();
    ws.heirlooms = heirlooms;
    ws.transmog_appearances = default_transmog_appearances();
    ws.mounts = default_mounts();
    ws.pets = default_pets();
    ws.toys = default_toys();
    ws.premade_listings = default_premade_listings();
}

/// Simulated player movement flags (all false = stationary).
#[derive(Debug, Clone, Default)]
pub struct MovementState {
    pub moving: bool,
    pub mounted: bool,
    pub flying: bool,
    pub falling: bool,
    pub swimming: bool,
}

/// An active loot roll (group loot item pending a player decision).
#[derive(Debug, Clone)]
pub struct LootRollInfo {
    /// Unique roll identifier.
    pub roll_id: i32,
    /// Duration in seconds for the roll timer.
    pub roll_time: f64,
    /// Item texture path/ID.
    pub texture: String,
    /// Item display name.
    pub name: String,
    /// Stack count.
    pub count: i32,
    /// Item quality (0=Poor..4=Epic).
    pub quality: i32,
    /// Whether the item binds on pickup.
    pub bind_on_pickup: bool,
    /// Whether need roll is allowed.
    pub can_need: bool,
    /// Whether greed roll is allowed.
    pub can_greed: bool,
    /// Whether disenchant roll is allowed.
    pub can_disenchant: bool,
    /// Disenchant required skill level.
    pub disenchant_level: i32,
    /// Item level.
    pub item_level: i32,
    /// Item link string.
    pub item_link: String,
}

/// One BattleNet friend entry. Drives `C_BattleNet.GetAccountInfoByGUID`,
/// `GetFriendAccountInfo`, `GetFriendNumAccounts`, and `GetNumFriends`.
/// The `game_accounts` vec holds all WoW (and other Blizzard game)
/// accounts attached to this BNet account that are currently online or
/// were recently seen.
#[derive(Debug, Clone)]
pub struct BnetFriend {
    /// 1-based index in the friends list (display order).
    pub friend_index: i32,
    /// Unique GUID for the BNet account (format: "BNet-0-<id>").
    pub bnet_account_guid: String,
    /// Numeric BNet account id (maps to `bnetAccountID` in the retail struct).
    pub bnet_account_id: i32,
    /// BattleTag display name (e.g. "Thrall#1234").
    pub battle_tag: String,
    /// Account name shown in the BNet friends list.
    pub account_name: String,
    /// Custom note set on this friend.
    pub note: String,
    /// Custom away message ("customMessage" in the retail struct).
    pub custom_message: String,
    /// Timestamp of the custom message (0 when not set).
    pub custom_message_time: i32,
    /// Whether this friend appears offline intentionally.
    pub appear_offline: bool,
    /// Whether this is a BattleTag friend (vs. RealID).
    pub is_battle_tag_friend: bool,
    /// Whether this entry represents a real friend vs. a pending request.
    pub is_friend: bool,
    /// Whether the friend is marked as a favorite.
    pub is_favorite: bool,
    /// Whether the friend has AFK status set.
    pub is_afk: bool,
    /// Whether the friend has DND status set.
    pub is_dnd: bool,
    /// Last-online timestamp (0 when currently online or unknown).
    pub last_online_time: i32,
    /// RAF link type (0 = none).
    pub raf_link_type: i32,
    /// Nested game-account list (one per Blizzard game logged in).
    pub game_accounts: Vec<BnetGameAccount>,
}

/// One Blizzard game account (WoW character, D3 account, etc.) attached
/// to a `BnetFriend`. Drives `C_BattleNet.GetGameAccountInfoByGUID`.
#[derive(Debug, Clone)]
pub struct BnetGameAccount {
    /// Unique GUID for this WoW/game account (format: "Player-<realm>-<id>").
    pub wow_account_guid: String,
    /// Numeric game account id (maps to `gameAccountID` in the retail struct).
    pub game_account_id: i32,
    /// Character name, or empty string when not in WoW.
    pub character_name: String,
    /// Realm name as shown in the UI.
    pub realm_name: String,
    /// Realm display name (may differ from `realm_name` in cross-realm contexts).
    pub realm_display_name: String,
    /// Numeric realm id.
    pub realm_id: i32,
    /// Character class id (1=Warrior … 13=Evoker); 0 when not applicable.
    pub class_id: i32,
    /// Class name string (e.g. "Paladin").
    pub class_name: String,
    /// Character level; 0 when not in WoW.
    pub character_level: i32,
    /// Current zone name.
    pub area_name: String,
    /// Whether this account is currently online.
    pub is_online: bool,
    /// Whether the game client is currently AFK.
    pub is_game_afk: bool,
    /// Whether the game client has DND set.
    pub is_game_busy: bool,
    /// Blizzard client program identifier (e.g. "WoW", "D3", "S2").
    pub client_program: String,
    /// Faction string: "Alliance", "Horde", or "" for neutral/non-WoW.
    pub faction_name: String,
    /// Race name string (e.g. "Human").
    pub race_name: String,
    /// Rich-presence text (game/activity description).
    pub rich_presence: String,
    /// Whether the player can be summoned.
    pub can_summon: bool,
    /// Whether this account is in the current region.
    pub is_in_current_region: bool,
    /// Whether this account has game focus.
    pub has_focus: bool,
    /// WoW project id (e.g. 1 for retail, 2 for classic).
    pub wow_project_id: i32,
    /// Timerunning season id (0 when not in a timerunning season).
    pub timerunning_season_id: i32,
    /// Numeric region id.
    pub region_id: i32,
    /// Player GUID string (character GUID, distinct from bnet/game account GUIDs).
    pub player_guid: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `WorldState::default()` returns a *fully empty* state (zero
    /// fields, empty collections). Seeded defaults come from
    /// `seeded_world_state()` — pin both contracts so callers that
    /// want "vanilla WoW-like" state reach for the seeded helper and
    /// anyone wiring a fresh sub-state can rely on Default being inert.
    #[test]
    fn world_default_is_empty_and_zeroed() {
        let world = WorldState::default();
        assert!(world.transmog_appearances.is_empty());
        assert!(world.heirlooms.is_empty());
        assert!(world.collected_heirlooms.is_empty());
        assert_eq!(world.zone_id, 0);
        assert!(world.zone_name.is_empty());
        assert!(world.guild_name.is_none());
        assert!(!world.guild_can_speak_in_chat);
    }

    #[test]
    fn seeded_world_populates_collections_and_seed_fields() {
        let world = seeded_world_state();
        assert!(!world.transmog_appearances.is_empty());
        assert!(!world.mounts.is_empty());
        assert!(!world.pets.is_empty());
        assert!(!world.toys.is_empty());
        assert!(!world.heirlooms.is_empty());
        assert!(!world.premade_listings.is_empty());
        assert_eq!(world.collected_heirlooms.len(), world.heirlooms.len());
        assert_eq!(world.zone_id, 1519);
        assert_eq!(world.pvp_type, "contested");
        assert!(world.guild_can_speak_in_chat);
    }

    #[test]
    fn transmog_default_appearances_populated() {
        let world = seeded_world_state();
        // 12 slots × 5 appearances each = 60
        assert_eq!(world.transmog_appearances.len(), 60);

        // Each armor slot has 4 collected + 1 uncollected
        let head: Vec<_> = world
            .transmog_appearances
            .iter()
            .filter(|a| a.category_id == 1)
            .collect();
        assert_eq!(head.len(), 5, "Head slot should have 5 appearances");
        assert_eq!(head.iter().filter(|a| a.is_collected).count(), 4);
        assert_eq!(head.iter().filter(|a| !a.is_collected).count(), 1);

        // Source IDs are unique and sequential
        let source_ids: HashSet<i32> = world
            .transmog_appearances
            .iter()
            .map(|a| a.source_id)
            .collect();
        assert_eq!(source_ids.len(), 60, "All source IDs should be unique");
    }

    #[test]
    fn heirloom_defaults_populated() {
        let world = seeded_world_state();
        assert_eq!(
            world.heirlooms.len(),
            11,
            "should have 11 default heirlooms"
        );
        assert_eq!(world.heirlooms[0].name, "Burnished Helm of Might");
        assert_eq!(world.heirlooms[0].equip_loc, "INVTYPE_HEAD");

        let ids: HashSet<u32> = world.heirlooms.iter().map(|h| h.item_id).collect();
        assert_eq!(ids.len(), 11, "all item IDs should be unique");
        assert_eq!(
            world.collected_heirlooms.len(),
            11,
            "all default heirlooms collected"
        );
        assert!(world.collected_heirlooms.contains(&122245));
    }
}
