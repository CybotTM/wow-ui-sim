//! Minimal `C_ClubFinder` surface for Communities UI defaults.
//!
//! The simulator does not model recruitment postings or applicant queues yet,
//! but Blizzard Communities code expects these APIs to return concrete empty
//! collections rather than nil namespace fallbacks.

use super::ensure_namespace;
use crate::lua_api::methods::{create_table, table_set};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, RustFn, Val};

pub(super) fn register_club_finder_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_ClubFinder")?;
    register_same_handler(state, table_ref, EMPTY_TABLE_METHODS, empty_table)?;
    register_same_handler(state, table_ref, NOOP_METHODS, noop)?;
    register_same_handler(state, table_ref, TRUE_METHODS, return_true)?;
    register_same_handler(state, table_ref, FALSE_METHODS, return_false)?;
    register_same_handler(state, table_ref, ZERO_METHODS, return_zero)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetClubRecruitmentSettings",
        club_recruitment_settings,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetPlayerApplicantSettings",
        player_applicant_settings,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetClubFinderDisableReason", noop)?;
    Ok(())
}

fn register_same_handler(
    state: &mut LuaState,
    table_ref: GcRef<Table>,
    names: &[&'static str],
    handler: RustFn,
) -> LuaResult<()> {
    for name in names {
        table_set_rust_fn_static(state, table_ref, name, handler)?;
    }
    Ok(())
}

const EMPTY_TABLE_METHODS: &[&str] = &[
    "PlayerGetClubInvitationList",
    "PlayerReturnPendingCommunitiesList",
    "PlayerReturnPendingGuildsList",
    "ReturnClubApplicantList",
    "ReturnMatchingCommunityList",
    "ReturnMatchingGuildList",
    "ReturnPendingClubApplicantList",
    "GetStatusOfPostingFromClubId",
];

const TRUE_METHODS: &[&str] = &[
    "IsEnabled",
    "IsCommunityFinderEnabled",
    "IsValidSearchString",
];

const FALSE_METHODS: &[&str] = &[
    "DoesPlayerBelongToClubFromClubGUID",
    "HasAlreadyAppliedToLinkedPosting",
    "HasPostingBeenDelisted",
    "IsListingEnabledFromFlags",
    "IsPostingBanned",
];

const ZERO_METHODS: &[&str] = &[
    "GetClubTypeFromFinderGUID",
    "GetFocusIndexFromFlag",
    "GetPlayerClubApplicationStatus",
    "GetTotalMatchingCommunityListSize",
    "GetTotalMatchingGuildListSize",
];

const NOOP_METHODS: &[&str] = &[
    "ApplicantAcceptClubInvite",
    "ApplicantDeclineClubInvite",
    "CancelMembershipRequest",
    "LookupClubPostingFromClubFinderGUID",
    "PlayerRequestPendingClubsList",
    "PostClub",
    "RequestApplicantList",
    "RequestClubsList",
    "RequestMembershipToClub",
    "RequestNextCommunityPage",
    "RequestNextGuildPage",
    "RequestPostingInformationFromClubId",
    "RequestSubscribedClubPostingIDs",
    "ResetClubPostingMapCache",
    "RespondToApplicant",
    "SendChatWhisper",
    "SetAllRecruitmentSettings",
    "SetPlayerApplicantLocaleFlags",
    "SetPlayerApplicantSettings",
    "SetRecruitmentLocale",
    "SetRecruitmentSettings",
];

const PLAY_STYLE_SETTINGS: &[&str] = &[
    "playStyleDungeon",
    "playStyleRaids",
    "playStylePvp",
    "playStyleRP",
    "playStyleSocial",
];

const PLAYER_APPLICANT_FALSE_SETTINGS: &[&str] = &[
    "roleTank",
    "roleHealer",
    "roleDps",
    "sizeSmall",
    "sizeMedium",
    "sizeLarge",
    "sortMembers",
    "sortNewest",
    "crossFaction",
];

fn club_recruitment_settings(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    set_false_fields(state, table, PLAY_STYLE_SETTINGS);
    table_set(state, table, "maxLevelOnly", Val::Bool(false));
    table_set(state, table, "enableListing", Val::Bool(false));
    state.push(table);
    Ok(1)
}

fn player_applicant_settings(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    set_false_fields(state, table, PLAY_STYLE_SETTINGS);
    set_false_fields(state, table, PLAYER_APPLICANT_FALSE_SETTINGS);
    table_set(state, table, "sortRelevance", Val::Bool(true));
    state.push(table);
    Ok(1)
}

fn set_false_fields(state: &mut LuaState, table: Val, fields: &[&str]) {
    for field in fields {
        table_set(state, table, field, Val::Bool(false));
    }
}

fn empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn return_true(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn return_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}
