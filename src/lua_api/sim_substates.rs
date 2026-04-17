//! Small admin-facing SimState sub-structs — separated from
//! `state.rs` so the main `SimState` definition + default/new machinery
//! stays readable.
//!
//! Everything here is a data holder owned by `SimState`: the public
//! API surface for each struct is defined by `A_Admin.Set*` setters
//! and the corresponding `C_*`/global probes in `lua_api::globals`.

use std::collections::HashMap;

/// Simulated network statistics returned by `GetNetStats()`.
///
/// WoW's real `GetNetStats` returns `(bandwidthIn, bandwidthOut, latencyHome,
/// latencyWorld)` in (kB/s, kB/s, ms, ms). The sim has no socket, so these are
/// purely a state knob — tests set values via the admin API to drive UI code
/// that renders latency/bandwidth indicators.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct NetStats {
    pub bandwidth_in_kbps: f64,
    pub bandwidth_out_kbps: f64,
    pub latency_home_ms: f64,
    pub latency_world_ms: f64,
}

/// Modifier-key down state. `IsModifierKeyDown()` returns true iff any of
/// shift/control/alt is held — matches real WoW's inclusive-or semantic,
/// excluding the meta key (meta tests via the dedicated `IsMetaKeyDown`).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ModifierKeys {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub meta: bool,
}

impl ModifierKeys {
    /// True iff shift, control, or alt is currently down. Does not include
    /// meta — WoW keeps that on its own `IsMetaKeyDown()` probe.
    pub fn any_modifier(&self) -> bool {
        self.shift || self.control || self.alt
    }
}

/// Backing state for the `C_GameRules` namespace. WoW's retail client
/// exposes a handful of named game rules (`"DISABLE_DUELS"`,
/// `"ALLOW_PING_PARTY_MEMBERS"`, etc.) that the UI queries to decide which
/// features to surface. Each rule has a float / int / string representation;
/// we store all three on one entry so a single rule can satisfy all three
/// getter variants without round-tripping strings.
#[derive(Debug, Clone, PartialEq)]
pub struct GameRulesState {
    /// Currently-active game mode id. Matches `Enum.GameMode`:
    /// `0 = Standard`, `1 = Plunderstorm`, `2 = Delves`, etc. Tests that don't
    /// care treat nonzero as "some non-standard mode".
    pub active_game_mode: i32,
    /// Glue screen name the current game mode opens on at the login flow.
    /// Default `"CharacterSelect"` (Standard).
    pub glue_screen_name: String,
    /// Sparse rule store keyed by rule name. Missing key = inactive.
    pub rules: HashMap<String, GameRuleValue>,
}

impl Default for GameRulesState {
    fn default() -> Self {
        Self {
            active_game_mode: 0,
            glue_screen_name: "CharacterSelect".into(),
            rules: HashMap::new(),
        }
    }
}

/// A single `C_GameRules` rule value. Stored as all three interpretations
/// (float/int/string) so each getter returns the "correct" form without a
/// parse step. Admin `A_Admin.SetGameRule(name, value)` fills the right
/// fields based on the Lua type passed in.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GameRuleValue {
    pub as_float: f64,
    pub as_int: i64,
    pub as_string: String,
}

/// Pet-battle state backing `C_PetBattles.GetNumPets(owner)` /
/// `GetBattleState()`. WoW's `owner` argument is 1 (player) or 2 (enemy);
/// other values return 0. `battle_state` mirrors
/// `Enum.PetbattleState` — default 0 (`PVEInvitationSent` / "no active
/// battle"). Non-zero = some battle phase is active.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PetBattleState {
    pub num_pets_player: i32,
    pub num_pets_enemy: i32,
    pub battle_state: i32,
}

