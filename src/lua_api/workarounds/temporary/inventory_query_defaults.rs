//! Temporary inventory query defaults not backed by equipment state yet.
//!
//! `GetInventoryItemID` and related equipped-item probes are SimState-backed in
//! `globals::inventory_probes`. Keep only the still-unmodeled bag-slot lookup
//! fallback here instead of in generic global stub tables.

const INVENTORY_QUERY_DEFAULTS_LUA: &str = r#"
if GetInventoryItemsForSlot == nil then
    function GetInventoryItemsForSlot()
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(INVENTORY_QUERY_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_unmodeled_inventory_items_for_slot_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (String, bool) = env
            .eval(
                r#"
                return type(GetInventoryItemsForSlot),
                       GetInventoryItemsForSlot(1) == nil
                "#,
            )
            .expect("inventory slot lookup fallback probe should run");

        assert_eq!(result, ("function".to_string(), true));
    }

    #[test]
    fn preserves_existing_inventory_items_for_slot_function() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("function GetInventoryItemsForSlot() return 'existing' end")
            .expect("fixture should install existing inventory slot lookup");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("inventory query defaults should apply");
        }

        let value: String = env
            .eval("return GetInventoryItemsForSlot(1)")
            .expect("inventory slot lookup preservation probe should run");

        assert_eq!(value, "existing");
    }
}
