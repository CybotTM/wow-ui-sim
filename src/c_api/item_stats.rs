//! Explicit exact-link item-stat inputs; no catalog or query producer.

use std::collections::HashMap;

#[derive(Default)]
pub struct ItemStatsState {
    pub by_link: HashMap<String, HashMap<String, f64>>,
}
