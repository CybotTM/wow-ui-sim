//! Temporary Blizzard_SharedXML animation mixin repair.
//!
//! Some SharedXML animation mixins expect `SetPlaying` to exist after Blizzard
//! loads them. Keep that compatibility patch isolated until the animation group
//! lifecycle exposes the same method from the simulator side.

use crate::lua_api::LoaderEnv;
#[cfg(test)]
use crate::lua_api::WowLuaEnv;

const SHARED_XML_ANIM_MIXINS_LUA: &str = r#"
local mixins = {
    VisibleWhilePlayingAnimGroupMixin,
    TargetsVisibleWhilePlayingAnimGroupMixin,
    SyncedAnimGroupMixin,
}

for _, mixin in ipairs(mixins) do
    if type(mixin) == "table" and type(mixin.SetPlaying) ~= "function" then
        function mixin:SetPlaying(playing)
            if playing then
                if type(self.Show) == "function" then
                    self:Show()
                end
                if type(self.PlaySynced) == "function" then
                    self:PlaySynced()
                else
                    self:Play()
                end
            else
                self:Stop()
                if type(self.Hide) == "function" then
                    self:Hide()
                end
            end
        end
    end
end
"#;

pub(crate) fn patch(env: &LoaderEnv<'_>) -> Result<(), crate::Error> {
    env.exec(SHARED_XML_ANIM_MIXINS_LUA)
}

#[cfg(test)]
fn patch_env(env: &WowLuaEnv) {
    env.exec(SHARED_XML_ANIM_MIXINS_LUA)
        .expect("SharedXML animation mixin patch should install");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_set_playing_on_missing_animation_mixins() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_animation_mixin_fixture(&env);

        patch_env(&env);

        let (visible_state, synced_state): (String, String) = env
            .eval(
                r#"
                local visible = {
                    log = {},
                    Show = function(self) table.insert(self.log, "show") end,
                    Hide = function(self) table.insert(self.log, "hide") end,
                    Play = function(self) table.insert(self.log, "play") end,
                    Stop = function(self) table.insert(self.log, "stop") end,
                }
                setmetatable(visible, { __index = VisibleWhilePlayingAnimGroupMixin })
                visible:SetPlaying(true)
                visible:SetPlaying(false)

                local synced = {
                    log = {},
                    Play = function(self) table.insert(self.log, "play") end,
                    PlaySynced = function(self) table.insert(self.log, "play_synced") end,
                    Stop = function(self) table.insert(self.log, "stop") end,
                }
                setmetatable(synced, { __index = SyncedAnimGroupMixin })
                synced:SetPlaying(true)

                return table.concat(visible.log, ","),
                    table.concat(synced.log, ",")
                "#,
            )
            .expect("SetPlaying methods should run");

        assert_eq!(visible_state, "show,play,stop,hide");
        assert_eq!(synced_state, "play_synced");
    }

    #[test]
    fn preserves_existing_set_playing_method() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            VisibleWhilePlayingAnimGroupMixin = {
                SetPlaying = function(self)
                    self.preserved = true
                end,
            }
            TargetsVisibleWhilePlayingAnimGroupMixin = {}
            SyncedAnimGroupMixin = {}
            "#,
        )
        .expect("animation mixin fixture should install");

        patch_env(&env);

        let preserved: bool = env
            .eval(
                r#"
                local instance = {}
                setmetatable(instance, { __index = VisibleWhilePlayingAnimGroupMixin })
                instance:SetPlaying(true)
                return instance.preserved == true
                "#,
            )
            .expect("existing SetPlaying method should run");

        assert!(preserved);
    }

    fn install_animation_mixin_fixture(env: &WowLuaEnv) {
        env.exec(
            r#"
            VisibleWhilePlayingAnimGroupMixin = {}
            TargetsVisibleWhilePlayingAnimGroupMixin = {}
            SyncedAnimGroupMixin = {}
            "#,
        )
        .expect("animation mixin fixture should install");
    }
}
