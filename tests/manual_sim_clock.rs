//! Explicit manual mode coordinates Lua-visible clocks, callbacks and OnUpdate.
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn manual_clock_advances_gettime_onupdate_and_independent_duration_clock() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        A_Admin.SetManualClock(100)
        assert(GetTime() == 100 and GetTimePreciseSec() == 100)
        local seen = {}
        local frame = CreateFrame('Frame')
        frame:SetScript('OnUpdate', function(_, elapsed)
            seen[#seen+1] = {elapsed, GetTime()}
        end)
        A_Admin.AdvanceTime(2.5)
        assert(GetTime() == 102.5 and GetTimePreciseSec() == 102.5)
        assert(#seen == 1 and seen[1][1] == 2.5 and seen[1][2] == 102.5)
        A_Admin.AdvanceTime(0)
        assert(#seen == 2 and seen[2][1] == 0)
    "#,
    )
    .unwrap();
    #[cfg(feature = "retail-12-0-5")]
    env.exec(
        r#"
        local clock = C_DurationUtil.CreateManualClock(7)
        A_Admin.AdvanceTime(3)
        assert(clock:GetTime() == 7)
        clock:AdvanceTime(2)
        assert(clock:GetTime() == 9 and GetTime() == 105.5)
    "#,
    )
    .unwrap();
    let independent = WowLuaEnv::new().unwrap();
    independent.exec("assert(GetTime() < 100)").unwrap();
}

#[test]
fn manual_clock_drains_deadlines_and_repeated_tickers_in_order() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        A_Admin.SetManualClock(0)
        local seen = {}
        local function record(label) seen[#seen+1] = label end
        C_Timer.After(3, function() record('after3') end)
        local ticker = C_Timer.NewTicker(1, function() record('ticker') end, 4)
        C_Timer.NewTimer(2.5, function() record('timer2.5') end)
        C_Timer.After(1.5, function()
            record('after1.5')
            C_Timer.After(0, function() record('nested') end)
        end)
        A_Admin.AdvanceTime(4)
        assert(table.concat(seen, ',') == 'ticker,after1.5,ticker,timer2.5,after3,ticker,ticker,nested', table.concat(seen, ','))
        A_Admin.AdvanceTime(1)
        assert(#seen == 8)
        local count = 0
        local cancel
        cancel = C_Timer.NewTicker(0.5, function() count=count+1; cancel:Cancel() end)
        A_Admin.AdvanceTime(5)
        assert(count == 1)
    "#).unwrap();
}

#[test]
fn manual_clock_rejects_invalid_deltas_without_advancing_or_dispatching() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(not pcall(A_Admin.AdvanceTime, 1), 'manual mode must be explicit')
        for _, invalid in ipairs({-1, math.huge, -math.huge, 0/0}) do
            assert(not pcall(A_Admin.SetManualClock, invalid))
        end
        A_Admin.SetManualClock(50)
        local count = 0
        CreateFrame('Frame'):SetScript('OnUpdate', function() count=count+1 end)
        for _, invalid in ipairs({-1, math.huge, -math.huge, 0/0}) do
            local ok, err = pcall(A_Admin.AdvanceTime, invalid)
            assert(not ok and type(err) == 'string')
            assert(GetTime() == 50 and count == 0)
        end
    "#,
    )
    .unwrap();
}
