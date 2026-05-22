//! Temporary GameTimeFrame calendar-invite default.
//!
//! Calendar invite state is not modeled yet, but Blizzard startup expects the
//! GameTime frame field to exist before real calendar data has been loaded.

use crate::lua_api::WowLuaEnv;

const GAME_TIME_CALENDAR_INVITES_LUA: &str = r#"
if type(GameTimeFrame) == "table" and GameTimeFrame.pendingCalendarInvites == nil then
    GameTimeFrame.pendingCalendarInvites = 0
end
"#;

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(GAME_TIME_CALENDAR_INVITES_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

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
