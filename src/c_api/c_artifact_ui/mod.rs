//! `C_ArtifactUI` surface consumed by both
//! `Blizzard_ActionBar/Mainline/ArtifactBar.lua` (the equipped-artifact
//! XP bar) and the LoD `Blizzard_ArtifactUI` panel addon (the full
//! artifact talent / appearance UI).
//!
//! State sources:
//!
//! - `state.equipped_artifact: Option<ArtifactInfo>` — feeds the
//!   action-bar getters (`GetEquippedArtifactItemID`,
//!   `GetEquippedArtifactInfo`, `IsEquippedArtifactMaxed`,
//!   `IsEquippedArtifactDisabled`, `GetArtifactXPRewardTargetInfo`).
//! - `state.artifact_point_costs: HashMap<(rank, tier), xp_cost>` —
//!   feeds `GetCostForPointAtRank`. A missing entry returns 0.
//! - `state.viewed_artifact: ViewedArtifactState` — feeds every
//!   panel-side getter (`GetArtifactInfo`, `GetArtifactArtInfo`,
//!   `GetPowerInfo`, etc.). When `viewed_artifact.info` is `None`
//!   every `MayReturnNothing` getter returns 0 values, matching the
//!   doc-canonical contract addon callsites guard for.
//!
//! Mutators (`AddPower`, `Clear`, `ConfirmRespec`, `SetAppearance`,
//! `SetPreviewAppearance`, `SetForgeRotation`, `ApplyCursorRelicToSlot`)
//! write through `state.viewed_artifact` and fire the matching
//! `ARTIFACT_UPDATE` / `ARTIFACT_CLOSE` events through the simulator
//! event queue, mirroring the live client's server round-trips.

mod action_bar;
mod helpers;
mod panel;

use crate::c_api::helpers::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;

type ArtifactUiFn = fn(&mut LuaState) -> LuaResult<u32>;

const ACTION_BAR_METHODS: &[(&str, ArtifactUiFn)] = &[
    (
        "GetEquippedArtifactItemID",
        action_bar::get_equipped_artifact_item_id,
    ),
    (
        "GetEquippedArtifactInfo",
        action_bar::get_equipped_artifact_info,
    ),
    (
        "IsEquippedArtifactMaxed",
        action_bar::is_equipped_artifact_maxed,
    ),
    (
        "IsEquippedArtifactDisabled",
        action_bar::is_equipped_artifact_disabled,
    ),
    (
        "GetCostForPointAtRank",
        action_bar::get_cost_for_point_at_rank,
    ),
    (
        "GetArtifactXPRewardTargetInfo",
        action_bar::get_artifact_xp_reward_target_info,
    ),
];

const PANEL_METHODS: &[(&str, ArtifactUiFn)] = &[
    // Panel core getters (read state.viewed_artifact).
    ("GetArtifactInfo", panel::get_artifact_info),
    ("GetArtifactItemID", panel::get_artifact_item_id),
    ("GetArtifactTier", panel::get_artifact_tier),
    ("GetArtifactArtInfo", panel::get_artifact_art_info),
    ("GetPointsRemaining", panel::get_points_remaining),
    ("GetTotalPurchasedRanks", panel::get_total_purchased_ranks),
    ("GetNumObtainedArtifacts", panel::get_num_obtained_artifacts),
    ("IsArtifactDisabled", panel::is_artifact_disabled),
    ("IsAtForge", panel::is_at_forge),
    ("IsMaxedByRulesOrEffect", panel::is_maxed_by_rules_or_effect),
    (
        "IsViewedArtifactEquipped",
        panel::is_viewed_artifact_equipped,
    ),
    ("CheckRespecNPC", panel::check_respec_npc),
    // Panel power getters.
    ("GetPowerInfo", panel::get_power_info),
    ("GetPowers", panel::get_powers),
    ("GetPowerLinks", panel::get_power_links),
    ("GetMetaPowerInfo", panel::get_meta_power_info),
    ("GetPowerHyperlink", panel::get_power_hyperlink),
    ("GetTotalPowerCost", panel::get_total_power_cost),
    (
        "GetPowersAffectedByRelic",
        panel::get_powers_affected_by_relic,
    ),
    (
        "GetPowersAffectedByRelicItemLink",
        panel::get_powers_affected_by_relic_item_link,
    ),
    ("IsPowerKnown", panel::is_power_known),
    // Panel appearance getters.
    ("GetNumAppearanceSets", panel::get_num_appearance_sets),
    ("GetAppearanceSetInfo", panel::get_appearance_set_info),
    ("GetAppearanceInfo", panel::get_appearance_info),
    ("GetAppearanceInfoByID", panel::get_appearance_info_by_id),
    ("GetPreviewAppearance", panel::get_preview_appearance),
    // Panel relic getters.
    ("GetNumRelicSlots", panel::get_num_relic_slots),
    ("GetRelicInfo", panel::get_relic_info),
    ("GetRelicInfoByItemID", panel::get_relic_info_by_item_id),
    ("GetRelicLockedReason", panel::get_relic_locked_reason),
    ("GetRelicSlotType", panel::get_relic_slot_type),
    (
        "CanApplyCursorRelicToSlot",
        panel::can_apply_cursor_relic_to_slot,
    ),
    (
        "CanApplyRelicItemIDToSlot",
        panel::can_apply_relic_item_id_to_slot,
    ),
    // Panel forge methods.
    ("GetForgeRotation", panel::get_forge_rotation),
    (
        "ShouldSuppressForgeRotation",
        panel::should_suppress_forge_rotation,
    ),
    ("SetForgeRotation", panel::set_forge_rotation),
    // Panel mutators (fire ARTIFACT_UPDATE / ARTIFACT_CLOSE).
    ("AddPower", panel::add_power),
    ("Clear", panel::clear_artifact),
    ("ConfirmRespec", panel::confirm_respec),
    ("SetAppearance", panel::set_appearance),
    ("SetPreviewAppearance", panel::set_preview_appearance),
    ("ApplyCursorRelicToSlot", panel::apply_cursor_relic_to_slot),
];

const ARTIFACT_UI_METHOD_GROUPS: &[&[(&str, ArtifactUiFn)]] = &[ACTION_BAR_METHODS, PANEL_METHODS];

pub(crate) fn register_c_artifact_ui_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_ArtifactUI")?;
    install_methods(state, ns)
}

fn install_methods(state: &mut LuaState, ns: GcRef<Table>) -> LuaResult<()> {
    for group in ARTIFACT_UI_METHOD_GROUPS {
        for &(name, func) in *group {
            table_set_rust_fn_static(state, ns, name, func)?;
        }
    }
    Ok(())
}
