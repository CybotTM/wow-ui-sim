use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn c_tooltip_info_socket_getters_delegate_to_item_tooltips() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            C_ItemSocketInfo._state.itemInfo = { itemID = 6948 }
            C_ItemSocketInfo._state.newSockets = {
                [1] = { itemID = 6948 },
            }
            C_ItemSocketInfo._state.existingSockets = {
                [1] = { link = "item:6948" },
            }

            local socketed = C_TooltipInfo.GetSocketedItem()
            local newGem = C_TooltipInfo.GetSocketGem(1)
            local existingGem = C_TooltipInfo.GetExistingSocketGem(1)
            local baseline = C_TooltipInfo.GetItemByID(6948)

            if socketed.lines[1].leftText ~= baseline.lines[1].leftText then
                return "socketed_item_should_delegate_to_item_tooltip"
            end
            if newGem.lines[1].leftText ~= baseline.lines[1].leftText then
                return "socket_gem_should_delegate_to_item_tooltip"
            end
            if existingGem.lines[1].leftText ~= baseline.lines[1].leftText then
                return "existing_socket_gem_should_delegate_to_item_tooltip"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "Socket tooltip getters should reuse the normal item tooltip path"
    );
}
