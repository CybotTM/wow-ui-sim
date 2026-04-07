//! Plain data types used by SimState.

use crate::lua_api::game_data::AuraInfo;
use mlua::RegistryKey;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

/// What is currently held on the cursor (drag-and-drop state).
#[derive(Debug, Clone)]
pub enum CursorInfo {
    /// An action bar spell: PickupAction(slot) removes it from the bar.
    Action { slot: u32, spell_id: u32 },
    /// A spell from the spellbook (doesn't remove from spellbook).
    Spell { spell_id: u32 },
}

/// A pending timer callback.
pub struct PendingTimer {
    /// Unique timer ID.
    pub id: u64,
    /// When this timer should fire.
    pub fire_at: Instant,
    /// Lua function to call (stored in registry).
    pub callback_key: RegistryKey,
    /// For tickers: interval between firings.
    pub interval: Option<std::time::Duration>,
    /// For tickers with limited iterations: remaining count.
    pub remaining: Option<i32>,
    /// Whether this timer has been cancelled.
    pub cancelled: bool,
    /// The timer/ticker handle table (stored in registry) to pass to callback.
    pub handle_key: Option<RegistryKey>,
    /// Addon that created this timer (for profiler attribution).
    pub owner_addon: Option<u16>,
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
        }
    }
}

fn default_equipped_items() -> HashMap<i32, EquippedItem> {
    let e = |item_id| EquippedItem {
        item_id,
        enchant_id: 0,
        gem_ids: [0, 0, 0],
    };
    [
        (1, e(211993)),  // Head: Entombed Seraph's Casque
        (2, e(230637)),  // Neck: Astral Gladiator's Amulet
        (3, e(211991)),  // Shoulder: Entombed Seraph's Plumes
        (5, e(211996)),  // Chest: Entombed Seraph's Breastplate
        (6, e(211990)),  // Waist: Entombed Seraph's Waistguard
        (7, e(211992)),  // Legs: Entombed Seraph's Greaves
        (8, e(211995)),  // Feet: Entombed Seraph's Sabatons
        (9, e(211989)),  // Wrist: Entombed Seraph's Shackles
        (10, e(211994)), // Hands: Entombed Seraph's Castigation
        (11, e(225748)), // Ring1: Seal of the Silent Vigil
        (12, e(215135)), // Ring2: Ring of Earthen Craftsmanship
        (13, e(218715)), // Trinket1: Forged Gladiator's Emblem
        (14, e(236914)), // Trinket2: Unbound Vision Journal
        (15, e(211988)), // Back: Entombed Seraph's Greatcloak
        (16, e(229181)), // MainHand: Ordained Forge Maul
    ]
    .into_iter()
    .collect()
}

fn default_mounts() -> Vec<MountData> {
    let m = |mount_id, name: &str, spell_id, icon, collected, mount_type| MountData {
        mount_id,
        name: name.to_string(),
        spell_id,
        icon,
        is_collected: collected,
        is_usable: collected,
        mount_type,
    };
    vec![
        m(6,    "Brown Horse",                  458,   132261, true,  230),  // Ground
        m(18,   "Swift Palomino",               23338, 132261, true,  230),  // Ground
        m(69,   "Swift Stormsaber",             23338, 132226, true,  230),  // Ground
        m(107,  "Ashes of Al'ar",               40192, 132238, true,  248),  // Flying
        m(219,  "Invincible",                   72286, 132226, true,  248),  // Flying
        m(280,  "Traveler's Tundra Mammoth",    61425, 236241, true,  230),  // Ground (vendor)
        m(376,  "Sandstone Drake",              93326, 656576, true,  248),  // Flying (2-person)
        m(440,  "Grand Expedition Yak",         122708,646372, true,  230),  // Ground (vendor)
        m(678,  "Llothien Prowler",             171851,1394955,true,  230),  // Ground
        m(1039, "Mighty Caravan Brutosaur",     264058,2399241,false, 230),  // Ground (AH mount)
    ]
}

