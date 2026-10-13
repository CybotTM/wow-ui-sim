//! C_PetJournal hover selection. Mutation handlers remain unimplemented.

/// Environment-owned hover selection; default None is simulator policy, not native proof.
#[derive(Debug, Default)]
pub struct PetJournalHoverState {
    pub hovered_battle_pet_guid: Option<String>,
}
