//! PvP and legacy Classic honor state surfaced through global PvP APIs.

#[derive(Debug, Clone, Default)]
pub struct PvpHonorState {
    /// `HonorSystemEnabled()` — MoP Classic keeps the old HonorFrame gate but
    /// returns false for the disabled legacy honor surface.
    pub classic_honor_system_enabled: bool,
    /// `GetPVPYesterdayStats()` — honorable and dishonorable kills.
    pub yesterday_honorable_kills: i32,
    pub yesterday_dishonorable_kills: i32,
    /// `GetPVPThisWeekStats()` — honorable kills and contribution.
    pub this_week_honorable_kills: i32,
    pub this_week_contribution: i32,
    /// `GetPVPLastWeekStats()` — honorable kills, dishonorable kills,
    /// contribution, and rank.
    pub last_week_honorable_kills: i32,
    pub last_week_dishonorable_kills: i32,
    pub last_week_contribution: i32,
    pub last_week_rank: i32,
    /// `GetPVPSessionStats()` — honorable and dishonorable kills.
    pub session_honorable_kills: i32,
    pub session_dishonorable_kills: i32,
    /// `GetPVPLifetimeStats()` — honorable kills, dishonorable kills, and
    /// highest rank.
    pub lifetime_honorable_kills: i32,
    pub lifetime_dishonorable_kills: i32,
    pub lifetime_highest_rank: i32,
    /// `GetPVPRankProgress()` — fractional progress toward the next rank.
    pub rank_progress: f64,
}
