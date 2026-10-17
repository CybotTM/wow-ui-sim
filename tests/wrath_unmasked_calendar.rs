//! Predicted behavioral RED only; no native-date policy claim.
#![cfg(feature = "client-wrath")]
#[path = "wrath_unmasked_support/mod.rs"]
mod support;
use support::{env, load, run};

#[test]
fn wrath_unmasked_original_gametime_default_calendar_projection() {
    let env = env();
    // Ordinary parent fixture; unrelated Minimap sibling scripts are not loaded.
    run(&env, "Minimap = CreateFrame('Frame', 'Minimap', UIParent)");
    load(
        &env,
        &[
            "GlobalStrings.lua",
            "Constants.lua",
            "Fonts.xml",
            "FontStyles.xml",
            "UIPanelTemplates.xml",
            "UIDropDownMenu.xml",
            "GameTime.xml",
        ],
    );
    // FrameXML/GameTime.lua:12 captures date BEFORE OnLoad; :133-148 calls
    // original OnUpdate :176-198; :246-248 writes the day into the button.
    run(
        &env,
        r#"
        local date = C_DateAndTime.GetCurrentCalendarTime()
        local values = {CalendarGetDate()}
        assert(#values == 4)
        assert(values[1] == date.weekday and values[2] == date.month)
        assert(values[3] == date.monthDay and values[4] == date.year)
        assert(GameTimeFrame:GetParent() == Minimap)
        assert(GameTimeFrame:GetText() == tostring(date.monthDay))
        assert(GameTimeFrame.timeOfDay == 12 * 60)
        -- :155-173: dispatch the original registered handler, not a substitute.
        FireEvent('PLAYER_ENTERING_WORLD')
        assert(CalendarGetNumPendingInvites() == C_Calendar.GetNumPendingInvites())
        assert(GameTimeFrame.pendingCalendarInvites == 0)
        assert(GameTimeFrame.flashInvite == false)
        assert(not GameTimeCalendarInvitesTexture:IsShown())
        assert(not GameTimeCalendarInvitesGlow:IsShown())
        assert(GameTimeFrame:GetText() == tostring(date.monthDay))
    "#,
    );
}
