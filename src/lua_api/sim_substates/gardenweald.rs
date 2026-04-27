//! Backing state for the `C_ArdenwealdGardening` namespace consumed by
//! `Blizzard_ArdenwealdGardening` and `Blizzard_GarrisonLandingPage`.
//!
//! `IsGardenAccessible` gates the entire panel from
//! `LandingPageMixin:UpdateArdenwealdGardeningSection`; `GetGardenData`
//! drives the OnEnter tooltip on the gardening button. `accessible`
//! defaults to `false` so a fresh simulator does not surface the panel
//! until a test or admin verb opts in.

/// Garden state surfaced as the `ArdenwealdGardenData` table.
///
/// Field semantics match `Blizzard_APIDocumentationGenerated/ArdenwealdGardeningDocumentation.lua`:
/// - `active` — number of wildseeds currently growing.
/// - `ready` — number of wildseeds ready to harvest.
/// - `remaining_seconds` — seconds until the next active wildseed finishes
///   (the addon formats this with `SecondsFormatter`). Stored as `i64`
///   because retail's `time_t` shape is signed.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GardenwealdState {
    pub accessible: bool,
    pub active: i32,
    pub ready: i32,
    pub remaining_seconds: i64,
}
