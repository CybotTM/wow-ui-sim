//! Shared state types for the WoW Lua API.

use crate::cvars::CVarStorage;
use crate::event::{EventQueue, ScriptRegistry};
use crate::lua_api::animation::AnimGroupState;
use crate::lua_api::message_frame::MessageFrameData;
use crate::lua_api::simple_html::SimpleHtmlData;
use crate::lua_api::tooltip::{TooltipData, build_cursor_anchor};
use crate::screen::ScreenKind;
use crate::sound::SoundManager;
use crate::widget::{Anchor, WidgetRegistry};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::path::PathBuf;
use std::time::Instant;

macro_rules! build_empty_sim_state {
    ($collections:ident, $runtime:ident) => {
        Self {
            widgets: WidgetRegistry::default(),
            events: EventQueue::default(),
            scripts: ScriptRegistry::default(),
            cvars: CVarStorage::new(),
            console_output: $collections.console_output,
            timers: $collections.timers,
            rilua_timers: ::std::collections::VecDeque::new(),
            focused_frame_id: $runtime.focused_frame_id,
            addons: $collections.addons,
            addon_saved_enable_state: None,
            system_chat_log: Vec::new(),
            tooltips: $collections.tooltips,
            blocked_auras_by_unit: $collections.blocked_auras_by_unit,
            quest_blobs: $collections.quest_blobs,
            fog_of_war_frames: $collections.fog_of_war_frames,
            unit_position_frames: $collections.unit_position_frames,
            pending_player_reports: $collections.pending_player_reports,
            simple_htmls: $collections.simple_htmls,
            message_frames: $collections.message_frames,
            on_update_frames: $collections.on_update_frames,
            visible_on_update_cache: $runtime.visible_on_update_cache,
            strata_buckets: $runtime.strata_buckets,
            pending_hit_grid_changes: $collections.pending_hit_grid_changes,
            pending_texture_preloads: $collections.pending_texture_preloads,
            animation_groups: $collections.animation_groups,
            next_anim_group_id: $runtime.next_anim_group_id,
            anim_frame_to_group: $collections.anim_frame_to_group,
            anim_frame_to_anim: $collections.anim_frame_to_anim,
            screen_width: $runtime.screen_width,
            screen_height: $runtime.screen_height,
            screen_kind: $runtime.screen_kind,
            is_logged_in: $runtime.is_logged_in,
            screen_first_displayed: $runtime.screen_first_displayed,
            saved_account_name: $runtime.saved_account_name,
            saved_account_list: $runtime.saved_account_list,
            uses_token: $runtime.uses_token,
            account_save_enabled: $runtime.account_save_enabled,
            account_save_in_progress: $runtime.account_save_in_progress,
            account_locked_post_save: $runtime.account_locked_post_save,
            last_account_store_purchase_request: $runtime.last_account_store_purchase_request,
            account_store_begin_purchase_succeeds: $runtime.account_store_begin_purchase_succeeds,
            last_account_store_refund_request: $runtime.last_account_store_refund_request,
            account_store_refund_succeeds: $runtime.account_store_refund_succeeds,
            account_store_category_items: $collections.account_store_category_items,
            account_store_currency_for_store: $collections.account_store_currency_for_store,
            account_store_currency_info: $collections.account_store_currency_info,
            account_store_storefront_state: $collections.account_store_storefront_state,
            last_account_store_storefront_info_request: $runtime
                .last_account_store_storefront_info_request,
            account_store_categories: $collections.account_store_categories,
            account_store_items: $collections.account_store_items,
            action_bar_state: ActionBarStateInfo::default(),
            action_bars: $collections.action_bars,
            action_highlights: ActionHighlightState::default(),
            equipped_artifact: None,
            artifact_point_costs: HashMap::new(),
            azerite_item: None,
            major_factions: HashMap::new(),
            major_faction_renown_levels: HashMap::new(),
            account_wide_reputation_factions: HashSet::new(),
            faction_paragon: HashMap::new(),
            transmog_outfit_locks: HashSet::new(),
            equipped_outfit_locked: false,
            locked_action_slots: HashSet::new(),
            is_active_battlefield: false,
            spell_trade_skill_links: HashMap::new(),
            spell_id_aliases: HashMap::new(),
            spell_loss_of_control: HashMap::new(),
            action_profession_quality: HashMap::new(),
            addon_base_paths: $collections.addon_base_paths,
            create_frame_initial_hidden: $runtime.create_frame_initial_hidden,
            suppress_runtime_on_load_depth: $runtime.suppress_runtime_on_load_depth,
            mouse_position: $runtime.mouse_position,
            hovered_frame: $runtime.hovered_frame,
            active_drag_frame: $runtime.active_drag_frame,
            active_slider_thumb_drag_frame: $runtime.active_slider_thumb_drag_frame,
            mouse_buttons: $runtime.mouse_buttons,
            next_report_token: $runtime.next_report_token,
            party_members: $collections.party_members,
            party_group_active: $runtime.party_group_active,
            current_target: $runtime.current_target,
            previous_target: None,
            current_focus: $runtime.current_focus,
            enemy_pool: Vec::new(),
            sound_manager: $runtime.sound_manager,
            last_sound_kit_requested: $runtime.last_sound_kit_requested,
            last_sound_file_requested: $runtime.last_sound_file_requested,
            last_stopped_sound_handle: $runtime.last_stopped_sound_handle,
            last_launched_url: $runtime.last_launched_url,
            highlighted_map_scene_character_guid: $runtime.highlighted_map_scene_character_guid,
            multi_action_bar_grids_shown: $runtime.multi_action_bar_grids_shown,
            secure_attribute_drivers: $collections.secure_attribute_drivers,
            rot_damage_level: $runtime.rot_damage_level,
            fps: $runtime.fps,
            start_time: $runtime.start_time,
            casting: $runtime.casting,
            channeling: $runtime.channeling,
            next_cast_id: $runtime.next_cast_id,
            gcd: $runtime.gcd,
            spell_cooldowns: $collections.spell_cooldowns,
            inventory_item_cooldowns: ::std::collections::HashMap::new(),
            action_ui_buttons: $collections.action_ui_buttons,
            cursor_item: $runtime.cursor_item,
            loading_addon_index: $runtime.loading_addon_index,
            loading_addon_stack: $runtime.loading_addon_stack,
            executing_addon_index: $runtime.executing_addon_index,
            xml_load_addon_depth: $runtime.xml_load_addon_depth,
            loading_forbidden: $runtime.loading_forbidden,
            app_frame_metrics: AppFrameMetrics::default(),
            talents: super::talent_state::TalentState::new(),
            lua_errors: $collections.lua_errors,
            lua_error_records: $collections.lua_error_records,
            lua_error_counts: $collections.lua_error_counts,
            nil_symbol_accesses: $collections.nil_symbol_accesses,
            global_show_hide_depth: 0,
            anim_sync_times: $collections.anim_sync_times,
            player: PlayerState::seeded(),
            player_xp: PlayerXpState::default(),
            world: super::state_types::seeded_world_state(),
            bag_items: $collections.bag_items,
            tracked_recipes: $collections.tracked_recipes,
            crafting: CraftingState::default(),
            net_stats: NetStats::default(),
            store_frame_shown: false,
            timerunning_season_id: None,
            modifier_keys: ModifierKeys::default(),
            game_rules: GameRulesState::default(),
            housing_service_enabled: true,
            housing: HousingState::default(),
            pet_battles: PetBattleState::default(),
            pet: PetState::default(),
            lfg_list_counts: LfgListCounts::default(),
            can_use_premade_group: false,
            lfg_category_info: default_lfg_category_info(),
            lfg_active_categories: ::std::collections::HashSet::new(),
            photo_sharing_authorized: false,
            photo_sharing_enabled: false,
            tutorial_flags: $collections.tutorial_flags,
            wowlabs: WowLabsState::default(),
            quest_log: Vec::new(),
            quest_log_entries: QuestLogState::seeded(),
            pending_quest_offer: None,
            quest_choice_id: None,
            quest_poi_map_id: None,
            selected_quest_log_id: None,
            abandon_quest_id: None,
            tracked_achievements: ::std::collections::HashSet::new(),
            bank_frame_open: false,
            guild_bank_frame_open: false,
            merchant_frame_open: false,
            tabard_frame_open: false,
            trainer_frame_open: false,
            socket_frame_open: false,
            loot_frame_open: false,
            guild_registrar_open: false,
            pet_stables_open: false,
            merchant_items: Vec::new(),
            loot_slots: Vec::new(),
            auction_browse_items: Vec::new(),
            loot_method: LootMethodState::default(),
            gossip: GossipState::default(),
            torghast: TorghastState::default(),
            titles: Vec::new(),
            shapeshift_forms: Vec::new(),
            shapeshift_cooldowns: ::std::collections::HashMap::new(),
            pet_actions: vec![PetActionSlot::default(); 10],
            glyph: GlyphState::default(),
            currency_info: super::globals::currency_data::seeded_currency_info_map(),
            maps: default_maps(),
            achievements: default_achievements(),
            achievement_guild_rep: ::std::collections::HashMap::new(),
            achievement_statistics: ::std::collections::HashMap::new(),
            achievement_comparison_unit: None,
            achievement_comparison_data: AchievementComparisonData::default(),
            guild_achievement_members: ::std::collections::HashMap::new(),
            achievement_search: AchievementSearchState::default(),
            focused_achievement: None,
            area_pois: default_area_pois(),
            bnet_friends: default_bnet_friends(),
            social_friends: default_social_friends(),
            auction_browse_results: default_auction_browse_results(),
            auction_replicate_items: default_auction_replicate_items(),
            auction_owned: Vec::new(),
            auction_bids: Vec::new(),
            mythic_plus: MythicPlusState::default(),
            character_services: CharacterServicesState::default(),
            scenario: ScenarioState::default(),
            death_recaps: Vec::new(),
            chat_bubbles: Vec::new(),
            summon_request: SummonRequestState::default(),
            player_map_position: (0.5, 0.5),
            factions: Vec::new(),
            selected_faction_index: 0,
            watched_faction_index: 0,
            battlefield_queue: BattlefieldQueue::default(),
            battlefield_minimap_visible: false,
            chat_channels: Vec::new(),
            macros: Vec::new(),
            running_macro: None,
            chat_windows: ::std::collections::HashMap::new(),
            chat_type_colors: ::std::collections::HashMap::new(),
            pending_duel: None,
            pending_resurrect: None,
            corpse_available: false,
            active_trade: None,
            open_panels: ::std::collections::HashSet::new(),
            is_party_lfg: false,
            everyone_assistant: false,
            party_leader_index: None,
            voice_chat: VoiceChatState::default(),
            known_spells: ::std::collections::HashSet::new(),
            harmful_spells: ::std::collections::HashSet::new(),
            helpful_spells: ::std::collections::HashSet::new(),
            pet_spells: ::std::collections::HashSet::new(),
            pvp_last_honor_gain: 0,
            equippable_items: ::std::collections::HashSet::new(),
            consumable_items: ::std::collections::HashSet::new(),
            can_replace_guild_master: false,
            auto_decline_guild_invites: false,
            guild_roster_show_offline: true,
            menu_open: false,
            xp_disabled: false,
            can_teleport: true,
            has_hearthstone: true,
            message_log: Vec::new(),
            keybindings: Keybindings::default(),
            debug_borders: false,
            debug_anchors: false,
        }
    };
}

// Re-export game data types so existing `crate::lua_api::state::X` imports keep working.
pub use super::game_data::AuraInfo;
pub use super::game_data::SpellCooldownState;
pub use super::game_data::{
    CLASS_LABELS, CastingState, PartyMember, RACE_DATA, ROT_DAMAGE_LEVELS, TargetInfo, XP_LEVELS,
    tick_party_health,
};
use super::game_data::{
    default_action_bars, default_party, default_player_buffs, random_player_name,
};
pub use super::state_types::{
    AchievementComparisonData, AchievementGuildRep, AchievementInfo, AchievementSearchState,
    AchievementStatistic, AddonInfo, AddonRuntimeMetrics, AppFrameMetrics, AreaPoiInfo,
    AuctionBrowseResult, AuctionReplicateItem, BagItem, BidAuction, BnetFriend, BnetGameAccount,
    ChatBubble, CraftingState, CurrencyInfo, CursorInfo, CursorItemOrigin, DeathRecapEntry,
    EquippedItem, GreatVaultActivity, GuildMember, GuildRank, KillingBlowInfo, LfgCategoryInfo,
    LootRollInfo, LuaErrorRecord, MacroInfo, MapData, MirrorTimer, MovementState, MythicPlusAffix,
    MythicPlusRatingMapSummary, MythicPlusRatingSummary, MythicPlusRun, MythicPlusState,
    MythicPlusWeeklyBest, NilSymbolAccess, OwnedAuction, PendingTimer, PlayerState, PlayerXpState,
    ScenarioState, ScenarioStep, SecondaryPowerState, SocialFriend, SummonRequestState, WorldState,
};
pub use super::tracked_recipes::TrackedRecipes;

