//! `C_Housing` and `C_HousingBlueprint` probes backed by `SimState.housing`.
//!
//! The simulator does not model the full War Within housing service yet, but it
//! already keeps house-favor display state. These 12.1 probes expose a small,
//! deterministic local contract: tests or future service glue may mark the
//! player as being inside an owned house and/or plot, `ResetHouse` clears local
//! housing/favor state, and blueprint import/export calls produce simulator
//! share codes that can be round-tripped in tests.

use crate::c_api::helpers::ensure_namespace;
#[cfg(feature = "retail-12-1-0")]
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
#[cfg(feature = "retail-12-1-0")]
use crate::lua_api::state::HousingState;
#[cfg(feature = "retail-12-1-0")]
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::LuaResult;
#[cfg(feature = "retail-12-1-0")]
use rilua::Val;
#[cfg(feature = "retail-12-1-0")]
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
#[cfg(feature = "retail-12-1-0")]
use rilua::vm::table::Table;

#[cfg(feature = "retail-12-1-0")]
type NamespaceTable = GcRef<Table>;

#[cfg(feature = "retail-12-1-0")]
const BLUEPRINT_CODE_PREFIX: &str = "wow-ui-sim:blueprint:";
#[cfg(feature = "retail-12-1-0")]
const ROOM_BLUEPRINT_CODE_PREFIX: &str = "wow-ui-sim:room-blueprint:";
#[cfg(feature = "retail-12-1-0")]
const BLUEPRINT_TYPE_HOUSE: i32 = 1;
#[cfg(feature = "retail-12-1-0")]
const BLUEPRINT_TYPE_ROOM: i32 = 2;

pub(crate) fn register_c_housing_surface(state: &mut LuaState) -> LuaResult<()> {
    let housing = ensure_namespace(state, "C_Housing")?;
    let blueprints = ensure_namespace(state, "C_HousingBlueprint")?;
    register_patch_12_1_c_housing_surface(state, housing, blueprints)
}

#[cfg(feature = "retail-12-1-0")]
fn register_patch_12_1_c_housing_surface(
    state: &mut LuaState,
    housing: NamespaceTable,
    blueprints: NamespaceTable,
) -> LuaResult<()> {
    register_housing_methods(state, housing)?;
    register_blueprint_methods(state, blueprints)
}

#[cfg(not(feature = "retail-12-1-0"))]
fn register_patch_12_1_c_housing_surface(
    _state: &mut LuaState,
    _housing: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
    _blueprints: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> LuaResult<()> {
    Ok(())
}

#[cfg(feature = "retail-12-1-0")]
fn register_housing_methods(state: &mut LuaState, housing: NamespaceTable) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        housing,
        "HouseFinderIgnoreNeighborhood",
        house_finder_ignore_neighborhood,
    )?;
    table_set_rust_fn_static(
        state,
        housing,
        "IsInsideOwnedHouseOrPlot",
        is_inside_owned_house_or_plot,
    )?;
    table_set_rust_fn_static(state, housing, "IsInsideOwnedHouse", is_inside_owned_house)?;
    table_set_rust_fn_static(state, housing, "IsInsideOwnedPlot", is_inside_owned_plot)?;
    table_set_rust_fn_static(state, housing, "ResetHouse", reset_house)
}

