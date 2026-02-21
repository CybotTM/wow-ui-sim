-- TestFramework: lightweight test runner for wow-ui-sim.
-- Provides test(), async_test(), and assertion functions as globals.
-- Loaded automatically by `wow-sim run-tests`.

-- Test registry (reset per file by the Rust runner)
__addon_tests = {}
__addon_test_results = {}
__async_done = false
__async_error = nil

function test(name, fn)
    table.insert(__addon_tests, { name = name, fn = fn, async = false })
end

function async_test(name, fn)
    table.insert(__addon_tests, { name = name, fn = fn, async = true })
end

-- Equality

function assertEquals(expected, actual)
    if expected ~= actual then
        error(string.format("expected %s, got %s", tostring(expected), tostring(actual)), 2)
    end
end

function assertNotEquals(expected, actual)
    if expected == actual then
        error(string.format("expected value to differ from %s", tostring(expected)), 2)
    end
end

-- Boolean / nil

function assertTrue(value)
    if not value then
        error(string.format("expected truthy, got %s", tostring(value)), 2)
    end
end

function assertFalse(value)
    if value then
        error(string.format("expected falsy, got %s", tostring(value)), 2)
    end
end

function assertNil(value)
    if value ~= nil then
        error(string.format("expected nil, got %s", tostring(value)), 2)
    end
end

function assertNotNil(value)
    if value == nil then
        error("expected non-nil value, got nil", 2)
    end
end

-- Error

function assertError(fn)
    local ok = pcall(fn)
    if ok then
        error("expected function to throw an error", 2)
    end
end

-- Type

function assertType(expected, value)
    local actual = type(value)
    if actual ~= expected then
        error(string.format("expected type %s, got %s", expected, actual), 2)
    end
end

-- Numeric

function assertAlmostEquals(expected, actual, tolerance)
    tolerance = tolerance or 0.001
    if math.abs(expected - actual) > tolerance then
        error(string.format("expected ~%s, got %s (tolerance %s)",
            tostring(expected), tostring(actual), tostring(tolerance)), 2)
    end
end

-- Strings

function assertContains(haystack, needle)
    if type(haystack) == "string" then
        if not haystack:find(needle, 1, true) then
            error(string.format("expected string to contain %q", needle), 2)
        end
    elseif type(haystack) == "table" then
        for _, v in pairs(haystack) do
            if v == needle then return end
        end
        error(string.format("expected table to contain %s", tostring(needle)), 2)
    else
        error(string.format("assertContains expects string or table, got %s", type(haystack)), 2)
    end
end

function assertStartsWith(str, prefix)
    if type(str) ~= "string" then
        error(string.format("expected string, got %s", type(str)), 2)
    end
    if str:sub(1, #prefix) ~= prefix then
        error(string.format("expected %q to start with %q", str, prefix), 2)
    end
end

function assertEndsWith(str, suffix)
    if type(str) ~= "string" then
        error(string.format("expected string, got %s", type(str)), 2)
    end
    if str:sub(-#suffix) ~= suffix then
        error(string.format("expected %q to end with %q", str, suffix), 2)
    end
end

function assertMatches(str, pattern)
    if type(str) ~= "string" then
        error(string.format("expected string, got %s", type(str)), 2)
    end
    if not str:match(pattern) then
        error(string.format("expected %q to match pattern %q", str, pattern), 2)
    end
end

-- Count

function assertCount(expected, tbl)
    local count = 0
    for _ in pairs(tbl) do count = count + 1 end
    if count ~= expected then
        error(string.format("expected %d elements, got %d", expected, count), 2)
    end
end

-- Tables (deep comparison)

do
    local function deep_equal(a, b)
        if a == b then return true end
        if type(a) ~= "table" or type(b) ~= "table" then return false end
        for k, v in pairs(a) do
            if not deep_equal(v, b[k]) then return false end
        end
        for k in pairs(b) do
            if a[k] == nil then return false end
        end
        return true
    end

    local function table_to_string(t, depth)
        depth = depth or 0
        if type(t) ~= "table" then return tostring(t) end
        if depth > 3 then return "{...}" end
        local parts = {}
        for k, v in pairs(t) do
            local key = type(k) == "string" and k or "[" .. tostring(k) .. "]"
            parts[#parts + 1] = key .. " = " .. table_to_string(v, depth + 1)
        end
        return "{ " .. table.concat(parts, ", ") .. " }"
    end

    function assertTableEquals(expected, actual)
        if not deep_equal(expected, actual) then
            error(string.format("expected %s, got %s",
                table_to_string(expected), table_to_string(actual)), 2)
        end
    end

    function assertTableContains(tbl, subset)
        for k, v in pairs(subset) do
            if not deep_equal(v, tbl[k]) then
                error(string.format("key %s: expected %s, got %s",
                    tostring(k), table_to_string(v), table_to_string(tbl[k])), 2)
            end
        end
    end
end
