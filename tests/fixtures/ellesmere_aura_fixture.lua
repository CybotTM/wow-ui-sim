-- Only acceptance inputs and observations; all region wiring is real AuraKit.
local regions = {}
local function create(candidateFilters)
    local host = CreateFrame('Frame', nil, UIParent)
    host:SetSize(64, 64)
    host:SetPoint('CENTER', UIParent, 'CENTER', 0, 0)
    return FixtureCreateContainer(host, 'player', {
        point = {'CENTER', host, 'CENTER'},
        groups = {{
            key = 'current-package-helpful', filter = {'HELPFUL'},
            maxFrameCount = 1, candidateFilters = candidateFilters,
            style = 'current-package-default',
            extraInit = function(button, data)
                regions[button] = data
            end,
        }},
    })
end
debug.setobjecttaint(create, 'EllesmereFixture')
FixtureArray = create({includeSpellIDs = {19750}})
FixtureKeyed = create({includeSpellIDs = {[19750] = true}})

local function readWidget(label, widget, method)
    local taint = debug.getstacktaint()
    local ok, value = pcall(widget[method], widget)
    assert(ok, label .. ':' .. method .. ' caller=' .. tostring(taint) .. ': ' .. tostring(value))
    return value
end

function FixtureAssertShown(container, expected, label)
    local shown = 0
    for index = 1, container:GetAuraGroupFrameCount('current-package-helpful') do
        local button = container:GetAuraGroupFrame('current-package-helpful', index)
        local context = label .. ':button[' .. index .. ']'
        if readWidget(context, button, 'IsShown') then
            shown = shown + 1
            local data = assert(regions[button], 'real initializer regions missing')
            assert(readWidget(context .. ':icon', data.icon, 'GetTexture') == 135907, 'wrong icon')
            assert(readWidget(context .. ':stack', data.stack, 'GetText') == '3', 'wrong stack text')
        end
    end
    assert(shown == expected, 'expected ' .. expected .. ' shown buttons, got ' .. shown)
end