// Per-frame side-table state (quest blobs, UnitPositionFrame, etc.)
// lives in `frame_substates.rs`; re-exported for existing
// `crate::lua_api::state::X` call sites.
pub use super::frame_substates::{
    FogOfWarFrameState, PendingPlayerReport, QuestBlobState, UnitPositionFrameState,
    UnitPositionPlayerPingTexture, UnitPositionUnit,
};

/// Account-store currency record — mirrors the official
/// `AccountStoreCurrencyInfo` structure in
/// `vendor/wow-ui-source/Interface/AddOns/Blizzard_APIDocumentationGenerated/AccountStoreDocumentation.lua`
/// (lines 240-249). The fields shipped here are the ones
/// `Blizzard_AccountStoreUtil.lua` actually reads from
/// `C_AccountStore.GetCurrencyInfo`.
#[derive(Clone, Debug)]
pub struct AccountStoreCurrencyInfo {
    pub id: i64,
    pub amount: i64,
    pub max_quantity: Option<i64>,
    pub name: String,
    pub icon: i64,
}

/// Account-store item record — mirrors the official `AccountStoreItemInfo`
/// structure in
/// `vendor/wow-ui-source/Interface/AddOns/Blizzard_APIDocumentationGenerated/AccountStoreDocumentation.lua`
/// (lines 252-271). Every card mixin in `Blizzard_AccountStore` reads
/// `status`, `mode`, `flags`, `price`, and `nonrefundable` from this struct
/// and the optional fields gate model-scene previews, descriptive tooltips,
/// transmog set previews, and the refund countdown overlay.
#[derive(Clone, Debug)]
pub struct AccountStoreItemInfo {
    pub id: i64,
    pub status: i64,
    pub mode: i64,
    pub currency_id: i64,
    pub flags: i64,
    pub custom_ui_model_scene_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub nonrefundable: bool,
    pub creature_display_id: Option<i64>,
    pub transmog_set_id: Option<i64>,
    pub display_icon: Option<i64>,
    pub refund_seconds_remaining: Option<i64>,
}

/// Account-store category record — mirrors the official
/// `AccountStoreCategoryInfo` structure in
/// `vendor/wow-ui-source/Interface/AddOns/Blizzard_APIDocumentationGenerated/AccountStoreDocumentation.lua`
/// (lines 229-238). `category_type` corresponds to
/// `Enum.AccountStoreCategoryType` and is read by
/// `AccountStoreItemDisplayMixin:OnCategorySelected` to choose between the
/// item-rack flavours (mounts vs. transmog vs. boost vs. service).
#[derive(Clone, Debug)]
pub struct AccountStoreCategoryInfo {
    pub id: i64,
    pub name: String,
    pub category_type: i64,
    pub icon: i64,
}

/// One shapeshift / stance form record consumed by `GetShapeshiftFormInfo`
/// and the `Blizzard_ActionBar/Shared/StanceBar.lua` mixin. Field shape
/// matches the live API: `(texture, isActive, isCastable, spellID)`. The
/// `name` field is read by `C_TooltipInfo.GetShapeshift` to render the
/// stance tooltip header.
#[derive(Clone, Debug)]
pub struct ShapeshiftForm {
    pub name: String,
    pub texture: String,
    pub spell_id: u32,
    pub is_active: bool,
    pub is_castable: bool,
}

/// One pet action-bar slot (10 slots total; `NUM_PET_ACTION_SLOTS = 10`).
/// Drives `GetPetActionInfo`, `GetPetActionCooldown`, `CastPetAction`,
/// `TogglePetAutocast`, and `PetHasActionBar` consumed by
/// `Blizzard_ActionBar/Shared/PetActionBar.lua`.
///
/// The 9-tuple returned by `GetPetActionInfo` maps as:
/// `(name, texture, is_token, is_active, auto_cast_allowed, auto_cast_enabled,
/// spell_id, _unused, passive)`. `has_action` is `false` for empty slots —
/// the live API returns `(nil, nil, false, false, false, false, nil, false, false)`
/// in that case.
#[derive(Clone, Debug, Default)]
pub struct PetActionSlot {
    pub has_action: bool,
    pub name: Option<String>,
    pub texture: Option<String>,
    pub is_token: bool,
    pub is_active: bool,
    pub auto_cast_allowed: bool,
    pub auto_cast_enabled: bool,
    pub spell_id: Option<u32>,
    pub passive: bool,
    pub cooldown: Option<SpellCooldownState>,
}

/// Glyph cursor state read by `Blizzard_ActionBar/Shared/SpellFlyout.lua`'s
/// `SpellFlyoutPopupButtonMixin:UpdateGlyphState` and the spellbook
/// glyph-attach flow. While a glyph is on the cursor,
/// `pending_glyph_name` carries its display name and
/// `pending_glyph_removal` is true if the cursor is the "Remove Glyph"
/// pseudo-glyph rather than a normal glyph item. `attached_glyphs` maps
/// spell id → glyph display name for spells that already have a glyph
/// inscribed; the flyout reads it to badge those spells with the icon.
#[derive(Clone, Debug, Default)]
pub struct GlyphState {
    pub pending_glyph_name: Option<String>,
    pub pending_glyph_removal: bool,
    pub attached_glyphs: HashMap<i32, String>,
}

/// Kind of an on-bar highlight mark — mirrors the action source that caused
/// the bar buttons to glow (spell hover, flyout drag, pet action drag).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionHighlightKind {
    Spell,
    Flyout,
    PetAction,
}

impl ActionHighlightKind {
    /// String tag returned alongside `GetOnBarHighlightMark(action)`.
    pub fn type_tag(self) -> &'static str {
        match self {
            ActionHighlightKind::Spell => "spell",
            ActionHighlightKind::Flyout => "flyout",
            ActionHighlightKind::PetAction => "petaction",
        }
    }
}

/// Action-highlight bookkeeping read by `Blizzard_ActionBar/Shared/ActionButton.lua`.
/// `new` mirrors `ACTION_HIGHLIGHT_MARKS` (set on `MarkNewActionHighlight`,
/// cleared on `ClearNewActionHighlight`); `on_bar` mirrors
/// `ON_BAR_HIGHLIGHT_MARKS` (rebuilt by the spell/flyout/pet update verbs).
#[derive(Default, Clone, Debug)]
pub struct ActionHighlightState {
    pub new: HashSet<i32>,
    pub on_bar: HashMap<i32, ActionHighlightKind>,
}

/// Action-bar transition state read by `MultiActionBar_Update` and
/// `ActionBarController_GetCurrentActionBarState`. `busy` is true while a
/// status-tracking-bar fade or page change is mid-animation, gating
/// `Blizzard_ActionBar/Shared/StanceBar.lua` updates. `current_state` matches
/// `LE_ACTIONBAR_STATE_MAIN` (1) or `LE_ACTIONBAR_STATE_OVERRIDE` (2) — the
/// override bar is only active while a vehicle/possess override is mounted.
#[derive(Clone, Debug)]
pub struct ActionBarStateInfo {
    pub busy: bool,
    pub current_state: i32,
}

impl Default for ActionBarStateInfo {
    fn default() -> Self {
        Self {
            busy: false,
            current_state: 1,
        }
    }
}

/// `C_Housing` favor-bar payload read by `HouseFavorBarMixin:Update`
/// (`vendor/wow-ui-source/Interface/AddOns/Blizzard_ActionBar/Mainline/HouseFavorBar.lua`).
/// `tracked_house_guid` is `Some` when a house is currently tracked for
/// favor display — a `None` value disables the bar entirely. `level_thresholds`
/// is indexed by level (1-based) and consumed by `GetHouseLevelFavorForLevel`;
/// out-of-range lookups (including the `level + 1` next-threshold probe past
/// the cap) return `0`, which is the sentinel the mixin checks to skip
/// `SetBarValues`.
#[derive(Clone, Debug, Default)]
pub struct HousingState {
    pub tracked_house_guid: Option<String>,
    pub current_level: i32,
    pub current_favor: i32,
    pub next_threshold: i32,
    pub max_level: i32,
    pub level_thresholds: Vec<i64>,
}

/// Profession-quality overlay payload returned by
/// `C_ActionBar.GetProfessionQualityInfo(slot)`. `inventoryQuality` matches
/// the retail 1..N tier band (T1 = lowest); `iconInventory` /
/// `iconQualityContainer` are the atlas keys
/// `ActionBarActionButtonMixin:UpdateProfessionQuality` passes to
/// `Texture:SetAtlas` for the overlay frame.
#[derive(Clone, Debug, Default)]
pub struct ProfessionQualityInfo {
    pub inventory_quality: i32,
    pub icon_inventory: String,
    pub icon_quality_container: String,
}

/// Loss-of-control cooldown payload returned by
/// `C_Spell.GetSpellLossOfControlCooldownInfo(spellID)`. Mirrors the retail
/// table shape consumed by `ActionButton_UpdateCooldown`'s lossOfControl
/// overlay branch — `isActive` flips the overlay on, the rest position the
/// swipe geometry. `should_replace_normal_cooldown` hides the underlying
/// spell cooldown swipe so the LoC overlay is the only one visible.
#[derive(Clone, Debug, Default)]
pub struct LossOfControlInfo {
    pub start_time: f64,
    pub duration: f64,
    pub mod_rate: f32,
    pub is_active: bool,
    pub should_replace_normal_cooldown: bool,
}

/// Paragon-rep payload returned by `C_Reputation.GetFactionParagonInfo`.
/// Presence in `state.faction_paragon` doubles as the
/// `IsFactionParagonForCurrentPlayer` truth, gating the gold reward badge in
/// `ReputationStatusBarMixin:Update`. Empty by default — the bar stays on the
/// standard rep code path.
#[derive(Clone, Debug)]
pub struct FactionParagonInfo {
    pub current_value: i32,
    pub threshold: i32,
    pub reward_quest_id: i32,
    pub has_reward_pending: bool,
    pub too_low_level_for_paragon: bool,
}

/// `MajorFactionData` row returned by `C_MajorFactions.GetMajorFactionData`
/// for a single faction. Drives `ReputationStatusBarMixin:Update` when the
/// watched faction is a major faction (Dragonflight Renown style bar).
#[derive(Clone, Debug)]
pub struct MajorFactionData {
    pub faction_id: i64,
    pub name: String,
    pub expansion_filter: i32,
    pub renown_level: i32,
    pub renown_reputation_earned: i32,
    pub renown_level_threshold: i32,
    pub is_unlocked: bool,
    pub unlock_description: Option<String>,
    pub celebration_sound_kit: i32,
    pub renown_fanfare_sound_kit_id: i32,
    pub texture_kit: String,
}

/// One entry in the renown level table returned by
/// `C_MajorFactions.GetRenownLevels`. `ReputationStatusBarMixin:GetMaxLevel`
/// reads the last entry's `level` to clamp the bar.
#[derive(Clone, Debug)]
pub struct RenownLevelInfo {
    pub faction_id: i64,
    pub level: i32,
    pub locked: bool,
    pub is_milestone: bool,
    pub is_capstone: bool,
}

/// `ItemLocation` payload returned by C surfaces that hand opaque locations
/// back to Lua (e.g. `C_AzeriteItem.FindActiveAzeriteItem`). Mirrors the
/// fields populated by `ItemLocationMixin` so callers that walk
/// `:GetBagAndSlot()`/`:GetEquipmentSlot()` see the expected shape.
#[derive(Clone, Debug, Default)]
pub struct ItemLocationData {
    pub bag_id: Option<i32>,
    pub slot_index: Option<i32>,
    pub equipment_slot_index: Option<i32>,
}

