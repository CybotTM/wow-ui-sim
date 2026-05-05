//! Focused tests for hero talent selection-node helpers.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn subtree_selection_node_ids_are_unique_and_sorted() {
    let env = env();
    let (count, unique_count, is_sorted): (i32, i32, bool) = env
        .eval(
            r#"
            local info = C_Traits.GetSubTreeInfo(1, 48)
            assert(info, "subtree info should not be nil")
            assert(info.subTreeSelectionNodeIDs, "selection nodes should be present")

            local seen = {}
            local isSorted = true
            for i, nodeID in ipairs(info.subTreeSelectionNodeIDs) do
                seen[nodeID] = true
                if i > 1 and info.subTreeSelectionNodeIDs[i - 1] > nodeID then
                    isSorted = false
                end
            end

            local uniqueCount = 0
            for _ in pairs(seen) do
                uniqueCount = uniqueCount + 1
            end

            return #info.subTreeSelectionNodeIDs, uniqueCount, isSorted
            "#,
        )
        .unwrap();

    assert!(count > 0, "Templar should have selection nodes");
    assert_eq!(unique_count, count, "selection nodes should be unique");
    assert!(is_sorted, "selection nodes should remain sorted");
}
