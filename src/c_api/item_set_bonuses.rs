//! Explicit ordered spell lists keyed by (specialization ID, item ID).

use std::collections::HashMap;

pub type ItemSetBonuses = HashMap<(i32, i32), Vec<i32>>;