/// Hunter / warlock pet state — drives the four legacy pet-stat
/// probes (`GetPetExperience` / `GetPetHappiness` / `GetPetLoyalty` /
/// `GetPetTimeInCombat`).
///
/// Modern WoW no longer exposes happiness / loyalty for hunter pets
/// (they were removed in Cataclysm), so the sim defaults all four
/// fields to 0 / empty. Addons that still query these probes receive
/// retail's modern shape unless a test seeds the struct.
#[derive(Debug, Default, Clone)]
pub struct PetState {
    /// Current XP within the pet's level. Drives `GetPetExperience`
    /// (first return).
    pub xp: i32,
    /// XP required for the pet to ding. Drives `GetPetExperience`
    /// (second return).
    pub xp_max: i32,
    /// Classic-era happiness level (1 = Unhappy, 2 = Content,
    /// 3 = Happy). Drives `GetPetHappiness` (first return). 0 = no pet.
    pub happiness: i32,
    /// Classic-era damage percentage (75/100/125 depending on
    /// happiness). Drives `GetPetHappiness` (second return).
    pub damage_percent: i32,
    /// Classic-era loyalty gain rate (internal counter). Drives
    /// `GetPetHappiness` (third return).
    pub loyalty_rate: i32,
    /// Classic-era loyalty level as a localized string (e.g.
    /// `"Loyal"`). Drives `GetPetLoyalty`. Empty when no pet.
    pub loyalty_label: String,
    /// Seconds the pet has been in combat. Drives `GetPetTimeInCombat`.
    pub time_in_combat: i32,
}

/// A single chat / addon message entry in the simulator's outbound log.
/// `kind` is `"chat"` for `SendChatMessage` or `"addon"` for
/// `SendAddonMessage`.
#[derive(Debug, Clone)]
pub struct MessageLogEntry {
    pub kind: String,
    pub prefix: String,
    pub message: String,
    pub channel: String,
    pub target: String,
}

/// Voice chat presentation state: volume sliders + mute/deafen flags.
/// Volumes are `[0.0, 1.0]`. `headset_mode` reflects whether the
/// headset-check confirmation dialog has been accepted.
#[derive(Debug, Clone)]
pub struct VoiceChatState {
    pub microphone_volume: f32,
    pub output_volume: f32,
    pub muted: bool,
    pub deafened: bool,
    pub headset_mode: bool,
    /// Whether voice chat is enabled in settings. Drives `IsVoiceEnabled`.
    /// Default true — retail ships with voice chat enabled by default.
    pub enabled: bool,
    /// Whether the player is actively using voice chat (connected and
    /// in a channel). Drives `IsUsingVoiceChat`. Default false.
    pub using: bool,
    /// Whether the voice-chat client is currently establishing a
    /// connection. Drives `VoiceChat_IsConnecting`.
    pub connecting: bool,
    /// Whether the local player is currently transmitting voice.
    /// Drives `VoiceChat_IsTalking`.
    pub talking: bool,
}

impl Default for VoiceChatState {
    fn default() -> Self {
        Self {
            microphone_volume: 1.0,
            output_volume: 1.0,
            muted: false,
            deafened: false,
            headset_mode: false,
            enabled: true,
            using: false,
            connecting: false,
            talking: false,
        }
    }
}

/// Active gossip-dialog state. Retail fires `GOSSIP_SHOW` when a
/// gossip window opens against an NPC and `GOSSIP_CLOSED` when the
/// player walks away. The sim exposes three count knobs that probes
/// return + an `active` flag so listeners know whether a dialog is
/// up (the `open` half of the event pair).
#[derive(Debug, Default, Clone)]
pub struct GossipState {
    pub active: bool,
    pub num_options: i32,
    pub num_available_quests: i32,
    pub num_active_quests: i32,
}

/// Single reputation-tab row. Drives `GetFactionInfoByID(id)` and the
/// broader reputation window. `standing` is the 1-8 reputation tier
/// (1 Hated .. 8 Exalted). `bottom` / `top` bracket the current
/// standing's value range; `earned` is how far through that bracket
/// the player is. `at_war` / `is_watched` / `is_header` /
/// `is_collapsed` carry the flag bits that addons destructure.
#[derive(Debug, Default, Clone)]
pub struct FactionEntry {
    pub faction_id: u32,
    pub name: String,
    pub description: String,
    pub standing: i32,
    pub bottom: i64,
    pub top: i64,
    pub earned: i64,
    pub at_war: bool,
    pub can_toggle_at_war: bool,
    pub is_header: bool,
    pub is_collapsed: bool,
    pub has_rep: bool,
    pub is_watched: bool,
    pub is_child: bool,
    pub has_bonus_rep_gain: bool,
    pub can_be_lfg_bonus: bool,
}