fn default_pets() -> Vec<PetData> {
    let mut id = 0u32;
    let p = |id: &mut u32, species: u32, name: &str, icon: u32, pet_type: i32, level: i32, quality: i32, collected: bool| {
        *id += 1;
        PetData {
            pet_id: format!("BattlePet-0-{:08X}", *id),
            species_id: species,
            name: name.to_string(),
            icon,
            pet_type,
            level,
            quality,
            is_collected: collected,
        }
    };
    vec![
        p(&mut id, 39,   "Mechanical Squirrel",   132932, 9, 25, 3, true),  // Mechanical
        p(&mut id, 87,   "Phoenix Hatchling",      132837, 3, 25, 3, true),  // Elemental
        p(&mut id, 68,   "Cat",                    132576, 7, 1,  1, true),  // Beast
        p(&mut id, 254,  "Lil' Ragnaros",          134153, 3, 25, 4, true),  // Elemental (legendary)
        p(&mut id, 1266, "Xu-Fu, Cub of Xuen",     648459, 7, 25, 4, true),  // Beast (legendary)
        p(&mut id, 630,  "Clockwork Gnome",        425954, 9, 25, 3, true),  // Mechanical
        p(&mut id, 846,  "Anubisath Idol",         607552, 10,25, 3, true),  // Humanoid
        p(&mut id, 40,   "Bombay Cat",             132576, 7, 1,  1, true),  // Beast
        p(&mut id, 1395, "Iron Starlette",         971374, 9, 25, 3, true),  // Mechanical
        p(&mut id, 2403, "Pocopoc",                4038816,9, 25, 4, false), // Mechanical (not collected)
    ]
}

fn default_toys() -> Vec<ToyData> {
    let t = |item_id, name: &str, icon, collected| ToyData {
        item_id,
        name: name.to_string(),
        icon,
        is_collected: collected,
        is_usable: collected,
    };
    vec![
        t(166779, "Hearthstone Game Table",       648323,  true),
        t(13379,  "Piccolo of the Flaming Fire",  134208,  true),
        t(34480,  "Romantic Picnic Basket",        236571,  true),
        t(33927,  "Brewfest Pony Keg",             132790,  true),
        t(119210, "Hearthstone Board",             1053079, true),
        t(69227,  "Foam Sword Rack",               318656,  true),
        t(88589,  "Gin-Ji Knife Set",              462768,  true),
        t(86575,  "Foxicopter Controller",         463485,  true),
        t(104324, "Foot Ball",                     620832,  true),
        t(187421, "Earpieces of Tranquil Focus",   4217589, false),
    ]
}