#[cfg(feature = "retail-12-1-0")]
fn register_blueprint_methods(state: &mut LuaState, blueprints: NamespaceTable) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        blueprints,
        "CanImportTypeFromCurrentLocation",
        can_import_type_from_current_location,
    )?;
    table_set_rust_fn_static(state, blueprints, "DeleteBlueprint", delete_blueprint)?;
    table_set_rust_fn_static(state, blueprints, "ExportBlueprint", export_blueprint)?;
    table_set_rust_fn_static(
        state,
        blueprints,
        "ExportRoomBlueprint",
        export_room_blueprint,
    )?;
    table_set_rust_fn_static(
        state,
        blueprints,
        "GetBlueprintHyperlink",
        get_blueprint_hyperlink,
    )?;
    table_set_rust_fn_static(
        state,
        blueprints,
        "GetBlueprintTypeForCode",
        get_blueprint_type_for_code,
    )?;
    table_set_rust_fn_static(state, blueprints, "ImportBlueprint", import_blueprint)?;
    table_set_rust_fn_static(state, blueprints, "IsShareCodeValid", is_share_code_valid)?;
    table_set_rust_fn_static(state, blueprints, "RenameBlueprint", rename_blueprint)?;
    table_set_rust_fn_static(
        state,
        blueprints,
        "RequestBlueprintCollection",
        request_blueprint_collection,
    )?;
    table_set_rust_fn_static(
        state,
        blueprints,
        "RequestBlueprintContents",
        request_blueprint_contents,
    )?;
    table_set_rust_fn_static(
        state,
        blueprints,
        "RequestBlueprintContentsForContext",
        request_blueprint_contents_for_context,
    )?;
    table_set_rust_fn_static(
        state,
        blueprints,
        "StartImportRoomBlueprint",
        start_import_room_blueprint,
    )
}