/// Party loot-method state — drives `GetLootMethod()` and
/// `GetMasterLooterThreshold()`. `method` is the retail token
/// (`"group"`, `"master"`, `"freeforall"`, `"roundrobin"`,
/// `"needbeforegreed"`, `"personalloot"`). `party_master_index` /
/// `raid_master_index` are 1-based loot-candidate indices that matter
/// only when `method == "master"`; 0 means "no master assigned".
/// `threshold` is the master-loot item-quality threshold: 0 Poor ..
/// 4 Epic.
#[derive(Debug, Clone)]
pub struct LootMethodState {
    pub method: String,
    pub party_master_index: i32,
    pub raid_master_index: i32,
    pub threshold: i32,
}

impl Default for LootMethodState {
    fn default() -> Self {
        Self {
            // Retail's modern default is personal loot; classic-era
            // addons still probe GetLootMethod, so keep the token there.
            method: "personalloot".into(),
            party_master_index: 0,
            raid_master_index: 0,
            threshold: 2, // Uncommon — retail's default master-loot threshold.
        }
    }
}

/// Active trade window state. `None` means no trade in progress.
#[derive(Debug, Default, Clone)]
pub struct TradeState {
    /// Opponent's display name.
    pub target: String,
    /// Item ids in the 7 player trade slots (retail has 7 including the
    /// non-tradable slot). `0` means empty.
    pub player_slots: [u32; 7],
    /// Item ids in the 7 opponent trade slots.
    pub target_slots: [u32; 7],
    /// Copper offered by the player.
    pub player_money: u64,
    /// Copper offered by the opponent.
    pub target_money: u64,
    /// Whether the player has pressed Accept.
    pub player_accepted: bool,
    /// Whether the opponent has pressed Accept.
    pub target_accepted: bool,
}

/// A chat window's presentation + subscription state. Keyed by the
/// window's 1-based chat-frame index (e.g. ChatFrame1 → index 1).
/// Drives the `SetChatWindow*` / `AddChatWindowChannel` /
/// `GetChatWindow*` verb family.
#[derive(Debug, Clone)]
pub struct ChatWindow {
    pub alpha: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub locked: bool,
    pub uninteractable: bool,
    /// Channels subscribed to this window, in insertion order.
    pub channels: Vec<String>,
    /// Message-type names this window receives (e.g. `"SAY"`, `"YELL"`).
    pub messages: Vec<String>,
}

impl Default for ChatWindow {
    fn default() -> Self {
        Self {
            alpha: 1.0,
            r: 1.0,
            g: 1.0,
            b: 1.0,
            locked: false,
            uninteractable: false,
            channels: Vec::new(),
            messages: Vec::new(),
        }
    }
}

/// A joined chat channel with its membership, moderators, and ban list.
/// Matches the shape retail addons expect when iterating the player's
/// channel list.
#[derive(Debug, Default, Clone)]
pub struct ChatChannel {
    /// Display name (e.g. `"General"`, `"Trade"`, a custom name).
    pub name: String,
    /// Channel members by display name. The local player is implicit.
    pub members: ::std::collections::BTreeSet<String>,
    /// Names granted moderator status in this channel.
    pub moderators: ::std::collections::BTreeSet<String>,
    /// Names banned from this channel — banning also removes membership.
    pub banned: ::std::collections::BTreeSet<String>,
}

/// Battlefield / arena / LFG queue state. A single slot models the
/// active queue — retail supports multiple concurrent slots, the sim
/// does not need them yet. `index` is the 1-based slot id callers
/// reference (e.g. `GetBattlefieldStatus(i)`); `name` is the
/// human-readable queue name for display.
#[derive(Debug, Default, Clone)]
pub struct BattlefieldQueue {
    pub status: BattlefieldStatus,
    pub index: i32,
    pub name: String,
}

/// Battlefield-queue status machine. Mirrors retail's
/// `GetBattlefieldStatus(index)` return strings — callers compare to
/// `"none" / "queued" / "confirm" / "active"`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BattlefieldStatus {
    #[default]
    None,
    Queued,
    Confirm,
    Active,
}

impl BattlefieldStatus {
    /// Canonical WoW status string returned by `GetBattlefieldStatus`.
    pub fn as_wow_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Queued => "queued",
            Self::Confirm => "confirm",
            Self::Active => "active",
        }
    }
}

/// LFG-list counts backing `C_LFGList.GetNumApplications()` and
/// `GetNumApplicants()`. Each returns `(total, viewed)` — shape matters
/// because callers destructure both values in one statement.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LfgListCounts {
    pub applications_total: i32,
    pub applications_viewed: i32,
    pub applicants_total: i32,
    pub applicants_viewed: i32,
}