/// Default transmog appearances: ~5 per armor slot + weapon slots.
///
/// Category IDs from Enum.TransmogCollectionType:
///   Head=1, Shoulder=2, Back=3, Chest=4, Shirt=5, Tabard=6, Wrist=7,
///   Hands=8, Waist=9, Legs=10, Feet=11, OneHSword=14, Staff=23, Shield=18
fn default_transmog_appearances() -> Vec<TransmogAppearance> {
    let mut sid = 0i32; // auto-increment source_id
    let mut vid = 0i32; // auto-increment visual_id
    let a = |sid: &mut i32, vid: &mut i32, cat: i32, item: i32, collected: bool, src: i32| {
        *sid += 1;
        *vid += 1;
        TransmogAppearance {
            source_id: *sid,
            visual_id: *vid,
            category_id: cat,
            item_id: item,
            is_collected: collected,
            source_type: src,
            item_mod_id: 0,
        }
    };
    // Source types: JournalEncounter=1, Quest=2, Vendor=3, WorldDrop=4
    vec![
        // Head (cat 1)
        a(&mut sid, &mut vid, 1, 31110, true,  1), // Helm of Wrath
        a(&mut sid, &mut vid, 1, 34333, true,  1), // Onslaught Greathelm
        a(&mut sid, &mut vid, 1, 44006, true,  2), // Titan-forged Plate Helm
        a(&mut sid, &mut vid, 1, 77154, true,  1), // Colossal Dragonplate Helmet
        a(&mut sid, &mut vid, 1, 99999, false, 4), // Uncollected head piece
        // Shoulder (cat 2)
        a(&mut sid, &mut vid, 2, 30055, true,  1), // Pauldrons of the Fallen Champion
        a(&mut sid, &mut vid, 2, 34392, true,  1), // Onslaught Shoulderblades
        a(&mut sid, &mut vid, 2, 50853, true,  1), // Boneguard Commander's Pauldrons
        a(&mut sid, &mut vid, 2, 77166, true,  1), // Backbreaker Spaulders
        a(&mut sid, &mut vid, 2, 99998, false, 3), // Uncollected shoulders
        // Back (cat 3)
        a(&mut sid, &mut vid, 3, 27878, true,  1), // Cloak of the Pit Stalker
        a(&mut sid, &mut vid, 3, 34241, true,  1), // Cloak of Unforgivable Sin
        a(&mut sid, &mut vid, 3, 65108, true,  2), // Wrap of Unity
        a(&mut sid, &mut vid, 3, 77098, true,  4), // Cape of Vaulted Secrets
        a(&mut sid, &mut vid, 3, 99997, false, 4), // Uncollected cloak
        // Chest (cat 4)
        a(&mut sid, &mut vid, 4, 30129, true,  1), // Breastplate of the Lightbinder
        a(&mut sid, &mut vid, 4, 34215, true,  1), // Onslaught Breastplate
        a(&mut sid, &mut vid, 4, 50078, true,  1), // Thassarian's Battleplate
        a(&mut sid, &mut vid, 4, 77171, true,  1), // Colossal Dragonplate Battleplate
        a(&mut sid, &mut vid, 4, 99996, false, 3), // Uncollected chest
        // Wrist (cat 7)
        a(&mut sid, &mut vid, 7, 30067, true,  1), // Bracers of Maliciousness
        a(&mut sid, &mut vid, 7, 34431, true,  1), // Wristguards of Tranquil Thought
        a(&mut sid, &mut vid, 7, 50611, true,  1), // Bracers of Dark Reckoning
        a(&mut sid, &mut vid, 7, 77162, true,  4), // Dragonbone Wargreaves
        a(&mut sid, &mut vid, 7, 99995, false, 4), // Uncollected bracers
        // Hands (cat 8)
        a(&mut sid, &mut vid, 8, 30113, true,  1), // Gloves of the Fallen Defender
        a(&mut sid, &mut vid, 8, 34342, true,  1), // Handguards of the Dawn
        a(&mut sid, &mut vid, 8, 50610, true,  1), // Gauntlets of Overexposure
        a(&mut sid, &mut vid, 8, 77160, true,  1), // Colossal Dragonplate Gauntlets
        a(&mut sid, &mut vid, 8, 99994, false, 2), // Uncollected gloves
        // Waist (cat 9)
        a(&mut sid, &mut vid, 9, 30034, true,  1), // Belt of One-Hundred Deaths
        a(&mut sid, &mut vid, 9, 34485, true,  1), // Girdle of the Fearless
        a(&mut sid, &mut vid, 9, 50069, true,  1), // Belt of Broken Bones
        a(&mut sid, &mut vid, 9, 77164, true,  4), // Gorge Stalker Belt
        a(&mut sid, &mut vid, 9, 99993, false, 3), // Uncollected belt
        // Legs (cat 10)
        a(&mut sid, &mut vid, 10, 30126, true,  1), // Leggings of the Fallen
        a(&mut sid, &mut vid, 10, 34180, true,  1), // Onslaught Legguards
        a(&mut sid, &mut vid, 10, 50063, true,  1), // Legguards of Lost Hope
        a(&mut sid, &mut vid, 10, 77155, true,  1), // Colossal Dragonplate Legguards
        a(&mut sid, &mut vid, 10, 99992, false, 2), // Uncollected legs
        // Feet (cat 11)
        a(&mut sid, &mut vid, 11, 30032, true,  1), // Red Riding Boots
        a(&mut sid, &mut vid, 11, 34567, true,  1), // Boots of the Protector
        a(&mut sid, &mut vid, 11, 50607, true,  1), // Treads of Impending Resurrection
        a(&mut sid, &mut vid, 11, 77159, true,  4), // Mosshide Treads
        a(&mut sid, &mut vid, 11, 99991, false, 4), // Uncollected boots
        // One-Hand Sword (cat 14)
        a(&mut sid, &mut vid, 14, 28573, true,  1), // Despair
        a(&mut sid, &mut vid, 14, 34247, true,  1), // Apolyon, the Soul-Render
        a(&mut sid, &mut vid, 14, 50070, true,  1), // Glorenzelg, High-Blade of the Silver Hand
        a(&mut sid, &mut vid, 14, 77188, true,  1), // Gurthalak, Voice of the Deeps
        a(&mut sid, &mut vid, 14, 99990, false, 1), // Uncollected sword
        // Staff (cat 23)
        a(&mut sid, &mut vid, 23, 30723, true,  1), // Staff of Infinite Mysteries
        a(&mut sid, &mut vid, 23, 34182, true,  1), // Grand Magister's Staff of Torrents
        a(&mut sid, &mut vid, 23, 50731, true,  4), // Archus, Greatstaff of Antonidas
        a(&mut sid, &mut vid, 23, 77196, true,  1), // Ti'tahk, the Steps of Time
        a(&mut sid, &mut vid, 23, 99989, false, 4), // Uncollected staff
        // Shield (cat 18)
        a(&mut sid, &mut vid, 18, 28606, true,  1), // Shield of Impenetrable Darkness
        a(&mut sid, &mut vid, 18, 34185, true,  1), // Sword Breaker's Bulwark
        a(&mut sid, &mut vid, 18, 50729, true,  1), // Icecrown Glacial Wall
        a(&mut sid, &mut vid, 18, 77167, true,  4), // Blackhorn's Mighty Bulwark
        a(&mut sid, &mut vid, 18, 99988, false, 3), // Uncollected shield
    ]
}