#[cfg(feature = "retail-12-1-0")]
fn house_finder_ignore_neighborhood(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

#[cfg(feature = "retail-12-1-0")]
fn is_inside_owned_house_or_plot(state: &mut LuaState) -> LuaResult<u32> {
    let is_inside = {
        let sim = borrow_state(state)?;
        sim.housing.inside_owned_house || sim.housing.inside_owned_plot
    };
    state.push(Val::Bool(is_inside));
    Ok(1)
}

#[cfg(feature = "retail-12-1-0")]
fn is_inside_owned_house(state: &mut LuaState) -> LuaResult<u32> {
    let inside_owned_house = { borrow_state(state)?.housing.inside_owned_house };
    state.push(Val::Bool(inside_owned_house));
    Ok(1)
}

#[cfg(feature = "retail-12-1-0")]
fn is_inside_owned_plot(state: &mut LuaState) -> LuaResult<u32> {
    let inside_owned_plot = { borrow_state(state)?.housing.inside_owned_plot };
    state.push(Val::Bool(inside_owned_plot));
    Ok(1)
}

#[cfg(feature = "retail-12-1-0")]
fn reset_house(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    sim.housing = HousingState::default();
    Ok(0)
}

#[cfg(feature = "retail-12-1-0")]
fn can_import_type_from_current_location(state: &mut LuaState) -> LuaResult<u32> {
    is_inside_owned_house_or_plot(state)
}

#[cfg(feature = "retail-12-1-0")]
fn delete_blueprint(state: &mut LuaState) -> LuaResult<u32> {
    let blueprint_id = Option::<String>::from_stack(state, 1)?;
    borrow_state_mut(state)?.housing.last_deleted_blueprint_id = blueprint_id;
    Ok(0)
}

#[cfg(feature = "retail-12-1-0")]
fn export_blueprint(state: &mut LuaState) -> LuaResult<u32> {
    let blueprint_id = string_arg_or_empty(state, 1)?;
    borrow_state_mut(state)?.housing.last_exported_blueprint_id = Some(blueprint_id.clone());
    push_string(state, &blueprint_code(BLUEPRINT_CODE_PREFIX, &blueprint_id));
    Ok(1)
}

#[cfg(feature = "retail-12-1-0")]
fn export_room_blueprint(state: &mut LuaState) -> LuaResult<u32> {
    let blueprint_id = string_arg_or_empty(state, 1)?;
    borrow_state_mut(state)?
        .housing
        .last_exported_room_blueprint_id = Some(blueprint_id.clone());
    push_string(
        state,
        &blueprint_code(ROOM_BLUEPRINT_CODE_PREFIX, &blueprint_id),
    );
    Ok(1)
}

#[cfg(feature = "retail-12-1-0")]
fn get_blueprint_hyperlink(state: &mut LuaState) -> LuaResult<u32> {
    let code = Option::<String>::from_stack(state, 1)?;
    match code.filter(|code| is_valid_share_code(code)) {
        Some(code) => push_string(
            state,
            &format!("|Hhousingblueprint:{code}|h[Housing Blueprint]|h"),
        ),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

#[cfg(feature = "retail-12-1-0")]
fn get_blueprint_type_for_code(state: &mut LuaState) -> LuaResult<u32> {
    let blueprint_type =
        Option::<String>::from_stack(state, 1)?.and_then(|code| blueprint_type_for_code(&code));
    match blueprint_type {
        Some(blueprint_type) => state.push(Val::Num(f64::from(blueprint_type))),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

#[cfg(feature = "retail-12-1-0")]
fn import_blueprint(state: &mut LuaState) -> LuaResult<u32> {
    let code = Option::<String>::from_stack(state, 1)?.filter(|code| is_valid_share_code(code));
    borrow_state_mut(state)?
        .housing
        .last_imported_blueprint_code = code;
    Ok(0)
}

#[cfg(feature = "retail-12-1-0")]
fn is_share_code_valid(state: &mut LuaState) -> LuaResult<u32> {
    let valid =
        Option::<String>::from_stack(state, 1)?.is_some_and(|code| is_valid_share_code(&code));
    state.push(Val::Bool(valid));
    Ok(1)
}

#[cfg(feature = "retail-12-1-0")]
fn rename_blueprint(state: &mut LuaState) -> LuaResult<u32> {
    let blueprint_id = string_arg_or_empty(state, 1)?;
    let name = string_arg_or_empty(state, 2)?;
    borrow_state_mut(state)?.housing.last_renamed_blueprint = Some((blueprint_id, name));
    Ok(0)
}

#[cfg(feature = "retail-12-1-0")]
fn request_blueprint_collection(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?
        .housing
        .requested_blueprint_collection = true;
    Ok(0)
}

#[cfg(feature = "retail-12-1-0")]
fn request_blueprint_contents(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?
        .housing
        .last_requested_blueprint_contents_id = Option::<String>::from_stack(state, 1)?;
    Ok(0)
}

#[cfg(feature = "retail-12-1-0")]
fn request_blueprint_contents_for_context(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?
        .housing
        .requested_blueprint_context_contents = true;
    Ok(0)
}

#[cfg(feature = "retail-12-1-0")]
fn start_import_room_blueprint(state: &mut LuaState) -> LuaResult<u32> {
    let code =
        Option::<String>::from_stack(state, 1)?.filter(|code| is_valid_room_blueprint_code(code));
    borrow_state_mut(state)?
        .housing
        .last_imported_room_blueprint_code = code;
    Ok(0)
}

#[cfg(feature = "retail-12-1-0")]
fn string_arg_or_empty(state: &mut LuaState, index: i32) -> LuaResult<String> {
    Ok(Option::<String>::from_stack(state, index)?.unwrap_or_default())
}

#[cfg(feature = "retail-12-1-0")]
fn push_string(state: &mut LuaState, value: &str) {
    let value = create_string(state, value);
    state.push(value);
}

#[cfg(feature = "retail-12-1-0")]
fn blueprint_code(prefix: &str, blueprint_id: &str) -> String {
    format!("{prefix}{blueprint_id}")
}

#[cfg(feature = "retail-12-1-0")]
fn is_valid_share_code(code: &str) -> bool {
    blueprint_type_for_code(code).is_some()
}

#[cfg(feature = "retail-12-1-0")]
fn is_valid_room_blueprint_code(code: &str) -> bool {
    code.strip_prefix(ROOM_BLUEPRINT_CODE_PREFIX)
        .is_some_and(|id| !id.is_empty())
}

#[cfg(feature = "retail-12-1-0")]
fn blueprint_type_for_code(code: &str) -> Option<i32> {
    if code
        .strip_prefix(BLUEPRINT_CODE_PREFIX)
        .is_some_and(|id| !id.is_empty())
    {
        Some(BLUEPRINT_TYPE_HOUSE)
    } else if is_valid_room_blueprint_code(code) {
        Some(BLUEPRINT_TYPE_ROOM)
    } else {
        None
    }
}
