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