fn default_heirlooms() -> Vec<HeirloomData> {
    let h = |id, name: &str, loc: &str, icon, lvl, src: &str, min, max| HeirloomData {
        item_id: id,
        name: name.into(),
        equip_loc: loc.into(),
        icon,
        upgrade_level: lvl,
        source: src.into(),
        min_level: min,
        max_level: max,
    };
    vec![
        h(122245, "Burnished Helm of Might",               "INVTYPE_HEAD",     133071, 6, "Vendor", 1, 50),
        h(122355, "Polished Breastplate of Valor",          "INVTYPE_CHEST",    132633, 6, "Vendor", 1, 50),
        h(122356, "Polished Spaulders of Valor",            "INVTYPE_SHOULDER", 132633, 6, "Vendor", 1, 50),
        h(122382, "Preened Ironfeather Shoulders",          "INVTYPE_SHOULDER", 135033, 6, "Vendor", 1, 50),
        h(122384, "Tattered Dreadmist Robe",                "INVTYPE_CHEST",    132673, 6, "Vendor", 1, 50),
        h(122247, "Burnished Legplates of Might",           "INVTYPE_LEGS",     133071, 6, "Vendor", 1, 50),
        h(122250, "Mystical Kilt of Elements",              "INVTYPE_LEGS",     134188, 6, "Vendor", 1, 50),
        h(122266, "Balanced Heartseeker",                   "INVTYPE_WEAPON",   135274, 6, "Vendor", 1, 50),
        h(122389, "Bloodied Arcanite Reaper",               "INVTYPE_2HWEAPON", 135277, 6, "Vendor", 1, 50),
        h(122390, "Dignified Headmaster's Charge",          "INVTYPE_2HWEAPON", 135146, 6, "Vendor", 1, 50),
        h(187997, "Eternal Amulet of the Redeemed",         "INVTYPE_NECK",     133280, 0, "Vendor", 1, 50),
    ]
}

