use super::*;

type LuaTableRef = rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>;
type RustLuaFn = rilua::vm::closure::RustFn;

const C_ADDONS_QUERY_METHODS: &[(&str, RustLuaFn)] = &[
    ("GetNumAddOns", c_addons_get_num_addons),
    ("GetAddOnInfo", c_addons_get_addon_info),
    ("IsAddOnLoaded", c_addons_is_addon_loaded),
    ("IsAddOnLoadable", c_addons_is_addon_loadable),
    ("IsAddOnLoadOnDemand", c_addons_is_addon_load_on_demand),
    ("GetAddOnEnableState", c_addons_get_addon_enable_state),
    ("GetAddOnMetadata", c_addons_get_addon_metadata),
    ("DoesAddOnExist", c_addons_does_addon_exist),
    ("GetAddOnName", c_addons_get_addon_name),
    ("GetAddOnTitle", c_addons_get_addon_title),
    ("GetAddOnNotes", c_addons_get_addon_notes),
    ("GetAddOnSecurity", c_addons_get_addon_security),
    ("GetAddOnDependencies", c_addons_get_addon_dependencies),
    ("IsAddOnDefaultEnabled", c_addons_is_addon_default_enabled),
];

const C_ADDONS_STATE_METHODS: &[(&str, RustLuaFn)] = &[
    ("EnableAddOn", c_addons_enable_addon),
    ("DisableAddOn", c_addons_disable_addon),
    ("EnableAllAddOns", c_addons_enable_all_addons),
    ("DisableAllAddOns", c_addons_disable_all_addons),
    ("SaveAddOns", c_addons_save_addons),
    ("ResetAddOns", c_addons_reset_addons),
    (
        "IsAddonVersionCheckEnabled",
        c_addons_is_addon_version_check_enabled,
    ),
    ("SetAddonVersionCheck", c_addons_set_addon_version_check),
    ("LoadAddOn", c_addons_load_addon),
];

pub(super) fn register_c_addons_methods(state: &mut LuaState, t: LuaTableRef) -> LuaResult<()> {
    register_method_group(state, t, C_ADDONS_QUERY_METHODS)?;
    register_method_group(state, t, C_ADDONS_STATE_METHODS)?;
    Ok(())
}

fn register_method_group(
    state: &mut LuaState,
    t: LuaTableRef,
    methods: &[(&'static str, RustLuaFn)],
) -> LuaResult<()> {
    for (name, rust_fn) in methods {
        table_set_rust_fn_static(state, t, name, *rust_fn)?;
    }
    Ok(())
}