/// Heart-of-Azeroth state read by `C_AzeriteItem.*`. `None` on
/// `state.azerite_item` means no Azerite item is equipped —
/// `FindActiveAzeriteItem` returns nil so `AzeriteBarMixin:Update`
/// short-circuits.
#[derive(Clone, Debug)]
pub struct AzeriteItemState {
    pub item_location: ItemLocationData,
    pub current_xp: i64,
    pub max_xp: i64,
    pub power_level: i32,
    pub unlimited_power_level: i32,
    pub unlimited_unlocked: bool,
    pub at_max_level: bool,
    pub enabled: bool,
}

/// Equipped artifact metadata read by `C_ArtifactUI.GetEquippedArtifactInfo`,
/// `GetEquippedArtifactItemID`, `IsEquippedArtifactMaxed`, and
/// `IsEquippedArtifactDisabled`. `None` on `state.equipped_artifact` means no
/// artifact is wielded — `C_ArtifactUI.GetEquippedArtifactItemID` returns nil
/// and the `Blizzard_ActionBar/Mainline/ArtifactBar.lua` mixin stays hidden.
#[derive(Clone, Debug)]
pub struct ArtifactInfo {
    pub item_id: i32,
    pub alt_item_id: i32,
    pub name: String,
    pub icon: String,
    pub total_xp: i64,
    pub points_spent: i32,
    pub quality: i32,
    pub artifact_appearance_id: i32,
    pub appearance_mod_id: i32,
    pub item_appearance_id: i32,
    pub alt_item_appearance_id: i32,
    pub alt_on_top: bool,
    pub tier: i32,
    pub maxed: bool,
    pub disabled: bool,
}

/// Shared simulator state accessible from Lua.
pub struct SimState {
    pub widgets: WidgetRegistry,
    pub events: EventQueue,
    pub scripts: ScriptRegistry,
    /// Console output from Lua print() calls.
    pub console_output: Vec<String>,
    /// Pending timer callbacks.
    pub timers: VecDeque<PendingTimer>,
    /// Pending timer callbacks for the rilua VM.
    pub rilua_timers: VecDeque<crate::lua_api::timer_layout::RiluaPendingTimer>,
    /// Currently focused frame ID (for keyboard input).
    pub focused_frame_id: Option<u64>,
    /// Registered addons (includes all scanned addons, not just loaded ones).
    pub addons: Vec<AddonInfo>,
    /// Last `C_AddOns.SaveAddOns` snapshot of `addons[i].enabled`, indexed
    /// to match `addons`. `None` means no commit has happened yet, in which
    /// case `ResetAddOns` is a no-op. Live mutations through
    /// `EnableAddOn`/`DisableAddOn`/`EnableAllAddOns`/`DisableAllAddOns`
    /// modify `addons[i].enabled` directly; `Save` overwrites this snapshot
    /// with the current live state, and `Reset` restores live state from
    /// this snapshot.
    pub addon_saved_enable_state: Option<Vec<bool>>,
    /// System chat log: append-only record of messages dispatched through
    /// `ChatFrameUtil.AddSystemMessage`. Persists even when no chat frame is
    /// available so tests and headless runs can inspect what was emitted.
    pub system_chat_log: Vec<String>,
    /// Console variables (CVars).
    pub cvars: CVarStorage,
    /// Tooltip state for GameTooltip frames (keyed by frame ID).
    pub tooltips: HashMap<u64, TooltipData>,
    /// Aura instance IDs hidden from default unit aura iteration (keyed by unit token).
    pub blocked_auras_by_unit: HashMap<String, HashSet<i32>>,
    /// Quest blob state for QuestPOIFrame widgets (keyed by frame ID).
    pub quest_blobs: HashMap<u64, QuestBlobState>,
    /// FogOfWarFrame state (keyed by frame ID).
    pub fog_of_war_frames: HashMap<u64, FogOfWarFrameState>,
    /// UnitPositionFrame state (keyed by frame ID).
    pub unit_position_frames: HashMap<u64, UnitPositionFrameState>,
    /// Pending report tokens created by `C_ReportSystem.InitiateReportPlayer`.
    pub pending_player_reports: HashMap<i64, PendingPlayerReport>,
    /// SimpleHTML state (keyed by frame ID).
    pub simple_htmls: HashMap<u64, SimpleHtmlData>,
    /// MessageFrame state (keyed by frame ID).
    pub message_frames: HashMap<u64, MessageFrameData>,
    /// Frame IDs with active OnUpdate script handlers.
    pub on_update_frames: HashSet<u64>,
    /// Cached subset of `on_update_frames` whose ancestors are all visible.
    /// Invalidated when `WidgetRegistry::visibility_dirty` is set.
    pub visible_on_update_cache: Option<Vec<u64>>,
    /// Per-strata buckets of visible frame IDs. Index = FrameStrata as usize.
    /// Contains only frames with render_alpha > 0 (visible or button state
    /// textures with visible parent). Built lazily, maintained surgically
    /// by `set_frame_visible`.
    pub strata_buckets: Option<Vec<Vec<u64>>>,
    /// Pending HitGrid updates from `set_frame_visible`. Each entry is the root
    /// frame ID that changed visibility and whether it became visible.
    /// Drained and applied by the App after Lua handlers run.
    pub pending_hit_grid_changes: Vec<(u64, bool)>,
    /// Texture paths queued by API-side preload requests such as `C_Map.RequestPreloadMap`.
    pub pending_texture_preloads: BTreeSet<String>,
    /// Animation groups keyed by unique group ID.
    pub animation_groups: HashMap<u64, AnimGroupState>,
    /// Counter for generating unique animation group IDs.
    pub next_anim_group_id: u64,
    /// Map: animation-group frame_id → group_id in `animation_groups`.
    pub anim_frame_to_group: HashMap<u64, u64>,
    /// Map: animation frame_id → (group_id, anim_index).
    pub anim_frame_to_anim: HashMap<u64, (u64, usize)>,
    /// Screen dimensions in UI coordinates.
    pub screen_width: f32,
    pub screen_height: f32,
    /// Requested UI surface (in-game vs glue screen).
    pub screen_kind: ScreenKind,
    /// Whether the simulated player is logged into the world.
    pub is_logged_in: bool,
    /// Whether the current glue screen has been displayed at least once.
    pub screen_first_displayed: bool,
    /// Remembered account name for glue login UI helpers.
    pub saved_account_name: String,
    /// Remembered account list string for glue login UI helpers.
    pub saved_account_list: String,
    /// Whether the saved account uses token login.
    pub uses_token: bool,
    /// Whether account-save export is available on this build/runtime.
    pub account_save_enabled: bool,
    /// Whether an account-save export is currently active.
    pub account_save_in_progress: bool,
    /// Whether the account is locked after a successful save/export.
    pub account_locked_post_save: bool,
    /// Last item id passed to `C_AccountStore.BeginPurchase`. None until called.
    pub last_account_store_purchase_request: Option<i64>,
    /// Return value for `C_AccountStore.BeginPurchase` — true to simulate a
    /// queued purchase request, false to simulate the C side rejecting it.
    pub account_store_begin_purchase_succeeds: bool,
    /// Last item id passed to `C_AccountStore.RefundItem`. None until called.
    pub last_account_store_refund_request: Option<i64>,
    /// Return value for `C_AccountStore.RefundItem` — true to simulate a
    /// queued refund request, false to simulate the C side rejecting it.
    pub account_store_refund_succeeds: bool,
    /// Item ids exposed by `C_AccountStore.GetCategoryItems(categoryID)`.
    /// Keyed by category id; absent or empty entries return an empty array.
    pub account_store_category_items: HashMap<i64, Vec<i64>>,
    /// Currency id used by each storefront. Drives
    /// `C_AccountStore.GetCurrencyIDForStore(storeFrontID)`; missing entries
    /// return nil so the footer's currency tooltip stays hidden.
    pub account_store_currency_for_store: HashMap<i64, i64>,
    /// Currency records exposed by `C_AccountStore.GetCurrencyInfo`.
    /// Missing entries return nil; AccountStoreUtil branches on `currencyInfo`
    /// being non-nil and on `maxQuantity` being non-nil to gate warnings.
    pub account_store_currency_info: HashMap<i64, AccountStoreCurrencyInfo>,
    /// Loading / availability state per storefront, as
    /// `Enum.AccountStoreState`. Drives `C_AccountStore.GetStoreFrontState`;
    /// missing entries default to `Available` (0) so unconfigured tests don't
    /// accidentally gate buttons closed.
    pub account_store_storefront_state: HashMap<i64, i64>,
    /// Last storefront id passed to `C_AccountStore.RequestStoreFrontInfoUpdate`.
    /// None until called. Tests assert on this to confirm the async refresh
    /// request reached the C side before they fire `ACCOUNT_STORE_FRONT_UPDATED`.
    pub last_account_store_storefront_info_request: Option<i64>,
    /// Category records exposed by `C_AccountStore.GetCategoryInfo(categoryID)`.
    /// Missing entries return nil so the AccountStore mixin can guard against
    /// stale category ids during reload.
    pub account_store_categories: HashMap<i64, AccountStoreCategoryInfo>,
    /// Item records exposed by `C_AccountStore.GetItemInfo(itemID)`. Missing
    /// entries return nil so card mixins surface "no data yet" rather than
    /// faking an unpriced item with default values.
    pub account_store_items: HashMap<i64, AccountStoreItemInfo>,
    /// Action bar slots: slot (1-120) → spell ID.
    pub action_bars: HashMap<u32, u32>,
    /// Action-bar transition state. Drives `ActionBarBusy()` (set true while
    /// a status-tracking-bar fade or page change is mid-animation) and
    /// `ActionBarController_GetCurrentActionBarState()` (1 = main bar,
    /// 2 = override bar mounted by a vehicle/possess override).
    pub action_bar_state: ActionBarStateInfo,
    /// Action-highlight bookkeeping for `MarkNewActionHighlight` and the
    /// `On Bar` highlight verbs. Read by Blizzard_ActionBar buttons during
    /// hover/drag updates.
    pub action_highlights: ActionHighlightState,
    /// Currently equipped artifact (legacy-spec content). `None` when no
    /// artifact is wielded — drives `C_ArtifactUI.GetEquippedArtifactItemID`
    /// returning nil and keeps the `ArtifactBarMixin` hidden.
    pub equipped_artifact: Option<ArtifactInfo>,
    /// XP cost of the next trait point keyed by `(points_spent, tier)`.
    /// Consumed by `C_ArtifactUI.GetCostForPointAtRank` and the
    /// `ArtifactBarGetNumArtifactTraitsPurchasableFromXP` helper. Empty by
    /// default — callers receive 0 for missing entries, which the helper
    /// treats as "no further point purchasable".
    pub artifact_point_costs: HashMap<(i32, i32), i64>,
    /// Heart of Azeroth state. `None` keeps
    /// `C_AzeriteItem.FindActiveAzeriteItem` returning nil so the
    /// `Blizzard_ActionBar/Mainline/AzeriteBar.lua` mixin stays hidden.
    pub azerite_item: Option<AzeriteItemState>,
    /// Major-faction (Renown) data keyed by `factionID`. Empty by default —
    /// `C_MajorFactions.GetMajorFactionData` returns nil for unknown ids and
    /// `C_Reputation.IsMajorFaction` reports false, keeping the
    /// `ReputationStatusBarMixin` on the standard rep code path.
    pub major_factions: HashMap<i64, MajorFactionData>,
    /// Renown level rungs keyed by `factionID`. Empty by default —
    /// `C_MajorFactions.GetRenownLevels` returns an empty sequence.
    pub major_faction_renown_levels: HashMap<i64, Vec<RenownLevelInfo>>,
    /// Faction ids whose reputation is shared across the Battle.net account.
    /// `C_Reputation.IsAccountWideReputation` looks up membership; empty
    /// keeps every faction reported as character-bound.
    pub account_wide_reputation_factions: HashSet<i64>,
    /// Paragon-rep state keyed by `factionID`. Presence flips
    /// `C_Reputation.IsFactionParagonForCurrentPlayer` to true and feeds
    /// `C_Reputation.GetFactionParagonInfo`. Empty by default.
    pub faction_paragon: HashMap<i64, FactionParagonInfo>,
    /// Locked transmog outfit ids — `C_TransmogOutfitInfo.IsLockedOutfit`
    /// reports membership. Empty by default. `ActionBarButtonMixin:UpdateUsable`
    /// uses this to gray out outfit-action buttons whose outfits are
    /// momentarily restricted (e.g. stance/spec switches).
    pub transmog_outfit_locks: HashSet<i64>,
    /// Whether `C_TransmogOutfitInfo.IsEquippedGearOutfitLocked()` reports
    /// true. Default false (the equipped-gear pseudo-outfit is freely
    /// usable). Drives the lock badge for the equipped-gear shortcut button
    /// in `ActionBarButtonMixin:UpdateUsable`.
    pub equipped_outfit_locked: bool,
    /// Action slot ids reported as locked by `C_LevelLink.IsActionLocked`.
    /// Empty by default — `ActionBarButtonMixin:UpdateAction` then leaves
    /// trial-account-restricted buttons un-dimmed.
    pub locked_action_slots: HashSet<i32>,
    /// Whether `C_PvP.IsActiveBattlefield()` reports true. Default false
    /// (no active battleground). `StatusTrackingManager` checks this to
    /// hide XP / honor bars while the player is queued into a battlefield.
    pub is_active_battlefield: bool,
    /// Trade-skill spell hyperlinks, keyed by spell id. Drives
    /// `C_Spell.GetSpellTradeSkillLink`. Empty by default — only profession
    /// recipe spells return a non-nil link in retail.
    pub spell_trade_skill_links: HashMap<u32, String>,
    /// Spell-identifier aliases for `C_Spell.GetSpellIDForSpellIdentifier`.
    /// Keys are either the numeric form (`"133"`) or the lowercased spell
    /// name (`"fireball"`); values are the resolved override spell id.
    /// Empty by default — the surface treats numeric input as an identity
    /// mapping when no alias is registered.
    pub spell_id_aliases: HashMap<String, u32>,
    /// Loss-of-control cooldown info, keyed by spell id. Drives
    /// `C_Spell.GetSpellLossOfControlCooldownInfo`. Empty by default —
    /// `ActionButton_UpdateCooldown` then falls back to the inert
    /// `defaultLossOfControlInfo` baseline.
    pub spell_loss_of_control: HashMap<u32, LossOfControlInfo>,
    /// Profession-quality overlays for action slots. Drives
    /// `C_ActionBar.GetProfessionQualityInfo`. Empty by default —
    /// `ActionBarActionButtonMixin:UpdateProfessionQuality` then clears the
    /// overlay frame for that slot.
    pub action_profession_quality: HashMap<i32, ProfessionQualityInfo>,
    /// Addon base paths for runtime on-demand loading (Blizzard UI + AddOns directories).
    pub addon_base_paths: Vec<PathBuf>,
    /// One-shot override for XML frame creation: whether the next CreateFrame
    /// should start hidden before registration/render eligibility.
    pub create_frame_initial_hidden: Option<bool>,
    /// Depth-counted suppression for runtime CreateFrame OnLoad firing while
    /// XML loader code is still building the frame tree.
    pub suppress_runtime_on_load_depth: u32,
    /// Depth-counted marker for execution that originated from XML loader or
    /// XML lifecycle handlers. Used to distinguish `LoadAddOn` trace output.
    pub xml_load_addon_depth: u32,
    /// Current mouse position in UI coordinates (for ANCHOR_CURSOR tooltip positioning).
    pub mouse_position: Option<(f32, f32)>,
    /// Currently hovered frame ID (for IsMouseMotionFocus / GetMouseFocus).
    pub hovered_frame: Option<u64>,
    /// Frame currently owning the active mouse drag, if any.
    pub active_drag_frame: Option<u64>,
    /// Slider currently holding the left mouse for thumb dragging, if any.
    pub active_slider_thumb_drag_frame: Option<u64>,
    /// Counter for generating unique report tokens.
    pub next_report_token: i64,
    /// Simulated party members (empty = not in group).
    pub party_members: Vec<PartyMember>,
    /// Whether group-wide APIs should expose the simulated party to Blizzard UI.
    pub party_group_active: bool,
    /// Current target (None = no target).
    pub current_target: Option<TargetInfo>,
    /// Previous target — set to the old `current_target` value each time the
    /// target changes or is cleared. Drives `TargetLastTarget`.
    pub previous_target: Option<TargetInfo>,
    /// Current focus target (None = no focus).
    pub current_focus: Option<TargetInfo>,
    /// Enemy pool for `TargetNearestEnemy`. Empty by default; seeded via
    /// `A_Admin.SetEnemyPool(...)`. Picking always returns the first entry.
    pub enemy_pool: Vec<TargetInfo>,
    /// Audio playback manager (None when no audio device or WOW_SIM_NO_SOUND=1).
    pub sound_manager: Option<SoundManager>,
    /// Most recent global `PlaySound(soundKitID)` request.
    pub last_sound_kit_requested: Option<u32>,
    /// Most recent global `PlaySoundFile(path)` request.
    pub last_sound_file_requested: Option<String>,
    /// Most recent global `StopSound(handle)` request.
    pub last_stopped_sound_handle: Option<u32>,
    /// Most recent global `LaunchURL(url)` request. The simulator never
    /// opens an external browser; tests assert what URL was passed.
    pub last_launched_url: Option<String>,
    /// Character GUID currently highlighted by glue `MapSceneCharacterHighlightStart`.
    pub highlighted_map_scene_character_guid: Option<String>,
    /// Whether the legacy `MultiActionBar_ShowAllGrids` shim has grids enabled.
    pub multi_action_bar_grids_shown: bool,
    /// Registered secure state/attribute drivers keyed by frame id and
    /// attribute name. The sim applies them eagerly when registered and keeps
    /// the raw option text so future updates can reuse the same mapping.
    pub secure_attribute_drivers: HashMap<u64, HashMap<String, String>>,
    /// Rot damage intensity (index into ROT_DAMAGE_LEVELS).
    pub rot_damage_level: usize,
    /// Current framerate (FPS), updated by the app's FPS counter.
    pub fps: f32,
    /// Instant at which the UI started (used by GetTime and message timestamps).
    pub start_time: Instant,
    /// Active spell cast (None = not casting).
    pub casting: Option<CastingState>,
    /// Active spell *channel* (None = not channeling). Read by
    /// `UnitChannelInfo("player")`. Independent from `casting` because
    /// real WoW reports both via separate APIs and a player can be
    /// casting one spell while channeling never happens, but a frame
    /// switching from cast→channel must drop the old `casting` slot.
    pub channeling: Option<CastingState>,
    /// Counter for generating unique cast IDs.
    pub next_cast_id: u32,
    /// Global Cooldown: (start_time, duration) in GetTime() seconds.
    pub gcd: Option<(f64, f64)>,
    /// Per-spell cooldowns: spell_id → SpellCooldownState.
    pub spell_cooldowns: HashMap<u32, SpellCooldownState>,
    /// Per-inventory-slot cooldowns keyed by equipment slot id (same slot
    /// values as `PlayerState.equipped_items`). Drives
    /// `GetInventoryItemCooldown(unit, slot)`. Empty by default.
    pub inventory_item_cooldowns: HashMap<i32, SpellCooldownState>,
    /// Buttons registered via SetActionUIButton(button, action, cooldownFrame).
    pub action_ui_buttons: Vec<(u64, u32)>,
    /// What is currently held on the cursor (drag-and-drop).
    pub cursor_item: Option<CursorInfo>,
    /// Index of the addon currently being loaded (into `addons` vec).
    pub loading_addon_index: Option<u16>,
    /// Stack of addon indexes currently being loaded, oldest to newest.
    pub loading_addon_stack: Vec<u16>,
    /// Index of the addon whose code is currently executing (event/timer/script handlers).
    pub executing_addon_index: Option<u16>,
    /// Whether loading inside a ScopedModifier with forbidden="true".
    pub loading_forbidden: bool,
    /// Application-level frame metrics (total frame time for profiler ratios).
    pub app_frame_metrics: AppFrameMetrics,
    /// Talent tree interactive state (ranks, selections, currency mappings).
    pub talents: super::talent_state::TalentState,
    /// Collected Lua errors (from call_error_handler and addframetext).
    pub lua_errors: Vec<String>,
    /// Collected Lua errors with optional addon attribution.
    pub lua_error_records: Vec<LuaErrorRecord>,
    /// Count of normalized Lua error messages seen so far.
    pub lua_error_counts: HashMap<String, usize>,
    /// Missing global / namespace symbol accesses captured by logging `__index` hooks.
    pub nil_symbol_accesses: Vec<NilSymbolAccess>,
    /// Global cross-frame Show/Hide dispatch depth (prevents Lua stack overflow
    /// when OnShow handlers trigger Show on other frames recursively).
    pub global_show_hide_depth: u32,
    /// Synced animation group start times (key → elapsed Duration when first PlaySynced was called).
    pub anim_sync_times: HashMap<String, std::time::Duration>,

