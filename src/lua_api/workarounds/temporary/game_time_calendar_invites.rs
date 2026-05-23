//! Temporary GameTime calendar defaults.
//!
//! Calendar invite and local clock state are not modeled yet, but Blizzard
//! startup expects these fields/globals before real calendar data has loaded.

use crate::lua_api::WowLuaEnv;

const GAME_TIME_BOOTSTRAP_LUA: &str = r#"
if GameTime_GetTime == nil then
    function GameTime_GetTime(_useLocalTime)
        return "12:00"
    end
end
"#;

const GAME_TIME_CALENDAR_INVITES_LUA: &str = r#"
if type(GameTimeFrame) == "table" and GameTimeFrame.pendingCalendarInvites == nil then
    GameTimeFrame.pendingCalendarInvites = 0
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(GAME_TIME_BOOTSTRAP_LUA)?;
    Ok(())
}

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(GAME_TIME_CALENDAR_INVITES_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_missing_game_time_clock_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("GameTime_GetTime = nil")
            .expect("game time fixture should clear global");

        {
            let mut lua = env.lua.borrow_mut();
            apply_bootstrap(&mut lua).expect("game time bootstrap should apply");
        }

        let time: String = env
            .eval("return GameTime_GetTime(true)")
            .expect("game time default should be callable");

        assert_eq!(time, "12:00");
    }

    #[test]
    fn preserves_existing_game_time_clock_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(r#"GameTime_GetTime = function() return "03:14" end"#)
            .expect("game time fixture should install existing global");

        {
            let mut lua = env.lua.borrow_mut();
            apply_bootstrap(&mut lua).expect("game time bootstrap should apply");
        }

        let time: String = env
            .eval("return GameTime_GetTime(true)")
            .expect("game time default should be callable");

        assert_eq!(time, "03:14");
    }

    #[test]
    fn seeds_missing_pending_calendar_invites() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("GameTimeFrame = {}")
            .expect("game time frame fixture should install");

        patch(&env);

        let pending_invites: i64 = env
            .eval("return GameTimeFrame.pendingCalendarInvites")
            .expect("pending calendar invites should be readable");

        assert_eq!(pending_invites, 0);
    }

    #[test]
    fn preserves_existing_pending_calendar_invites() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec("GameTimeFrame = { pendingCalendarInvites = 3 }")
            .expect("game time frame fixture should install");

        patch(&env);

        let pending_invites: i64 = env
            .eval("return GameTimeFrame.pendingCalendarInvites")
            .expect("pending calendar invites should be readable");

        assert_eq!(pending_invites, 3);
    }
}
