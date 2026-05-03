use super::env;

#[test]
fn test_traits_generate_import_string() {
    let env = env();
    let s: String = env.eval("return C_Traits.GenerateImportString(1)").unwrap();
    assert!(!s.is_empty());
}

#[test]
fn test_traits_get_config_id_by_system_id() {
    let env = env();
    let id: i32 = env
        .eval("return C_Traits.GetConfigIDBySystemID(1)")
        .unwrap();
    assert_eq!(id, 201);
}

#[test]
fn test_traits_get_config_id_by_tree_id() {
    let env = env();
    let id: i32 = env
        .eval("return C_Traits.GetConfigIDByTreeID(790)")
        .unwrap();
    assert_eq!(id, 201);
}

#[test]
fn test_traits_get_config_info() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Traits.GetConfigInfo(1)) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_traits_get_config_info_exposes_tree_ids() {
    let env = env();
    let first_tree_id: i32 = env
        .eval("return C_Traits.GetConfigInfo(201).treeIDs[1]")
        .unwrap();
    assert_eq!(first_tree_id, 790);
}

#[test]
fn test_traits_get_node_info_unknown() {
    let env = env();
    let id: i32 = env.eval("return C_Traits.GetNodeInfo(1, 1).ID").unwrap();
    assert_eq!(id, 1);
}

#[test]
fn test_traits_get_node_info_exposes_position_fields() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local nodeID = C_Traits.GetTreeNodes(201, 790)[1]
            local info = C_Traits.GetNodeInfo(201, nodeID)
            if type(info) ~= "table" then
                return "expected_table"
            end
            if info.posX == nil or info.posY == nil then
                return "missing_position"
            end
            if info.type == nil or info.flags == nil then
                return "missing_node_shape"
            end
            if type(info.visibleEdges) ~= "table" then
                return "missing_edges"
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_traits_get_entry_info_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Traits.GetEntryInfo(1, 1) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_traits_get_condition_info_exposes_condition_shape() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local configID = 201
            local treeID = C_Traits.GetConfigInfo(configID).treeIDs[1]
            for _, nodeID in ipairs(C_Traits.GetTreeNodes(configID, treeID)) do
                local node = C_Traits.GetNodeInfo(configID, nodeID)
                for _, conditionID in ipairs(node.conditionIDs or {}) do
                    local info = C_Traits.GetConditionInfo(configID, conditionID)
                    if type(info) ~= "table" then
                        return "expected_table"
                    end
                    if info.condID ~= conditionID then
                        return "bad_id"
                    end
                    if type(info.ranksGranted) ~= "number" then
                        return "missing_ranks"
                    end
                    if type(info.isMet) ~= "boolean" or type(info.isSufficient) ~= "boolean" then
                        return "missing_state"
                    end
                    if type(info.type) ~= "number" then
                        return "missing_type"
                    end
                    return "ok"
                end
            end
            return "no_conditions"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_traits_get_condition_info_unknown_returns_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Traits.GetConditionInfo(201, 999999999) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_traits_initialize_view_loadout() {
    let env = env();
    let ok: bool = env
        .eval("return C_Traits.InitializeViewLoadout(1, 1)")
        .unwrap();
    assert!(ok);
}

#[test]
fn test_traits_get_tree_info_valid() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Traits.GetTreeInfo(1, 1)) == 'table'")
        .unwrap();
    assert!(is_table, "Tree 1 exists in TRAIT_TREE_DB");
}

#[test]
fn test_traits_get_tree_hash_is_stable_and_tree_specific() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local function hashString(treeID)
                local hash = C_Traits.GetTreeHash(treeID)
                assert(type(hash) == "table" and #hash == 16, "expected a 16-lane tree hash")
                return table.concat(hash, ",")
            end

            return hashString(790) .. "|" .. hashString(999999)
            "#,
        )
        .unwrap();

    let (paladin_hash, missing_hash) = result
        .split_once('|')
        .expect("hash result should contain a separator");
    assert!(
        paladin_hash.split(',').any(|lane| lane != "0"),
        "known trees should produce a non-zero tree hash"
    );
    assert!(
        missing_hash.split(',').all(|lane| lane == "0"),
        "unknown trees should produce an all-zero tree hash"
    );
    assert_ne!(paladin_hash, missing_hash);
}

#[test]
fn test_traits_get_tree_currency_info_exposes_currency_fields() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local info = C_Traits.GetTreeCurrencyInfo(201, 790, false)
            if type(info) ~= "table" then
                return "expected_table"
            end
            if type(info[1]) ~= "table" then
                return "expected_entry"
            end
            if info[1].traitCurrencyID == nil then
                return "missing_currency_id"
            end
            if info[1].quantity == nil then
                return "missing_quantity"
            end
            if info[1].spent == nil then
                return "missing_spent"
            end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_traits_get_trait_currency_info_returns_currency_type() {
    let env = env();
    let has_currency_type: bool = env
        .eval(
            r#"
            local _, _, currencyTypesID = C_Traits.GetTraitCurrencyInfo(2801)
            return currencyTypesID ~= nil
            "#,
        )
        .unwrap();
    assert!(has_currency_type);
}

