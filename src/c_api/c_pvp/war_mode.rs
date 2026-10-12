//! Explicit War Mode inputs, independent of generic PvP and active phase state.

#[derive(Debug, Clone)]
pub struct WarModeState {
    pub desired: bool,
    pub reward_bonus: f64,
}

impl Default for WarModeState {
    fn default() -> Self {
        Self {
            desired: false,
            // Preserve the existing simulator reward getter's 10-percent baseline.
            // This is compatibility policy, not a native reward/default claim.
            reward_bonus: 10.0,
        }
    }
}
