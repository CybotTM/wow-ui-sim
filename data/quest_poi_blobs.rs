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

/// Point-in-polygon test using the ray casting algorithm.
/// Returns true if the point (px, py) is inside the polygon defined by `vertices`.
pub fn point_in_polygon(px: f32, py: f32, vertices: &[(f32, f32)]) -> bool {
    let n = vertices.len();
    if n < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let (xi, yi) = vertices[i];
        let (xj, yj) = vertices[j];
        if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// Find the first active quest whose blob polygon contains the given point.
/// Returns `(quest_id, blob_count)` where blob_count is the number of matching blobs.
pub fn hit_test_blobs(active_quests: &[u32], map_id: u32, x: f32, y: f32) -> Option<(u32, usize)> {
    for &quest_id in active_quests {
        let blobs = get_quest_blobs_for_map(quest_id, map_id);
        let hit_count = blobs
            .iter()
            .filter(|b| point_in_polygon(x, y, b.vertices))
            .count();
        if hit_count > 0 {
            return Some((quest_id, hit_count));
        }
    }
    None
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
    fn test_point_in_polygon_square() {
        let square = &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        assert!(point_in_polygon(0.5, 0.5, square));
        assert!(!point_in_polygon(1.5, 0.5, square));
        assert!(!point_in_polygon(0.5, -0.1, square));
    }

    #[test]
    fn test_point_in_polygon_triangle() {
        let tri = &[(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)];
        assert!(point_in_polygon(0.5, 0.3, tri));
        assert!(!point_in_polygon(0.0, 1.0, tri));
    }

    #[test]
    fn test_point_in_polygon_degenerate() {
        assert!(!point_in_polygon(0.0, 0.0, &[]));
        assert!(!point_in_polygon(0.0, 0.0, &[(0.0, 0.0), (1.0, 0.0)]));
    }

    #[test]
    fn test_hit_test_blobs_inside() {
        // Quest 80000 blob center is roughly (0.45, 0.58) on map 2248
        let result = hit_test_blobs(&[80000], 2248, 0.45, 0.58);
        assert!(result.is_some(), "Point inside blob should hit");
        let (qid, count) = result.unwrap();
        assert_eq!(qid, 80000);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_hit_test_blobs_outside() {
        let result = hit_test_blobs(&[80000], 2248, 0.1, 0.1);
        assert!(result.is_none(), "Point outside all blobs should miss");
    }

    #[test]
    fn test_hit_test_blobs_wrong_map() {
        // Quest 80000 is on map 2248, not 37
        let result = hit_test_blobs(&[80000], 37, 0.45, 0.58);
        assert!(result.is_none(), "Wrong map should not match");
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
