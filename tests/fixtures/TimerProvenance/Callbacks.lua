TimerProvenance = { calls = 0 }
C_Timer.After(0, function(...)
    assert(select('#', ...) == 0, 'After received arguments')
    TimerProvenance.calls = TimerProvenance.calls + 1
    print('timer-provenance-success-body')
end)
C_Timer.After(0, function(...)
    assert(select('#', ...) == 0, 'After error received arguments')
    TimerProvenance.calls = TimerProvenance.calls + 1
    print('timer-provenance-error-body')
    error('timer-provenance-sentinel')
end)
local ticker
local callback = C_FunctionContainers.CreateCallback(function(proxy)
    assert(proxy == ticker, 'ticker proxy identity changed')
    TimerProvenance.calls = TimerProvenance.calls + 1
end)
ticker = C_Timer.NewTicker(0, callback, 2)
assert(ticker == callback, 'container identity changed')
