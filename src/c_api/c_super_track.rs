//! Supplied SuperTrack targets. No priority arbitration is inferred.

#[derive(Debug, Clone, Default)]
pub struct SuperTrackState {
    pub quest_id: Option<i32>,
    pub content: Option<(i32, i32)>,
    pub map_pin: Option<(i32, i32)>,
    pub vignette: Option<String>,
}
