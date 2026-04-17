//! WoW Labs / Plunderstorm matchmaking and map state.
//!
//! Backs the `C_WoWLabsMatchmaking` / `C_WowLabsDataManager` namespaces:
//! party members, invites, world-map area choices, and shrinking-circle
//! values returned to Lua.

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
