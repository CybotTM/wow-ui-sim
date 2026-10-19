-- Loaded unchanged as two ordinary addons; AsyncRequest itself is never replaced.
local owner = ...;
local responseEvent = owner == "AsyncRequestOwnerA" and "PLAYER_LOGIN" or "PLAYER_LOGOUT";
local probe = { requests = 0, responses = 0, timeouts = 0, cancelledTimeouts = 0 };
AsyncRequestProvenance[owner] = probe;

probe.live = AsyncRequests:CreateAsyncRequest({
    requestFunction = function(payload)
        assert(payload == owner .. "-live");
        probe.requests = probe.requests + 1;
    end,
    responseEventName = "PLAYER_ENTERING_WORLD",
    responseEventCallback = function() error("unexpected live response"); end,
    timeoutSeconds = 3600,
    timeoutCallback = function()
        probe.timeouts = probe.timeouts + 1;
        print("asyncreq-timeout owner=" .. owner);
    end,
});
probe.cancelled = AsyncRequests:CreateAsyncRequest({
    requestFunction = function(payload)
        assert(payload == owner .. "-cancelled");
        probe.requests = probe.requests + 1;
    end,
    responseEventName = responseEvent,
    responseEventCallback = function(payload)
        assert(payload == owner .. "-response");
        probe.responses = probe.responses + 1;
    end,
    timeoutSeconds = 3600,
    timeoutCallback = function()
        probe.cancelledTimeouts = probe.cancelledTimeouts + 1;
        print("asyncreq-cancelled-timeout owner=" .. owner);
    end,
});
probe.live:StartRequest(owner .. "-live");
probe.cancelled:StartRequest(owner .. "-cancelled");
probe.liveHandle = probe.live.timeoutTimer;
probe.cancelledHandle = probe.cancelled.timeoutTimer;
assert(not probe.liveHandle:IsCancelled());
assert(not probe.cancelledHandle:IsCancelled());
assert(probe.live.isRunning and probe.cancelled.isRunning);
assert(probe.cancelled:IsEventRegistered(responseEvent));
probe.responseEvent = responseEvent;