    /// Player character state (identity, combat, power, buffs, spec).
    pub player: PlayerState,
    /// XP / honor / rest cluster consumed by `Blizzard_ActionBar/Shared/ExpBar.lua`
    /// and `Mainline/HonorBar.lua`. Holds rest-XP exhaustion, the Rested/Normal
    /// rest state, honor totals, trial-account caps, and limited-mode banked
    /// XP flags. Basic xp / xp_max / honor_level / is_resting / xp_disabled
    /// stay on `player` because their pre-existing readers key off it.
    pub player_xp: PlayerXpState,
    /// World state (zone, instance, guild, collections, vault, loot).
    pub world: WorldState,
    /// Bags/Inventory: (bag_index, slot_index) → BagItem.
    pub bag_items: HashMap<(i32, i32), BagItem>,
    /// Tracked recipes for the Profession Recipe Tracker, keyed by
    /// `is_recrafting`. Drives `C_TradeSkillUI.GetRecipesTracked` /
    /// `IsRecipeTracked` / `SetRecipeTracked`. Empty by default.
    pub tracked_recipes: TrackedRecipes,
    /// Dynamic crafting state (selected profession, known recipes).
    /// Static recipe catalogue lives in `globals::profession_data`.
    pub crafting: CraftingState,
    /// Simulated network stats returned by `GetNetStats`. All fields default to 0
    /// because the sim has no real network socket; tests can inject values via
    /// `A_Admin.SetNetStats(bandwidthIn, bandwidthOut, latencyHome, latencyWorld)`
    /// to exercise UI paths that depend on latency or bandwidth thresholds
    /// (e.g. Blizzard_MicroMenu's status-icon color ramp).
    pub net_stats: NetStats,
    /// Whether the in-game Store window is currently shown. The sim doesn't
    /// actually render the Store, but `MainMenuBarMicroButtons` colours the
    /// Store micro-button as pushed when `StoreFrame_IsShown()` returns true,
    /// so tests can flip this flag via `A_Admin.SetStoreFrameShown(true)` to
    /// exercise that pushed-state rendering.
    pub store_frame_shown: bool,
    /// Active Timerunning season id, or `None` when the player is not in a
    /// seasonal mode. Drives both `PlayerIsTimerunning()` (returns `is_some`)
    /// and `PlayerGetTimerunningSeasonID()` (returns the id, or 0 when none —
    /// WoW uses 0 as "not timerunning" on the integer-returning API).
    /// Admin: `A_Admin.SetTimerunningSeasonID(id?)` — nil/0 clears.
    pub timerunning_season_id: Option<u32>,
    /// Modifier key state backing `IsShiftKeyDown` / `IsControlKeyDown` /
    /// `IsAltKeyDown` / `IsMetaKeyDown` / `IsModifierKeyDown`. All default
    /// false (no input to the sim). Admin: `A_Admin.SetShiftKeyDown(b)` and
    /// friends toggle individual keys.
    pub modifier_keys: ModifierKeys,
    /// Mouse-button down state backing `IsMouseButtonDown([button])`.
    /// Defaults all false (no input to the sim).
    pub mouse_buttons: MouseButtons,
    /// `C_GameRules` backing state — active game mode + glue-screen name +
    /// a rules map. Default: Standard mode, `CharacterSelect` glue screen,
    /// empty rules.
    pub game_rules: GameRulesState,
    /// Whether `C_Housing.IsHousingServiceEnabled()` reports true. Drives
    /// MainMenuBarMicroButtons' decision to render the Housing micro-button.
    /// Default true so the housing dashboard can be opened from the live UI.
    pub housing_service_enabled: bool,
    /// `C_Housing` favor-bar state — drives `GetTrackedHouseGuid`,
    /// `GetCurrentHouseLevelFavor`, `GetHouseLevelFavorForLevel`, and
    /// `GetMaxHouseLevel`. Default all-zero / `None`, which keeps
    /// `HouseFavorBarMixin:Update` on the inert "no tracked house" path.
    pub housing: HousingState,
    /// Backing state for `C_PetBattles.GetNumPets(owner)` and
    /// `C_PetBattles.GetBattleState()`. Default zeros (no active battle).
    pub pet_battles: PetBattleState,
    /// Hunter / warlock pet stats. Drives the legacy `GetPetExperience`
    /// / `GetPetHappiness` / `GetPetLoyalty` / `GetPetTimeInCombat`
    /// probes. Default all-zero (no pet).
    pub pet: PetState,
    /// Backing state for `C_LFGList.GetNumApplications` /
    /// `GetNumApplicants`. Each probe returns `(total, viewed)` — the sim
    /// exposes both knobs so tests can assert `total > 0 && viewed == 0`
    /// scroll behaviour without standing up a real LFG listing.
    pub lfg_list_counts: LfgListCounts,
    /// Whether `C_LFGInfo.CanPlayerUsePremadeGroup()` reports true. Sim
    /// default is false — the Premade Group Finder UI is gated off in a
    /// fresh env. Admin: `A_Admin.SetCanUsePremadeGroup(b?)`.
    pub can_use_premade_group: bool,
    /// Category metadata for `C_LFGInfo.GetLFGCategoryInfo(id)`.
    /// Seeded with category 2 (Dungeons) and 3 (Raids). Unknown ids
    /// return nil.
    pub lfg_category_info: std::collections::HashMap<i32, LfgCategoryInfo>,
    /// Set of category ids for which `C_LFGInfo.IsLFGModeActiveForCategory`
    /// returns true. Default empty (no active LFG modes).
    pub lfg_active_categories: std::collections::HashSet<i32>,
    /// Whether `C_PhotoSharing.IsAuthorized()` reports true. Sim has no
    /// real photo-sharing service; default false. Admin:
    /// `A_Admin.SetPhotoSharingAuthorized(b?)`.
    pub photo_sharing_authorized: bool,
    /// Whether `C_PhotoSharing.IsEnabled()` reports true. Separate from
    /// authorization — a user can decline the feature after authorizing.
    /// Default false. Admin: `A_Admin.SetPhotoSharingEnabled(b?)`.
    pub photo_sharing_enabled: bool,
    /// Tutorial/account flags acknowledged through `C_Tutorial`.
    /// Default empty: a fresh sim has not seen any account tutorials.
    pub tutorial_flags: HashSet<u32>,
    /// Seeded WoW Labs / Plunderstorm matchmaking state used by the
    /// `C_WowLabs*` namespaces.
    pub wowlabs: WowLabsState,
    /// Active quests in the player's log (quest IDs). Order reflects
    /// accept order. Drives `GetNumQuestLogEntries` and the quest-verbs
    /// module in `globals/quest_verbs.rs`.
    pub quest_log: Vec<u32>,
    /// Rich quest metadata for `C_QuestLog.*` probes (GetInfo, IsComplete,
    /// GetNextWaypoint, etc.). Seeded at init; tests mutate via `st.quest_log_entries`.
    pub quest_log_entries: QuestLogState,
    /// Pending quest offer displayed in the quest detail frame. Consumed
    /// by `ConfirmAcceptQuest`. `None` means no pending offer.
    pub pending_quest_offer: Option<u32>,
    /// Active quest-choice dialog id set by `QuestChoiceFrame_SetActiveChoice`.
    pub quest_choice_id: Option<u32>,
    /// Last map id passed to `C_QuestLog.SetMapForQuestPOIs`.
    pub quest_poi_map_id: Option<i32>,
    /// Quest id most recently clicked in the quest map log via
    /// `QuestMapLogTitleButton_OnClick`.
    pub selected_quest_log_id: Option<u32>,
    /// Quest id marked for abandonment via `SetAbandonQuest`. `AbandonQuest`
    /// would complete the flow in real WoW; the sim just records the mark.
    pub abandon_quest_id: Option<u32>,
    /// Achievements the player is actively tracking. Drives
    /// `SetTrackedAchievement` / `UntrackAchievement`.
    pub tracked_achievements: HashSet<i32>,
    /// Whether the player has the bank frame open.
    pub bank_frame_open: bool,
    /// Whether the guild bank frame is open.
    pub guild_bank_frame_open: bool,
    /// Whether the merchant frame is open.
    pub merchant_frame_open: bool,
    /// Whether the tabard-creation UI is open.
    pub tabard_frame_open: bool,
    /// Whether the class/profession trainer UI is open.
    pub trainer_frame_open: bool,
    /// Whether the item-socket frame is open.
    pub socket_frame_open: bool,
    /// Whether the loot window is open.
    pub loot_frame_open: bool,
    /// Whether the guild registrar/petition UI is open.
    pub guild_registrar_open: bool,
    /// Whether the pet-stables UI is open.
    pub pet_stables_open: bool,
    /// Items currently on the active merchant's page. Retail's
    /// `GetMerchantNumItems` returns this length. Empty when no merchant
    /// is open.
    pub merchant_items: Vec<u32>,
    /// Loot slots on the active loot window. Retail's `GetNumLootItems`
    /// returns this length. Empty when no loot window is open.
    pub loot_slots: Vec<u32>,
    /// Auction-house browse results (first return of
    /// `GetNumAuctionItems("list")`). We only model the `"list"` bucket;
    /// `"owner"` / `"bidder"` are always 0. Empty when no browse has
    /// completed.
    pub auction_browse_items: Vec<u32>,
    /// Party / raid loot-method state — drives `GetLootMethod()`,
    /// `GetMasterLooterThreshold()`, and the `RequestPartyLootMethod()`
    /// event refresh. Defaults to personal loot, threshold 2 (Uncommon).
    pub loot_method: LootMethodState,
    /// Active gossip-dialog state. Drives `GetGossipNumOptions` /
    /// `GetGossipNumAvailableQuests` / `GetGossipNumActiveQuests`.
    /// Defaults to inactive with zero counts.
    pub gossip: GossipState,
    /// Torghast (Jailer's Tower) run state. Drives
    /// `IsOnGroundFloorInJailersTower()`. Defaults to inactive / floor 0.
    pub torghast: TorghastState,
    /// Title names the player has unlocked, in display order. Drives
    /// `GetNumTitles` / `GetTitleName(index)`. Empty by default.
    pub titles: Vec<String>,
    /// Currently-available shapeshift forms (druid / shaman / priest
    /// tokens). Drives `GetNumShapeshiftForms`, `GetShapeshiftFormInfo`,
    /// `CastShapeshiftForm`, and the StanceBar mixin. Empty by default;
    /// the seeded Paladin player has no forms.
    pub shapeshift_forms: Vec<ShapeshiftForm>,
    /// Per-form cooldowns keyed by 1-based form index. Drives
    /// `GetShapeshiftFormCooldown`. Empty by default.
    pub shapeshift_cooldowns: HashMap<i32, SpellCooldownState>,
    /// Pet-bar slots (10 slots, fixed length). Drives
    /// `GetNumPetActions`, `GetPetActionInfo`, `GetPetActionCooldown`,
    /// `CastPetAction`, `TogglePetAutocast`, and `PetHasActionBar`.
    /// Default 10 empty slots; `PetHasActionBar` reports false until at
    /// least one slot has `has_action = true`.
    pub pet_actions: Vec<PetActionSlot>,
    /// Glyph cursor state. Drives `HasPendingGlyphCast`, `HasAttachedGlyph`,
    /// `IsPendingGlyphRemoval`, `GetCurrentGlyphNameForSpell`,
    /// `GetPendingGlyphName`, `AttachGlyphToSpell`, and
    /// `IsSpellValidForPendingGlyph` consumed by `SpellFlyout.lua` and the
    /// spellbook. Default empty: no glyph on cursor, no spells inscribed.
    pub glyph: GlyphState,
    /// Currency info keyed by currency id. Drives
    /// `C_CurrencyInfo.GetCurrencyInfo`, `GetCurrencyInfoFromLink`,
    /// and `GetCurrencyContainerInfo`. Seeded at startup from the
    /// static `currency_data::CURRENCY_LIST`.
    pub currency_info: HashMap<i32, CurrencyInfo>,
    /// Map metadata keyed by ui-map id. Drives `C_Map.GetMapArtID`,
    /// `GetMapChildrenInfo`, `GetPlayerMapPosition`. Seeded with the
    /// Azeroth world map, Eastern Kingdoms continent, and Stormwind
    /// City zone.
    pub maps: HashMap<i32, MapData>,
    /// Achievement metadata keyed by achievement id. Drives
    /// `C_AchievementInfo.GetAchievementInfo` / `GetRewardItemID` /
    /// `IsValidAchievement`. Seeded with a handful of well-known
    /// retail ids (Level 10, Explore Eastern Kingdoms, World Defender,
    /// Feats of Strength headers). Completion state is derived from
    /// `world.earned_achievements` at read time.
    pub achievements: HashMap<i32, AchievementInfo>,
    /// Reputation-gating metadata keyed by achievement id. Drives
    /// `GetAchievementGuildRep`, which the tooltip calls to mark
    /// achievements like Justicar as "requires Honored with X". Empty
    /// by default — unseeded ids report `(false, false, nil)`.
    pub achievement_guild_rep: HashMap<i32, AchievementGuildRep>,
    /// Statistic display rows keyed by achievement id. Drives the
    /// legacy global `GetStatistic`, called from the
    /// `AchievementFrameStats` summary list. Empty by default — the
    /// addon's `if not quantity then quantity = "--" end` fallback
    /// handles missing rows.
    pub achievement_statistics: HashMap<i32, AchievementStatistic>,
    /// Active inspect/comparison target unit token (e.g. `"target"`,
    /// `"party1"`). `Some(unit)` when the player has selected a
    /// comparison via `SetAchievementComparisonUnit`; `None` after
    /// `ClearAchievementComparisonUnit` or before any selection.
    pub achievement_comparison_unit: Option<String>,
    /// Friend-side achievement snapshot read by the
    /// `GetComparison*` getters. Stays at default until a test or
    /// fixture seeds it; the addon then renders the friend's row.
    pub achievement_comparison_data: AchievementComparisonData,
    /// Guild member rosters keyed by guild achievement id. Drives the
    /// trio `GetGuildAchievementNumMembers`,
    /// `GetGuildAchievementMembers` (async no-op), and
    /// `GetGuildAchievementMemberInfo`. Empty rosters yield
    /// `numMembers == 0`, which the addon treats as "fetch from
    /// server" and silently drops the tooltip section.
    pub guild_achievement_members: HashMap<i32, Vec<String>>,
    /// Achievement search state. Drives `SetAchievementSearchString`
    /// plus the four read-only progress/result globals consumed by
    /// `AchievementFrameSearchProgressBar_OnUpdate` and
    /// `AchievementFrame_ShowSearchPreviewResults`.
    pub achievement_search: AchievementSearchState,
    /// Last achievement id passed to `SetFocusedAchievement`. The
    /// addon writes this on selection but never reads it back, so the
    /// field exists purely so tests can verify the call landed.
    pub focused_achievement: Option<i32>,
    /// Area-POI metadata keyed by area poi id. Drives
    /// `C_AreaPoiInfo.GetAreaPOIInfo` / `GetAreaPOISecondsLeft`.
    /// Seeded with a tiny fixture (Mage Tower Stormwind +
    /// one time-limited world event).
    pub area_pois: HashMap<i32, AreaPoiInfo>,
    /// BattleNet friends list. Drives `C_BattleNet.GetNumFriends`,
    /// `GetFriendAccountInfo`, `GetAccountInfoByGUID`,
    /// `GetGameAccountInfoByGUID`, and `GetFriendNumAccounts`.
    /// Seeded with one online and one offline friend.
    pub bnet_friends: Vec<BnetFriend>,
    /// WoW friends list. Drives `C_Social.GetFriendInfo`,
    /// `C_Social.GetFriends`, and `C_FriendList.GetNumFriends`.
    /// Seeded with two representative entries.
    pub social_friends: Vec<SocialFriend>,
    /// Browse-tab results on the Auction House. Drives
    /// `C_AuctionHouse.GetBrowseResults`. Seeded with a couple of
    /// representative listings.
    pub auction_browse_results: Vec<AuctionBrowseResult>,
    /// Replicate-scan snapshot for the Auction House. Drives
    /// `C_AuctionHouse.GetReplicateItemInfo`. Seeded with a couple of
    /// commodity rows.
    pub auction_replicate_items: Vec<AuctionReplicateItem>,
    /// Player's own posted auctions (Auctions tab). Drives
    /// `C_AuctionHouse.GetNumOwnedAuctions` / `GetOwnedAuctionInfo`.
    /// Empty by default — tests / addons populate via
    /// `A_Admin.AddOwnedAuction`.
    pub auction_owned: Vec<OwnedAuction>,
    /// Player bid rows (Bids tab). Drives `C_AuctionHouse.GetNumBids`
    /// / `GetBidInfo`. Empty by default — tests / addons populate via
    /// `A_Admin.AddAuctionBid`.
    pub auction_bids: Vec<BidAuction>,
    /// Mythic+ probe state. Drives `C_MythicPlus.*` methods. Seeded
    /// with season 14, affix id=9 (Tyrannical), no run history, no
    /// owned key (level 0), no weekly best.
    pub mythic_plus: MythicPlusState,
    /// Character-boost / trial service state.  Drives
    /// `C_CharacterServices.GetActiveCharacterUpgradeBoostType` and
    /// `C_CharacterServices.GetActiveClassTrialBoostType`.  Both
    /// default to None (no active service).
    pub character_services: CharacterServicesState,
    /// Scenario probe state. Drives `C_ScenarioInfo.*` methods.
    /// Defaults to not-in-scenario. Seed via tests to exercise
    /// scenario tracker addons.
    pub scenario: ScenarioState,
    /// Death-recap log. Each entry represents one player death. Drives
    /// `C_DeathRecap.GetKillingBlows` (from the most-recent entry) and
    /// `C_DeathRecap.GetMostRecentDeathRecap`. Empty by default; seed
    /// via tests to exercise the UI.
    pub death_recaps: Vec<DeathRecapEntry>,
    /// Chat bubbles visible in the world. Drives
    /// `C_ChatBubbles.GetAllChatBubbles`. Empty by default; seed via
    /// tests or admin helpers.
    pub chat_bubbles: Vec<ChatBubble>,
    /// Pending summon request. Drives `C_SummonInfo.GetSummonReason`,
    /// `GetSummonConfirmTimeLeft`, `IsSummonSkippingStartExperience`,
    /// `C_IncomingSummon.HasIncomingSummon`, and `IncomingSummonStatus`.
    /// Defaults to inactive.
    pub summon_request: SummonRequestState,
    /// Player's normalized position (0..=1) in the current map.
    /// Drives `C_Map.GetPlayerMapPosition`. Default `(0.5, 0.5)`.
    pub player_map_position: (f64, f64),
    /// Reputation rows in reputation-window display order. Drives
    /// `GetFactionInfoByID`, `GetGuildFactionInfo`, and the selected /
    /// watched faction getters / setters. Empty by default.
    pub factions: Vec<FactionEntry>,
    /// 1-based index into `factions` currently selected in the
    /// reputation window. 0 = nothing selected. Drives
    /// `GetSelectedFaction()` / `SetSelectedFaction()`.
    pub selected_faction_index: i32,
    /// 1-based index into `factions` shown on the XP bar. 0 = none.
    /// Drives `SetWatchedFaction()`.
    pub watched_faction_index: i32,
    /// PvP / battlefield queue state — a single slot for the active
    /// battleground, arena, or LFG queue. See `BattlefieldQueue`.
    pub battlefield_queue: BattlefieldQueue,
    /// Whether the battlefield minimap overlay is visible.
    pub battlefield_minimap_visible: bool,
    /// Chat channels the player has joined, in display order. Position
    /// drives channel numbers (slot 1 = channel #1). Drives the
    /// `Channel*` verb family and `SwapChatChannelLinks`.
    pub chat_channels: Vec<ChatChannel>,
    /// Player macros by 1-based slot index. Drives `PickupMacro`,
    /// `RunMacro`, `EditMacro`, `StopMacro`.
    pub macros: Vec<MacroInfo>,
    /// Macro slot currently executing. `None` when no macro is running.
    pub running_macro: Option<u32>,
    /// Chat window presentation + subscription state keyed by 1-based
    /// chat-frame index (`ChatFrame1` → 1). Windows are lazily created
    /// on first `SetChatWindow*` / `AddChatWindowChannel` call.
    pub chat_windows: ::std::collections::HashMap<i32, ChatWindow>,
    /// Per-chat-type color overrides set by `ChangeChatColor`. Keyed by
    /// the uppercase chat-type token (`"SAY"`, `"CHANNEL"`, etc.).
    pub chat_type_colors: ::std::collections::HashMap<String, (f32, f32, f32)>,
    /// Pending duel opponent name. `Some(name)` when a duel request is
    /// open; cleared by `AcceptDuel` / `DeclineDuel`.
    pub pending_duel: Option<String>,
    /// Pending resurrect offerer name. `Some(name)` when a resurrect
    /// offer is open; `ResurrectGetOfferer` reads it; `AcceptResurrect`
    /// / `DeclineResurrect` clear it.
    pub pending_resurrect: Option<String>,
    /// Whether the player has a corpse waiting to be retrieved at a
    /// graveyard. Cleared by `RetrieveCorpse`.
    pub corpse_available: bool,
    /// Active trade window state. `None` means no trade in progress.
    pub active_trade: Option<TradeState>,
    /// UI panels the player has opened via `Toggle*` verbs. Entries are
    /// the canonical panel token (`"Character"`, `"SpellBook"`, …).
    /// Drives the fallback for panels whose backing frame doesn't exist
    /// yet in the sim.
    pub open_panels: ::std::collections::HashSet<String>,
    /// Whether the current party was formed via LFG. Drives `IsPartyLFG`.
    pub is_party_lfg: bool,
    /// Whether raid-wide "everyone assist" is enabled. Drives
    /// `IsEveryoneAssistant`.
    pub everyone_assistant: bool,
    /// Group leader — `None` means the player is the leader; `Some(i)`
    /// means `party_members[i]` is the leader. Default `None`.
    pub party_leader_index: Option<usize>,
    /// Voice chat volume + mute/deafen/headset state.
    pub voice_chat: VoiceChatState,
    /// Spell IDs the player has learned. Drives `IsSpellKnown` and
    /// `IsSpellKnownOrOverridesKnown`.
    pub known_spells: ::std::collections::HashSet<u32>,
    /// Spell IDs classified as harmful (damage / debuff). Seeded empty;
    /// admin tests can populate before invoking `IsHarmfulSpell`.
    pub harmful_spells: ::std::collections::HashSet<u32>,
    /// Spell IDs classified as helpful (heal / buff). Seeded empty.
    pub helpful_spells: ::std::collections::HashSet<u32>,
    /// Pet spells the player currently has learned (BM Hunter /
    /// Warlock pets). Empty when no pet active. Drives `HasPetSpells`.
    pub pet_spells: ::std::collections::HashSet<u32>,
    /// Most recent honor amount the player gained. Drives
    /// `GetPVPLastHonorGain`. Default 0.
    pub pvp_last_honor_gain: i32,
    /// Item ids classified as equippable. Drives `IsEquippableItem`.
    pub equippable_items: ::std::collections::HashSet<u32>,
    /// Item ids classified as consumable (potions, food, runes…).
    /// Drives `IsConsumableItem`.
    pub consumable_items: ::std::collections::HashSet<u32>,
    /// Whether the player can designate a new Guild Master (retail: GM
    /// can transfer leadership via guild-control UI). Default false.
    pub can_replace_guild_master: bool,
    /// Whether guild invites are auto-declined. Default false.
    pub auto_decline_guild_invites: bool,
    /// Whether the guild roster shows offline members. Default true
    /// (retail's default for new accounts).
    pub guild_roster_show_offline: bool,
    /// Whether the game-system menu (ESC menu) is open. Drives global
    /// `IsMenuOpen`. Default false.
    pub menu_open: bool,
    /// Whether XP gain is disabled for this character. Drives
    /// `IsXPUserDisabled`. Default false.
    pub xp_disabled: bool,
    /// Whether the player can hearthstone / teleport. Drives
    /// `PlayerCanTeleport` and `PlayerHasHearthstone`. Default true
    /// — retail assumes the bag hearthstone until a quest removes it.
    pub can_teleport: bool,
    /// Whether the player has a hearthstone item. Default true.
    pub has_hearthstone: bool,
    /// Append-only log of outbound chat / addon messages sent via
    /// `SendChatMessage` / `SendAddonMessage`. Most recent at the tail.
    pub message_log: Vec<MessageLogEntry>,
    /// User-set keybinding store (base + overrides). See `Keybindings`.
    pub keybindings: Keybindings,
    /// Debug visualization: red borders around elements.
    pub debug_borders: bool,
    /// Debug visualization: green dots at anchor points.
    pub debug_anchors: bool,
}

