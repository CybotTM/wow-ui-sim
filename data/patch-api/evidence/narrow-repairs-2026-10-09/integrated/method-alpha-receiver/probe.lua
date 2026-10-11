-- Diagnostic: current simulator behavior only; no native receiver contract.
local parent = CreateFrame('Frame')
local fs = parent:CreateFontString()
local other = parent:CreateFontString()
local getter = fs.GetAlphaGradient
assert(type(getter) == 'function')
local function check(label, object, expectedStart, expectedLength)
    assert(select('#', getter(object)) == 2)
    local start, length = getter(object)
    assert(start == expectedStart and length == expectedLength)
    print('ALPHA_VALID ' .. label .. ' 2 ' .. start .. ' ' .. length)
end
check('default', fs, 0, 0)
fs:SetAlphaGradient(12, 48)
check('set', fs, 12, 48)
fs:SetAlphaGradient(6, 24)
check('replace', fs, 6, 24)
check('isolated', other, 0, 0)
fs:ClearAlphaGradient()
check('clear', fs, 0, 0)
print('ALPHA_LOOKUP Frame ' .. type(parent.GetAlphaGradient))
-- Explicit receiver call is observation, not asserted native compatibility.
local ok, start, length = pcall(getter, parent)
print('ALPHA_RECEIVER Frame ' .. tostring(ok) .. ' ' .. (ok and tostring(start) or 'error') .. ' ' .. (ok and tostring(length) or 'omitted'))
local plain = {}
local plainOk = pcall(getter, plain)
print('ALPHA_RECEIVER plain_table ' .. tostring(plainOk))
check('after_receiver_probes', fs, 0, 0)
print('ALPHA_DIAGNOSTIC_DONE')
