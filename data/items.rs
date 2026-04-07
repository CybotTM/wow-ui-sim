//! Auto-generated item data from WoW CSV exports.
//! Do not edit manually - regenerate with: wow-cli generate items

#[derive(Debug, Clone)]
pub struct ItemInfo {
    pub name: &'static str,
    pub quality: u8,
    pub item_level: u16,
    pub required_level: u16,
    pub inventory_type: u8,
    pub sell_price: u32,
    pub stackable: u32,
    pub bonding: u8,
    pub expansion_id: u8,
    pub icon_file_data_id: u32,
}

pub static ITEM_DB: phf::Map<u32, ItemInfo> = ::phf::Map {
    key: 6581282999337146909,
    disps: &[(0, 0), (1, 0), (1, 6), (5, 21), (9, 23)],
    entries: &[
        (
            210935,
            ItemInfo {
                name: "Aqirite",
                quality: 2,
                item_level: 70,
                required_level: 1,
                inventory_type: 0,
                sell_price: 1500,
                stackable: 1000,
                bonding: 0,
                expansion_id: 10,
                icon_file_data_id: 0,
            },
        ),
        (
            210931,
            ItemInfo {
                name: "Bismuth",
                quality: 1,
                item_level: 70,
                required_level: 1,
                inventory_type: 0,
                sell_price: 500,
                stackable: 1000,
                bonding: 0,
                expansion_id: 10,
                icon_file_data_id: 0,
            },
        ),
        (
            6948,
            ItemInfo {
                name: "Hearthstone",
                quality: 1,
                item_level: 1,
                required_level: 0,
                inventory_type: 0,
                sell_price: 0,
                stackable: 1,
                bonding: 1,
                expansion_id: 0,
                icon_file_data_id: 0,
            },
        ),
        (
            210934,
            ItemInfo {
                name: "Aqirite",
                quality: 2,
                item_level: 70,
                required_level: 1,
                inventory_type: 0,
                sell_price: 1500,
                stackable: 1000,
                bonding: 0,
                expansion_id: 10,
                icon_file_data_id: 0,
            },
        ),
        (
            211992,
            ItemInfo {
                name: "Entombed Seraph's Greaves",
                quality: 4,
                item_level: 571,
                required_level: 80,
                inventory_type: 7,
                sell_price: 943227,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 5567639,
            },
        ),
        (
            211994,
            ItemInfo {
                name: "Entombed Seraph's Castigation",
                quality: 4,
                item_level: 571,
                required_level: 80,
                inventory_type: 10,
                sell_price: 477906,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 5567636,
            },
        ),
        (
            211995,
            ItemInfo {
                name: "Entombed Seraph's Sabatons",
                quality: 4,
                item_level: 571,
                required_level: 80,
                inventory_type: 8,
                sell_price: 715770,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 5567632,
            },
        ),
        (
            211989,
            ItemInfo {
                name: "Entombed Seraph's Shackles",
                quality: 4,
                item_level: 571,
                required_level: 80,
                inventory_type: 9,
                sell_price: 470052,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 5567633,
            },
        ),
        (
            211993,
            ItemInfo {
                name: "Entombed Seraph's Casque",
                quality: 4,
                item_level: 571,
                required_level: 80,
                inventory_type: 1,
                sell_price: 708751,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 5567638,
            },
        ),
        (
            236914,
            ItemInfo {
                name: "Unbound Vision Journal",
                quality: 4,
                item_level: 600,
                required_level: 80,
                inventory_type: 12,
                sell_price: 532442,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 0,
            },
        ),
        (
            211988,
            ItemInfo {
                name: "Entombed Seraph's Greatcloak",
                quality: 4,
                item_level: 571,
                required_level: 80,
                inventory_type: 16,
                sell_price: 697124,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 5567634,
            },
        ),
        (
            211990,
            ItemInfo {
                name: "Entombed Seraph's Waistguard",
                quality: 4,
                item_level: 571,
                required_level: 80,
                inventory_type: 6,
                sell_price: 471855,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 5567630,
            },
        ),
        (
            225748,
            ItemInfo {
                name: "Seal of the Silent Vigil",
                quality: 4,
                item_level: 571,
                required_level: 80,
                inventory_type: 11,
                sell_price: 496336,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 0,
            },
        ),
        (
            211991,
            ItemInfo {
                name: "Entombed Seraph's Plumes",
                quality: 4,
                item_level: 571,
                required_level: 80,
                inventory_type: 3,
                sell_price: 703467,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 5567640,
            },
        ),
        (
            210930,
            ItemInfo {
                name: "Bismuth",
                quality: 1,
                item_level: 70,
                required_level: 1,
                inventory_type: 0,
                sell_price: 500,
                stackable: 1000,
                bonding: 0,
                expansion_id: 10,
                icon_file_data_id: 0,
            },
        ),
        (
            215135,
            ItemInfo {
                name: "Ring of Earthen Craftsmanship",
                quality: 4,
                item_level: 610,
                required_level: 1,
                inventory_type: 11,
                sell_price: 168450,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 0,
            },
        ),
        (
            218715,
            ItemInfo {
                name: "Forged Gladiator's Emblem",
                quality: 4,
                item_level: 584,
                required_level: 80,
                inventory_type: 12,
                sell_price: 761425,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 0,
            },
        ),
        (
            229181,
            ItemInfo {
                name: "Ordained Forge Maul",
                quality: 4,
                item_level: 610,
                required_level: 80,
                inventory_type: 17,
                sell_price: 1752504,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 5367208,
            },
        ),
        (
            210937,
            ItemInfo {
                name: "Ironclaw Ore",
                quality: 2,
                item_level: 70,
                required_level: 1,
                inventory_type: 0,
                sell_price: 1500,
                stackable: 1000,
                bonding: 0,
                expansion_id: 10,
                icon_file_data_id: 0,
            },
        ),
        (
            230637,
            ItemInfo {
                name: "Astral Gladiator's Amulet",
                quality: 4,
                item_level: 584,
                required_level: 80,
                inventory_type: 2,
                sell_price: 376497,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 0,
            },
        ),
        (
            4540,
            ItemInfo {
                name: "Tough Hunk of Bread",
                quality: 1,
                item_level: 1,
                required_level: 1,
                inventory_type: 0,
                sell_price: 1,
                stackable: 20,
                bonding: 0,
                expansion_id: 0,
                icon_file_data_id: 0,
            },
        ),
        (
            159,
            ItemInfo {
                name: "Refreshing Spring Water",
                quality: 1,
                item_level: 1,
                required_level: 1,
                inventory_type: 0,
                sell_price: 0,
                stackable: 20,
                bonding: 0,
                expansion_id: 0,
                icon_file_data_id: 0,
            },
        ),
        (
            211996,
            ItemInfo {
                name: "Entombed Seraph's Breastplate",
                quality: 4,
                item_level: 571,
                required_level: 80,
                inventory_type: 5,
                sell_price: 957441,
                stackable: 1,
                bonding: 1,
                expansion_id: 10,
                icon_file_data_id: 5567635,
            },
        ),
        (
            7005,
            ItemInfo {
                name: "Skinning Knife",
                quality: 1,
                item_level: 2,
                required_level: 1,
                inventory_type: 29,
                sell_price: 24,
                stackable: 1,
                bonding: 0,
                expansion_id: 0,
                icon_file_data_id: 135637,
            },
        ),
    ],
};

pub fn get_item(id: u32) -> Option<&'static ItemInfo> {
    ITEM_DB.get(&id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_count() {
        assert!(ITEM_DB.len() > 10);
    }

    #[test]
    fn test_hearthstone() {
        let item = get_item(6948).expect("Hearthstone (6948) should exist");
        assert_eq!(item.name, "Hearthstone");
        assert_eq!(item.quality, 1);
    }

    #[test]
    fn test_nonexistent_item() {
        assert!(get_item(999_999_999).is_none());
    }
}