// Small admin-facing state structs (NetStats, ModifierKeys,
// GameRulesState, GameRuleValue, PetBattleState, LfgListCounts,
// Keybindings) live in `sim_substates.rs`; re-exported here so existing
// `crate::lua_api::state::X` call sites keep working.
pub use super::sim_substates::{
    BattlefieldQueue, BattlefieldStatus, CharacterServicesState, ChatChannel, ChatWindow,
    FactionEntry, GameRuleValue, GameRulesState, GossipOption, GossipQuestRow, GossipState,
    Keybindings, LfgListCounts, LootMethodState, MessageLogEntry, ModifierKeys, MouseButtons,
    NetStats, PetBattlePet, PetBattleState, PetState, QuestLogEntry, QuestLogState, TorghastState,
    TradeState, VoiceChannel, VoiceChatState, VoiceMember, WowLabsAreaInfo, WowLabsCircleInfo,
    WowLabsDataManagerState, WowLabsMatchmakingState, WowLabsPartyInvite, WowLabsPartyMember,
    WowLabsPoint, WowLabsState,
};

#[derive(Default)]
struct EmptyStateCollections {
    console_output: Vec<String>,
    timers: VecDeque<PendingTimer>,
    addons: Vec<AddonInfo>,
    lua_errors: Vec<String>,
    lua_error_records: Vec<LuaErrorRecord>,
    lua_error_counts: HashMap<String, usize>,
    nil_symbol_accesses: Vec<NilSymbolAccess>,
    tooltips: HashMap<u64, TooltipData>,
    blocked_auras_by_unit: HashMap<String, HashSet<i32>>,
    quest_blobs: HashMap<u64, QuestBlobState>,
    fog_of_war_frames: HashMap<u64, FogOfWarFrameState>,
    unit_position_frames: HashMap<u64, UnitPositionFrameState>,
    pending_player_reports: HashMap<i64, PendingPlayerReport>,
    account_store_category_items: HashMap<i64, Vec<i64>>,
    account_store_currency_for_store: HashMap<i64, i64>,
    account_store_currency_info: HashMap<i64, AccountStoreCurrencyInfo>,
    account_store_storefront_state: HashMap<i64, i64>,
    account_store_categories: HashMap<i64, AccountStoreCategoryInfo>,
    account_store_items: HashMap<i64, AccountStoreItemInfo>,
    simple_htmls: HashMap<u64, SimpleHtmlData>,
    message_frames: HashMap<u64, MessageFrameData>,
    animation_groups: HashMap<u64, AnimGroupState>,
    anim_sync_times: HashMap<String, std::time::Duration>,
    anim_frame_to_group: HashMap<u64, u64>,
    anim_frame_to_anim: HashMap<u64, (u64, usize)>,
    on_update_frames: HashSet<u64>,
    pending_hit_grid_changes: Vec<(u64, bool)>,
    pending_texture_preloads: BTreeSet<String>,
    action_bars: HashMap<u32, u32>,
    addon_base_paths: Vec<PathBuf>,
    spell_cooldowns: HashMap<u32, SpellCooldownState>,
    action_ui_buttons: Vec<(u64, u32)>,
    secure_attribute_drivers: HashMap<u64, HashMap<String, String>>,
    party_members: Vec<PartyMember>,
    bag_items: HashMap<(i32, i32), BagItem>,
    tracked_recipes: TrackedRecipes,
    tutorial_flags: HashSet<u32>,
}

