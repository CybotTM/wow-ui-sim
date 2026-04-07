//! Quest POI blob polygon data — vertex coordinates for quest area highlights.
//!
//! Each quest can have multiple blobs (polygon areas) displayed on the world map.
//! Coordinates are in map-normalized space (0.0–1.0) matching `QuestPOIPoint.db2`.

/// A single polygon blob for a quest area.
pub struct QuestPOIBlob {
    pub quest_id: u32,
    pub map_id: u32,
    /// Polygon vertices in map-normalized coordinates (0.0–1.0).
    pub vertices: &'static [(f32, f32)],
}

/// Look up all blobs for a given quest ID.
pub fn get_quest_blobs(quest_id: u32) -> &'static [QuestPOIBlob] {
    match quest_id {
        80000 => &QUEST_80000_BLOBS,
        80001 => &QUEST_80001_BLOBS,
        80002 => &QUEST_80002_BLOBS,
        _ => &[],
    }
}

/// Look up all blobs for a given quest ID on a specific map.
pub fn get_quest_blobs_for_map(quest_id: u32, map_id: u32) -> Vec<&'static QuestPOIBlob> {
    get_quest_blobs(quest_id)
        .iter()
        .filter(|b| b.map_id == map_id)
        .collect()
}

// --- Quest 80000: The Lost Expedition (Khaz Algar, map 2248) ---
// Polygon around the Old Quarry area
static QUEST_80000_BLOBS: [QuestPOIBlob; 1] = [QuestPOIBlob {
    quest_id: 80000,
    map_id: 2248,
    vertices: &[
        (0.42, 0.55),
        (0.46, 0.53),
        (0.49, 0.55),
        (0.50, 0.59),
        (0.47, 0.62),
        (0.43, 0.61),
        (0.41, 0.58),
    ],
}];

// --- Quest 80001: Defending the Gates (Stormwind, map 37) ---
// Polygon around the Stormwind gate area
static QUEST_80001_BLOBS: [QuestPOIBlob; 1] = [QuestPOIBlob {
    quest_id: 80001,
    map_id: 37,
    vertices: &[
        (0.70, 0.74),
        (0.74, 0.72),
        (0.78, 0.74),
        (0.79, 0.78),
        (0.76, 0.81),
        (0.72, 0.80),
        (0.69, 0.77),
    ],
}];

// --- Quest 80002: Supply Run (Elwynn Forest, map 37) ---
// Polygon around farmstead area
static QUEST_80002_BLOBS: [QuestPOIBlob; 1] = [QuestPOIBlob {
    quest_id: 80002,
    map_id: 37,
    vertices: &[
        (0.40, 0.48),
        (0.45, 0.46),
        (0.50, 0.48),
        (0.51, 0.53),
        (0.47, 0.56),
        (0.42, 0.55),
        (0.39, 0.52),
    ],
}];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_quest_blobs_known() {
        let blobs = get_quest_blobs(80000);
        assert_eq!(blobs.len(), 1);
        assert_eq!(blobs[0].quest_id, 80000);
        assert_eq!(blobs[0].map_id, 2248);
        assert!(
            blobs[0].vertices.len() >= 3,
            "Polygon needs at least 3 vertices"
        );
    }

    #[test]
    fn test_get_quest_blobs_unknown() {
        let blobs = get_quest_blobs(99999);
        assert!(blobs.is_empty());
    }

    #[test]
    fn test_get_quest_blobs_for_map_filters() {
        let blobs = get_quest_blobs_for_map(80001, 37);
        assert_eq!(blobs.len(), 1);

        let blobs_wrong_map = get_quest_blobs_for_map(80001, 9999);
        assert!(blobs_wrong_map.is_empty());
    }

    #[test]
    fn test_vertices_in_normalized_range() {
        for quest_id in [80000, 80001, 80002] {
            for blob in get_quest_blobs(quest_id) {
                for (x, y) in blob.vertices {
                    assert!(
                        (0.0..=1.0).contains(x) && (0.0..=1.0).contains(y),
                        "Quest {} vertex ({}, {}) out of 0.0–1.0 range",
                        quest_id,
                        x,
                        y
                    );
                }
            }
        }
    }
}