fn default_premade_listings() -> Vec<PremadeListing> {
    let mut id = 0u32;
    let l = |id: &mut u32, name: &str, comment: &str, leader: &str, activity: u32, num: i32, max: i32| {
        *id += 1;
        PremadeListing {
            search_result_id: *id,
            name: name.to_string(),
            comment: comment.to_string(),
            leader_name: leader.to_string(),
            activity_id: activity,
            num_members: num,
            max_members: max,
            voice_chat: false,
            auto_accept: false,
            is_delisted: false,
        }
    };
    vec![
        l(&mut id, "+15 Mists chill run",     "Know mechanics, 2.5k io",  "Thrallx",    1195, 3, 5),
        l(&mut id, "+12 Siege weekly",         "Weekly key, all welcome",  "Jainavx",    1188, 2, 5),
        l(&mut id, "Nerub-ar Palace HC fresh", "AOTC prog, be geared",    "Anduin",     1296, 12,20),
        l(&mut id, "Nerub-ar Palace N learn",  "Learning run, patient",   "Sylvanas",   1295, 8, 20),
        l(&mut id, "World Boss — Aggregation", "Quick kill, summon up",   "Khadgar",    1350, 18,40),
        l(&mut id, "2v2 Arena chill",          "Just capping",            "Garrosh",    491,  1, 2),
        l(&mut id, "RBG yolo",                 "Casual RBG, no rage",     "Velen",      493,  7, 10),
        l(&mut id, "WQ group Ringing Deeps",   "Doing WQs together",      "Malfurion",  0,    3, 5),
    ]
}

/// World/instance state: zone, guild, collections, vault, loot.
#[derive(Debug, Clone)]
pub struct WorldState {
    pub zone_name: String,
    pub zone_id: i32,
    pub sub_zone_name: String,
    pub instance_name: String,
    pub instance_type: String,
    pub instance_difficulty: i32,
    pub instance_max_players: i32,
    pub in_instance: bool,
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
    pub heirlooms: Vec<HeirloomData>,
    pub collected_heirlooms: HashSet<u32>,
    pub earned_achievements: HashSet<i32>,
    pub premade_listings: Vec<PremadeListing>,
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            zone_name: "Stormwind City".into(),
            zone_id: 1519,
            sub_zone_name: "Trade District".into(),
            instance_name: String::new(),
            instance_type: "none".into(),
            instance_difficulty: 0,
            instance_max_players: 0,
            in_instance: false,
            guild_name: Some("Heroes of Azeroth".into()),
            guild_rank: Some("Member".into()),
            guild_num_members: 150,
            great_vault_activities: Vec::new(),
            great_vault_has_rewards: false,
            great_vault_can_claim: false,
            loot_rolls: HashMap::new(),
            collected_transmogs: HashSet::new(),
            transmog_appearances: default_transmog_appearances(),
            applied_transmog_slots: HashMap::new(),
            collected_mounts: HashSet::new(),
            mounts: default_mounts(),
            collected_pets: HashSet::new(),
            pets: default_pets(),
            collected_toys: HashSet::new(),
            toys: default_toys(),
            heirlooms: default_heirlooms(),
            collected_heirlooms: default_heirlooms().iter().map(|h| h.item_id).collect(),
            earned_achievements: HashSet::new(),
            premade_listings: default_premade_listings(),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transmog_default_appearances_populated() {
        let world = WorldState::default();
        // 12 slots × 5 appearances each = 60
        assert_eq!(world.transmog_appearances.len(), 60);

        // Each armor slot has 4 collected + 1 uncollected
        let head: Vec<_> = world.transmog_appearances.iter().filter(|a| a.category_id == 1).collect();
        assert_eq!(head.len(), 5, "Head slot should have 5 appearances");
        assert_eq!(head.iter().filter(|a| a.is_collected).count(), 4);
        assert_eq!(head.iter().filter(|a| !a.is_collected).count(), 1);

        // Source IDs are unique and sequential
        let source_ids: HashSet<i32> = world.transmog_appearances.iter().map(|a| a.source_id).collect();
        assert_eq!(source_ids.len(), 60, "All source IDs should be unique");
    }

    #[test]
    fn heirloom_defaults_populated() {
        let world = WorldState::default();
        assert_eq!(world.heirlooms.len(), 11, "should have 11 default heirlooms");
        assert_eq!(world.heirlooms[0].name, "Burnished Helm of Might");
        assert_eq!(world.heirlooms[0].equip_loc, "INVTYPE_HEAD");

        let ids: HashSet<u32> = world.heirlooms.iter().map(|h| h.item_id).collect();
        assert_eq!(ids.len(), 11, "all item IDs should be unique");
        assert_eq!(world.collected_heirlooms.len(), 11, "all default heirlooms collected");
        assert!(world.collected_heirlooms.contains(&122245));
    }
}
