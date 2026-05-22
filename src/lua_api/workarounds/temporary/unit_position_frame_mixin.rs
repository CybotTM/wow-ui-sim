//! Temporary UnitPositionFrame/GroupMembersPin OnHide guard.
//!
//! These mixins can receive `OnHide` before `SetDataProvider`/`OnAcquired`
//! during initial load. Guard the Blizzard handler until the simulator models
//! that acquisition order.

use crate::lua_api::LoaderEnv;
#[cfg(test)]
use crate::lua_api::WowLuaEnv;

const UNIT_POSITION_FRAME_MIXIN_LUA: &str = r#"
if UnitPositionFrameMixin ~= nil then
    local orig = UnitPositionFrameMixin.OnHide
    UnitPositionFrameMixin.OnHide = function(self, ...)
        if self.dataProvider then
            return orig(self, ...)
        end
    end
end
if GroupMembersPinMixin ~= nil then
    local orig = GroupMembersPinMixin.OnHide
    GroupMembersPinMixin.OnHide = function(self, ...)
        if self.dataProvider then
            return orig(self, ...)
        end
    end
end
"#;

pub(crate) fn patch(env: &LoaderEnv<'_>) -> Result<(), crate::Error> {
    env.exec(UNIT_POSITION_FRAME_MIXIN_LUA)
}

#[cfg(test)]
fn patch_env(env: &WowLuaEnv) {
    env.exec(UNIT_POSITION_FRAME_MIXIN_LUA)
        .expect("UnitPosition frame mixin patch should install");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_original_onhide_when_data_provider_exists() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_unit_position_fixture(&env);

        patch_env(&env);

        let (unit_count, group_count): (i64, i64) = env
            .eval(
                r#"
                UnitPositionFrameMixin.OnHide({ dataProvider = true })
                GroupMembersPinMixin.OnHide({ dataProvider = true })
                return unitPositionOnHideCount, groupMembersOnHideCount
                "#,
            )
            .expect("guarded OnHide handlers should run");

        assert_eq!((unit_count, group_count), (1, 1));
    }

    #[test]
    fn ignores_onhide_when_data_provider_is_missing() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_unit_position_fixture(&env);

        patch_env(&env);

        let (unit_count, group_count): (i64, i64) = env
            .eval(
                r#"
                UnitPositionFrameMixin.OnHide({})
                GroupMembersPinMixin.OnHide({})
                return unitPositionOnHideCount, groupMembersOnHideCount
                "#,
            )
            .expect("missing dataProvider should not call original handlers");

        assert_eq!((unit_count, group_count), (0, 0));
    }

    fn install_unit_position_fixture(env: &WowLuaEnv) {
        env.exec(
            r#"
            unitPositionOnHideCount = 0
            groupMembersOnHideCount = 0
            UnitPositionFrameMixin = {
                OnHide = function()
                    unitPositionOnHideCount = unitPositionOnHideCount + 1
                end,
            }
            GroupMembersPinMixin = {
                OnHide = function()
                    groupMembersOnHideCount = groupMembersOnHideCount + 1
                end,
            }
            "#,
        )
        .expect("UnitPosition frame fixture should install");
    }
}
