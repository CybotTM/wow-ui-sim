-- Test observers only. No replacement of any ScrollingEdit function or method.
assert(__wow_sim_wrath_scrolling_edit_wrapped ~= true, 'SETUP FAILURE: guard already installed')
assert(type(ScrollingEdit_OnTextChanged) == 'function')
assert(type(ScrollingEdit_OnCursorChanged) == 'function')
assert(type(ScrollingEdit_OnUpdate) == 'function')

C2Scroll = CreateFrame('ScrollFrame', 'C2WrathConsumerScroll', UIParent)
C2Scroll:SetSize(400, 48)
C2Scroll:SetPoint('TOPLEFT', UIParent, 'TOPLEFT', 113, -79)
C2Box = CreateFrame('EditBox', 'C2WrathConsumerBox', C2Scroll, 'C2WrathScrollingConsumerTemplate')
C2Box:SetAutoFocus(false)
C2Box:SetMultiLine(true)
C2Box:SetPoint('TOPLEFT', C2Scroll, 'TOPLEFT', 0, 0)
assert(C2Box:SetFont('Fonts/FRIZQT__.TTF', 20))
C2Box:SetJustifyH('LEFT')
C2Box:SetJustifyV('TOP')
C2Box:SetTextInsets(5, 7, 3, 4)
C2Scroll:SetScrollChild(C2Box)
-- Automatic child resize establishes range without a cursor warm-up tick.
C2Box:SetSize(400, 240)
assert(C2Scroll:GetVerticalScrollRange() == 192, 'SETUP FAILURE: positive consumer range')
C2Scroll:SetVerticalScroll(0)
C2Scroll:Show()
C2Box:Show()
assert(C2Box:GetScript('OnCursorChanged') == ScrollingEdit_OnCursorChanged,
    'XML must bind original cursor producer before adding observer hook')
assert(C2Box:GetText() == '', 'SETUP FAILURE: fresh text')
assert(C2Box.cursorOffset == nil and C2Box.cursorHeight == nil,
    'SETUP FAILURE: no cursor seeding or warm-up tick allowed')

C2ZWidth = C2Scroll:CreateFontString(nil, 'ARTWORK')
C2CatWidth = C2Scroll:CreateFontString(nil, 'ARTWORK')
for _, reference in ipairs({C2ZWidth, C2CatWidth}) do
    assert(reference:SetFont('Fonts/FRIZQT__.TTF', 20))
end
C2ZWidth:SetText('z')
C2CatWidth:SetText('猫')

local function near(actual, expected, tolerance)
    assert(type(actual) == 'number', 'numeric geometry required')
    assert(math.abs(actual - expected) < (tolerance or 0.001),
        tostring(actual) .. ' != ' .. tostring(expected))
end

function C2Expect(byte, scalar, x, y, scroll, userInput)
    C2Expected = {byte = byte, scalar = scalar, x = x, y = y,
        scroll = scroll, userInput = userInput}
    C2Trace, C2Cursors = {}, {}
    C2ChangedReturns, C2UpdateReturns = 0, 0
end

local function assert_public_cursor(self)
    assert(self == C2Box)
    assert(self:GetText() == 'é\n猫\nz', 'committed Unicode text')
    assert(self:GetCursorPosition() == C2Expected.byte, 'public byte cursor')
    assert(self:GetUTF8CursorPosition() == C2Expected.scalar, 'public scalar cursor')
end