/// Minimal keybinding store. In retail WoW the binding registry is
/// populated from `Bindings.xml` at load; here we only back the
/// *user-set* side (`SetBinding(key, action)` / `SetOverrideBinding`)
/// because the sim has no Bindings.xml to pre-register from. Overrides
/// shadow the base bindings during lookup, matching WoW's
/// `GetBindingAction(key, checkOverride=true)` semantics.
#[derive(Debug, Default, Clone)]
pub struct Keybindings {
    /// Insertion-ordered base bindings set by `SetBinding(key, action)`.
    /// Keyed by key name; the value is the bound action. Actions can be
    /// bound to at most 2 keys (WoW's documented limit).
    pub base: Vec<(String, String)>,
    /// Override bindings set by `SetOverrideBinding(owner, isPriority, key, action)`.
    /// Stored as a flat list so `ClearOverrideBindings` can drop the
    /// whole set.
    pub overrides: Vec<(String, String)>,
}

impl Keybindings {
    /// Return up to 2 keys currently bound to `action`. Overrides take
    /// precedence over base — an overridden key shadows its base entry.
    pub fn keys_for_action(&self, action: &str) -> (Option<String>, Option<String>) {
        let mut found: Vec<String> = self
            .overrides
            .iter()
            .filter(|(_, a)| a == action)
            .map(|(k, _)| k.clone())
            .collect();
        for (k, a) in &self.base {
            if a == action && self.overrides.iter().all(|(ok, _)| ok != k) {
                found.push(k.clone());
            }
        }
        let first = found.first().cloned();
        let second = found.get(1).cloned();
        (first, second)
    }

    /// Return the action bound to `key`, preferring an override. Empty
    /// string when unbound — WoW returns `""` (not nil) for
    /// `GetBindingAction`.
    pub fn action_for_key(&self, key: &str) -> String {
        if let Some((_, a)) = self.overrides.iter().rev().find(|(k, _)| k == key) {
            return a.clone();
        }
        self.base
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, a)| a.clone())
            .unwrap_or_default()
    }

    /// Bind `key` to `action`. An empty action unbinds `key`. A second
    /// key binding to the same action evicts the oldest key still bound
    /// (WoW's 2-keys-per-action limit).
    pub fn set(&mut self, key: &str, action: &str) {
        self.base.retain(|(k, _)| k != key);
        if action.is_empty() {
            return;
        }
        let bound: Vec<usize> = self
            .base
            .iter()
            .enumerate()
            .filter(|(_, (_, a))| a == action)
            .map(|(i, _)| i)
            .collect();
        if bound.len() >= 2 {
            self.base.remove(bound[0]);
        }
        self.base.push((key.to_string(), action.to_string()));
    }

    /// Install an override for `key` → `action`. Does NOT touch base.
    pub fn set_override(&mut self, key: &str, action: &str) {
        self.overrides.retain(|(k, _)| k != key);
        if !action.is_empty() {
            self.overrides.push((key.to_string(), action.to_string()));
        }
    }

    /// Drop every override; base bindings unaffected.
    pub fn clear_overrides(&mut self) {
        self.overrides.clear();
    }
}

/// Minimal WoW Labs / Plunderstorm matchmaking member record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WowLabsPartyMember {
    pub player_name: String,
    pub party_member_guid: String,
    pub is_local_player: bool,
    pub is_party_leader: bool,
    pub is_ready: bool,
}

/// Pending invite visible through `C_WoWLabsMatchmaking.GetPartyInviteByIndex`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WowLabsPartyInvite {
    pub inviter_name: String,
    pub inviter_guid: String,
    pub invite_id: String,
}

/// World-map area choice exposed by `C_WowLabsDataManager`.
#[derive(Debug, Clone, PartialEq)]
pub struct WowLabsAreaInfo {
    pub wow_labs_area_id: i32,
    pub x: f64,
    pub y: f64,
    pub area_type: i32,
}

/// 2D point payload used by `PushCircleInfoToLua`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WowLabsPoint {
    pub x: f64,
    pub y: f64,
}

