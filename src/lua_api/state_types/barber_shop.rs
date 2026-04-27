//! `C_BarberShop` sim-state types.
//!
//! Backs the surface read by `Blizzard_BarbershopUI`. The addon's
//! `BarberShopMixin:OnShow` reads `current_character` and dispatches a
//! body-type button per `Enum.UnitSex` value; `UpdateCharCustomizationFrame`
//! short-circuits when `available_customizations` is `None` so tests
//! that don't seed customizations still exercise the open/close flow.

use std::collections::{HashMap, HashSet};

/// Mirrors `PlayerInfoCharacterData` from
/// `vendor/wow-ui-source/.../PlayerInfoSharedDocumentation.lua` —
/// `Blizzard_BarbershopUI:UpdateSex` reads `.sex` and forwards the whole
/// table to `CharCustomizeFrame:SetSelectedData`.
#[derive(Clone, Debug, Default)]
pub struct BarberShopCharacterData {
    pub name: String,
    pub file_name: String,
    pub alternate_form_race: Option<BarberShopAlternateFormRace>,
    pub create_screen_icon_atlas: String,
    /// `Enum.UnitSex` value. The addon iterates `{Male, Female}` body-type
    /// buttons and compares each against this field to mark the active one.
    pub sex: i32,
}

/// `CharacterAlternateFormData` — shape `currentCharacterData.alternateFormRaceData`.
/// Driven by races with druid/worgen-style alt forms (Mechagnome, Worgen, etc.).
#[derive(Clone, Debug, Default)]
pub struct BarberShopAlternateFormRace {
    pub race_id: i32,
    pub name: String,
    pub file_name: String,
    pub create_screen_icon_atlas: String,
}

/// One customization category row (Hair, Face, Skin, etc.) returned by
/// `GetAvailableCustomizations`. The addon hands the list straight to
/// `CharCustomizeFrame:SetCustomizations`, so tests only need a name +
/// option list to exercise the round-trip.
#[derive(Clone, Debug, Default)]
pub struct BarberShopCategory {
    pub name: String,
    pub options: Vec<BarberShopOption>,
}

/// One customization option (Hair Style, Skin Color, etc.) inside a
/// category. Tests assert on `option_id` to verify the table builder
/// preserves identity across the Lua boundary.
#[derive(Clone, Debug, Default)]
pub struct BarberShopOption {
    pub option_id: i32,
    pub name: String,
    pub current_choice_id: Option<i32>,
}

/// `C_BarberShop` backing state. Defaults reflect a freshly-opened
/// barber shop with no customization data loaded — `GetAvailableCustomizations`
/// returns nil so the addon's "no character component set up" branch
/// (Blizzard_BarberShopUI.lua:130) takes over until tests seed data.
#[derive(Clone, Debug, Default)]
pub struct BarberShopState {
    /// `Enum.ChrModelFeatureFlags` bitmask. `HasCustomizationFeature`
    /// returns `(feature_flags & arg) != 0`. Default 0 means every
    /// feature probe (e.g. `Mounts`) reports false, sending the addon
    /// down the non-dragonriding sound branch.
    pub feature_flags: i32,
    /// Currently-edited character snapshot. `None` makes
    /// `GetCurrentCharacterData` return nil and the body-type row stays
    /// empty (the addon only iterates `Enum.UnitSex` when this is set).
    pub current_character: Option<BarberShopCharacterData>,
    /// Whether `IsViewingAlteredForm` reports true. Druid/worgen alt-form
    /// branch in `CharCustomizeFrame:SetSelectedData`.
    pub viewing_altered_form: bool,
    /// `GetViewingChrModel` return — the addon hides `BodyTypes` when
    /// non-nil because mount/dynaflight customization owns the camera.
    pub viewing_chr_model: Option<i32>,
    /// Currently-previewed shapeshift form id (Druid forms). The addon
    /// hides body-type buttons whenever this is non-nil.
    pub viewing_shapeshift_form: Option<i32>,
    /// Customization category rows. `None` returns nil from
    /// `GetAvailableCustomizations`; `Some(empty)` returns a non-nil
    /// empty table, which the addon would hand to CharCustomizeFrame.
    pub available_customizations: Option<Vec<BarberShopCategory>>,
    /// Saved choices keyed by `optionID`. Mutated by
    /// `SetCustomizationChoice`; cleared by `ResetCustomizationChoices`.
    pub choices: HashMap<i32, i32>,
    /// Preview-only choices keyed by `optionID`. Mutated by
    /// `PreviewCustomizationChoice`; cleared by `ClearPreviewChoices`.
    /// `ApplyCustomizationChoices` folds these into `choices`.
    pub preview_choices: HashMap<i32, i32>,
    /// Whether `HasAnyChanges` reports true. `SetCustomizationChoice`
    /// flips it on; `ApplyCustomizationChoices` / `ResetCustomizationChoices`
    /// flip it off. Drives the Accept/Reset button enable state.
    pub has_changes: bool,
    /// Current camera zoom level returned by `GetCurrentCameraZoom`.
    pub camera_zoom: f32,
    /// Camera distance offset, written by `SetCameraDistanceOffset`.
    /// Test-observable; the addon doesn't read it back.
    pub camera_distance_offset: f32,
    /// Whether the previewed character model is shown dressed. Mirrors
    /// `SetModelDressState`'s argument. Test-observable only.
    pub model_dressed: bool,
    /// "Seen" choice ids — `MarkCustomizationChoiceAsSeen` inserts.
    /// `SaveSeenChoices` is a no-op because the set already lives in
    /// SimState (which tests can inspect directly).
    pub seen_choices: HashSet<i32>,
    /// "Seen" option ids — `MarkCustomizationOptionAsSeen` inserts.
    pub seen_options: HashSet<i32>,
}
