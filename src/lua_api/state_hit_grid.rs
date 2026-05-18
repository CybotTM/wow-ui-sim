use super::SimState;

impl SimState {
    /// Schedule one frame for incremental hit-grid repair after input
    /// eligibility changes without changing layout or visibility.
    pub fn queue_hit_grid_eligibility_change(&mut self, id: u64) {
        self.pending_hit_grid_changes.push((id, true));
    }
}