impl EmptyStateCollections {
    fn new() -> Self {
        let mut c = Self::empty();
        c.bag_items = default_backpack_items();
        c
    }

    fn empty() -> Self {
        Self::default()
    }
}

/// Seed the `SimState.maps` table with the handful of ui-map ids
/// commonly referenced by Blizzard UI (Azeroth world map, Eastern
/// Kingdoms continent, Stormwind City zone). Retail ids from
/// wago.tools / Wowpedia.
fn default_maps() -> HashMap<i32, MapData> {
    [
        MapData {
            ui_map_id: 946,
            name: "Azeroth".into(),
            map_type: 1,
            parent_map_id: 0,
            art_id: 0,
            flags: 0,
            child_map_ids: vec![13],
        },
        MapData {
            ui_map_id: 13,
            name: "Eastern Kingdoms".into(),
            map_type: 2,
            parent_map_id: 946,
            art_id: 62,
            flags: 0,
            child_map_ids: vec![84],
        },
        MapData {
            ui_map_id: 1,
            name: "Dun Morogh".into(),
            map_type: 3,
            parent_map_id: 0,
            art_id: 12,
            flags: 0,
            child_map_ids: Vec::new(),
        },
        MapData {
            ui_map_id: 84,
            name: "Stormwind City".into(),
            map_type: 3,
            parent_map_id: 13,
            art_id: 104,
            flags: 0,
            child_map_ids: Vec::new(),
        },
        MapData {
            ui_map_id: 2248,
            name: "Isle of Dorn".into(),
            map_type: 3,
            parent_map_id: 0,
            art_id: 5920,
            flags: 0,
            child_map_ids: Vec::new(),
        },
    ]
    .into_iter()
    .map(|m| (m.ui_map_id, m))
    .collect()
}

