use super::*;

fn register_c_traits_query_fns(state: &mut LuaState, table_ref: LuaTableRef) -> LuaResult<()> {
    register_c_traits_config_query_fns(state, table_ref)?;
    register_c_traits_node_query_fns(state, table_ref)?;
    register_c_traits_tree_query_fns(state, table_ref)?;
    Ok(())
}

fn register_c_traits_config_query_fns(
    state: &mut LuaState,
    table_ref: LuaTableRef,
) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table_ref,
        "GenerateImportString",
        c_traits_generate_import_string,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetConfigIDBySystemID",
        c_traits_get_config_id_by_system_id,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetConfigIDByTreeID",
        c_traits_get_config_id_by_tree_id,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetConfigInfo", c_traits_get_config_info)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "InitializeViewLoadout",
        c_traits_initialize_view_loadout,
    )?;
    Ok(())
}

fn register_c_traits_node_query_fns(state: &mut LuaState, table_ref: LuaTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(state, table_ref, "GetNodeInfo", c_traits_get_node_info)?;
    table_set_rust_fn_static(state, table_ref, "GetEntryInfo", c_traits_get_entry_info)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetDefinitionInfo",
        c_traits_get_definition_info,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetTraitDescription",
        c_traits_get_trait_description,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetConditionInfo",
        c_traits_get_condition_info,
    )?;
    Ok(())
}

fn register_c_traits_tree_query_fns(state: &mut LuaState, table_ref: LuaTableRef) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetTreeCurrencyInfo",
        c_traits_get_tree_currency_info,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetTraitCurrencyInfo",
        c_traits_get_trait_currency_info,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetTreeHash", c_traits_get_tree_hash)?;
    table_set_rust_fn_static(state, table_ref, "GetTreeInfo", c_traits_get_tree_info)?;
    table_set_rust_fn_static(state, table_ref, "GetTreeNodes", c_traits_get_tree_nodes)?;
    Ok(())
}

fn register_c_traits_action_fns(state: &mut LuaState, table_ref: LuaTableRef) -> LuaResult<()> {
    register_c_traits_system_action_fns(state, table_ref)?;
    register_c_traits_staged_action_fns(state, table_ref)?;
    register_c_traits_rank_action_fns(state, table_ref)?;
    Ok(())
}

fn register_c_traits_system_action_fns(
    state: &mut LuaState,
    table_ref: LuaTableRef,
) -> LuaResult<()> {
    table_set_rust_fn_static(state, table_ref, "GetAllTreeIDs", c_traits_get_all_tree_ids)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetTraitSystemFlags",
        c_traits_get_trait_system_flags,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "CanPurchaseRank",
        c_traits_can_purchase_rank,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetLoadoutSerializationVersion",
        c_traits_get_loadout_serialization_version,
    )?;
    Ok(())
}

fn register_c_traits_staged_action_fns(
    state: &mut LuaState,
    table_ref: LuaTableRef,
) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table_ref,
        "ConfigHasStagedChanges",
        c_traits_config_has_staged_changes,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetStagedChanges",
        c_traits_get_staged_changes,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetStagedChangesCost",
        c_traits_get_staged_changes_cost,
    )?;
    Ok(())
}

fn register_c_traits_rank_action_fns(
    state: &mut LuaState,
    table_ref: LuaTableRef,
) -> LuaResult<()> {
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetSubTreeInfo",
        c_traits_get_subtree_info,
    )?;
    table_set_rust_fn_static(state, table_ref, "GetNodeCost", c_traits_get_node_cost)?;
    table_set_rust_fn_static(state, table_ref, "SetSelection", c_traits_set_selection)?;
    table_set_rust_fn_static(state, table_ref, "PurchaseRank", c_traits_purchase_rank)?;
    table_set_rust_fn_static(state, table_ref, "RefundRank", c_traits_refund_rank)?;
    Ok(())
}

pub(super) fn register_c_traits(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_Traits")?;
    register_c_traits_query_fns(state, table_ref)?;
    register_c_traits_action_fns(state, table_ref)?;
    Ok(())
}
