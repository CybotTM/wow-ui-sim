//! Minimal item data.
//! This intentionally keeps only the API surface the simulator consumes.

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
}

pub static ITEM_DB: &[(u32, ItemInfo)] = &[
    (6948, ItemInfo { name: "Hearthstone", quality: 1, item_level: 1, required_level: 1, inventory_type: 0, sell_price: 0, stackable: 1, bonding: 0, expansion_id: 0 }),
    // Ret Paladin gear set (ilvl 615)
    (221096, ItemInfo { name: "Entombed Seraph's Casque", quality: 4, item_level: 615, required_level: 80, inventory_type: 1, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (225577, ItemInfo { name: "Sureki Zealot's Insignia", quality: 4, item_level: 615, required_level: 80, inventory_type: 2, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (221094, ItemInfo { name: "Entombed Seraph's Mantle", quality: 4, item_level: 615, required_level: 80, inventory_type: 3, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (221091, ItemInfo { name: "Entombed Seraph's Castigation", quality: 4, item_level: 615, required_level: 80, inventory_type: 5, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (221086, ItemInfo { name: "Devoted Priest's Sash", quality: 4, item_level: 615, required_level: 80, inventory_type: 6, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (221095, ItemInfo { name: "Entombed Seraph's Greaves", quality: 4, item_level: 615, required_level: 80, inventory_type: 7, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (221087, ItemInfo { name: "Devoted Priest's Treads", quality: 4, item_level: 615, required_level: 80, inventory_type: 8, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (221088, ItemInfo { name: "Devoted Priest's Wristguards", quality: 4, item_level: 615, required_level: 80, inventory_type: 9, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (221092, ItemInfo { name: "Entombed Seraph's Hallowed Grasp", quality: 4, item_level: 615, required_level: 80, inventory_type: 10, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (225578, ItemInfo { name: "Seal of the Poisoned Pact", quality: 4, item_level: 615, required_level: 80, inventory_type: 11, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (225579, ItemInfo { name: "Loop of Hovering Menace", quality: 4, item_level: 615, required_level: 80, inventory_type: 11, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (225580, ItemInfo { name: "Skarmorak Shard", quality: 4, item_level: 615, required_level: 80, inventory_type: 12, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (225581, ItemInfo { name: "Void Reaper's Contract", quality: 4, item_level: 615, required_level: 80, inventory_type: 12, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (225582, ItemInfo { name: "Shroud of the Priory", quality: 4, item_level: 615, required_level: 80, inventory_type: 16, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    (225583, ItemInfo { name: "Greatsword of Radiant Dawn", quality: 4, item_level: 615, required_level: 80, inventory_type: 17, sell_price: 100000, stackable: 1, bonding: 1, expansion_id: 10 }),
    // Crafting reagents
    (210930, ItemInfo { name: "Bismuth", quality: 1, item_level: 1, required_level: 1, inventory_type: 0, sell_price: 100, stackable: 200, bonding: 0, expansion_id: 10 }),
    (210931, ItemInfo { name: "Aqirite", quality: 2, item_level: 1, required_level: 1, inventory_type: 0, sell_price: 100, stackable: 200, bonding: 0, expansion_id: 10 }),
    (210932, ItemInfo { name: "Ironcrest Ore", quality: 1, item_level: 1, required_level: 1, inventory_type: 0, sell_price: 100, stackable: 200, bonding: 0, expansion_id: 10 }),
    (210933, ItemInfo { name: "Null Stone", quality: 3, item_level: 1, required_level: 1, inventory_type: 0, sell_price: 100, stackable: 200, bonding: 0, expansion_id: 10 }),
    (210934, ItemInfo { name: "Khaz Algar Ingot", quality: 1, item_level: 1, required_level: 1, inventory_type: 0, sell_price: 100, stackable: 200, bonding: 0, expansion_id: 10 }),
    (210935, ItemInfo { name: "Aqirite Ingot", quality: 2, item_level: 1, required_level: 1, inventory_type: 0, sell_price: 100, stackable: 200, bonding: 0, expansion_id: 10 }),
    (210936, ItemInfo { name: "Ironcrest Ingot", quality: 2, item_level: 1, required_level: 1, inventory_type: 0, sell_price: 100, stackable: 200, bonding: 0, expansion_id: 10 }),
    (210937, ItemInfo { name: "Flux", quality: 1, item_level: 1, required_level: 1, inventory_type: 0, sell_price: 100, stackable: 200, bonding: 0, expansion_id: 10 }),
];

pub fn get_item(id: u32) -> Option<&'static ItemInfo> {
    ITEM_DB
        .iter()
        .find_map(|(item_id, item)| (*item_id == id).then_some(item))
}
