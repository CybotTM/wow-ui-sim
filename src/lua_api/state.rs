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
            action_bars: $collections.action_bars,
            addon_base_paths: $collections.addon_base_paths,
            create_frame_initial_hidden: $runtime.create_frame_initial_hidden,
            suppress_runtime_on_load_depth: $runtime.suppress_runtime_on_load_depth,
            mouse_position: $runtime.mouse_position,
            hovered_frame: $runtime.hovered_frame,
            active_drag_frame: $runtime.active_drag_frame,
            active_slider_thumb_drag_frame: $runtime.active_slider_thumb_drag_frame,
            next_report_token: $runtime.next_report_token,
            party_members: $collections.party_members,
            party_group_active: $runtime.party_group_active,
            current_target: $runtime.current_target,
            previous_target: None,
            current_focus: $runtime.current_focus,
            enemy_pool: Vec::new(),
            sound_manager: $runtime.sound_manager,
            rot_damage_level: $runtime.rot_damage_level,
            fps: $runtime.fps,
            start_time: $runtime.start_time,
            casting: $runtime.casting,
            next_cast_id: $runtime.next_cast_id,
            gcd: $runtime.gcd,
            spell_cooldowns: $collections.spell_cooldowns,
            inventory_item_cooldowns: ::std::collections::HashMap::new(),
            action_ui_buttons: $collections.action_ui_buttons,
            cursor_item: $runtime.cursor_item,
            loading_addon_index: $runtime.loading_addon_index,
            executing_addon_index: $runtime.executing_addon_index,
            loading_forbidden: $runtime.loading_forbidden,
            app_frame_metrics: AppFrameMetrics::default(),
            talents: super::talent_state::TalentState::new(),
            lua_errors: $collections.lua_errors,
            lua_error_records: $collections.lua_error_records,
            lua_error_counts: $collections.lua_error_counts,
            nil_symbol_accesses: $collections.nil_symbol_accesses,
            global_show_hide_depth: 0,
            anim_sync_times: $collections.anim_sync_times,
            player: PlayerState::default(),
            world: super::state_types::seeded_world_state(),
            bag_items: $collections.bag_items,
            tracked_recipes: $collections.tracked_recipes,
            net_stats: NetStats::default(),
            store_frame_shown: false,
            timerunning_season_id: None,
            modifier_keys: ModifierKeys::default(),
            game_rules: GameRulesState::default(),
            housing_service_enabled: false,
            pet_battles: PetBattleState::default(),
            pet: PetState::default(),
            lfg_list_counts: LfgListCounts::default(),
            can_use_premade_group: false,
            photo_sharing_authorized: false,
            photo_sharing_enabled: false,
            tutorial_flags: $collections.tutorial_flags,
            wowlabs: WowLabsState::default(),
            quest_log: Vec::new(),
            pending_quest_offer: None,
            quest_choice_id: None,
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
pub use super::game_data::SpellCooldownState;
pub use super::game_data::{
    CLASS_LABELS, CastingState, PartyMember, RACE_DATA, ROT_DAMAGE_LEVELS, TargetInfo, XP_LEVELS,
    tick_party_health,
};
use super::game_data::{
    default_action_bars, default_party, default_player_buffs, random_player_name,
};
pub use super::state_types::{
    AddonInfo, AddonRuntimeMetrics, AppFrameMetrics, BagItem, CursorInfo, CursorItemOrigin,
    EquippedItem, GreatVaultActivity, GuildMember, GuildRank, LootRollInfo, LuaErrorRecord,
    MacroInfo, MovementState, NilSymbolAccess, PendingTimer, PlayerState, SecondaryPowerState,
    WorldState,
};
pub use super::tracked_recipes::TrackedRecipes;

