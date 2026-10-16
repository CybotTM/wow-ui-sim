//! Explicit item-to-quest metadata supplied by the simulator host.
//! No native item/quest catalogue or association inference is provided.

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ContainerItemQuestMetadata {
    pub is_quest_item: bool,
    pub quest_id: Option<i32>,
}