/// Seed the `SimState.achievements` table with a handful of the
/// commonly-referenced retail achievement ids. Unknown ids are
/// treated as invalid by `IsValidAchievement`.
fn default_achievements() -> HashMap<i32, AchievementInfo> {
    [
        achievement_level_ten(),
        achievement_level_twenty(),
        achievement_level_thirty(),
        achievement_level_forty(),
        achievement_level_fifty(),
        achievement_level_sixty(),
        achievement_explore_elwynn_forest(),
        achievement_explore_eastern_kingdoms(),
        achievement_veteran_of_the_alliance(),
    ]
    .into_iter()
    .map(|a| (a.achievement_id, a))
    .collect()
}

fn achievement_level_ten() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 6,
        name: "Level 10".into(),
        points: 10,
        description: "Reach Level 10.".into(),
        flags: 0,
        icon: 236562,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_level_twenty() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 7,
        name: "Level 20".into(),
        points: 10,
        description: "Reach Level 20.".into(),
        flags: 0,
        icon: 236563,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_level_thirty() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 8,
        name: "Level 30".into(),
        points: 10,
        description: "Reach Level 30.".into(),
        flags: 0,
        icon: 236563,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_level_forty() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 9,
        name: "Level 40".into(),
        points: 10,
        description: "Reach Level 40.".into(),
        flags: 0,
        icon: 236565,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_level_fifty() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 10,
        name: "Level 50".into(),
        points: 10,
        description: "Reach Level 50.".into(),
        flags: 0,
        icon: 236565,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_level_sixty() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 11,
        name: "Level 60".into(),
        points: 10,
        description: "Reach Level 60.".into(),
        flags: 0,
        icon: 236567,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_explore_eastern_kingdoms() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 42,
        name: "Explore Eastern Kingdoms".into(),
        points: 30,
        description: "Explore Eastern Kingdoms, revealing the covered areas of the world map."
            .into(),
        flags: 0,
        icon: 236541,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_explore_elwynn_forest() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 776,
        name: "Explore Elwynn Forest".into(),
        points: 10,
        description: "Explore Elwynn Forest, revealing the covered areas of the world map.".into(),
        flags: 0,
        icon: 236809,
        reward_text: String::new(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: None,
    }
}

fn achievement_veteran_of_the_alliance() -> AchievementInfo {
    AchievementInfo {
        achievement_id: 558,
        name: "Veteran of the Alliance".into(),
        points: 25,
        description: "Earn 100 honorable kills in a single battleground.".into(),
        flags: 0,
        icon: 236412,
        reward_text: "Tabard reward".into(),
        is_guild: false,
        is_statistic: false,
        reward_item_id: Some(43155),
    }
}

/// Seed the `SimState.area_pois` table with one permanent and one
/// time-limited POI so tests can exercise both the nil and the
/// number return paths of `GetAreaPOISecondsLeft`.
fn default_area_pois() -> HashMap<i32, AreaPoiInfo> {
    [
        stormwind_portal_room_poi(),
        legion_invasion_poi(),
        warsong_gulch_poi(),
        cinderbrew_meadery_poi(),
        darkmoon_island_poi(),
    ]
    .into_iter()
    .map(|p| (p.area_poi_id, p))
    .collect()
}

fn stormwind_portal_room_poi() -> AreaPoiInfo {
    AreaPoiInfo {
        area_poi_id: 7000,
        name: "Stormwind Portal Room".into(),
        ui_map_id: Some(84),
        position: (0.52, 0.38),
        atlas_name: Some("Mage-Portal".into()),
        description: Some("Portals to every capital city.".into()),
        faction_id: None,
        icon_widget_set: None,
        linked_ui_map_id: None,
        is_current_event: false,
        should_glow: false,
        seconds_left: None,
    }
}

fn legion_invasion_poi() -> AreaPoiInfo {
    AreaPoiInfo {
        area_poi_id: 7001,
        name: "Legion Invasion".into(),
        ui_map_id: Some(13),
        position: (0.41, 0.62),
        atlas_name: Some("DemonInvasion3".into()),
        description: Some("A demonic incursion.".into()),
        faction_id: None,
        icon_widget_set: None,
        linked_ui_map_id: None,
        is_current_event: true,
        should_glow: true,
        seconds_left: Some(3600),
    }
}

fn warsong_gulch_poi() -> AreaPoiInfo {
    AreaPoiInfo {
        area_poi_id: 1001,
        name: "Warsong Gulch".into(),
        ui_map_id: Some(8685),
        position: (0.452, 0.641),
        atlas_name: Some("worldquest-icon-pvpbattle".into()),
        description: Some("Compete in the current PvP brawl.".into()),
        faction_id: None,
        icon_widget_set: None,
        linked_ui_map_id: None,
        is_current_event: true,
        should_glow: true,
        seconds_left: None,
    }
}

fn cinderbrew_meadery_poi() -> AreaPoiInfo {
    AreaPoiInfo {
        area_poi_id: 1002,
        name: "The Cinderbrew Meadery".into(),
        ui_map_id: Some(1980),
        position: (0.518, 0.274),
        atlas_name: Some("worldquest-icon-worldevent".into()),
        description: Some("A seasonal brewing challenge.".into()),
        faction_id: None,
        icon_widget_set: None,
        linked_ui_map_id: None,
        is_current_event: true,
        should_glow: true,
        seconds_left: None,
    }
}

fn darkmoon_island_poi() -> AreaPoiInfo {
    AreaPoiInfo {
        area_poi_id: 1004,
        name: "Darkmoon Island".into(),
        ui_map_id: Some(5861),
        position: (0.281, 0.734),
        atlas_name: Some("worldquest-icon-tournament".into()),
        description: Some("Take part in the traveling carnival.".into()),
        faction_id: None,
        icon_widget_set: None,
        linked_ui_map_id: None,
        is_current_event: true,
        should_glow: true,
        seconds_left: None,
    }
}

/// Seed `SimState.lfg_category_info` with the two standard retail categories:
/// category 2 = Dungeons, category 3 = Raids.
fn default_lfg_category_info() -> std::collections::HashMap<i32, LfgCategoryInfo> {
    let mut map = std::collections::HashMap::new();
    map.insert(
        2,
        LfgCategoryInfo {
            name: "Dungeons".into(),
            order: 1,
        },
    );
    map.insert(
        3,
        LfgCategoryInfo {
            name: "Raids".into(),
            order: 2,
        },
    );
    map
}

/// Seed the `SimState.auction_browse_results` list with two
/// representative Browse-tab rows (a crafting mat and a gear piece).
fn default_auction_browse_results() -> Vec<AuctionBrowseResult> {
    vec![
        AuctionBrowseResult {
            item_id: 210935,
            item_level: 70,
            min_price: 25_000,
            total_quantity: 400,
            contains_owner_item: false,
        },
        AuctionBrowseResult {
            item_id: 122245,
            item_level: 50,
            min_price: 1_500_000,
            total_quantity: 1,
            contains_owner_item: true,
        },
    ]
}

/// Seed the `SimState.auction_replicate_items` list with two
/// commodity rows so `GetReplicateItemInfo(index)` returns data for
/// both index 1 and 2.
fn default_auction_replicate_items() -> Vec<AuctionReplicateItem> {
    vec![
        AuctionReplicateItem {
            name: "Aqirite".into(),
            texture: 0,
            count: 20,
            quality_id: 2,
            usable: true,
            level: 70,
            level_type: "Item Level".into(),
        },
        AuctionReplicateItem {
            name: "Burnished Helm of Might".into(),
            texture: 133071,
            count: 1,
            quality_id: 3,
            usable: true,
            level: 50,
            level_type: "Item Level".into(),
        },
    ]
}

/// Seed the `SimState.bnet_friends` list with two representative
/// entries: one online Alliance Paladin with two game accounts, and
/// one offline friend. Provides coverage for all five C_BattleNet
/// probes out of the box.
fn default_bnet_friends() -> Vec<BnetFriend> {
    vec![uther_online_friend(), thrall_offline_friend()]
}

fn uther_online_friend() -> BnetFriend {
    BnetFriend {
        friend_index: 1,
        bnet_account_guid: "BNet-0-100001".into(),
        bnet_account_id: 100001,
        battle_tag: "Uther#1000".into(),
        account_name: "Uther".into(),
        note: String::new(),
        custom_message: String::new(),
        custom_message_time: 0,
        appear_offline: false,
        is_battle_tag_friend: true,
        is_friend: true,
        is_favorite: false,
        is_afk: false,
        is_dnd: false,
        last_online_time: 0,
        raf_link_type: 0,
        game_accounts: vec![uther_stormwind_account(), lightbringer_alt_account()],
    }
}

fn uther_stormwind_account() -> BnetGameAccount {
    BnetGameAccount {
        wow_account_guid: "Player-1-00000001".into(),
        game_account_id: 200001,
        character_name: "Uther".into(),
        realm_name: "Stormwind".into(),
        realm_display_name: "Stormwind".into(),
        realm_id: 1,
        class_id: 2,
        class_name: "Paladin".into(),
        character_level: 70,
        area_name: "Stormwind City".into(),
        is_online: true,
        is_game_afk: false,
        is_game_busy: false,
        client_program: "WoW".into(),
        faction_name: "Alliance".into(),
        race_name: "Human".into(),
        rich_presence: "In Stormwind City".into(),
        can_summon: true,
        is_in_current_region: true,
        has_focus: true,
        wow_project_id: 1,
        timerunning_season_id: 0,
        region_id: 1,
        player_guid: String::new(),
    }
}

fn lightbringer_alt_account() -> BnetGameAccount {
    BnetGameAccount {
        wow_account_guid: "Player-1-00000002".into(),
        game_account_id: 200002,
        character_name: "Lightbringer".into(),
        realm_name: "Stormwind".into(),
        realm_display_name: "Stormwind".into(),
        realm_id: 1,
        class_id: 2,
        class_name: "Paladin".into(),
        character_level: 60,
        area_name: "Ironforge".into(),
        is_online: false,
        is_game_afk: false,
        is_game_busy: false,
        client_program: "WoW".into(),
        faction_name: "Alliance".into(),
        race_name: "Dwarf".into(),
        rich_presence: String::new(),
        can_summon: false,
        is_in_current_region: true,
        has_focus: false,
        wow_project_id: 1,
        timerunning_season_id: 0,
        region_id: 1,
        player_guid: String::new(),
    }
}

