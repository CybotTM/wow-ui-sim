//! Lua-side fallback for the `SecureHandler*` API surface.
//!
//! Replaces the old no-op stubs with real snippet storage + pcall-protected
//! execution so protected frames can wire click-cast / state-driver actions
//! before `Blizzard_RestrictedAddOnEnvironment` loads (that addon registers
//! the full retail implementation, which shadows this fallback once it runs).
//!
//! Semantics:
//! - `SecureHandlerSetFrameRef(frame, label, refFrame)` stores `refFrame` in a
//!   weak-keyed registry at `_G.__secure_handler_frame_refs[frame][label]`.
//!   `SecureHandlerGetFrameRef(frame, label)` is the companion lookup helper.
//!   Weak keys so per-frame refs drop when the frame is GC'd.
//! - `SecureHandlerExecute(frame, body, ...)` compiles `body` with a
//!   `local self = ...;` prelude and runs it under `pcall` with `frame` as
//!   `self` plus any extra varargs. Errors are swallowed (same policy as
//!   retail, which routes through `securecall`).
//! - `SecureHandlerWrapScript(frame, script, header, preBody, postBody)`
//!   installs a wrapping script handler: `preBody` (if any) runs first with
//!   `self = header`, the prior handler runs next, and `postBody` runs last.
//!   Every step is `pcall`-isolated so a bad snippet can't prevent the others
//!   from firing.
//! - `SecureHandlerUnwrapScript(frame, script)` restores the handler that was
//!   active before the first fallback wrap for that frame/script pair.

use rilua::{LuaResult, runtime_error};

pub(super) fn register_secure_handler_stubs(lua: &mut rilua::Lua) -> LuaResult<()> {
    lua.exec(SECURE_HANDLER_FALLBACK_LUA)
        .map_err(|e| runtime_error(format!("secure-handler fallback: {e}")))?;
    Ok(())
}

const SECURE_HANDLER_FALLBACK_LUA: &str = r#"
-- Weak-keyed registries so per-frame state GCs with the owner.
if _G.__secure_handler_frame_refs == nil then
    _G.__secure_handler_frame_refs = setmetatable({}, { __mode = "k" })
end
if _G.__secure_handler_original_scripts == nil then
    _G.__secure_handler_original_scripts = setmetatable({}, { __mode = "k" })
end

if type(SecureHandlerSetFrameRef) ~= "function" then
    function SecureHandlerSetFrameRef(frame, label, refFrame)
        if frame == nil or type(label) ~= "string" or refFrame == nil then
            return
        end
        local refs = _G.__secure_handler_frame_refs[frame]
        if refs == nil then
            refs = {}
            _G.__secure_handler_frame_refs[frame] = refs
        end
        refs[label] = refFrame
    end
end

if type(SecureHandlerGetFrameRef) ~= "function" then
    function SecureHandlerGetFrameRef(frame, label)
        if frame == nil or type(label) ~= "string" then
            return nil
        end
        local refs = _G.__secure_handler_frame_refs[frame]
        if refs == nil then
            return nil
        end
        return refs[label]
    end
end

local function readonly_copy(source)
    local copy = {}
    for key, value in pairs(source) do
        copy[key] = value
    end
    return setmetatable(copy, {
        __newindex = function()
            error("restricted table is read-only")
        end,
        __metatable = false,
    })
end

local restricted_env = setmetatable({
    assert = assert,
    error = error,
    ipairs = ipairs,
    math = readonly_copy(math),
    next = next,
    pairs = pairs,
    print = print,
    select = select,
    string = readonly_copy(string),
    table = readonly_copy(table),
    tonumber = tonumber,
    tostring = tostring,
    type = type,
    unpack = unpack,
}, {
    __newindex = function()
        error("restricted environment is read-only")
    end,
    __metatable = false,
})

-- Compile `body` as a closure `function(self, ...) <body> end`. Returning the
-- closure through an outer loadstring wrapper keeps `self` and the varargs
-- cleanly separated (plain `local self = ...` would consume from the same
-- vararg list and mis-index subsequent destructures). The closure runs in a
-- restricted environment: frame refs arrive through `self` and globals are
-- limited to safe utility tables/functions.
local function compile_snippet(body, chunk_name)
    local loader, err = loadstring("return function(self, ...) " .. body .. " end", chunk_name)
    if not loader then return nil end
    local ok, closure = pcall(loader)
    if not ok or type(closure) ~= "function" then return nil end
    setfenv(closure, restricted_env)
    return closure
end

if type(SecureHandlerExecute) ~= "function" then
    function SecureHandlerExecute(frame, body, ...)
        if frame == nil or type(body) ~= "string" then
            return
        end
        local closure = compile_snippet(body, "SecureHandlerExecute")
        if closure == nil then return end
        pcall(closure, frame, ...)
    end
end

local function original_scripts_for_frame(frame)
    local scripts = _G.__secure_handler_original_scripts[frame]
    if scripts == nil then
        scripts = {}
        _G.__secure_handler_original_scripts[frame] = scripts
    end
    return scripts
end

if type(SecureHandlerWrapScript) ~= "function" then
    function SecureHandlerWrapScript(frame, script, header, preBody, postBody)
        if frame == nil or type(script) ~= "string" or type(preBody) ~= "string" then
            return
        end
        local owner = header or frame
        local pre_closure = compile_snippet(preBody, "SecureHandlerWrapScript-pre")
        local post_closure
        if type(postBody) == "string" then
            post_closure = compile_snippet(postBody, "SecureHandlerWrapScript-post")
        end
        local original = frame.GetScript and frame:GetScript(script) or nil
        local scripts = original_scripts_for_frame(frame)
        if scripts[script] == nil then
            scripts[script] = original or false
        end
        frame:SetScript(script, function(self, ...)
            if pre_closure then
                pcall(pre_closure, owner, ...)
            end
            if original then
                pcall(original, self, ...)
            end
            if post_closure then
                pcall(post_closure, owner, ...)
            end
        end)
    end
end

if type(SecureHandlerUnwrapScript) ~= "function" then
    function SecureHandlerUnwrapScript(frame, script)
        if frame == nil or type(script) ~= "string" then
            return
        end
        local scripts = _G.__secure_handler_original_scripts[frame]
        if scripts == nil or scripts[script] == nil then
            return
        end
        local original = scripts[script]
        scripts[script] = nil
        if original == false then
            original = nil
        end
        frame:SetScript(script, original)
    end
end
"#;
