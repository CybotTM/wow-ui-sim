//! Auto-generated quest UI map data from WoW QuestPOIBlob CSV.
//! Do not edit manually - regenerate with: wow-cli generate quest-poi

static QUEST_UI_MAP: phf::Map<u32, u32> = ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        (80000, 2248),
    ],
};

pub fn get_quest_ui_map_id(quest_id: u32) -> u32 {
    QUEST_UI_MAP.get(&quest_id).copied().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_quest_returns_zero() {
        assert_eq!(get_quest_ui_map_id(999_999_999), 0);
    }

    #[test]
    fn test_known_quest() {
        assert_eq!(get_quest_ui_map_id(80000), 2248);
    }
}
