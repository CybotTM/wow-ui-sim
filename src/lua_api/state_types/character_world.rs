//! Character stats, player state, world/instance state, and related helpers.

use std::collections::{HashMap, HashSet};

use crate::lua_api::game_data::AuraInfo;

use super::collections::{
    EquippedItem, GreatVaultActivity, HeirloomData, MailAttachment, MailMessage, MountData,
    PetData, PremadeListing, ToyData, TransmogAppearance, WarbandSceneData,
};

use super::super::state_defaults::*;

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

/// One map entry in a player's Mythic+ rating summary.
/// Mirrors `C_PlayerInfo.MythicPlusRatingMapSummary`.
#[derive(Debug, Clone)]
pub struct MythicPlusRatingMapSummary {
    pub challenge_mode_id: i32,
    pub map_score: f64,
    pub best_run_level: i32,
    pub best_run_duration_ms: i64,
    pub finished_success: bool,
}

/// Overall Mythic+ rating summary for the player.
/// Mirrors `C_PlayerInfo.MythicPlusRatingSummary`.
#[derive(Debug, Clone)]
pub struct MythicPlusRatingSummary {
    pub current_season_score: f64,
    pub runs: Vec<MythicPlusRatingMapSummary>,
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

/// Player character state: identity, combat, power, health, buffs, spec.
///
/// `Default` is derived: every field is its zero/empty value. The seeded
/// "level 70 paladin" preset lives in [`PlayerState::seeded`].
#[derive(Debug, Clone, Default)]
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
    /// True when the player is currently in their alternate form (e.g. worgen, druid).
    pub is_alternate_form: bool,
    /// True when the alternate form is the default/native form.
    pub alternate_form_is_default: bool,
    /// True when the player is eligible for the New Player Experience.
    pub is_npe_eligible: bool,
    /// True when the player is restricted by NPE (starter zone limits).
    pub is_npe_restricted: bool,
    /// True when the player is currently in the Returning Player Experience.
    pub is_in_rpe: bool,
    /// Mythic+ rating summary for the player. None = no rating data.
    pub mythic_plus_rating_summary: Option<MythicPlusRatingSummary>,
}

impl PlayerState {
    /// Seeded "level 70 paladin" preset used to bootstrap a fresh `Sim`.
    /// All other fields keep their `Default`-derived zero/empty values.
    pub fn seeded() -> Self {
        let equipped_items = default_equipped_items();
        let stats = CharacterStats::compute(&equipped_items, 2);
        Self {
            health: 100_000,
            health_max: 100_000,
            class_index: 2,
            level: 70,
            sex: 2,
            power: 100,
            power_max: 100,
            item_level: 615.0,
            equipped_items,
            stats,
            active_spec_index: 2,
            next_mail_id: 1,
            xp_max: 180_000,
            alternate_form_is_default: false,
            ..Self::default()
        }
    }
}

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

/// World/instance state: zone, guild, collections, vault, loot.
///
/// `Default::default()` produces a fully-empty / zero-valued state. The
/// sim's seeded defaults (Stormwind zone, "Heroes of Azeroth" guild,
/// populated collections) live in `seeded_world_state` and are applied
/// by `SimState::Default`.
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
    pub warband_scenes: Vec<WarbandSceneData>,
    pub favorite_toys: HashSet<u32>,
    pub heirlooms: Vec<HeirloomData>,
    pub collected_heirlooms: HashSet<u32>,
    pub earned_achievements: HashSet<i32>,
    pub premade_listings: Vec<PremadeListing>,
    /// Seeded PvP battleground info returned by `C_PvP.GetWorldPVPAreaInfo()`.
    pub world_pvp_areas: Vec<WorldPvpBattlegroundInfo>,
    /// Seeded holiday battleground info returned by `C_PvP.GetHolidayBGInfo()`.
    pub holiday_bg_info: Option<RandomBGInfo>,
    /// Player-managed battleground locklist returned by `C_PvP.GetLocklistMap()`.
    pub locklist_maps: Vec<u32>,
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
    /// Guild members (names + 1-based rank indices + online state). Populated by
    /// `GuildInvite` / `GuildUninvite` / `GuildKick` / `GuildPromote`.
    /// Empty when the player has no guild.
    pub guild_members: Vec<GuildMember>,
}