C2Box:HookScript('OnCursorChanged', function(self, ...)
    assert_public_cursor(self)
    assert(select('#', ...) == 4, 'actual callback has four geometry arguments')
    local x, y, width, height = ...
    near(x, C2Expected.x, C2Expected.x > 5 and 1 or 0.001)
    near(y, C2Expected.y)
    near(width, 1.5)
    near(height, 24)
    near(self.cursorOffset, y) -- Written by original producer, never test code.
    near(self.cursorHeight, height)
    C2Cursors[#C2Cursors + 1] = {x, y, width, height}
    C2Trace[#C2Trace + 1] = 'cursor'
end)

local function assert_presented(self)
    assert_public_cursor(self)
    assert(#C2Cursors == 1, 'consumer requires one current real cursor delivery')
    near(self.cursorOffset, C2Expected.y)
    near(self.cursorHeight, 24)
end

function C2BeforeChanged(self, userInput)
    -- Observe without gating the original consumer: reversed delivery must
    -- actually reach its arithmetic on nil, not stop at our precondition.
    C2ChangedEntry = {offset = self.cursorOffset, height = self.cursorHeight,
        cursorCount = #C2Cursors, userInput = userInput,
        scroll = C2Scroll:GetVerticalScroll(), pending = self.handleCursorChange}
    C2Trace[#C2Trace + 1] = 'changed-enter'
end

function C2AfterChanged(self)
    assert_presented(self)
    assert(C2ChangedEntry.cursorCount == 1, 'cursor produced before changed entry')
    near(C2ChangedEntry.offset, C2Expected.y)
    near(C2ChangedEntry.height, 24)
    near(C2ChangedEntry.scroll, 0)
    assert(C2ChangedEntry.pending == true, 'original producer flagged cursor')
    assert(C2ChangedEntry.userInput == C2Expected.userInput,
        'programmatic SetText vs Insert origin')
    assert(C2ChangedReturns == 0 and C2UpdateReturns == 0)
    near(C2Scroll:GetVerticalScroll(), C2Expected.scroll)
    assert(self.handleCursorChange == false, 'synchronous original update consumed cursor')
    C2ChangedReturns = C2ChangedReturns + 1
    C2Trace[#C2Trace + 1] = 'changed-return'
end

function C2BeforeUpdate(self)
    assert_presented(self)
    local expectsChanged = C2Expected.userInput ~= nil
    assert(C2ChangedReturns == (expectsChanged and 1 or 0),
        'synchronous changed consumer must finish before ordinary update')
    assert(self.handleCursorChange == (not expectsChanged))
    C2Trace[#C2Trace + 1] = 'update-enter'
end

function C2AfterUpdate(self)
    assert_presented(self)
    near(C2Scroll:GetVerticalScroll(), C2Expected.scroll)
    assert(self.handleCursorChange == false)
    C2UpdateReturns = C2UpdateReturns + 1
    C2Trace[#C2Trace + 1] = 'update-return'
end

function C2AssertPending()
    assert_public_cursor(C2Box)
    assert(#C2Trace == 0 and #C2Cursors == 0, 'no cursor/changed/update before real tick')
    assert(C2ChangedReturns == 0 and C2UpdateReturns == 0)
end

function C2AssertColdPending(byte, scalar)
    C2AssertPending()
    assert(C2Box:GetCursorPosition() == byte and C2Box:GetUTF8CursorPosition() == scalar)
    assert(C2Box.cursorOffset == nil and C2Box.cursorHeight == nil,
        'original missing-cursor boundary must remain cold until real tick')
    near(C2Scroll:GetVerticalScroll(), 0)
end

function C2AssertDelivered(changed)
    local expected = changed and 'cursor,changed-enter,changed-return,update-enter,update-return'
        or 'cursor,update-enter,update-return'
    assert(table.concat(C2Trace, ',') == expected, table.concat(C2Trace, ','))
    assert(#C2Cursors == 1 and C2ChangedReturns == (changed and 1 or 0))
    assert(C2UpdateReturns == 1, 'ordinary update actually completed')
    assert_presented(C2Box)
    near(C2Scroll:GetHeight(), 48)
    near(C2Scroll:GetVerticalScrollRange(), 192)
    near(C2Scroll:GetVerticalScroll(), C2Expected.scroll)
    assert(__wow_sim_wrath_scrolling_edit_wrapped ~= true, 'no post-load guard masking regression')
end