#[test]
fn test_traits_get_tree_info_nil_invalid() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_Traits.GetTreeInfo(1, 999999) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_traits_get_tree_nodes_empty() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Traits.GetTreeNodes(1, 1)) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_traits_get_all_tree_ids_empty() {
    let env = env();
    let is_table: bool = env
        .eval("return type(C_Traits.GetAllTreeIDs()) == 'table'")
        .unwrap();
    assert!(is_table);
}

#[test]
fn test_traits_get_trait_system_flags() {
    let env = env();
    let flags: i32 = env.eval("return C_Traits.GetTraitSystemFlags(1)").unwrap();
    assert_eq!(flags, 2);
}

#[test]
fn test_traits_get_trait_system_flags_for_class_config() {
    let env = env();
    let flags: i32 = env
        .eval("return C_Traits.GetTraitSystemFlags(201)")
        .unwrap();
    assert_eq!(flags, 0);
}

#[test]
fn test_traits_can_purchase_rank() {
    let env = env();
    let can: bool = env
        .eval(
            r#"
            local configID = C_ClassTalents.GetActiveConfigID()
            local treeID = C_Traits.GetConfigInfo(configID).treeIDs[1]
            for _, nodeID in ipairs(C_Traits.GetTreeNodes(treeID)) do
                local nodeInfo = C_Traits.GetNodeInfo(configID, nodeID)
                if nodeInfo and nodeInfo.canPurchaseRank and nodeInfo.entryIDs and nodeInfo.entryIDs[1] then
                    return C_Traits.CanPurchaseRank(configID, nodeID, nodeInfo.entryIDs[1])
                end
            end
            return false
            "#,
        )
        .unwrap();
    assert!(can);
}

#[test]
fn test_traits_can_purchase_rank_matches_node_info() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local configID = C_ClassTalents.GetActiveConfigID()
            local treeID = C_Traits.GetConfigInfo(configID).treeIDs[1]
            for _, nodeID in ipairs(C_Traits.GetTreeNodes(treeID)) do
                local nodeInfo = C_Traits.GetNodeInfo(configID, nodeID)
                if nodeInfo and nodeInfo.canPurchaseRank and nodeInfo.entryIDs and nodeInfo.entryIDs[1] then
                    local entryID = nodeInfo.entryIDs[1]
                    assert(C_Traits.CanPurchaseRank(configID, nodeID, entryID) == true, "CanPurchaseRank should match nodeInfo.canPurchaseRank")
                    return "ok"
                end
            end
            error("expected a purchasable node in the active config")
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_traits_staged_changes_are_visible_after_purchase() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local configID = C_ClassTalents.GetActiveConfigID()
            local treeID = C_Traits.GetConfigInfo(configID).treeIDs[1]
            local nodeID = nil
            local entryID = nil
            for _, candidateNodeID in ipairs(C_Traits.GetTreeNodes(treeID)) do
                local candidateInfo = C_Traits.GetNodeInfo(configID, candidateNodeID)
                if candidateInfo and candidateInfo.canPurchaseRank and candidateInfo.entryIDs and candidateInfo.entryIDs[1] then
                    nodeID = candidateNodeID
                    entryID = candidateInfo.entryIDs[1]
                    break
                end
            end
            assert(nodeID and entryID, "expected a purchasable node in the active config")

            assert(C_Traits.ConfigHasStagedChanges(configID) == false, "fresh config should not have staged changes")
            local purchases, refunds, swaps = C_Traits.GetStagedChanges(configID)
            assert(purchases == nil and refunds == nil and swaps == nil, "fresh config should not report staged changes")

            assert(C_Traits.CanPurchaseRank(configID, nodeID, entryID) == true, "node should be purchasable before the change")
            assert(C_Traits.PurchaseRank(configID, nodeID) == true, "purchase should succeed")
            assert(C_Traits.ConfigHasStagedChanges(configID) == true, "purchase should mark staged changes")

            purchases, refunds, swaps = C_Traits.GetStagedChanges(configID)
            assert(purchases and #purchases > 0 and purchases[1] == nodeID, "purchase should be reported for the changed node")
            assert(refunds and #refunds == 0, "purchase should not report refunds")
            assert(swaps and #swaps == 0, "purchase should not report selection swaps")

            local costs = C_Traits.GetStagedChangesCost(configID)
            assert(costs and #costs > 0, "purchase should report staged currency cost")
            local sawAmount = false
            for _, cost in ipairs(costs) do
                if cost.amount ~= 0 then
                    sawAmount = true
                    break
                end
            end
            assert(sawAmount, "staged costs should contain a non-zero amount")

            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_traits_get_loadout_serialization_version() {
    let env = env();
    let ver: i32 = env
        .eval("return C_Traits.GetLoadoutSerializationVersion()")
        .unwrap();
    assert_eq!(ver, 2);
}
