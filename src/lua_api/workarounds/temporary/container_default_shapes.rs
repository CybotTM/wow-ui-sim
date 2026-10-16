//! Temporary C_Container no-state defaults.
//!
//! Purchase/refund, bag filtering, battle-pay markers, and
//! related metadata depend on systems that are not modeled yet.
//! Real bag contents and item metadata stay in the state-backed C_Container
//! surface.

const CONTAINER_DEFAULT_SHAPES_LUA: &str = r#"
C_Container = C_Container or __wow_namespace()

if rawget(C_Container, "GetContainerItemPurchaseInfo") == nil then
    function C_Container.GetContainerItemPurchaseInfo(_bag, _slot)
        return nil
    end
end

if rawget(C_Container, "IsContainerFiltered") == nil then
    function C_Container.IsContainerFiltered(_bag)
        return false
    end
end

if rawget(C_Container, "IsBattlePayItem") == nil then
    function C_Container.IsBattlePayItem(_bag, _slot)
        return false
    end
end

"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(CONTAINER_DEFAULT_SHAPES_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_container_default_shapes() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let result: (bool, bool, bool, bool, bool) = env
            .eval(
                r#"
                local questInfo = C_Container.GetContainerItemQuestInfo(0, 1)
                local actionsOk =
                    pcall(C_Container.UseContainerItem, 0, 1)
                    and pcall(C_Container.PickupContainerItem, 0, 1)
                    and pcall(C_Container.SplitContainerItem, 0, 1, 1)
                return C_Container.GetContainerItemPurchaseInfo(0, 1) == nil,
                       type(questInfo) == "table"
                           and questInfo.isQuestItem == false
                           and questInfo.questID == nil
                           and questInfo.isActive == false,
                       C_Container.IsContainerFiltered(0),
                       C_Container.IsBattlePayItem(0, 1),
                       actionsOk
                "#,
            )
            .expect("container default shapes should be callable");

        assert_eq!(result, (true, true, false, false, true));
    }

    #[test]
    fn preserves_existing_container_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_Container = C_Container or __wow_namespace()

            function C_Container.GetContainerItemPurchaseInfo(_bag, _slot)
                return "purchase"
            end

            function C_Container.IsContainerFiltered(_bag)
                return true
            end
            "#,
        )
        .expect("fixture should install existing container provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: (String, bool) = env
            .eval(
                r#"
                return C_Container.GetContainerItemPurchaseInfo(0, 1),
                       C_Container.IsContainerFiltered(0)
                "#,
            )
            .expect("existing container provider should remain callable");

        assert_eq!(result, ("purchase".to_string(), true));
    }
}