fn thrall_offline_friend() -> BnetFriend {
    BnetFriend {
        friend_index: 2,
        bnet_account_guid: "BNet-0-100002".into(),
        bnet_account_id: 100002,
        battle_tag: "Thrall#2000".into(),
        account_name: "Thrall".into(),
        note: "old friend".into(),
        custom_message: String::new(),
        custom_message_time: 0,
        appear_offline: false,
        is_battle_tag_friend: true,
        is_friend: true,
        is_favorite: true,
        is_afk: false,
        is_dnd: false,
        last_online_time: 1700000000,
        raf_link_type: 0,
        game_accounts: vec![],
    }
}

/// Seed the `SimState.social_friends` list with three representative
/// WoW friends: two online and one offline. Provides coverage for all
/// C_Social probes out of the box.
fn default_social_friends() -> Vec<SocialFriend> {
    vec![
        SocialFriend {
            name: "Arthax".into(),
            level: 70,
            area: "Stormwind City".into(),
            class_name: "Paladin".into(),
            note: String::new(),
            is_online: true,
            guid: "Player-1-0000A001".into(),
        },
        SocialFriend {
            name: "Durotan".into(),
            level: 65,
            area: "Orgrimmar".into(),
            class_name: "Shaman".into(),
            note: "old guildie".into(),
            is_online: false,
            guid: "Player-2-0000A002".into(),
        },
        SocialFriend {
            name: "Sylvara".into(),
            level: 60,
            area: "Ironforge".into(),
            class_name: "Mage".into(),
            note: String::new(),
            is_online: true,
            guid: "Player-1-0000A003".into(),
        },
    ]
}

/// Default items in bag 0 (backpack) at startup. Slots are 1-based (WoW convention).
fn default_backpack_items() -> HashMap<(i32, i32), BagItem> {
    [
        (
            1,
            BagItem {
                item_id: 6948,
                stack_count: 1,
            },
        ), // Hearthstone
        (
            2,
            BagItem {
                item_id: 159,
                stack_count: 5,
            },
        ), // Refreshing Spring Water
        (
            3,
            BagItem {
                item_id: 4540,
                stack_count: 5,
            },
        ), // Tough Hunk of Bread
        (
            4,
            BagItem {
                item_id: 7005,
                stack_count: 1,
            },
        ), // Skinning Knife
    ]
    .into_iter()
    .map(|(slot, item)| ((0, slot), item))
    .collect()
}

struct EmptyRuntimeState {
    focused_frame_id: Option<u64>,
    visible_on_update_cache: Option<Vec<u64>>,
    strata_buckets: Option<Vec<Vec<u64>>>,
    create_frame_initial_hidden: Option<bool>,
    suppress_runtime_on_load_depth: u32,
    mouse_position: Option<(f32, f32)>,
    hovered_frame: Option<u64>,
    active_drag_frame: Option<u64>,
    active_slider_thumb_drag_frame: Option<u64>,
    mouse_buttons: MouseButtons,
    next_report_token: i64,
    party_group_active: bool,
    current_target: Option<TargetInfo>,
    current_focus: Option<TargetInfo>,
    sound_manager: Option<SoundManager>,
    last_sound_kit_requested: Option<u32>,
    last_sound_file_requested: Option<String>,
    last_stopped_sound_handle: Option<u32>,
    last_launched_url: Option<String>,
    highlighted_map_scene_character_guid: Option<String>,
    multi_action_bar_grids_shown: bool,
    casting: Option<CastingState>,
    channeling: Option<CastingState>,
    gcd: Option<(f64, f64)>,
    cursor_item: Option<CursorInfo>,
    loading_addon_index: Option<u16>,
    loading_addon_stack: Vec<u16>,
    executing_addon_index: Option<u16>,
    xml_load_addon_depth: u32,
    loading_forbidden: bool,
    next_anim_group_id: u64,
    next_cast_id: u32,
    screen_width: f32,
    screen_height: f32,
    screen_kind: ScreenKind,
    is_logged_in: bool,
    screen_first_displayed: bool,
    saved_account_name: String,
    saved_account_list: String,
    uses_token: bool,
    account_save_enabled: bool,
    account_save_in_progress: bool,
    account_locked_post_save: bool,
    last_account_store_purchase_request: Option<i64>,
    account_store_begin_purchase_succeeds: bool,
    last_account_store_refund_request: Option<i64>,
    account_store_refund_succeeds: bool,
    last_account_store_storefront_info_request: Option<i64>,
    fps: f32,
    rot_damage_level: usize,
    start_time: Instant,
}

macro_rules! build_empty_runtime_state {
    (
        start_time: $start_time:expr,
        next_report_token: $next_report_token:expr,
        next_anim_group_id: $next_anim_group_id:expr,
        next_cast_id: $next_cast_id:expr,
        screen_width: $screen_width:expr,
        screen_height: $screen_height:expr
    ) => {
        EmptyRuntimeState {
            focused_frame_id: None,
            visible_on_update_cache: None,
            strata_buckets: None,
            create_frame_initial_hidden: None,
            suppress_runtime_on_load_depth: 0,
            mouse_position: None,
            hovered_frame: None,
            active_drag_frame: None,
            active_slider_thumb_drag_frame: None,
            mouse_buttons: MouseButtons::default(),
            next_report_token: $next_report_token,
            party_group_active: false,
            current_target: None,
            current_focus: None,
            sound_manager: None,
            last_sound_kit_requested: None,
            last_sound_file_requested: None,
            last_stopped_sound_handle: None,
            last_launched_url: None,
            highlighted_map_scene_character_guid: None,
            multi_action_bar_grids_shown: false,
            casting: None,
            channeling: None,
            gcd: None,
            cursor_item: None,
            loading_addon_index: None,
            loading_addon_stack: Vec::new(),
            executing_addon_index: None,
            xml_load_addon_depth: 0,
            loading_forbidden: false,
            next_anim_group_id: $next_anim_group_id,
            next_cast_id: $next_cast_id,
            screen_width: $screen_width,
            screen_height: $screen_height,
            screen_kind: ScreenKind::Game,
            is_logged_in: false,
            screen_first_displayed: false,
            saved_account_name: String::new(),
            saved_account_list: String::new(),
            uses_token: false,
            account_save_enabled: false,
            account_save_in_progress: false,
            account_locked_post_save: false,
            last_account_store_purchase_request: None,
            account_store_begin_purchase_succeeds: true,
            last_account_store_refund_request: None,
            account_store_refund_succeeds: true,
            last_account_store_storefront_info_request: None,
            fps: 0.0,
            rot_damage_level: 0,
            start_time: $start_time,
        }
    };
}

const INITIAL_REPORT_TOKEN: i64 = 1;
const INITIAL_ANIM_GROUP_ID: u64 = 1;
const INITIAL_CAST_ID: u32 = 1;
const DEFAULT_SCREEN_WIDTH: f32 = 1600.0;
const DEFAULT_SCREEN_HEIGHT: f32 = 1200.0;

impl EmptyRuntimeState {
    fn new() -> Self {
        build_initialized_empty_runtime_state(Instant::now())
    }
}

fn build_initialized_empty_runtime_state(start_time: Instant) -> EmptyRuntimeState {
    build_empty_runtime_state!(
        start_time: start_time,
        next_report_token: INITIAL_REPORT_TOKEN,
        next_anim_group_id: INITIAL_ANIM_GROUP_ID,
        next_cast_id: INITIAL_CAST_ID,
        screen_width: DEFAULT_SCREEN_WIDTH,
        screen_height: DEFAULT_SCREEN_HEIGHT
    )
}

impl Default for SimState {
    fn default() -> Self {
        let mut state = Self::new_empty();
        state.seed_default_game_state();
        state
    }
}

impl SimState {
    fn seed_default_game_state(&mut self) {
        self.action_bars = default_action_bars();
        self.party_members = default_party();
        self.party_group_active = false;
        crate::lua_api::globals::keybindings::init_keybindings(self);
        self.player.name = random_player_name();
        self.player.power = 50_000;
        self.player.power_max = 100_000;
        self.player.buffs = default_player_buffs();
    }

    fn new_empty() -> Self {
        Self::build_empty_state(EmptyStateCollections::new(), EmptyRuntimeState::new())
    }

    fn build_empty_state(c: EmptyStateCollections, r: EmptyRuntimeState) -> Self {
        build_empty_sim_state!(c, r)
    }

    /// Look up bag item at (bag, slot). Returns (item_id, stack_count).
    pub fn get_bag_item(&self, bag: i32, slot: i32) -> Option<(u32, i32)> {
        self.bag_items
            .get(&(bag, slot))
            .map(|i| (i.item_id, i.stack_count))
    }

    /// Count occupied slots in a bag.
    pub fn bag_occupied_slots(&self, bag: i32) -> i32 {
        self.bag_items.keys().filter(|(b, _)| *b == bag).count() as i32
    }

    pub fn set_screen_kind(&mut self, screen_kind: ScreenKind) {
        self.screen_kind = screen_kind;
        self.screen_first_displayed = false;
        if screen_kind.is_glue() {
            self.is_logged_in = false;
        }
    }

    pub fn set_mouse_position(&mut self, pos: Option<(f32, f32)>) {
        self.mouse_position = pos;
        let Some((mx, my)) = pos else {
            return;
        };
        let cursor_tooltips = self.collect_cursor_tooltip_positions(mx, my);
        for (tooltip_id, anchor) in cursor_tooltips {
            self.reanchor_tooltip_to_cursor(tooltip_id, anchor);
        }
    }

    pub fn set_active_drag_frame(&mut self, frame_id: Option<u64>) {
        self.active_drag_frame = frame_id;
    }

    pub fn set_active_slider_thumb_drag_frame(&mut self, frame_id: Option<u64>) {
        self.active_slider_thumb_drag_frame = frame_id;
    }

    pub fn set_mouse_button_down(&mut self, button: &str, down: bool) -> bool {
        self.mouse_buttons.set_down(button, down)
    }

    pub fn enqueue_texture_preloads<I>(&mut self, paths: I)
    where
        I: IntoIterator<Item = String>,
    {
        self.pending_texture_preloads.extend(paths);
    }

    pub fn drain_texture_preloads(&mut self) -> Vec<String> {
        std::mem::take(&mut self.pending_texture_preloads)
            .into_iter()
            .collect()
    }

    fn collect_cursor_tooltip_positions(&self, mx: f32, my: f32) -> Vec<(u64, Anchor)> {
        self.tooltips
            .iter()
            .filter(|(_, td)| td.anchor_type == "ANCHOR_CURSOR")
            .map(|(&tooltip_id, td)| {
                (
                    tooltip_id,
                    build_cursor_anchor(mx, my, td.anchor_x_offset, td.anchor_y_offset),
                )
            })
            .collect()
    }

    fn reanchor_tooltip_to_cursor(&mut self, tooltip_id: u64, anchor: Anchor) {
        let Some(frame) = self.widgets.get_mut_visual(tooltip_id) else {
            return;
        };
        frame.anchors.clear();
        frame.anchors.push(anchor);
        let _ = frame;
        self.widgets.mark_rect_dirty(tooltip_id);
        self.widgets.mark_visual_dirty(tooltip_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn empty_runtime_state_new_seeds_expected_runtime_defaults() {
        let state = EmptyRuntimeState::new();

        assert_eq!(state.next_report_token, INITIAL_REPORT_TOKEN);
        assert_eq!(state.next_anim_group_id, INITIAL_ANIM_GROUP_ID);
        assert_eq!(state.next_cast_id, INITIAL_CAST_ID);
        assert_eq!(state.screen_width, DEFAULT_SCREEN_WIDTH);
        assert_eq!(state.screen_height, DEFAULT_SCREEN_HEIGHT);
        assert_eq!(state.screen_kind, ScreenKind::Game);
        assert!(!state.is_logged_in);
        assert!(!state.screen_first_displayed);
        assert!(state.focused_frame_id.is_none());
        assert!(state.hovered_frame.is_none());
        assert!(state.saved_account_name.is_empty());
        assert!(state.saved_account_list.is_empty());
        assert!(state.start_time.elapsed() < Duration::from_secs(1));
    }
}
