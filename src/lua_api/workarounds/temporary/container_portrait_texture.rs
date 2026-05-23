//! Temporary C_Container bag portrait texture helper.
//!
//! Real WoW derives bag portrait art from equipped inventory data. The
//! simulator only has a shallow inventory icon path today, so this helper keeps
//! Blizzard bag frames rendering while the richer inventory/equipment model is
//! still missing.

const CONTAINER_PORTRAIT_TEXTURE_LUA: &str = r#"
-- C_Container's metatable can expose generated method stubs through __index;
-- rawget keeps this workaround tied to the explicit table slot.
if C_Container ~= nil and type(rawget(C_Container, "SetBagPortraitTexture")) ~= "function" then
  function C_Container.SetBagPortraitTexture(texture, bagID)
    if texture ~= nil then
      local inventoryID = C_Container.ContainerIDToInventoryID and C_Container.ContainerIDToInventoryID(bagID)
      if inventoryID == nil and type(bagID) == "number" then
        inventoryID = 20 + bagID
      end
      local portraitTexture = GetInventoryItemTexture("player", inventoryID or 20)
      texture:SetTexture(portraitTexture)
    end
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(CONTAINER_PORTRAIT_TEXTURE_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_bag_portrait_texture_helper_when_missing() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            rawset(C_Container, "SetBagPortraitTexture", nil)
            C_Container.ContainerIDToInventoryID = function()
              return 21
            end
            GetInventoryItemTexture = function()
              return "portrait_texture"
            end
            "#,
        )
        .expect("fixture should reset helper and inventory texture");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("container portrait helper should apply");
        }
        let helper_type: String = env
            .eval("return type(C_Container.SetBagPortraitTexture)")
            .expect("helper type probe should run");
        assert_eq!(helper_type, "function");

        let texture: String = env
            .eval(
                r#"
                local portrait = {
                  value = nil,
                  SetTexture = function(self, texture)
                    self.value = texture
                  end,
                }
                C_Container.SetBagPortraitTexture(portrait, Enum.BagIndex.Bag_1)
                return portrait.value
                "#,
            )
            .expect("bag portrait helper should run");

        assert_eq!(texture, "portrait_texture");
    }

    #[test]
    fn preserves_existing_bag_portrait_texture_helper() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_Container.SetBagPortraitTexture = function(texture)
              texture:SetTexture("custom")
            end
            "#,
        )
        .expect("fixture should install custom helper");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("container portrait helper should apply");
        }

        let texture: String = env
            .eval(
                r#"
                local frame = CreateFrame("Frame")
                local portrait = frame:CreateTexture(nil, "ARTWORK")
                C_Container.SetBagPortraitTexture(portrait, Enum.BagIndex.Bag_1)
                return portrait:GetTexture()
                "#,
            )
            .expect("custom bag portrait helper should run");

        assert_eq!(texture, "custom");
    }
}
