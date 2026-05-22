//! Temporary UIParent managed-frame mixin guard.
//!
//! Blizzard wires `layoutParent` through an eager global template key. During
//! simulator startup some managed frames can fire visibility handlers before
//! that global exists, so the mixin needs nil-safe add/remove calls.

use crate::lua_api::LoaderEnv;
#[cfg(test)]
use crate::lua_api::WowLuaEnv;

const UIPARENT_MANAGED_FRAME_MIXIN_LUA: &str = r#"
if UIParentManagedFrameMixin ~= nil then
    function UIParentManagedFrameMixin:OnShow()
        if self.layoutParent and self.layoutParent.AddManagedFrame then
            self.layoutParent:AddManagedFrame(self)
        end
    end
    function UIParentManagedFrameMixin:OnHide()
        if self.layoutParent and self.layoutParent.RemoveManagedFrame then
            self.layoutParent:RemoveManagedFrame(self)
        end
    end
end
"#;

pub(crate) fn patch(env: &LoaderEnv<'_>) -> Result<(), crate::Error> {
    env.exec(UIPARENT_MANAGED_FRAME_MIXIN_LUA)
}

#[cfg(test)]
fn patch_env(env: &WowLuaEnv) {
    env.exec(UIPARENT_MANAGED_FRAME_MIXIN_LUA)
        .expect("UIParent managed-frame mixin patch should install");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guarded_handlers_add_and_remove_when_layout_parent_exists() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_managed_frame_fixture(&env);

        patch_env(&env);

        let (added, removed): (i64, i64) = env
            .eval(
                r#"
                local frame = { layoutParent = layoutParent }
                UIParentManagedFrameMixin.OnShow(frame)
                UIParentManagedFrameMixin.OnHide(frame)
                return layoutParent.added, layoutParent.removed
                "#,
            )
            .expect("guarded managed-frame handlers should run");

        assert_eq!((added, removed), (1, 1));
    }

    #[test]
    fn guarded_handlers_ignore_missing_layout_parent() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_managed_frame_fixture(&env);

        patch_env(&env);

        let ok: bool = env
            .eval(
                r#"
                local frame = {}
                UIParentManagedFrameMixin.OnShow(frame)
                UIParentManagedFrameMixin.OnHide(frame)
                return true
                "#,
            )
            .expect("missing layoutParent should not error");

        assert!(ok);
    }

    fn install_managed_frame_fixture(env: &WowLuaEnv) {
        env.exec(
            r#"
            UIParentManagedFrameMixin = {}
            layoutParent = {
                added = 0,
                removed = 0,
                AddManagedFrame = function(self)
                    self.added = self.added + 1
                end,
                RemoveManagedFrame = function(self)
                    self.removed = self.removed + 1
                end,
            }
            "#,
        )
        .expect("managed-frame fixture should install");
    }
}
