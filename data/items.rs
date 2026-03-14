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

pub static ITEM_DB: &[(u32, ItemInfo)] = &[(
    6948,
    ItemInfo {
        name: "Hearthstone",
        quality: 1,
        item_level: 1,
        required_level: 1,
        inventory_type: 0,
        sell_price: 0,
        stackable: 1,
        bonding: 0,
        expansion_id: 0,
    },
)];

pub fn get_item(id: u32) -> Option<&'static ItemInfo> {
    ITEM_DB
        .iter()
        .find_map(|(item_id, item)| (*item_id == id).then_some(item))
}
