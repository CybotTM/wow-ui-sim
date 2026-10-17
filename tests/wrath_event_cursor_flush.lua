local box = ScriptErrorsFrameScrollFrameText
assert(box and not box:IsVisible() and not ScriptErrorsFrame:IsShown())
assert(box:GetText() == '' and box.cursorOffset == nil and box.cursorHeight == nil,
    'fixture must start pristine: no incidental cursor delivery during loading')
assert(#ScriptErrorsFrame.order == 0, 'fixture must not inherit load errors')
SetCVar('scriptErrors', '0')
assert(geterrorhandler() == _ERRORMESSAGE)

CursorFlushTrace = {}
debug.sethook(function(_, line)
    if line ~= 348 and line ~= 365 then return end
    local info = debug.getinfo(2, 'S')
    if not info or not string.find(info.source, 'UIPanelTemplates.lua', 1, true) then return end
    local _, frame = debug.getlocal(2, 1)
    if frame ~= box then return end
    if line == 348 then
        local geometry = {}
        for index = 2, 5 do
            local name, value = debug.getlocal(2, index)
            geometry[name] = value
            assert(type(value) == 'number', 'original cursor callback needs numeric geometry')
        end
        assert(geometry.h > 0, 'real font metrics required')
        CursorFlushGeometry = geometry
        CursorFlushTrace[#CursorFlushTrace + 1] = 'cursor'
    else
        CursorFlushConsumer = { offset = frame.cursorOffset, height = frame.cursorHeight }
        CursorFlushTrace[#CursorFlushTrace + 1] = 'consumer'
    end
end, 'l')

local first = CreateFrame('Frame')
first:RegisterEvent('CURSOR_FLUSH_TEST')
first:SetScript('OnEvent', function()
    CursorFlushTrace[#CursorFlushTrace + 1] = 'first'
    error('cursor-flush-first')
end)
local second = CreateFrame('Frame')
second:RegisterEvent('CURSOR_FLUSH_TEST')
second:SetScript('OnEvent', function()
    CursorFlushTrace[#CursorFlushTrace + 1] = 'second'
    CursorFlushFirstText = box:GetText()
    error('cursor-flush-second')
end)
