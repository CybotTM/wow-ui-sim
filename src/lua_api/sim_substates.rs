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
