//! Temporary global frame defaults for partial Blizzard loads.
//!
//! These named frames are shallow compatibility placeholders. Keep them out of
//! the generic runtime bootstrap so their workaround status stays explicit.

const GLOBAL_FRAME_DEFAULTS_LUA: &str = r#"
local function ensure_named_frame(frameType, name)
    if rawget(_G, name) ~= nil then
        return rawget(_G, name)
    end
    if type(CreateFrame) ~= "function" then
        return nil
    end
    return CreateFrame(frameType or "Frame", name, UIParent)
end

ensure_named_frame("Frame", "EventToastManagerFrame")
ensure_named_frame("Frame", "EditModeManagerFrame")
ensure_named_frame("Frame", "RolePollPopup")
ensure_named_frame("Frame", "TimerTracker")
ensure_named_frame("MessageFrame", "UIErrorsFrame")
ensure_named_frame("Frame", "SideDressUpFrame")
ensure_named_frame("Frame", "ContainerFrameCombinedBags")
ensure_named_frame("Frame", "LootFrame")
ensure_named_frame("Frame", "RaidWarningFrame")
ensure_named_frame("Frame", "GossipFrame")
ensure_named_frame("Frame", "FriendsFrame")
ensure_named_frame("Frame", "HelpFrame")
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(GLOBAL_FRAME_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_global_frame_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("global frame defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                local names = {
                    "EventToastManagerFrame",
                    "EditModeManagerFrame",
                    "RolePollPopup",
                    "TimerTracker",
                    "UIErrorsFrame",
                    "SideDressUpFrame",
                    "ContainerFrameCombinedBags",
                    "LootFrame",
                    "RaidWarningFrame",
                    "GossipFrame",
                    "FriendsFrame",
                    "HelpFrame",
                }
                for _, name in ipairs(names) do
                    local frame = rawget(_G, name)
                    if type(frame) ~= "table" then
                        return name
                    end
                end
                return "ok"
                "#,
            )
            .expect("global frame defaults probe should run");

        assert_eq!(result, "ok");
    }
}
