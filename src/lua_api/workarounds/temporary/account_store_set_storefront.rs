//! Temporary Account Store storefront setter repair.
//!
//! Account Store storefront model state is not represented yet. This keeps the
//! frame/mixin setter available for startup code without pretending the backing
//! Account Store domain exists.

use crate::lua_api::LoaderEnv;
#[cfg(test)]
use crate::lua_api::WowLuaEnv;

const ACCOUNT_STORE_SET_STOREFRONT_LUA: &str = r#"
local function __wow_account_store_set_storefront_id(self, storeFrontID)
    self.storeFrontID = storeFrontID
end

if type(AccountStoreMixin) == "table" then
    AccountStoreMixin.SetStoreFrontID = __wow_account_store_set_storefront_id
end
if type(AccountStoreFrame) == "table" then
    AccountStoreFrame.SetStoreFrontID = __wow_account_store_set_storefront_id
end
"#;

pub(crate) fn patch(env: &LoaderEnv<'_>) -> Result<(), crate::Error> {
    env.exec(ACCOUNT_STORE_SET_STOREFRONT_LUA)
}

#[cfg(test)]
fn patch_env(env: &WowLuaEnv) {
    env.exec(ACCOUNT_STORE_SET_STOREFRONT_LUA)
        .expect("account store storefront patch should install");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_storefront_setter_on_mixin_and_live_frame() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            AccountStoreMixin = {}
            AccountStoreFrame = {}
            "#,
        )
        .expect("account store fixture should install");

        patch_env(&env);

        let (mixin_storefront_id, frame_storefront_id): (i64, i64) = env
            .eval(
                r#"
                local mixinInstance = {}
                AccountStoreMixin.SetStoreFrontID(mixinInstance, 44)
                AccountStoreFrame:SetStoreFrontID(55)
                return mixinInstance.storeFrontID, AccountStoreFrame.storeFrontID
                "#,
            )
            .expect("account store storefront setters should run");

        assert_eq!(mixin_storefront_id, 44);
        assert_eq!(frame_storefront_id, 55);
    }
}