/// A guild member: display name, 1-based rank index, and online state.
/// Rank 1 is highest (Guild Master). Higher values = lower rank.
#[derive(Debug, Clone)]
pub struct GuildMember {
    pub name: String,
    pub rank_index: i32,
    pub online: bool,
}

/// Seeded `C_PvP.GetWorldPVPAreaInfo()` row.
#[derive(Debug, Clone, Default)]
pub struct WorldPvpBattlegroundInfo {
    pub bg_id: i32,
    pub can_enter: bool,
    pub can_queue: bool,
    pub is_active: bool,
    pub max_level: i32,
    pub min_level: i32,
    pub name: String,
    pub start_time: i32,
}

/// Seeded `C_PvP.GetHolidayBGInfo()` row.
#[derive(Debug, Clone, Default)]
pub struct RandomBGInfo {
    pub bg_id: i32,
    pub bg_index: i32,
    pub can_queue: bool,
    pub has_random_win_today: bool,
    pub max_level: i32,
    pub min_level: i32,
    pub name: String,
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
        guild_num_members: 2,
        guild_members: default_guild_members(),
        pvp_type: "contested".into(),
        guild_can_speak_in_chat: true,
        world_pvp_areas: default_world_pvp_areas(),
        holiday_bg_info: Some(default_holiday_bg_info()),
        ..WorldState::default()
    };
    apply_collection_defaults(&mut ws);
    ws
}

fn default_guild_members() -> Vec<GuildMember> {
    vec![
        GuildMember {
            name: "Uther".into(),
            rank_index: 1,
            online: true,
        },
        GuildMember {
            name: "Jaina".into(),
            rank_index: 2,
            online: false,
        },
    ]
}

fn default_world_pvp_areas() -> Vec<WorldPvpBattlegroundInfo> {
    vec![
        WorldPvpBattlegroundInfo {
            bg_id: 571,
            can_enter: true,
            can_queue: true,
            is_active: true,
            max_level: 80,
            min_level: 80,
            name: "Wintergrasp".into(),
            start_time: 900,
        },
        WorldPvpBattlegroundInfo {
            bg_id: 607,
            can_enter: false,
            can_queue: false,
            is_active: false,
            max_level: 85,
            min_level: 80,
            name: "Tol Barad".into(),
            start_time: 0,
        },
    ]
}

fn default_holiday_bg_info() -> RandomBGInfo {
    RandomBGInfo {
        bg_id: 108,
        bg_index: 2,
        can_queue: true,
        has_random_win_today: false,
        max_level: 80,
        min_level: 10,
        name: "Warsong Scramble".into(),
    }
}

fn apply_collection_defaults(ws: &mut WorldState) {
    let heirlooms = default_heirlooms();
    ws.collected_heirlooms = heirlooms.iter().map(|h| h.item_id).collect();
    ws.heirlooms = heirlooms;
    ws.transmog_appearances = default_transmog_appearances();
    ws.mounts = default_mounts();
    ws.pets = default_pets();
    ws.toys = default_toys();
    ws.warband_scenes = default_warband_scenes();
    ws.premade_listings = default_premade_listings();
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
        assert!(!world.warband_scenes.is_empty());
        assert!(!world.heirlooms.is_empty());
        assert!(!world.premade_listings.is_empty());
        assert_eq!(world.collected_heirlooms.len(), world.heirlooms.len());
        assert_eq!(world.zone_id, 1519);
        assert_eq!(world.pvp_type, "contested");
        assert!(world.guild_can_speak_in_chat);
        assert_eq!(world.world_pvp_areas.len(), 2);
        assert_eq!(world.world_pvp_areas[0].name, "Wintergrasp");
        assert_eq!(world.world_pvp_areas[1].name, "Tol Barad");
        assert_eq!(
            world
                .holiday_bg_info
                .as_ref()
                .expect("holiday bg info should be seeded")
                .name,
            "Warsong Scramble"
        );
        assert!(world.locklist_maps.is_empty());
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
