//! Minimal duration object placeholder during the rilua migration.

use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy)]
pub struct LuaDurationObject {
    id: u64,
}

impl LuaDurationObject {
    pub fn new() -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn id(self) -> u64 {
        self.id
    }
}
