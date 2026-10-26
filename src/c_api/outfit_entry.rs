//! Complete supplied outfit record; no native values are inferred.

#[derive(Debug, Clone)]
pub struct OutfitEntry {
    pub outfit_id: i64,
    pub name: String,
    pub situation_categories: Vec<String>,
    pub icon: i64,
    pub is_event_outfit: bool,
    pub is_disabled: bool,
    pub player_facing_outfit_index: i64,
}
