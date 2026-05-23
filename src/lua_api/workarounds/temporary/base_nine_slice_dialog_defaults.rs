//! Temporary `BaseNineSliceDialogMixin` defaults.
//!
//! The real mixin comes from Blizzard SharedXML. These shallow methods keep
//! isolated static-popup/glue addon loads from failing when that provider is
//! absent.

const BASE_NINE_SLICE_DIALOG_DEFAULTS_LUA: &str = r#"
if type(BaseNineSliceDialogMixin) ~= "table" then
  BaseNineSliceDialogMixin = {}
end

if type(BaseNineSliceDialogMixin.OnShow) ~= "function" then
  function BaseNineSliceDialogMixin:OnShow()
  end
end

if type(BaseNineSliceDialogMixin.OnCloseClick) ~= "function" then
  function BaseNineSliceDialogMixin:OnCloseClick()
    if type(self.Hide) == "function" then
      self:Hide()
    end
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(BASE_NINE_SLICE_DIALOG_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_base_nine_slice_dialog_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("base dialog defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                local frame = CreateFrame("Frame")
                BaseNineSliceDialogMixin.OnShow(frame)
                if frame:IsShown() ~= true then return "on_show" end
                BaseNineSliceDialogMixin.OnCloseClick(frame)
                if frame:IsShown() ~= false then return "close_hide" end
                return "ok"
                "#,
            )
            .expect("base dialog defaults should be callable");

        assert_eq!(result, "ok");
    }

    #[test]
    fn preserves_existing_base_nine_slice_dialog_members() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            BaseNineSliceDialogMixin = {
              OnShow = function(self) self.existingShow = true end,
              OnCloseClick = function(self) self.existingClose = true end,
            }
            "#,
        )
        .expect("fixture should install existing dialog mixin");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("base dialog defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                local frame = {}
                BaseNineSliceDialogMixin.OnShow(frame)
                BaseNineSliceDialogMixin.OnCloseClick(frame)
                if frame.existingShow ~= true then return "overwrote_show" end
                if frame.existingClose ~= true then return "overwrote_close" end
                return "ok"
                "#,
            )
            .expect("base dialog preservation probe should run");

        assert_eq!(result, "ok");
    }
}