/// Plunderstorm shrinking-circle values returned to Lua.
#[derive(Debug, Clone, PartialEq)]
pub struct WowLabsCircleInfo {
    pub start_lerp_time: f64,
    pub time_to_lerp: f64,
    pub outer_position: WowLabsPoint,
    pub inner_position: WowLabsPoint,
    pub base_radius: f64,
    pub outer_scale: f64,
    pub inner_scale: f64,
    pub prediction_position: WowLabsPoint,
    pub prediction_scale: f64,
    pub initial_base_size: f64,
}

impl Default for WowLabsCircleInfo {
    fn default() -> Self {
        Self {
            start_lerp_time: 12.0,
            time_to_lerp: 20.0,
            outer_position: WowLabsPoint { x: 0.52, y: 0.48 },
            inner_position: WowLabsPoint { x: 0.61, y: 0.44 },
            base_radius: 1500.0,
            outer_scale: 1.0,
            inner_scale: 0.78,
            prediction_position: WowLabsPoint { x: 0.65, y: 0.41 },
            prediction_scale: 0.62,
            initial_base_size: 2048.0,
        }
    }
}

/// Matchmaking session state for the WoW Labs namespaces.
#[derive(Debug, Clone, PartialEq)]
pub struct WowLabsMatchmakingState {
    pub party_members: Vec<WowLabsPartyMember>,
    pub party_invites: Vec<WowLabsPartyInvite>,
    pub party_playlist_entry: i32,
    pub auto_queue_on_logout: bool,
    pub auto_queue_queue_type: i32,
    pub is_player_ready: bool,
    pub is_finding_match: bool,
    pub in_queue_time_start: f64,
    pub fast_login: bool,
}

impl Default for WowLabsMatchmakingState {
    fn default() -> Self {
        Self {
            party_members: vec![
                WowLabsPartyMember {
                    player_name: "Player".into(),
                    party_member_guid: "WoWLabsPlayer-Local".into(),
                    is_local_player: true,
                    is_party_leader: true,
                    is_ready: false,
                },
                WowLabsPartyMember {
                    player_name: "DuoBuddy".into(),
                    party_member_guid: "WoWLabsPlayer-DuoBuddy".into(),
                    is_local_player: false,
                    is_party_leader: false,
                    is_ready: false,
                },
            ],
            party_invites: vec![WowLabsPartyInvite {
                inviter_name: "PartyPal".into(),
                inviter_guid: "WoWLabsPlayer-PartyPal".into(),
                invite_id: "WoWLabsInvite-1".into(),
            }],
            party_playlist_entry: 2,
            auto_queue_on_logout: false,
            auto_queue_queue_type: 2,
            is_player_ready: false,
            is_finding_match: false,
            in_queue_time_start: 0.0,
            fast_login: false,
        }
    }
}

/// World-map selection state for WoW Labs.
#[derive(Debug, Clone, PartialEq)]
pub struct WowLabsDataManagerState {
    pub in_prematch: bool,
    pub areas: Vec<WowLabsAreaInfo>,
    pub selected_area_id: Option<i32>,
    pub confirmed_area_id: Option<i32>,
    pub circle_info: WowLabsCircleInfo,
}

impl Default for WowLabsDataManagerState {
    fn default() -> Self {
        Self {
            in_prematch: true,
            areas: vec![
                WowLabsAreaInfo {
                    wow_labs_area_id: 101,
                    x: 0.34,
                    y: 0.63,
                    area_type: 1,
                },
                WowLabsAreaInfo {
                    wow_labs_area_id: 102,
                    x: 0.56,
                    y: 0.47,
                    area_type: 2,
                },
                WowLabsAreaInfo {
                    wow_labs_area_id: 103,
                    x: 0.71,
                    y: 0.29,
                    area_type: 3,
                },
            ],
            selected_area_id: None,
            confirmed_area_id: None,
            circle_info: WowLabsCircleInfo::default(),
        }
    }
}

/// Top-level WoW Labs feature flags and nested namespace state.
#[derive(Debug, Clone, PartialEq)]
pub struct WowLabsState {
    pub enabled: bool,
    pub matchmaking_enabled: bool,
    pub available_queues: Vec<i32>,
    pub matchmaking: WowLabsMatchmakingState,
    pub data_manager: WowLabsDataManagerState,
}

impl Default for WowLabsState {
    fn default() -> Self {
        Self {
            enabled: true,
            matchmaking_enabled: true,
            available_queues: vec![0, 1, 2, 3],
            matchmaking: WowLabsMatchmakingState::default(),
            data_manager: WowLabsDataManagerState::default(),
        }
    }
}