// Per-frame side-table state (quest blobs, UnitPositionFrame, etc.)
// lives in `frame_substates.rs`; re-exported for existing
// `crate::lua_api::state::X` call sites.
pub use super::frame_substates::{
    FogOfWarFrameState, PendingPlayerReport, QuestBlobState, UnitPositionFrameState,
    UnitPositionPlayerPingTexture, UnitPositionUnit,
};

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
    /// Action bar slots: slot (1-120) → spell ID.
    pub action_bars: HashMap<u32, u32>,
    /// Addon base paths for runtime on-demand loading (Blizzard UI + AddOns directories).
    pub addon_base_paths: Vec<PathBuf>,
    /// One-shot override for XML frame creation: whether the next CreateFrame
    /// should start hidden before registration/render eligibility.
    pub create_frame_initial_hidden: Option<bool>,
    /// Depth-counted suppression for runtime CreateFrame OnLoad firing while
    /// XML loader code is still building the frame tree.
    pub suppress_runtime_on_load_depth: u32,
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
    /// Rot damage intensity (index into ROT_DAMAGE_LEVELS).
    pub rot_damage_level: usize,
    /// Current framerate (FPS), updated by the app's FPS counter.
    pub fps: f32,
    /// Instant at which the UI started (used by GetTime and message timestamps).
    pub start_time: Instant,
    /// Active spell cast (None = not casting).
    pub casting: Option<CastingState>,
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
    /// World state (zone, instance, guild, collections, vault, loot).
    pub world: WorldState,
    /// Bags/Inventory: (bag_index, slot_index) → BagItem.
    pub bag_items: HashMap<(i32, i32), BagItem>,
    /// Tracked recipes for the Profession Recipe Tracker, keyed by
    /// `is_recrafting`. Drives `C_TradeSkillUI.GetRecipesTracked` /
    /// `IsRecipeTracked` / `SetRecipeTracked`. Empty by default.
    pub tracked_recipes: TrackedRecipes,
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
    /// `C_GameRules` backing state — active game mode + glue-screen name +
    /// a rules map. Default: Standard mode, `CharacterSelect` glue screen,
    /// empty rules.
    pub game_rules: GameRulesState,
    /// Whether `C_Housing.IsHousingServiceEnabled()` reports true. Drives
    /// MainMenuBarMicroButtons' decision to render the Housing micro-button.
    /// Default false (sim has no housing service).
    pub housing_service_enabled: bool,
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
    /// Pending quest offer displayed in the quest detail frame. Consumed
    /// by `ConfirmAcceptQuest`. `None` means no pending offer.
    pub pending_quest_offer: Option<u32>,
    /// Active quest-choice dialog id set by `QuestChoiceFrame_SetActiveChoice`.
    pub quest_choice_id: Option<u32>,
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
    BattlefieldQueue, BattlefieldStatus, ChatChannel, ChatWindow, GameRuleValue, GameRulesState,
    Keybindings, LfgListCounts, LootMethodState, MessageLogEntry, ModifierKeys, NetStats,
    PetBattleState, PetState,
    TradeState, VoiceChatState, WowLabsAreaInfo, WowLabsCircleInfo, WowLabsDataManagerState,
    WowLabsMatchmakingState, WowLabsPartyInvite, WowLabsPartyMember, WowLabsPoint, WowLabsState,
};

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
    party_members: Vec<PartyMember>,
    bag_items: HashMap<(i32, i32), BagItem>,
    tracked_recipes: TrackedRecipes,
    tutorial_flags: HashSet<u32>,
}

impl EmptyStateCollections {
    fn new() -> Self {
        Self {
            console_output: Vec::new(),
            timers: VecDeque::new(),
            addons: Vec::new(),
            lua_errors: Vec::new(),
            lua_error_records: Vec::new(),
            lua_error_counts: HashMap::new(),
            nil_symbol_accesses: Vec::new(),
            tooltips: HashMap::new(),
            blocked_auras_by_unit: HashMap::new(),
            quest_blobs: HashMap::new(),
            fog_of_war_frames: HashMap::new(),
            unit_position_frames: HashMap::new(),
            pending_player_reports: HashMap::new(),
            simple_htmls: HashMap::new(),
            message_frames: HashMap::new(),
            animation_groups: HashMap::new(),
            anim_sync_times: HashMap::new(),
            anim_frame_to_group: HashMap::new(),
            anim_frame_to_anim: HashMap::new(),
            on_update_frames: HashSet::new(),
            pending_hit_grid_changes: Vec::new(),
            pending_texture_preloads: BTreeSet::new(),
            action_bars: HashMap::new(),
            addon_base_paths: Vec::new(),
            spell_cooldowns: HashMap::new(),
            action_ui_buttons: Vec::new(),
            party_members: Vec::new(),
            bag_items: default_backpack_items(),
            tracked_recipes: TrackedRecipes::default(),
            tutorial_flags: HashSet::new(),
        }
    }
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
    next_report_token: i64,
    party_group_active: bool,
    current_target: Option<TargetInfo>,
    current_focus: Option<TargetInfo>,
    sound_manager: Option<SoundManager>,
    casting: Option<CastingState>,
    gcd: Option<(f64, f64)>,
    cursor_item: Option<CursorInfo>,
    loading_addon_index: Option<u16>,
    executing_addon_index: Option<u16>,
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
            next_report_token: $next_report_token,
            party_group_active: false,
            current_target: None,
            current_focus: None,
            sound_manager: None,
            casting: None,
            gcd: None,
            cursor_item: None,
            loading_addon_index: None,
            executing_addon_index: None,
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
